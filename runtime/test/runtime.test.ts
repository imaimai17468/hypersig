import { afterEach, describe, expect, it, vi } from "vitest";
import { Runtime } from "../src/runtime.ts";

const tick = () => new Promise((resolve) => setTimeout(resolve, 0));

function mount(html: string, signals: Record<string, unknown>): Runtime {
  const root = document.createElement("main");
  root.innerHTML = html;
  document.body.replaceChildren(root);
  return new Runtime(root, signals);
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
    const reads: string[] = [];
    const get = runtime.store.get.bind(runtime.store);
    runtime.store.get = (path: string) => {
      reads.push(path);
      return get(path);
    };
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

  it("moves the reads of items that shift unchanged, and reruns the bindings that read their position", async () => {
    const runtime = mount(
      `<ul id="l"><template data-hs-each='["rows",0]'><li data-hs-key='["item",0,"id"]'>` +
        `<b data-hs-text='["str",["index",0]]'></b><i data-hs-text='["item",0,"label"]'></i>` +
        `<s data-hs-text='["concat",["item",0,"label"],["sig","suffix"]]'></s></li></template></ul>`,
      { rows: [1, 2, 3, 4].map((id) => ({ id, label: `r${id}` })), suffix: "" },
    );
    await tick();
    const [first, ...rest] = [...document.querySelectorAll("#l li")];
    const reads: string[] = [];
    const get = runtime.store.get.bind(runtime.store);
    runtime.store.get = (path: string) => {
      reads.push(path);
      return get(path);
    };
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
