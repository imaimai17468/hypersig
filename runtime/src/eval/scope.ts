import { AstError, type Compiled, type Context, isAst, type Slot } from "../eval.ts";
import { join } from "../store/path.ts";

/** A list's store path, its items, and the name its items' reads are recorded under. */
export type Found = [path: string, items: unknown[], read: string];

/** The slot of the list item at `depth` around the expression. */
export function slotAt(ctx: Context, depth: unknown): Slot {
  const found = ctx.scope[depth as number];
  if (found === undefined) throw new Error(`hypersig: no list item at depth ${String(depth)}`);
  return found;
}

/** The store path a place names: `"a.b"` for a page signal, `["item",depth,"a"]` for an item field. */
export function resolve(place: unknown, ctx: Context): string {
  if (typeof place === "string") return place;
  if (isAst(place) && place[0] === "item")
    return join(slotAt(ctx, place[1]).path, place[2] as string);
  throw new AstError(place);
}

/** The name a read of the value at `resolve(place, ctx)` is recorded under. */
export function resolveRead(place: unknown, ctx: Context): string {
  if (typeof place === "string") return place;
  if (isAst(place) && place[0] === "item") {
    const found = slotAt(ctx, place[1]);
    return join(found.read ?? found.path, place[2] as string);
  }
  throw new AstError(place);
}

/** The path of the list at `place`, its items, and the name its items' reads are recorded under. */
export function list(ctx: Context, place: unknown): Found {
  const path = resolve(place, ctx);
  const read = resolveRead(place, ctx);
  const value = ctx.store.getShallowAs(path, read);
  return [path, Array.isArray(value) ? value : [], read];
}

/** A slot that `at` moves to each item of the list at `path`, whose items' reads are recorded under `read`. */
export function walker(path: string, read: string): [slot: Slot, at: (index: number) => void] {
  const slot: Slot = { path };
  const at = (index: number): void => {
    slot.path = join(path, String(index));
    if (read !== path) slot.read = join(read, String(index));
  };
  return [slot, at];
}

/** The value of `each` for each item of the list `found`, with one context reused for every item. */
export function mapItems(ctx: Context, depth: number, found: Found, each: Compiled): unknown[] {
  const [path, items, read] = found;
  const [slot, at] = walker(path, read);
  const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth), slot] };
  return items.map((_, index) => {
    at(index);
    return each(itemCtx);
  });
}

/** Whether `test` holds for each item of the list `found`. */
export function testItems(ctx: Context, depth: number, found: Found, test: Compiled): boolean[] {
  return mapItems(ctx, depth, found, test).map(Boolean);
}

/** The key of each item of the list `found`, as `JSON.stringify` writes it, the way a keyed list compares keys. */
export function itemKeys(ctx: Context, depth: number, found: Found, key: Compiled): string[] {
  return ctx.store.untracked(() =>
    mapItems(ctx, depth, found, key).map((value) => JSON.stringify(value)),
  );
}
