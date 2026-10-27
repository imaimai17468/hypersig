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
    // `hs-end`, the last event of every answer, patches nothing, and `call.ts` reads it to tell a complete answer from a cut one.
    if (this.#rt.navigation.leaving || event.event === "hs-end") return;
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
   *
   * The page is read once per id and per attribute of `NAMES`, rather than
   * once per element, so a patch of a long list costs a pass over the page.
   */
  #warnDuplicates(element: Element): void {
    const inside = [element, ...element.querySelectorAll("*")];
    const warn = (what: string) =>
      console.warn(`hypersig: the patch for #${element.id} adds a second element with ${what}`);
    const withId = inside.filter((el) => el.id !== "");
    if (withId.length > 0) {
      const ids = count(document.querySelectorAll("[id]"), (el) => el.id);
      for (const el of withId) if (ids.get(el.id)! > 1) warn(`the id "${el.id}"`);
    }
    for (const attribute of NAMES) {
      const named = inside.filter((el) => el.hasAttribute(attribute));
      if (named.length === 0) continue;
      const all = Array.from(this.#rt.root.querySelectorAll(`[${attribute}]`));
      const first = new Map<string, Element>();
      for (const other of all) {
        const name = other.getAttribute(attribute)!;
        if (!first.has(name)) first.set(name, other);
      }
      const onPage = count(
        all.filter((other) => !inCopy(other)),
        (other) => other.getAttribute(attribute)!,
      );
      for (const el of named) {
        const name = el.getAttribute(attribute)!;
        // A lookup from inside a list item meets the page's elements too, so
        // one page element is too many there, and two are too many outside.
        if (
          this.#rt.bindings.refs.find(attribute, name, el, first) !== el ||
          (onPage.get(name) ?? 0) > (inCopy(el) ? 0 : 1)
        )
          warn(`${attribute}="${name}"`);
      }
    }
  }
}

/** How many of `elements` give each key. */
function count(elements: Iterable<Element>, key: (el: Element) => string): Map<string, number> {
  const counts = new Map<string, number>();
  for (const el of elements) counts.set(key(el), (counts.get(key(el)) ?? 0) + 1);
  return counts;
}
