import { type Changes } from "../../store/change.ts";
import { join } from "../../store/path.ts";
import type { Instance } from "./item.ts";

/**
 * Whether `changes` only appended items to the list at `path`, which `known`
 * rendered at its previous length, so the items it holds are unchanged and
 * still sit at the same paths.
 */
export function appended(
  changes: Changes,
  path: string,
  known: ReadonlyMap<string, Instance>,
  length: number,
): boolean {
  if (changes.get(path) !== false || length < known.size) return false;
  const prefix = `${path}.`;
  for (const changed of changes.keys()) {
    if (!changed.startsWith(prefix)) continue;
    const segment = changed.slice(prefix.length).split(".", 1)[0]!;
    if (!(Number(segment) >= known.size)) return false;
  }
  return inPlace(path, known);
}

/**
 * The positions at which `changes` put other values into the list at `path`,
 * which `known` rendered at the same length, where it changed nothing else of
 * the list. The items at the other positions are unchanged and still sit at
 * the same paths. Null where the changes did more, or reached most items.
 */
export function replaced(
  changes: Changes,
  path: string,
  known: ReadonlyMap<string, Instance>,
  length: number,
): Set<number> | null {
  if (changes.get(path) !== false || length !== known.size) return null;
  const prefix = `${path}.`;
  const touched = new Set<number>();
  for (const changed of changes.keys()) {
    if (!changed.startsWith(prefix)) continue;
    const index = Number(changed.slice(prefix.length).split(".", 1)[0]);
    if (!(Number.isInteger(index) && index >= 0 && index < length)) return null;
    touched.add(index);
  }
  return touched.size * 2 <= length && inPlace(path, known) ? touched : null;
}

/**
 * Whether the items of `known` are in the document, each at the path of its
 * position in the list at `path`. Only the first and the last path are
 * compared, because `Lists.render` and `Slots.relocate` give the items of
 * one list their paths together, so they agree with each other.
 */
function inPlace(path: string, known: ReadonlyMap<string, Instance>): boolean {
  let last: Instance | undefined;
  for (const instance of known.values()) {
    if (last === undefined && instance.slot.path !== join(path, "0")) return false;
    if (!instance.element.isConnected) return false;
    last = instance;
  }
  return last === undefined || last.slot.path === join(path, String(known.size - 1));
}
