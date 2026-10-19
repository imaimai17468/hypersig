import { type Context, compile, evaluate, type Slot, type Statement } from "../../eval.ts";
import { list } from "../scope.ts";

/** Adds the list `addedAst` evaluates to, before the items of the list at `place` when `first`, or after them. */
function add(place: unknown, addedAst: unknown, ctx: Context, first: boolean): void {
  const [path, items] = list(ctx, place);
  const added = evaluate(addedAst, ctx);
  if (!Array.isArray(added) || added.length === 0) return;
  ctx.store.set(
    path,
    first ? [...(added as unknown[]), ...items] : [...items, ...(added as unknown[])],
  );
}

/** The statements that add, remove, or swap items of a list by position. */
export const positions: Record<string, Statement> = {
  push: ([place, itemAst], ctx) => {
    const [path, items] = list(ctx, place);
    ctx.store.set(path, [...items, evaluate(itemAst, ctx)]);
  },
  append: ([place, addedAst], ctx) => add(place, addedAst, ctx, false),
  prepend: ([place, addedAst], ctx) => add(place, addedAst, ctx, true),
  remove: ([place, indexAst], ctx) => {
    const [path, items] = list(ctx, place);
    const index = evaluate(indexAst, ctx) as number;
    if (index >= 0 && index < items.length)
      ctx.store.set(
        path,
        items.filter((_, i) => i !== index),
      );
  },
  swap: ([place, aAst, bAst], ctx) => {
    const [path, items] = list(ctx, place);
    const [a, b] = [evaluate(aAst, ctx) as number, evaluate(bAst, ctx) as number];
    if (a === b || !(a in items) || !(b in items)) return;
    const next = [...items];
    [next[a], next[b]] = [next[b], next[a]];
    ctx.store.set(path, next);
  },
  extend: ([place, countAst, depth, makeAst], ctx) => {
    const [path, items] = list(ctx, place);
    const count = evaluate(countAst, ctx) as number;
    const make = compile(makeAst);
    const at: Slot = { path: "" };
    const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth as number), at] };
    const added = Array.from({ length: Math.max(0, count) }, (_, index) => {
      at.path = String(index);
      return make(itemCtx);
    });
    if (added.length > 0) ctx.store.set(path, [...items, ...added]);
  },
};
