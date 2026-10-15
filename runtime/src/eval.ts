import { equal, getPath, join, setPath, type Store } from "./store.ts";

/**
 * The JSON form of an expression or statement, as `crates/hypersig/src/expr.rs` writes it.
 * The first element names the operation.
 */
export type Ast = readonly unknown[];

/** The store path of one list item, updated in place when the item moves. */
export interface Slot {
  path: string;
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

const unary: Record<string, (a: never) => unknown> = {
  not: (a: unknown) => !a,
  str: (a: unknown) => String(a),
  trim: (a: string) => a.trim(),
  lower: (a: string) => a.toLowerCase(),
  upper: (a: string) => a.toUpperCase(),
  chars: (a: string) => [...a].length,
  len: (a: unknown) => (Array.isArray(a) ? a.length : 0),
};

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

function list(ctx: Context, place: unknown): [string, unknown[]] {
  const path = resolve(place, ctx);
  const value = ctx.store.getShallow(path);
  return [path, Array.isArray(value) ? value : []];
}

/** Whether `test` holds for each item of the list at `path`, with one context reused for every item. */
function testItems(ctx: Context, depth: number, path: string, items: readonly unknown[], test: Compiled): boolean[] {
  const slot: Slot = { path };
  const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth), slot] };
  return items.map((_, index) => {
    slot.path = join(path, String(index));
    return Boolean(test(itemCtx));
  });
}

type Compiled = (ctx: Context) => unknown;

const compiled = new WeakMap<Ast, Compiled>();

/** Evaluates an expression, building it into a function the first time this AST is seen. */
export function evaluate(ast: unknown, ctx: Context): unknown {
  return compile(ast)(ctx);
}

function compile(ast: unknown): Compiled {
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
      return (ctx) => ctx.store.get(join(slot(ctx, depth).path, field));
    }
    case "index": {
      const depth = rest[0];
      return (ctx) => {
        const path = slot(ctx, depth).path;
        ctx.store.getPosition(path);
        return Number(path.slice(path.lastIndexOf(".") + 1));
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
        const [path, items] = list(ctx, place);
        return testItems(ctx, depth, path, items, test).filter(Boolean).length;
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
      const [path, items] = list(ctx, rest[0]);
      const depth = rest[1] as number;
      const keep = testItems(ctx, depth, path, items, compile(rest[2]));
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
      const [path, items] = list(ctx, rest[0]);
      const depth = rest[1] as number;
      const when = testItems(ctx, depth, path, items, compile(rest[2]));
      const field = rest[3] as string;
      const value = compile(rest[4]);
      const slot: Slot = { path };
      const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth), slot] };
      let next: unknown[] | null = null;
      items.forEach((item, index) => {
        if (!when[index]) return;
        slot.path = join(path, String(index));
        next ??= [...items];
        next[index] = setPath(item, field, value(itemCtx));
      });
      if (next !== null) store.set(path, next);
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
      const [path, items] = list(ctx, rest[0]);
      const depth = rest[1] as number;
      const key = compile(rest[2]);
      const slot: Slot = { path };
      const itemCtx: Context = { ...ctx, scope: [...ctx.scope.slice(0, depth), slot] };
      const keys = items.map((_, index) => {
        slot.path = join(path, String(index));
        return key(itemCtx) as number | string;
      });
      const order = items.map((_, index) => index).sort((a, b) => (keys[a]! < keys[b]! ? -1 : keys[a]! > keys[b]! ? 1 : a - b));
      if (order.some((at, index) => at !== index)) store.set(path, order.map((at) => items[at]));
      return;
    }
    case "set_all": {
      const [path, items] = list(ctx, rest[0]);
      const value = evaluate(rest[2], ctx);
      items.forEach((_, i) => store.set(join(path, String(i), rest[1] as string), value));
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
