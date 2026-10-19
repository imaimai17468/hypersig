import { expect, it } from "vitest";
import { type Context, run } from "../../../src/eval.ts";
import { context, host } from "../context.ts";

const ids = (ctx: Context) => (ctx.store.get("rows") as { id: number }[]).map((row) => row.id);

it("extends a list with the items an expression builds from each index", async () => {
  const ctx = context({ rows: [{ id: 1, label: "b" }], next: 2 });
  await run(
    [
      "extend",
      "rows",
      ["lit", 3],
      0,
      [
        "with",
        ["with", ["lit", { id: 0, label: "" }], "id", ["add", ["sig", "next"], ["index", 0]]],
        "label",
        ["str", ["index", 0]],
      ],
    ],
    ctx,
    host(),
  );
  expect(ctx.store.get("rows")).toEqual([
    { id: 1, label: "b" },
    { id: 2, label: "0" },
    { id: 3, label: "1" },
    { id: 4, label: "2" },
  ]);
});

it("swaps two items, and leaves the list as it is for an index past its end", async () => {
  const ctx = context({ rows: [1, 2, 3, 4].map((id) => ({ id })) });
  const h = host();
  await run(["swap", "rows", ["lit", 0], ["lit", 3]], ctx, h);
  expect(ids(ctx)).toEqual([4, 2, 3, 1]);
  await run(["swap", "rows", ["lit", 0], ["lit", 9]], ctx, h);
  expect(ids(ctx)).toEqual([4, 2, 3, 1]);
});

it("pushes, removes, and appends items", async () => {
  const ctx = context({
    todos: [
      { t: "a", done: true },
      { t: "b", done: false },
    ],
  });
  const h = host();
  await run(["push", "todos", ["lit", { t: "c", done: false }]], ctx, h);
  await run(["remove", "todos", ["lit", 0]], ctx, h);
  expect(ctx.store.get("todos")).toEqual([
    { t: "b", done: false },
    { t: "c", done: false },
  ]);
  ctx.store.set("todos", [{ t: "c", done: true }]);
  await run(
    [
      "append",
      "todos",
      [
        "lit",
        [
          { t: "d", done: false },
          { t: "e", done: false },
        ],
      ],
    ],
    ctx,
    h,
  );
  expect((ctx.store.get("todos") as { t: string }[]).map((todo) => todo.t)).toEqual([
    "c",
    "d",
    "e",
  ]);
});
