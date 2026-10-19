import type { Slot } from "../eval.ts";
import type { Runtime } from "../runtime.ts";
import { own, state } from "./element.ts";
import { data } from "./spec.ts";

/** The copy each `data-hs-if` template shows after itself while its condition holds. */
export class Conditions {
  readonly #rt: Runtime;
  readonly #mounted = new WeakMap<Element, Element>();

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /** The copy shown after `template`, if any. */
  mounted(template: Element): Element | undefined {
    return this.#mounted.get(template);
  }

  /** The copy drawn after `template`, as a set, where it shows one. */
  drawn(template: HTMLTemplateElement): ReadonlySet<Element> | undefined {
    const mounted = this.#mounted.get(template);
    return mounted === undefined ? undefined : new Set([mounted]);
  }

  /** Shows a copy of `proto` after `template` while `visible`, and removes it otherwise. */
  render(
    template: HTMLTemplateElement,
    proto: Element | null,
    visible: boolean,
    scope: readonly Slot[],
  ): void {
    if (this.#adopt(template, proto, visible, scope)) return;
    const current = this.#mounted.get(template);
    if (visible && (current === undefined || !current.isConnected)) {
      if (proto === null || template.parentNode === null) return;
      const element = proto.cloneNode(true) as Element;
      this.#mounted.set(template, element);
      state(element).root = true;
      template.after(element);
      this.#rt.bindings.attributes.mount(element, proto, scope, false);
    } else if (!visible && current !== undefined) {
      this.#rt.render.sweep.unmount(current);
      this.#mounted.delete(template);
    }
  }

  /**
   * Takes over the copy the server rendered after `template` on the first
   * render: it keeps it where the condition holds and removes it otherwise.
   * Returns whether it kept it.
   */
  #adopt(
    template: HTMLTemplateElement,
    proto: Element | null,
    visible: boolean,
    scope: readonly Slot[],
  ): boolean {
    const rendered = template.nextElementSibling;
    if (
      this.#mounted.has(template) ||
      rendered === null ||
      data(rendered).hsSsr !== "" ||
      own(rendered)?.root === true
    )
      return false;
    state(rendered).root = true;
    if (visible && proto !== null) {
      this.#mounted.set(template, rendered);
      this.#rt.bindings.attributes.mount(rendered, proto, scope, true);
      return true;
    }
    this.#rt.render.sweep.unmount(rendered);
    return false;
  }
}
