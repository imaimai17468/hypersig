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
    const runtime = mount(`<input id="i" data-hs-bind="name">`, { name: "a" });
    const input = document.getElementById("i") as HTMLInputElement;
    expect(input.value).toBe("a");
    input.value = "ab";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    expect(runtime.store.get("name")).toBe("ab");
    runtime.store.set("name", "");
    await tick();
    expect(input.value).toBe("");
  });

  it("posts the signals to an action and applies the patches it returns", async () => {
    const fetch = vi.fn(async (_url: URL, _init: RequestInit) =>
      sseResponse(
        'event: hs-signals\ndata: {"draft":""}\n\n' +
          'event: hs-element\ndata: {"html":"<ul id=\\"list\\"><li>tea</li></ul>"}\n\n',
      ),
    );
    vi.stubGlobal("fetch", fetch);
    const runtime = mount(
      `<form data-hs-on-submit='["call","POST","/add",{"id":3}]'><input data-hs-bind="draft"><button id="go">go</button></form><ul id="list"></ul>`,
      { draft: "tea" },
    );
    document.querySelector("form")!.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
    await vi.waitFor(() => expect(document.getElementById("list")!.innerHTML).toBe("<li>tea</li>"));

    const [url, init] = fetch.mock.calls[0]!;
    expect(url.pathname).toBe("/add");
    expect(url.searchParams.get("args")).toBe('{"id":3}');
    expect(init.method).toBe("POST");
    expect(init.body).toBe('{"draft":"tea"}');
    expect(runtime.store.get("draft")).toBe("");
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
