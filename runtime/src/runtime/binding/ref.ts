import type { Runtime } from "../../runtime.ts";
import { own } from "../element.ts";

/** The attribute `El::node_ref` writes, which DOM statements look elements up by. */
export const REF = "data-hs-ref";
/**
 * The attribute `FileInput::input` writes, which uploads look file inputs up
 * by. It is apart from `REF`, so a file input and a node ref never share a name.
 */
export const FILES = "data-hs-files";
/** The attributes that name an element, which a list item's copy keeps from its template. */
export const NAMES = [REF, FILES];

/** Runs `op`, a DOM method a statement names, on `target` where it applies. */
function act(op: string, target: Element): void {
  const html = target instanceof HTMLElement ? target : null;
  if (op === "focus") html?.focus();
  else if (op === "blur") html?.blur();
  else if (op === "select") {
    if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement)
      target.select();
  } else if (op === "scroll") target.scrollIntoView({ block: "nearest" });
  else if (op === "hide-popover") {
    if (html?.matches(":popover-open")) html.hidePopover();
  } else if (op === "show-popover") {
    if (html?.hasAttribute("popover") && !html.matches(":popover-open")) html.showPopover();
  } else if (target instanceof HTMLDialogElement) {
    // `showModal` throws on a dialog that is already open.
    if (op === "show-modal" && !target.open) target.showModal();
    else if (op === "close" && target.open) target.close();
  }
}

/** The elements statements reach by name: node refs, which DOM statements act on, and file inputs. */
export class Refs {
  readonly #rt: Runtime;

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Runs a DOM method on the element a node ref names, after the next render. */
  dom(op: string, name: string, origin: Element | undefined): void {
    this.#afterRender(REF, name, origin, (target) => act(op, target));
  }

  /** Empties the file input named `name`, after the next render. */
  clearFiles(name: string, origin: Element | undefined): void {
    this.#afterRender(FILES, name, origin, (target) => {
      if (target instanceof HTMLInputElement && target.type === "file") target.value = "";
    });
  }

  /**
   * The element whose `attribute` is `name`, looked for first in the list
   * items around `origin`, innermost first, then in the whole page.
   *
   * `first`, where given, maps each name to the first element of the page
   * that carries it, so that a caller who looks up many names reads the
   * page once.
   */
  find(
    attribute: string,
    name: string,
    origin: Element | undefined,
    first?: ReadonlyMap<string, Element>,
  ): Element | null {
    const selector = `[${attribute}="${CSS.escape(name)}"]`;
    for (let at = origin ?? null; at !== null; at = at.parentElement) {
      if (own(at)?.root !== true) continue;
      const found = at.matches(selector) ? at : at.querySelector(selector);
      if (found !== null) return found;
    }
    return first === undefined ? this.#rt.root.querySelector(selector) : (first.get(name) ?? null);
  }

  /**
   * Calls `then` with the element whose `attribute` is `name` once the render
   * the current statement started has run, mounting waiting list items first
   * when the element is not in the document yet.
   */
  #afterRender(
    attribute: string,
    name: string,
    origin: Element | undefined,
    then: (target: Element) => void,
  ): void {
    queueMicrotask(() =>
      queueMicrotask(() => {
        let target = this.find(attribute, name, origin);
        if (target === null) {
          this.#rt.lists.adoption.all();
          target = this.find(attribute, name, origin);
        }
        if (target === null) console.warn(`hypersig: no element with ${attribute}="${name}"`);
        else then(target);
      }),
    );
  }
}
