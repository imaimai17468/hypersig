import type { Runtime } from "../../../runtime.ts";
import { attr } from "../../element.ts";
import { parse } from "../../spec.ts";

/** An entry of `data-hs-on-key`, which `keys_json` in `crates/hypersig/src/html.rs` writes. */
type Shortcut = [key: string, stmt: unknown, modifiers?: string[]];

/** Whether the primary modifier is Meta, as on Apple platforms, rather than Ctrl. */
const PRIMARY_IS_META =
  typeof navigator !== "undefined" && /Mac|iPhone|iPad|iPod/u.test(navigator.platform);

/** The `type`s of an `<input>` that holds no text a key edits. */
const NOT_TEXT = new Set([
  "button",
  "checkbox",
  "color",
  "file",
  "hidden",
  "image",
  "radio",
  "range",
  "reset",
  "submit",
]);
/** Keys a text field edits or moves the caret with. */
const TEXT_KEYS = new Set([" ", "Backspace", "Delete", "ArrowLeft", "ArrowRight", "Home", "End"]);
/** Keys a field of several lines also uses. */
const LINE_KEYS = new Set(["Enter", "ArrowUp", "ArrowDown"]);

/** Runs the statements of the nearest `data-hs-on-key` element, from `target` out, that has a shortcut matching `event`. */
export function onKey(rt: Runtime, target: Element, event: KeyboardEvent): void {
  for (let handler: Element | null = target; handler !== null; handler = handler.parentElement) {
    const source = attr(handler, "hsOnKey");
    if (source === undefined) continue;
    if (!rt.root.contains(handler)) return;
    let matched = false;
    for (const [key, stmt, modifiers = []] of parse(source) as Shortcut[]) {
      if (!pressed(event, key, modifiers)) continue;
      // A field inside the handler's element keeps the keys it types with, so a page shortcut does not swallow them.
      if (handler !== target && editsText(target, key, modifiers)) continue;
      if (!matched) event.preventDefault();
      matched = true;
      rt.bindings.statements.run(stmt, rt.bindings.statements.context(handler));
    }
    if (matched) return;
  }
}

/** Whether `event` is `key` with exactly the `modifiers`, where Shift is compared for a character only when they name it. */
function pressed(event: KeyboardEvent, key: string, modifiers: readonly string[]): boolean {
  const primary = modifiers.includes("primary");
  if (event.ctrlKey !== (modifiers.includes("ctrl") || (primary && !PRIMARY_IS_META))) return false;
  if (event.metaKey !== (modifiers.includes("meta") || (primary && PRIMARY_IS_META))) return false;
  if (event.altKey !== modifiers.includes("alt")) return false;
  const shift = modifiers.includes("shift");
  // A character already shows Shift, as `K` and `?` do, and Shift held for one turns `k` into `K`.
  if (isCharacter(key))
    return shift
      ? event.shiftKey && event.key.toLowerCase() === key.toLowerCase()
      : event.key === key;
  return event.key === key && event.shiftKey === shift;
}

/** Whether `key`, as `KeyboardEvent.key` names it, is a character rather than a named key such as `Enter` or Space. */
function isCharacter(key: string): boolean {
  return key !== " " && [...key].length === 1;
}

/** Whether `field` is a text field that acts on `key` by itself, which a shortcut with Ctrl, Alt, Meta, or the primary modifier never is. */
function editsText(field: Element, key: string, modifiers: readonly string[]): boolean {
  if (modifiers.some((modifier) => modifier !== "shift")) return false;
  const lines =
    field instanceof HTMLTextAreaElement ||
    (field instanceof HTMLElement && field.isContentEditable);
  if (!lines && !(field instanceof HTMLInputElement && !NOT_TEXT.has(field.type))) return false;
  return isCharacter(key) || TEXT_KEYS.has(key) || (lines && LINE_KEYS.has(key));
}
