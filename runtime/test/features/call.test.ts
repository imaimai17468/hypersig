import { afterEach, expect, it, vi } from "vitest";
import { $, $$, tick, type } from "../dom.ts";
import type { Fetch } from "../runtime/mount.ts";
import { mount } from "./fixture.ts";

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

it("sets the indicator during a call and the error after a failed one", async () => {
  let finish!: (response: Response) => void;
  vi.stubGlobal(
    "fetch",
    vi.fn<Fetch>(
      () =>
        new Promise<Response>((resolve) => {
          finish = resolve;
        }),
    ),
  );
  vi.spyOn(console, "error").mockImplementation(() => {});
  const runtime = mount("timers");
  $("#save").click();
  await tick();
  expect(runtime.store.get("saving")).toBe(true);
  finish(new Response("You can keep 3 notes.\n", { status: 409 }));
  await vi.waitFor(() => expect(runtime.store.get("saving")).toBe(false));
  expect(runtime.store.get("error")).toBe("You can keep 3 notes.");
  expect(console.error).toHaveBeenCalledWith(
    "hypersig: POST /search failed with 409: You can keep 3 notes.",
  );
});

it("writes the status and its reason to the error when the failed answer has no body", async () => {
  vi.stubGlobal(
    "fetch",
    vi.fn<Fetch>(() =>
      Promise.resolve(new Response(null, { status: 502, statusText: "Bad Gateway" })),
    ),
  );
  vi.spyOn(console, "error").mockImplementation(() => {});
  const runtime = mount("timers");
  $("#save").click();
  await vi.waitFor(() => expect(runtime.store.get("error")).toBe("502 Bad Gateway"));
  expect(console.error).toHaveBeenCalledWith("hypersig: POST /search failed with 502 Bad Gateway");
});

it("aborts the earlier latest call and applies only the newer answer", async () => {
  const answers: ((response: Response) => void)[] = [];
  const fetch = vi.fn<Fetch>(
    (_url, init) =>
      new Promise<Response>((resolve, reject) => {
        init.signal?.addEventListener("abort", () =>
          reject(new DOMException("aborted", "AbortError")),
        );
        answers.push(resolve);
      }),
  );
  vi.stubGlobal("fetch", fetch);
  const runtime = mount("timers");
  type($<HTMLInputElement>("#find"), "t");
  type($<HTMLInputElement>("#find"), "te");
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
  expect(fetch.mock.calls.map(([url]) => url.searchParams.get("args"))).toEqual(['"t"', '"te"']);
  expect(fetch.mock.calls[0]![1].signal!.aborted).toBe(true);
  answers[1]!(new Response('event: hs-signals\ndata: {"found":"te"}\n\n', { status: 200 }));
  answers[0]!(new Response('event: hs-signals\ndata: {"found":"t"}\n\n', { status: 200 }));
  await vi.waitFor(() => expect(runtime.store.get("finding")).toBe(false));
  expect(runtime.store.get("found")).toBe("te");
});

it("sends a reconnect call again after the connection fails", async () => {
  vi.useFakeTimers();
  vi.spyOn(console, "error").mockImplementation(() => {});
  const fetch = vi
    .fn<Fetch>()
    .mockRejectedValueOnce(new TypeError("Failed to fetch"))
    .mockResolvedValueOnce(
      new Response('event: hs-signals\ndata: {"found":"back"}\n\nevent: hs-end\ndata: {}\n\n', {
        status: 200,
      }),
    );
  vi.stubGlobal("fetch", fetch);
  const runtime = mount("timers");
  $("#follow").click();
  await vi.advanceTimersByTimeAsync(0);
  expect(runtime.store.get("error")).toContain("Failed to fetch");
  await vi.advanceTimersByTimeAsync(1000);
  expect(fetch).toHaveBeenCalledTimes(2);
  expect(runtime.store.get("found")).toBe("back");
  expect(runtime.store.get("error")).toBe("");
});

it("marks only the line whose call failed, with the values named when it was sent", async () => {
  const answers = new Map<string, (response: Response) => void>();
  vi.stubGlobal(
    "fetch",
    vi.fn<Fetch>(
      (_url, init) =>
        new Promise<Response>((resolve) => {
          answers.set(String(init.body), resolve);
        }),
    ),
  );
  vi.spyOn(console, "error").mockImplementation(() => {});
  const runtime = mount("outbox");
  const states = () =>
    (runtime.store.get("letters") as { state: string }[]).map((letter) => letter.state);
  type($<HTMLInputElement>("#draft"), "first");
  $("#send").click();
  await tick();
  type($<HTMLInputElement>("#draft"), "second");
  $("#send").click();
  await vi.waitFor(() => expect(answers.size).toBe(2));
  expect(states()).toEqual(["sending", "sending"]);

  answers.get("1")!(new Response("The outbox is full.", { status: 409 }));
  answers.get("2")!(new Response("event: hs-signals\ndata: {}\n\n", { status: 200 }));
  await vi.waitFor(() => expect(states()).toEqual(["The outbox is full.", "sent"]));

  answers.clear();
  $$("#letters .edit")[0]!.click();
  expect(states()).toEqual(["sending", "sent"]);
  await vi.waitFor(() => expect(answers.size).toBe(1));
  answers.get("1")!(new Response("event: hs-signals\ndata: {}\n\n", { status: 200 }));
  await vi.waitFor(() => expect(states()).toEqual(["sent", "sent"]));
});
