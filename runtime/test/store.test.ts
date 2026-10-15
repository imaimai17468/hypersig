import { expect, it } from "vitest";
import { affects, Store } from "../src/store.ts";

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
