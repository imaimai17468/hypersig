import { describe, expect, it, vi } from "vitest";
import { AstError, evaluate, run } from "../src/eval.ts";
import { Store } from "../src/store.ts";

describe("evaluate", () => {
  const store = new Store({ count: 3, name: "tea" });

  it("reads signals and literals", () => {
    expect(evaluate(["sig", "count"], store)).toBe(3);
    expect(evaluate(["lit", "x"], store)).toBe("x");
  });

  it("matches the operators expr.rs writes", () => {
    expect(evaluate(["gt", ["add", ["sig", "count"], ["lit", 1]], ["mul", ["sig", "count"], ["lit", 2]]], store)).toBe(false);
    expect(evaluate(["concat", ["sig", "name"], ["str", ["sig", "count"]]], store)).toBe("tea3");
    expect(evaluate(["or", ["not", ["gt", ["sig", "count"], ["lit", 0]]], ["eq", ["sig", "name"], ["lit", ""]]], store)).toBe(false);
  });

  it("rejects an operation it does not know", () => {
    expect(() => evaluate(["pow", ["lit", 2], ["lit", 3]], store)).toThrow(AstError);
  });
});

describe("run", () => {
  it("sets signals in order and awaits calls", async () => {
    const store = new Store({ count: 0 });
    const call = vi.fn(async () => {});
    await run(
      ["seq", [["set", "count", ["add", ["sig", "count"], ["lit", 1]]], ["call", "POST", "/save", null], ["set", "count", ["lit", 9]]]],
      store,
      call,
    );
    expect(call).toHaveBeenCalledWith("POST", "/save", null);
    expect(store.get("count")).toBe(9);
  });
});
