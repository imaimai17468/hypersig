import { afterEach, expect, it, vi } from "vitest";
import { $, $$, nextFrame, tick } from "../../dom.ts";
import { bound, mount } from "../mount.ts";

afterEach(() => {
  vi.restoreAllMocks();
});

const fragment = (tag: string) =>
  `<div id="frag"><ul><template data-hs-each='["rows",0]'><${tag} data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></${tag}></template></ul></div>`;
const labels = (selector: string) => $$(selector).map((element) => element.textContent);

it("keeps the items of a list and their bindings when a patch keeps its template", async () => {
  const list = `<ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul>`;
  const runtime = mount(`<div id="frag"><p>a</p>${list}</div>`, {
    rows: [1, 2].map((id) => ({ id, label: `r${id}` })),
  });
  await tick();
  const items = $$("#frag li");
  runtime.apply({
    event: "hs-element",
    data: JSON.stringify({ html: `<div id="frag"><p>b</p>${list}</div>` }),
  });
  await tick();
  expect($("#frag p").textContent).toBe("b");
  expect($$("#frag li")).toEqual(items);
  await nextFrame();
  runtime.store.set("rows.1.label", "x");
  runtime.store.set("rows", [...(runtime.store.get("rows") as unknown[]), { id: 3, label: "r3" }]);
  await tick();
  expect(labels("#frag li")).toEqual(["r1", "x", "r3"]);
  expect($("#frag li")).toBe(items[0]);
});

it("does not warn about a node ref name every item of a list carries", async () => {
  const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
  const html = `<div id="frag"><ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,""]'><input data-hs-ref="field"></li></template></ul></div>`;
  const runtime = mount(html, { rows: [1, 2, 3] });
  await tick();
  runtime.apply({ event: "hs-element", data: JSON.stringify({ html }) });
  await tick();
  expect(document.querySelectorAll("#frag input")).toHaveLength(3);
  expect(warn).not.toHaveBeenCalled();
  warn.mockRestore();
});

it("builds the items of a list again when a patch changes its template", async () => {
  const runtime = mount(fragment("li"), { rows: [1, 2].map((id) => ({ id, label: `r${id}` })) });
  await tick();
  runtime.apply({ event: "hs-element", data: JSON.stringify({ html: fragment("b") }) });
  await tick();
  expect(document.querySelectorAll("#frag li")).toHaveLength(0);
  expect(labels("#frag b")).toEqual(["r1", "r2"]);
});

it("forgets the items of a list that a patch removed after the next frame", async () => {
  const runtime = mount(
    `<div id="frag"><ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul></div>`,
    { rows: [1, 2].map((id) => ({ id, label: `r${id}` })) },
  );
  await tick();
  const removed = $("#frag li");
  // An item's bindings live in its row rather than in an `Effect` per binding.
  expect(bound(removed).row).toBeDefined();
  runtime.apply({
    event: "hs-element",
    data: JSON.stringify({ html: `<div id="frag"><p>gone</p></div>` }),
  });
  expect(removed.isConnected).toBe(false);
  await nextFrame();
  expect(bound(removed).row).toBeUndefined();
  expect(bound(removed).effects).toBeUndefined();
  expect(bound(removed).watch).toBeUndefined();
  runtime.store.set("rows.0.label", "x");
  await tick();
  expect($("#frag").textContent).toBe("gone");
});
