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
  const getIn = store.getIn.bind(store);
  store.getIn = (base: string, field: string, read: string) => {
    reads.push(field === "" ? base : `${base}.${field}`);
    return getIn(base, field, read);
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

  it("binds an input inside a list item in both directions", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
        `<input data-hs-bind='[["item",0,"title"],"text"]'></li></template></ul>`,
      { rows: [1, 2].map((id) => ({ id, title: `t${id}` })) },
    );
    await tick();
    const inputs = () => [...document.querySelectorAll<HTMLInputElement>("#l input")];
    expect(inputs().map((input) => input.value)).toEqual(["t1", "t2"]);
    inputs()[1]!.value = "typed";
    inputs()[1]!.dispatchEvent(new Event("input", { bubbles: true }));
    expect(runtime.store.get("rows.1.title")).toBe("typed");
    runtime.store.set("rows.0.title", "set");
    await tick();
    expect(inputs().map((input) => input.value)).toEqual(["set", "typed"]);
  });

  it("shows the condition of only the items whose comparison with a selected signal changes, with the bindings inside it", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
        `<template data-hs-if='["eq",["sig","editing"],["item",0,"id"]]'><input data-hs-bind='[["item",0,"title"],"text"]'></template>` +
        `</li></template></ul>`,
      { rows: [1, 2, 3, 4].map((id) => ({ id, title: `t${id}` })), editing: 2 },
    );
    await tick();
    const editing = () => [...document.querySelectorAll("#l li")].map((li) => li.querySelector("input")?.value ?? null);
    expect(editing()).toEqual([null, "t2", null, null]);
    const reads = spyReads(runtime);
    runtime.store.set("editing", 4);
    await tick();
    expect(editing()).toEqual([null, null, null, "t4"]);
    expect(reads.filter((path) => path.endsWith(".id")).sort()).toEqual(["rows.1.id", "rows.3.id"]);

    runtime.store.set("rows", [4, 3, 2, 1].map((id) => ({ id, title: `t${id}` })));
    await tick();
    expect(editing()).toEqual(["t4", null, null, null]);
    runtime.store.set("rows.0.title", "renamed");
    await tick();
    expect(editing()).toEqual(["renamed", null, null, null]);
    const input = document.querySelector<HTMLInputElement>("#l input")!;
    input.value = "typed";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    expect(runtime.store.get("rows.0.title")).toBe("typed");
  });

  it("updates the items of a nested list when the enclosing item or a page signal they read changes", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
        `<template data-hs-each='[["item",0,"cells"],1]'><span data-hs-text='["concat",["concat",["item",0,"label"],["item",1,""]],["sig","suffix"]]'></span></template>` +
        `</li></template></ul>`,
      { rows: [{ id: 1, label: "a", cells: ["x", "y"] }, { id: 2, label: "b", cells: ["z"] }], suffix: "" },
    );
    await tick();
    const texts = () => [...document.querySelectorAll("#l span")].map((span) => span.textContent);
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
      { rows: [{ id: 1, tags: [{ name: "a" }, { name: "b" }] }, { id: 2, tags: [{ name: "c" }] }] },
    );
    await tick();
    const tags = () => [...document.querySelectorAll("#l li")].map((li) => [...li.querySelectorAll("b")].map((b) => b.textContent).join(""));
    expect(tags()).toEqual(["ab", "c"]);
    const first = document.querySelector("#l b")!;
    const reads = spyReads(runtime);
    runtime.store.set("rows.0.tags.1.name", "B");
    await tick();
    expect(tags()).toEqual(["aB", "c"]);
    expect(reads.filter((path) => path.startsWith("rows.0.tags.0"))).toEqual([]);

    runtime.store.set("rows.1.tags", [{ name: "c" }, { name: "d" }]);
    await tick();
    expect(tags()).toEqual(["aB", "cd"]);
    runtime.store.set("rows", [...(runtime.store.get("rows") as unknown[])].reverse());
    await tick();
    expect(tags()).toEqual(["cd", "aB"]);
    expect(document.querySelectorAll("#l b")[2]).toBe(first);
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
    const first = document.querySelector("#l b")!;
    runtime.store.set("suffix", "!");
    await tick();
    expect([...document.querySelectorAll("#l b")].map((b) => b.textContent)).toEqual(["a", "b"]);
    expect(document.querySelector("#l b")).not.toBe(first);
  });

  it("keeps comparing items with a selected signal after a patch registers their list again", async () => {
    const html =
      `<div id="frag"><ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' ` +
      `data-hs-class='[["on",["eq",["sig","selected"],["item",0,"id"]]]]'></li></template></ul></div>`;
    const runtime = mount(html, { rows: [1, 2, 3].map((id) => ({ id })), selected: 1 });
    await tick();
    const on = () => [...document.querySelectorAll("#frag li")].map((li) => li.classList.contains("on"));
    expect(on()).toEqual([true, false, false]);
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html }) });
    await tick();
    runtime.store.set("selected", 3);
    await tick();
    expect(on()).toEqual([false, false, true]);
  });

  it("keeps the items of a list and their bindings when a patch keeps its template", async () => {
    const list = `<ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul>`;
    const runtime = mount(`<div id="frag"><p>a</p>${list}</div>`, { rows: [1, 2].map((id) => ({ id, label: `r${id}` })) });
    await tick();
    const items = [...document.querySelectorAll("#frag li")];
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html: `<div id="frag"><p>b</p>${list}</div>` }) });
    await tick();
    expect(document.querySelector("#frag p")!.textContent).toBe("b");
    expect([...document.querySelectorAll("#frag li")]).toEqual(items);
    await new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)));
    runtime.store.set("rows.1.label", "x");
    runtime.store.set("rows", [...(runtime.store.get("rows") as unknown[]), { id: 3, label: "r3" }]);
    await tick();
    expect([...document.querySelectorAll("#frag li")].map((li) => li.textContent)).toEqual(["r1", "x", "r3"]);
    expect(document.querySelector("#frag li")).toBe(items[0]);
  });

  it("warns when a patch adds an id or a node ref name the rest of the page already has", async () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const runtime = mount(`<p id="total"></p><input data-hs-ref="field"><div id="frag"></div>`, {});
    await tick();
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html: `<div id="frag"><p id="total"></p><input data-hs-ref="field"></div>` }) });
    expect(warn.mock.calls.map(([message]) => message)).toEqual([
      'hypersig: the patch for #frag adds a second element with the id "total"',
      'hypersig: the patch for #frag adds a second element with data-hs-ref="field"',
    ]);
    warn.mockRestore();
  });

  it("does not warn about a node ref name every item of a list carries", async () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const html = `<div id="frag"><ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,""]'><input data-hs-ref="field"></li></template></ul></div>`;
    const runtime = mount(html, { rows: [1, 2, 3] });
    await tick();
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html }) });
    await tick();
    expect(document.querySelectorAll("#frag input")).toHaveLength(3);
    expect(warn).not.toHaveBeenCalled();
    warn.mockRestore();
  });

  it("builds the items of a list again when a patch changes its template", async () => {
    const list = (tag: string) =>
      `<div id="frag"><ul><template data-hs-each='["rows",0]'><${tag} data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></${tag}></template></ul></div>`;
    const runtime = mount(list("li"), { rows: [1, 2].map((id) => ({ id, label: `r${id}` })) });
    await tick();
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html: list("b") }) });
    await tick();
    expect(document.querySelectorAll("#frag li")).toHaveLength(0);
    expect([...document.querySelectorAll("#frag b")].map((b) => b.textContent)).toEqual(["r1", "r2"]);
  });

  it("keeps the content of a condition, with its focus, when a patch keeps its template", async () => {
    const html = `<div id="frag"><template data-hs-if='["sig","on"]'><input data-hs-bind='["text","text"]'></template></div>`;
    const runtime = mount(html, { on: true, text: "a" });
    await tick();
    const input = document.querySelector<HTMLInputElement>("#frag input")!;
    input.focus();
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html }) });
    await tick();
    expect(document.querySelector("#frag input")).toBe(input);
    expect(document.activeElement).toBe(input);
    runtime.store.set("on", false);
    await tick();
    expect(document.querySelector("#frag input")).toBeNull();
  });

  it("forgets the items of a list that a patch removed after the next frame", async () => {
    const runtime = mount(
      `<div id="frag"><ul><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul></div>`,
      { rows: [1, 2].map((id) => ({ id, label: `r${id}` })) },
    );
    await tick();
    const removed = document.querySelector("#frag li")!;
    const key = Object.getOwnPropertySymbols(removed).find((symbol) => symbol.description === "hypersig")!;
    // An item's bindings live in its row rather than in an `Effect` per binding.
    const bound = () => (removed as unknown as Record<symbol, { effects?: unknown[]; watch?: unknown; row?: unknown }>)[key]!;
    expect(bound().row).toBeDefined();
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html: `<div id="frag"><p>gone</p></div>` }) });
    expect(removed.isConnected).toBe(false);
    await new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)));
    expect(bound().row).toBeUndefined();
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

  it("loads the page a navigate event names, and applies no event after it", () => {
    const assign = vi.spyOn(location, "assign").mockImplementation(() => {});
    const runtime = mount(`<p></p>`, { n: 1 });
    runtime.apply({ event: "hs-navigate", data: JSON.stringify({ url: "/todos/3" }) });
    runtime.apply({ event: "hs-signals", data: JSON.stringify({ n: 2 }) });
    expect(assign).toHaveBeenCalledWith("/todos/3");
    expect(runtime.store.get("n")).toBe(1);
    assign.mockRestore();
  });

  it("pushes and replaces the URL without loading it, and loads the page Back moves to", async () => {
    history.replaceState(null, "", "/fruit");
    const reload = vi.spyOn(location, "reload").mockImplementation(() => {});
    const runtime = mount(`<button data-hs-on-click='["history","push",["lit","/fruit?q=fig"]]'>go</button>`, { n: 1 });
    const length = history.length;
    document.querySelector("button")!.click();
    await tick();
    expect(location.pathname + location.search).toBe("/fruit?q=fig");
    expect(history.length).toBe(length + 1);

    document.querySelector("button")!.click();
    await tick();
    expect(history.length).toBe(length + 1);

    runtime.apply({ event: "hs-history", data: JSON.stringify({ url: "/fruit?q=kiwi", replace: true }) });
    runtime.apply({ event: "hs-signals", data: JSON.stringify({ n: 2 }) });
    expect(location.pathname + location.search).toBe("/fruit?q=kiwi");
    expect(history.length).toBe(length + 1);
    expect(runtime.store.get("n")).toBe(2);

    // A move to a `#fragment` of the URL the page shows keeps the page.
    history.replaceState(null, "", "/fruit?q=kiwi#top");
    runtime.popped();
    expect(reload).not.toHaveBeenCalled();

    history.replaceState(null, "", "/fruit");
    runtime.popped();
    expect(reload).toHaveBeenCalledOnce();
    runtime.apply({ event: "hs-signals", data: JSON.stringify({ n: 3 }) });
    expect(runtime.store.get("n")).toBe(2);
    reload.mockRestore();
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

describe("visibility", () => {
  it("runs an on-visible statement each time the element comes into view, and stops watching it once it leaves the page", () => {
    let report!: (entries: { target: Element; isIntersecting: boolean }[]) => void;
    const observed: Element[] = [];
    const unobserved: Element[] = [];
    class Observer {
      constructor(callback: (entries: unknown[], observer: Observer) => void) {
        report = (entries) => callback(entries, this);
      }
      observe(target: Element) {
        observed.push(target);
      }
      unobserve(target: Element) {
        unobserved.push(target);
      }
    }
    vi.stubGlobal("IntersectionObserver", Observer);
    const runtime = mount(`<p id="end" data-hs-on-visible='["set","loads",["add",["sig","loads"],["lit",1]]]'></p>`, { loads: 0 });
    const end = document.getElementById("end")!;
    expect(observed).toEqual([end]);
    report([{ target: end, isIntersecting: true }]);
    report([{ target: end, isIntersecting: false }]);
    report([{ target: end, isIntersecting: true }]);
    expect(runtime.store.get("loads")).toBe(2);
    end.remove();
    report([{ target: end, isIntersecting: false }]);
    expect(unobserved).toEqual([end]);
    vi.unstubAllGlobals();
  });
});

describe("calls", () => {
  /** A fetch whose answers the test settles one by one, which rejects once its request is aborted. */
  function pendingFetch() {
    const answers: ((body: string) => void)[] = [];
    const fetch = vi.fn(
      (_url: URL, init: RequestInit) =>
        new Promise<Response>((resolve, reject) => {
          init.signal?.addEventListener("abort", () => reject(new DOMException("aborted", "AbortError")));
          answers.push((body) => resolve(sseResponse(body)));
        }),
    );
    vi.stubGlobal("fetch", fetch);
    return { fetch, answers };
  }

  const signals = (results: string) => `event: hs-signals\ndata: {"results":"${results}"}\n\n`;

  it("aborts the earlier latest call of the same action, so its answer cannot overwrite a newer one", async () => {
    const { fetch, answers } = pendingFetch();
    const runtime = mount(
      `<input data-hs-bind='["q","text"]' data-hs-on-input='["call","GET","/find",["sig","q"],"busy",null,["latest"]]'>`,
      { q: "", results: "", busy: false },
    );
    const input = document.querySelector("input")!;
    for (const typed of ["t", "te"]) {
      input.value = typed;
      input.dispatchEvent(new Event("input", { bubbles: true }));
    }
    await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
    expect(fetch.mock.calls[0]![1].signal!.aborted).toBe(true);
    expect(runtime.store.get("busy")).toBe(true);
    answers[1]!(signals("te"));
    answers[0]!(signals("t"));
    await vi.waitFor(() => expect(runtime.store.get("busy")).toBe(false));
    expect(runtime.store.get("results")).toBe("te");
  });

  it("aborts no call sent without latest, and keeps the indicator on until every call answers", async () => {
    const { fetch, answers } = pendingFetch();
    const runtime = mount(`<button data-hs-on-click='["call","POST","/save",null,"busy",null]'>save</button>`, {
      results: "",
      busy: false,
    });
    const button = document.querySelector("button")!;
    button.click();
    button.click();
    await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
    expect(fetch.mock.calls.map(([, init]) => init.signal ?? null)).toEqual([null, null]);
    answers[0]!(signals("first"));
    await vi.waitFor(() => expect(runtime.store.get("results")).toBe("first"));
    expect(runtime.store.get("busy")).toBe(true);
    answers[1]!(signals("second"));
    await vi.waitFor(() => expect(runtime.store.get("busy")).toBe(false));
  });

  it("empties the error signal when a call is sent, so the same failure twice changes it twice", async () => {
    vi.spyOn(console, "error").mockImplementation(() => {});
    vi.stubGlobal(
      "fetch",
      vi.fn(async () => new Response("Name taken", { status: 409 })),
    );
    const runtime = mount(`<button data-hs-on-click='["call","POST","/save",null,null,"problem"]'>save</button>`, {
      problem: "",
    });
    const seen: unknown[] = [];
    runtime.store.subscribe((changes) => {
      if (changes.has("problem")) seen.push(runtime.store.get("problem"));
    });
    const button = document.querySelector("button")!;
    button.click();
    await vi.waitFor(() => expect(seen).toEqual(["Name taken"]));
    button.click();
    await vi.waitFor(() => expect(seen).toEqual(["Name taken", "", "Name taken"]));
    vi.restoreAllMocks();
  });

  describe("with reconnect", () => {
    afterEach(() => {
      vi.useRealTimers();
      vi.restoreAllMocks();
    });

    /** A stream that sends `body` and then fails, as a connection the server drops does. */
    function droppedResponse(body: string): Response {
      let sent = false;
      // `error` drops the chunks still queued, so the stream fails only once `body` has been read.
      const stream = new ReadableStream<Uint8Array>({
        pull(controller) {
          if (sent) controller.error(new TypeError("network error"));
          else controller.enqueue(new TextEncoder().encode(body));
          sent = true;
        },
      });
      return new Response(stream, { status: 200 });
    }

    const feed = `<p data-hs-on-load='["call","GET","/feed",null,null,"problem",["reconnect"]]'></p>`;

    it("sends the request again after a dropped stream and a refused connection, waiting longer each time", async () => {
      vi.useFakeTimers();
      vi.spyOn(console, "error").mockImplementation(() => {});
      const fetch = vi
        .fn()
        .mockResolvedValueOnce(droppedResponse(signals("one")))
        .mockRejectedValueOnce(new TypeError("Failed to fetch"))
        .mockResolvedValueOnce(sseResponse(signals("three")));
      vi.stubGlobal("fetch", fetch);
      const runtime = mount(feed, { results: "", problem: "" });
      await vi.advanceTimersByTimeAsync(0);
      expect(runtime.store.get("results")).toBe("one");
      expect(runtime.store.get("problem")).toContain("network error");
      expect(fetch).toHaveBeenCalledTimes(1);
      await vi.advanceTimersByTimeAsync(1000);
      expect(fetch).toHaveBeenCalledTimes(2);
      await vi.advanceTimersByTimeAsync(1999);
      expect(fetch).toHaveBeenCalledTimes(2);
      await vi.advanceTimersByTimeAsync(1);
      expect(fetch).toHaveBeenCalledTimes(3);
      expect(runtime.store.get("results")).toBe("three");
      expect(runtime.store.get("problem")).toBe("");
      await vi.advanceTimersByTimeAsync(60_000);
      expect(fetch).toHaveBeenCalledTimes(3);
    });

    it("evaluates the arguments again for each attempt, so the stream can start after what the page holds", async () => {
      vi.useFakeTimers();
      vi.spyOn(console, "error").mockImplementation(() => {});
      const fetch = vi
        .fn()
        .mockResolvedValueOnce(droppedResponse(`event: hs-signals\ndata: {"last":7}\n\n`))
        .mockReturnValueOnce(new Promise<Response>(() => {}));
      vi.stubGlobal("fetch", fetch);
      mount(`<p data-hs-on-load='["call","GET","/feed",["sig","last"],null,null,["reconnect"]]'></p>`, { last: 3 });
      await vi.advanceTimersByTimeAsync(0);
      await vi.advanceTimersByTimeAsync(1000);
      expect(fetch).toHaveBeenCalledTimes(2);
      const sent = fetch.mock.calls.map(([url]) => (url as URL).searchParams.get("args"));
      expect(sent).toEqual(["3", "7"]);
    });

    it("sets a connected flag only while the stream's answer is open", async () => {
      vi.useFakeTimers();
      vi.spyOn(console, "error").mockImplementation(() => {});
      let drop!: () => void;
      const open = new ReadableStream<Uint8Array>({
        start(controller) {
          drop = () => controller.error(new TypeError("network error"));
        },
      });
      let connect!: (response: Response) => void;
      const fetch = vi
        .fn()
        .mockReturnValueOnce(new Promise<Response>((resolve) => (connect = resolve)))
        .mockReturnValueOnce(new Promise<Response>(() => {}));
      vi.stubGlobal("fetch", fetch);
      const runtime = mount(`<p data-hs-on-load='["call","GET","/feed",null,"live",null,["reconnect","connected"]]'></p>`, {
        live: false,
      });
      await vi.advanceTimersByTimeAsync(0);
      expect(fetch).toHaveBeenCalledTimes(1);
      expect(runtime.store.get("live")).toBe(false);
      connect(new Response(open, { status: 200 }));
      await vi.advanceTimersByTimeAsync(0);
      expect(runtime.store.get("live")).toBe(true);
      drop();
      await vi.advanceTimersByTimeAsync(0);
      expect(runtime.store.get("live")).toBe(false);
      await vi.advanceTimersByTimeAsync(1000);
      expect(fetch).toHaveBeenCalledTimes(2);
      expect(runtime.store.get("live")).toBe(false);
    });

    it("does not send the request again after a 4xx answer or once its element has left the page", async () => {
      vi.useFakeTimers();
      vi.spyOn(console, "error").mockImplementation(() => {});
      const fetch = vi.fn().mockResolvedValue(new Response("bad args", { status: 400 }));
      vi.stubGlobal("fetch", fetch);
      mount(feed, { results: "", problem: "" });
      await vi.advanceTimersByTimeAsync(5000);
      expect(fetch).toHaveBeenCalledTimes(1);

      fetch.mockRejectedValue(new TypeError("Failed to fetch"));
      mount(feed, { results: "", problem: "" });
      await vi.advanceTimersByTimeAsync(0);
      expect(fetch).toHaveBeenCalledTimes(2);
      document.querySelector("p")!.remove();
      await vi.advanceTimersByTimeAsync(5000);
      expect(fetch).toHaveBeenCalledTimes(2);
    });
  });
});

describe("lists", () => {
  it("remove every item at once where the list fills its parent, keeping the template to render into again", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ul>` +
        `<ol id="m"><li>head</li><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]' data-hs-text='["item",0,"label"]'></li></template></ol>`,
      { rows: [1, 2, 3].map((id) => ({ id, label: `r${id}` })) },
    );
    await tick();
    runtime.store.set("rows", [4, 5].map((id) => ({ id, label: `r${id}` })));
    await tick();
    const texts = (id: string) => [...document.querySelectorAll(`#${id} li`)].map((li) => li.textContent);
    expect(texts("l")).toEqual(["r4", "r5"]);
    expect(texts("m")).toEqual(["head", "r4", "r5"]);
    runtime.store.set("rows", []);
    await tick();
    expect(document.querySelector("#l")!.children.length).toBe(1);
    expect(texts("m")).toEqual(["head"]);
    runtime.store.set("rows", [{ id: 6, label: "r6" }]);
    await tick();
    expect(texts("l")).toEqual(["r6"]);
    expect(texts("m")).toEqual(["head", "r6"]);
  });
});

