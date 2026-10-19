import type { Events } from "../event.ts";

/** The keys that move focus to the next and the previous item of a group laid out in `orientation`. */
function directions(group: Element, orientation: string): [next: string, previous: string] {
  if (orientation === "vertical") return ["ArrowDown", "ArrowUp"];
  // Left and right trade places in a right-to-left layout, as they do for the reader.
  return orientation === "horizontal" && getComputedStyle(group).direction === "rtl"
    ? ["ArrowLeft", "ArrowRight"]
    : ["ArrowRight", "ArrowLeft"];
}

/** Moves focus among the items of the closest `data-hs-roving` element for an arrow key, Home, or End. */
export function rove(events: Events, target: Element, event: KeyboardEvent): void {
  if (event.altKey || event.ctrlKey || event.metaKey) return;
  const found = events.closest(target, "hsRoving");
  if (found === null) return;
  const [group, orientation] = found;
  const items = Array.from(group.querySelectorAll<HTMLElement>("[tabindex]")).filter(
    (item) =>
      item.closest("[data-hs-roving]") === group &&
      !item.matches(":disabled") &&
      item.getAttribute("aria-disabled") !== "true",
  );
  const at = items.findIndex((item) => item.contains(target));
  if (at < 0) return;
  const [next, previous] = directions(group, orientation);
  let to: number;
  if (event.key === next) to = (at + 1) % items.length;
  else if (event.key === previous) to = (at - 1 + items.length) % items.length;
  else if (event.key === "Home") to = 0;
  else if (event.key === "End") to = items.length - 1;
  else return;
  event.preventDefault();
  items[to]!.focus();
}
