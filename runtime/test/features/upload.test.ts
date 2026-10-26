import { afterEach, expect, it, vi } from "vitest";
import { $, tick, type } from "../dom.ts";
import type { Fetch } from "../runtime/mount.ts";
import { mount } from "./fixture.ts";

afterEach(() => {
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

/** Puts `files` in the file input, as picking them would. jsdom has no `DataTransfer` to build a `FileList`. */
function pick(files: File[]): void {
  const input = $<HTMLInputElement>("#pictures");
  Object.defineProperty(input, "files", { value: files, configurable: true });
}

/** The names of the files the `call`th request sent. */
function sent(fetch: { mock: { calls: Parameters<Fetch>[] } }, call: number): string[] {
  return ((fetch.mock.calls[call]![1].body as FormData).getAll("files") as File[]).map(
    (file) => file.name,
  );
}

it("sends the arguments and the picked files as multipart with the upload header", async () => {
  const fetch = vi.fn<Fetch>(() =>
    Promise.resolve(
      new Response('event: hs-signals\ndata: {"saved":"2"}\n\nevent: hs-end\ndata: {}\n\n', {
        status: 200,
        headers: { "content-type": "text/event-stream" },
      }),
    ),
  );
  vi.stubGlobal("fetch", fetch);
  const runtime = mount("upload");
  type($<HTMLInputElement>("#caption"), "beach");
  pick([new File(["one"], "a.png", { type: "image/png" }), new File(["two"], "b.txt")]);
  $("#send").click();
  await vi.waitFor(() => expect(runtime.store.get("saved")).toBe("2"));
  const [url, init] = fetch.mock.calls[0]!;
  expect(url.pathname).toBe("/pictures");
  expect(init.method).toBe("POST");
  expect(init.headers).toEqual({
    accept: "text/event-stream",
    "hypersig-call": "1",
    "hypersig-upload": "1",
  });
  const body = init.body as FormData;
  expect(await (body.get("args") as Blob).text()).toBe('"beach"');
  const files = body.getAll("files") as File[];
  expect(files.map((file) => file.name)).toEqual(["a.png", "b.txt"]);
  expect(await files[0]!.text()).toBe("one");
  expect(runtime.store.get("uploading")).toBe(false);
});

it("looks up the file input apart from a node ref of the same name", async () => {
  const fetch = vi.fn<Fetch>(() =>
    Promise.resolve(
      new Response("event: hs-end\ndata: {}\n\n", {
        status: 200,
        headers: { "content-type": "text/event-stream" },
      }),
    ),
  );
  vi.stubGlobal("fetch", fetch);
  const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
  mount("upload");
  pick([new File(["one"], "a.png", { type: "image/png" })]);
  $("#send").click();
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
  expect(sent(fetch, 0)).toEqual(["a.png"]);
  $("#focus").click();
  await vi.waitFor(() => expect(document.activeElement).toBe($("#note")));
  expect(warn).not.toHaveBeenCalled();
});

it("sends no file part when nothing is picked, and empties the input after sending", async () => {
  const fetch = vi.fn<Fetch>(() =>
    Promise.resolve(
      new Response("event: hs-end\ndata: {}\n\n", {
        status: 200,
        headers: { "content-type": "text/event-stream" },
      }),
    ),
  );
  vi.stubGlobal("fetch", fetch);
  mount("upload");
  $<HTMLInputElement>("#pictures").value = "";
  $("#send").click();
  await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
  expect(sent(fetch, 0)).toEqual([]);
  await tick();
  expect($<HTMLInputElement>("#pictures").value).toBe("");
});

it("writes a refused upload to the error signal", async () => {
  vi.stubGlobal(
    "fetch",
    vi.fn<Fetch>(() =>
      Promise.resolve(new Response("the upload is longer than its limit", { status: 413 })),
    ),
  );
  vi.spyOn(console, "error").mockImplementation(() => {});
  const runtime = mount("upload");
  $("#send").click();
  await vi.waitFor(() =>
    expect(runtime.store.get("error")).toBe("the upload is longer than its limit"),
  );
  expect(runtime.store.get("uploading")).toBe(false);
});
