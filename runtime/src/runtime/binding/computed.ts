import { type Context, evaluate } from "../../eval.ts";
import type { Effect } from "../../deps.ts";
import { ChangeLookup, type Changes } from "../../store/change.ts";
import type { Runtime } from "../../runtime.ts";
import { data, parse } from "../spec.ts";

/** The signals `<body>` computes from others with `data-hs-computed`, and the page title it binds. */
export class Computed {
  readonly #rt: Runtime;
  /** The computed signals, in the order they run. */
  #derived: Effect[] = [];

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Registers the computed signals on `root`, then its title binding, before any element, so they run first. */
  register(root: Element): void {
    const ctx: Context = { store: this.#rt.store, scope: [] };
    const computed = parse(data(root).hsComputed);
    if (Array.isArray(computed)) {
      this.#derived = (computed as [string, unknown][]).map(([path, expr]) =>
        this.#rt.render.effect(root, () => {
          const value = evaluate(expr, ctx);
          this.#rt.store.untracked(() => this.#rt.store.set(path, value));
        }),
      );
      this.#rt.render.queue(this.#derived);
    }
    const title = parse(data(root).hsTitle);
    if (title === undefined) return;
    this.#rt.render.queue([
      this.#rt.render.effect(root, () => {
        const next = String(evaluate(title, ctx));
        if (document.title !== next) document.title = next;
      }),
    ]);
  }

  /**
   * Runs the computed signals whose last reads `changes` affects, oldest
   * first. Their own writes join `changes`, so a computed signal that reads
   * an earlier one sees its new value.
   */
  derive(changes: Changes): void {
    let lookup = new ChangeLookup(changes);
    for (const effect of this.#derived) {
      if (effect.deps === null || !lookup.affectsAny(effect.deps)) continue;
      this.#rt.store.untracked(effect.run);
      lookup = new ChangeLookup(changes);
    }
  }
}
