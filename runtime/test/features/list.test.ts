import { expect, it } from "vitest";
import { $, $$, key, tick, type } from "../dom.ts";
import { mount } from "./fixture.ts";

it("renders items from a list signal and adds one on Enter", async () => {
  const runtime = mount("todos");
  expect($$("#list li").map((li) => li.textContent)).toEqual(["tea×"]);
  expect($("#left").textContent).toBe("1");

  type($<HTMLInputElement>("#draft"), "  milk ");
  key($("#draft"), "Enter");
  await tick();
  expect(runtime.store.get("todos")).toEqual([
    { id: 1, title: "tea", done: false },
    { id: 2, title: "milk", done: false },
  ]);
  expect($$("#list li label").map((label) => label.textContent)).toEqual(["tea", "milk"]);
  expect($<HTMLInputElement>("#draft").value).toBe("");
  expect($("#left").textContent).toBe("2");
});

it("binds a field of each item and keeps elements by key when an item before them goes", async () => {
  const runtime = mount("todos");
  runtime.store.set("todos", [
    { id: 1, title: "a", done: false },
    { id: 2, title: "b", done: false },
  ]);
  await tick();
  const second = $$("#list li")[1]!;
  const box = second.querySelector<HTMLInputElement>("input[type=checkbox]")!;
  box.checked = true;
  box.dispatchEvent(new Event("change", { bubbles: true }));
  await tick();
  expect(runtime.store.get("todos.1.done")).toBe(true);
  expect(second.classList.contains("done")).toBe(true);
  expect($("#clear").style.display).toBe("");

  $$("#list li .remove")[0]!.click();
  await tick();
  expect($$("#list li")).toEqual([second]);
  second.querySelector<HTMLInputElement>("input[type=checkbox]")!.click();
  await tick();
  expect(runtime.store.get("todos")).toEqual([{ id: 2, title: "b", done: false }]);
});

it("clears completed items and toggles all", async () => {
  const runtime = mount("todos");
  const all = $<HTMLInputElement>("#all");
  expect(all.checked).toBe(false);
  all.dispatchEvent(new Event("change", { bubbles: true }));
  await tick();
  expect(runtime.store.get("todos.0.done")).toBe(true);
  expect(all.checked).toBe(true);
  $("#clear").click();
  await tick();
  expect(runtime.store.get("todos")).toEqual([]);
  expect($$("#list li")).toEqual([]);
});

it("mounts an edit field on double click, focuses it, and removes it on Escape", async () => {
  const runtime = mount("todos");
  expect($$("#list input.edit")).toEqual([]);
  $("#list label").dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
  await tick();
  const edit = $<HTMLInputElement>("#list input.edit");
  expect(edit.value).toBe("tea");
  expect(document.activeElement).toBe(edit);
  type(edit, "green tea");
  expect(runtime.store.get("todos.0.title")).toBe("green tea");
  key(edit, "Escape");
  await tick();
  expect($$("#list input.edit")).toEqual([]);
  expect($("#list label").textContent).toBe("green tea");
});

it("keeps the list items and the field being edited when a server patch carries only the list's template", async () => {
  const runtime = mount("todos");
  $("#list label").dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
  await tick();
  const item = $("#list li");
  const edit = $<HTMLInputElement>("#list input.edit");
  edit.value = "green";
  // A fragment the server renders holds a list as its template alone, with no items.
  const fragment = $("#list").cloneNode(true) as Element;
  fragment.replaceChildren(fragment.querySelector("template")!);
  runtime.apply({ event: "hs-element", data: JSON.stringify({ html: fragment.outerHTML }) });
  await tick();
  expect($("#list li")).toBe(item);
  expect(document.activeElement).toBe(edit);
  expect(edit.value).toBe("green");
  runtime.store.set("todos.0.title", "oolong");
  await tick();
  expect($$("#list li label").map((label) => label.textContent)).toEqual(["oolong"]);
});
