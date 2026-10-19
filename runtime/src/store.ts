export type Values = Record<string, unknown>;

/** Joins dotted path parts, skipping empty ones. */
export function join(...parts: string[]): string {
  let joined = "";
  for (const part of parts) {
    if (part !== "") joined = joined === "" ? part : `${joined}.${part}`;
  }
  return joined;
}

const splits = new Map<string, readonly string[]>();

/** `path.split(".")`, remembered, because the same paths are read on every render. */
function segmentsOf(path: string): readonly string[] {
  let segments = splits.get(path);
  if (segments === undefined) {
    segments = path.split(".");
    if (splits.size > 100_000) splits.clear();
    splits.set(path, segments);
  }
  return segments;
}

/** Reads the value at a dotted path, where a numeric segment indexes an array. */
export function getPath(value: unknown, path: string): unknown {
  if (path === "") return value;
  // A field of an item is read once per item, and most fields are one segment.
  if (!path.includes(".")) return value !== null && typeof value === "object" ? (value as Record<string, unknown>)[path] : undefined;
  let current = value;
  for (const segment of segmentsOf(path)) {
    if (current === null || typeof current !== "object") return undefined;
    current = (current as Record<string, unknown>)[segment];
  }
  return current;
}

/** Returns a copy of `value` with the value at `path` replaced, copying each container on the way. */
export function setPath(value: unknown, path: string, next: unknown): unknown {
  return path === "" ? next : setAt(value, segmentsOf(path), 0, next);
}

/** `setPath` for the path whose segments from `at` on are left in `segments`. */
function setAt(value: unknown, segments: readonly string[], at: number, next: unknown): unknown {
  if (at === segments.length) return next;
  const head = segments[at]!;
  if (Array.isArray(value)) {
    const copy = [...value];
    copy[Number(head)] = setAt(copy[Number(head)], segments, at + 1, next);
    return copy;
  }
  const record = value !== null && typeof value === "object" ? (value as Values) : {};
  return { ...record, [head]: setAt(record[head], segments, at + 1, next) };
}

/** `setPath`, changing in place the containers on the way that `copied` holds, and copying the others into it. */
function setCopied(root: unknown, path: string, next: unknown, copied: Set<unknown>): unknown {
  if (path === "") return next;
  const own = (value: unknown): Record<string, unknown> => {
    if (copied.has(value)) return value as Record<string, unknown>;
    const copy = Array.isArray(value) ? [...value] : value !== null && typeof value === "object" ? { ...(value as Values) } : {};
    copied.add(copy);
    return copy as Record<string, unknown>;
  };
  const top = own(root);
  const segments = path.split(".");
  let at = top;
  for (let index = 0; index < segments.length - 1; index++) {
    const child = own(at[segments[index]!]);
    at[segments[index]!] = child;
    at = child;
  }
  at[segments.at(-1)!] = next;
  return top;
}

/** Structural equality over JSON values. */
export function equal(a: unknown, b: unknown): boolean {
  if (Object.is(a, b) || a === b) return true;
  if (a === null || b === null || typeof a !== "object" || typeof b !== "object") return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  const keysA = Object.keys(a);
  const keysB = Object.keys(b);
  if (keysA.length !== keysB.length) return false;
  return keysA.every((key) => equal((a as Values)[key], (b as Values)[key]));
}

/** Whether a change at one path can change the value read at the other. */
export function overlaps(a: string, b: string): boolean {
  return a === b || a.startsWith(`${b}.`) || b.startsWith(`${a}.`);
}

/**
 * The paths a binding read. A deep read (`true`) depends on everything inside
 * the value. A shallow read (`false`) depends only on which value sits at the
 * path, such as a list whose items are read one by one through their own paths.
 */
export type Reads = Map<string, boolean>;

/**
 * The paths a write changed. `true` means the value at the path and anything
 * inside it may differ. `false` means the value at the path was replaced by a
 * container whose children are the old ones, apart from the children listed
 * under their own paths, as when an item is pushed onto a list.
 */
export type Changes = ReadonlyMap<string, boolean>;

/** Whether `changes` can change what a binding that made `reads` saw. */
export function affects(reads: ReadonlyMap<string, boolean>, changes: Changes): boolean {
  for (const [read, deep] of reads) {
    for (const [path, inside] of changes) {
      const hit = inside
        ? deep
          ? overlaps(read, path)
          : read === path || read.startsWith(`${path}.`)
        : read === path || (deep && path.startsWith(`${read}.`));
      if (hit) return true;
    }
  }
  return false;
}

/**
 * Answers `affects` for one read at a time, in time proportional to the
 * read's depth rather than to the number of changes.
 */
export class ChangeLookup {
  readonly #changes: Changes;
  /** Every path that has a change strictly below it. */
  readonly #above = new Set<string>();

  constructor(changes: Changes) {
    this.#changes = changes;
    for (const path of changes.keys()) {
      for (let dot = path.indexOf("."); dot !== -1; dot = path.indexOf(".", dot + 1)) this.#above.add(path.slice(0, dot));
    }
  }

  /** Whether the changes can change what a read at `read`, deep or shallow, saw. */
  affects(read: string, deep: boolean): boolean {
    if (this.#changes.has(read)) return true;
    for (let dot = read.indexOf("."); dot !== -1; dot = read.indexOf(".", dot + 1)) {
      if (this.#changes.get(read.slice(0, dot)) === true) return true;
    }
    return deep && this.#above.has(read);
  }
}

/**
 * The read that `index` records for the list item at `path`. Its last
 * segment is never a field name, so it marks a read of the item's position,
 * which changes when the item moves without its value changing.
 */
export function positionOf(path: string): string {
  return `${path}.#`;
}

function isContainer(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object";
}

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
    this.#values = (copied === null ? setPath(this.#values, path, value) : setCopied(this.#values, path, value, copied)) as Values;
    if (this.#changes.size === 0) queueMicrotask(() => this.#notify());
    if (isContainer(previous) && isContainer(value) && Array.isArray(previous) === Array.isArray(value)) {
      this.#record(path, false);
      if (Array.isArray(previous) && Array.isArray(value)) {
        for (let index = 0; index < Math.max(previous.length, value.length); index++) {
          if (!Object.is(previous[index], value[index])) this.#record(join(path, String(index)), true);
        }
      } else {
        for (const key of new Set([...Object.keys(previous), ...Object.keys(value)])) {
          if (!Object.is(previous[key], value[key])) this.#record(join(path, key), true);
        }
      }
    } else {
      this.#record(path, true);
    }
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
      if (deep || !this.#reads.has(path)) this.#reads.set(path, deep || this.#reads.get(path) === true);
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
