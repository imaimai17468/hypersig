import { equal, getPath, join, setPath, Store, type Values } from "./store.ts";

/**
 * The JSON form of an expression or statement, as `crates/hypersig/src/expr.rs` writes it.
 * The first element names the operation.
 */
export type Ast = readonly unknown[];

/** The store path of one list item, updated in place when the item moves. */
export interface Slot {
  path: string;
  /**
   * The name that reads through this slot are recorded under, where it is not
   * `path`. The runtime gives each list item it renders a name of its own, so
   * the reads of the item's bindings stay valid when the item moves.
   */
  read?: string;
}

/** What an expression is evaluated against: the store, and one slot per enclosing list. */
export interface Context {
  readonly store: Store;
  readonly scope: readonly Slot[];
  /** The element whose attribute is being evaluated, for statements that act on the DOM. */
  readonly origin?: Element;
  /** The values `["let",...]` names, by depth. */
  readonly vars?: readonly unknown[];
}

/** What a call asks for beyond sending its request once. */
export interface CallOptions {
  /** Abort the unfinished earlier call of the same action that was also sent with `latest`. */
  readonly latest: boolean;
  /** Send the request again after the connection fails or the server answers with a 5xx status. */
  readonly reconnect: boolean;
  /** The element whose statement made the call. Reconnecting stops once it leaves the document. */
  readonly origin: Element | undefined;
  /** The name of the file input whose files go with the arguments, for an upload. */
  readonly files?: string;
}

/** The side effects a statement can ask the runtime for. */
export interface Host {
  call(
    method: string,
    path: string,
    args: unknown,
    indicator: string | null,
    error: string | null,
    options: CallOptions,
  ): Promise<void>;
  dom(op: string, name: string, origin: Element | undefined): void;
  clearFiles(name: string, origin: Element | undefined): void;
  navigate(url: string): void;
  history(url: string, replace: boolean): void;
  debounce(origin: Element | undefined, key: string, ms: number, action: () => void): void;
}

const binary: Record<string, (a: never, b: never) => unknown> = {
  add: (a: number, b: number) => a + b,
  sub: (a: number, b: number) => a - b,
  mul: (a: number, b: number) => a * b,
  div: (a: number, b: number) => a / b,
  rem: (a: number, b: number) => a % b,
  min: (a: number, b: number) => Math.min(a, b),
  max: (a: number, b: number) => Math.max(a, b),
  pow: (a: number, b: number) => a ** b,
  concat: (a: string, b: string) => a + b,
  contains: (a: string, b: string) => a.includes(b),
  starts: (a: string, b: string) => a.startsWith(b),
  eq: (a: unknown, b: unknown) => equal(a, b),
  ne: (a: unknown, b: unknown) => !equal(a, b),
  gt: (a: number, b: number) => a > b,
  ge: (a: number, b: number) => a >= b,
  lt: (a: number, b: number) => a < b,
  le: (a: number, b: number) => a <= b,
  or_else: (a: unknown, b: unknown) => a ?? b,
};

/**
 * The query string of a search struct, as `query_string` in `crates/hypersig/src/route.rs` writes it:
 * fields in the order of their names, `null` ones left out, each name and value percent-encoded.
 */
function queryString(value: unknown): string {
  if (value === null || typeof value !== "object") return "";
  const fields = value as Record<string, unknown>;
  const pairs = Object.keys(fields)
    .sort()
    .filter((name) => fields[name] !== null && fields[name] !== undefined)
    .map((name) => `${encodeURIComponent(name)}=${encodeURIComponent(String(fields[name]))}`);
  return pairs.length === 0 ? "" : `?${pairs.join("&")}`;
}

/**
 * A number written as `format_number` in `crates/hypersig/src/ssr.rs` writes it: rounded by `toFixed`,
 * its whole part grouped by threes, a minus sign only where a digit is not zero, then the affixes.
 */
function formatNumber(value: number, digits: number, group: string, point: string, prefix: string, suffix: string): string {
  if (!Number.isFinite(value)) return `${prefix}${String(value)}${suffix}`;
  const fixed = Math.abs(value).toFixed(digits);
  const sign = value < 0 && /[1-9]/.test(fixed) ? "-" : "";
  if (fixed.includes("e")) return `${sign}${prefix}${fixed}${suffix}`;
  const [whole = "", fraction = ""] = fixed.split(".");
  const grouped = whole.replace(/\B(?=(\d{3})+$)/g, () => group);
  return `${sign}${prefix}${grouped}${fraction === "" ? "" : point + fraction}${suffix}`;
}

