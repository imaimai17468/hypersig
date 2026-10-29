import { expect, it } from "vitest";
import { $, tick } from "../dom.ts";
import { events, serve, start } from "./fixture.ts";

const buckets = () =>
  [...document.head.querySelectorAll<HTMLStyleElement>("style[data-hs-css]")].map(
    (style) => style.dataset.hsCss,
  );

it("switches a toggled style's classes with its condition", async () => {
  serve("styled_page");
  start();
  const box = $("#box");
  expect(box.className).toBe("md:width_50%");
  box.click();
  await tick();
  expect(box.classList.contains("when:color_crimson")).toBe(true);
  box.click();
  await tick();
  expect(box.classList.contains("when:color_crimson")).toBe(false);
});

it("puts a patch's rule buckets at their place in the order", async () => {
  serve("styled_page");
  const runtime = start();
  // The fixture's stream holds one event.
  const [patch] = events("styled_patch");
  expect(buckets()).toEqual(["0 k01-----01", "1 k-----01"]);
  runtime.apply(patch!);
  await tick();
  expect(buckets()).toEqual([
    "0 k-----01",
    "0 k-----02",
    "0 k----4019-01",
    "0 k01-----01",
    "1 k-----01",
    "2 pulse",
  ]);
  expect($("#slot").className).toBe("width_100% padding_4px animation-name_pulse hover:color_blue");
  const base = document.head.querySelector<HTMLStyleElement>('style[data-hs-css="0 k-----01"]')!;
  const once = base.textContent;
  runtime.apply(patch!);
  await tick();
  expect(base.textContent).toBe(once);
  expect(buckets()).toHaveLength(6);
});
