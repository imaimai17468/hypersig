import { expect, it } from "vitest";
import { $, nextFrame, tick } from "../dom.ts";
import { bound, mount } from "./mount.ts";

it("keeps the content of a condition, with its focus, when a patch keeps its template", async () => {
  const html = `<div id="frag"><template data-hs-if='["sig","on"]'><input data-hs-bind='["text","text"]'></template></div>`;
  const runtime = mount(html, { on: true, text: "a" });
  await tick();
  const input = $<HTMLInputElement>("#frag input");
  input.focus();
  runtime.apply({ event: "hs-element", data: JSON.stringify({ html }) });
  await tick();
  expect($("#frag input")).toBe(input);
  expect(document.activeElement).toBe(input);
  runtime.store.set("on", false);
  await tick();
  expect(document.querySelector("#frag input")).toBeNull();
});

it("forgets the bindings inside a removed condition after the next frame", async () => {
  const runtime = mount(
    `<template data-hs-if='["sig","on"]'><b id="b" data-hs-text='["sig","label"]'></b></template>`,
    { on: true, label: "x" },
  );
  await tick();
  const removed = $("#b");
  expect(bound(removed).effects).toHaveLength(1);
  runtime.store.set("on", false);
  await tick();
  expect(removed.isConnected).toBe(false);
  await nextFrame();
  expect(bound(removed).effects).toBeUndefined();
});
