import type { Effect } from "../../deps.ts";
import type { Runtime } from "../../runtime.ts";
import { own } from "../element.ts";

/**
 * The elements taken out of the document and the bindings found outside it,
 * whose registrations are removed after the next paint. The runtime never
 * puts a removed element back, so until then they only hold memory, and a
 * render that reaches them skips them.
 */
export class Sweep {
  readonly #rt: Runtime;
  /** Items and conditions taken out of the document, and elements a patch took out. */
  #removed: Element[] = [];
  #stale = new Set<Effect>();
  #sweeping = false;
  /** Whether a patch has replaced elements since the last sweep, which may have taken list items with them. */
  #patched = false;

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Removes `root` from the document, and the bindings inside it after the next paint. */
  unmount(root: Element): void {
    root.remove();
    this.removed(root);
  }

  /** Forgets the bindings inside `root`, which has left the document, after the next paint. */
  removed(root: Element): void {
    this.#removed.push(root);
    this.#later();
  }

  /** Forgets the bindings of `elements`, which a patch took out of the document, after the next paint. */
  patched(elements: readonly Element[]): void {
    this.#removed.push(...elements);
    this.#patched = true;
    this.#later();
  }

  /** Forgets `effect`, found outside the document, after the next paint. */
  stale(effect: Effect): void {
    this.#stale.add(effect);
    this.#later();
  }

  #later(): void {
    if (this.#sweeping) return;
    this.#sweeping = true;
    // A frame callback runs right before a paint, so the timer it sets runs after one. A hidden page paints nothing.
    const after = (): void => void setTimeout(() => this.#sweep(), 0);
    if (document.hidden) after();
    else requestAnimationFrame(after);
  }

  /** Forgets the bindings of the removed items and conditions, and of the bindings found outside the document. */
  #sweep(): void {
    this.#sweeping = false;
    if (this.#patched) this.#unfilePatched();
    for (const root of this.#removed) {
      // A patch puts back an element it removed where a later child with the same id moves it.
      if (root.isConnected) continue;
      this.#forget(root);
      if (own(root)?.root === true) {
        this.#visit(root, (element) => this.#forget(element));
        this.#unfileInside(root);
      } else {
        // An element a patch removed has no list of its members. The slots of the items below it were taken out above.
        for (const element of root.querySelectorAll("*")) this.#forget(element);
      }
    }
    this.#removed = [];
    for (const effect of this.#stale) if (!effect.element.isConnected) this.#forget(effect.element);
    this.#stale = new Set();
    this.#rt.actions.calls.prune();
  }

  /** Takes out the slots of the items a patch removed, which it removes without unmounting them. */
  #unfilePatched(): void {
    this.#patched = false;
    const { slots, rows } = this.#rt.lists;
    for (const slot of slots.filed()) {
      // Every filed slot belongs to an item whose row is filed under the slot's name until the item is forgotten.
      const owner = rows.get(slot.read!)?.element;
      if (owner === undefined || owner.isConnected) continue;
      slots.unfile(slot);
      this.#removed.push(owner);
    }
  }

  #forget(element: Element): void {
    const bound = own(element);
    if (bound === undefined) return;
    const { index } = this.#rt.render;
    for (const effect of bound.effects ?? []) index.remove(effect);
    if (bound.watch !== undefined) index.remove(bound.watch);
    if (bound.row !== undefined) this.#rt.lists.rows.drop(bound.row);
    if (element instanceof HTMLTemplateElement) this.#rt.lists.watches.drop(element);
    delete bound.effects;
    delete bound.watch;
    delete bound.row;
  }

  /** Takes out the slots of the items of the lists inside `root`, nested ones included, and forgets their roots. */
  #unfileInside(root: Element): void {
    for (const element of own(root)?.members ?? []) {
      if (!(element instanceof HTMLTemplateElement)) continue;
      for (const instance of this.#rt.lists.items(element)) {
        this.#rt.lists.slots.unfile(instance.slot);
        this.#forget(instance.element);
        this.#unfileInside(instance.element);
      }
      const mounted = this.#rt.conditions.mounted(element);
      if (mounted !== undefined) this.#unfileInside(mounted);
    }
  }

  /** Calls `visit` on each element with bindings inside the item or condition `root`, nested ones included. */
  #visit(root: Element, visit: (element: Element) => void): void {
    for (const element of own(root)?.members ?? []) {
      visit(element);
      if (!(element instanceof HTMLTemplateElement)) continue;
      for (const instance of this.#rt.lists.items(element)) this.#visit(instance.element, visit);
      const mounted = this.#rt.conditions.mounted(element);
      if (mounted !== undefined) this.#visit(mounted, visit);
    }
  }
}
