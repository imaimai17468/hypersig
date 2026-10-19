import { type Context, evaluate, type Slot } from "../../eval.ts";
import { resolve, resolveRead } from "../../eval/scope.ts";
import type { Effect } from "../../deps.ts";
import type { Runtime } from "../../runtime.ts";
import { attr, own, state } from "../element.ts";
import type { Row } from "../list/row.ts";
import { locate, planOf } from "../plan.ts";
import { NAMES } from "./ref.ts";
import { parse, REACTIVE, readSpec, type Spec, specOf } from "../spec.ts";
import { CONDITIONS, writable, write, writes } from "../write.ts";

/** Registers the `data-hs-*` attributes of elements as bindings, which the render runs. */
export class Attributes {
  readonly #rt: Runtime;
  /**
   * Reports each element with an on-visible statement as it enters or leaves
   * the viewport, and once when it starts being observed. Absent where the
   * browser has no `IntersectionObserver`.
   */
  readonly #visibility: IntersectionObserver | null;

  constructor(rt: Runtime) {
    this.#rt = rt;
    this.#visibility =
      typeof IntersectionObserver === "undefined"
        ? null
        : new IntersectionObserver((entries, observer) => {
            for (const { target, isIntersecting } of entries) {
              // An element leaves the viewport as it leaves the document, so it is reported once more then.
              if (!target.isConnected) observer.unobserve(target);
              else if (isIntersecting)
                rt.bindings.statements.runAttribute(attr(target, "hsOnVisible")!, target);
            }
          });
  }

  /**
   * Registers the reactive attributes of `root` and every element below it,
   * apart from the elements of list items and conditions, which `mount` registers.
   */
  scan(root: Element, scope: readonly Slot[]): void {
    const elements = root.matches(REACTIVE) ? [root] : [];
    elements.push(...root.querySelectorAll(REACTIVE));
    for (const element of elements) {
      if (own(element)?.source !== undefined) continue;
      this.#register(element, own(element)?.scope ?? scope);
    }
  }

  /**
   * Registers the bindings of `root`, an item or condition built from
   * `proto` by cloning it or, where `rendered`, by the server, finding its
   * elements by their position in `proto`. The bindings of a list item that
   * are parts of its `row` are left to the row, which runs them all in the
   * next render.
   */
  mount(root: Element, proto: Element, scope: readonly Slot[], rendered: boolean, row?: Row): void {
    const members: Element[] = [];
    const nodes: (Element | undefined)[] = [];
    locate(planOf(proto), root, rendered, (element, source, reactive) => {
      state(element).source = source;
      // A clone of `proto` already carries every attribute of its source.
      if (rendered) copyNames(element, source);
      if (!reactive) return;
      members.push(element);
      const parts = row?.shape.partsOf.get(source);
      if (parts === undefined) this.#register(element, scope);
      else for (const part of parts) nodes[part] = element;
    });
    state(root).members = members;
    if (row === undefined) return;
    // `scopeOf` finds an item's scope here for an event on an element whose bindings are all parts.
    state(root).scope = scope;
    row.nodes = nodes;
    row.ctx = { store: this.#rt.store, scope };
    this.#rt.lists.rows.mark(row, row.shape.all);
  }

  #register(element: Element, scope: readonly Slot[]): void {
    const bound = state(element);
    bound.scope = scope;
    for (const old of bound.effects ?? []) this.#rt.render.index.remove(old);
    const effects = this.#effectsFor(element, scope);
    bound.effects = effects;
    this.#rt.render.queue(effects);
    if (bound.started) return;
    bound.started = true;
    this.#start(element);
  }

  /** Starts the statements of `element` that run on their own: on load, on a timer, on becoming visible, and on a custom event. */
  #start(element: Element): void {
    const { statements, timers, events } = this.#rt.bindings;
    const load = attr(element, "hsOnLoad");
    if (load !== undefined) statements.runAttribute(load, element);
    const interval = parse(attr(element, "hsOnInterval"));
    if (Array.isArray(interval)) timers.interval(element, interval[0] as number, interval[1]);
    const timeout = parse(attr(element, "hsOnTimeout"));
    if (Array.isArray(timeout)) timers.timeout(element, timeout[0] as number, timeout[1]);
    if (attr(element, "hsOnVisible") !== undefined) this.#visibility?.observe(element);
    const custom = parse(attr(element, "hsOnCustom"));
    if (Array.isArray(custom)) events.custom(element, custom as [string, number, unknown][]);
  }

  #effectsFor(element: Element, scope: readonly Slot[]): Effect[] {
    const source = own(element)?.source;
    const spec = source === undefined ? readSpec(element) : specOf(source);
    const ctx: Context = { store: this.#rt.store, scope, origin: element };
    const effects: Effect[] = [];
    const add = (body: () => void): Effect => {
      const effect = this.#rt.render.effect(element, body);
      effects.push(effect);
      return effect;
    };
    this.#writers(element, spec, ctx, add);
    this.#statements(spec, ctx, add);
    if (element instanceof HTMLTemplateElement) {
      const { proto, each, cond } = spec;
      if (each !== undefined) {
        const list: Effect = add(() =>
          this.#rt.lists.render(element, proto, each[0], each[1], scope, list),
        );
      }
      if (cond !== undefined)
        add(() => this.#rt.conditions.render(element, proto, Boolean(evaluate(cond, ctx)), scope));
    }
    return effects;
  }

  /** Adds the bindings of `spec` that write to `element` or its control. */
  #writers(element: Element, spec: Spec, ctx: Context, add: (body: () => void) => Effect): void {
    const truth = (ast: unknown): boolean => this.#rt.render.truth(ast, ctx);
    for (const [kind, name, ast] of writes(spec)) {
      if (!writable(element, kind)) continue;
      add(() => write(element, kind, name, CONDITIONS.has(kind) ? truth(ast) : evaluate(ast, ctx)));
    }
    const { bind } = spec;
    if (bind !== undefined) add(() => this.#rt.bindings.controls.write(element, bind, ctx));
  }

  /** Adds the effects and the signal watches of `spec`, which run statements. */
  #statements(spec: Spec, ctx: Context, add: (body: () => void) => Effect): void {
    const { statements } = this.#rt.bindings;
    const { store } = this.#rt;
    for (const stmt of spec.effects) add(() => statements.runTracked(stmt, ctx));
    for (const [place, stmt] of spec.watches) {
      let first = true;
      add(() => {
        store.getAs(resolve(place, ctx), resolveRead(place, ctx));
        if (!first) statements.run(stmt, ctx);
        first = false;
      });
    }
  }
}

/** Gives `element`, which the server rendered without them, the node ref and file input names of its template element `source`. */
function copyNames(element: Element, source: Element): void {
  for (const attribute of NAMES) {
    const name = source.getAttribute(attribute);
    if (name !== null && !element.hasAttribute(attribute)) element.setAttribute(attribute, name);
  }
}
