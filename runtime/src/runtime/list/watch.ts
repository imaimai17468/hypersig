import type { Slot } from "../../eval.ts";
import type { Effect } from "../../deps.ts";
import type { Reads } from "../../store/change.ts";
import type { Runtime } from "../../runtime.ts";
import { ANY, keyOf } from "../selector.ts";
import type { Row } from "./row.ts";
import { readName, type Shape } from "./shape.ts";

/** The items whose selector part compared a value under each key, kept for one list. */
export interface Selection {
  readonly path: string;
  /** The key of the signal's value when the selection last looked. */
  current: string | undefined;
  readonly byKey: Map<string, Row | Row[]>;
}

/** The bindings a list keeps for its items' reads outside them. */
interface ListParts {
  readonly list: Effect;
  readonly effects: readonly Effect[];
  /** By part, for the selector parts. */
  readonly selections: readonly (Selection | undefined)[];
}

/** Files `row` in `selection` under `key`. */
function fileRow(selection: Selection, key: string, row: Row): void {
  const filed = selection.byKey.get(key);
  selection.byKey.set(
    key,
    filed === undefined ? row : Array.isArray(filed) ? [...filed, row] : [filed, row],
  );
}

/** Takes `row` out of `selection`, where it is filed under `key`. */
function unfileRow(selection: Selection, key: string, row: Row): void {
  const filed = selection.byKey.get(key);
  if (filed === row) selection.byKey.delete(key);
  else if (Array.isArray(filed)) {
    const rest = filed.filter((other) => other !== row);
    if (rest.length === 0) selection.byKey.delete(key);
    else selection.byKey.set(key, rest.length === 1 ? rest[0]! : rest);
  }
}

/** The rows filed under one key of a selection, as a list. */
function filedRows(filed: Row | Row[] | undefined): readonly Row[] {
  if (filed === undefined) return [];
  return Array.isArray(filed) ? filed : [filed];
}

/**
 * The bindings each list keeps for the reads its items' parts make outside
 * the items: one per read, which marks the parts that make it, and one per
 * selector part, which marks only the rows its signal's change can flip.
 */
export class ListWatches {
  readonly #rt: Runtime;
  readonly #lists = new WeakMap<HTMLTemplateElement, ListParts>();

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** The binding the list at `template` renders under, once it has rendered. */
  list(template: HTMLTemplateElement): Effect | undefined {
    return this.#lists.get(template)?.list;
  }

  /** The selection of the selector part at `index` of the list at `template`. */
  selection(template: HTMLTemplateElement, index: number): Selection | undefined {
    return this.#lists.get(template)?.selections[index];
  }

  /**
   * Makes the bindings of the list at `template`, rendered under `list` with
   * items shaped as `shape` inside the items `outer`, unless it has them.
   */
  watch(template: HTMLTemplateElement, shape: Shape, outer: readonly Slot[], list: Effect): void {
    const existing = this.#lists.get(template);
    if (existing !== undefined && existing.list === list) return;
    this.drop(template);
    if (existing !== undefined) {
      // The rows filed themselves in the selections being replaced, so they run every part again to file themselves in the new ones.
      for (const instance of this.#rt.lists.items(template)) {
        instance.row.keys = null;
        this.#rt.lists.rows.mark(instance.row, shape.all);
      }
    }
    const effects: Effect[] = [];
    for (const [read, mask] of shape.external) {
      effects.push(
        this.#effect(template, new Map([[readName(read, outer), true]]), () => {
          for (const instance of this.#rt.lists.items(template))
            this.#rt.lists.rows.mark(instance.row, mask);
        }),
      );
    }
    const selections: (Selection | undefined)[] = [];
    shape.parts.forEach((part, index) => {
      if (part.selector === null) return;
      const selection: Selection = {
        path: part.selector.path,
        current: undefined,
        byKey: new Map(),
      };
      selection.current = this.#keyAt(selection.path);
      effects.push(
        this.#effect(template, new Map([[selection.path, true]]), () =>
          this.#reselect(selection, index),
        ),
      );
      selections[index] = selection;
    });
    this.#lists.set(template, { list, effects, selections });
    this.#rt.render.queue(effects);
  }

  /**
   * Files `row` under `key` in the selection of its part at `index`, taking
   * it out from under the key it was filed under before. Returns whether the
   * list has that selection.
   */
  refile(row: Row, index: number, key: string): boolean {
    const selection = this.selection(row.template, index);
    if (selection === undefined) return false;
    const was = row.keys?.[index];
    if (was !== undefined) unfileRow(selection, was, row);
    fileRow(selection, key, row);
    return true;
  }

  /** Takes `row` out of every selection it filed itself in. */
  unfileAll(row: Row): void {
    row.keys?.forEach((key, index) => {
      const selection = this.selection(row.template, index);
      if (key !== undefined && selection !== undefined) unfileRow(selection, key, row);
    });
  }

  /** Forgets the bindings of the list at `template`. */
  drop(template: HTMLTemplateElement): void {
    const parts = this.#lists.get(template);
    if (parts === undefined) return;
    for (const effect of parts.effects) this.#rt.render.index.remove(effect);
    this.#lists.delete(template);
  }

  /** A binding of `template` that reads `reads` and runs `changed` each time after its first run. */
  #effect(template: HTMLTemplateElement, reads: Reads, changed: () => void): Effect {
    let started = false;
    return this.#rt.render.effect(template, () => {
      this.#rt.store.replay(reads);
      if (started) changed();
      started = true;
    });
  }

  #keyAt(path: string): string {
    return keyOf(this.#rt.store.untracked(() => this.#rt.store.get(path)));
  }

  /** Marks the part at `index` of the rows whose comparison the new value of the selection's signal can flip. */
  #reselect(selection: Selection, index: number): void {
    const was = selection.current!;
    const key = this.#keyAt(selection.path);
    selection.current = key;
    const reach = (filed: Row | Row[] | undefined): void => {
      for (const row of filedRows(filed)) this.#rt.lists.rows.mark(row, 1 << index);
    };
    reach(selection.byKey.get(ANY));
    if (key === was) return;
    reach(selection.byKey.get(was));
    reach(selection.byKey.get(key));
  }
}
