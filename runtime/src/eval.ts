import type { Store } from "./store.ts";

/**
 * The JSON form of an expression or statement, as `crates/hypersig/src/expr.rs` writes it.
 * The first element names the operation.
 */
export type Ast = readonly unknown[];

export type CallAction = (method: string, path: string, args: unknown) => Promise<void>;

const binary: Record<string, (a: never, b: never) => unknown> = {
  add: (a: number, b: number) => a + b,
  sub: (a: number, b: number) => a - b,
  mul: (a: number, b: number) => a * b,
  div: (a: number, b: number) => a / b,
  concat: (a: string, b: string) => a + b,
  eq: (a: unknown, b: unknown) => a === b,
  ne: (a: unknown, b: unknown) => a !== b,
  gt: (a: number, b: number) => a > b,
  ge: (a: number, b: number) => a >= b,
  lt: (a: number, b: number) => a < b,
  le: (a: number, b: number) => a <= b,
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

export function evaluate(ast: unknown, store: Store): unknown {
  if (!isAst(ast)) throw new AstError(ast);
  const [op, ...rest] = ast;
  switch (op) {
    case "sig":
      return store.get(rest[0] as string);
    case "lit":
      return rest[0];
    case "not":
      return !evaluate(rest[0], store);
    case "str":
      return String(evaluate(rest[0], store));
    case "and":
      return Boolean(evaluate(rest[0], store)) && Boolean(evaluate(rest[1], store));
    case "or":
      return Boolean(evaluate(rest[0], store)) || Boolean(evaluate(rest[1], store));
    default: {
      const fn = binary[op as string];
      if (fn === undefined || rest.length !== 2) throw new AstError(ast);
      return fn(evaluate(rest[0], store) as never, evaluate(rest[1], store) as never);
    }
  }
}

export async function run(ast: unknown, store: Store, call: CallAction): Promise<void> {
  if (!isAst(ast)) throw new AstError(ast);
  const [op, ...rest] = ast;
  switch (op) {
    case "set":
      store.set(rest[0] as string, evaluate(rest[1], store));
      return;
    case "call":
      await call(rest[0] as string, rest[1] as string, rest[2]);
      return;
    case "seq":
      for (const step of rest[0] as unknown[]) await run(step, store, call);
      return;
    default:
      throw new AstError(ast);
  }
}
