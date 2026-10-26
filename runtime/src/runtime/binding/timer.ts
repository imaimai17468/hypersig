import type { Runtime } from "../../runtime.ts";
import { attr } from "../element.ts";
import { parse } from "../spec.ts";

/** The `[ms, statement]` of `element`'s attribute with the `dataset` name `key`, if it has one. */
function timed(element: Element, key: string): [number, unknown] | undefined {
  const value = parse(attr(element, key));
  return Array.isArray(value) ? (value as [number, unknown]) : undefined;
}

/** The statements that run after a delay or on an interval. */
export class Timers {
  readonly #rt: Runtime;
  /** The pending debounced actions of each element, by key. */
  readonly #debounced = new WeakMap<Element, Map<string, ReturnType<typeof setTimeout>>>();
  /** The interval each element runs, with the period it was started with. */
  readonly #intervals = new WeakMap<
    Element,
    { timer: ReturnType<typeof setInterval>; ms: number }
  >();

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Runs `action` once no call with the same key on `origin` has arrived for `ms`. */
  debounce(origin: Element | undefined, key: string, ms: number, action: () => void): void {
    const owner = origin ?? this.#rt.root;
    let timers = this.#debounced.get(owner);
    if (timers === undefined) {
      timers = new Map();
      this.#debounced.set(owner, timers);
    }
    clearTimeout(timers.get(key));
    timers.set(
      key,
      setTimeout(() => {
        timers.delete(key);
        if (owner.isConnected) action();
      }, ms),
    );
  }

  /**
   * Runs the statement of `element`'s `data-hs-on-interval` on its period
   * while the element is in the document. Called again after a patch changed
   * the attribute, it starts the new period, or stops where the attribute is
   * gone. Each tick runs the statement the attribute holds then.
   */
  interval(element: Element): void {
    const ms = timed(element, "hsOnInterval")?.[0];
    const running = this.#intervals.get(element);
    if (running?.ms === ms) return;
    if (running !== undefined) clearInterval(running.timer);
    this.#intervals.delete(element);
    if (ms === undefined) return;
    const timer = setInterval(() => {
      const now = timed(element, "hsOnInterval");
      if (!element.isConnected || now === undefined) {
        clearInterval(timer);
        this.#intervals.delete(element);
        return;
      }
      this.#run(element, now[1]);
    }, ms);
    this.#intervals.set(element, { timer, ms });
  }

  /**
   * Runs the statement of `element`'s `data-hs-on-timeout` once after its
   * delay, if the element is still in the document, reading the statement
   * the attribute holds then.
   */
  timeout(element: Element): void {
    const delay = timed(element, "hsOnTimeout");
    if (delay === undefined) return;
    setTimeout(() => {
      const now = timed(element, "hsOnTimeout");
      if (element.isConnected && now !== undefined) this.#run(element, now[1]);
    }, delay[0]);
  }

  #run(element: Element, stmt: unknown): void {
    const { statements } = this.#rt.bindings;
    statements.run(stmt, statements.context(element));
  }
}
