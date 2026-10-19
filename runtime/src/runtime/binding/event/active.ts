import { resolve } from "../../../eval/scope.ts";
import type { Runtime } from "../../../runtime.ts";
import { attr } from "../../element.ts";
import { REF } from "../ref.ts";
import { data, parse } from "../../spec.ts";

/** Whether `element`, or an element between it and `root`, is hidden by `data-hs-show` or the `hidden` attribute. */
function hiddenWithin(element: HTMLElement, root: Element): boolean {
  for (let at: Element | null = element; at !== null && at !== root; at = at.parentElement) {
    if (
      at instanceof HTMLElement &&
      (at.hidden || at.style.display === "none" || data(at).hsHide !== undefined)
    )
      return true;
  }
  return false;
}

/** The shown options of `listbox` that can be picked, in order. */
function optionsOf(listbox: Element): HTMLElement[] {
  return Array.from(listbox.querySelectorAll<HTMLElement>('[role="option"]')).filter(
    (option) =>
      option.id !== "" &&
      option.getAttribute("aria-disabled") !== "true" &&
      !hiddenWithin(option, listbox),
  );
}

/**
 * Moves the signal of a `data-hs-active` element to the id of the next or the previous shown option of its listbox
 * for the down and up arrows, and clicks the option the signal names for Enter.
 */
export function activate(rt: Runtime, target: Element, event: KeyboardEvent): void {
  if (event.altKey || event.ctrlKey || event.metaKey) return;
  const source = attr(target, "hsActive");
  if (source === undefined) return;
  const [name, place] = parse(source) as [string, unknown];
  const listbox = rt.bindings.refs.find(REF, name, target);
  if (listbox === null) return;
  const options = optionsOf(listbox);
  const path = resolve(place, rt.bindings.statements.context(target));
  const at = options.findIndex((option) => option.id === rt.store.get(path));
  if (event.key === "Enter") {
    if (at < 0) return;
    event.preventDefault();
    options[at]!.click();
    return;
  }
  let to: number;
  if (event.key === "ArrowDown") to = at < 0 ? 0 : (at + 1) % options.length;
  else if (event.key === "ArrowUp")
    to = at < 0 ? options.length - 1 : (at - 1 + options.length) % options.length;
  else return;
  if (options.length === 0) return;
  event.preventDefault();
  rt.store.set(path, options[to]!.id);
  options[to]!.scrollIntoView({ block: "nearest" });
}
