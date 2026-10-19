import { expect, it } from "vitest";
import { $, $$, tick } from "../dom.ts";
import { mount } from "./fixture.ts";

function dispatch(target: Element, type: string, detail?: unknown, bubbles = false): void {
  target.dispatchEvent(new CustomEvent(type, { detail, bubbles }));
}

it("runs the statements of a custom event in order, with its detail", () => {
  const runtime = mount("widgets");
  dispatch($("#rating"), "rate", 3);
  expect(runtime.store.get("stars")).toBe(6);
});

it("reads the fields of a detail", () => {
  const runtime = mount("widgets");
  dispatch($("#map"), "map-moved", { lat: 35.6, lng: 139.7 });
  expect(runtime.store.get("center")).toEqual({ lat: 35.6, lng: 139.7 });
});

it("runs on an event that bubbles from inside the element, and not on one dispatched beside it", () => {
  const runtime = mount("widgets");
  const inside = document.createElement("span");
  $("#rating").append(inside);
  dispatch(inside, "rate", 1, true);
  expect(runtime.store.get("stars")).toBe(2);
  dispatch($("#map"), "rate", 5, true);
  expect(runtime.store.get("stars")).toBe(2);
});

it("runs the statement of a list item against that item", async () => {
  const runtime = mount("widgets");
  await tick();
  const pins = $$("#pins x-pin");
  expect(pins.map((pin) => pin.textContent)).toEqual(["a", "b"]);
  dispatch(pins[1]!, "pick");
  expect(runtime.store.get("pins")).toEqual([
    { name: "a", picked: false },
    { name: "b", picked: true },
  ]);
});
