import { expect, it } from "vitest";
import { $ } from "../dom.ts";
import { serve, start } from "./fixture.ts";

const primary = /Mac|iPhone|iPad|iPod/u.test(navigator.platform)
  ? { metaKey: true }
  : { ctrlKey: true };

function press(target: Element, name: string, init: KeyboardEventInit = {}): KeyboardEvent {
  const event = new KeyboardEvent("keydown", {
    key: name,
    bubbles: true,
    cancelable: true,
    ...init,
  });
  target.dispatchEvent(event);
  return event;
}

it("runs a page shortcut while nothing has focus, and matches its modifiers exactly", () => {
  serve("shortcuts_page");
  const runtime = start();
  expect(press(document.body, "k", primary).defaultPrevented).toBe(true);
  expect(runtime.store.get("palette")).toBe(1);
  press(document.body, "k");
  press(document.body, "k", { ...primary, altKey: true });
  expect(runtime.store.get("palette")).toBe(1);
  press(document.body, "/");
  expect(runtime.store.get("palette")).toBe(11);
});

it("leaves the keys a text field types with to the field, unless the shortcut holds a modifier", () => {
  serve("shortcuts_page");
  const runtime = start();
  const field = $("#field");
  expect(press(field, "s").defaultPrevented).toBe(false);
  expect(press(field, "/").defaultPrevented).toBe(false);
  expect(runtime.store.get("saves")).toBe(0);
  expect(runtime.store.get("palette")).toBe(0);
  press($("#section"), "s");
  expect(runtime.store.get("saves")).toBe(1);

  press(field, "S", { ctrlKey: true, shiftKey: true });
  expect(runtime.store.get("saves")).toBe(101);
  press(field, "s", { ctrlKey: true });
  expect(runtime.store.get("saves")).toBe(101);
  // The section has handlers, none for this key, so the page's runs.
  press(field, "k", primary);
  expect(runtime.store.get("palette")).toBe(1);
});

it("gives a field its own handlers, and the nearest handler that matches stops the search", () => {
  serve("shortcuts_page");
  const runtime = start();
  press($("#notes"), "Escape");
  expect(runtime.store.get("escapes")).toBe(1);
  press($("#own"), "s");
  expect(runtime.store.get("saves")).toBe(1000);
  expect(press($("#own"), "Enter").defaultPrevented).toBe(false);
  expect(runtime.store.get("newlines")).toBe(0);
  press($("#own"), "Enter", { shiftKey: true });
  expect(runtime.store.get("newlines")).toBe(1);
});
