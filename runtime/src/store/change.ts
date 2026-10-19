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
      for (let dot = path.indexOf("."); dot !== -1; dot = path.indexOf(".", dot + 1))
        this.#above.add(path.slice(0, dot));
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

  /** Whether the changes can change what a binding that made `reads` saw. */
  affectsAny(reads: Reads): boolean {
    for (const [read, deep] of reads) if (this.affects(read, deep)) return true;
    return false;
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
