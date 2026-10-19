import { type Context, run } from "../../eval.ts";
import type { Runtime } from "../../runtime.ts";
import { scopeOf } from "../element.ts";
import { parse } from "../spec.ts";

/** Runs the statements of handlers, effects, and timers against the page's store. */
export class Statements {
  readonly #rt: Runtime;

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** What a statement of `element` runs against: the store, and the list items around it. */
  context(element: Element): Context {
    return { store: this.#rt.store, scope: scopeOf(element), origin: element };
  }

  /** Runs the statement whose JSON is `source`, written on `element`. */
  runAttribute(source: string, element: Element): void {
    this.run(parse(source), this.context(element));
  }

  /** Runs a statement whose reads do not belong to the binding being rendered. */
  run(ast: unknown, ctx: Context): void {
    this.#rt.store.untracked(() => this.runTracked(ast, ctx));
  }

  /** Runs a statement, recording its reads in the enclosing binding. */
  runTracked(ast: unknown, ctx: Context): void {
    try {
      run(ast, ctx, this.#rt);
    } catch (error) {
      console.error("hypersig:", error);
    }
  }
}
