import { type Context } from "../../eval.ts";
import { resolve, resolveRead } from "../../eval/scope.ts";
import { equal } from "../../store/value.ts";
import type { Runtime } from "../../runtime.ts";
import { attr, scopeOf } from "../element.ts";
import { type Bind, parse } from "../spec.ts";

type Control = HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement;

function isControl(element: Element): element is Control {
  return (
    element instanceof HTMLInputElement ||
    element instanceof HTMLTextAreaElement ||
    element instanceof HTMLSelectElement
  );
}

/**
 * The form controls bound to a signal with `data-hs-bind`, in both directions.
 *
 * The visitor is typing in a control that has focus and has had an `input`
 * event since it got focus, fired `change`, or had its form submitted. An
 * action's answer can repeat a text such a control wrote to the store before
 * the visitor typed on. Where a store change brings back one of those texts,
 * the control keeps its newer text and writes it to the store again, so the
 * next call sends what the visitor sees. Any other value is shown at once,
 * because a statement that empties the field after Enter or picks an option
 * writes a text the visitor never typed. A change that reaches a focused
 * control keeps the caret, moved back to the end of the new text where that
 * is shorter.
 */
export class Controls {
  readonly #rt: Runtime;
  /** The texts each control the visitor is typing in wrote to the store since the typing began. */
  readonly #typed = new WeakMap<Control, Set<string>>();

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Shows the value of the place `element` is bound to. */
  write(element: Element, bind: Bind, ctx: Context): void {
    const [place, kind, choice] = bind;
    const current = this.#rt.store.getAs(resolve(place, ctx), resolveRead(place, ctx));
    if (!isControl(element)) return;
    switch (kind) {
      case "bool":
        if (element instanceof HTMLInputElement && element.checked !== current)
          element.checked = Boolean(current);
        return;
      case "radio":
        if (element instanceof HTMLInputElement) element.checked = equal(current, choice);
        return;
      default:
        this.#show(element, bind, text(element, kind, current));
    }
  }

  /** Writes what the visitor entered in `element` to the place it is bound to, for an event of `type`. */
  read(element: Element, type: string): void {
    const bind = parse(attr(element, "hsBind")) as Bind | undefined;
    if (bind === undefined || !isControl(element)) return;
    const [, kind, choice] = bind;
    const path = this.#path(element, bind);
    const set = (value: unknown): void => this.#rt.store.set(path, value);
    if (type === "input" && !this.#typed.has(element)) this.#typed.set(element, new Set());
    switch (kind) {
      case "bool":
        if (type === "change" && element instanceof HTMLInputElement) set(element.checked);
        return;
      case "radio":
        if (type === "change" && element instanceof HTMLInputElement && element.checked)
          set(choice);
        return;
      case "json":
        if (type === "change") readText(element.value, kind, set);
        break;
      case "int":
      case "float":
      case "opt_int":
      case "opt_float":
        readText(element.value, kind, set);
        break;
      default:
        if (type === "input" || element instanceof HTMLSelectElement)
          readText(element.value, kind, set);
    }
    if (type === "input") this.#typed.get(element)?.add(element.value);
    if (type === "change") this.#end(element);
  }

  /** Ends the typing in `element`, which focus is leaving. */
  left(element: Element): void {
    if (isControl(element)) this.#end(element);
  }

  /** Ends the typing in each control of `form`, before the submit's handler reads the store. */
  submitted(form: Element): void {
    if (!(form instanceof HTMLFormElement)) return;
    for (const element of form.elements) if (isControl(element)) this.#end(element);
  }

  /** Ends the typing in `element`. */
  #end(element: Control): void {
    this.#typed.delete(element);
  }

  /** Writes the text of `element` to the place `bind` names, read as `bind` says. */
  #take(element: Control, bind: Bind): void {
    const path = this.#path(element, bind);
    readText(element.value, bind[1], (value) => this.#rt.store.set(path, value));
  }

  /** Sets the text of `element` to `next`, unless the visitor is typing in it, as the class comment says. */
  #show(element: Control, bind: Bind, next: string | undefined): void {
    if (next === undefined || element.value === next) return;
    const focused = element === document.activeElement;
    if (focused && this.#typed.get(element)?.has(next) === true) {
      this.#take(element, bind);
      return;
    }
    setKeepingCaret(element, next, focused);
  }

  /** The store path of the place `element` is bound to by `bind`. */
  #path(element: Element, [place]: Bind): string {
    return resolve(place, { store: this.#rt.store, scope: scopeOf(element), origin: element });
  }
}

/**
 * The text `element`, bound as `kind`, shows for `current`, or `undefined`
 * where it already shows that value, as a number field showing `1.50` for `1.5` does.
 */
function text(element: Control, kind: string, current: unknown): string | undefined {
  const isNumber = kind.endsWith("int") || kind.endsWith("float");
  if (isNumber && Number(element.value) === current && element.value !== "") return undefined;
  return shown(kind, current);
}

/** The text a control bound as `kind` shows for the store value `value`. */
function shown(kind: string, value: unknown): string {
  if (kind === "json") return JSON.stringify(value);
  return value === undefined || value === null ? "" : String(value);
}

/** Sets the value of `element` to `next`, and puts the caret back where `element` has focus. */
function setKeepingCaret(element: Control, next: string, focused: boolean): void {
  if (!focused || element instanceof HTMLSelectElement || element.selectionStart === null) {
    element.value = next;
    return;
  }
  const { selectionStart: start, selectionEnd: end, selectionDirection: direction } = element;
  element.value = next;
  const at = (offset: number | null): number => Math.min(offset ?? next.length, next.length);
  element.setSelectionRange(at(start), at(end), direction ?? "none");
}

/** Sets the value the text `raw` of a control bound as `kind` holds, as `kind` reads it. */
function readText(raw: string, kind: string, set: (value: unknown) => void): void {
  switch (kind) {
    case "int":
    case "float":
    case "opt_int":
    case "opt_float":
      readNumber(raw, kind, set);
      return;
    case "opt_text":
      set(raw === "" ? null : raw);
      return;
    case "json":
      readJson(raw, set);
      return;
    default:
      set(raw);
  }
}

/**
 * Sets the value `raw` holds as JSON, where it holds one. The server writes
 * each option of a bound `<select>` as JSON, so other text comes from a
 * control whose value the visitor typed, and leaves the signal as it was.
 */
function readJson(raw: string, set: (value: unknown) => void): void {
  let value: unknown;
  try {
    value = JSON.parse(raw);
  } catch {
    return;
  }
  set(value);
}

/** Sets the number `raw` holds, read as `kind` says, where it holds one. */
function readNumber(raw: string, kind: string, set: (value: unknown) => void): void {
  // An emptied field sets an `Option` signal to `None` and leaves a plain number as it was.
  if (raw.trim() === "") {
    if (kind.startsWith("opt_")) set(null);
    return;
  }
  const number = Number(raw);
  if (!Number.isFinite(number)) return;
  set(kind.endsWith("int") ? Math.trunc(number) : number);
}
