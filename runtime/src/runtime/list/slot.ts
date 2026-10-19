import type { Slot } from "../../eval.ts";
import type { Changes } from "../../store/change.ts";
import type { Runtime } from "../../runtime.ts";
import { own } from "../element.ts";

/**
 * The slots of the rendered list items at each store path: one, or one per
 * list where several lists show the same signal. The bindings of an item
 * record their reads under its slot's name, so a change written at the
 * item's path is looked up here to reach them.
 */
export class Slots {
  readonly #rt: Runtime;
  readonly #slots = new Map<string, Slot | Slot[]>();
  #next = 0;

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** A slot for a new item at `path`, with a name of its own, filed unless `path` is empty. */
  create(path: string): Slot {
    const slot: Slot = { path, read: `@${this.#next++}` };
    if (path !== "") this.file(path, slot);
    return slot;
  }

  /** Files `slot` under `path`, where its item now sits. */
  place(slot: Slot, path: string): void {
    this.unfile(slot);
    slot.path = path;
    this.file(path, slot);
  }

  file(path: string, slot: Slot): void {
    const filed = this.#slots.get(path);
    if (filed === undefined) this.#slots.set(path, slot);
    else if (Array.isArray(filed)) filed.push(slot);
    else if (filed !== slot) this.#slots.set(path, [filed, slot]);
  }

  /** Takes `slot` out, where it is filed under its path. */
  unfile(slot: Slot): void {
    const filed = this.#slots.get(slot.path);
    if (filed === slot) {
      this.#slots.delete(slot.path);
    } else if (Array.isArray(filed)) {
      const rest = filed.filter((other) => other !== slot);
      if (rest.length === 0) this.#slots.delete(slot.path);
      else this.#slots.set(slot.path, rest.length === 1 ? rest[0]! : rest);
    }
  }

  /** Every filed slot, copied, so the caller can unfile them as it goes. */
  filed(): Slot[] {
    return [...this.#slots.values()].flat();
  }

  /** Gives the items of the lists inside `root`, whose item moved from `from` to `to`, their new paths. */
  relocate(root: Element, from: string, to: string): void {
    const prefix = `${from}.`;
    for (const element of own(root)?.members ?? []) {
      if (!(element instanceof HTMLTemplateElement)) continue;
      for (const instance of this.#rt.lists.items(element)) {
        if (!instance.slot.path.startsWith(prefix)) continue;
        this.place(instance.slot, to + instance.slot.path.slice(from.length));
        this.relocate(instance.element, from, to);
      }
      const mounted = this.#rt.conditions.mounted(element);
      if (mounted !== undefined) this.relocate(mounted, from, to);
    }
  }

  /**
   * `changes` with each change written inside a rendered list item added
   * again under the item's slot name, where the bindings inside the item
   * recorded their reads. A change under a list that `changes` replaces is
   * left to that list, which compares its items when it renders.
   */
  named(changes: Changes): Changes {
    if (this.#slots.size === 0) return changes;
    let named: Map<string, boolean> | null = null;
    for (const [path, inside] of changes) {
      for (let dot = path.indexOf("."); ; dot = path.indexOf(".", dot + 1)) {
        const prefix = dot === -1 ? path : path.slice(0, dot);
        const filed = this.#slots.get(prefix);
        if (filed !== undefined) {
          if (changes.has(prefix.slice(0, prefix.lastIndexOf(".")))) break;
          named ??= new Map(changes);
          for (const slot of Array.isArray(filed) ? filed : [filed]) {
            const name = slot.read! + path.slice(prefix.length);
            named.set(name, named.get(name) === true || inside);
          }
        }
        if (dot === -1) break;
      }
    }
    return named ?? changes;
  }
}
