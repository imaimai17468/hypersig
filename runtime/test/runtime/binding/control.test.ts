import { expect, it } from "vitest";
import { $, $$, tick } from "../../dom.ts";
import { mount } from "../mount.ts";

const inputs = () => $$("#l input") as HTMLInputElement[];

it("binds an input in both directions", async () => {
  const runtime = mount(`<input id="i" data-hs-bind='["name","text"]'>`, { name: "a" });
  const input = $<HTMLInputElement>("#i");
  expect(input.value).toBe("a");
  input.value = "ab";
  input.dispatchEvent(new Event("input", { bubbles: true }));
  expect(runtime.store.get("name")).toBe("ab");
  runtime.store.set("name", "");
  await tick();
  expect(input.value).toBe("");
});

it("binds an input inside a list item in both directions", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
      `<input data-hs-bind='[["item",0,"title"],"text"]'></li></template></ul>`,
    { rows: [1, 2].map((id) => ({ id, title: `t${id}` })) },
  );
  await tick();
  expect(inputs().map((input) => input.value)).toEqual(["t1", "t2"]);
  inputs()[1]!.value = "typed";
  inputs()[1]!.dispatchEvent(new Event("input", { bubbles: true }));
  expect(runtime.store.get("rows.1.title")).toBe("typed");
  runtime.store.set("rows.0.title", "set");
  await tick();
  expect(inputs().map((input) => input.value)).toEqual(["set", "typed"]);
});

it("leaves a JSON-bound signal as it was when the control holds text that is not JSON", () => {
  const runtime = mount(`<input id="i" data-hs-bind='["choice","json"]'>`, { choice: 1 });
  const input = $<HTMLInputElement>("#i");
  input.value = "two";
  input.dispatchEvent(new Event("change", { bubbles: true }));
  expect(runtime.store.get("choice")).toBe(1);
  input.value = "2";
  input.dispatchEvent(new Event("change", { bubbles: true }));
  expect(runtime.store.get("choice")).toBe(2);
});
