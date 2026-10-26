import { expect, it } from "vitest";
import { $, $$, tick, type } from "../../dom.ts";
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

/** A form with a text field bound to `title`, focused, after the visitor typed `typed` into it. */
function typing(typed: string[]): [ReturnType<typeof mount>, HTMLInputElement] {
  const runtime = mount(`<form id="f"><input id="i" data-hs-bind='["title","text"]'></form>`, {
    title: "",
  });
  const input = $<HTMLInputElement>("#i");
  input.focus();
  for (const text of typed) type(input, text);
  return [runtime, input];
}

it("keeps the text being typed when an answer repeats an older value of it", async () => {
  const [runtime, input] = typing(["H", "He", "Hel", "Hell", "Hello"]);
  input.setSelectionRange(5, 5);
  runtime.store.merge({ title: "Hel" });
  await tick();
  expect(input.value).toBe("Hello");
  expect(runtime.store.get("title")).toBe("Hello");
  expect([input.selectionStart, input.selectionEnd]).toEqual([5, 5]);
});

it("shows a value the visitor did not type at once, while the visitor is typing", async () => {
  const [runtime, input] = typing(["w", "wr", "write"]);
  runtime.store.merge({ title: "" });
  await tick();
  expect(input.value).toBe("");
  runtime.store.merge({ title: "Japan" });
  await tick();
  expect(input.value).toBe("Japan");
  expect(document.activeElement).toBe(input);
});

it("clears a field at once after a change event commits its text", async () => {
  const [runtime, input] = typing(["H", "Hi"]);
  type(input, "Hi", "change");
  runtime.store.merge({ title: "" });
  await tick();
  expect(input.value).toBe("");
  expect(document.activeElement).toBe(input);
});

it("clears a field the visitor submitted at once, while it keeps focus", async () => {
  const [runtime, input] = typing(["H", "Hi"]);
  $("#f").dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
  runtime.store.merge({ title: "" });
  await tick();
  expect(input.value).toBe("");
  expect(document.activeElement).toBe(input);
});

it("shows a new value from the server when the form is submitted, before its handler runs", async () => {
  const [runtime, input] = typing(["H", "Hi"]);
  runtime.store.merge({ title: "Renamed" });
  await tick();
  $("#f").dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
  expect(input.value).toBe("Renamed");
  expect(runtime.store.get("title")).toBe("Renamed");
});

it("keeps the caret when a value reaches a focused control the visitor has not typed in", async () => {
  const runtime = mount(`<input id="i" data-hs-bind='["title","text"]'>`, { title: "Hello" });
  const input = $<HTMLInputElement>("#i");
  input.focus();
  input.setSelectionRange(2, 2);
  runtime.store.merge({ title: "Hello, world" });
  await tick();
  expect(input.value).toBe("Hello, world");
  expect([input.selectionStart, input.selectionEnd]).toEqual([2, 2]);
  runtime.store.merge({ title: "H" });
  await tick();
  expect([input.selectionStart, input.selectionEnd]).toEqual([1, 1]);
});
