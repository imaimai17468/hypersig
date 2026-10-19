import { afterEach, expect, it, vi } from "vitest";
import { $, key, tick } from "../dom.ts";
import { mount } from "./fixture.ts";

afterEach(() => {
  vi.restoreAllMocks();
});

/** Makes `menu` match `:popover-open` while `open` returns true, which the test DOM does not track. */
function popover(menu: HTMLElement, open: () => boolean): void {
  const matches = menu.matches.bind(menu);
  vi.spyOn(menu, "matches").mockImplementation((selector: string) =>
    selector === ":popover-open" ? open() : matches(selector),
  );
}

it("hides an open popover and leaves a closed one alone", async () => {
  mount("roving");
  const menu = $("#menu");
  const hide = vi.fn<() => void>();
  menu.hidePopover = hide;
  let open = false;
  popover(menu, () => open);
  key($("#item0"), "Tab");
  await tick();
  expect(hide).not.toHaveBeenCalled();
  open = true;
  key($("#item0"), "Tab");
  await tick();
  expect(hide).toHaveBeenCalledOnce();
  $("#item0").dispatchEvent(
    new KeyboardEvent("keydown", { key: "Tab", shiftKey: true, bubbles: true, cancelable: true }),
  );
  await tick();
  expect(hide).toHaveBeenCalledTimes(2);
});

it("shows a closed popover and leaves an open one alone", async () => {
  mount("roving");
  const menu = $("#menu");
  menu.setAttribute("popover", "auto");
  const show = vi.fn<() => void>();
  menu.showPopover = show;
  let open = true;
  popover(menu, () => open);
  key($("#item0"), "Enter");
  await tick();
  expect(show).not.toHaveBeenCalled();
  open = false;
  key($("#item0"), "Enter");
  await tick();
  expect(show).toHaveBeenCalledOnce();
});
