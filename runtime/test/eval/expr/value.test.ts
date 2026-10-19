import { expect, it } from "vitest";
import { evaluate } from "../../../src/eval.ts";
import { context, ctx } from "../context.ts";

it("reads signals, nested fields, and literals", () => {
  expect(evaluate(["sig", "count"], ctx)).toBe(3);
  expect(evaluate(["sig", "panel.open"], ctx)).toBe(true);
  expect(evaluate(["lit", "x"], ctx)).toBe("x");
});

it("names a value once for its body", () => {
  const rows = context({ rows: [{ v: 3 }, { v: 7 }] });
  const best = ["max_of", "rows", 0, ["item", 0, "v"]];
  expect(evaluate(["let", best, 0, ["add", ["var", 0], ["var", 0]]], rows)).toBe(14);
  expect(
    evaluate(["let", ["lit", 2], 0, ["let", ["lit", 5], 1, ["sub", ["var", 1], ["var", 0]]]], rows),
  ).toBe(3);
});

it("builds a tuple into an array, as serde writes one", () => {
  expect(evaluate(["tuple", ["sig", "count"], ["lit", "x"], ["not", ["lit", false]]], ctx)).toEqual(
    [3, "x", true],
  );
});

it("draws random integers below the bound, reads the clock, and indexes lists", () => {
  for (let k = 0; k < 50; k++) {
    const value = evaluate(["random", ["lit", 3]], ctx) as number;
    expect(Number.isInteger(value)).toBe(true);
    expect(value).toBeGreaterThanOrEqual(0);
    expect(value).toBeLessThan(3);
  }
  expect(Math.abs((evaluate(["now"], ctx) as number) - Date.now())).toBeLessThan(1000);
  expect(evaluate(["nth", ["lit", ["a", "b"]], ["lit", 1]], ctx)).toBe("b");
  expect(evaluate(["nth", ["lit", ["a", "b"]], ["lit", 5]], ctx)).toBe(null);
});

it("reads and updates record fields", () => {
  expect(evaluate(["get", ["sig", "panel"], "open"], ctx)).toBe(true);
  expect(
    evaluate(["with", ["lit", { title: "", done: false }], "title", ["lit", "x"]], ctx),
  ).toEqual({ title: "x", done: false });
});
