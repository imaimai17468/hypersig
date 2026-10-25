import { afterEach, expect, it, vi } from "vitest";
import { reloadOnRestart } from "../src/dev.ts";
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
