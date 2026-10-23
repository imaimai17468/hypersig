import {
  type CallOptions,
  type Context,
  evaluate,
  type Host,
  run,
  type Statement,
} from "../../eval.ts";

function report(cause: unknown): void {
  console.error("hypersig:", cause);
}

/** Runs `stmt`, reporting a failure instead of throwing it, for a statement that runs after its caller returned. */
function runLater(stmt: unknown, ctx: Context, host: Host): void {
  try {
    run(stmt, ctx, host);
  } catch (cause) {
    report(cause);
  }
}

/**
 * What a call's `{"error": [depth, statement], "ok": statement, "status": path, "part": path}`
 * element asks for. The statements run when the call fails or succeeds, in
 * the context of the statement that made the call, so they reach its list
 * items and `let` values, and the error statement reads the error text as
 * the `let` value at `depth`. `status` names the signal that holds the
 * status of a failed answer.
 */
function outcomes(
  spec: unknown,
  ctx: Context,
  host: Host,
): Pick<CallOptions, "failed" | "succeeded" | "status" | "part"> {
  if (spec === null || typeof spec !== "object") return {};
  const { error, ok, status, part } = spec as {
    error?: [number, unknown];
    ok?: unknown;
    status?: string;
    part?: string;
  };
  const found: {
    -readonly [K in "failed" | "succeeded" | "status" | "part"]?: CallOptions[K];
  } = {};
  if (part !== undefined) found.part = part;
  if (status !== undefined) found.status = status;
  if (error !== undefined) {
    const [depth, stmt] = error;
    found.failed = (message) =>
      runLater(stmt, { ...ctx, vars: [...(ctx.vars ?? []).slice(0, depth), message] }, host);
  }
  if (ok !== undefined) found.succeeded = () => runLater(ok, ctx, host);
  return found;
}

/** The statements that call an action of the server. */
export const calls: Record<string, Statement> = {
  call: ([method, path, argsAst, indicator, error, flagList, spec], ctx, host) => {
    const args = (): unknown => (argsAst === null ? null : evaluate(argsAst, ctx));
    // A call without flags has six elements, so its flags are absent.
    const flags = (flagList ?? []) as string[];
    const options: CallOptions = {
      latest: flags.includes("latest"),
      reconnect: flags.includes("reconnect"),
      connected: flags.includes("connected"),
      origin: ctx.origin,
      ...outcomes(spec, ctx, host),
    };
    void host.call(
      method as string,
      path as string,
      args,
      indicator as string | null,
      error as string | null,
      options,
    );
  },
  // `["upload", path, args, indicator, error, file input, flags?, outcomes?]`, always a POST.
  upload: ([path, argsAst, indicator, error, files, flagList, spec], ctx, host) => {
    const args = (): unknown => (argsAst === null ? null : evaluate(argsAst, ctx));
    const flags = (flagList ?? []) as string[];
    const options: CallOptions = {
      latest: flags.includes("latest"),
      reconnect: false,
      connected: false,
      origin: ctx.origin,
      files: files as string,
      ...outcomes(spec, ctx, host),
    };
    void host.call(
      "POST",
      path as string,
      args,
      indicator as string | null,
      error as string | null,
      options,
    );
  },
};
