import { describe, expect, it, vi } from "vitest";
import { AstError, type Context, evaluate, type Host, run } from "../src/eval.ts";
import { Store } from "../src/store.ts";

function context(values: Record<string, unknown>): Context {
  return { store: new Store(values), scope: [] };
}

function host(): Host & { call: ReturnType<typeof vi.fn> } {
  return { call: vi.fn(async () => {}), dom: vi.fn(), debounce: vi.fn() };
}

describe("evaluate", () => {
  const ctx = context({ count: 3, name: " Tea ", panel: { open: true }, filter: "Active" });

  it("reads signals, nested fields, and literals", () => {
    expect(evaluate(["sig", "count"], ctx)).toBe(3);
    expect(evaluate(["sig", "panel.open"], ctx)).toBe(true);
    expect(evaluate(["lit", "x"], ctx)).toBe("x");
  });

  it("matches the operators expr.rs writes", () => {
    expect(evaluate(["gt", ["add", ["sig", "count"], ["lit", 1]], ["mul", ["sig", "count"], ["lit", 2]]], ctx)).toBe(false);
    expect(evaluate(["concat", ["sig", "name"], ["str", ["sig", "count"]]], ctx)).toBe(" Tea 3");
    expect(evaluate(["rem", ["lit", -7], ["lit", 3]], ctx)).toBe(-1);
    expect(evaluate(["lower", ["trim", ["sig", "name"]]], ctx)).toBe("tea");
    expect(evaluate(["chars", ["lit", "a😀"]], ctx)).toBe(2);
    expect(evaluate(["if", ["eq", ["sig", "filter"], ["lit", "Active"]], ["lit", "a"], ["lit", "b"]], ctx)).toBe("a");
    expect(evaluate(["fixed", ["lit", 2.5], 2], ctx)).toBe("2.50");
    expect(evaluate(["or_else", ["lit", null], ["lit", 4]], ctx)).toBe(4);
  });

  it("builds a tuple into an array, as serde writes one", () => {
    expect(evaluate(["tuple", ["sig", "count"], ["lit", "x"], ["not", ["lit", false]]], ctx)).toEqual([3, "x", true]);
  });

  it("compares records structurally", () => {
    expect(evaluate(["eq", ["lit", { a: [1, { b: 2 }] }], ["lit", { a: [1, { b: 2 }] }]], ctx)).toBe(true);
    expect(evaluate(["ne", ["lit", { a: 1 }], ["lit", { a: 1, b: 2 }]], ctx)).toBe(true);
  });

  it("reads and updates record fields", () => {
    expect(evaluate(["get", ["sig", "panel"], "open"], ctx)).toBe(true);
    expect(evaluate(["with", ["lit", { title: "", done: false }], "title", ["lit", "x"]], ctx)).toEqual({ title: "x", done: false });
  });

  it("counts list items with the item bound at its depth", () => {
    const todos = context({ todos: [{ done: true }, { done: false }, { done: false }] });
    expect(evaluate(["count", "todos", 0, ["not", ["item", 0, "done"]]], todos)).toBe(2);
  });

  it("rejects an operation it does not know", () => {
    expect(() => evaluate(["pow", ["lit", 2], ["lit", 3]], ctx)).toThrow(AstError);
  });
});

describe("run", () => {
  it("sets signals in order and awaits calls", async () => {
    const ctx = context({ count: 0 });
    const h = host();
    await run(
      ["seq", [["set", "count", ["add", ["sig", "count"], ["lit", 1]]], ["call", "POST", "/save", ["lit", 5], "busy", null], ["set", "count", ["lit", 9]]]],
      ctx,
      h,
    );
    expect(h.call).toHaveBeenCalledWith("POST", "/save", 5, "busy", null);
    expect(ctx.store.get("count")).toBe(9);
  });

  it("edits lists", async () => {
    const ctx = context({ todos: [{ t: "a", done: true }, { t: "b", done: false }] });
    const h = host();
    await run(["push", "todos", ["lit", { t: "c", done: false }]], ctx, h);
    await run(["remove", "todos", ["lit", 0]], ctx, h);
    expect(ctx.store.get("todos")).toEqual([{ t: "b", done: false }, { t: "c", done: false }]);
    await run(["set_all", "todos", "done", ["lit", true]], ctx, h);
    await run(["set", ["item", 0, "t"], ["lit", "B"]], { ...ctx, scope: [{ path: "todos.0" }] }, h);
    expect(ctx.store.get("todos")).toEqual([{ t: "B", done: true }, { t: "c", done: true }]);
    await run(["retain", "todos", 0, ["eq", ["item", 0, "t"], ["lit", "c"]]], ctx, h);
    expect(ctx.store.get("todos")).toEqual([{ t: "c", done: true }]);
  });

  it("runs the branch the condition picks", async () => {
    const ctx = context({ n: 10 });
    await run(["if", ["gt", ["sig", "n"], ["lit", 9]], ["set", "n", ["lit", 0]], ["seq", []]], ctx, host());
    expect(ctx.store.get("n")).toBe(0);
  });
});
