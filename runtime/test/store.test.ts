import { describe, expect, it } from "vitest";
import { affects, ChangeLookup, setPath, Store } from "../src/store.ts";

it("setPath copies each container on the way and leaves the value it was given unchanged", () => {
  const value = { rows: [{ id: 1, tags: ["a"] }], name: "n" };
  const next = setPath(value, "rows.0.tags.1", "b") as typeof value;
  expect(next).toEqual({ rows: [{ id: 1, tags: ["a", "b"] }], name: "n" });
  expect(value).toEqual({ rows: [{ id: 1, tags: ["a"] }], name: "n" });
  expect(next.rows).not.toBe(value.rows);
  expect(setPath(null, "a.b", 1)).toEqual({ a: { b: 1 } });
  expect(setPath(value, "", 2)).toBe(2);
});

it("getIn reads a field inside the value at a path and records it under the given name", () => {
  const store = new Store({ rows: [{ label: "x" }] });
  let value: unknown;
  const reads = store.track(() => {
    value = store.getIn("rows.0", "label", "@3");
  });
  expect(value).toBe("x");
  expect([...reads]).toEqual([["@3.label", true]]);
  expect(store.getIn("rows.0", "", "@3")).toEqual({ label: "x" });
});

it("a shallow read depends on the path and its ancestors, a deep read also on what is inside", () => {
  const shallow = new Map([["todos", false]]);
  const deep = new Map([["todos", true]]);
  expect(affects(shallow, new Map([["todos.5.done", true]]))).toBe(false);
  expect(affects(deep, new Map([["todos.5.done", true]]))).toBe(true);
  expect(affects(shallow, new Map([["todos", true]]))).toBe(true);
  expect(affects(new Map([["page.todos", false]]), new Map([["page", true]]))).toBe(true);
  expect(affects(shallow, new Map([["todosx", true]]))).toBe(false);
});

it("a replaced container affects the reads at its path and the deep reads above it", () => {
  const changes = new Map([["page.todos", false]]);
  expect(affects(new Map([["page.todos", false]]), changes)).toBe(true);
  expect(affects(new Map([["page", true]]), changes)).toBe(true);
  expect(affects(new Map([["page", false]]), changes)).toBe(false);
  expect(affects(new Map([["page.todos.0.done", true]]), changes)).toBe(false);
  expect(affects(new Map([["page.todos.0", false]]), changes)).toBe(false);
});

it("records a push as the list replaced and the new item changed", () => {
  const store = new Store({ todos: [{ done: false }, { done: true }] });
  let seen: ReadonlyMap<string, boolean> | null = null;
  store.subscribe((changes) => {
    seen = changes;
  });
  store.set("todos", [...(store.get("todos") as unknown[]), { done: false }]);
  return Promise.resolve().then(() => {
    expect(seen).toEqual(new Map([["todos", false], ["todos.2", true]]));
  });
});

it("writes several fields at once without changing the values read before", () => {
  const store = new Store({ todos: [{ title: "a" }, { title: "b" }, { title: "c" }] });
  const before = store.get("todos") as { title: string }[];
  let seen: ReadonlyMap<string, boolean> | null = null;
  store.subscribe((changes) => {
    seen = changes;
  });
  store.setMany([
    ["todos.0.title", "A"],
    ["todos.2.title", "C"],
    ["todos.1.title", "b"],
  ]);
  expect(store.get("todos")).toEqual([{ title: "A" }, { title: "b" }, { title: "C" }]);
  expect(before).toEqual([{ title: "a" }, { title: "b" }, { title: "c" }]);
  expect((store.get("todos") as unknown[])[1]).toBe(before[1]);
  return Promise.resolve().then(() => {
    expect(seen).toEqual(
      new Map([
        ["todos.0.title", true],
        ["todos.2.title", true],
      ]),
    );
  });
});

it("records a value that changes kind as changed throughout", () => {
  const store = new Store({ filter: { kind: "all" } });
  let seen: ReadonlyMap<string, boolean> | null = null;
  store.subscribe((changes) => {
    seen = changes;
  });
  store.set("filter", "all");
  return Promise.resolve().then(() => {
    expect(seen).toEqual(new Map([["filter", true]]));
  });
});

it("a deep read of a path wins over a shallow read of it in the same binding", () => {
  const store = new Store({ todos: [] });
  const reads = store.track(() => {
    store.getShallow("todos");
    store.get("todos");
    store.getShallow("todos");
  });
  expect(reads.get("todos")).toBe(true);
});

describe("ChangeLookup", () => {
  it("answers as affects does for one read", () => {
    const paths = ["a", "b", "a.0", "a.1", "a.0.x", "a.0.y", "a.1.x", "b.c", "b.c.d"];
    const cases: [string, boolean][][] = [
      [["a", false], ["a.0", true]],
      [["a", true]],
      [["a.0.x", true]],
      [["b.c", false], ["b.c.d", true], ["a.1", true]],
    ];
    for (const entries of cases) {
      const changes = new Map(entries);
      const lookup = new ChangeLookup(changes);
      for (const read of paths) {
        for (const deep of [true, false]) {
          expect([read, deep, lookup.affects(read, deep)]).toEqual([read, deep, affects(new Map([[read, deep]]), changes)]);
        }
      }
    }
  });
});
