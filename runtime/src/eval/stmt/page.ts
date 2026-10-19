import { evaluate, type Statement } from "../../eval.ts";

/** The statements that act on an element of the page or on its URL. */
export const pages: Record<string, Statement> = {
  dom: ([op, name], ctx, host) => host.dom(op as string, name as string, ctx.origin),
  "clear-files": ([name], ctx, host) => host.clearFiles(name as string, ctx.origin),
  navigate: ([url], ctx, host) => host.navigate(String(evaluate(url, ctx))),
  history: ([mode, url], ctx, host) => host.history(String(evaluate(url, ctx)), mode === "replace"),
};
