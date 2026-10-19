import { type Context, evaluate } from "../eval.ts";
import { DepIndex, type Effect, type Selector } from "../deps.ts";
import { ChangeLookup, type Changes, overlaps } from "../store/change.ts";
import { equal } from "../store/value.ts";
import type { Runtime } from "../runtime.ts";
import { Sweep } from "./render/sweep.ts";
import { comparison, keyOf, scalar } from "./selector.ts";

/**
 * Runs the bindings a change affects, and the new ones, in the order they
 * were made. It owns the `DepIndex` that files each binding under the reads
 * it made when it last ran.
 */
export class Render {
  readonly index = new DepIndex();
  /** Forgets the bindings of what leaves the document. */
  readonly sweep: Sweep;
  /** The changes the current render is running for. */
  changes: Changes = new Map();
  readonly #rt: Runtime;
  /** Bindings to run in the current or next render: new ones, and ones whose item moved. */
  #fresh: Effect[] = [];
  #ran = new Set<Effect>();
  /** The selector the binding being run recorded through `compare`. */
  #selector: Selector | null = null;
  #nextId = 0;

  constructor(rt: Runtime) {
    this.#rt = rt;
    this.sweep = new Sweep(rt);
  }

  /** A binding of `element` that runs `run`, numbered after every binding made before it. */
  effect(element: Element, run: () => void): Effect {
    return { id: this.#nextId++, element, run, deps: null, nodes: new Map(), selector: null };
  }

  /** Has the current or next render run `effects`. */
  queue(effects: readonly Effect[]): void {
    this.#fresh.push(...effects);
  }

  /** Queues `effect` to run again in this render. */
  readonly again = (effect: Effect): void => {
    this.#ran.delete(effect);
    this.#fresh.push(effect);
  };

  /** Runs the bindings that `changes` affects, oldest first, then every binding that has not run yet. */
  run(changes: Changes): void {
    const { slots, rows } = this.#rt.lists;
    this.#ran = new Set();
    this.changes = changes;
    const named = slots.named(changes);
    const found = this.index.affected(named);
    rows.touch(named);
    if (changes.size > 0) {
      let lookup: ChangeLookup | null = null;
      this.index.selected(
        (path) => (lookup ??= new ChangeLookup(changes)).affects(path, true),
        (path) => keyOf(this.#rt.store.untracked(() => this.#rt.store.get(path))),
        found,
      );
    }
    for (const effect of [...found].toSorted((a, b) => a.id - b.id)) this.runEffect(effect);
    while (this.#fresh.length > 0 || rows.pending()) {
      while (this.#fresh.length > 0) {
        const batch = this.#fresh;
        this.#fresh = [];
        for (const effect of batch) this.runEffect(effect);
      }
      rows.flush();
    }
    // The bindings that ran hold their elements, which may have left the document.
    this.#ran = new Set();
    this.changes = new Map();
  }

  runEffect(effect: Effect): void {
    if (this.#ran.has(effect)) return;
    this.#ran.add(effect);
    if (!effect.element.isConnected) {
      this.sweep.stale(effect);
      return;
    }
    this.#selector = null;
    const reads = this.#rt.store.track(() => {
      try {
        effect.run();
      } catch (error) {
        console.error("hypersig:", error);
      }
    });
    this.index.set(effect, reads);
    this.index.select(effect, this.#selector);
    this.#selector = null;
  }

  /** Whether the condition `ast` holds, for the binding being run, comparing a page signal through `compare` where it can. */
  truth(ast: unknown, ctx: Context): boolean {
    const compared = comparison(ast);
    return compared === null
      ? Boolean(evaluate(ast, ctx))
      : this.compare(compared[0], compared[1], ctx);
  }

  /**
   * Evaluates `["eq", ["sig", path], other]` for the binding being run. Where
   * `other` is a JSON scalar that does not read `path`, it records a selector
   * in place of reading `path`, so a change there runs only the bindings
   * whose result it can flip.
   */
  compare(path: string, other: unknown, ctx: Context): boolean {
    const { store } = this.#rt;
    let value: unknown;
    const reads = store.track(() => {
      value = evaluate(other, ctx);
    });
    store.replay(reads);
    if (!scalar(value) || [...reads.keys()].some((read) => overlaps(read, path)))
      return equal(store.get(path), value);
    const current = store.untracked(() => store.get(path));
    this.#selector = { path, key: keyOf(value), current: keyOf(current) };
    return equal(current, value);
  }
}
