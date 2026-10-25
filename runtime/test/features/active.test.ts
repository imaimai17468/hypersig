import { expect, it } from "vitest";
import { $, key, tick, type } from "../dom.ts";
import { mount } from "./fixture.ts";

it("moves the signal among the shown options that are not disabled, and clicks the active one on Enter", async () => {
  const runtime = mount("combo");
  const field = $("#field");
  key(field, "ArrowDown");
  await tick();
  expect(runtime.store.get("active")).toBe("o0");
  // `active_descendant` binds the attribute a screen reader follows to the same signal.
  expect(field.getAttribute("aria-activedescendant")).toBe("o0");
  key(field, "ArrowDown");
  await tick();
  expect(runtime.store.get("active")).toBe("o3");
  expect(field.getAttribute("aria-activedescendant")).toBe("o3");
  key(field, "ArrowDown");
  await tick();
  expect(runtime.store.get("active")).toBe("o0");
  key(field, "ArrowUp");
  await tick();
  expect(runtime.store.get("active")).toBe("o3");
  expect(runtime.store.get("downs")).toBe(3);
  key(field, "Enter");
  await tick();
  expect(runtime.store.get("picked")).toBe("o3");
});

it("starts from the last option on the up arrow and counts an option once it is shown", async () => {
  const runtime = mount("combo");
  const field = $<HTMLInputElement>("#field");
  type(field, "a");
  await tick();
  key(field, "ArrowUp");
  await tick();
  expect(runtime.store.get("active")).toBe("o3");
  key(field, "ArrowUp");
  await tick();
  expect(runtime.store.get("active")).toBe("o1");
});

it("leaves Enter alone while no option is active", async () => {
  const runtime = mount("combo");
  const event = new KeyboardEvent("keydown", { key: "Enter", bubbles: true, cancelable: true });
  $("#field").dispatchEvent(event);
  await tick();
  expect(event.defaultPrevented).toBe(false);
  expect(runtime.store.get("picked")).toBe("");
});
