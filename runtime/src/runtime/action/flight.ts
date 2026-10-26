/** How many calls an element started await their answer, and whether the runtime set its `aria-busy`. */
interface Marked {
  count: number;
  readonly ours: boolean;
}

/**
 * The calls of each action that await their answer: whether one is in
 * flight, for `exclusive`, the order they were sent in, for dropping an
 * answer that a newer one has already overtaken, and the `aria-busy` of the
 * elements that sent them.
 */
export class Flights {
  /** The number of calls awaiting an answer, by action key. */
  readonly #open = new Map<string, number>();
  /** The number the next call gets, which grows with each call sent. */
  #next = 0;
  /** The number of the newest call whose complete answer has arrived, by action key. */
  readonly #settled = new Map<string, number>();
  readonly #marked = new WeakMap<Element, Marked>();

  /** Whether a call to the action `key` awaits its answer. */
  busy(key: string): boolean {
    return this.#open.has(key);
  }

  /** Records a call to `key` as sent, marking `origin` busy where given, and returns the call's number. */
  start(key: string, origin: Element | undefined): number {
    this.#open.set(key, (this.#open.get(key) ?? 0) + 1);
    if (origin !== undefined) this.#mark(origin);
    return this.#next++;
  }

  /** Records the call numbered `sent` as done, and as the newest complete answer to `key` where `complete`. */
  finish(key: string, sent: number, origin: Element | undefined, complete: boolean): void {
    const open = (this.#open.get(key) ?? 1) - 1;
    if (open === 0) this.#open.delete(key);
    else this.#open.set(key, open);
    if (complete && sent > (this.#settled.get(key) ?? -1)) this.#settled.set(key, sent);
    if (origin !== undefined) this.#unmark(origin);
  }

  /** Whether a call to `key` sent after the call numbered `sent` has already had its complete answer. */
  overtaken(key: string, sent: number): boolean {
    return sent < (this.#settled.get(key) ?? -1);
  }

  /**
   * Sets `aria-busy="true"` on `origin` while its first call is awaited.
   * An element whose own `aria-busy` the page already set keeps it, and the
   * runtime then leaves the attribute alone.
   */
  #mark(origin: Element): void {
    const marked = this.#marked.get(origin);
    if (marked !== undefined) {
      marked.count += 1;
      return;
    }
    const ours = !origin.hasAttribute("aria-busy");
    if (ours) origin.setAttribute("aria-busy", "true");
    this.#marked.set(origin, { count: 1, ours });
  }

  #unmark(origin: Element): void {
    const marked = this.#marked.get(origin);
    if (marked === undefined) return;
    marked.count -= 1;
    if (marked.count > 0) return;
    this.#marked.delete(origin);
    if (marked.ours && origin.getAttribute("aria-busy") === "true")
      origin.removeAttribute("aria-busy");
  }
}
