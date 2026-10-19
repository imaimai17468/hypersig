import { expect, it } from "vitest";
import { $$, tick } from "../../dom.ts";
import { mount } from "../mount.ts";

const list = (key: string) =>
  `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='${key}' data-hs-text='["item",0,"label"]'></li></template></ul>`;
const items = () => $$("#l li");

it("evaluates again the key of an unchanged item where the key reads more than the item", async () => {
  const byPrefix = mount(list('["concat",["sig","prefix"],["str",["item",0,"id"]]]'), {
    rows: [1, 2, 3].map((id) => ({ id, label: `r${id}` })),
    prefix: "a",
  });
  await tick();
  const before = items();
  const rows = byPrefix.store.get("rows") as unknown[];
  byPrefix.store.set("rows", rows.toReversed());
  await tick();
  expect(items()).toEqual(before.toReversed());
  byPrefix.store.set("prefix", "b");
  byPrefix.store.set("rows", rows);
  await tick();
  expect(items().map((li) => li.textContent)).toEqual(["r1", "r2", "r3"]);
  expect(items().some((li) => before.includes(li))).toBe(false);

  const byIndex = mount(list('["index",0]'), {
    rows: [1, 2, 3].map((id) => ({ id, label: `r${id}` })),
  });
  await tick();
  const placed = items();
  byIndex.store.set("rows", (byIndex.store.get("rows") as unknown[]).toReversed());
  await tick();
  expect(items()).toEqual(placed);
  expect(items().map((li) => li.textContent)).toEqual(["r3", "r2", "r1"]);
});

it("matches only the positions a write replaced, and keeps the other items and their bindings", async () => {
  const runtime = mount(list('["item",0,"id"]'), {
    rows: [1, 2, 3, 4, 5, 6].map((id) => ({ id, label: `r${id}` })),
  });
  await tick();
  const before = items();
  const rows = runtime.store.get("rows") as { id: number; label: string }[];
  runtime.store.set("rows", [rows[0], rows[4], rows[2], rows[3], rows[1], rows[5]]);
  await tick();
  expect(items()).toEqual([0, 4, 2, 3, 1, 5].map((at) => before[at]));

  const swapped = runtime.store.get("rows") as { id: number; label: string }[];
  runtime.store.set(
    "rows",
    swapped.with(2, { id: 3, label: "x" }).with(3, { id: 9, label: "new" }),
  );
  await tick();
  const after = items();
  expect(after.map((li) => li.textContent)).toEqual(["r1", "r5", "x", "new", "r2", "r6"]);
  expect(after[2]).toBe(before[2]);
  expect(before.includes(after[3]!)).toBe(false);
  expect(before[3]!.isConnected).toBe(false);

  runtime.store.set("rows.0.label", "first");
  await tick();
  expect(items()[0]!.textContent).toBe("first");
});
