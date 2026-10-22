import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { $ } from "../../../dom.ts";
import { type Fetch, mount, sseResponse } from "../../mount.ts";

beforeEach(() => {
  vi.useFakeTimers();
  vi.spyOn(console, "error").mockImplementation(() => {});
});

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

const signals = (results: string) => `event: hs-signals\ndata: {"results":"${results}"}\n\n`;

/** The event the server sends last on a stream whose handler finished. */
const end = "event: hs-end\ndata: {}\n\n";

const feed = `<p data-hs-on-load='["call","GET","/feed",null,null,"problem",["reconnect"]]'></p>`;

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

it("sends the request again after a dropped stream and a refused connection, waiting longer each time", async () => {
  const fetch = vi
    .fn<Fetch>()
    .mockResolvedValueOnce(droppedResponse(signals("one")))
    .mockRejectedValueOnce(new TypeError("Failed to fetch"))
    .mockResolvedValueOnce(sseResponse(signals("three") + end));
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

it("opens a stream again once 1 second after the host closes it before the server ended it", async () => {
  const fetch = vi
    .fn<Fetch>()
    .mockResolvedValueOnce(sseResponse(signals("one")))
    .mockResolvedValueOnce(sseResponse(signals("two") + end));
  vi.stubGlobal("fetch", fetch);
  const runtime = mount(feed, { results: "", problem: "" });
  await vi.advanceTimersByTimeAsync(0);
  expect(runtime.store.get("results")).toBe("one");
  expect(runtime.store.get("problem")).toBe("the stream closed before the server ended it");
  await vi.advanceTimersByTimeAsync(1000);
  expect(fetch).toHaveBeenCalledTimes(2);
  expect(runtime.store.get("results")).toBe("two");
  expect(runtime.store.get("problem")).toBe("");
  await vi.advanceTimersByTimeAsync(60_000);
  expect(fetch).toHaveBeenCalledTimes(2);
});

it("applies no part of an event the host cut off in the middle", async () => {
  vi.stubGlobal(
    "fetch",
    vi
      .fn<Fetch>()
      .mockResolvedValueOnce(
        sseResponse(signals("one") + 'event: hs-signals\ndata: {"results":"tw'),
      )
      .mockReturnValueOnce(new Promise<Response>(() => {})),
  );
  const runtime = mount(feed, { results: "", problem: "" });
  await vi.advanceTimersByTimeAsync(0);
  expect(runtime.store.get("results")).toBe("one");
  expect(runtime.store.get("problem")).toBe("the stream closed before the server ended it");
});

it("evaluates the arguments again for each attempt, so the stream can start after what the page holds", async () => {
  const fetch = vi
    .fn<Fetch>()
    .mockResolvedValueOnce(droppedResponse(`event: hs-signals\ndata: {"last":7}\n\n`))
    .mockReturnValueOnce(new Promise<Response>(() => {}));
  vi.stubGlobal("fetch", fetch);
  mount(`<p data-hs-on-load='["call","GET","/feed",["sig","last"],null,null,["reconnect"]]'></p>`, {
    last: 3,
  });
  await vi.advanceTimersByTimeAsync(0);
  await vi.advanceTimersByTimeAsync(1000);
  expect(fetch).toHaveBeenCalledTimes(2);
  const sent = fetch.mock.calls.map(([url]) => url.searchParams.get("args"));
  expect(sent).toEqual(["3", "7"]);
});

it("sets a connected flag only while the stream's answer is open", async () => {
  let drop!: () => void;
  const open = new ReadableStream<Uint8Array>({
    start(controller) {
      drop = () => controller.error(new TypeError("network error"));
    },
  });
  let connect!: (response: Response) => void;
  const fetch = vi
    .fn<Fetch>()
    .mockReturnValueOnce(
      new Promise<Response>((resolve) => {
        connect = resolve;
      }),
    )
    .mockReturnValueOnce(new Promise<Response>(() => {}));
  vi.stubGlobal("fetch", fetch);
  const runtime = mount(
    `<p data-hs-on-load='["call","GET","/feed",null,"live",null,["reconnect","connected"]]'></p>`,
    {
      live: false,
    },
  );
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
  const fetch = vi.fn<Fetch>().mockResolvedValue(new Response("bad args", { status: 400 }));
  vi.stubGlobal("fetch", fetch);
  mount(feed, { results: "", problem: "" });
  await vi.advanceTimersByTimeAsync(5000);
  expect(fetch).toHaveBeenCalledTimes(1);

  fetch.mockRejectedValue(new TypeError("Failed to fetch"));
  mount(feed, { results: "", problem: "" });
  await vi.advanceTimersByTimeAsync(0);
  expect(fetch).toHaveBeenCalledTimes(2);
  $("p").remove();
  await vi.advanceTimersByTimeAsync(5000);
  expect(fetch).toHaveBeenCalledTimes(2);
});
