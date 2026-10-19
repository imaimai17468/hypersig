import type { Effect } from "../deps.ts";

/** A segment of the paths bindings read. Its collections are made on first use, because most nodes hold one binding or none. */
export class PathNode {
  children: Map<string, PathNode> | null = null;
  deep: Set<Effect> | null = null;
  shallow: Set<Effect> | null = null;
  readonly parent: PathNode | null;
  readonly path: string;
  readonly segment: string;

  constructor(parent: PathNode | null, path: string, segment: string) {
    this.parent = parent;
    this.path = path;
    this.segment = segment;
  }

  /** Files `effect` here, as a deep or a shallow read. */
  enter(effect: Effect, deep: boolean): void {
    if (deep) (this.deep ??= new Set()).add(effect);
    else (this.shallow ??= new Set()).add(effect);
  }

  /** Takes `effect` off this node. */
  leave(effect: Effect): void {
    this.deep?.delete(effect);
    this.shallow?.delete(effect);
  }

  /** Whether the node holds no binding and no child. */
  empty(): boolean {
    return (this.deep?.size ?? 0) + (this.shallow?.size ?? 0) + (this.children?.size ?? 0) === 0;
  }

  /** Calls `found` with each binding that read exactly this path. */
  each(found: (effect: Effect) => void): void {
    for (const effect of this.deep ?? []) found(effect);
    for (const effect of this.shallow ?? []) found(effect);
  }

  /** Calls `found` with each binding that read this path or a path below it. */
  visit(found: (effect: Effect) => void): void {
    this.each(found);
    for (const child of this.children?.values() ?? []) child.visit(found);
  }
}
