import { isOperator } from "./eval.ts";

/** A read that an expression can make, found from its AST without running it. */
export type StaticRead =
  | { readonly kind: "sig"; readonly path: string }
  | { readonly kind: "item"; readonly depth: number; readonly field: string }
  | { readonly kind: "index"; readonly depth: number };

/**
 * Every read `ast` can make, in whichever branch it takes, or null where it
 * holds an operation not listed here. A read of a page signal or of an item's
 * field is deep. `["count", ...]` reads its whole list, so the reads its test
 * makes of the counted item are left out.
 */
export function staticReads(ast: unknown): StaticRead[] | null {
  const found: StaticRead[] = [];
  return collect(ast, found, -1) ? found : null;
}

/** The read of the value at `place`: `"a.b"` for a page signal, `["item",depth,"a"]` for an item field. */
export function placeRead(place: unknown): StaticRead | null {
  if (typeof place === "string") return { kind: "sig", path: place };
  if (Array.isArray(place) && place[0] === "item" && typeof place[1] === "number" && typeof place[2] === "string") {
    return { kind: "item", depth: place[1], field: place[2] };
  }
  return null;
}

/** Adds the reads of `ast` to `found`, leaving out those of the item at `counted`. False for an unknown operation. */
function collect(ast: unknown, found: StaticRead[], counted: number): boolean {
  if (!Array.isArray(ast) || typeof ast[0] !== "string") return false;
  const [op, ...rest] = ast as [string, ...unknown[]];
  switch (op) {
    case "sig":
      if (typeof rest[0] !== "string") return false;
      found.push({ kind: "sig", path: rest[0] });
      return true;
    case "item": {
      const read = placeRead(ast);
      if (read === null) return false;
      if (read.kind === "item" && read.depth !== counted) found.push(read);
      return true;
    }
    case "index":
      if (typeof rest[0] !== "number") return false;
      if (rest[0] !== counted) found.push({ kind: "index", depth: rest[0] });
      return true;
    case "lit":
    case "now":
      return true;
    case "count": {
      const read = placeRead(rest[0]);
      if (read === null || typeof rest[1] !== "number") return false;
      if (!(read.kind === "item" && read.depth === counted)) found.push(read);
      return collect(rest[2], found, rest[1]);
    }
    case "fixed":
    case "get":
      return collect(rest[0], found, counted);
    case "with":
      return collect(rest[0], found, counted) && collect(rest[2], found, counted);
    case "random":
    case "nth":
    case "tuple":
    case "and":
    case "or":
    case "if":
      return rest.every((part) => collect(part, found, counted));
    default:
      return isOperator(op, rest.length) && rest.every((part) => collect(part, found, counted));
  }
}
