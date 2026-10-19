import type { Runtime } from "../../runtime.ts";

/** The statements that run after a delay or on an interval. */
export class Timers {
  readonly #rt: Runtime;
  /** The pending debounced actions of each element, by key. */
  readonly #debounced = new WeakMap<Element, Map<string, ReturnType<typeof setTimeout>>>();

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

  /** Runs `stmt` of `element` every `ms` while it is in the document. */
  interval(element: Element, ms: number, stmt: unknown): void {
    const timer = setInterval(() => {
      if (!element.isConnected) {
        clearInterval(timer);
        return;
      }
      this.#rt.bindings.statements.run(stmt, this.#rt.bindings.statements.context(element));
    }, ms);
  }

  /** Runs `stmt` of `element` once after `ms`, if it is still in the document. */
  timeout(element: Element, ms: number, stmt: unknown): void {
    setTimeout(() => {
      if (element.isConnected)
        this.#rt.bindings.statements.run(stmt, this.#rt.bindings.statements.context(element));
    }, ms);
  }
}