const unary: Record<string, (a: never) => unknown> = {
  not: (a: unknown) => !a,
  str: (a: unknown) => String(a),
  uri: (a: unknown) => encodeURIComponent(String(a)),
  uri_path: (a: unknown) => String(a).split("/").map(encodeURIComponent).join("/"),
  query: queryString,
  trim: (a: string) => a.trim(),
  lower: (a: string) => a.toLowerCase(),
  upper: (a: string) => a.toUpperCase(),
  chars: (a: string) => [...a].length,
  len: (a: unknown) => (Array.isArray(a) ? a.length : 0),
  floor: (a: number) => Math.floor(a),
  ceil: (a: number) => Math.ceil(a),
  round: (a: number) => Math.round(a),
  log10: (a: number) => Math.log10(a),
};

/** Whether `op` names an operation that `evaluate` builds through `unary` or `binary`. */
export function isOperator(op: string, arity: number): boolean {
  return (arity === 1 && op in unary) || (arity === 2 && op in binary);
}

export class AstError extends Error {
  constructor(ast: unknown) {
    super(`hypersig: unknown expression ${JSON.stringify(ast)}`);
    this.name = "AstError";
  }
}

function isAst(value: unknown): value is Ast {
  return Array.isArray(value) && typeof value[0] === "string";
}

function slot(ctx: Context, depth: unknown): Slot {
  const found = ctx.scope[depth as number];
  if (found === undefined) throw new Error(`hypersig: no list item at depth ${String(depth)}`);
  return found;
}

/** The store path a place names: `"a.b"` for a page signal, `["item",depth,"a"]` for an item field. */
export function resolve(place: unknown, ctx: Context): string {
  if (typeof place === "string") return place;
  if (isAst(place) && place[0] === "item") return join(slot(ctx, place[1]).path, place[2] as string);
  throw new AstError(place);
}

/** The name a read of the value at `resolve(place, ctx)` is recorded under. */
export function resolveRead(place: unknown, ctx: Context): string {
  if (typeof place === "string") return place;
  if (isAst(place) && place[0] === "item") {
    const found = slot(ctx, place[1]);
    return join(found.read ?? found.path, place[2] as string);
  }
  throw new AstError(place);
}

/** The path of the list at `place`, its items, and the name its items' reads are recorded under. */
function list(ctx: Context, place: unknown): [path: string, items: unknown[], read: string] {
  const path = resolve(place, ctx);
  const read = resolveRead(place, ctx);
  const value = ctx.store.getShallowAs(path, read);
  return [path, Array.isArray(value) ? value : [], read];
}

/** A slot that `at` moves to each item of the list at `path`, whose items' reads are recorded under `read`. */
function walker(path: string, read: string): [slot: Slot, at: (index: number) => void] {
  const slot: Slot = { path };
  const at = (index: number): void => {
    slot.path = join(path, String(index));
    if (read !== path) slot.read = join(read, String(index));
  };
  return [slot, at];
}

/** Whether `test` holds for each item of the list at `path`, with one context reused for every item. */
function testItems(ctx: Context, depth: number, [path, items, read]: [string, readonly unknown[], string], test: Compiled): boolean[] {
  const [slot, at] = walker(path, read);
  const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth), slot] };
  return items.map((_, index) => {
    at(index);
    return Boolean(test(itemCtx));
  });
}

/** The key of each item of the list at `path`, as `JSON.stringify` writes it, the way a keyed list compares keys. */
function itemKeys(ctx: Context, depth: number, found: [string, readonly unknown[], string], key: Compiled): string[] {
  return ctx.store.untracked(() => mapItems(ctx, depth, found, key).map((value) => JSON.stringify(value)));
}

/** The value of `each` for each item of the list at `path`, with one context reused for every item. */
function mapItems(ctx: Context, depth: number, [path, items, read]: [string, readonly unknown[], string], each: Compiled): unknown[] {
  const [slot, at] = walker(path, read);
  const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth), slot] };
  return items.map((_, index) => {
    at(index);
    return each(itemCtx);
  });
}

