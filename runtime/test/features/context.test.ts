import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { $, $$, tick } from "../dom.ts";
import { mount } from "./fixture.ts";

/** Gives the viewport `width` by `height`, which the test DOM leaves at 0 by 0. */
function viewport(width: number, height: number): void {
  const view = document.documentElement;
  vi.spyOn(view, "clientWidth", "get").mockReturnValue(width);
  vi.spyOn(view, "clientHeight", "get").mockReturnValue(height);
}

beforeEach(() => {
  viewport(1000, 800);
});

afterEach(() => {
  vi.restoreAllMocks();
});

/** Asks for a context menu on `target` with the pointer at `x`, `y`, and returns whether the browser's menu was kept from showing. */
function rightClick(target: Element, x: number, y: number): boolean {
  const event = new MouseEvent("contextmenu", {
    bubbles: true,
    cancelable: true,
    button: 2,
    clientX: x,
    clientY: y,
  });
  target.dispatchEvent(event);
  return event.defaultPrevented;
}

/** Gives `element` the size `width` by `height` at `left`, `top`, which the test DOM does not lay out. */
function box(element: Element, left: number, top: number, width: number, height: number): void {
  vi.spyOn(element, "getBoundingClientRect").mockReturnValue(new DOMRect(left, top, width, height));
}

/** Makes `menu` a popover that opens on `showPopover` and reports it through `:popover-open`. */
function popover(menu: HTMLElement): () => number {
  let open = false;
  let shown = 0;
  menu.showPopover = () => {
    open = true;
    shown += 1;
  };
  const matches = menu.matches.bind(menu);
  vi.spyOn(menu, "matches").mockImplementation((selector: string) =>
    selector === ":popover-open" ? open : matches(selector),
  );
  return () => shown;
}

it("runs a list item's handler with the pointer's position and opens the menu there", async () => {
  const runtime = mount("board");
  await tick();
  const menu = $("#row-menu");
  const shown = popover(menu);
  box(menu, 0, 0, 100, 50);
  const rows = $$("#rows li");
  expect(rightClick(rows[1]!, 30.5, 40)).toBe(true);
  expect(runtime.store.get("picked")).toBe("b");
  expect(runtime.store.get("at")).toEqual({ x: 30.5, y: 40 });
  await tick();
  expect(shown()).toBe(1);
  expect([menu.style.left, menu.style.top]).toEqual(["30.5px", "40px"]);
  expect([menu.style.right, menu.style.bottom, menu.style.margin]).toEqual(["auto", "auto", "0px"]);
});

it("moves an open menu to the new position without opening it again", async () => {
  mount("board");
  await tick();
  const menu = $("#row-menu");
  const shown = popover(menu);
  box(menu, 0, 0, 100, 50);
  const rows = $$("#rows li");
  rightClick(rows[0]!, 10, 10);
  await tick();
  rightClick(rows[1]!, 60, 70);
  await tick();
  expect(shown()).toBe(1);
  expect([menu.style.left, menu.style.top]).toEqual(["60px", "70px"]);
});

it("puts a menu that would cross the right or bottom edge left of the pointer or above it", async () => {
  mount("board");
  await tick();
  const menu = $("#row-menu");
  popover(menu);
  box(menu, 0, 0, 100, 50);
  viewport(400, 300);
  rightClick($$("#rows li")[0]!, 350, 280);
  await tick();
  expect([menu.style.left, menu.style.top]).toEqual(["250px", "230px"]);
  rightClick($$("#rows li")[0]!, 40, 30);
  await tick();
  expect([menu.style.left, menu.style.top]).toEqual(["40px", "30px"]);
});

it("runs only the innermost handler", () => {
  const runtime = mount("board");
  rightClick($("#inner"), 5, 6);
  expect(runtime.store.get("at")).toEqual({ x: 5, y: 6 });
  expect(runtime.store.get("outer")).toBe(0);
  rightClick($("#outer"), 1, 1);
  expect(runtime.store.get("outer")).toBe(1);
});

it("leaves the browser's menu where no element has a handler", () => {
  mount("board");
  expect(rightClick($("#plain"), 1, 1)).toBe(false);
});

it("gives a menu asked for with a key the focused element's bottom left corner", () => {
  const runtime = mount("board");
  const inner = $("#inner");
  box(inner, 100, 200, 80, 30);
  inner.dispatchEvent(
    new MouseEvent("contextmenu", {
      bubbles: true,
      cancelable: true,
      button: -1,
      clientX: 140,
      clientY: 215,
    }),
  );
  expect(runtime.store.get("at")).toEqual({ x: 100, y: 230 });
});

it("waits for the button to come up when the event arrives while it is down", () => {
  const runtime = mount("board");
  $("#inner").dispatchEvent(
    new MouseEvent("contextmenu", {
      bubbles: true,
      cancelable: true,
      button: 2,
      buttons: 2,
      clientX: 7,
      clientY: 8,
    }),
  );
  expect(runtime.store.get("at")).toEqual({ x: 0, y: 0 });
  window.dispatchEvent(new Event("pointerup"));
  expect(runtime.store.get("at")).toEqual({ x: 7, y: 8 });
  window.dispatchEvent(new Event("pointerup"));
  expect(runtime.store.get("at")).toEqual({ x: 7, y: 8 });
});
