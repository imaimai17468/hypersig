import type { Spec } from "./spec.ts";

/** What a binding writes to its element. */
export type Write = "show" | "text" | "checked" | "class" | "attr" | "toggle" | "style";

/** The kinds of binding whose expression is a condition, which a comparison or a selector part can answer. */
export const CONDITIONS: ReadonlySet<string> = new Set(["show", "class", "toggle", "if"]);

/**
 * The attributes whose binding holds a `SafeUrl`, which `El::href`,
 * `El::src`, and `El::poster` write and no `Attr` can name.
 */
const URLS: ReadonlySet<string> = new Set(["href", "src", "poster"]);

/**
 * Whether `url` is one a `SafeUrl` can hold, the rule of `check` in
 * `crates/hypersig/src/href.rs`. A `SafeUrl` the server sent passed it
 * already, and one the browser built, such as the `detail` of a custom
 * event, reaches the binding unchecked.
 */
export function safeUrl(url: string): boolean {
  // The browser drops spaces and control characters around and inside the
  // scheme before reading it, so `java\tscript:` runs as `javascript:`.
  for (const char of url) if (char <= " " || char === "\u007F") return false;
  const scheme = /^[a-z][a-z0-9+.-]*:/iu.exec(url);
  return scheme === null || /^(?:https?|mailto):$/iu.test(scheme[0]);
}

/** The bindings of `spec` that write to the element, in the order the runtime runs them. */
export function writes(spec: Spec): [kind: Write, name: string, ast: unknown][] {
  const found: [Write, string, unknown][] = [];
  if (spec.show !== undefined) found.push(["show", "", spec.show]);
  if (spec.text !== undefined) found.push(["text", "", spec.text]);
  if (spec.checked !== undefined) found.push(["checked", "", spec.checked]);
  for (const [name, when] of spec.classes) found.push(["class", name, when]);
  for (const [name, expr] of spec.attrs) found.push(["attr", name, expr]);
  for (const [name, when] of spec.toggles) found.push(["toggle", name, when]);
  for (const [name, expr] of spec.styles) found.push(["style", name, expr]);
  return found;
}

/** Whether a binding of `kind` can write to `element`: `show` and `style` need an inline style, `checked` an input. */
export function writable(element: Element, kind: Write): boolean {
  if (kind === "show" || kind === "style")
    return element instanceof HTMLElement || element instanceof SVGElement;
  return kind !== "checked" || element instanceof HTMLInputElement;
}

/** Writes `value` to `element` as a binding of `kind` does, where it differs from what the element shows and `writable` holds. */
export function write(element: Element, kind: Write, name: string, value: unknown): void {
  if (!writable(element, kind)) return;
  switch (kind) {
    case "show": {
      const styled = element as HTMLElement;
      // The server marks an element it hid with `data-hs-hide` in place of
      // an inline style, and from here on `style.display` decides.
      delete styled.dataset.hsHide;
      const next = value ? "" : "none";
      if (styled.style.display !== next) styled.style.display = next;
      return;
    }
    case "text": {
      const next = String(value);
      if (element.textContent !== next) element.textContent = next;
      return;
    }
    case "checked":
      (element as HTMLInputElement).checked = Boolean(value);
      return;
    case "class":
      if (element.classList.contains(name) !== value)
        element.classList.toggle(name, Boolean(value));
      return;
    case "attr": {
      const next = String(value);
      if (URLS.has(name) && !safeUrl(next)) element.removeAttribute(name);
      else if (element.getAttribute(name) !== next) element.setAttribute(name, next);
      return;
    }
    case "toggle":
      if (element.hasAttribute(name) !== value) element.toggleAttribute(name, Boolean(value));
      return;
    case "style": {
      const styled = element as HTMLElement;
      // On a page with a CSP nonce, the server writes the element's values
      // as a rule selected by `data-hs-inline` in place of an inline style.
      // The rule is `!important`, so it goes once the inline style takes over.
      delete styled.dataset.hsInline;
      styled.style.setProperty(name, String(value));
    }
  }
}
