import type { Store } from "./store.ts";
import { lists } from "./eval/expr/list.ts";
import { operator } from "./eval/expr/operator.ts";
import { texts } from "./eval/expr/text.ts";
import { values } from "./eval/expr/value.ts";
import { calls } from "./eval/stmt/call.ts";
import { eaches } from "./eval/stmt/each.ts";
import { flows } from "./eval/stmt/flow.ts";
import { positions } from "./eval/stmt/list.ts";
import { pages } from "./eval/stmt/page.ts";

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
  read?: string | undefined;
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
  /** Set the indicator while the server's answer is open, rather than from sending until the answer ends. */
  readonly connected: boolean;
  /** The element whose statement made the call. Reconnecting stops once it leaves the document. */
  readonly origin: Element | undefined;
  /** The name of the file input whose files go with the arguments, for an upload. */
  readonly files?: string;
  /** Runs the call's own statement for a failure, with the error text. */
  readonly failed?: (message: string) => void;
  /** Runs the call's own statement once the answer has been applied. */
  readonly succeeded?: () => void;
  /** The signal set to the status of a failed answer, and to 0 when the call is sent or a stream connects. */
  readonly status?: string;
  /** The dotted path of the page field the part that made the call sits at. */
  readonly part?: string;
}

/** The side effects a statement can ask the runtime for. */
export interface Host {
  call(
    method: string,
    path: string,
    args: () => unknown,
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

export type Compiled = (ctx: Context) => unknown;

/** Builds the function of one kind of expression from its operands, the rest of its AST after the operation's name. */
export type Builder = (rest: readonly unknown[], ast: Ast) => Compiled;

/** Runs one kind of statement with its operands, the rest of its AST after the statement's name. */
export type Statement = (rest: readonly unknown[], ctx: Context, host: Host) => void;

export class AstError extends Error {
  constructor(ast: unknown) {
    super(`hypersig: unknown expression ${JSON.stringify(ast)}`);
    this.name = "AstError";
  }
}

export function isAst(value: unknown): value is Ast {
  return Array.isArray(value) && typeof value[0] === "string";
}

/** The builders of the expressions that have a name of their own, by name. The others are operators. */
const builders: Readonly<Record<string, Builder>> = { ...values, ...texts, ...lists };

/** The statements, by name. */
const statements: Readonly<Record<string, Statement>> = {
  ...flows,
  ...pages,
  ...positions,
  ...eaches,
  ...calls,
};

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
    const builder = builders[ast[0] as string];
    fn = builder === undefined ? operator(ast) : builder(ast.slice(1), ast);
    compiled.set(ast, fn);
  }
  return fn;
}

/**
 * Runs a statement. Every step runs before the store notifies its listeners,
 * so the page renders once for the whole statement.
 */
export function run(ast: unknown, ctx: Context, host: Host): void {
  if (!isAst(ast)) throw new AstError(ast);
  const statement = statements[ast[0] as string];
  if (statement === undefined) throw new AstError(ast);
  statement(ast.slice(1), ctx, host);
}
