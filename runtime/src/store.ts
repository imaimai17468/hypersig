import { type Changes, type Reads, positionOf } from "./store/change.ts";
import { getPath, join, setCopied, setPath } from "./store/path.ts";
import { isContainer, type Values } from "./store/value.ts";

/**
 * The page's signals as one JSON object. Reads made inside `track` are
 * recorded, and listeners receive the changes made since they last ran.
 */
export class Store {
  #values: Values;
  #listeners = new Set<(changes: Changes) => void>();
  #changes = new Map<string, boolean>();
  #reads: Reads | null = null;
  /** The prefixes, each a path and a dot, under which `fold` records no reads of its own. */
  #folds: string[] = [];
  #derive: ((changes: Changes) => void) | null = null;

  constructor(initial: Values) {
    this.#values = { ...initial };
  }

  get(path: string): unknown {
    this.#note(path, true);
    return getPath(this.#values, path);
  }

  /** The value at `path`, recorded as a read of `read`, the name the caller tracks it under. */
  getAs(path: string, read: string): unknown {
    this.#note(read, true);
    return getPath(this.#values, path);
  }

  /**
   * The value at `field` inside the value at `base`, recorded as a read of
   * `field` under `read`. The joined path is built only where a read is
   * being recorded.
   */
  getIn(base: string, field: string, read: string): unknown {
    if (this.#reads !== null) this.#note(join(read, field), true);
    return getPath(getPath(this.#values, base), field);
  }

  /** Records a read of the position of the list item at `path`, for `index`. */
  getPosition(path: string): void {
    if (this.#reads !== null) this.#note(positionOf(path), true);
  }

  /**
   * Reads a value whose contents the caller reads separately, path by path,
   * so a change inside it does not count as a change to this read. Lists use
   * it: a binding over a list reruns when the list is replaced or resized,
   * and through its per-item reads when an item it looked at changes.
   */
  getShallow(path: string): unknown {
    return this.getShallowAs(path, path);
  }

  /** `getShallow(path)`, recorded as a read of `read`. */
  getShallowAs(path: string, read: string): unknown {
    this.#note(read, false);
    return getPath(this.#values, path);
  }

  #note(read: string, deep: boolean): void {
    const reads = this.#reads;
    if (reads === null) return;
    for (const fold of this.#folds) if (read.startsWith(fold)) return;
    if (deep) reads.set(read, true);
    else if (!reads.has(read)) reads.set(read, false);
  }

  /**
   * Runs `body`, recording one deep read of `read` in place of the reads it
   * makes below `read`, for a value computed from every item of a list,
   * which a change to any item can change.
   */
  fold<T>(read: string, body: () => T): T {
    this.#note(read, true);
    this.#folds.push(`${read}.`);
    try {
      return body();
    } finally {
      this.#folds.pop();
    }
  }

  set(path: string, value: unknown): void {
    this.#write(path, value, null);
  }

  /**
   * Sets each path to its value, as `set` would one after another. The
   * arrays and objects on the way to them are copied once for all of them, so
   * writing one field of every item of a list copies the list once.
   */
  setMany(writes: readonly (readonly [path: string, value: unknown])[]): void {
    const copied = new Set<unknown>();
    for (const [path, value] of writes) this.#write(path, value, copied);
  }

  /** Writes `value` at `path`, copying only what `copied` does not hold where it is given, and adding the copies to it. */
  #write(path: string, value: unknown, copied: Set<unknown> | null): void {
    const previous = getPath(this.#values, path);
    if (Object.is(previous, value)) return;
    this.#values = (
      copied === null
        ? setPath(this.#values, path, value)
        : setCopied(this.#values, path, value, copied)
    ) as Values;
    if (this.#changes.size === 0) queueMicrotask(() => this.#notify());
    if (
      isContainer(previous) &&
      isContainer(value) &&
      Array.isArray(previous) === Array.isArray(value)
    ) {
      if (Array.isArray(previous) && Array.isArray(value)) this.#recordItems(path, previous, value);
      else {
        this.#record(path, false);
        for (const key of new Set([...Object.keys(previous), ...Object.keys(value)])) {
          if (!Object.is(previous[key], value[key])) this.#record(join(path, key), true);
        }
      }
    } else {
      this.#record(path, true);
    }
  }

  /** Records the write of the list `value` over the list `previous` at `path`. */
  #recordItems(path: string, previous: readonly unknown[], value: readonly unknown[]): void {
    const length = Math.max(previous.length, value.length);
    const differing: number[] = [];
    for (let index = 0; index < length; index++)
      if (!Object.is(previous[index], value[index])) differing.push(index);
    // Where most items differ and the write is not an append onto items already there, one change for the list
    // costs less than one per item, and a list that renders it compares each item with the one it drew.
    if (
      differing.length * 2 > length &&
      (previous.length === 0 || differing[0]! < previous.length)
    ) {
      this.#record(path, true);
      return;
    }
    this.#record(path, false);
    for (const index of differing) this.#record(join(path, String(index)), true);
  }

  #record(path: string, inside: boolean): void {
    this.#changes.set(path, this.#changes.get(path) === true || inside);
  }

  merge(values: Values): void {
    for (const [path, value] of Object.entries(values)) this.set(path, value);
  }

  /** Runs `body` and returns the paths it read. */
  track(body: () => void): Reads {
    const outer = this.#reads;
    const folds = this.#folds;
    const reads: Reads = new Map();
    this.#reads = reads;
    this.#folds = [];
    try {
      body();
    } finally {
      this.#reads = outer;
      this.#folds = folds;
    }
    return reads;
  }

  /** Records `reads` in the enclosing `track` as if they had been made again. */
  replay(reads: Reads): void {
    if (this.#reads === null) return;
    for (const [path, deep] of reads) {
      if (deep || !this.#reads.has(path))
        this.#reads.set(path, deep || this.#reads.get(path) === true);
    }
  }

  /** Runs `body` without recording its reads in the enclosing `track`. */
  untracked<T>(body: () => T): T {
    const outer = this.#reads;
    this.#reads = null;
    try {
      return body();
    } finally {
      this.#reads = outer;
    }
  }

  /** Sets what `settle` runs: the update of the values computed from others. */
  derive(update: (changes: Changes) => void): void {
    this.#derive = update;
  }

  /**
   * Updates the computed values that the writes since the last notification
   * affect, so a later step of the same statement reads them up to date
   * before the page renders.
   */
  settle(): void {
    if (this.#changes.size > 0) this.#derive?.(this.#changes);
  }

  subscribe(listener: (changes: Changes) => void): void {
    this.#listeners.add(listener);
  }

  #notify(): void {
    const changes = this.#changes;
    this.#changes = new Map();
    for (const listener of this.#listeners) listener(changes);
  }
}
