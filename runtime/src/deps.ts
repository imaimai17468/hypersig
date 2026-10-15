import type { Changes, Reads } from "./store.ts";

/** One binding of one element. `deps` is null until it has run once. */
export interface Effect {
  /** Creation order. A list's binding is older than its items' bindings, so it runs first. */
  readonly id: number;
  readonly element: Element;
  readonly run: () => void;
  deps: Reads | null;
  /** The tree node each read is registered under, by path. */
  nodes: Map<string, PathNode>;
}

class PathNode {
  readonly children = new Map<string, PathNode>();
  readonly deep = new Set<Effect>();
  readonly shallow = new Set<Effect>();
}

/**
 * Finds the bindings a change can affect without visiting the others. Reads
 * are stored in a tree of path segments. A change at a path affects the deep
 * reads above it and the reads at it, and also the reads below it unless only
 * the container at the path was replaced.
 */
export class DepIndex {
  readonly #root = new PathNode();

  /** Replaces what `effect` is registered under with `reads`, touching only the reads that differ. */
  set(effect: Effect, reads: Reads): void {
    const old = effect.deps;
    effect.deps = reads;
    if (old !== null) {
      for (const [path, deep] of old) {
        if (reads.get(path) === deep) continue;
        const node = effect.nodes.get(path);
        if (node === undefined) continue;
        node.deep.delete(effect);
        node.shallow.delete(effect);
        effect.nodes.delete(path);
      }
    }
    for (const [path, deep] of reads) {
      if (old?.get(path) === deep && effect.nodes.has(path)) continue;
      const node = this.#node(path);
      (deep ? node.shallow : node.deep).delete(effect);
      (deep ? node.deep : node.shallow).add(effect);
      effect.nodes.set(path, node);
    }
  }

  remove(effect: Effect): void {
    for (const node of effect.nodes.values()) {
      node.deep.delete(effect);
      node.shallow.delete(effect);
    }
    effect.nodes = new Map();
  }

  #node(path: string): PathNode {
    let node = this.#root;
    for (const segment of path.split(".")) {
      let child = node.children.get(segment);
      if (child === undefined) {
        child = new PathNode();
        node.children.set(segment, child);
      }
      node = child;
    }
    return node;
  }

  /** The bindings whose reads `changes` can affect, as `affects` decides. */
  affected(changes: Changes): Set<Effect> {
    const found = new Set<Effect>();
    for (const [path, inside] of changes) {
      let node: PathNode | undefined = this.#root;
      const segments = path.split(".");
      for (const [i, segment] of segments.entries()) {
        node = node.children.get(segment);
        if (node === undefined) break;
        if (i < segments.length - 1) for (const effect of node.deep) found.add(effect);
      }
      if (node === undefined) continue;
      if (inside) {
        collect(node, found);
      } else {
        for (const effect of node.deep) found.add(effect);
        for (const effect of node.shallow) found.add(effect);
      }
    }
    return found;
  }
}

function collect(node: PathNode, found: Set<Effect>): void {
  for (const effect of node.deep) found.add(effect);
  for (const effect of node.shallow) found.add(effect);
  for (const child of node.children.values()) collect(child, found);
}
