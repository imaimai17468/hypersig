import type { Store } from "./store.ts";
import { affects } from "./store/change.ts";
import { getPath, setPath } from "./store/path.ts";
import type { Values } from "./store/value.ts";

/**
 * The signals `Page::persist` names, as `data-hs-persist` on `<body>` lists
 * them: each signal's dotted path and the `localStorage` key it is kept under.
 */
export type Persisted = readonly (readonly [path: string, key: string])[];

/**
 * `initial` with each persisted signal set to the value stored under its
 * key. A stored value that does not parse, or whose JSON type differs from
 * the initial value's, is left out, since it was written by another build of
 * the page or by another program.
 */
export function restore(initial: Values, persisted: Persisted): Values {
  let values: unknown = initial;
  for (const [path, key] of persisted) {
    const stored = read(key);
    if (stored !== undefined && kind(stored) === kind(getPath(initial, path)))
      values = setPath(values, path, stored);
  }
  return values as Values;
}

/** Writes each persisted signal to its key whenever a write to the store changes it, and shows the page. */
export function keep(store: Store, persisted: Persisted): void {
  // The script `Page::persist` adds to `<head>` sets `data-hs-restoring` on
  // `<html>` to hide the page until the stored values have rendered.
  delete document.documentElement.dataset.hsRestoring;
  if (persisted.length === 0) return;
  store.subscribe((changes) => {
    for (const [path, key] of persisted) {
      if (!affects(new Map([[path, true]]), changes)) continue;
      try {
        localStorage.setItem(key, JSON.stringify(store.untracked(() => store.get(path))));
      } catch {
        // A browser that refuses storage, such as one with it turned off or
        // full, keeps the value for this page alone.
      }
    }
  });
}

function read(key: string): unknown {
  try {
    const text = localStorage.getItem(key);
    return text === null ? undefined : (JSON.parse(text) as unknown);
  } catch {
    return undefined;
  }
}

function kind(value: unknown): string {
  return value === null ? "null" : Array.isArray(value) ? "array" : typeof value;
}