/** How `sum_of`, `max_of`, and `min_of` fold the values of a list, as `fold` in `ssr.rs` does. */
const folds: Record<string, (values: number[]) => number | null> = {
  sum_of: (values) => values.reduce((sum, value) => sum + value, 0),
  max_of: (values) => (values.length === 0 ? null : values.reduce((max, value) => Math.max(max, value))),
  min_of: (values) => (values.length === 0 ? null : values.reduce((min, value) => Math.min(min, value))),
};

export type Compiled = (ctx: Context) => unknown;

const compiled = new WeakMap<Ast, Compiled>();

/** Evaluates an expression, building it into a function the first time this AST is seen. */
export function evaluate(ast: unknown, ctx: Context): unknown {
  return compile(ast)(ctx);
}

/** The function `evaluate` runs for `ast`, for a caller that evaluates one AST many times. */
export function compile(ast: unknown): Compiled {
  if (!isAst(ast)) throw new AstError(ast);
  let fn = compiled.get(ast);
  if (fn === undefined) {
    fn = build(ast);
    compiled.set(ast, fn);
  }
  return fn;
}

function build(ast: Ast): Compiled {
  const [op, ...rest] = ast;
  switch (op) {
    case "sig": {
      const path = rest[0] as string;
      return (ctx) => ctx.store.get(path);
    }
    case "item": {
      const [depth, field] = rest as [number, string];
      return (ctx) => {
        const found = slot(ctx, depth);
        return ctx.store.getIn(found.path, field, found.read ?? found.path);
      };
    }
    case "index": {
      const depth = rest[0];
      return (ctx) => {
        const found = slot(ctx, depth);
        ctx.store.getPosition(found.read ?? found.path);
        return Number(found.path.slice(found.path.lastIndexOf(".") + 1));
      };
    }
    case "let": {
      const [value, depth, body] = [compile(rest[0]), rest[1] as number, compile(rest[2])];
      return (ctx) => body({ ...ctx, vars: [...(ctx.vars ?? []).slice(0, depth), value(ctx)] });
    }
    case "var": {
      const depth = rest[0] as number;
      return (ctx) => ctx.vars?.[depth] ?? null;
    }
    case "lit": {
      const value = rest[0];
      return () => value;
    }
    case "random": {
      const bound = compile(rest[0]);
      return (ctx) => Math.floor(Math.random() * (bound(ctx) as number));
    }
    case "now":
      return () => Date.now();
    case "nth": {
      const [list, index] = [compile(rest[0]), compile(rest[1])];
      return (ctx) => {
        const items = list(ctx);
        return Array.isArray(items) ? (items[index(ctx) as number] ?? null) : null;
      };
    }
    case "tuple": {
      const parts = rest.map(compile);
      return (ctx) => parts.map((part) => part(ctx));
    }
    case "and": {
      const [a, b] = [compile(rest[0]), compile(rest[1])];
      return (ctx) => Boolean(a(ctx)) && Boolean(b(ctx));
    }
    case "or": {
      const [a, b] = [compile(rest[0]), compile(rest[1])];
      return (ctx) => Boolean(a(ctx)) || Boolean(b(ctx));
    }
    case "if": {
      const [cond, then, otherwise] = [compile(rest[0]), compile(rest[1]), compile(rest[2])];
      return (ctx) => (cond(ctx) ? then(ctx) : otherwise(ctx));
    }
    case "fixed": {
      const [value, digits] = [compile(rest[0]), rest[1] as number];
      return (ctx) => (value(ctx) as number).toFixed(digits);
    }
    case "format": {
      const value = compile(rest[0]);
      const [digits, group, point, prefix, suffix] = rest.slice(1) as [number, string, string, string, string];
      return (ctx) => formatNumber(value(ctx) as number, digits, group, point, prefix, suffix);
    }
    case "get": {
      const [value, path] = [compile(rest[0]), rest[1] as string];
      return (ctx) => getPath(value(ctx), path);
    }
    case "with": {
      const [value, path, next] = [compile(rest[0]), rest[1] as string, compile(rest[2])];
      return (ctx) => setPath(value(ctx), path, next(ctx));
    }
    case "count": {
      const [place, depth, test] = [rest[0], rest[1] as number, compile(rest[2])];
      return (ctx) => {
        const found = list(ctx, place);
        return ctx.store.fold(found[2], () => testItems(ctx, depth, found, test)).filter(Boolean).length;
      };
    }
    case "map": {
      const [place, depth, each] = [rest[0], rest[1] as number, compile(rest[2])];
      return (ctx) => {
        const found = list(ctx, place);
        return ctx.store.fold(found[2], () => mapItems(ctx, depth, found, each));
      };
    }
    case "sum_of":
    case "max_of":
    case "min_of": {
      const [place, depth, each, combine] = [rest[0], rest[1] as number, compile(rest[2]), folds[op]!];
      return (ctx) => {
        const found = list(ctx, place);
        return combine(ctx.store.fold(found[2], () => mapItems(ctx, depth, found, each)) as number[]);
      };
    }
    default: {
      const one = unary[op as string];
      if (one !== undefined && rest.length === 1) {
        const a = compile(rest[0]);
        return (ctx) => one(a(ctx) as never);
      }
      const two = binary[op as string];
      if (two === undefined || rest.length !== 2) throw new AstError(ast);
      const [a, b] = [compile(rest[0]), compile(rest[1])];
      return (ctx) => two(a(ctx) as never, b(ctx) as never);
    }
  }
}

