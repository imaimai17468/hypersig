import { expect, it } from "vitest";
import { $, $$, tick } from "../../dom.ts";
import { mount, spyReads } from "../mount.ts";

const on = (selector: string) => $$(selector).map((li) => li.classList.contains("on"));
const editing = () => $$("#l li").map((li) => li.querySelector("input")?.value ?? null);

it("runs only the items whose comparison with a selected signal changes", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' ` +
      `data-hs-class='[["on",["eq",["sig","selected"],["item",0,"id"]]]]'></li></template></ul>`,
    { rows: [1, 2, 3, 4].map((id) => ({ id })), selected: 2 },
  );
  await tick();
  expect(on("#l li")).toEqual([false, true, false, false]);
  const get = runtime.store.get.bind(runtime.store);
  const reads = spyReads(runtime);
  runtime.store.set("selected", 4);
  await tick();
  expect(on("#l li")).toEqual([false, false, false, true]);
  expect(reads.filter((path) => path.endsWith(".id")).toSorted()).toEqual([
    "rows.1.id",
    "rows.3.id",
  ]);

  runtime.store.set("selected", 1);
  runtime.store.set("rows.0.id", 5);
  await tick();
  expect(on("#l li")).toEqual([false, false, false, false]);
  runtime.store.set("selected", 5);
  await tick();
  expect(on("#l li")).toEqual([true, false, false, false]);

  runtime.store.set("selected", 3);
  runtime.store.set("rows", (get("rows") as unknown[]).slice(1));
  await tick();
  expect(on("#l li")).toEqual([false, true, false]);
});

it("shows the condition of only the items whose comparison with a selected signal changes, with the bindings inside it", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
      `<template data-hs-if='["eq",["sig","editing"],["item",0,"id"]]'><input data-hs-bind='[["item",0,"title"],"text"]'></template>` +
      `</li></template></ul>`,
    { rows: [1, 2, 3, 4].map((id) => ({ id, title: `t${id}` })), editing: 2 },
  );
  await tick();
  expect(editing()).toEqual([null, "t2", null, null]);
  const reads = spyReads(runtime);
  runtime.store.set("editing", 4);
  await tick();
  expect(editing()).toEqual([null, null, null, "t4"]);
  expect(reads.filter((path) => path.endsWith(".id")).toSorted()).toEqual([
    "rows.1.id",
    "rows.3.id",
  ]);

  runtime.store.set(
    "rows",
    [4, 3, 2, 1].map((id) => ({ id, title: `t${id}` })),
  );
  await tick();
  expect(editing()).toEqual(["t4", null, null, null]);
  runtime.store.set("rows.0.title", "renamed");
  await tick();
  expect(editing()).toEqual(["renamed", null, null, null]);
  const input = $<HTMLInputElement>("#l input");
  input.value = "typed";
  input.dispatchEvent(new Event("input", { bubbles: true }));
  expect(runtime.store.get("rows.0.title")).toBe("typed");
});

it("keeps comparing items with a selected signal after a patch registers their list again", async () => {
  const html =
    `<div id="frag"><ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' ` +
    `data-hs-class='[["on",["eq",["sig","selected"],["item",0,"id"]]]]'></li></template></ul></div>`;
  const runtime = mount(html, { rows: [1, 2, 3].map((id) => ({ id })), selected: 1 });
  await tick();
  expect(on("#frag li")).toEqual([true, false, false]);
  runtime.apply({ event: "hs-element", data: JSON.stringify({ html }) });
  await tick();
  runtime.store.set("selected", 3);
  await tick();
  expect(on("#frag li")).toEqual([false, false, true]);
});
