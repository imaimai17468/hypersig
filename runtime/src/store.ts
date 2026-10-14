export type Values = Record<string, unknown>;

/** The page's signals, keyed by field name, with batched change notification. */
export class Store {
  #values: Values;
  #listeners = new Set<() => void>();
  #scheduled = false;

  constructor(initial: Values) {
    this.#values = { ...initial };
  }

  get(path: string): unknown {
    return this.#values[path];
  }

  set(path: string, value: unknown): void {
    if (Object.is(this.#values[path], value)) return;
    this.#values[path] = value;
    this.#schedule();
  }

  merge(values: Values): void {
    for (const [path, value] of Object.entries(values)) this.set(path, value);
  }

  snapshot(): Values {
    return { ...this.#values };
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