export async function run(ast: unknown, ctx: Context, host: Host): Promise<void> {
  if (!isAst(ast)) throw new AstError(ast);
  const [op, ...rest] = ast;
  const store = ctx.store;
  switch (op) {
    case "set":
      store.set(resolve(rest[0], ctx), evaluate(rest[1], ctx));
      return;
    case "push": {
      const [path, items] = list(ctx, rest[0]);
      store.set(path, [...items, evaluate(rest[1], ctx)]);
      return;
    }
    case "append": {
      const [path, items] = list(ctx, rest[0]);
      const added = evaluate(rest[1], ctx);
      if (Array.isArray(added) && added.length > 0) store.set(path, [...items, ...(added as unknown[])]);
      return;
    }
    case "prepend": {
      const [path, items] = list(ctx, rest[0]);
      const added = evaluate(rest[1], ctx);
      if (Array.isArray(added) && added.length > 0) store.set(path, [...(added as unknown[]), ...items]);
      return;
    }
    case "upsert": {
      // `["upsert", place, depth, key, items]`: an item replaces the held item with its key, or goes at the end.
      const [path, items, read] = list(ctx, rest[0]);
      const added = evaluate(rest[3], ctx);
      if (!Array.isArray(added) || added.length === 0) return;
      // The key may read the item and the page's signals, so the added items are keyed in a copy of the store that holds them after the list's items.
      const all = [...items, ...(added as unknown[])];
      const copy = new Store(setPath(store.untracked(() => store.get("")), path, all) as Values);
      const keys = itemKeys({ ...ctx, store: copy }, rest[1] as number, [path, all, read], compile(rest[2]));
      const next = [...items];
      const at = new Map<string, number>();
      items.forEach((_, index) => {
        if (!at.has(keys[index]!)) at.set(keys[index]!, index);
      });
      added.forEach((item: unknown, offset) => {
        const key = keys[items.length + offset]!;
        const index = at.get(key);
        if (index === undefined) {
          at.set(key, next.length);
          next.push(item);
        } else next[index] = item;
      });
      store.set(path, next);
      return;
    }
    case "remove_keys": {
      // `["remove_keys", place, depth, key, keys]`: the keys arrive as JSON values, and are compared as `JSON.stringify` writes them.
      const found = list(ctx, rest[0]);
      const [path, items] = found;
      const removed = evaluate(rest[3], ctx);
      if (!Array.isArray(removed) || removed.length === 0) return;
      const gone = new Set(removed.map((key: unknown) => JSON.stringify(key)));
      const keys = itemKeys(ctx, rest[1] as number, found, compile(rest[2]));
      const kept = items.filter((_, index) => !gone.has(keys[index]!));
      if (kept.length !== items.length) store.set(path, kept);
      return;
    }
    case "remove": {
      const [path, items] = list(ctx, rest[0]);
      const index = evaluate(rest[1], ctx) as number;
      if (index >= 0 && index < items.length) store.set(path, items.filter((_, i) => i !== index));
      return;
    }
    case "retain": {
      const found = list(ctx, rest[0]);
      const [path, items] = found;
      const depth = rest[1] as number;
      const keep = testItems(ctx, depth, found, compile(rest[2]));
      const kept = items.filter((_, i) => keep[i]);
      if (kept.length !== items.length) store.set(path, kept);
      return;
    }
    case "extend": {
      const [path, items] = list(ctx, rest[0]);
      const count = evaluate(rest[1], ctx) as number;
      const build = compile(rest[3]);
      const slot: Slot = { path: "" };
      const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, rest[2] as number), slot] };
      const added = Array.from({ length: Math.max(0, count) }, (_, index) => {
        slot.path = String(index);
        return build(itemCtx);
      });
      if (added.length > 0) store.set(path, [...items, ...added]);
      return;
    }
    case "update": {
      const found = list(ctx, rest[0]);
      const [path, items, read] = found;
      const depth = rest[1] as number;
      const when = testItems(ctx, depth, found, compile(rest[2]));
      const field = rest[3] as string;
      const value = compile(rest[4]);
      const [slot, at] = walker(path, read);
      const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth), slot] };
      // Each selected item's field is written at its own path, so only the bindings that read that field run.
      const writes: [string, unknown][] = [];
      items.forEach((_, index) => {
        if (!when[index]) return;
        at(index);
        writes.push([join(path, String(index), field), value(itemCtx)]);
      });
      store.setMany(writes);
      return;
    }
    case "swap": {
      const [path, items] = list(ctx, rest[0]);
      const [a, b] = [evaluate(rest[1], ctx) as number, evaluate(rest[2], ctx) as number];
      if (a === b || !(a in items) || !(b in items)) return;
      const next = [...items];
      [next[a], next[b]] = [next[b], next[a]];
      store.set(path, next);
      return;
    }
    case "sort": {
      const [path, items, read] = list(ctx, rest[0]);
      const depth = rest[1] as number;
      const key = compile(rest[2]);
      const [slot, at] = walker(path, read);
      const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth), slot] };
      const keys = items.map((_, index) => {
        at(index);
        return key(itemCtx) as number | string;
      });
      const sign = rest[3] === true ? -1 : 1;
      const order = items.map((_, index) => index).sort((a, b) => (keys[a]! < keys[b]! ? -sign : keys[a]! > keys[b]! ? sign : a - b));
      if (order.some((at, index) => at !== index)) store.set(path, order.map((at) => items[at]));
      return;
    }
    case "set_all": {
      const [path, items] = list(ctx, rest[0]);
      const value = evaluate(rest[2], ctx);
      store.setMany(items.map((_, i) => [join(path, String(i), rest[1] as string), value]));
      return;
    }
    case "if":
      await run(evaluate(rest[0], ctx) ? rest[1] : rest[2], ctx, host);
      return;
    case "call": {
      const args = rest[2] === null ? null : evaluate(rest[2], ctx);
      // A call without flags has six elements, so `rest[5]` is absent.
      const flags = (rest[5] ?? []) as string[];
      const options = { latest: flags.includes("latest"), reconnect: flags.includes("reconnect"), origin: ctx.origin };
      void host.call(rest[0] as string, rest[1] as string, args, rest[3] as string | null, rest[4] as string | null, options);
      return;
    }
    case "upload": {
      // `["upload", path, args, indicator, error, file input, flags?]`, always a POST.
      const args = rest[1] === null ? null : evaluate(rest[1], ctx);
      const flags = (rest[5] ?? []) as string[];
      const options = { latest: flags.includes("latest"), reconnect: false, origin: ctx.origin, files: rest[4] as string };
      void host.call("POST", rest[0] as string, args, rest[2] as string | null, rest[3] as string | null, options);
      return;
    }
    case "dom":
      host.dom(rest[0] as string, rest[1] as string, ctx.origin);
      return;
    case "clear-files":
      host.clearFiles(rest[0] as string, ctx.origin);
      return;
    case "navigate":
      host.navigate(String(evaluate(rest[0], ctx)));
      return;
    case "history":
      host.history(String(evaluate(rest[1], ctx)), rest[0] === "replace");
      return;
    case "debounce":
      host.debounce(ctx.origin, JSON.stringify(rest[1]), rest[0] as number, () => {
        run(rest[1], ctx, host).catch((error: unknown) => console.error("hypersig:", error));
      });
      return;
    case "seq":
      for (const step of rest[0] as unknown[]) await run(step, ctx, host);
      return;
    default:
      throw new AstError(ast);
  }
}
