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

/** The form controls bound to a signal with `data-hs-bind`, in both directions. */
export class Controls {
  readonly #rt: Runtime;

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Shows the value of the place `element` is bound to. */
  write(element: Element, [place, kind, choice]: Bind, ctx: Context): void {
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
      case "int":
      case "float":
      case "opt_int":
      case "opt_float":
        if (Number(element.value) !== current || element.value === "")
          element.value = String(current ?? "");
        return;
      case "json": {
        const next = JSON.stringify(current);
        if (element.value !== next) element.value = next;
        return;
      }
      default: {
        const next = current === undefined || current === null ? "" : String(current);
        if (element.value !== next) element.value = next;
      }
    }
  }

  /** Writes what the visitor entered in `element` to the place it is bound to, for an event of `type`. */
  read(element: Element, type: string): void {
    const bind = parse(attr(element, "hsBind")) as Bind | undefined;
    if (bind === undefined || !isControl(element)) return;
    const [place, kind, choice] = bind;
    const path = resolve(place, {
      store: this.#rt.store,
      scope: scopeOf(element),
      origin: element,
    });
    const set = (value: unknown): void => this.#rt.store.set(path, value);
    const raw = element.value;
    switch (kind) {
      case "bool":
        if (type === "change" && element instanceof HTMLInputElement) set(element.checked);
        return;
      case "radio":
        if (type === "change" && element instanceof HTMLInputElement && element.checked)
          set(choice);
        return;
      case "int":
      case "float":
      case "opt_int":
      case "opt_float":
        readNumber(raw, kind, set);
        return;
      case "opt_text":
        if (type === "input" || element instanceof HTMLSelectElement) set(raw === "" ? null : raw);
        return;
      case "json":
        if (type === "change") set(JSON.parse(raw));
        return;
      default:
        if (type === "input" || element instanceof HTMLSelectElement) set(raw);
    }
  }
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
