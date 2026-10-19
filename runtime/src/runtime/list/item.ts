import type { Slot } from "../../eval.ts";
import type { Effect } from "../../deps.ts";
import type { Reads } from "../../store/change.ts";
import type { Runtime } from "../../runtime.ts";
import { own, state } from "../element.ts";
import { data } from "../spec.ts";
import type { Row } from "./row.ts";
import type { Shape } from "./shape.ts";

/** One rendered item of a list, which is also the `Row` that runs its parts, so an item takes one object. */
export interface Instance extends Row {
  /** False for an item the server rendered, until the runtime registers its bindings. */
  scanned: boolean;
  /** The item's key as its key expression evaluated it, before a duplicate is told apart by position. */
  key: string;
  /** What evaluating this item's key read, which `watch` is registered under. */
  keyReads: Reads;
  /**
   * Reruns the list when something its key read changes. The key's reads are
   * registered under it rather than under the list's own binding, so a
   * render of the list does not record every item's key reads again.
   */
  watch: Effect | null;
  /**
   * Whether the key read nothing but the item's own fields, so another item
   * with the same value has the same key wherever it sits.
   */
  local: boolean;
  /** The item's value when the list last matched it, to tell a move from a change. */
  value: unknown;
}

/** What `Matcher` found for an item's key: the key, what evaluating it read, and the name it read the item under. */
export interface Keyed {
  readonly key: string;
  readonly keyReads: Reads;
  /** The name `keyReads` read the item under, where they read nothing else. */
  readonly named: string | null;
}

/** The list a new item belongs to, and where its copy comes from. */
export interface Origin {
  readonly template: HTMLTemplateElement;
  readonly proto: Element;
  readonly shape: Shape;
  readonly list: Effect;
}

/** The item whose root is `element`, at `slot`, filed under the slot's name and put on the element. */
function item(
  rt: Runtime,
  element: Element,
  slot: Slot,
  origin: Pick<Origin, "shape" | "template">,
  key: Pick<Instance, "scanned" | "key" | "keyReads" | "local" | "value">,
): Instance {
  const instance: Instance = {
    element,
    slot,
    shape: origin.shape,
    template: origin.template,
    nodes: null,
    ctx: null,
    dirty: 0,
    queued: false,
    keys: null,
    lists: null,
    scanned: key.scanned,
    key: key.key,
    keyReads: key.keyReads,
    watch: null,
    local: key.local,
    value: key.value,
  };
  rt.lists.rows.add(instance);
  state(element).root = true;
  state(element).row = instance;
  return instance;
}

/** `reads`, made by a key that read only the item, under the name `from`, as if it had read it under `to`. */
export function rebased(reads: Reads, from: string, to: string): Reads {
  if (reads.size === 0) return reads;
  const moved: Reads = new Map();
  for (const [path, deep] of reads) moved.set(to + path.slice(from.length), deep);
  return moved;
}

/** A new item at `path` whose value is `value`, built from a clone of `origin.proto` and keyed as `keyed` says. */
export function createItem(
  rt: Runtime,
  origin: Origin,
  path: string,
  keyed: Keyed,
  value: unknown,
): Instance {
  const slot = rt.lists.slots.create(path);
  // A key that read only the item reads it under the item's name, so the item can move without registering it again.
  const keyReads =
    keyed.named === null ? keyed.keyReads : rebased(keyed.keyReads, keyed.named, slot.read!);
  const element = origin.proto.cloneNode(true) as Element;
  const instance = item(rt, element, slot, origin, {
    scanned: true,
    key: keyed.key,
    keyReads,
    local: keyed.named !== null,
    value,
  });
  watchKey(rt, instance, origin.list);
  return instance;
}

/** Takes over the items the server rendered right after `template`, keyed as it keyed them. */
export function adoptItems(
  rt: Runtime,
  template: HTMLTemplateElement,
  shape: Shape,
): Map<string, Instance> {
  const adopted = new Map<string, Instance>();
  for (
    let at = template.nextElementSibling;
    at !== null && data(at).hsSsr !== undefined;
    at = at.nextElementSibling
  ) {
    if (own(at)?.root === true) break;
    const key = data(at).hsSsr!;
    const slot = rt.lists.slots.create("");
    adopted.set(
      key,
      item(
        rt,
        at,
        slot,
        { shape, template },
        {
          scanned: false,
          key,
          keyReads: new Map(),
          local: false,
          value: undefined,
        },
      ),
    );
  }
  return adopted;
}

/** Registers the reads of `instance`'s key under its watch, which reruns `list` when they change. */
export function watchKey(rt: Runtime, instance: Instance, list: Effect): void {
  if (instance.keyReads.size === 0 && instance.watch === null) return;
  if (instance.watch === null) {
    const watch = rt.render.effect(instance.element, () => {
      rt.store.replay(instance.keyReads);
      rt.render.again(list);
    });
    instance.watch = watch;
    state(instance.element).watch = watch;
  }
  rt.render.index.set(instance.watch, instance.keyReads);
}
