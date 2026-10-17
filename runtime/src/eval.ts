import { equal, getPath, join, setPath, type Store } from "./store.ts";

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
}

/** The side effects a statement can ask the runtime for. */
export interface Host {
  call(
    method: string,
    path: string,
    args: unknown,
    indicator: string | null,
    error: string | null,
  ): Promise<void>;
  dom(op: string, name: string, origin: Element | undefined): void;
  navigate(url: string): void;
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
      void host.call(rest[0] as string, rest[1] as string, args, rest[3] as string | null, rest[4] as string | null);
      return;
    }
    case "dom":
      host.dom(rest[0] as string, rest[1] as string, ctx.origin);
      return;
    case "navigate":
      host.navigate(String(evaluate(rest[0], ctx)));
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
