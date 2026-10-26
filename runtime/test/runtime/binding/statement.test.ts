import { expect, it } from "vitest";
import { Runtime } from "../../../src/runtime.ts";
import { $, tick } from "../../dom.ts";
import { mount } from "../mount.ts";

it("renders once for all the steps of a sequence", async () => {
  const runtime = mount(
    `<button data-hs-on-click='["seq",[["set","rows",["lit",[]]],["push","rows",["lit",1]],["push","rows",["lit",2]]]]'></button>`,
    { rows: [0] },
  );
  await tick();
  const seen: unknown[] = [];
  runtime.store.subscribe(() => seen.push(runtime.store.get("rows")));
  $("button").click();
  await tick();
  expect(seen).toEqual([[1, 2]]);
});

it("reads a computed signal that an earlier step of the sequence changed", async () => {
  const root = document.createElement("main");
  root.dataset.hsComputed = JSON.stringify([["total", ["add", ["sig", "n"], ["lit", 1]]]]);
  root.innerHTML = `<button data-hs-on-click='["seq",[["set","n",["lit",5]],["set","copy",["sig","total"]]]]'></button>`;
  document.body.replaceChildren(root);
  const runtime = new Runtime(root, { n: 0, total: 1, copy: 0 });
  await tick();
  $("button").click();
  await tick();
  expect(runtime.store.get("copy")).toBe(6);
});

it("records in an effect the reads of every step of a sequence", async () => {
  const runtime = mount(
    `<div data-hs-effect='[["seq",[["set","a",["sig","x"]],["set","b",["sig","y"]]]]]'></div>`,
    {
      x: 1,
      y: 2,
      a: 0,
      b: 0,
    },
  );
  await tick();
  runtime.store.set("y", 3);
  await tick();
  expect(runtime.store.get("b")).toBe(3);
});

it("keeps the browser from submitting a form that has no submit statement", () => {
  mount(`<form id="f"><input name="q"></form>`, {});
  const submit = new Event("submit", { bubbles: true, cancelable: true });
  $("#f").dispatchEvent(submit);
  expect(submit.defaultPrevented).toBe(true);
});
