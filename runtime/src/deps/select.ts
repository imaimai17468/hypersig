import type { Effect, Selector } from "../deps.ts";

/** The bindings that compared the value at one path, by the key of the value each compared against. */
interface Selection {
  readonly byKey: Map<string, Set<Effect>>;
  /** The keys of the values at the path that registered bindings compared against since the last `selected`. */
  seen: Set<string>;
}

/** The comparisons bindings registered in place of reading a path, by path. */
export class Selections {
  readonly #byPath = new Map<string, Selection>();

  /**
   * Registers `effect` to run when the value at `selector.path` becomes, or
   * stops being, the value whose key is `selector.key`, replacing what it
   * registered before.
   */
  select(effect: Effect, selector: Selector | null): void {
    const old = effect.selector;
    if (old !== null) this.#unselect(effect, old);
    effect.selector = selector;
    if (selector === null) return;
    let selection = this.#byPath.get(selector.path);
    if (selection === undefined) {
      selection = { byKey: new Map(), seen: new Set() };
      this.#byPath.set(selector.path, selection);
    }
    let effects = selection.byKey.get(selector.key);
    if (effects === undefined) {
      effects = new Set();
      selection.byKey.set(selector.key, effects);
    }
    effects.add(effect);
    selection.seen.add(selector.current);
  }

  #unselect(effect: Effect, old: Selector): void {
    const selection = this.#byPath.get(old.path);
    const effects = selection?.byKey.get(old.key);
    effects?.delete(effect);
    if (effects?.size === 0) selection!.byKey.delete(old.key);
    if (selection?.byKey.size === 0) this.#byPath.delete(old.path);
  }

  /**
   * Adds to `found` the bindings whose comparison can have changed: for each
   * path that `touched` says a change reached, the ones registered under a
   * key seen there before or under `keyAt(path)`, the key of its value now.
   */
  selected(
    touched: (path: string) => boolean,
    keyAt: (path: string) => string,
    found: Set<Effect>,
  ): void {
    for (const [path, selection] of this.#byPath) {
      if (!touched(path)) continue;
      const now = keyAt(path);
      for (const key of [...selection.seen, now]) {
        for (const effect of selection.byKey.get(key) ?? []) found.add(effect);
      }
      selection.seen = new Set([now]);
    }
  }
}
