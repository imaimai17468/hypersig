import { keepInView } from "../../anchor.ts";
import type { Instance } from "./item.ts";

/**
 * Puts the elements of `order` right after `template` in that order, moving
 * as few as it can. The items of `previous` still in the document keep their
 * place where their old positions increase along `order`, the longest such
 * run, and the rest are inserted around them.
 */
export function arrange(
  parent: Node,
  template: Element,
  previous: ReadonlyMap<string, Instance>,
  order: readonly Instance[],
): void {
  const placed: Instance[] = [];
  for (const instance of previous.values())
    if (instance.element.parentNode === parent) placed.push(instance);
  if (placed.length === 0) {
    // Nothing stays, so the items go in together, as one insertion into the document.
    const fragment = document.createDocumentFragment();
    for (const instance of order) fragment.append(instance.element);
    parent.insertBefore(fragment, template.nextSibling);
    return;
  }
  const [start, end] = kept(placed, order);
  const positions = new Map<Instance, number>();
  for (let at = start; at < placed.length - end; at++) positions.set(placed[at]!, at);
  const stay = increasing(
    order.slice(start, order.length - end).map((instance) => positions.get(instance) ?? -1),
  );
  const last = placed.at(-1)!.element;
  let before: Node | null = end > 0 ? order[order.length - end]!.element : last.nextSibling;
  for (let index = order.length - 1 - end; index >= start; index--) {
    const element = order[index]!.element;
    if (!stay.has(index - start)) move(parent, element, before);
    before = element;
  }
}

/** How many items keep their place at the start and at the end of `order`, which stay out of the run `arrange` looks for. */
function kept(
  placed: readonly Instance[],
  order: readonly Instance[],
): [start: number, end: number] {
  let start = 0;
  while (start < order.length && start < placed.length && order[start] === placed[start]) start++;
  let end = 0;
  while (
    end < order.length - start &&
    end < placed.length - start &&
    order[order.length - 1 - end] === placed[placed.length - 1 - end]
  )
    end++;
  return [start, end];
}

/** The indices of a longest strictly increasing run of the values of `values` that are not -1. */
export function increasing(values: readonly number[]): Set<number> {
  /** `tails[k]` is the index ending the run of length k + 1 whose last value is smallest. */
  const tails: number[] = [];
  const previous = values.map(() => -1);
  values.forEach((value, index) => {
    if (value === -1) return;
    let low = 0;
    let high = tails.length;
    while (low < high) {
      const mid = (low + high) >> 1;
      if (values[tails[mid]!]! < value) low = mid + 1;
      else high = mid;
    }
    if (low > 0) previous[index] = tails[low - 1]!;
    tails[low] = index;
  });
  const run = new Set<number>();
  for (let at = tails.at(-1) ?? -1; at !== -1; at = previous[at]!) run.add(at);
  return run;
}

/** Whether `parent` holds `template` followed by the items' elements in order, and nothing else. */
export function filling(parent: Node, template: Element, instances: readonly Instance[]): boolean {
  if (parent.firstChild !== template || parent.lastChild !== instances.at(-1)!.element)
    return false;
  let at: Node | null = template;
  for (const instance of instances) {
    at = at.nextSibling;
    if (at !== instance.element) return false;
  }
  return true;
}

/**
 * Where items were only added before the items of `known`, which stay in
 * their order after them, the function that scrolls them back to where they
 * were on screen once the new items are in the document. Null otherwise.
 */
export function prepended(
  known: ReadonlyMap<string, Instance>,
  order: readonly Instance[],
  created: ReadonlySet<Instance>,
): (() => void) | null {
  const first = known.values().next().value;
  if (first === undefined || !first.element.isConnected) return null;
  const at = order.indexOf(first);
  if (at <= 0 || at + known.size > order.length) return null;
  for (let index = 0; index < at; index++) if (!created.has(order[index]!)) return null;
  let index = at;
  for (const instance of known.values()) if (order[index++] !== instance) return null;
  return keepInView(first.element);
}

/**
 * Puts `element` before `before` in `parent`, as `insertBefore` does.
 *
 * `insertBefore` takes an element in the document out before it puts it back,
 * which blurs a focused element inside it and drops the selection of a text
 * field. `moveBefore` keeps both, and where the browser lacks it the focused
 * element is focused again with its selection afterwards.
 */
export function move(parent: Node, element: Element, before: Node | null): void {
  if (
    parent instanceof Element &&
    "moveBefore" in parent &&
    parent.isConnected &&
    element.isConnected
  ) {
    parent.moveBefore(element, before);
    return;
  }
  const focused = document.activeElement;
  if (!(focused instanceof HTMLElement) || !element.contains(focused)) {
    parent.insertBefore(element, before);
    return;
  }
  const field =
    focused instanceof HTMLInputElement || focused instanceof HTMLTextAreaElement ? focused : null;
  // An input whose type has no text selection, such as a checkbox, reads `null`.
  const start = field?.selectionStart ?? null;
  const end = field?.selectionEnd ?? null;
  const direction = field?.selectionDirection ?? undefined;
  parent.insertBefore(element, before);
  if (document.activeElement === focused) return;
  focused.focus({ preventScroll: true });
  if (field !== null && start !== null && end !== null)
    field.setSelectionRange(start, end, direction);
}
