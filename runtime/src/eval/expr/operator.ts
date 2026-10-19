import { AstError, type Ast, type Compiled, compile } from "../../eval.ts";
import { equal } from "../../store/value.ts";

/**
 * The query string of a search struct, as `query_string` in `crates/hypersig/src/route.rs` writes it:
 * fields in the order of their names, `null` ones left out, a list's name once per item,
 * each name and value percent-encoded.
 */
function queryString(value: unknown): string {
  if (value === null || typeof value !== "object") return "";
  const fields = value as Record<string, unknown>;
  const pairs = Object.keys(fields)
    .toSorted()
    .flatMap((name) => {
      const field = fields[name];
      const values = Array.isArray(field)
        ? field
        : field === null || field === undefined
          ? []
          : [field];
      return values.map(
        (item) => `${encodeURIComponent(name)}=${encodeURIComponent(String(item))}`,
      );
    });
  return pairs.length === 0 ? "" : `?${pairs.join("&")}`;
}

const unary: Record<string, (a: never) => unknown> = {
  not: (a: unknown) => !a,
  str: String,
  uri: (a: unknown) => encodeURIComponent(String(a)),
  uri_path: (a: unknown) =>
    String(a)
      .split("/")
      .map((segment) => encodeURIComponent(segment))
      .join("/"),
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

/** Whether `op` names an operation that `operator` builds. */
export function isOperator(op: string, arity: number): boolean {
  return (arity === 1 && op in unary) || (arity === 2 && op in binary);
}

/** Builds `ast` as an operation of one or two operands, the expressions no other builder names. */
export function operator(ast: Ast): Compiled {
  const [op, ...rest] = ast as [string, ...unknown[]];
  const one = unary[op];
  if (one !== undefined && rest.length === 1) {
    const a = compile(rest[0]);
    return (ctx) => one(a(ctx) as never);
  }
  const two = binary[op];
  if (two === undefined || rest.length !== 2) throw new AstError(ast);
  const [a, b] = [compile(rest[0]), compile(rest[1])];
  return (ctx) => two(a(ctx) as never, b(ctx) as never);
}
