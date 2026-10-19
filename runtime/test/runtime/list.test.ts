import { expect, it } from "vitest";
import { $, $$, tick } from "../dom.ts";
import { mount } from "./mount.ts";

const texts = (id: string) => $$(`#${id} li`).map((li) => li.textContent);

it("removes every item at once where the list fills its parent, keeping the template to render into again", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul>` +
      `<ol id="m"><li>head</li><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ol>`,
    { rows: [1, 2, 3].map((id) => ({ id, label: `r${id}` })) },
  );
  await tick();
  runtime.store.set(
    "rows",
    [4, 5].map((id) => ({ id, label: `r${id}` })),
  );
  await tick();
  expect(texts("l")).toEqual(["r4", "r5"]);
  expect(texts("m")).toEqual(["head", "r4", "r5"]);
  runtime.store.set("rows", []);
  await tick();
  expect($("#l").children.length).toBe(1);
  expect(texts("m")).toEqual(["head"]);
  runtime.store.set("rows", [{ id: 6, label: "r6" }]);
  await tick();
  expect(texts("l")).toEqual(["r6"]);
  expect(texts("m")).toEqual(["head", "r6"]);
});
