import { expect, it, vi } from "vitest";
import { $$, tick } from "../../dom.ts";
import { mount, spyReads } from "../mount.ts";

const list = (id: string) =>
  `<ul id="${id}"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul>`;
const labels = (id: string) => $$(`#${id} li`).map((li) => li.textContent);
const items = () => $$("#l > li");
const tagNames = (item: Element) => [...item.querySelectorAll("i")].map((i) => i.textContent);

it("keeps the bindings of items that shift unchanged, and reruns the bindings that read their position", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
      `<b data-hs-text='["str",["index",0]]'></b><i data-hs-text='["item",0,"label"]'></i>` +
      `<s data-hs-text='["concat",["item",0,"label"],["sig","suffix"]]'></s></li></template></ul>`,
    { rows: [1, 2, 3, 4].map((id) => ({ id, label: `r${id}` })), suffix: "" },
  );
  await tick();
  const [first, ...rest] = items();
  const get = runtime.store.get.bind(runtime.store);
  const reads = spyReads(runtime);
  runtime.store.set("rows", (get("rows") as unknown[]).slice(1));
  await tick();
  expect(items()).toEqual(rest);
  expect(first!.isConnected).toBe(false);
  expect(rest.map((li) => li.querySelector("b")!.textContent)).toEqual(["0", "1", "2"]);
  expect(reads.filter((path) => path.endsWith(".label"))).toEqual([]);

  runtime.store.set("suffix", "!");
  runtime.store.set("rows.0.label", "x");
  await tick();
  expect(rest.map((li) => li.querySelector("s")!.textContent)).toEqual(["x!", "r3!", "r4!"]);
  expect(rest.map((li) => li.querySelector("i")!.textContent)).toEqual(["x", "r3", "r4"]);
});

it("moves the items between the ones that keep their place at either end", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["str",["item",0,"id"]]'></li></template></ul>`,
    { rows: [1, 2, 3, 4, 5, 6].map((id) => ({ id })) },
  );
  await tick();
  const byId = new Map(items().map((li) => [li.textContent, li]));
  const rows = runtime.store.get("rows") as unknown[];
  runtime.store.set(
    "rows",
    [0, 1, 4, 3, 2, 5].map((at) => rows[at]),
  );
  await tick();
  const after = items();
  expect(after.map((li) => li.textContent)).toEqual(["1", "2", "5", "4", "3", "6"]);
  expect(after).toEqual(["1", "2", "5", "4", "3", "6"].map((id) => byId.get(id)));
});

it("reaches the bindings of an item that moved through a write at its new path", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
      `<b data-hs-text='["item",0,"label"]'></b><input type="checkbox" data-hs-bind='[["item",0,"done"],"bool"]'>` +
      `<ol><template data-hs-each='[["item",0,"tags"],1]'><i data-hs-key='["item",1,"id"]' data-hs-text='["item",1,"name"]'></i></template></ol>` +
      `</li></template></ul>`,
    {
      rows: [1, 2, 3].map((id) => ({
        id,
        label: `r${id}`,
        done: false,
        tags: [
          { id: 1, name: `t${id}` },
          { id: 2, name: `u${id}` },
        ],
      })),
    },
  );
  await tick();
  const [, second, third] = items();
  runtime.store.set("rows", (runtime.store.get("rows") as unknown[]).slice(1));
  await tick();
  expect(items()).toEqual([second, third]);

  runtime.store.set("rows.0.label", "x");
  runtime.store.set("rows.1.done", true);
  runtime.store.set("rows.1.tags.1.name", "y");
  await tick();
  expect(second!.querySelector("b")!.textContent).toBe("x");
  expect(third!.querySelector("b")!.textContent).toBe("r3");
  expect(third!.querySelector("input")!.checked).toBe(true);
  expect(second!.querySelector("input")!.checked).toBe(false);
  expect(tagNames(third!)).toEqual(["t3", "y"]);
  expect(tagNames(second!)).toEqual(["t2", "u2"]);

  runtime.store.set("rows.1.tags", [{ id: 2, name: "y" }]);
  await tick();
  expect(tagNames(third!)).toEqual(["y"]);
  runtime.store.set("rows", (runtime.store.get("rows") as unknown[]).toReversed());
  await tick();
  runtime.store.set("rows.0.tags.0.name", "z");
  await tick();
  expect(items()).toEqual([third, second]);
  expect(tagNames(third!)).toEqual(["z"]);
});

it("reaches the items of every list over one signal through a write at an item's path", async () => {
  const runtime = mount(list("a") + list("b"), {
    rows: [1, 2, 3].map((id) => ({ id, label: `r${id}` })),
  });
  await tick();
  runtime.store.set("rows", (runtime.store.get("rows") as unknown[]).slice(1));
  await tick();
  runtime.store.set("rows.0.label", "x");
  await tick();
  expect(labels("a")).toEqual(["x", "r3"]);
  expect(labels("b")).toEqual(["x", "r3"]);
});

const insertBefore = Node.prototype.insertBefore;

/**
 * Makes `insertBefore` blur a focused element inside the node it moves. A
 * browser takes a moved element out of the document first, which blurs it,
 * where the test DOM keeps the focus.
 */
function blurOnMove() {
  return vi.spyOn(Node.prototype, "insertBefore").mockImplementation(function <T extends Node>(
    this: Node,
    node: T,
    child: Node | null,
  ): T {
    const focused = document.activeElement;
    if (focused instanceof HTMLElement && node.contains(focused)) focused.blur();
    return insertBefore.call(this, node, child) as T;
  });
}

const reorderable = () =>
  mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'><input></li></template></ul>`,
    { rows: [1, 2, 3].map((id) => ({ id })) },
  );

it("focuses a field again with its caret where a reorder moved its item with insertBefore", async () => {
  const spy = blurOnMove();
  try {
    const runtime = reorderable();
    await tick();
    // The reorder keeps the item with id 1 in place and moves the other two around it.
    const field = items()[2]!.querySelector("input")!;
    field.value = "hello";
    field.focus();
    field.setSelectionRange(2, 4);
    runtime.store.set(
      "rows",
      [3, 2, 1].map((id) => ({ id })),
    );
    await tick();
    expect(items()[0]!.querySelector("input")).toBe(field);
    expect(document.activeElement).toBe(field);
    expect([field.selectionStart, field.selectionEnd]).toEqual([2, 4]);
  } finally {
    spy.mockRestore();
  }
});

it("moves a reordered item with moveBefore where the browser has it", async () => {
  const moved: Node[] = [];
  Object.defineProperty(Element.prototype, "moveBefore", {
    configurable: true,
    value(this: Element, node: Node, child: Node | null): void {
      moved.push(node);
      this.insertBefore(node, child);
    },
  });
  try {
    const runtime = reorderable();
    await tick();
    const first = items()[0]!;
    runtime.store.set(
      "rows",
      [3, 2, 1].map((id) => ({ id })),
    );
    await tick();
    expect(items().at(-1)).toBe(first);
    expect(moved.length).toBeGreaterThan(0);
  } finally {
    Reflect.deleteProperty(Element.prototype, "moveBefore");
  }
});