describe("statements", () => {
  it("render once for all the steps of a sequence", async () => {
    const runtime = mount(
      `<button data-hs-on-click='["seq",[["set","rows",["lit",[]]],["push","rows",["lit",1]],["push","rows",["lit",2]]]]'></button>`,
      { rows: [0] },
    );
    await tick();
    const seen: unknown[] = [];
    runtime.store.subscribe(() => seen.push(runtime.store.get("rows")));
    document.querySelector("button")!.click();
    await tick();
    expect(seen).toEqual([[1, 2]]);
  });

  it("read a computed signal that an earlier step of the sequence changed", async () => {
    const root = document.createElement("main");
    root.setAttribute("data-hs-computed", JSON.stringify([["total", ["add", ["sig", "n"], ["lit", 1]]]]));
    root.innerHTML = `<button data-hs-on-click='["seq",[["set","n",["lit",5]],["set","copy",["sig","total"]]]]'></button>`;
    document.body.replaceChildren(root);
    const runtime = new Runtime(root, { n: 0, total: 1, copy: 0 });
    await tick();
    document.querySelector("button")!.click();
    await tick();
    expect(runtime.store.get("copy")).toBe(6);
  });

  it("record in an effect the reads of every step of a sequence", async () => {
    const runtime = mount(`<div data-hs-effect='[["seq",[["set","a",["sig","x"]],["set","b",["sig","y"]]]]]'></div>`, {
      x: 1,
      y: 2,
      a: 0,
      b: 0,
    });
    await tick();
    runtime.store.set("y", 3);
    await tick();
    expect(runtime.store.get("b")).toBe(3);
  });
});
