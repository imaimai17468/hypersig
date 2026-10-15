import { expect, it } from "vitest";
import { affects, Store } from "../src/store.ts";

it("a shallow read depends on the path and its ancestors, a deep read also on what is inside", () => {
  const shallow = new Map([["todos", false]]);
  const deep = new Map([["todos", true]]);
  expect(affects(shallow, new Set(["todos.5.done"]))).toBe(false);
  expect(affects(deep, new Set(["todos.5.done"]))).toBe(true);
  expect(affects(shallow, new Set(["todos"]))).toBe(true);
  expect(affects(new Map([["page.todos", false]]), new Set(["page"]))).toBe(true);
  expect(affects(shallow, new Set(["todosx"]))).toBe(false);
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
