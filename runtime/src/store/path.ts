import type { Values } from "./value.ts";

/** Joins two dotted paths, skipping an empty one. */
export function join(a: string, b: string): string {
  return a === "" ? b : b === "" ? a : `${a}.${b}`;
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
  const dot = path.indexOf(".");
  if (dot === -1)
    return value !== null && typeof value === "object"
      ? (value as Record<string, unknown>)[path]
      : undefined;
  // An item's own path, such as `rows.12`, has two segments and is new for each item, so it is split here without being remembered.
  if (path.indexOf(".", dot + 1) === -1) {
    const head =
      value !== null && typeof value === "object"
        ? (value as Record<string, unknown>)[path.slice(0, dot)]
        : undefined;
    return head !== null && typeof head === "object"
      ? (head as Record<string, unknown>)[path.slice(dot + 1)]
      : undefined;
  }
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

/** `setPath` for a path already split into its segments, for a caller that writes at one path many times. */
export function setSegments(value: unknown, segments: readonly string[], next: unknown): unknown {
  return setAt(value, segments, 0, next);
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
export function setCopied(
  root: unknown,
  path: string,
  next: unknown,
  copied: Set<unknown>,
): unknown {
  if (path === "") return next;
  const own = (value: unknown): Record<string, unknown> => {
    if (copied.has(value)) return value as Record<string, unknown>;
    const copy = Array.isArray(value)
      ? [...value]
      : value !== null && typeof value === "object"
        ? { ...(value as Values) }
        : {};
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
