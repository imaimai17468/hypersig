import { afterEach, expect, it, vi } from "vitest";
import { followServer, keepSignals, reloadOnRestart, restoreSignals } from "../src/dev.ts";
import { Runtime } from "../src/runtime.ts";
import { Navigation } from "../src/runtime/navigation.ts";

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

function events(text: string): Response {
  return new Response(text, { headers: { "Content-Type": "text/event-stream" } });
}

/** A stream the server holds open until the page closes it with `signal`. */
function held(signal: AbortSignal): Response {
  const body = new ReadableStream<Uint8Array>({
    start(controller) {
      signal.addEventListener("abort", () => controller.error(signal.reason));
    },
  });
  return new Response(body, { headers: { "Content-Type": "text/event-stream" } });
}

it("asks again while no server answers, and reloads once another server does", async () => {
  vi.useFakeTimers();
  const reload = vi.spyOn(location, "reload").mockImplementation(() => {});
  const fetch = vi
    .fn<typeof globalThis.fetch>()
    // The server that rendered the page holds the stream until it stops.
    .mockResolvedValueOnce(events(": keep-alive\n\n"))
    .mockRejectedValueOnce(new TypeError("Failed to fetch"))
    .mockResolvedValueOnce(events("event: hs-reload\ndata: {}\n\n"));
  vi.stubGlobal("fetch", fetch);
  const navigation = new Navigation();

  reloadOnRestart("/_hypersig/dev?since=a1", navigation);
  await vi.advanceTimersByTimeAsync(1000);

  expect(fetch).toHaveBeenCalledTimes(3);
  expect(fetch.mock.calls.map(([url]) => url)).toEqual([
    "/_hypersig/dev?since=a1",
    "/_hypersig/dev?since=a1",
    "/_hypersig/dev?since=a1",
  ]);
  expect(reload).toHaveBeenCalledOnce();
  expect(navigation.leaving).toBe(true);
});

it("closes a stream it has held for two seconds and asks again", async () => {
  vi.useFakeTimers();
  const reload = vi.spyOn(location, "reload").mockImplementation(() => {});
  const fetch = vi.fn<typeof globalThis.fetch>((_url, init) =>
    Promise.resolve(held(init?.signal as AbortSignal)),
  );
  vi.stubGlobal("fetch", fetch);

  reloadOnRestart("/_hypersig/dev?since=a1", new Navigation());
  await vi.advanceTimersByTimeAsync(1999);
  expect(fetch).toHaveBeenCalledOnce();
  expect(fetch.mock.calls[0]?.[1]?.signal?.aborted).toBe(false);
  await vi.advanceTimersByTimeAsync(1 + 250);

  expect(fetch.mock.calls[0]?.[1]?.signal?.aborted).toBe(true);
  expect(fetch).toHaveBeenCalledTimes(2);
  expect(reload).not.toHaveBeenCalled();
});

it("asks less often once no server has answered for three seconds", async () => {
  vi.useFakeTimers();
  const fetch = vi
    .fn<typeof globalThis.fetch>()
    .mockRejectedValue(new TypeError("Failed to fetch"));
  vi.stubGlobal("fetch", fetch);

  reloadOnRestart("/_hypersig/dev?since=a1", new Navigation());
  await vi.advanceTimersByTimeAsync(3000);
  const fast = fetch.mock.calls.length;
  await vi.advanceTimersByTimeAsync(3000);

  // Every 50 ms for the first three seconds, then once a second.
  expect(fast).toBeGreaterThanOrEqual(55);
  expect(fetch.mock.calls.length - fast).toBeLessThanOrEqual(4);
});

it("stops asking a server that serves no stream", async () => {
  vi.useFakeTimers();
  const reload = vi.spyOn(location, "reload").mockImplementation(() => {});
  const fetch = vi
    .fn<typeof globalThis.fetch>()
    .mockResolvedValue(new Response("", { status: 404 }));
  vi.stubGlobal("fetch", fetch);

  reloadOnRestart("/_hypersig/dev?since=a1", new Navigation());
  await vi.advanceTimersByTimeAsync(1000);

  expect(fetch).toHaveBeenCalledOnce();
  expect(reload).not.toHaveBeenCalled();
});

it("stops asking once the page is loading another", () => {
  const fetch = vi.fn<typeof globalThis.fetch>();
  vi.stubGlobal("fetch", fetch);
  const navigation = new Navigation();
  navigation.leaving = true;

  reloadOnRestart("/_hypersig/dev?since=a1", navigation);

  expect(fetch).not.toHaveBeenCalled();
});

it("keeps the signals just before the reload", async () => {
  vi.useFakeTimers();
  const order: string[] = [];
  vi.spyOn(location, "reload").mockImplementation(() => order.push("reload"));
  vi.stubGlobal(
    "fetch",
    vi.fn<typeof globalThis.fetch>().mockResolvedValue(events("event: hs-reload\ndata: {}\n\n")),
  );

  reloadOnRestart("/_hypersig/dev?since=a1", new Navigation(), () => order.push("keep"));
  await vi.advanceTimersByTimeAsync(100);

  expect(order).toEqual(["keep", "reload"]);
});

it("counts a signal as the visitor's only where it differs from the server's last write", () => {
  const runtime = new Runtime(document.createElement("div"), { draft: "", todos: [] });
  const changed = followServer(runtime, { draft: "", todos: [] });

  runtime.store.set("draft", "half a");
  runtime.apply({ event: "hs-signals", data: JSON.stringify({ todos: [{ id: 1 }] }) });

  // The server wrote `todos`, so the new server's list stands after a reload.
  expect(changed()).toEqual({ draft: "half a" });

  runtime.apply({ event: "hs-signals", data: JSON.stringify({ draft: "" }) });

  expect(changed()).toEqual({});
});

it("restores the kept signals over the new server's values", () => {
  keepSignals("draft:String,count:i32", { draft: "half a" });

  const restored = restoreSignals("draft:String,count:i32", { draft: "", count: 7 });

  expect(restored).toEqual({ draft: "half a", count: 7 });
  // The kept signals are read once.
  expect(restoreSignals("draft:String,count:i32", { draft: "", count: 7 })).toEqual({
    draft: "",
    count: 7,
  });
});

it("restores nothing into a struct of another shape", () => {
  keepSignals("draft:String", { draft: "half a" });

  expect(restoreSignals("draft:i32", { draft: 0 })).toEqual({ draft: 0 });
  expect(sessionStorage.length).toBe(0);
});

it("restores nothing into a page of another route", () => {
  keepSignals("draft:String", { draft: "half a" });
  const shown = location.href;
  history.replaceState(null, "", "/elsewhere");

  try {
    expect(restoreSignals("draft:String", { draft: "" })).toEqual({ draft: "" });
  } finally {
    history.replaceState(null, "", shown);
  }
});
