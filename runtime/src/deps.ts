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

class Selection {
  readonly byKey = new Map<string, Set<Effect>>();
  /** The keys of the values at the path that registered bindings compared against since the last `selected`. */
  seen = new Set<string>();
}

/** A segment of the paths bindings read. Its collections are made on first use, because most nodes hold one binding or none. */
class PathNode {
  children: Map<string, PathNode> | null = null;
  deep: Set<Effect> | null = null;
  shallow: Set<Effect> | null = null;

  constructor(
    readonly parent: PathNode | null,
    readonly path: string,
    readonly segment: string,
  ) {}
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
  readonly #selections = new Map<string, Selection>();

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
      enter(node, effect, deep);
      effect.nodes.set(path, node);
    }
  }

  /** Calls `found` with each binding that read `path` or a path below it. */
  within(path: string, found: (effect: Effect) => void): void {
    const node = this.#nodes.get(path);
    if (node !== undefined) visit(node, found);
  }

  /** Calls `found` with each binding that read exactly `path`. */
  at(path: string, found: (effect: Effect) => void): void {
    const node = this.#nodes.get(path);
    for (const effect of node?.deep ?? []) found(effect);
    for (const effect of node?.shallow ?? []) found(effect);
  }

  remove(effect: Effect): void {
    for (const node of effect.nodes.values()) this.#leave(node, effect);
    effect.nodes = new Map();
    this.select(effect, null);
  }

  /**
   * Registers `effect` to run when the value at `selector.path` becomes, or
   * stops being, the value whose key is `selector.key`, replacing what it
   * registered before.
   */
  select(effect: Effect, selector: Selector | null): void {
    const old = effect.selector;
    if (old !== null) {
      const selection = this.#selections.get(old.path);
      const effects = selection?.byKey.get(old.key);
      effects?.delete(effect);
      if (effects?.size === 0) selection!.byKey.delete(old.key);
      if (selection?.byKey.size === 0) this.#selections.delete(old.path);
    }
    effect.selector = selector;
    if (selector === null) return;
    let selection = this.#selections.get(selector.path);
    if (selection === undefined) {
      selection = new Selection();
      this.#selections.set(selector.path, selection);
    }
    let effects = selection.byKey.get(selector.key);
    if (effects === undefined) {
      effects = new Set();
      selection.byKey.set(selector.key, effects);
    }
    effects.add(effect);
    selection.seen.add(selector.current);
  }

  /**
   * Adds to `found` the bindings whose comparison can have changed: for each
   * path that `touched` says a change reached, the ones registered under a
   * key seen there before or under `keyAt(path)`, the key of its value now.
   */
  selected(touched: (path: string) => boolean, keyAt: (path: string) => string, found: Set<Effect>): void {
    for (const [path, selection] of this.#selections) {
      if (!touched(path)) continue;
      const now = keyAt(path);
      for (const key of [...selection.seen, now]) {
        for (const effect of selection.byKey.get(key) ?? []) found.add(effect);
      }
      selection.seen = new Set([now]);
    }
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
    node.deep?.delete(effect);
    node.shallow?.delete(effect);
    this.#drop(node);
  }

  /** Drops `node` and the nodes above it while they hold nothing. */
  #drop(node: PathNode): void {
    for (let at = node; at.parent !== null && (at.deep?.size ?? 0) + (at.shallow?.size ?? 0) + (at.children?.size ?? 0) === 0; at = at.parent) {
      // A node dropped earlier stays out of the tree, and one put back since is a different node.
      if (this.#nodes.get(at.path) !== at) return;
      at.parent.children!.delete(at.segment);
      this.#nodes.delete(at.path);
    }
  }

  /** The bindings whose reads `changes` can affect, as `affects` decides. */
  affected(changes: Changes): Set<Effect> {
    const found = new Set<Effect>();
    for (const [path, inside] of changes) {
      let node: PathNode | undefined = this.#root;
      const segments = path.split(".");
      for (const [i, segment] of segments.entries()) {
        node = node.children?.get(segment);
        if (node === undefined) break;
        if (i < segments.length - 1) for (const effect of node.deep ?? []) found.add(effect);
      }
      if (node === undefined) continue;
      if (inside) {
        collect(node, found);
      } else {
        for (const effect of node.deep ?? []) found.add(effect);
        for (const effect of node.shallow ?? []) found.add(effect);
      }
    }
    return found;
  }
}

function collect(node: PathNode, found: Set<Effect>): void {
  visit(node, (effect) => found.add(effect));
}

function visit(node: PathNode, found: (effect: Effect) => void): void {
  for (const effect of node.deep ?? []) found(effect);
  for (const effect of node.shallow ?? []) found(effect);
  for (const child of node.children?.values() ?? []) visit(child, found);
}

function enter(node: PathNode, effect: Effect, deep: boolean): void {
  if (deep) (node.deep ??= new Set()).add(effect);
  else (node.shallow ??= new Set()).add(effect);
}
