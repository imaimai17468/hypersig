import { afterEach, expect, it, vi } from "vitest";
import { $, tick } from "../dom.ts";
import { mount } from "./mount.ts";

afterEach(() => {
  vi.restoreAllMocks();
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

it("applies events again once a navigation that kept the page, such as a download, has had 3 seconds", () => {
  vi.useFakeTimers();
  try {
    vi.spyOn(location, "assign").mockImplementation(() => {});
    const runtime = mount(`<p></p>`, { n: 1 });
    runtime.apply({ event: "hs-navigate", data: JSON.stringify({ url: "/report.csv" }) });
    vi.advanceTimersByTime(2999);
    runtime.apply({ event: "hs-signals", data: JSON.stringify({ n: 2 }) });
    expect(runtime.store.get("n")).toBe(1);
    vi.advanceTimersByTime(1);
    runtime.apply({ event: "hs-signals", data: JSON.stringify({ n: 3 }) });
    expect(runtime.store.get("n")).toBe(3);
  } finally {
    vi.useRealTimers();
  }
});

it("keeps dropping events after the page hides, until the back-forward cache restores it", () => {
  vi.useFakeTimers();
  try {
    vi.spyOn(location, "assign").mockImplementation(() => {});
    const runtime = mount(`<p></p>`, { n: 1 });
    runtime.apply({ event: "hs-navigate", data: JSON.stringify({ url: "/elsewhere" }) });
    dispatchEvent(new Event("pagehide"));
    vi.advanceTimersByTime(10_000);
    runtime.apply({ event: "hs-signals", data: JSON.stringify({ n: 2 }) });
    expect(runtime.store.get("n")).toBe(1);
    const shown = Object.assign(new Event("pageshow"), { persisted: true });
    dispatchEvent(shown);
    runtime.apply({ event: "hs-signals", data: JSON.stringify({ n: 3 }) });
    expect(runtime.store.get("n")).toBe(3);
  } finally {
    vi.useRealTimers();
  }
});

it("pushes and replaces the URL without loading it, and loads the page Back moves to", async () => {
  history.replaceState(null, "", "/fruit");
  const reload = vi.spyOn(location, "reload").mockImplementation(() => {});
  const runtime = mount(
    `<button data-hs-on-click='["history","push",["lit","/fruit?q=fig"]]'>go</button>`,
    { n: 1 },
  );
  const length = history.length;
  $("button").click();
  await tick();
  expect(location.pathname + location.search).toBe("/fruit?q=fig");
  expect(history.length).toBe(length + 1);

  $("button").click();
  await tick();
  expect(history.length).toBe(length + 1);

  runtime.apply({
    event: "hs-history",
    data: JSON.stringify({ url: "/fruit?q=kiwi", replace: true }),
  });
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
