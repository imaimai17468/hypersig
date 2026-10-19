import { type Context, type Slot } from "../../eval.ts";
import { resolve, resolveRead } from "../../eval/scope.ts";
import type { Effect } from "../../deps.ts";
import type { Runtime } from "../../runtime.ts";
import { appended, replaced } from "./change.ts";
import { adoptItems, type Instance, type Origin } from "./item.ts";
import { Matcher } from "./match.ts";
import { arrange, filling, prepended } from "./order.ts";
import type { Row } from "./row.ts";
import { shapeOf } from "./shape.ts";

/** What one render of a list works on. */
interface Pass {
  readonly origin: Origin;
  readonly parent: ParentNode & Node;
  readonly path: string;
  readonly items: readonly unknown[];
  readonly matcher: Matcher;
  /** The slots of the items around the list. */
  readonly outer: readonly Slot[];
}

/**
 * Renders a list: it matches the list's values to the items it drew by key,
 * and moves, builds, and removes items until the document shows them in order.
 */
export class ListRender {
  readonly #rt: Runtime;

  constructor(rt: Runtime) {
    this.#rt = rt;
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
    const parent = template.parentNode;
    if (proto === null || parent === null) return;
    const ctx: Context = { store: this.#rt.store, scope };
    const path = resolve(place, ctx);
    const read = resolveRead(place, ctx);
    const value = this.#rt.store.getShallowAs(path, read);
    const items = Array.isArray(value) ? value : [];
    const shape = shapeOf(proto, depth);
    const outer = scope.slice(0, depth);
    this.#rt.lists.watches.watch(template, shape, outer, list);
    const origin: Origin = { template, proto, shape, list };
    const matcher = new Matcher(this.#rt, origin, items, { path, read }, outer);
    const pass: Pass = { origin, parent, path, items, matcher, outer };
    const known = this.#rt.lists.get(template);
    const { changes } = this.#rt.render;
    let next: Map<string, Instance>;
    if (known !== undefined && appended(changes, path, known, items.length))
      next = this.#append(pass, known);
    else {
      const touched = known === undefined ? null : replaced(changes, path, known, items.length);
      next =
        known !== undefined && touched !== null
          ? this.#replace(pass, known, touched)
          : this.#reorder(pass, known);
    }
    this.#rt.lists.set(template, next);
  }

  /** Renders the items appended after the items of `known`. */
  #append(pass: Pass, known: Map<string, Instance>): Map<string, Instance> {
    let anchor: Node = pass.origin.template;
    for (const instance of known.values()) anchor = instance.element;
    const added: Instance[] = [];
    for (let index = known.size; index < pass.items.length; index++)
      added.push(pass.matcher.match(index, known, known, true));
    this.#register(pass, added);
    const fragment = document.createDocumentFragment();
    for (const instance of added) fragment.append(instance.element);
    pass.parent.insertBefore(fragment, anchor.nextSibling);
    return known;
  }

  /** Renders the items at the `touched` positions of the items of `known`, keeping the others where they are. */
  #replace(
    pass: Pass,
    known: ReadonlyMap<string, Instance>,
    touched: ReadonlySet<number>,
  ): Map<string, Instance> {
    const was = [...known.values()];
    const wasKeys = [...known.keys()];
    const keys = [...wasKeys];
    const order = [...was];
    const end = was.at(-1)?.element.nextSibling ?? null;
    const filed = new Map<string, Instance>();
    for (let index = 0; index < was.length; index++)
      if (!touched.has(index)) filed.set(keys[index]!, was[index]!);
    const at = [...touched].toSorted((a, b) => a - b);
    for (const index of at) {
      order[index] = pass.matcher.match(index, known, filed);
      keys[index] = pass.matcher.matched;
    }
    const next = new Map<string, Instance>();
    order.forEach((instance, index) => next.set(keys[index]!, instance));
    for (const index of at)
      if (next.get(wasKeys[index]!) !== was[index]) this.#unmount(was[index]!);
    this.#register(
      pass,
      at.map((index) => order[index]!),
    );
    for (const index of at.toReversed()) {
      const { element } = order[index]!;
      const before = index + 1 < order.length ? order[index + 1]!.element : end;
      if (element.parentNode !== pass.parent || element.nextSibling !== before)
        pass.parent.insertBefore(element, before);
    }
    return next;
  }

  /** Renders every item, matching each to an item of `known`, or of the server's, by key. */
  #reorder(pass: Pass, known: ReadonlyMap<string, Instance> | undefined): Map<string, Instance> {
    const { template, shape } = pass.origin;
    const previous = known ?? adoptItems(this.#rt, template, shape);
    const next = new Map<string, Instance>();
    const order = pass.items.map((_, index) => pass.matcher.match(index, previous, next));
    const restore = known === undefined ? null : prepended(known, order, pass.matcher.created);
    const gone = [...previous]
      .filter(([key, instance]) => next.get(key) !== instance)
      .map(([, instance]) => instance);
    if (gone.length === previous.size && gone.length > 1 && filling(pass.parent, template, gone))
      this.#unmountAll(pass.parent, template, gone);
    else for (const instance of gone) this.#unmount(instance);
    this.#register(pass, order);
    arrange(pass.parent, template, previous, order);
    // The new items' bindings outside their parts run later in this render, so their height is known after it.
    if (restore !== null) queueMicrotask(restore);
    return next;
  }

  /** Registers the bindings of the items in `order`, writing the parts of the created ones while they are still out of the document. */
  #register(pass: Pass, order: readonly Instance[]): void {
    const { bindings } = this.#rt;
    const { adoption, rows } = this.#rt.lists;
    const { proto } = pass.origin;
    const filled: Row[] = [];
    for (const instance of order) {
      if (pass.matcher.created.has(instance)) {
        bindings.attributes.mount(
          instance.element,
          proto,
          [...pass.outer, instance.slot],
          false,
          instance,
        );
        filled.push(instance);
      } else if (!instance.scanned && !adoption.has(instance.element))
        adoption.wait(instance, proto, [...pass.outer, instance.slot]);
    }
    rows.fillIn(filled);
  }

  /** Removes a list item, and its slot. */
  #unmount(instance: Instance): void {
    this.#rt.lists.slots.unfile(instance.slot);
    this.#rt.render.sweep.unmount(instance.element);
  }

  /** Removes every item of the list at `template`, which `filling` found to be all of `parent` but `template`, in one call to the DOM. */
  #unmountAll(
    parent: ParentNode,
    template: HTMLTemplateElement,
    instances: readonly Instance[],
  ): void {
    parent.textContent = "";
    parent.append(template);
    for (const instance of instances) {
      this.#rt.lists.slots.unfile(instance.slot);
      this.#rt.render.sweep.removed(instance.element);
    }
  }
}
