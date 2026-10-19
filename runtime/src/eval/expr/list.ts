import { type Builder, compile, type Compiled } from "../../eval.ts";
import { list, mapItems, testItems } from "../scope.ts";

/** How `sum_of`, `max_of`, and `min_of` fold the values of a list, as `fold` in `ssr.rs` does. */
const folds: Record<string, (values: number[]) => number | null> = {
  sum_of: (values) => values.reduce((sum, value) => sum + value, 0),
  max_of: (values) =>
    values.length === 0 ? null : values.reduce((max, value) => Math.max(max, value)),
  min_of: (values) =>
    values.length === 0 ? null : values.reduce((min, value) => Math.min(min, value)),
};

/** An expression that folds the value of `eachAst` over the items of the list at `place` with `combine`. */
function fold(
  place: unknown,
  depth: number,
  eachAst: unknown,
  combine: (values: unknown[]) => unknown,
): Compiled {
  const each = compile(eachAst);
  return (ctx) => {
    const found = list(ctx, place);
    return combine(ctx.store.fold(found[2], () => mapItems(ctx, depth, found, each)));
  };
}

/** The expressions over every item of a list, which a change to any item can change. */
export const lists: Record<string, Builder> = {
  count: ([place, depth, testAst]) => {
    const test = compile(testAst);
    return (ctx) => {
      const found = list(ctx, place);
      return ctx.store
        .fold(found[2], () => testItems(ctx, depth as number, found, test))
        .filter(Boolean).length;
    };
  },
  map: ([place, depth, eachAst]) => fold(place, depth as number, eachAst, (values) => values),
  sum_of: ([place, depth, eachAst]) =>
    fold(place, depth as number, eachAst, (values) => folds.sum_of!(values as number[])),
  max_of: ([place, depth, eachAst]) =>
    fold(place, depth as number, eachAst, (values) => folds.max_of!(values as number[])),
  min_of: ([place, depth, eachAst]) =>
    fold(place, depth as number, eachAst, (values) => folds.min_of!(values as number[])),
};
