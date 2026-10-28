import { evaluate, run, type Statement } from "../../eval.ts";
import { resolve } from "../scope.ts";

/** The statements that write a signal, choose, name a value, run in order, wait, or animate. */
export const flows: Record<string, Statement> = {
  set: ([place, valueAst], ctx) => ctx.store.set(resolve(place, ctx), evaluate(valueAst, ctx)),
  if: ([condAst, then, otherwise], ctx, host) =>
    run(evaluate(condAst, ctx) ? then : otherwise, ctx, host),
  // `["let", value, depth, statement]`: the statement form of the `let` expression.
  let: ([valueAst, depth, stmt], ctx, host) => {
    const value = evaluate(valueAst, ctx);
    run(stmt, { ...ctx, vars: [...(ctx.vars ?? []).slice(0, depth as number), value] }, host);
  },
  seq: ([steps], ctx, host) => {
    (steps as unknown[]).forEach((step, index) => {
      if (index > 0) ctx.store.settle();
      run(step, ctx, host);
    });
  },
  transition: ([stmt], ctx, host) => host.transition(() => run(stmt, ctx, host)),
  debounce: ([ms, stmt], ctx, host) => {
    host.debounce(ctx.origin, JSON.stringify(stmt), ms as number, () => {
      try {
        run(stmt, ctx, host);
      } catch (error) {
        console.error("hypersig:", error);
      }
    });
  },
};
