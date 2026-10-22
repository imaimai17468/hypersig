import type { CallOptions } from "../../eval.ts";
import { readEventStream } from "../../sse.ts";
import type { Runtime } from "../../runtime.ts";
import { FILES } from "../binding/ref.ts";

/** The wait before a call sent with `reconnect` is sent again, doubling after each failure up to the last. */
const FIRST_RETRY_MS = 1000;
const LAST_RETRY_MS = 30_000;

/** The header an upload carries, which `crates/hypersig/src/axum.rs` requires. */
const UPLOAD_HEADER = "hypersig-upload";
/** The header that tells the server which page field the part that made a call sits at. */
const PART_HEADER = "hypersig-part";
/** The event `crates/hypersig/src/axum.rs` sends last on a stream whose handler finished. */
const END_EVENT = "hs-end";

/** A call in progress: what it sends, and what it reports to while it waits. */
interface Pending {
  readonly method: string;
  readonly path: string;
  readonly init: RequestInit;
  /** Builds the URL and the body again from the arguments as they are now. */
  readonly request: () => URL;
  readonly error: string | null;
  readonly options: CallOptions;
  readonly latest: AbortController | undefined;
  /** The indicator that is true while the server's answer is open, for a call sent with `connected`. */
  readonly open: string | null;
  /** Whether nothing waits for the call any more. */
  readonly abandoned: () => boolean;
}

/** A failed request: the text the page shows for it, and the HTTP status, 0 where none came back. */
interface Failure {
  readonly message: string;
  readonly status: number;
}

function sleep(ms: number): Promise<void> {
  return new Promise((done) => {
    setTimeout(done, ms);
  });
}

/** The action calls statements make, and the indicators that count the ones awaiting an answer. */
export class Calls {
  readonly #rt: Runtime;
  /** The unfinished call sent with `latest`, by method and action path. */
  readonly #latest = new Map<string, AbortController>();
  /** The number of calls awaiting an answer, by the indicator signal they set. */
  readonly #waiting = new Map<string, number>();

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /**
   * Calls an action with the value of `args` and applies the patches it
   * streams back. `args` runs again before each reconnect, so a stream can
   * be told what the page already holds.
   */
  async call(
    method: string,
    path: string,
    args: () => unknown,
    indicator: string | null,
    error: string | null,
    options: CallOptions,
  ): Promise<void> {
    const init: RequestInit = { method, headers: { accept: "text/event-stream" } };
    const request = (): URL => this.#request(method, path, args(), init, options);
    // The first request is built before the call returns, so a statement after it can change what the arguments read.
    const url = request();
    // The handler decides which signals an action patches, so two call sites of one action write the same signals, and the key is the action.
    const key = `${method} ${path}`;
    let latest: AbortController | undefined;
    if (options.latest) {
      this.#latest.get(key)?.abort();
      latest = new AbortController();
      this.#latest.set(key, latest);
      init.signal = latest.signal;
    }
    const abandoned = (): boolean =>
      this.#rt.navigation.leaving ||
      latest?.signal.aborted === true ||
      options.origin?.isConnected === false;
    // The store notifies only a change, so a failure that repeats the last message would go unnoticed if the signal kept it.
    if (error !== null) this.#rt.store.set(error, "");
    // A stream's indicator says whether its answer is open, so it is false while the stream connects and while it waits to reconnect.
    const busy = options.connected ? null : indicator;
    const open = options.connected ? indicator : null;
    if (busy !== null) this.#busy(busy, 1);
    const pending: Pending = {
      method,
      path,
      init,
      request,
      error,
      options,
      latest,
      open,
      abandoned,
    };
    try {
      await this.#attempt(pending, url, FIRST_RETRY_MS);
    } finally {
      if (latest !== undefined && this.#latest.get(key) === latest) this.#latest.delete(key);
      if (busy !== null) this.#busy(busy, -1);
    }
  }

  /** The URL of a call with the arguments `value`, setting the body and headers of `init` to match. */
  #request(
    method: string,
    path: string,
    value: unknown,
    init: RequestInit,
    options: CallOptions,
  ): URL {
    const url = new URL(path, document.baseURI);
    const body = JSON.stringify(value ?? null);
    if (options.files !== undefined) {
      // The server refuses an upload without this header, which a form on another site cannot add.
      init.headers = { accept: "text/event-stream", [UPLOAD_HEADER]: "1" };
      init.body = this.#uploadBody(body, options.files, options.origin);
    } else if (method === "GET") {
      if (value !== null && value !== undefined) url.searchParams.set("args", body);
    } else {
      init.headers = { accept: "text/event-stream", "content-type": "application/json" };
      init.body = body;
    }
    if (options.part !== undefined)
      init.headers = { ...(init.headers as Record<string, string>), [PART_HEADER]: options.part };
    return url;
  }

