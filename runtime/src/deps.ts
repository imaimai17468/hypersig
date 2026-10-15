import type { Reads } from "./store.ts";

/** One binding of one element. `deps` is null until it has run once. */
export interface Effect {
  /** Creation order. A list's binding is older than its items' bindings, so it runs first. */
  readonly id: number;
  readonly element: Element;
  readonly run: () => void;
  deps: Reads | null;
  nodes: PathNode[];
}

class PathNode {
  readonly children = new Map<string, PathNode>();
  readonly deep = new Set<Effect>();
  readonly shallow = new Set<Effect>();
}

/**
 * Finds the bindings a change can affect without visiting the others. Reads
 * are stored in a tree of path segments. A change at a path affects every
 * read at or below it, and the deep reads above it.
 */
export class DepIndex {
  readonly #root = new PathNode();

  /** Replaces what `effect` is registered under with `reads`. */
  set(effect: Effect, reads: Reads): void {
    if (effect.deps !== null && effect.nodes.length > 0 && same(effect.deps, reads)) {
      effect.deps = reads;
      return;
    }
    this.remove(effect);
    effect.deps = reads;
    for (const [path, deep] of reads) {
      let node = this.#root;
      for (const segment of path.split(".")) {
        let child = node.children.get(segment);
        if (child === undefined) {
          child = new PathNode();
          node.children.set(segment, child);
        }
        node = child;
      }
      (deep ? node.deep : node.shallow).add(effect);
      effect.nodes.push(node);
    }
  }

  remove(effect: Effect): void {
    for (const node of effect.nodes) {
      node.deep.delete(effect);
      node.shallow.delete(effect);
    }
    effect.nodes = [];
  }

  /** The bindings that read a path a change in `changed` can affect. */
  affected(changed: Iterable<string>): Set<Effect> {
    const found = new Set<Effect>();
    for (const path of changed) {
      let node: PathNode | undefined = this.#root;
      const segments = path.split(".");
      for (const [i, segment] of segments.entries()) {
        node = node.children.get(segment);
        if (node === undefined) break;
        if (i < segments.length - 1) for (const effect of node.deep) found.add(effect);
      }
      if (node !== undefined) collect(node, found);
    }
    return found;
  }
}

function same(a: Reads, b: Reads): boolean {
  if (a.size !== b.size) return false;
  for (const [path, deep] of a) if (b.get(path) !== deep) return false;
  return true;
}

function collect(node: PathNode, found: Set<Effect>): void {
  for (const effect of node.deep) found.add(effect);
  for (const effect of node.shallow) found.add(effect);
  for (const child of node.children.values()) collect(child, found);
}
