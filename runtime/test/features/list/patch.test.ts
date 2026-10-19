import { afterEach, expect, it, vi } from "vitest";
import { $, $$, tick } from "../../dom.ts";
import { events, mount } from "../fixture.ts";

afterEach(() => {
  vi.restoreAllMocks();
});

const bodies = () => $$("#feed li").map((li) => li.textContent);

/** The element whose `scrollTop` scrolls the page. */
function scroller(): Element {
  return document.scrollingElement ?? document.documentElement;
}

it("appends, prepends, upserts, and removes items by key, keeping the elements of the items that stay", async () => {
  const runtime = mount("feed");
  await tick();
  const [one, two] = $$("#feed li");
  const [append, prepend, upsert, remove] = events("feed_patches");
  runtime.apply(append!);
  await tick();
  expect(bodies()).toEqual(["one", "two", "three"]);
  expect($$("#feed li").slice(0, 2)).toEqual([one, two]);
  runtime.apply(prepend!);
  runtime.apply(upsert!);
  await tick();
  expect(bodies()).toEqual(["zero", "one!", "two", "three", "four"]);
  expect($$("#feed li")[1]).toBe(one);
  runtime.apply(remove!);
  await tick();
  expect(bodies()).toEqual(["zero", "one!", "four"]);
  expect((runtime.store.get("notes") as { id: number }[]).map((note) => note.id)).toEqual([
    0, 1, 4,
  ]);
});

it("prepends items in the browser and scrolls the page back to the items that were in view", async () => {
  mount("feed");
  await tick();
  const [one] = $$("#feed li");
  // The test DOM lays nothing out, so the first item reports the 40 pixels the two new items above it would push it down.
  vi.spyOn(one!, "getBoundingClientRect")
    .mockReturnValueOnce(new DOMRect(0, 0, 100, 20))
    .mockReturnValue(new DOMRect(0, 40, 100, 20));
  const scrolled = vi.spyOn(scroller(), "scrollTop", "set");
  $("#older").click();
  await tick();
  expect(bodies()).toEqual(["minus one", "zero", "one", "two"]);
  expect($$("#feed li")[2]).toBe(one);
  expect(scrolled).toHaveBeenCalledWith(40);
});
