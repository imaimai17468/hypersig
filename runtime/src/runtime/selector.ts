/** The key a selector part files an item under when its value is not a JSON scalar, which every change of the signal reaches. */
export const ANY = "*";

/** Whether `value` is a JSON scalar, which a selector can file a binding under. */
export function scalar(value: unknown): boolean {
  return (
    value === null ||
    typeof value === "string" ||
    typeof value === "number" ||
    typeof value === "boolean"
  );
}

/** The key `DepIndex.select` files a value under. */
export function keyOf(value: unknown): string {
  // `String` writes a finite number or a boolean as `JSON.stringify` does, without its checks for other types.
  if ((typeof value === "number" && Number.isFinite(value)) || typeof value === "boolean")
    return String(value);
  return JSON.stringify(value) ?? "undefined";
}

/** The path of `side`, where it reads a page signal. */
function signal(side: unknown): string | null {
  return Array.isArray(side) && side[0] === "sig" && typeof side[1] === "string" ? side[1] : null;
}

/** The page signal path and the other operand of `ast`, where `ast` compares a page signal for equality. */
export function comparison(ast: unknown): [path: string, other: unknown] | null {
  if (!Array.isArray(ast) || ast[0] !== "eq" || ast.length !== 3) return null;
  const [, a, b] = ast as [string, unknown, unknown];
  const left = signal(a);
  if (left !== null) return [left, b];
  const right = signal(b);
  return right === null ? null : [right, a];
}
