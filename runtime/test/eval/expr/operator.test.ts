import { expect, it } from "vitest";
import { evaluate } from "../../../src/eval.ts";
import { context, ctx } from "../context.ts";

it("matches the operators expr.rs writes", () => {
  expect(
    evaluate(
      ["gt", ["add", ["sig", "count"], ["lit", 1]], ["mul", ["sig", "count"], ["lit", 2]]],
      ctx,
    ),
  ).toBe(false);
  expect(evaluate(["concat", ["sig", "name"], ["str", ["sig", "count"]]], ctx)).toBe(" Tea 3");
  expect(evaluate(["rem", ["lit", -7], ["lit", 3]], ctx)).toBe(-1);
  expect(evaluate(["lower", ["trim", ["sig", "name"]]], ctx)).toBe("tea");
  expect(evaluate(["chars", ["lit", "a😀"]], ctx)).toBe(2);
  expect(
    evaluate(["if", ["eq", ["sig", "filter"], ["lit", "Active"]], ["lit", "a"], ["lit", "b"]], ctx),
  ).toBe("a");
  expect(evaluate(["fixed", ["lit", 2.5], 2], ctx)).toBe("2.50");
  expect(evaluate(["fixed", ["lit", 2.5], 0], ctx)).toBe("3");
  expect(evaluate(["or_else", ["lit", null], ["lit", 4]], ctx)).toBe(4);
});

it("rounds as ssr.rs does", () => {
  expect(evaluate(["floor", ["lit", -2.5]], ctx)).toBe(-3);
  expect(evaluate(["ceil", ["lit", 2.1]], ctx)).toBe(3);
  expect(evaluate(["round", ["lit", 2.5]], ctx)).toBe(3);
  expect(evaluate(["round", ["lit", -2.5]], ctx)).toBe(-2);
  expect(evaluate(["pow", ["lit", 10], ["lit", 2]], ctx)).toBe(100);
  expect(evaluate(["log10", ["lit", 1000]], ctx)).toBe(3);
});

it("compares records structurally", () => {
  expect(evaluate(["eq", ["lit", { a: [1, { b: 2 }] }], ["lit", { a: [1, { b: 2 }] }]], ctx)).toBe(
    true,
  );
  expect(evaluate(["ne", ["lit", { a: 1 }], ["lit", { a: 1, b: 2 }]], ctx)).toBe(true);
});

it("encodes each part of a rest of a path and keeps its slashes", () => {
  expect(evaluate(["uri_path", ["lit", "a b/c?/d"]], context({}))).toBe("a%20b/c%3F/d");
});
