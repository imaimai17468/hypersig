import { expect, it } from "vitest";
import { evaluate } from "../../../src/eval.ts";
import { context } from "../context.ts";

const rows = context({ rows: [{ v: 3 }, { v: -1.5 }, { v: 7 }], none: [] });
const fold = (op: string, list: string) => evaluate([op, list, 0, ["item", 0, "v"]], rows);

it("folds a list as ssr.rs does", () => {
  expect(fold("sum_of", "rows")).toBe(8.5);
  expect(fold("max_of", "rows")).toBe(7);
  expect(fold("min_of", "rows")).toBe(-1.5);
  expect(fold("sum_of", "none")).toBe(0);
  expect(fold("max_of", "none")).toBe(null);
  expect(fold("map", "rows")).toEqual([3, -1.5, 7]);
});

it("counts list items with the item bound at its depth", () => {
  const todos = context({ todos: [{ done: true }, { done: false }, { done: false }] });
  expect(evaluate(["count", "todos", 0, ["not", ["item", 0, "done"]]], todos)).toBe(2);
});
