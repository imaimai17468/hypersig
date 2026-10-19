import { type Compiled, compile, type Context, evaluate, type Slot } from "../../eval.ts";
import { join } from "../../store/path.ts";
import { positionOf, type Reads } from "../../store/change.ts";
import type { Runtime } from "../../runtime.ts";
import { keyOf } from "../selector.ts";
import { data, parse } from "../spec.ts";
import { createItem, type Instance, type Keyed, type Origin, rebased, watchKey } from "./item.ts";

const NO_READS: Reads = new Map();

/**
 * Pairs each value of a list with the item that showed it, by key, for one
 * render of the list: it moves and marks the items it keeps and builds the
 * ones it does not find.
 */
export class Matcher {
  /** The items `match` built in this render. */
  readonly created = new Set<Instance>();
  /** The key `match` filed its last item under. */
  matched = "";
  readonly #rt: Runtime;
  readonly #origin: Origin;
  readonly #items: readonly unknown[];
  readonly #path: string;
  readonly #read: string;
  readonly #outer: readonly Slot[];
  readonly #keyAst: unknown;
  readonly #key: Compiled | null;
  /** The previous items by position and, built on first use, by value, to find one whose key holds for a value. */
  #placed: Instance[] | null = null;
  #byValue: Map<unknown, Instance> | null = null;
  /** The slot and context a key that reads only the item is evaluated in, moved to each item in turn. */
  readonly #keySlot: Slot = { path: "", read: undefined };
  readonly #keyCtx: Context;

  constructor(
    rt: Runtime,
    origin: Origin,
    items: readonly unknown[],
    place: { readonly path: string; readonly read: string },
    outer: readonly Slot[],
  ) {
    this.#rt = rt;
    this.#origin = origin;
    this.#items = items;
    this.#path = place.path;
    this.#read = place.read;
    this.#outer = outer;
    this.#keyAst = parse(data(origin.proto).hsKey);
    this.#key = this.#keyAst === undefined ? null : compile(this.#keyAst);
    this.#keyCtx = { store: rt.store, scope: [...outer, this.#keySlot] };
  }

  /**
   * The item for the value at `index`: the item of `previous` with its key,
   * moved there, or a new one. It is filed in `next` under its key. An
   * `appending` value has no item at its position before.
   */
  match(
    index: number,
    previous: ReadonlyMap<string, Instance>,
    next: Map<string, Instance>,
    appending = false,
  ): Instance {
    const itemPath = join(this.#path, String(index));
    const itemAt = this.#read === this.#path ? itemPath : join(this.#read, String(index));
    const keyed = this.#keyed(index, previous, appending, itemPath, itemAt);
    let { key } = keyed;
    if (next.has(key)) {
      console.warn(`hypersig: duplicate key ${key} in a list; matching it by position`);
      key = `${key}#${index}`;
    }
    let instance = previous.get(key);
    if (instance === undefined || !instance.element.isConnected) {
      instance = createItem(this.#rt, this.#origin, itemPath, keyed, this.#items[index]);
      this.created.add(instance);
    } else this.#keep(instance, keyed, itemPath, index);
    next.set(key, instance);
    this.matched = key;
    return instance;
  }

  #keyed(
    index: number,
    previous: ReadonlyMap<string, Instance>,
    appending: boolean,
    itemPath: string,
    itemAt: string,
  ): Keyed {
    if (this.#key === null) return { key: `#${index}`, keyReads: new Map(), named: null };
    // An appended item has no previous item at its position, and is rarely one moved from elsewhere in the list.
    const same = appending ? undefined : this.#sameValue(previous, index);
    if (same !== undefined)
      return { key: same.key, keyReads: same.keyReads, named: same.slot.read! };
    if (this.#origin.shape.keyFields !== null) {
      // A key that reads only the item's fields is watched through the item's row, so its reads are not recorded.
      this.#keySlot.path = itemPath;
      this.#keySlot.read = itemAt === itemPath ? undefined : itemAt;
      const key = keyOf(this.#rt.store.untracked(this.#evaluateKey));
      return { key, keyReads: NO_READS, named: itemAt };
    }
    // The key reads the item by its path, or by a name below the enclosing item's, so the list reruns when they change.
    const slot: Slot = itemAt === itemPath ? { path: itemPath } : { path: itemPath, read: itemAt };
    let key = "";
    const keyReads = this.#rt.store.track(() => {
      key = JSON.stringify(
        evaluate(this.#keyAst, { store: this.#rt.store, scope: [...this.#outer, slot] }),
      );
    });
    const local = [...keyReads.keys()].every(
      (at) => at !== positionOf(itemAt) && (at === itemAt || at.startsWith(`${itemAt}.`)),
    );
    return { key, keyReads, named: local ? itemAt : null };
  }

  readonly #evaluateKey = (): unknown => this.#key!(this.#keyCtx);

  #sameValue(previous: ReadonlyMap<string, Instance>, index: number): Instance | undefined {
    this.#placed ??= [...previous.values()];
    const item = this.#items[index];
    const at = this.#placed[index];
    if (at !== undefined && at.local && Object.is(at.value, item)) return at;
    if (this.#byValue === null) {
      this.#byValue = new Map();
      for (const instance of this.#placed)
        if (instance.local && !this.#byValue.has(instance.value))
          this.#byValue.set(instance.value, instance);
    }
    // `match` gives an instance its new value as it goes, so the one found may no longer hold `item`.
    const found = this.#byValue.get(item);
    return found !== undefined && found.local && Object.is(found.value, item) ? found : undefined;
  }

  /** Moves `instance`, kept for the value at `index`, to `itemPath`, and marks what its new key or value changes. */
  #keep(instance: Instance, keyed: Keyed, itemPath: string, index: number): void {
    const rt = this.#rt;
    const { shape, list } = this.#origin;
    instance.key = keyed.key;
    const name = instance.slot.read!;
    const keyReads =
      keyed.named !== null && keyed.named !== name
        ? rebased(keyed.keyReads, keyed.named, name)
        : keyed.keyReads;
    if (instance.keyReads !== keyReads || instance.watch === null) {
      instance.keyReads = keyReads;
      watchKey(rt, instance, list);
    }
    instance.local = keyed.named !== null;
    const changed = !Object.is(instance.value, this.#items[index]);
    const owned = instance.scanned && !shape.whole;
    if (instance.slot.path !== itemPath) {
      const from = instance.slot.path;
      rt.lists.slots.place(instance.slot, itemPath);
      if (owned) {
        rt.lists.slots.relocate(instance.element, from, itemPath);
        if (changed) rt.render.index.within(name, rt.render.again);
        else rt.render.index.at(positionOf(name), rt.render.again);
      }
      rt.lists.rows.mark(instance, changed ? shape.all : shape.moved);
    } else if (changed) {
      if (owned) rt.render.index.within(name, rt.render.again);
      rt.lists.rows.mark(instance, shape.all);
    }
    instance.value = this.#items[index];
  }
}
