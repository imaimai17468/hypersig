import type { Runtime } from "../../runtime.ts";
import { attr } from "../element.ts";
import { activate } from "./event/active.ts";
import { onKey } from "./event/key.ts";
import { rove } from "./event/rove.ts";
import { BUBBLING, DIRECT, parse } from "../spec.ts";

/** Whether `node` is `element` or inside it. */
function within(element: Element, node: EventTarget | null): boolean {
  return node instanceof Node && element.contains(node);
}

/** The DOM events the page's handlers listen for, caught on the root. */
export class Events {
  readonly #rt: Runtime;
  /** The custom event types each element listens for. */
  readonly #listened = new WeakMap<Element, Set<string>>();

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** Listens on `root` for every event a handler can name. */
  listen(root: Element): void {
    for (const type of BUBBLING) root.addEventListener(type, (event) => this.#bubbled(type, event));
    for (const type of DIRECT)
      root.addEventListener(type, (event) => this.#direct(type, event), { capture: true });
  }

  /** The closest element from `element` up whose attribute with the `dataset` name `key` is set, with its value. */
  closest(element: Element, key: string): [Element, string] | null {
    for (let at: Element | null = element; at !== null; at = at.parentElement) {
      const value = attr(at, key);
      if (value !== undefined) return this.#rt.root.contains(at) ? [at, value] : null;
    }
    return null;
  }

  /**
   * Listens on `element` for the custom events its `data-hs-on-custom` names.
   * Each entry is an event's type, the depth its `detail` takes among the
   * statement's variables, and the statement. An event runs the entries the
   * attribute holds when it arrives, so calling this again after a patch
   * changed the attribute only adds the types it did not name before.
   */
  custom(element: Element): void {
    let listened = this.#listened.get(element);
    if (listened === undefined) {
      listened = new Set();
      this.#listened.set(element, listened);
    }
    for (const [type] of customHandlers(element)) {
      if (listened.has(type)) continue;
      listened.add(type);
      element.addEventListener(type, (event) => this.#customEvent(element, type, event));
    }
  }

  #customEvent(element: Element, type: string, event: Event): void {
    const { statements } = this.#rt.bindings;
    this.#rt.lists.adoption.around(element);
    const detail = event instanceof CustomEvent ? (event.detail as unknown) : null;
    for (const [named, depth, stmt] of customHandlers(element)) {
      if (named !== type) continue;
      const ctx = statements.context(element);
      statements.run(stmt, {
        ...ctx,
        vars: [...(ctx.vars ?? []).slice(0, depth), detail ?? null],
      });
    }
  }

  #bubbled(type: (typeof BUBBLING)[number], event: Event): void {
    const target = event.target;
    if (!(target instanceof Element)) return;
    this.#rt.lists.adoption.around(target);
    if (type === "input" || type === "change") this.#rt.bindings.controls.read(target, type);
    if (type === "keydown" && event instanceof KeyboardEvent) {
      activate(this.#rt, target, event);
      onKey(this.#rt, target, event);
      if (!event.defaultPrevented) rove(this, target, event);
    }
    const found = this.closest(target, handlerKey(type));
    if (found === null) return;
    // Focus moving between two elements inside the handler's element neither enters nor leaves it.
    if (
      (type === "focusin" || type === "focusout") &&
      event instanceof FocusEvent &&
      within(found[0], event.relatedTarget)
    )
      return;
    if (type === "submit") event.preventDefault();
    this.#rt.bindings.statements.runAttribute(found[1], found[0]);
  }

  #direct(type: (typeof DIRECT)[number], event: Event): void {
    const target = event.target;
    if (!(target instanceof Element)) return;
    this.#rt.lists.adoption.around(target);
    const stmt = attr(target, handlerKey(type));
    if (stmt !== undefined) this.#rt.bindings.statements.runAttribute(stmt, target);
  }
}

/** The entries of `element`'s `data-hs-on-custom`. */
function customHandlers(element: Element): readonly [string, number, unknown][] {
  const handlers = parse(attr(element, "hsOnCustom"));
  return Array.isArray(handlers) ? (handlers as [string, number, unknown][]) : [];
}

/** The `dataset` name of the handler attribute for events of `type`: `hsOnClick` for `data-hs-on-click`. */
function handlerKey(type: string): string {
  return `hsOn${type[0]!.toUpperCase()}${type.slice(1)}`;
}
