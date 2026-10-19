import type { Shape } from "./shape.ts";

/** Whether a change at `rest` inside an item, the whole value there when `inside`, can change a deep read of its field `field`. */
export function reaches(field: string, rest: string, inside: boolean): boolean {
  if (field === "" || rest === field || rest.startsWith(`${field}.`)) return true;
  return inside && (rest === "" || field.startsWith(`${rest}.`));
}

/** `reaches` for a shallow read of `field`, which a change below it leaves alone. */
function reachesShallow(field: string, rest: string, inside: boolean): boolean {
  if (rest === field) return true;
  return inside && (rest === "" || field.startsWith(`${rest}.`));
}

/** The parts of `shape` that a change at `rest` inside an item can reach. */
export function maskFor(shape: Shape, rest: string, inside: boolean): number {
  const id = `${inside ? 1 : 0}${rest}`;
  let mask = shape.masks.get(id);
  if (mask === undefined) {
    let found = 0;
    shape.fields.forEach((fields, index) => {
      const reach = (shape.shallow & (1 << index)) === 0 ? reaches : reachesShallow;
      if (fields.some((field) => reach(field, rest, inside))) found |= 1 << index;
    });
    // A change inside a nested list names the position it is at, so the changes seen can keep growing.
    if (shape.masks.size > 1000) shape.masks.clear();
    shape.masks.set(id, found);
    mask = found;
  }
  return mask;
}
