import { expect, it } from "vitest";
import { $, $$, tick } from "../../dom.ts";
import { mount, spyReads } from "../mount.ts";

const texts = () => $$("#l span").map((span) => span.textContent);
const tags = () =>
  $$("#l li").map((li) => [...li.querySelectorAll("b")].map((b) => b.textContent).join(""));

it("updates the items of a nested list when the enclosing item or a page signal they read changes", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
      `<template data-hs-each='[["item",0,"cells"],1]'><span data-hs-text='["concat",["concat",["item",0,"label"],["item",1,""]],["sig","suffix"]]'></span></template>` +
      `</li></template></ul>`,
    {
      rows: [
        { id: 1, label: "a", cells: ["x", "y"] },
        { id: 2, label: "b", cells: ["z"] },
      ],
      suffix: "",
    },
  );
  await tick();
  expect(texts()).toEqual(["ax", "ay", "bz"]);
  runtime.store.set("rows.0.label", "A");
  await tick();
  expect(texts()).toEqual(["Ax", "Ay", "bz"]);
  runtime.store.set("suffix", "!");
  await tick();
  expect(texts()).toEqual(["Ax!", "Ay!", "bz!"]);
  runtime.store.set("rows.1.cells.0", "w");
  await tick();
  expect(texts()).toEqual(["Ax!", "Ay!", "bw!"]);
});

it("renders a nested list again when its field is replaced or resized, and only its item for a change inside one", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
      `<template data-hs-each='[["item",0,"tags"],1]'><b data-hs-key='["item",1,"name"]' data-hs-text='["item",1,"name"]'></b></template>` +
      `</li></template></ul>`,
    {
      rows: [
        { id: 1, tags: [{ name: "a" }, { name: "b" }] },
        { id: 2, tags: [{ name: "c" }] },
      ],
    },
  );
  await tick();
  expect(tags()).toEqual(["ab", "c"]);
  const first = $("#l b");
  const reads = spyReads(runtime);
  runtime.store.set("rows.0.tags.1.name", "B");
  await tick();
  expect(tags()).toEqual(["aB", "c"]);
  expect(reads.filter((path) => path.startsWith("rows.0.tags.0"))).toEqual([]);

  runtime.store.set("rows.1.tags", [{ name: "c" }, { name: "d" }]);
  await tick();
  expect(tags()).toEqual(["aB", "cd"]);
  runtime.store.set("rows", (runtime.store.get("rows") as unknown[]).toReversed());
  await tick();
  expect(tags()).toEqual(["cd", "aB"]);
  expect($$("#l b")[2]).toBe(first);
  runtime.store.set("rows.1.tags.1.name", "b");
  await tick();
  expect(tags()).toEqual(["cd", "ab"]);
});

it("renders a nested list again when a page signal its key reads changes", async () => {
  const runtime = mount(
    `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
      `<template data-hs-each='[["item",0,"tags"],1]'><b data-hs-key='["concat",["item",1,""],["sig","suffix"]]' data-hs-text='["item",1,""]'></b></template>` +
      `</li></template></ul>`,
    { rows: [{ id: 1, tags: ["a", "b"] }], suffix: "" },
  );
  await tick();
  const first = $("#l b");
  runtime.store.set("suffix", "!");
  await tick();
  expect($$("#l b").map((b) => b.textContent)).toEqual(["a", "b"]);
  expect($("#l b")).not.toBe(first);
});
