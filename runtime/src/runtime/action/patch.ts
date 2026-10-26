import { addRules } from "../../css.ts";
import { run } from "../../eval.ts";
import { morph } from "../../morph.ts";
import type { ServerEvent } from "../../sse.ts";
import type { Values } from "../../store/value.ts";
import type { Runtime } from "../../runtime.ts";
import { inCopy, scopeOf } from "../element.ts";
import { NAMES } from "../binding/ref.ts";
import { upgradeTemplates } from "../binding/template.ts";

/** The events an action's answer streams, which change the store, the document, or the URL. */
export class Patches {
  readonly #rt: Runtime;

  constructor(rt: Runtime) {
    this.#rt = rt;
  }

  /**
   * Applies one server event. An event whose data does not parse, or whose
   * patch fails, is reported and skipped, so the events after it still apply.
   */
  apply(event: ServerEvent): void {
    if (this.#rt.navigation.leaving) return;
    try {
      this.#apply(event);
    } catch (error) {
      console.error(`hypersig: the server event "${event.event}" failed:`, error);
    }
  }

  #apply(event: ServerEvent): void {
    const rt = this.#rt;
    switch (event.event) {
      case "hs-signals":
        rt.store.merge(JSON.parse(event.data) as Values);
        return;
      case "hs-list":
        run(JSON.parse(event.data), { store: rt.store, scope: [] }, rt);
        return;
      case "hs-element":
        this.#element(JSON.parse(event.data) as { html: string; css?: [string, string][] });
        return;
      case "hs-navigate":
        rt.navigation.navigate((JSON.parse(event.data) as { url: string }).url);
        return;
      case "hs-history": {
        const { url, replace } = JSON.parse(event.data) as { url: string; replace: boolean };
        rt.navigation.history(url, replace);
        return;
      }
      default:
        console.warn(`hypersig: ignored server event "${event.event}"`);
    }
  }

  /** Morphs the element whose id the root of `html` carries into it, and registers what it added. */
  #element({ html, css }: { html: string; css?: [string, string][] }): void {
    const rt = this.#rt;
    if (css !== undefined) addRules(css);
    const template = document.createElement("template");
    template.innerHTML = html;
    upgradeTemplates(template.content);
    const next = template.content.firstElementChild;
    if (next === null || next.id === "") {
      console.error("hypersig: an element patch needs a root element with an id");
      return;
    }
    const target = document.querySelector(`#${CSS.escape(next.id)}`);
    if (target === null) {
      console.error(`hypersig: no element with id "${next.id}" to patch`);
      return;
    }
    const removed: Element[] = [];
    const morphed = morph(
      target,
      next,
      removed,
      (kept) => rt.lists.drawn(kept) ?? rt.conditions.drawn(kept),
    );
    rt.bindings.attributes.scan(morphed, scopeOf(morphed.parentElement));
    this.#warnDuplicates(morphed);
    rt.render.run(new Map());
    rt.render.sweep.patched(removed);
  }

  /**
   * Warns about an id or a name inside `element` that another element the
   * runtime can reach also carries. The server checks a page when it renders
   * it, but a patched element is rendered alone, so only the browser sees it
   * next to the rest of the page.
   */
  #warnDuplicates(element: Element): void {
    for (const el of [element, ...element.querySelectorAll("*")]) {
      if (el.id !== "" && document.querySelectorAll(`[id="${CSS.escape(el.id)}"]`).length > 1) {
        console.warn(
          `hypersig: the patch for #${element.id} adds a second element with the id "${el.id}"`,
        );
      }
      for (const attribute of NAMES) {
        const name = el.getAttribute(attribute);
        if (name !== null && this.#named(attribute, name, el))
          console.warn(
            `hypersig: the patch for #${element.id} adds a second element with ${attribute}="${name}"`,
          );
      }
    }
  }

  /** Whether another element the runtime can reach from `el` also has `attribute` set to `name`. */
  #named(attribute: string, name: string, el: Element): boolean {
    const selector = `[${attribute}="${CSS.escape(name)}"]`;
    const onPage = Array.from(this.#rt.root.querySelectorAll(selector)).filter(
      (other) => !inCopy(other),
    );
    // A lookup from inside a list item meets the page's elements too, so
    // one page element is too many there, and two are too many outside.
    return (
      this.#rt.bindings.refs.find(attribute, name, el) !== el ||
      onPage.length > (inCopy(el) ? 0 : 1)
    );
  }
}
