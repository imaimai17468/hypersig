import { afterEach, describe, expect, it, vi } from "vitest";
import { Runtime } from "../src/runtime.ts";

const tick = () => new Promise((resolve) => setTimeout(resolve, 0));

function mount(html: string, signals: Record<string, unknown>): Runtime {
  const root = document.createElement("main");
  root.innerHTML = html;
  document.body.replaceChildren(root);
  return new Runtime(root, signals);
}

/** The store paths `runtime` reads from now on, whatever name it records each read under. */
function spyReads(runtime: Runtime): string[] {
  const reads: string[] = [];
  const { store } = runtime;
  const get = store.get.bind(store);
  const getAs = store.getAs.bind(store);
  store.get = (path: string) => {
    reads.push(path);
    return get(path);
  };
  store.getAs = (path: string, read: string) => {
    reads.push(path);
    return getAs(path, read);
  };
  return reads;
}

function sseResponse(body: string): Response {
  return new Response(body, { status: 200, headers: { "content-type": "text/event-stream" } });
}

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("Runtime", () => {
  it("runs only the items whose comparison with a selected signal changes", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' ` +
        `data-hs-class='[["on",["eq",["sig","selected"],["item",0,"id"]]]]'></li></template></ul>`,
      { rows: [1, 2, 3, 4].map((id) => ({ id })), selected: 2 },
    );
    await tick();
    const on = () => [...document.querySelectorAll("#l li")].map((li) => li.classList.contains("on"));
    expect(on()).toEqual([false, true, false, false]);
    const get = runtime.store.get.bind(runtime.store);
    const reads = spyReads(runtime);
    runtime.store.set("selected", 4);
    await tick();
    expect(on()).toEqual([false, false, false, true]);
    expect(reads.filter((path) => path.endsWith(".id")).sort()).toEqual(["rows.1.id", "rows.3.id"]);

    runtime.store.set("selected", 1);
    runtime.store.set("rows.0.id", 5);
    await tick();
    expect(on()).toEqual([false, false, false, false]);
    runtime.store.set("selected", 5);
    await tick();
    expect(on()).toEqual([true, false, false, false]);

    runtime.store.set("selected", 3);
    runtime.store.set("rows", (get("rows") as unknown[]).slice(1));
    await tick();
    expect(on()).toEqual([false, true, false]);
  });

  it("keeps the bindings of items that shift unchanged, and reruns the bindings that read their position", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
        `<b data-hs-text='["str",["index",0]]'></b><i data-hs-text='["item",0,"label"]'></i>` +
        `<s data-hs-text='["concat",["item",0,"label"],["sig","suffix"]]'></s></li></template></ul>`,
      { rows: [1, 2, 3, 4].map((id) => ({ id, label: `r${id}` })), suffix: "" },
    );
    await tick();
    const [first, ...rest] = [...document.querySelectorAll("#l li")];
    const get = runtime.store.get.bind(runtime.store);
    const reads = spyReads(runtime);
    runtime.store.set("rows", (get("rows") as unknown[]).slice(1));
    await tick();
    expect([...document.querySelectorAll("#l li")]).toEqual(rest);
    expect(first!.isConnected).toBe(false);
    expect(rest.map((li) => li.querySelector("b")!.textContent)).toEqual(["0", "1", "2"]);
    expect(reads.filter((path) => path.endsWith(".label"))).toEqual([]);

    runtime.store.set("suffix", "!");
    runtime.store.set("rows.0.label", "x");
    await tick();
    expect(rest.map((li) => li.querySelector("s")!.textContent)).toEqual(["x!", "r3!", "r4!"]);
    expect(rest.map((li) => li.querySelector("i")!.textContent)).toEqual(["x", "r3", "r4"]);
  });

  it("evaluates again the key of an unchanged item where the key reads more than the item", async () => {
    const list = (key: string) =>
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='${key}' data-hs-text='["item",0,"label"]'></li></template></ul>`;
    const items = () => [...document.querySelectorAll("#l li")];

    const byPrefix = mount(list('["concat",["sig","prefix"],["str",["item",0,"id"]]]'), {
      rows: [1, 2, 3].map((id) => ({ id, label: `r${id}` })),
      prefix: "a",
    });
    await tick();
    const before = items();
    const rows = byPrefix.store.get("rows") as unknown[];
    byPrefix.store.set("rows", [...rows].reverse());
    await tick();
    expect(items()).toEqual([...before].reverse());
    byPrefix.store.set("prefix", "b");
    byPrefix.store.set("rows", rows);
    await tick();
    expect(items().map((li) => li.textContent)).toEqual(["r1", "r2", "r3"]);
    expect(items().some((li) => before.includes(li))).toBe(false);

    const byIndex = mount(list('["index",0]'), { rows: [1, 2, 3].map((id) => ({ id, label: `r${id}` })) });
    await tick();
    const placed = items();
    byIndex.store.set("rows", [...(byIndex.store.get("rows") as unknown[])].reverse());
    await tick();
    expect(items()).toEqual(placed);
    expect(items().map((li) => li.textContent)).toEqual(["r3", "r2", "r1"]);
  });

  it("moves the items between the ones that keep their place at either end", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["str",["item",0,"id"]]'></li></template></ul>`,
      { rows: [1, 2, 3, 4, 5, 6].map((id) => ({ id })) },
    );
    await tick();
    const byId = new Map([...document.querySelectorAll("#l li")].map((li) => [li.textContent, li]));
    const rows = runtime.store.get("rows") as unknown[];
    runtime.store.set("rows", [0, 1, 4, 3, 2, 5].map((at) => rows[at]));
    await tick();
    const after = [...document.querySelectorAll("#l li")];
    expect(after.map((li) => li.textContent)).toEqual(["1", "2", "5", "4", "3", "6"]);
    expect(after).toEqual(["1", "2", "5", "4", "3", "6"].map((id) => byId.get(id)));
  });

  it("matches only the positions a write replaced, and keeps the other items and their bindings", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul>`,
      { rows: [1, 2, 3, 4, 5, 6].map((id) => ({ id, label: `r${id}` })) },
    );
    await tick();
    const items = () => [...document.querySelectorAll("#l li")];
    const before = items();
    const rows = runtime.store.get("rows") as { id: number; label: string }[];
    runtime.store.set("rows", [rows[0], rows[4], rows[2], rows[3], rows[1], rows[5]]);
    await tick();
    expect(items()).toEqual([0, 4, 2, 3, 1, 5].map((at) => before[at]));

    const swapped = runtime.store.get("rows") as { id: number; label: string }[];
    runtime.store.set("rows", swapped.map((row, at) => (at === 2 ? { id: 3, label: "x" } : at === 3 ? { id: 9, label: "new" } : row)));
    await tick();
    const after = items();
    expect(after.map((li) => li.textContent)).toEqual(["r1", "r5", "x", "new", "r2", "r6"]);
    expect(after[2]).toBe(before[2]);
    expect(before.includes(after[3]!)).toBe(false);
    expect(before[3]!.isConnected).toBe(false);

    runtime.store.set("rows.0.label", "first");
    await tick();
    expect(items()[0]!.textContent).toBe("first");
  });

  it("reaches the bindings of an item that moved through a write at its new path", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
        `<b data-hs-text='["item",0,"label"]'></b><input type="checkbox" data-hs-bind='[["item",0,"done"],"bool"]'>` +
        `<ol><template data-hs-each='[["item",0,"tags"],1]'><i data-hs-key='["item",1,"id"]' data-hs-text='["item",1,"name"]'></i></template></ol>` +
        `</li></template></ul>`,
      {
        rows: [1, 2, 3].map((id) => ({ id, label: `r${id}`, done: false, tags: [{ id: 1, name: `t${id}` }, { id: 2, name: `u${id}` }] })),
      },
    );
    await tick();
    const items = () => [...document.querySelectorAll("#l > li")];
    const [, second, third] = items();
    runtime.store.set("rows", (runtime.store.get("rows") as unknown[]).slice(1));
    await tick();
    expect(items()).toEqual([second, third]);

    runtime.store.set("rows.0.label", "x");
    runtime.store.set("rows.1.done", true);
    runtime.store.set("rows.1.tags.1.name", "y");
    await tick();
    expect(second!.querySelector("b")!.textContent).toBe("x");
    expect(third!.querySelector("b")!.textContent).toBe("r3");
    expect((third!.querySelector("input") as HTMLInputElement).checked).toBe(true);
    expect((second!.querySelector("input") as HTMLInputElement).checked).toBe(false);
    expect([...third!.querySelectorAll("i")].map((i) => i.textContent)).toEqual(["t3", "y"]);
    expect([...second!.querySelectorAll("i")].map((i) => i.textContent)).toEqual(["t2", "u2"]);

    runtime.store.set("rows.1.tags", [{ id: 2, name: "y" }]);
    await tick();
    expect([...third!.querySelectorAll("i")].map((i) => i.textContent)).toEqual(["y"]);
    runtime.store.set("rows", [...(runtime.store.get("rows") as unknown[])].reverse());
    await tick();
    runtime.store.set("rows.0.tags.0.name", "z");
    await tick();
    expect(items()).toEqual([third, second]);
    expect([...third!.querySelectorAll("i")].map((i) => i.textContent)).toEqual(["z"]);
  });

  it("reaches the items of every list over one signal through a write at an item's path", async () => {
    const list = (id: string) =>
      `<ul id="${id}"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul>`;
    const runtime = mount(list("a") + list("b"), { rows: [1, 2, 3].map((id) => ({ id, label: `r${id}` })) });
    await tick();
    const labels = (id: string) => [...document.querySelectorAll(`#${id} li`)].map((li) => li.textContent);
    runtime.store.set("rows", (runtime.store.get("rows") as unknown[]).slice(1));
    await tick();
    runtime.store.set("rows.0.label", "x");
    await tick();
    expect(labels("a")).toEqual(["x", "r3"]);
    expect(labels("b")).toEqual(["x", "r3"]);
  });

  it("counts the items of a list with one read of the list, and still reads the signals the test compares with", async () => {
    const runtime = mount(`<b id="n" data-hs-text='["str",["count","rows",0,["eq",["item",0,"done"],["sig","want"]]]]'></b>`, {
      rows: [false, true, false].map((done) => ({ done })),
      want: false,
    });
    await tick();
    const count = () => document.getElementById("n")!.textContent;
    expect(count()).toBe("2");
    runtime.store.set("rows.1.done", false);
    await tick();
    expect(count()).toBe("3");
    runtime.store.set("want", true);
    await tick();
    expect(count()).toBe("0");
    runtime.store.set("rows", [...(runtime.store.get("rows") as unknown[]), { done: true }]);
    await tick();
    expect(count()).toBe("1");
  });

  it("forgets the items of a list that a patch removed after the next frame", async () => {
    const runtime = mount(
      `<div id="frag"><ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul></div>`,
      { rows: [1, 2].map((id) => ({ id, label: `r${id}` })) },
    );
    await tick();
    const removed = document.querySelector("#frag li")!;
    const key = Object.getOwnPropertySymbols(removed).find((symbol) => symbol.description === "hypersig")!;
    const bound = () => (removed as unknown as Record<symbol, { effects?: unknown[]; watch?: unknown }>)[key]!;
    expect(bound().effects).toHaveLength(1);
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html: `<div id="frag"><p>gone</p></div>` }) });
    expect(removed.isConnected).toBe(false);
    await new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)));
    expect(bound().effects).toBeUndefined();
    expect(bound().watch).toBeUndefined();
    runtime.store.set("rows.0.label", "x");
    await tick();
    expect(document.getElementById("frag")!.textContent).toBe("gone");
  });

  it("forgets the bindings below an element that a patch removed after the next frame", async () => {
    const runtime = mount(
      `<div id="frag"><div><span data-hs-text='["sig","label"]'></span><template data-hs-if='["sig","on"]'><b data-hs-text='["sig","label"]'></b></template></div></div>`,
      { label: "a", on: true },
    );
    await tick();
    const span = document.querySelector("#frag span")!;
    const b = document.querySelector("#frag b")!;
    const key = Object.getOwnPropertySymbols(span).find((symbol) => symbol.description === "hypersig")!;
    const bound = (element: Element) => (element as unknown as Record<symbol, { effects?: unknown[] }>)[key]!.effects;
    expect(bound(span)).toHaveLength(1);
    expect(bound(b)).toHaveLength(1);
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html: `<div id="frag"><p>gone</p></div>` }) });
    expect(span.isConnected).toBe(false);
    await new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)));
    expect(bound(span)).toBeUndefined();
    expect(bound(b)).toBeUndefined();
  });

  it("keeps the bindings of an element that a patch moved by its id", async () => {
    const runtime = mount(
      `<div id="frag"><span id="kept" data-hs-text='["sig","label"]'></span><i>x</i></div>`,
      { label: "a" },
    );
    await tick();
    const kept = document.getElementById("kept")!;
    const key = Object.getOwnPropertySymbols(kept).find((symbol) => symbol.description === "hypersig")!;
    const bound = () => (kept as unknown as Record<symbol, { effects?: unknown[] }>)[key]!.effects;
    // The morph replaces the span at the first position with the <b>, then finds it by id and moves it to the second.
    runtime.apply({
      event: "hs-element",
      data: JSON.stringify({ html: `<div id="frag"><b>y</b><span id="kept" data-hs-text='["sig","label"]'></span></div>` }),
    });
    expect(document.getElementById("kept")).toBe(kept);
    await new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)));
    expect(bound()).toHaveLength(1);
    runtime.store.set("label", "b");
    await tick();
    expect(kept.textContent).toBe("b");
  });

  it("forgets the bindings inside a removed condition after the next frame", async () => {
    const runtime = mount(`<template data-hs-if='["sig","on"]'><b id="b" data-hs-text='["sig","label"]'></b></template>`, { on: true, label: "x" });
    await tick();
    const removed = document.getElementById("b")!;
    // The runtime keeps what it knows about an element under the symbol `hypersig`.
    const key = Object.getOwnPropertySymbols(removed).find((symbol) => symbol.description === "hypersig")!;
    const bound = () => (removed as unknown as Record<symbol, { effects?: unknown[] }>)[key]!.effects;
    expect(bound()).toHaveLength(1);
    runtime.store.set("on", false);
    await tick();
    expect(removed.isConnected).toBe(false);
    await new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)));
    expect(bound()).toBeUndefined();
  });

  it("renders text and show from signals, and reacts to a click", async () => {
    mount(
      `<span id="t" data-hs-text='["str",["sig","count"]]'></span>` +
        `<p id="p" data-hs-show='["gt",["sig","count"],["lit",0]]'>positive</p>` +
        `<button id="b" data-hs-on-click='["set","count",["add",["sig","count"],["lit",1]]]'><i id="icon">+</i></button>`,
      { count: 0 },
    );
    expect(document.getElementById("t")!.textContent).toBe("0");
    expect(document.getElementById("p")!.style.display).toBe("none");

    document.getElementById("icon")!.click();
    await tick();
    expect(document.getElementById("t")!.textContent).toBe("1");
    expect(document.getElementById("p")!.style.display).toBe("");
  });

  it("binds an input in both directions", async () => {
    const runtime = mount(`<input id="i" data-hs-bind='["name","text"]'>`, { name: "a" });
    const input = document.getElementById("i") as HTMLInputElement;
    expect(input.value).toBe("a");
    input.value = "ab";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    expect(runtime.store.get("name")).toBe("ab");
    runtime.store.set("name", "");
    await tick();
    expect(input.value).toBe("");
  });

  it("posts the arguments to an action, and nothing else, and applies the patches it returns", async () => {
    const fetch = vi.fn(async (_url: URL, _init: RequestInit) =>
      sseResponse(
        'event: hs-signals\ndata: {"draft":""}\n\n' +
          'event: hs-element\ndata: {"html":"<ul id=\\"list\\"><li>tea</li></ul>"}\n\n',
      ),
    );
    vi.stubGlobal("fetch", fetch);
    const runtime = mount(
      `<form data-hs-on-submit='["call","POST","/add",["lit",{"id":3}],null,null]'><input data-hs-bind='["draft","text"]'><button id="go">go</button></form><ul id="list"></ul>`,
      { draft: "tea" },
    );
    document.querySelector("form")!.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
    await vi.waitFor(() => expect(document.getElementById("list")!.innerHTML).toBe("<li>tea</li>"));

    const [url, init] = fetch.mock.calls[0]!;
    expect(url.pathname).toBe("/add");
    expect(url.search).toBe("");
    expect(init.method).toBe("POST");
    expect(init.body).toBe('{"id":3}');
    expect(runtime.store.get("draft")).toBe("");
  });

  it("runs the statement after a call without waiting for the answer", async () => {
    vi.stubGlobal("fetch", vi.fn(() => new Promise<Response>(() => {})));
    const runtime = mount(
      `<button data-hs-on-click='["seq",[["call","POST","/slow",["sig","n"],null,null],["set","n",["lit",2]]]]'>go</button>`,
      { n: 1 },
    );
    document.querySelector("button")!.click();
    await vi.waitFor(() => expect(runtime.store.get("n")).toBe(2));
  });

  it("re-registers effects inside a patched fragment", async () => {
    const runtime = mount(`<div id="frag"></div>`, { on: false });
    runtime.apply({
      event: "hs-element",
      data: JSON.stringify({ html: `<div id="frag"><b id="x" data-hs-show='["sig","on"]'>x</b></div>` }),
    });
    expect(document.getElementById("x")!.style.display).toBe("none");
    runtime.store.set("on", true);
    await tick();
    expect(document.getElementById("x")!.style.display).toBe("");
  });
});
