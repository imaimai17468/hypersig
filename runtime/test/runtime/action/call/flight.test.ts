import { afterEach, expect, it, vi } from "vitest";
import { $ } from "../../../dom.ts";
import { type Fetch, mount } from "../../mount.ts";

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

const signals = (results: string) => `event: hs-signals\ndata: {"results":"${results}"}\n\n`;
const end = "event: hs-end\ndata: {}\n\n";

/** A fetch whose answers the test writes to and closes, which fails once its request is aborted. */
function streamingFetch() {
  const answers: { push: (text: string) => void; close: () => void }[] = [];
  const fetch = vi.fn<Fetch>(
    (_url, init) =>
      new Promise<Response>((resolve, reject) => {
        init.signal?.addEventListener("abort", () =>
          reject(new DOMException("aborted", "AbortError")),
        );
        const body = new ReadableStream<Uint8Array>({
          start(controller) {
            answers.push({
              push: (text) => controller.enqueue(new TextEncoder().encode(text)),
              close: () => controller.close(),
            });
          },
        });
        resolve(
          new Response(body, { status: 200, headers: { "content-type": "text/event-stream" } }),
        );
      }),
  );
  vi.stubGlobal("fetch", fetch);
  return { fetch, answers };
}

it("skips an exclusive call while another call to the action awaits its answer", async () => {
  const { fetch, answers } = streamingFetch();
  mount(
    `<button data-hs-on-click='["call","POST","/order",null,"busy",null,["exclusive"]]'>order</button>`,
    { busy: false, results: "" },
  );
  $("button").click();
  $("button").click();
  await vi.waitFor(() => expect(answers).toHaveLength(1));
  expect(fetch).toHaveBeenCalledOnce();
  answers[0]!.push(end);
  answers[0]!.close();
  await vi.waitFor(() => expect($("button").hasAttribute("aria-busy")).toBe(false));
  $("button").click();
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
});

it("marks the element that sent a POST busy until every call it sent has its answer", async () => {
  const { answers } = streamingFetch();
  mount(`<button data-hs-on-click='["call","POST","/save",null,null,null]'>save</button>`, {
    results: "",
  });
  $("button").click();
  $("button").click();
  await vi.waitFor(() => expect(answers).toHaveLength(2));
  expect($("button").getAttribute("aria-busy")).toBe("true");
  answers[0]!.push(end);
  answers[0]!.close();
  await new Promise((done) => {
    setTimeout(done, 10);
  });
  expect($("button").getAttribute("aria-busy")).toBe("true");
  answers[1]!.push(end);
  answers[1]!.close();
  await vi.waitFor(() => expect($("button").hasAttribute("aria-busy")).toBe(false));
});

it("leaves the aria-busy of a GET's element and one the page set alone", async () => {
  const { answers } = streamingFetch();
  mount(
    `<p data-hs-on-load='["call","GET","/feed",null,null,null]'></p>` +
      `<button aria-busy="false" data-hs-on-click='["call","POST","/save",null,null,null]'>save</button>`,
    { results: "" },
  );
  $("button").click();
  await vi.waitFor(() => expect(answers).toHaveLength(2));
  expect($("p").hasAttribute("aria-busy")).toBe(false);
  expect($("button").getAttribute("aria-busy")).toBe("false");
});

it("drops the events of a POST that arrive after a later POST to the action has its complete answer", async () => {
  const { answers } = streamingFetch();
  const runtime = mount(
    `<button data-hs-on-click='["call","POST","/rename",null,null,null]'>rename</button>`,
    { results: "" },
  );
  $("button").click();
  $("button").click();
  await vi.waitFor(() => expect(answers).toHaveLength(2));
  answers[1]!.push(signals("second") + end);
  answers[1]!.close();
  await vi.waitFor(() => expect(runtime.store.get("results")).toBe("second"));
  answers[0]!.push(signals("first") + end);
  answers[0]!.close();
  await new Promise((done) => {
    setTimeout(done, 10);
  });
  expect(runtime.store.get("results")).toBe("second");
});

it("fails a call whose answer has not ended once its timeout has passed", async () => {
  vi.useFakeTimers();
  vi.spyOn(console, "error").mockImplementation(() => {});
  const { answers } = streamingFetch();
  const runtime = mount(
    `<button data-hs-on-click='["call","POST","/slow",null,"busy","problem",[],{"status":"failure","timeout":5000}]'>go</button>`,
    { busy: false, problem: "", failure: -1, results: "" },
  );
  $("button").click();
  await vi.advanceTimersByTimeAsync(0);
  answers[0]!.push(signals("partial"));
  await vi.advanceTimersByTimeAsync(4999);
  expect(runtime.store.get("busy")).toBe(true);
  await vi.advanceTimersByTimeAsync(1);
  expect(runtime.store.get("busy")).toBe(false);
  expect(runtime.store.get("problem")).toBe("the answer took longer than 5000 ms");
  expect(runtime.store.get("failure")).toBe(0);
  expect(runtime.store.get("results")).toBe("partial");
  // The runtime cancelled the body, so the server can write nothing more to it.
  expect(() => answers[0]!.push(signals("late"))).toThrow("Controller is already closed");
});
