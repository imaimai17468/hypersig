import { type Context, compile, evaluate, type Statement } from "../../eval.ts";
import { Store } from "../../store.ts";
import { join, setPath } from "../../store/path.ts";
import type { Values } from "../../store/value.ts";
import { itemKeys, list, mapItems, testItems, walker } from "../scope.ts";

/** Puts each of `added` in place of the item of `items` whose key it has, or after the last item, by `keys` of both. */
function merged(
  items: readonly unknown[],
  added: readonly unknown[],
  keys: readonly string[],
): unknown[] {
  const next = [...items];
  const at = new Map<string, number>();
  items.forEach((_, index) => {
    if (!at.has(keys[index]!)) at.set(keys[index]!, index);
  });
  added.forEach((item, offset) => {
    const key = keys[items.length + offset]!;
    const index = at.get(key);
    if (index === undefined) {
      at.set(key, next.length);
      next.push(item);
    } else next[index] = item;
  });
  return next;
}

/** The statements that evaluate an expression for each item of a list to filter, change, order, or key its items. */
export const eaches: Record<string, Statement> = {
  retain: ([place, depth, testAst], ctx) => {
    const found = list(ctx, place);
    const [path, items] = found;
    const keep = testItems(ctx, depth as number, found, compile(testAst));
    const kept = items.filter((_, i) => keep[i]);
    if (kept.length !== items.length) ctx.store.set(path, kept);
  },
  update: ([place, depth, whenAst, field, valueAst], ctx) => {
    const found = list(ctx, place);
    const [path, items, read] = found;
    const when = testItems(ctx, depth as number, found, compile(whenAst));
    const value = compile(valueAst);
    const [slot, at] = walker(path, read);
    const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth as number), slot] };
    // Each selected item's field is written at its own path, so only the bindings that read that field run.
    const writes: [string, unknown][] = [];
    items.forEach((_, index) => {
      if (!when[index]) return;
      at(index);
      writes.push([join(join(path, String(index)), field as string), value(itemCtx)]);
    });
    ctx.store.setMany(writes);
  },
  sort: ([place, depth, keyAst, descending], ctx) => {
    const found = list(ctx, place);
    const [path, items] = found;
    const keys = mapItems(ctx, depth as number, found, compile(keyAst)) as (number | string)[];
    const sign = descending === true ? -1 : 1;
    const order = items
      .map((_, index) => index)
      .toSorted((a, b) => (keys[a]! < keys[b]! ? -sign : keys[a]! > keys[b]! ? sign : a - b));
    if (order.some((from, index) => from !== index))
      ctx.store.set(
        path,
        order.map((from) => items[from]),
      );
  },
  set_all: ([place, field, valueAst], ctx) => {
    const [path, items] = list(ctx, place);
    const value = evaluate(valueAst, ctx);
    ctx.store.setMany(items.map((_, i) => [join(join(path, String(i)), field as string), value]));
  },
  // `["upsert", place, depth, key, items]`: an item replaces the held item with its key, or goes at the end.
  upsert: ([place, depth, keyAst, addedAst], ctx) => {
    const [path, items, read] = list(ctx, place);
    const added = evaluate(addedAst, ctx);
    if (!Array.isArray(added) || added.length === 0) return;
    // The key may read the item and the page's signals, so the added items are keyed in a copy of the store that holds them after the list's items.
    const all = [...items, ...(added as unknown[])];
    const copy = new Store(
      setPath(
        ctx.store.untracked(() => ctx.store.get("")),
        path,
        all,
      ) as Values,
    );
    const keys = itemKeys(
      { ...ctx, store: copy },
      depth as number,
      [path, all, read],
      compile(keyAst),
    );
    ctx.store.set(path, merged(items, added as unknown[], keys));
  },
  // `["remove_keys", place, depth, key, keys]`: the keys arrive as JSON values, and are compared as `JSON.stringify` writes them.
  remove_keys: ([place, depth, keyAst, removedAst], ctx) => {
    const found = list(ctx, place);
    const [path, items] = found;
    const removed = evaluate(removedAst, ctx);
    if (!Array.isArray(removed) || removed.length === 0) return;
    const gone = new Set(removed.map((key: unknown) => JSON.stringify(key)));
    const keys = itemKeys(ctx, depth as number, found, compile(keyAst));
    const kept = items.filter((_, index) => !gone.has(keys[index]!));
    if (kept.length !== items.length) ctx.store.set(path, kept);
  },
};