  /** Sends `url`, and sends it again after `delay` while the call asks to reconnect and nothing abandoned it. */
  async #attempt(call: Pending, url: URL, delay: number): Promise<void> {
    let opened = false;
    let wait = delay;
    const failure = await this.#send(url, call.init, call.error, call.options.reconnect, () => {
      wait = FIRST_RETRY_MS;
      if (call.open === null) return;
      opened = true;
      this.#busy(call.open, 1);
    });
    if (opened) this.#busy(call.open!, -1);
    if (call.latest?.signal.aborted === true) return;
    if (failure === null) {
      if (!this.#rt.navigation.leaving) call.options.succeeded?.();
      return;
    }
    this.#fail(call, failure);
    call.options.failed?.(failure.message);
    const again =
      call.options.reconnect &&
      call.method === "GET" &&
      (failure.status === 0 || failure.status >= 500);
    if (!again || call.abandoned()) return;
    await sleep(wait);
    if (call.abandoned()) return;
    await this.#attempt(call, call.request(), Math.min(wait * 2, LAST_RETRY_MS));
  }

  /**
   * Sends one request and applies the events of its answer. Returns `null`
   * once the server ends the stream, or the failure. With `ends`, an answer
   * that closes before `hs-end` is a failure, which is how a host that cuts
   * a response at a time limit ends it. `connected` runs when the server
   * answers with a 2xx status.
   */
  async #send(
    url: URL,
    init: RequestInit,
    error: string | null,
    ends: boolean,
    connected: () => void,
  ): Promise<Failure | null> {
    try {
      const response = await fetch(url, init);
      if (!response.ok || response.body === null) {
        // The error signal is text a page shows, so it holds the body the server wrote for the visitor, and the status only when there is no body.
        const body = (await response.text()).trim();
        const message = body === "" ? `${response.status} ${response.statusText}`.trim() : body;
        return { message, status: response.status };
      }
      connected();
      if (error !== null) this.#rt.store.set(error, "");
      let ended = false;
      await readEventStream(response.body, (event) => {
        if (event.event === END_EVENT) ended = true;
        else this.#rt.actions.patches.apply(event);
      });
      if (ends && !ended)
        return { message: "the stream closed before the server ended it", status: 0 };
      return null;
    } catch (cause) {
      return { message: String(cause), status: 0 };
    }
  }

  /**
   * The body of an upload: the arguments as JSON in the `args` part, then
   * each file picked in the file input named `files` in a `files` part.
   */
  #uploadBody(args: string, files: string, origin: Element | undefined): FormData {
    const data = new FormData();
    data.append("args", new Blob([args], { type: "application/json" }));
    const input = this.#rt.bindings.refs.find(FILES, files, origin);
    if (input instanceof HTMLInputElement && input.files !== null) {
      for (const file of input.files) data.append("files", file, file.name);
    } else if (!(input instanceof HTMLInputElement)) {
      console.warn(`hypersig: no file input named "${files}"`);
    }
    return data;
  }

  /** Counts the calls awaiting an answer under `indicator`, which is true while any of them is. */
  #busy(indicator: string, by: 1 | -1): void {
    const count = (this.#waiting.get(indicator) ?? 0) + by;
    if (count === 0) this.#waiting.delete(indicator);
    else this.#waiting.set(indicator, count);
    this.#rt.store.set(indicator, count > 0);
  }

  #fail({ method, path, error }: Pending, { message, status }: Failure): void {
    if (error !== null) this.#rt.store.set(error, message);
    this.#rt.root.dispatchEvent(new CustomEvent("hs-error", { detail: { status, message } }));
    // A message made from an empty body already starts with the status.
    const answer =
      status === 0 || message.startsWith(`${status} `) || message === `${status}`
        ? message
        : `${status}: ${message}`;
    console.error(`hypersig: ${method} ${path} failed with ${answer}`);
  }
}
