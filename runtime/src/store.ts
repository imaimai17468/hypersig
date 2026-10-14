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

/** The page's signals as one JSON object, with batched change notification. */
export class Store {
  #values: Values;
  #listeners = new Set<() => void>();
  #scheduled = false;

  constructor(initial: Values) {
    this.#values = { ...initial };
  }

  get(path: string): unknown {
    return getPath(this.#values, path);
  }

  set(path: string, value: unknown): void {
    if (Object.is(this.get(path), value)) return;
    this.#values = setPath(this.#values, path, value) as Values;
    this.#schedule();
  }

  merge(values: Values): void {
    for (const [path, value] of Object.entries(values)) this.set(path, value);
  }

  snapshot(): Values {
    return this.#values;
  }

  subscribe(listener: () => void): void {
    this.#listeners.add(listener);
  }

  #schedule(): void {
    if (this.#scheduled) return;
    this.#scheduled = true;
    queueMicrotask(() => {
      this.#scheduled = false;
      for (const listener of this.#listeners) listener();
    });
  }
}
