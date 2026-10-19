import { expect, it } from "vitest";
import { $, $$ } from "../dom.ts";
import { mount } from "./fixture.ts";

function move(kind: string, target: Element, related: Element | null): void {
  target.dispatchEvent(new FocusEvent(kind, { bubbles: true, relatedTarget: related }));
}

it("runs mouseenter and mouseleave on the element itself, and focus on focusin", () => {
  const runtime = mount("nested");
  const row = $$("#rows > li")[1]!;
  row.dispatchEvent(new MouseEvent("mouseenter"));
  expect(runtime.store.get("hovered")).toBe("b");
  row.querySelector("button")!.dispatchEvent(new MouseEvent("mouseenter"));
  expect(runtime.store.get("hovered")).toBe("b");
  row.dispatchEvent(new MouseEvent("mouseleave"));
  expect(runtime.store.get("hovered")).toBe("");

  $("#field input").dispatchEvent(new FocusEvent("focusin", { bubbles: true }));
  expect(runtime.store.get("focused")).toBe(true);
  $("#field input").dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
  expect(runtime.store.get("focused")).toBe(false);
});

it("runs focus and blur only when focus enters or leaves the element", () => {
  const runtime = mount("nested");
  const [first, second] = $$("#field input");
  move("focusin", first!, document.body);
  expect(runtime.store.get("focused")).toBe(true);

  // From the first input to the second, as an arrow key moves between radios.
  runtime.store.set("focused", "unchanged");
  move("focusout", first!, second!);
  move("focusin", second!, first!);
  expect(runtime.store.get("focused")).toBe("unchanged");

  move("focusout", second!, document.body);
  expect(runtime.store.get("focused")).toBe(false);
  move("focusin", second!, null);
  expect(runtime.store.get("focused")).toBe(true);
});
