import type { CallOptions } from "../../eval.ts";
import { readEventStream, type ServerEvent } from "../../sse.ts";
import type { Runtime } from "../../runtime.ts";
import { END_EVENT, type Failure, refused } from "./answer.ts";
import { Flights } from "./flight.ts";
import { buildRequest } from "./request.ts";
import { backoff, FIRST_RETRY_MS, LAST_RETRY_MS } from "./retry.ts";

/** The reason a call's controller aborts with once its `timeout` has passed. */
const TIMED_OUT = Symbol("timed out");

/** A call in progress: what it sends, and what it reports to while it waits. */
interface Pending {
  readonly method: string;
  readonly path: string;
  /** The action and the part the call was made in, which `latest`, `exclusive`, and the order of answers group calls by. */
  readonly key: string;
  /** The number `Flights` gave the call when it was sent. */
  readonly sent: number;
  /** Whether the call changes data, so that its answer can overtake an earlier one, as `#attempt` describes. */
  readonly ordered: boolean;
  /** The element whose `aria-busy` `Flights` sets while the call is awaited. */
  readonly marked: Element | undefined;
  readonly init: RequestInit;
  /** Builds the URL and the body again from the arguments as they are now. */
  readonly request: () => URL;
  readonly error: string | null;
  readonly options: CallOptions;
  /** Aborts the request, for `latest`, for a GET whose element left the document, or once `timeout` has passed. */
  readonly controller: AbortController;
  /** The indicator that is true while the server's answer is open, for a call sent with `connected`. */
  readonly open: string | null;
  /** Whether nothing waits for the call any more. */
  readonly abandoned: () => boolean;
}

/** The action calls statements make, and the indicators that count the ones awaiting an answer. */
export class Calls {
  readonly #rt: Runtime;
  /** The unfinished call sent with `latest`, by method and action path. */
  readonly #latest = new Map<string, AbortController>();
  /** The number of calls awaiting an answer, by the indicator signal they set. */
  readonly #waiting = new Map<string, number>();
  readonly #flights = new Flights();
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
    // The handler decides which signals an action patches, so two call sites
    // of one action write the same signals, and the key is the action. A
    // call made in a part writes them under the part's field, so the part
    // is in the key too.
    const key = `${method} ${path} ${options.part ?? ""}`;
    if (options.exclusive && this.#flights.busy(key)) return;
    const pending = this.#pending(method, path, key, args, error, options, indicator);
    // The first request is built before the call returns, so a statement after it can change what the arguments read.
    const url = pending.request();
    // The store notifies only a change, so a failure that repeats the last message would go unnoticed if the signal kept it.
    if (error !== null) this.#rt.store.set(error, "");
    if (options.status !== undefined) this.#rt.store.set(options.status, 0);
    // A stream's indicator says whether its answer is open, so it is false while the stream connects and while it waits to reconnect.
    const busy = options.connected ? null : indicator;
    if (busy !== null) this.#busy(busy, 1);
    // A GET changes nothing on the server, so the patches of one whose element
    // is gone have nothing to update, and a statement that runs again, such as
    // on load of the element that replaced it, opens its own.
    if (method === "GET" && options.origin !== undefined) this.#owned.add(pending);
    const timer =
      options.timeout === undefined
        ? undefined
        : setTimeout(() => pending.controller.abort(TIMED_OUT), options.timeout);
    let complete = false;
    try {
      complete = await this.#attempt(pending, url, FIRST_RETRY_MS);
    } finally {
      clearTimeout(timer);
      this.#flights.finish(key, pending.sent, pending.marked, complete && pending.ordered);
      this.#owned.delete(pending);
      if (this.#latest.get(key) === pending.controller) this.#latest.delete(key);
      if (busy !== null) this.#busy(busy, -1);
    }
  }

  /** The record of a new call to `path`, which `Flights` counts as sent from now on. */
  #pending(
    method: string,
    path: string,
    key: string,
    args: () => unknown,
    error: string | null,
    options: CallOptions,
    indicator: string | null,
  ): Pending {
    const init: RequestInit = { method };
    const controller = this.#controller(key, options.latest);
    init.signal = controller.signal;
    const ordered = method !== "GET";
    // A GET may be a stream that stays open, and `aria-busy` on its element
    // would hold back what assistive technology announces inside it for as
    // long as it does.
    const marked = ordered ? options.origin : undefined;
    return {
      method,
      path,
      key,
      sent: this.#flights.start(key, marked),
      ordered,
      marked,
      init,
      request: () => buildRequest(this.#rt, method, path, args(), init, options),
      error,
      options,
      controller,
      open: options.connected ? indicator : null,
      abandoned: () =>
        this.#rt.navigation.leaving ||
        controller.signal.aborted ||
        options.origin?.isConnected === false,
    };
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

  /**
   * Sends `url`, and sends it again after `delay` while the call asks to
   * reconnect and nothing abandoned it. Resolves to whether the server's
   * answer arrived complete.
   *
   * The events of a POST that arrive after the complete answer to a POST
   * of the same action sent later are dropped, since the later answer was
   * written from newer data. A GET is left out of that order: two elements
   * that each open a stream on load send the same request, and the first
   * stream to end would otherwise silence the other. A GET that replaces
   * an earlier answer is sent with `latest` instead.
   */
  async #attempt(call: Pending, url: URL, delay: number): Promise<boolean> {
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
      if (call.ordered && this.#flights.overtaken(call.key, call.sent)) return;
      // An answer that closes before it sends anything is retried later each
      // time, so a server that closes every stream at once is not asked again
      // every second.
      wait = FIRST_RETRY_MS;
      this.#rt.actions.patches.apply(event);
    };
    const sent = await this.#send(url, call, connected, received);
    if (opened) this.#busy(call.open!, -1);
    const { signal } = call.controller;
    if (signal.aborted && signal.reason !== TIMED_OUT) return false;
    const failure = signal.aborted
      ? { message: `the answer took longer than ${call.options.timeout} ms`, status: 0 }
      : sent;
    if (failure === null) {
      if (!this.#rt.navigation.leaving) call.options.succeeded?.();
      return true;
    }
    this.#fail(call, failure);
    call.options.failed?.(failure.message);
    const again =
      call.options.reconnect &&
      call.method === "GET" &&
      (failure.status === 0 || failure.status >= 500);
    if (!again || call.abandoned()) return false;
    await backoff(wait);
    if (call.abandoned()) return false;
    return this.#attempt(call, call.request(), Math.min(wait * 2, LAST_RETRY_MS));
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
      const failure = await refused(response);
      if (failure !== null) return failure;
      connected();
      if (error !== null) this.#rt.store.set(error, "");
      let ended = false;
      await readEventStream(
        response.body!,
        (event) => {
          if (controller.signal.aborted) return;
          if (event.event === END_EVENT) ended = true;
          else received(event);
        },
        controller.signal,
      );
      if (!ended) return { message: "the answer closed before the server ended it", status: 0 };
      return null;
    } catch (cause) {
      return { message: String(cause), status: 0 };
    }
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
