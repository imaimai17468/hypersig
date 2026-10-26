import { afterEach, expect, it, vi } from "vitest";
import type { Runtime } from "../../../src/runtime.ts";
import { $ } from "../../dom.ts";
import { type Fetch, mount, sseResponse } from "../mount.ts";

afterEach(() => {
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

const signals = (results: string) => `event: hs-signals\ndata: {"results":"${results}"}\n\n`;

/** A fetch whose answers the test settles one by one, which rejects once its request is aborted. */
function pendingFetch() {
  const answers: ((body: string) => void)[] = [];
  const fetch = vi.fn<Fetch>(
    (_url, init) =>
      new Promise<Response>((resolve, reject) => {
        init.signal?.addEventListener("abort", () =>
          reject(new DOMException("aborted", "AbortError")),
        );
        answers.push((body) => resolve(sseResponse(body)));
      }),
  );
  vi.stubGlobal("fetch", fetch);
  return { fetch, answers };
}

/** Whether the request sent with `init` was aborted. */
function aborted(init: RequestInit): boolean {
  return init.signal?.aborted === true;
}

/** The values the signal at `path` takes, one for each render that changes it. */
function watch(runtime: Runtime, path: string): unknown[] {
  const seen: unknown[] = [];
  runtime.store.subscribe((changes) => {
    if (changes.has(path)) seen.push(runtime.store.get(path));
  });
  return seen;
}

it("posts the arguments to an action, and nothing else, and applies the patches it returns", async () => {
  const fetch = vi.fn<Fetch>(() =>
    Promise.resolve(
      sseResponse(
        'event: hs-signals\ndata: {"draft":""}\n\n' +
          'event: hs-element\ndata: {"html":"<ul id=\\"list\\"><li>tea</li></ul>"}\n\n',
      ),
    ),
  );
  vi.stubGlobal("fetch", fetch);
  const runtime = mount(
    `<form data-hs-on-submit='["call","POST","/add",["lit",{"id":3}],null,null]'><input data-hs-bind='["draft","text"]'><button id="go">go</button></form><ul id="list"></ul>`,
    { draft: "tea" },
  );
  $("form").dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
  await vi.waitFor(() => expect($("#list").innerHTML).toBe("<li>tea</li>"));

  const [url, init] = fetch.mock.calls[0]!;
  expect(url.pathname).toBe("/add");
  expect(url.search).toBe("");
  expect(init.method).toBe("POST");
  expect(init.body).toBe('{"id":3}');
  expect(runtime.store.get("draft")).toBe("");
});

it("runs the statement after a call without waiting for the answer", async () => {
  vi.stubGlobal(
    "fetch",
    vi.fn<Fetch>(() => new Promise<Response>(() => {})),
  );
  const runtime = mount(
    `<button data-hs-on-click='["seq",[["call","POST","/slow",["sig","n"],null,null],["set","n",["lit",2]]]]'>go</button>`,
    { n: 1 },
  );
  $("button").click();
  await vi.waitFor(() => expect(runtime.store.get("n")).toBe(2));
});

it("aborts the earlier latest call of the same action, so its answer cannot overwrite a newer one", async () => {
  const { fetch, answers } = pendingFetch();
  const runtime = mount(
    `<input data-hs-bind='["q","text"]' data-hs-on-input='["call","GET","/find",["sig","q"],"busy",null,["latest"]]'>`,
    { q: "", results: "", busy: false },
  );
  const input = $<HTMLInputElement>("input");
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
  const runtime = mount(
    `<button data-hs-on-click='["call","POST","/save",null,"busy",null]'>save</button>`,
    {
      results: "",
      busy: false,
    },
  );
  const button = $("button");
  button.click();
  button.click();
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
  expect(fetch.mock.calls.map(([, init]) => aborted(init))).toEqual([false, false]);
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
    vi.fn<Fetch>(() => Promise.resolve(new Response("Name taken", { status: 409 }))),
  );
  const runtime = mount(
    `<button data-hs-on-click='["call","POST","/save",null,null,"problem"]'>save</button>`,
    {
      problem: "",
    },
  );
  const seen = watch(runtime, "problem");
  const button = $("button");
  button.click();
  await vi.waitFor(() => expect(seen).toEqual(["Name taken"]));
  button.click();
  await vi.waitFor(() => expect(seen).toEqual(["Name taken", "", "Name taken"]));
});

/** A fetch whose one answer is a stream the test writes events to. */
function openStream() {
  let push!: (text: string) => void;
  const body = new ReadableStream<Uint8Array>({
    start(controller) {
      push = (text) => controller.enqueue(new TextEncoder().encode(text));
    },
  });
  const fetch = vi.fn<Fetch>(() => Promise.resolve(new Response(body, { status: 200 })));
  vi.stubGlobal("fetch", fetch);
  return { fetch, push: (text: string) => push(text) };
}

const feed = `<p data-hs-on-load='["call","GET","/feed",null,null,null]'></p>`;

it("ends a GET stream whose element left the page, and applies none of its later events", async () => {
  const { fetch, push } = openStream();
  const runtime = mount(feed, { results: "" });
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
  push(signals("one"));
  await vi.waitFor(() => expect(runtime.store.get("results")).toBe("one"));
  $("p").remove();
  push(signals("two"));
  await vi.waitFor(() => expect(aborted(fetch.mock.calls[0]![1])).toBe(true));
  expect(runtime.store.get("results")).toBe("one");
});

it("ends a quiet GET stream once a patch has taken its element out of the page", async () => {
  const { fetch } = openStream();
  const runtime = mount(`<div id="box">${feed}</div>`, { results: "" });
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
  runtime.apply({ event: "hs-element", data: JSON.stringify({ html: `<div id="box"></div>` }) });
  await vi.waitFor(() => expect(aborted(fetch.mock.calls[0]![1])).toBe(true));
});

it("keeps applying the answer of a POST whose element left the page", async () => {
  const { fetch, answers } = pendingFetch();
  const runtime = mount(
    `<button data-hs-on-click='["call","POST","/save",null,null,null]'>save</button>`,
    { results: "" },
  );
  $("button").click();
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
  $("button").remove();
  answers[0]!(signals("saved"));
  await vi.waitFor(() => expect(runtime.store.get("results")).toBe("saved"));
});
