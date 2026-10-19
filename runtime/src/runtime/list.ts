import type { Slot } from "../eval.ts";
import type { Effect } from "../deps.ts";
import type { Runtime } from "../runtime.ts";
import { Adoption } from "./list/adopt.ts";
import type { Instance } from "./list/item.ts";
import { ListRender } from "./list/render.ts";
import { Rows } from "./list/row.ts";
import { Slots } from "./list/slot.ts";
import { ListWatches } from "./list/watch.ts";

/** The rendered items of each list, by its `<template>`, keyed, and what keeps them up to date. */
export class Lists {
  readonly rows: Rows;
  readonly slots: Slots;
  readonly watches: ListWatches;
  readonly adoption: Adoption;
  readonly #render: ListRender;
  readonly #lists = new WeakMap<Element, Map<string, Instance>>();

  constructor(rt: Runtime) {
    this.#render = new ListRender(rt);
    this.rows = new Rows(rt);
    this.slots = new Slots(rt);
    this.watches = new ListWatches(rt);
    this.adoption = new Adoption(rt);
  }

  /** The items the list at `template` shows, in order. */
  items(template: Element): Iterable<Instance> {
    return this.#lists.get(template)?.values() ?? [];
  }

  /** The elements of the items drawn after `template`, where it is a rendered list. */
  drawn(template: HTMLTemplateElement): ReadonlySet<Element> | undefined {
    const items = this.#lists.get(template);
    return items === undefined
      ? undefined
      : new Set(Array.from(items.values(), (instance) => instance.element));
  }

  /** The items of the list at `template`, by key, once it has rendered. */
  get(template: Element): Map<string, Instance> | undefined {
    return this.#lists.get(template);
  }

  set(template: Element, items: Map<string, Instance>): void {
    this.#lists.set(template, items);
  }

  /**
   * Renders the list at `template`, over the list at `place` with items at
   * depth `depth` inside the items `scope`, as the binding `list`.
   */
  render(
    template: HTMLTemplateElement,
    proto: Element | null,
    place: unknown,
    depth: number,
    scope: readonly Slot[],
    list: Effect,
  ): void {
    this.#render.render(template, proto, place, depth, scope, list);
  }
}
