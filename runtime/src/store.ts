export type Values = Record<string, unknown>;

/** Joins dotted path parts, skipping empty ones. */
export function join(...parts: string[]): string {
  return parts.filter((part) => part !== "").join(".");
}

/** Reads the value at a dotted path, where a numeric segment indexes an array. */
export function getPath(value: unknown, path: string): unknown {
  if (path === "") return value;
  let current = value;
  for (const segment of path.split(".")) {
    if (current === null || typeof current !== "object") return undefined;
    current = (current as Record<string, unknown>)[segment];
  }
  return current;
}

/** Returns a copy of `value` with the value at `path` replaced, copying each container on the way. */
export function setPath(value: unknown, path: string, next: unknown): unknown {
  if (path === "") return next;
  const [head, ...rest] = path.split(".") as [string, ...string[]];
  const tail = rest.join(".");
  if (Array.isArray(value)) {
    const copy = [...value];
    copy[Number(head)] = setPath(copy[Number(head)], tail, next);
    return copy;
  }
  const record = value !== null && typeof value === "object" ? (value as Values) : {};
  return { ...record, [head]: setPath(record[head], tail, next) };
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
 * The page's signals as one JSON object. Reads made inside `track` are
 * recorded, and listeners receive the paths changed since they last ran.
 */
export class Store {
  #values: Values;
  #listeners = new Set<(changed: ReadonlySet<string>) => void>();
  #changed = new Set<string>();
  #reads: Set<string> | null = null;

  constructor(initial: Values) {
    this.#values = { ...initial };
  }

  get(path: string): unknown {
    this.#reads?.add(path);
    return getPath(this.#values, path);
  }

  set(path: string, value: unknown): void {
    if (Object.is(getPath(this.#values, path), value)) return;
    this.#values = setPath(this.#values, path, value) as Values;
    if (this.#changed.size === 0) queueMicrotask(() => this.#notify());
    this.#changed.add(path);
  }

  merge(values: Values): void {
    for (const [path, value] of Object.entries(values)) this.set(path, value);
  }

  snapshot(): Values {
    return this.#values;
  }

  /** Runs `body` and returns the paths it read. */
  track(body: () => void): Set<string> {
    const outer = this.#reads;
    const reads = new Set<string>();
    this.#reads = reads;
    try {
      body();
    } finally {
      this.#reads = outer;
    }
    return reads;
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

  subscribe(listener: (changed: ReadonlySet<string>) => void): void {
    this.#listeners.add(listener);
  }

  #notify(): void {
    const changed = this.#changed;
    this.#changed = new Set();
    for (const listener of this.#listeners) listener(changed);
  }
}
