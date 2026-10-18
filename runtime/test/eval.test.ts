import { describe, expect, it, vi } from "vitest";
import { AstError, type Context, evaluate, type Host, run } from "../src/eval.ts";
import { Store } from "../src/store.ts";

function context(values: Record<string, unknown>): Context {
  return { store: new Store(values), scope: [] };
}

function host() {
  return {
    call: vi.fn(async (..._call: Parameters<Host["call"]>) => {}),
    dom: vi.fn(),
    clearFiles: vi.fn(),
    navigate: vi.fn((_url: string) => {}),
    history: vi.fn((_url: string, _replace: boolean) => {}),
    debounce: vi.fn(),
  } satisfies Host;
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
    expect(evaluate(["fixed", ["lit", 2.5], 0], ctx)).toBe("3");
    expect(evaluate(["or_else", ["lit", null], ["lit", 4]], ctx)).toBe(4);
  });

  it("formats numbers as ssr.rs does", () => {
    const plain = (value: number) => evaluate(["format", ["lit", value], 0, ",", ".", "", ""], ctx);
    expect(plain(0)).toBe("0");
    expect(plain(999)).toBe("999");
    expect(plain(1000)).toBe("1,000");
    expect(plain(-1234567)).toBe("-1,234,567");
    expect(plain(-0.4)).toBe("0");
    expect(evaluate(["format", ["lit", -1234.5], 2, ",", ".", "$", ""], ctx)).toBe("-$1,234.50");
    expect(evaluate(["format", ["lit", 1234567.891], 2, ".", ",", "", " €"], ctx)).toBe("1.234.567,89 €");
    expect(evaluate(["format", ["lit", 12345], 0, "", ".", "", "%"], ctx)).toBe("12345%");
    expect(evaluate(["format", ["lit", Number.NaN], 0, ",", ".", "¥", ""], ctx)).toBe("¥NaN");
    expect(evaluate(["format", ["lit", 2e21], 0, ",", ".", "", ""], ctx)).toBe("2e+21");
  });

  it("writes dates in a fixed offset and relative times as date.rs does", () => {
    const at = Date.UTC(2026, 0, 5, 15, 4, 5);
    expect(evaluate(["date", ["lit", at], "en", "medium", "short", 0], ctx)).toBe("Jan 5, 2026, 3:04 PM");
    expect(evaluate(["date", ["lit", at], "ja", null, "medium", 540], ctx)).toBe("0:04:05");
    expect(evaluate(["date", ["lit", at], "en", "long", null, -300], ctx)).toBe("January 5, 2026");
    const ago = (millis: number, locale = "en") => evaluate(["reltime", ["lit", at + millis], ["lit", at], locale], ctx);
    expect(ago(-500)).toBe("0 seconds ago");
    expect(ago(59_999)).toBe("in 59 seconds");
    expect(ago(-3_600_000)).toBe("1 hour ago");
    expect(ago(29 * 86_400_000)).toBe("in 29 days");
    expect(ago(-31 * 86_400_000, "ja")).toBe("1 か月前");
    expect(ago(400 * 86_400_000, "ja")).toBe("1 年後");
  });

  it("picks the plural form as ssr.rs does", () => {
    const forms = ["z", "1", "2", "f", "m", "o"].map((form) => ["lit", form]);
    const plural = (locale: string, n: number) => evaluate(["plural", ["lit", n], locale, forms], ctx);
    expect([1, -1, 0, 1.5, 2].map((n) => plural("en", n))).toEqual(["1", "1", "o", "o", "o"]);
    expect(plural("ja", 1)).toBe("o");
  });

  it("tests characters by ASCII class as ssr.rs does", () => {
    expect(evaluate(["chars_all", ["lit", "0123"], "digit"], ctx)).toBe(true);
    expect(evaluate(["chars_all", ["lit", ""], "digit"], ctx)).toBe(true);
    expect(evaluate(["chars_any", ["lit", ""], "digit"], ctx)).toBe(false);
    expect(evaluate(["chars_all", ["lit", "１"], "digit"], ctx)).toBe(false);
    expect(evaluate(["chars_all", ["lit", "aF9"], "hex"], ctx)).toBe(true);
    expect(evaluate(["chars_any", ["lit", "a\vb"], "space"], ctx)).toBe(false);
    expect(evaluate(["chars_any", ["lit", "a\fb"], "space"], ctx)).toBe(true);
    expect(evaluate(["chars_all", ["lit", "aZ"], "alpha"], ctx)).toBe(true);
    expect(evaluate(["chars_any", ["lit", "😀"], "alnum"], ctx)).toBe(false);
    expect(() => evaluate(["chars_all", ["lit", "a"], "emoji"], ctx)).toThrow(AstError);
  });

  it("names a value once for its body", () => {
    const rows = context({ rows: [{ v: 3 }, { v: 7 }] });
    const best = ["max_of", "rows", 0, ["item", 0, "v"]];
    expect(evaluate(["let", best, 0, ["add", ["var", 0], ["var", 0]]], rows)).toBe(14);
    expect(evaluate(["let", ["lit", 2], 0, ["let", ["lit", 5], 1, ["sub", ["var", 1], ["var", 0]]]], rows)).toBe(3);
  });

  it("folds a list and rounds as ssr.rs does", () => {
    const rows = context({ rows: [{ v: 3 }, { v: -1.5 }, { v: 7 }], none: [] });
    const fold = (op: string, list: string) => evaluate([op, list, 0, ["item", 0, "v"]], rows);
    expect(fold("sum_of", "rows")).toBe(8.5);
    expect(fold("max_of", "rows")).toBe(7);
    expect(fold("min_of", "rows")).toBe(-1.5);
    expect(fold("sum_of", "none")).toBe(0);
    expect(fold("max_of", "none")).toBe(null);
    expect(fold("map", "rows")).toEqual([3, -1.5, 7]);
    expect(evaluate(["floor", ["lit", -2.5]], ctx)).toBe(-3);
    expect(evaluate(["ceil", ["lit", 2.1]], ctx)).toBe(3);
    expect(evaluate(["round", ["lit", 2.5]], ctx)).toBe(3);
    expect(evaluate(["round", ["lit", -2.5]], ctx)).toBe(-2);
    expect(evaluate(["pow", ["lit", 10], ["lit", 2]], ctx)).toBe(100);
    expect(evaluate(["log10", ["lit", 1000]], ctx)).toBe(3);
  });

  it("builds a tuple into an array, as serde writes one", () => {
    expect(evaluate(["tuple", ["sig", "count"], ["lit", "x"], ["not", ["lit", false]]], ctx)).toEqual([3, "x", true]);
  });

  it("draws random integers below the bound, reads the clock, and indexes lists", () => {
    for (let k = 0; k < 50; k++) {
      const value = evaluate(["random", ["lit", 3]], ctx) as number;
      expect(Number.isInteger(value) && value >= 0 && value < 3).toBe(true);
    }
    expect(Math.abs((evaluate(["now"], ctx) as number) - Date.now())).toBeLessThan(1000);
    expect(evaluate(["nth", ["lit", ["a", "b"]], ["lit", 1]], ctx)).toBe("b");
    expect(evaluate(["nth", ["lit", ["a", "b"]], ["lit", 5]], ctx)).toBe(null);
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
    expect(() => evaluate(["cube", ["lit", 2]], ctx)).toThrow(AstError);
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
    expect(h.call).toHaveBeenCalledWith("POST", "/save", expect.any(Function), "busy", null, {
      latest: false,
      reconnect: false,
      connected: false,
      origin: undefined,
    });
    expect(h.call.mock.calls[0]![2]()).toBe(5);
    expect(ctx.store.get("count")).toBe(9);
  });

  it("passes the flags after the error signal to the call", async () => {
    const h = host();
    await run(["call", "GET", "/find", ["lit", "tea"], null, null, ["latest", "reconnect", "connected"]], context({}), h);
    expect(h.call).toHaveBeenCalledWith("GET", "/find", expect.any(Function), null, null, {
      latest: true,
      reconnect: true,
      connected: true,
      origin: undefined,
    });
  });

  it("extends, updates, swaps, and sorts lists", async () => {
    const ctx = context({ rows: [{ id: 1, label: "b" }], next: 2 });
    const h = host();
    await run(["extend", "rows", ["lit", 3], 0, ["with", ["with", ["lit", { id: 0, label: "" }], "id", ["add", ["sig", "next"], ["index", 0]]], "label", ["str", ["index", 0]]]], ctx, h);
    expect(ctx.store.get("rows")).toEqual([
      { id: 1, label: "b" },
      { id: 2, label: "0" },
      { id: 3, label: "1" },
      { id: 4, label: "2" },
    ]);
    await run(["update", "rows", 0, ["eq", ["rem", ["index", 0], ["lit", 2]], ["lit", 0]], "label", ["concat", ["item", 0, "label"], ["lit", "!"]]], ctx, h);
    expect((ctx.store.get("rows") as { label: string }[]).map((row) => row.label)).toEqual(["b!", "0", "1!", "2"]);
    await run(["swap", "rows", ["lit", 0], ["lit", 3]], ctx, h);
    expect((ctx.store.get("rows") as { id: number }[]).map((row) => row.id)).toEqual([4, 2, 3, 1]);
    await run(["swap", "rows", ["lit", 0], ["lit", 9]], ctx, h);
    expect((ctx.store.get("rows") as { id: number }[]).map((row) => row.id)).toEqual([4, 2, 3, 1]);
    await run(["sort", "rows", 0, ["item", 0, "id"], false], ctx, h);
    expect((ctx.store.get("rows") as { id: number }[]).map((row) => row.id)).toEqual([1, 2, 3, 4]);
    await run(["sort", "rows", 0, ["item", 0, "id"], true], ctx, h);
    expect((ctx.store.get("rows") as { id: number }[]).map((row) => row.id)).toEqual([4, 3, 2, 1]);
    await run(["sort", "rows", 0, ["chars", ["item", 0, "label"]], true], ctx, h);
    expect((ctx.store.get("rows") as { id: number }[]).map((row) => row.id)).toEqual([3, 1, 4, 2]);
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
    await run(["append", "todos", ["lit", [{ t: "d", done: false }, { t: "e", done: false }]]], ctx, h);
    expect((ctx.store.get("todos") as { t: string }[]).map((todo) => todo.t)).toEqual(["c", "d", "e"]);
  });

  it("encodes each part of a rest of a path and keeps its slashes", () => {
    expect(evaluate(["uri_path", ["lit", "a b/c?/d"]], context({}))).toBe("a%20b/c%3F/d");
  });

  it("hands the evaluated URL to the host to navigate", async () => {
    const h = host();
    await run(["navigate", ["concat", ["lit", "/members/"], ["uri", ["sig", "id"]]]], context({ id: "a b" }), h);
    expect(h.navigate).toHaveBeenCalledWith("/members/a%20b");
  });

  it("hands the evaluated URL to the host to push or replace in the address bar", async () => {
    const h = host();
    const url = ["concat", ["lit", "/fruit"], ["query", ["lit", { q: "a b" }]]];
    await run(["history", "push", url], context({}), h);
    await run(["history", "replace", url], context({}), h);
    expect(h.history.mock.calls).toEqual([
      ["/fruit?q=a%20b", false],
      ["/fruit?q=a%20b", true],
    ]);
  });

  it("runs the branch the condition picks", async () => {
    const ctx = context({ n: 10 });
    await run(["if", ["gt", ["sig", "n"], ["lit", 9]], ["set", "n", ["lit", 0]], ["seq", []]], ctx, host());
    expect(ctx.store.get("n")).toBe(0);
  });
});
