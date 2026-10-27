import { afterEach, expect, it, vi } from "vitest";
import { $, nextFrame, tick } from "../../dom.ts";
import { bound, mount } from "../mount.ts";

afterEach(() => {
  vi.restoreAllMocks();
});

it("warns when a patch adds an id or a node ref name the rest of the page already has", async () => {
  const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
  const runtime = mount(`<p id="total"></p><input data-hs-ref="field"><div id="frag"></div>`, {});
  await tick();
  runtime.apply({
    event: "hs-element",
    data: JSON.stringify({
      html: `<div id="frag"><p id="total"></p><input data-hs-ref="field"></div>`,
    }),
  });
  expect(warn.mock.calls.map(([message]) => message)).toEqual([
    'hypersig: the patch for #frag adds a second element with the id "total"',
    'hypersig: the patch for #frag adds a second element with data-hs-ref="field"',
  ]);
  warn.mockRestore();
});

it("forgets the bindings below an element that a patch removed after the next frame", async () => {
  const runtime = mount(
    `<div id="frag"><div><span data-hs-text='["sig","label"]'></span><template data-hs-if='["sig","on"]'><b data-hs-text='["sig","label"]'></b></template></div></div>`,
    { label: "a", on: true },
  );
  await tick();
  const span = $("#frag span");
  const b = $("#frag b");
  expect(bound(span).effects).toHaveLength(1);
  expect(bound(b).effects).toHaveLength(1);
  runtime.apply({
    event: "hs-element",
    data: JSON.stringify({ html: `<div id="frag"><p>gone</p></div>` }),
  });
  expect(span.isConnected).toBe(false);
  await nextFrame();
  expect(bound(span).effects).toBeUndefined();
  expect(bound(b).effects).toBeUndefined();
});

it("keeps the bindings of an element that a patch moved by its id", async () => {
  const runtime = mount(
    `<div id="frag"><span id="kept" data-hs-text='["sig","label"]'></span><i>x</i></div>`,
    { label: "a" },
  );
  await tick();
  const kept = $("#kept");
  // The morph replaces the span at the first position with the <b>, then finds it by id and moves it to the second.
  runtime.apply({
    event: "hs-element",
    data: JSON.stringify({
      html: `<div id="frag"><b>y</b><span id="kept" data-hs-text='["sig","label"]'></span></div>`,
    }),
  });
  expect($("#kept")).toBe(kept);
  await nextFrame();
  expect(bound(kept).effects).toHaveLength(1);
  runtime.store.set("label", "b");
  await tick();
  expect(kept.textContent).toBe("b");
});

it("re-registers effects inside a patched fragment", async () => {
  const runtime = mount(`<div id="frag"></div>`, { on: false });
  runtime.apply({
    event: "hs-element",
    data: JSON.stringify({
      html: `<div id="frag"><b id="x" data-hs-show='["sig","on"]'>x</b></div>`,
    }),
  });
  expect($("#x").style.display).toBe("none");
  runtime.store.set("on", true);
  await tick();
  expect($("#x").style.display).toBe("");
});

it("applies the events after one whose data does not parse", async () => {
  const error = vi.spyOn(console, "error").mockImplementation(() => {});
  const runtime = mount(`<p data-hs-text='["sig","label"]'></p>`, { label: "a" });
  for (const data of ["{not json", '{"label":"b"}']) runtime.apply({ event: "hs-signals", data });
  await tick();
  expect($("p").textContent).toBe("b");
  expect(error).toHaveBeenCalledTimes(1);
});

/** A list of `n` rows, each with an id and a node ref name. */
function rows(n: number): string {
  return Array.from(
    { length: n },
    (_, i) => `<li id="row-${i}"><input data-hs-ref="row-${i}"></li>`,
  ).join("");
}

/** The calls among `calls` that select by an id or a `data-hs-` name. */
function pageQueries(calls: unknown[][]): unknown[][] {
  return calls.filter(([selector]) => /^\[(id|data-hs-)/u.test(String(selector)));
}

it("reads the page once per id and per name when it checks a long patched list", async () => {
  const runtime = mount(`<ul id="frag">${rows(300)}</ul>`, {});
  await tick();
  const all = vi.spyOn(Document.prototype, "querySelectorAll");
  const one = vi.spyOn(Element.prototype, "querySelectorAll");
  const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
  runtime.apply({
    event: "hs-element",
    data: JSON.stringify({ html: `<ul id="frag">${rows(301)}</ul>` }),
  });
  expect(warn).not.toHaveBeenCalled();
  expect(
    pageQueries(all.mock.calls).length + pageQueries(one.mock.calls).length,
  ).toBeLessThanOrEqual(3);
});
