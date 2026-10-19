import type { Context, Slot } from "../../eval.ts";
import type { Effect } from "../../deps.ts";
import { type Changes } from "../../store/change.ts";
import { equal } from "../../store/value.ts";
import type { Runtime } from "../../runtime.ts";
import { ANY, keyOf, scalar } from "../selector.ts";
import type { Bind } from "../spec.ts";
import { CONDITIONS, write, type Write } from "../write.ts";
import { maskFor, reaches } from "./reach.ts";
import type { Shape } from "./shape.ts";

/** A list item's parts, and which of them the next render runs. */
export interface Row {
  /** The item's root element. */
  readonly element: Element;
  readonly slot: Slot;
  readonly shape: Shape;
  readonly template: HTMLTemplateElement;
  /** The element each part writes, by part, once the item is mounted. */
  nodes: readonly (Element | undefined)[] | null;
  ctx: Context | null;
  /** The parts to run, one bit each. */
  dirty: number;
  queued: boolean;
  /** The key of the value each selector part compared, where it filed the row in its `Selection`. */
  keys: (string | undefined)[] | null;
  /**
   * For each `each` part that has run, the binding its nested list is
   * rendered under. It is never registered for reads, and running it marks
   * the part, so a rerun the nested list asks for goes through the row.
   */
  lists: (Effect | undefined)[] | null;
}

/** The rows of the rendered list items, and the queue of those with parts to run. */
export class Rows {
  readonly #rt: Runtime;
  /** The rows of the list items, by slot name, which the changes `Slots.named` names reach. */
  readonly #rows = new Map<string, Row>();
  /** The rows with parts to run, in the order they were marked. */
  #queue: Row[] = [];

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Files `row` under its slot's name, where the changes `Slots.named` names reach it. */
  add(row: Row): void {
    this.#rows.set(row.slot.read!, row);
  }

  /** The row filed under the slot name `name`, until its item is forgotten. */
  get(name: string): Row | undefined {
    return this.#rows.get(name);
  }

  /** Forgets `row`, and takes it out of the selections it filed itself in. */
  drop(row: Row): void {
    if (this.#rows.get(row.slot.read!) === row) this.#rows.delete(row.slot.read!);
    this.#rt.lists.watches.unfileAll(row);
  }

  /** Has the next render run the parts of `row` in `mask`. */
  mark(row: Row, mask: number): void {
    if (mask === 0) return;
    row.dirty |= mask;
    if (row.queued) return;
    row.queued = true;
    this.#queue.push(row);
  }

  /** Whether a row has parts to run. */
  pending(): boolean {
    return this.#queue.length > 0;
  }

  /** Marks the parts of each row that a change `Slots.named` named under the row's slot can reach. */
  touch(named: Changes): void {
    if (this.#rows.size === 0) return;
    for (const [path, inside] of named) {
      if (!path.startsWith("@")) continue;
      const dot = path.indexOf(".");
      const row = this.#rows.get(dot === -1 ? path : path.slice(0, dot));
      if (row === undefined) continue;
      const rest = dot === -1 ? "" : path.slice(dot + 1);
      const { shape } = row;
      const list = this.#rt.lists.watches.list(row.template);
      if (list !== undefined && shape.keyFields?.some((field) => reaches(field, rest, inside)))
        this.#rt.render.again(list);
      this.mark(row, maskFor(shape, rest, inside));
    }
  }

  /** Runs the marked parts of the queued rows. Their reads are known already, so none are recorded. */
  flush(): void {
    this.#rt.store.untracked(() => {
      while (this.#queue.length > 0) {
        const batch = this.#queue;
        this.#queue = [];
        for (const row of batch) {
          row.queued = false;
          if (row.element.isConnected) this.#runParts(row);
          else row.dirty = 0;
        }
      }
    });
  }

  /**
   * Runs the marked parts of `rows`, which `Lists.render` created and has
   * not put into the document yet, so the browser lays each one out once,
   * with its content. They stay queued with nothing marked.
   */
  fillIn(rows: readonly Row[]): void {
    if (rows.length === 0) return;
    this.#rt.store.untracked(() => {
      for (const row of rows) this.#runParts(row);
    });
  }

  #runParts(row: Row): void {
    const mask = row.dirty;
    row.dirty = 0;
    if (row.ctx === null) return;
    for (let index = 0; mask >>> index !== 0; index++) {
      if ((mask & (1 << index)) === 0) continue;
      try {
        this.runPart(row, index);
      } catch (error) {
        console.error("hypersig:", error);
      }
    }
  }

  runPart(row: Row, index: number): void {
    const element = row.nodes![index];
    if (element === undefined) return;
    const part = row.shape.parts[index]!;
    const ctx = row.ctx!;
    if (part.kind === "bind") this.#rt.bindings.controls.write(element, part.ast as Bind, ctx);
    else if (part.kind === "if")
      this.#rt.conditions.render(
        element as HTMLTemplateElement,
        part.proto,
        this.#holds(row, index),
        ctx.scope,
      );
    else if (part.kind === "each") {
      const [place, depth] = part.ast as [unknown, number];
      const lists = (row.lists ??= []);
      const list = (lists[index] ??= this.#rt.render.effect(element, () =>
        this.mark(row, 1 << index),
      ));
      this.#rt.lists.render(
        element as HTMLTemplateElement,
        part.proto,
        place,
        depth,
        ctx.scope,
        list,
      );
    } else {
      const kind: Write = part.kind;
      write(
        element,
        kind,
        part.name,
        CONDITIONS.has(kind) ? this.#holds(row, index) : part.value!(ctx),
      );
    }
  }

  /** Whether the condition of a show, class, or toggle part holds, filing a selector part's row under the value it compared. */
  #holds(row: Row, index: number): boolean {
    const part = row.shape.parts[index]!;
    const ctx = row.ctx!;
    if (part.selector === null) return Boolean(part.value!(ctx));
    const other = part.selector.other(ctx);
    const key = scalar(other) ? keyOf(other) : ANY;
    const keys = (row.keys ??= []);
    if (keys[index] !== key && this.#rt.lists.watches.refile(row, index, key)) keys[index] = key;
    return equal(this.#rt.store.get(part.selector.path), other);
  }
}
