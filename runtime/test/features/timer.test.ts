import { afterEach, expect, it, vi } from "vitest";
import { $, type } from "../dom.ts";
import { mount } from "./fixture.ts";

afterEach(() => {
  vi.useRealTimers();
});

it("runs an interval while the element is in the document", async () => {
  vi.useFakeTimers();
  const runtime = mount("timers");
  await vi.advanceTimersByTimeAsync(350);
  expect(runtime.store.get("ticks")).toBe(3);
  $("#clock").remove();
  await vi.advanceTimersByTimeAsync(300);
  expect(runtime.store.get("ticks")).toBe(3);
});

it("runs a timeout once, and not for an element that left the document", async () => {
  vi.useFakeTimers();
  const runtime = mount("timers");
  $("#gone").remove();
  await vi.advanceTimersByTimeAsync(350);
  expect(runtime.store.get("fired")).toBe(1);
});

it("debounces input, runs an on-signal-change handler per change, and reruns an effect from what it reads", async () => {
  vi.useFakeTimers();
  const runtime = mount("timers");
  expect(runtime.store.get("effects")).toBe(0);
  type($<HTMLInputElement>("#query"), "t");
  await vi.advanceTimersByTimeAsync(20);
  type($<HTMLInputElement>("#query"), "te");
  await vi.advanceTimersByTimeAsync(20);
  expect(runtime.store.get("searched")).toBe("");
  expect(runtime.store.get("echo")).toBe("TE");
  await vi.advanceTimersByTimeAsync(60);
  expect(runtime.store.get("searched")).toBe("te");
  expect(runtime.store.get("effects")).toBe(2);
  runtime.store.set("ticks", 99);
  await vi.advanceTimersByTimeAsync(0);
  expect(runtime.store.get("effects")).toBe(2);
});
