import type { Slot } from "../../eval.ts";
import type { Runtime } from "../../runtime.ts";
import { own } from "../element.ts";
import type { Instance } from "./item.ts";

/**
 * How long one task may spend taking over server-rendered items before it
 * yields, in milliseconds. A task over 50 ms counts as blocking the page.
 */
const ADOPT_BUDGET_MS = 30;

/**
 * The server-rendered list items still waiting for the runtime to register
 * their bindings. They already show their values, so they wait for later
 * tasks, unless an event reaches one first.
 */
export class Adoption {
  readonly #rt: Runtime;
  /** Registers the bindings of a waiting item, by its root. */
  readonly #waiting = new Map<Element, () => void>();
  #draining = false;

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Whether the item whose root is `element` is waiting. */
  has(element: Element): boolean {
    return this.#waiting.has(element);
  }

  /** Registers the bindings of the server-rendered `instance` in a later task. */
  wait(instance: Instance, proto: Element, scope: readonly Slot[]): void {
    this.#waiting.set(instance.element, () => {
      this.#waiting.delete(instance.element);
      if (instance.scanned || !instance.element.isConnected) return;
      instance.scanned = true;
      this.#rt.bindings.attributes.mount(instance.element, proto, scope, true, instance);
    });
    if (this.#draining) return;
    this.#draining = true;
    setTimeout(() => this.#drain(), 0);
  }

  /** Registers every waiting item now. */
  all(): void {
    for (const mount of this.#waiting.values()) mount();
  }

  /** Registers waiting items until the budget runs out, then yields to the browser and continues. */
  #drain(): void {
    const end = performance.now() + ADOPT_BUDGET_MS;
    while (this.#waiting.size > 0 && performance.now() < end) {
      let count = 0;
      for (const mount of this.#waiting.values()) {
        mount();
        if (++count === 50) break;
      }
      this.#rt.render.run(new Map());
    }
    if (this.#waiting.size > 0) setTimeout(() => this.#drain(), 0);
    else this.#draining = false;
  }

  /**
   * Registers the waiting items that contain `element`, outermost first, so
   * an event finds their bindings. Only the bindings of nested lists and
   * conditions run here, to reach the items inside them. The rest run in the
   * next render, so a binding does not write a control while the event that
   * changes it is still being dispatched.
   */
  around(element: Element): void {
    while (this.#waiting.size > 0) {
      const outermost = this.#outermost(element);
      if (outermost === undefined) return;
      this.#waiting.get(outermost)!();
      for (const member of own(outermost)?.members ?? []) {
        if (member instanceof HTMLTemplateElement)
          for (const effect of own(member)?.effects ?? []) this.#rt.render.runEffect(effect);
      }
      const row = own(outermost)?.row;
      row?.nodes?.forEach((node, index) => {
        const { kind } = row.shape.parts[index]!;
        if ((kind === "if" || kind === "each") && node !== undefined)
          this.#rt.store.untracked(() => this.#rt.lists.rows.runPart(row, index));
      });
    }
  }

  /** The outermost waiting item from `element` up. */
  #outermost(element: Element): Element | undefined {
    let outermost: Element | undefined;
    for (let at: Element | null = element; at !== null; at = at.parentElement) {
      if (this.#waiting.has(at)) outermost = at;
    }
    return outermost;
  }
}
