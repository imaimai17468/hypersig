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
  const value = ctx.store.get(path);
  return [path, Array.isArray(value) ? value : []];
}

/** `ctx` with the list item `path` bound at `depth`. */
export function withItem(ctx: Context, depth: number, path: string): Context {
  return { ...ctx, scope: [...ctx.scope.slice(0, depth), { path }] };
}

export function evaluate(ast: unknown, ctx: Context): unknown {
  if (!isAst(ast)) throw new AstError(ast);
  const [op, ...rest] = ast;
  switch (op) {
    case "sig":
      return ctx.store.get(rest[0] as string);
    case "item":
      return ctx.store.get(resolve(ast, ctx));
    case "index": {
      const path = slot(ctx, rest[0]).path;
      return Number(path.slice(path.lastIndexOf(".") + 1));
    }
    case "lit":
      return rest[0];
    case "and":
      return Boolean(evaluate(rest[0], ctx)) && Boolean(evaluate(rest[1], ctx));
    case "or":
      return Boolean(evaluate(rest[0], ctx)) || Boolean(evaluate(rest[1], ctx));
    case "if":
      return evaluate(evaluate(rest[0], ctx) ? rest[1] : rest[2], ctx);
    case "fixed":
      return (evaluate(rest[0], ctx) as number).toFixed(rest[1] as number);
    case "get":
      return getPath(evaluate(rest[0], ctx), rest[1] as string);
    case "with":
      return setPath(evaluate(rest[0], ctx), rest[1] as string, evaluate(rest[2], ctx));
    case "count": {
      const [path, items] = list(ctx, rest[0]);
      const depth = rest[1] as number;
      return items.filter((_, i) => evaluate(rest[2], withItem(ctx, depth, join(path, String(i))))).length;
    }
    default: {
      const one = unary[op as string];
      if (one !== undefined && rest.length === 1) return one(evaluate(rest[0], ctx) as never);
      const two = binary[op as string];
      if (two === undefined || rest.length !== 2) throw new AstError(ast);
      return two(evaluate(rest[0], ctx) as never, evaluate(rest[1], ctx) as never);
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
      const kept = items.filter((_, i) => evaluate(rest[2], withItem(ctx, depth, join(path, String(i)))));
      if (kept.length !== items.length) store.set(path, kept);
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
      await host.call(rest[0] as string, rest[1] as string, args, rest[3] as string | null, rest[4] as string | null);
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
