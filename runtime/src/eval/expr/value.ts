import { type Builder, compile } from "../../eval.ts";
import { getPath, setSegments } from "../../store/path.ts";
import { slotAt } from "../scope.ts";

/** The expressions that read a signal, an item, a `let` value, or a literal, and those that choose or combine values. */
export const values: Record<string, Builder> = {
  sig:
    ([path]) =>
    (ctx) =>
      ctx.store.get(path as string),
  item:
    ([depth, field]) =>
    (ctx) => {
      const found = slotAt(ctx, depth);
      return ctx.store.getIn(found.path, field as string, found.read ?? found.path);
    },
  index:
    ([depth]) =>
    (ctx) => {
      const found = slotAt(ctx, depth);
      ctx.store.getPosition(found.read ?? found.path);
      return Number(found.path.slice(found.path.lastIndexOf(".") + 1));
    },
  let: ([valueAst, depth, bodyAst]) => {
    const [value, body] = [compile(valueAst), compile(bodyAst)];
    return (ctx) =>
      body({ ...ctx, vars: [...(ctx.vars ?? []).slice(0, depth as number), value(ctx)] });
  },
  var:
    ([depth]) =>
    (ctx) =>
      ctx.vars?.[depth as number] ?? null,
  lit:
    ([value]) =>
    () =>
      value,
  random: ([boundAst]) => {
    const bound = compile(boundAst);
    return (ctx) => Math.floor(Math.random() * (bound(ctx) as number));
  },
  now: () => () => Date.now(),
  nth: ([ofAst, indexAst]) => {
    const [of, index] = [compile(ofAst), compile(indexAst)];
    return (ctx) => {
      const items = of(ctx);
      return Array.isArray(items) ? (items[index(ctx) as number] ?? null) : null;
    };
  },
  tuple: (rest) => {
    const parts = rest.map((part) => compile(part));
    return (ctx) => parts.map((part) => part(ctx));
  },
  and: ([aAst, bAst]) => {
    const [a, b] = [compile(aAst), compile(bAst)];
    return (ctx) => Boolean(a(ctx)) && Boolean(b(ctx));
  },
  or: ([aAst, bAst]) => {
    const [a, b] = [compile(aAst), compile(bAst)];
    return (ctx) => Boolean(a(ctx)) || Boolean(b(ctx));
  },
  if: ([condAst, thenAst, otherwiseAst]) => {
    const [cond, then, otherwise] = [compile(condAst), compile(thenAst), compile(otherwiseAst)];
    return (ctx) => (cond(ctx) ? then(ctx) : otherwise(ctx));
  },
  get: ([valueAst, path]) => {
    const value = compile(valueAst);
    return (ctx) => getPath(value(ctx), path as string);
  },
  with: ([valueAst, path, nextAst]) => {
    const [value, next] = [compile(valueAst), compile(nextAst)];
    if (path === "") return next;
    const segments = (path as string).split(".");
    return (ctx) => setSegments(value(ctx), segments, next(ctx));
  },
};
