import type { Slot } from "../eval.ts";
import type { Effect } from "../deps.ts";
import type { Row } from "./list/row.ts";
import { data } from "./spec.ts";

const STATE = Symbol("hypersig");

/**
 * What the runtime keeps about one element. It lives on the element, so it
 * goes when the element does, without a table that keeps its size after the
 * elements it held are gone.
 */
export interface ElementState {
  /**
   * The template element whose `data-hs-*` attributes this element, part of
   * a list item or a condition, carries its bindings from. The server writes
   * those copies without the attributes, and a patch that morphs a clone
   * removes them.
   */
  source?: Element;
  scope?: readonly Slot[];
  effects?: Effect[];
  /** Whether its on-load, on-interval, and on-visible statements have started. */
  started?: boolean;
  /** Whether it is the root of a list item or a condition. */
  root?: boolean;
  /** For such a root, the elements with bindings inside it, outside its nested lists and conditions. */
  members?: readonly Element[];
  /** For the root of a list item, the binding that watches the item's key. */
  watch?: Effect;
  /** For the root of a list item, its parts. */
  row?: Row;
}

type Stateful = Element & { [STATE]?: ElementState };

export function own(element: Element): ElementState | undefined {
  return (element as Stateful)[STATE];
}

export function state(element: Element): ElementState {
  return ((element as Stateful)[STATE] ??= {});
}

/** Whether `element` sits in a list item or a conditional copy the runtime drew. */
export function inCopy(element: Element): boolean {
  for (let at: Element | null = element; at !== null; at = at.parentElement) {
    if (own(at)?.root === true) return true;
  }
  return false;
}

/** The `data-hs-*` attribute of `element` whose `dataset` name is `key`, read from its template element where it has one. */
export function attr(element: Element, key: string): string | undefined {
  return data(own(element)?.source ?? element)[key];
}

/** The slots of the list items around `element`, innermost last, as the closest registered element holds them. */
export function scopeOf(element: Element | null): readonly Slot[] {
  for (let at = element; at !== null; at = at.parentElement) {
    const scope = own(at)?.scope;
    if (scope !== undefined) return scope;
  }
  return [];
}
