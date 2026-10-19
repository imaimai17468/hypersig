import { expect, it } from "vitest";
import { Store } from "../src/store.ts";

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

it("records a push as the list replaced and the new item changed", async () => {
  const store = new Store({ todos: [{ done: false }, { done: true }] });
  let seen: ReadonlyMap<string, boolean> | null = null;
  store.subscribe((changes) => {
    seen = changes;
  });
  store.set("todos", [...(store.get("todos") as unknown[]), { done: false }]);
  await Promise.resolve();
  expect(seen).toEqual(
    new Map([
      ["todos", false],
      ["todos.2", true],
    ]),
  );
});

it("writes several fields at once without changing the values read before", async () => {
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
  await Promise.resolve();
  expect(seen).toEqual(
    new Map([
      ["todos.0.title", true],
      ["todos.2.title", true],
    ]),
  );
});

it("records a value that changes kind as changed throughout", async () => {
  const store = new Store({ filter: { kind: "all" } });
  let seen: ReadonlyMap<string, boolean> | null = null;
  store.subscribe((changes) => {
    seen = changes;
  });
  store.set("filter", "all");
  await Promise.resolve();
  expect(seen).toEqual(new Map([["filter", true]]));
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

it("records a write that replaces most items of a list as one change to the list, and an append item by item", async () => {
  const store = new Store({ rows: [1, 2, 3], more: [1] });
  const seen: [string, boolean][][] = [];
  store.subscribe((changes) => seen.push([...changes]));
  store.set("rows", [4, 5, 3]);
  await Promise.resolve();
  store.set("rows", [4]);
  await Promise.resolve();
  store.set("more", [1, 2, 3]);
  await Promise.resolve();
  expect(seen).toEqual([
    [["rows", true]],
    [["rows", true]],
    [
      ["more", false],
      ["more.1", true],
      ["more.2", true],
    ],
  ]);
});
