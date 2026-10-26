import type { CallOptions } from "../../eval.ts";
import { readEventStream, type ServerEvent } from "../../sse.ts";
import type { Runtime } from "../../runtime.ts";
import { FILES } from "../binding/ref.ts";

/** The wait before a call sent with `reconnect` is sent again, doubling after each failure up to the last. */
const FIRST_RETRY_MS = 1000;
const LAST_RETRY_MS = 30_000;

/** The header an upload carries, which `crates/hypersig/src/axum.rs` requires. */
const UPLOAD_HEADER = "hypersig-upload";
/**
 * The header every call carries, which `crates/hypersig/src/axum.rs` requires
 * of a GET. A page on another site cannot send it without a preflight, which
 * the server does not answer.
 */
const CALL_HEADER = "hypersig-call";
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
  /** Aborts the request, for `latest`, for a GET whose element left the document, or when the page leaves. */
  readonly controller: AbortController;
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

/** The media type of `response`, lowercase and without its parameters, or `""` without one. */
function mediaType(response: Response): string {
  return (response.headers.get("content-type") ?? "").split(";")[0]!.trim().toLowerCase();
}

/**
 * The text the error signal shows for an answer outside 2xx. A handler that
 * fails writes the sentence for the visitor as `text/plain`, which axum gives
 * a `String`, while a proxy's error page is HTML, so only a plain text body
 * is kept, and the status and its reason stand in for any other.
 */
async function failureText(response: Response): Promise<string> {
  const body = mediaType(response) === "text/plain" ? (await response.text()).trim() : "";
  return body === "" ? `${response.status} ${response.statusText}`.trim() : body;
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
  /** The unfinished GET calls made by an element's statement, which end once the element leaves the document. */
  readonly #owned = new Set<Pending>();

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
    const init: RequestInit = { method };
    const request = (): URL => this.#request(method, path, args(), init, options);
    // The first request is built before the call returns, so a statement after it can change what the arguments read.
    const url = request();
    // The handler decides which signals an action patches, so two call sites of one action write the same signals, and the key is the action.
    const key = `${method} ${path}`;
    const controller = this.#controller(key, options.latest);
    init.signal = controller.signal;
    const abandoned = (): boolean =>
      this.#rt.navigation.leaving ||
      controller.signal.aborted ||
      options.origin?.isConnected === false;
    // The store notifies only a change, so a failure that repeats the last message would go unnoticed if the signal kept it.
    if (error !== null) this.#rt.store.set(error, "");
    if (options.status !== undefined) this.#rt.store.set(options.status, 0);
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
      controller,
      open,
      abandoned,
    };
    // A GET changes nothing on the server, so the patches of one whose element
    // is gone have nothing to update, and a statement that runs again, such as
    // on load of the element that replaced it, opens its own.
    if (method === "GET" && options.origin !== undefined) this.#owned.add(pending);
    try {
      await this.#attempt(pending, url, FIRST_RETRY_MS);
    } finally {
      this.#owned.delete(pending);
      if (this.#latest.get(key) === controller) this.#latest.delete(key);
      if (busy !== null) this.#busy(busy, -1);
    }
  }

  /** The controller of a new call of the action `key`, which aborts the earlier one sent with `latest` where this one is too. */
  #controller(key: string, latest: boolean): AbortController {
    const controller = new AbortController();
    if (latest) {
      this.#latest.get(key)?.abort();
      this.#latest.set(key, controller);
    }
    return controller;
  }

  /** Aborts the unfinished GET calls whose element has left the document. */
  prune(): void {
    for (const call of this.#owned) if (call.options.origin?.isConnected === false) this.#end(call);
  }

  #end(call: Pending): void {
    this.#owned.delete(call);
    call.controller.abort();
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
    const headers: Record<string, string> = { accept: "text/event-stream", [CALL_HEADER]: "1" };
    if (options.files !== undefined) {
      // The server refuses an upload without this header, which a form on another site cannot add.
      headers[UPLOAD_HEADER] = "1";
      init.body = this.#uploadBody(body, options.files, options.origin);
    } else if (method === "GET") {
      if (value !== null && value !== undefined) url.searchParams.set("args", body);
    } else {
      headers["content-type"] = "application/json";
      init.body = body;
    }
    if (options.part !== undefined) headers[PART_HEADER] = options.part;
    init.headers = headers;
    return url;
  }

  /** Sends `url`, and sends it again after `delay` while the call asks to reconnect and nothing abandoned it. */
  async #attempt(call: Pending, url: URL, delay: number): Promise<void> {
    let opened = false;
    let wait = delay;
    const connected = (): void => {
      if (call.options.status !== undefined) this.#rt.store.set(call.options.status, 0);
      if (call.open === null) return;
      opened = true;
      this.#busy(call.open, 1);
    };
    const received = (event: ServerEvent): void => {
      if (this.#owned.has(call) && call.options.origin?.isConnected === false) {
        this.#end(call);
        return;
      }
      // An answer that closes before it sends anything is retried later each
      // time, so a server that closes every stream at once is not asked again
      // every second.
      wait = FIRST_RETRY_MS;
      this.#rt.actions.patches.apply(event);
    };
    const failure = await this.#send(url, call, connected, received);
    if (opened) this.#busy(call.open!, -1);
    if (call.controller.signal.aborted) return;
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
   * Sends one request and hands each event of its answer to `received` until
   * the call is aborted. Returns `null` once the server ends the stream with
   * `hs-end`, or the failure. An answer that closes before `hs-end` is a
   * failure, which is how a connection that drops in the middle, or a host
   * that cuts a response at a time limit, ends it. `connected` runs when the
   * server answers with a 2xx event stream.
   */
  async #send(
    url: URL,
    { init, error, controller }: Pending,
    connected: () => void,
    received: (event: ServerEvent) => void,
  ): Promise<Failure | null> {
    try {
      const response = await fetch(url, init);
      if (!response.ok) return { message: await failureText(response), status: response.status };
      // A proxy that sends a signed-out visitor to its sign-in page answers
      // the redirected `fetch` with a 200 HTML page, which carries no patches.
      if (response.redirected)
        return { message: `the answer was redirected to ${response.url}`, status: 0 };
      if (mediaType(response) !== "text/event-stream" || response.body === null)
        return {
          message: `the answer is ${mediaType(response) || "empty"}, not an event stream`,
          status: 0,
        };
      connected();
      if (error !== null) this.#rt.store.set(error, "");
      let ended = false;
      await readEventStream(response.body, (event) => {
        if (controller.signal.aborted) return;
        if (event.event === END_EVENT) ended = true;
        else received(event);
      });
      if (!ended) return { message: "the answer closed before the server ended it", status: 0 };
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

  #fail({ method, path, error, options }: Pending, { message, status }: Failure): void {
    if (error !== null) this.#rt.store.set(error, message);
    if (options.status !== undefined) this.#rt.store.set(options.status, status);
    this.#rt.root.dispatchEvent(new CustomEvent("hs-error", { detail: { status, message } }));
    // A message made from an empty body already starts with the status.
    const answer =
      status === 0 || message.startsWith(`${status} `) || message === `${status}`
        ? message
        : `${status}: ${message}`;
    console.error(`hypersig: ${method} ${path} failed with ${answer}`);
  }
}
