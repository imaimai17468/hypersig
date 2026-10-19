import { expect, it } from "vitest";
import { type Context, run } from "../../../src/eval.ts";
import { context, host } from "../context.ts";

const ids = (ctx: Context) => (ctx.store.get("rows") as { id: number }[]).map((row) => row.id);

it("updates a field of the items the condition picks", async () => {
  const ctx = context({ rows: ["b", "0", "1", "2"].map((label, at) => ({ id: at + 1, label })) });
  await run(
    [
      "update",
      "rows",
      0,
      ["eq", ["rem", ["index", 0], ["lit", 2]], ["lit", 0]],
      "label",
      ["concat", ["item", 0, "label"], ["lit", "!"]],
    ],
    ctx,
    host(),
  );
  expect((ctx.store.get("rows") as { label: string }[]).map((row) => row.label)).toEqual([
    "b!",
    "0",
    "1!",
    "2",
  ]);
});

it("sorts items by a key, in either direction", async () => {
  const labels = new Map([
    [1, "b!"],
    [2, "0"],
    [3, "1!"],
    [4, "2"],
  ]);
  const ctx = context({ rows: [4, 2, 3, 1].map((id) => ({ id, label: labels.get(id) })) });
  const h = host();
  await run(["sort", "rows", 0, ["item", 0, "id"], false], ctx, h);
  expect(ids(ctx)).toEqual([1, 2, 3, 4]);
  await run(["sort", "rows", 0, ["item", 0, "id"], true], ctx, h);
  expect(ids(ctx)).toEqual([4, 3, 2, 1]);
  await run(["sort", "rows", 0, ["chars", ["item", 0, "label"]], true], ctx, h);
  expect(ids(ctx)).toEqual([3, 1, 4, 2]);
});

it("sets a field of every item and of the item in scope, and keeps the items the condition picks", async () => {
  const ctx = context({
    todos: [
      { t: "b", done: false },
      { t: "c", done: false },
    ],
  });
  const h = host();
  await run(["set_all", "todos", "done", ["lit", true]], ctx, h);
  await run(["set", ["item", 0, "t"], ["lit", "B"]], { ...ctx, scope: [{ path: "todos.0" }] }, h);
  expect(ctx.store.get("todos")).toEqual([
    { t: "B", done: true },
    { t: "c", done: true },
  ]);
  await run(["retain", "todos", 0, ["eq", ["item", 0, "t"], ["lit", "c"]]], ctx, h);
  expect(ctx.store.get("todos")).toEqual([{ t: "c", done: true }]);
});
