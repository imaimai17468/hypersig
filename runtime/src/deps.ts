import type { Changes, Reads } from "./store/change.ts";
import { PathNode } from "./deps/node.ts";
import { Selections } from "./deps/select.ts";

/** One binding of one element. `deps` is null until it has run once. */
export interface Effect {
  /** Creation order. A list's binding is older than its items' bindings, so it runs first. */
  readonly id: number;
  readonly element: Element;
  readonly run: () => void;
  deps: Reads | null;
  /** The tree node each read is registered under, by path. */
  nodes: Map<string, PathNode>;
  /** The comparison it registered with `DepIndex.select` in place of a read, if any. */
  selector: Selector | null;
}

/**
 * A binding's comparison of the value at `path` with a value whose key is
 * `key`, made without reading `path`. `current` is the key of the value at
 * `path` that the binding compared against. A key is a value's JSON.
 */
export interface Selector {
  readonly path: string;
  readonly key: string;
  readonly current: string;
}

/**
 * Finds the bindings a change can affect without visiting the others. Reads
 * are stored in a tree of path segments. A change at a path affects the deep
 * reads above it and the reads at it, and also the reads below it unless only
 * the container at the path was replaced.
 */
export class DepIndex {
  readonly #root = new PathNode(null, "", "");
  /** The node at each path that has one, so registering a read does not walk the tree. */
  readonly #nodes = new Map<string, PathNode>();
  readonly #selections = new Selections();

  /** Replaces what `effect` is registered under with `reads`, touching only the reads that differ. */
  set(effect: Effect, reads: Reads): void {
    const old = effect.deps;
    effect.deps = reads;
    if (old !== null) {
      for (const [path, deep] of old) {
        if (reads.get(path) === deep) continue;
        const node = effect.nodes.get(path);
        if (node === undefined) continue;
        this.#leave(node, effect);
        effect.nodes.delete(path);
      }
    }
    for (const [path, deep] of reads) {
      if (old?.get(path) === deep && effect.nodes.has(path)) continue;
      const node = this.#node(path);
      (deep ? node.shallow : node.deep)?.delete(effect);
      node.enter(effect, deep);
      effect.nodes.set(path, node);
    }
  }

  /** Calls `found` with each binding that read `path` or a path below it. */
  within(path: string, found: (effect: Effect) => void): void {
    const node = this.#nodes.get(path);
    node?.visit(found);
  }

  /** Calls `found` with each binding that read exactly `path`. */
  at(path: string, found: (effect: Effect) => void): void {
    this.#nodes.get(path)?.each(found);
  }

  remove(effect: Effect): void {
    for (const node of effect.nodes.values()) this.#leave(node, effect);
    effect.nodes = new Map();
    this.select(effect, null);
  }

  /** Registers the comparison `selector` of `effect`, as `Selections.select` does. */
  select(effect: Effect, selector: Selector | null): void {
    this.#selections.select(effect, selector);
  }

  /** Adds to `found` the bindings whose comparison can have changed, as `Selections.selected` does. */
  selected(
    touched: (path: string) => boolean,
    keyAt: (path: string) => string,
    found: Set<Effect>,
  ): void {
    this.#selections.selected(touched, keyAt, found);
  }

  #node(path: string): PathNode {
    let node = this.#nodes.get(path);
    if (node !== undefined) return node;
    const dot = path.lastIndexOf(".");
    const parent = dot === -1 ? this.#root : this.#node(path.slice(0, dot));
    node = new PathNode(parent, path, path.slice(dot + 1));
    (parent.children ??= new Map()).set(node.segment, node);
    this.#nodes.set(path, node);
    return node;
  }

  /** Takes `effect` off `node`, and drops the nodes that are left holding nothing. */
  #leave(node: PathNode, effect: Effect): void {
    node.leave(effect);
    this.#drop(node);
  }

  /** Drops `node` and the nodes above it while they hold nothing. */
  #drop(node: PathNode): void {
    for (let at = node; at.parent !== null && at.empty(); at = at.parent) {
      // A node dropped earlier stays out of the tree, and one put back since is a different node.
      if (this.#nodes.get(at.path) !== at) return;
      at.parent.children!.delete(at.segment);
      this.#nodes.delete(at.path);
    }
  }

  /** The bindings whose reads `changes` can affect, as `affects` decides. */
  affected(changes: Changes): Set<Effect> {
    const found = new Set<Effect>();
    const add = (effect: Effect): void => {
      found.add(effect);
    };
    for (const [path, inside] of changes) {
      let node: PathNode | undefined = this.#root;
      // Walks the segments by their dots, because a list write records one path per item and splitting each would allocate an array.
      for (let start = 0; ;) {
        const dot = path.indexOf(".", start);
        node = node.children?.get(dot === -1 ? path.slice(start) : path.slice(start, dot));
        if (node === undefined || dot === -1) break;
        for (const effect of node.deep ?? []) found.add(effect);
        start = dot + 1;
      }
      if (node === undefined) continue;
      if (inside) node.visit(add);
      else node.each(add);
    }
    return found;
  }
}
