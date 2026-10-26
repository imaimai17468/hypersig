/**
 * The attribute `El::foreign` writes, naming an element whose attributes and
 * children a script owns. `morph` keeps such an element as it is, and
 * matches it among its siblings by that name where it has no `id`.
 */
const FOREIGN = "data-hs-foreign";

/** What `morphChildren` matches `element` among its siblings by: its id, else its foreign name, else nothing. */
function keyOf(element: Element): string {
  if (element.id !== "") return `#${element.id}`;
  const foreign = element.getAttribute(FOREIGN);
  return foreign === null ? "" : `@${foreign}`;
}

/**
 * The elements the runtime drew right after a `<template>`: the items of a
 * list or the content of a condition. A server patch carries the template
 * alone, so `morph` keeps these where it keeps their template.
 */
export type Drawn = (template: HTMLTemplateElement) => ReadonlySet<Element> | undefined;

/**
 * Updates `from` in place until it matches `to`, keeping elements that are
 * already there so focus and scroll position survive. Children with an `id`
 * are matched by id, those `El::foreign` names by that name, and the rest by
 * position, looking ahead past inserted and removed siblings for one with
 * the same tag and `name`. A `<template>` whose content changed is replaced whole, because
 * its content is not among its children. One that stays keeps the elements
 * `drawn` names for it, which `to` does not hold. A `<details>` or a
 * `<dialog>` keeps its `open` as the visitor or a script left it. Each element it takes out
 * of the document is pushed onto `removed`. One of them can be back in the
 * document by the time `morph` returns, where a later child with the same id
 * or foreign name moved it there. Where the focused element left the
 * document or lost focus on the way, focus and the text selection move to
 * the element now there with its id, or else its tag and `name`.
 */
export function morph(from: Element, to: Element, removed: Element[] = [], drawn?: Drawn): Element {
  const focus = saveFocus();
  const morphed = morphElement(from, to, removed, drawn);
  if (focus !== undefined) restoreFocus(focus, morphed);
  return morphed;
}

/** The element that had focus before a morph, and its text selection where it has one. */
interface Focus {
  element: Element;
  selection: [number, number, "forward" | "backward" | "none"] | undefined;
}

function saveFocus(): Focus | undefined {
  const element = document.activeElement;
  if (element === null || element === document.body) return undefined;
  let selection: Focus["selection"];
  // `selectionStart` is null on an input whose type has no text selection, such as `checkbox`.
  if (
    (element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement) &&
    element.selectionStart !== null &&
    element.selectionEnd !== null
  )
    selection = [
      element.selectionStart,
      element.selectionEnd,
      element.selectionDirection ?? "none",
    ];
  return { element, selection };
}

/**
 * Focuses the element that stands for `focus.element` once `root` is
 * morphed, where focus fell to the body. Moving or replacing the focused
 * element takes focus away from it in the browser.
 */
function restoreFocus({ element, selection }: Focus, root: Element): void {
  const active = document.activeElement;
  if (active !== null && active !== document.body) return;
  const target = element.isConnected ? element : standIn(element, root);
  if (!(target instanceof HTMLElement || target instanceof SVGElement)) return;
  target.focus({ preventScroll: true });
  if (
    selection !== undefined &&
    (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement)
  )
    target.setSelectionRange(...selection);
}

/** The element in `root`, or `root` itself, with the id of `gone`, or else its tag and `name`. */
function standIn(gone: Element, root: Element): Element | undefined {
  const name = gone.getAttribute("name");
  let selector: string;
  if (gone.id !== "") selector = `#${CSS.escape(gone.id)}`;
  else if (name === null) return undefined;
  else selector = `${gone.localName}[name="${CSS.escape(name)}"]`;
  return root.matches(selector) ? root : (root.querySelector(selector) ?? undefined);
}

function morphElement(from: Element, to: Element, removed: Element[], drawn?: Drawn): Element {
  if (
    from.tagName !== to.tagName ||
    (from instanceof HTMLTemplateElement &&
      from.innerHTML !== (to as HTMLTemplateElement).innerHTML)
  ) {
    from.replaceWith(to);
    removed.push(from);
    return to;
  }
  // A script set the attributes and children it has, and the page renders only the element itself.
  if (from.hasAttribute(FOREIGN) && to.getAttribute(FOREIGN) === from.getAttribute(FOREIGN))
    return from;
  syncAttributes(from, to);
  // A `<template>` that stays has the same content, which `morphChildren` must not walk as children.
  if (!(from instanceof HTMLTemplateElement)) morphChildren(from, to, removed, drawn);
  return from;
}

/**
 * Whether the browser owns attribute `name` of `element`: `open` on a
 * `<details>` the visitor toggles and on a `<dialog>` that `show`,
 * `showModal`, and `close` set. Taking `open` off a modal dialog hides it
 * without `close`, which leaves the rest of the page inert.
 */
function browserOwned(element: Element, name: string): boolean {
  return (
    name === "open" &&
    (element instanceof HTMLDetailsElement || element instanceof HTMLDialogElement)
  );
}

function syncAttributes(from: Element, to: Element): void {
  // A copy, because removing an attribute changes `from.attributes` while it is walked.
  for (const { name } of Array.from(from.attributes)) {
    if (!to.hasAttribute(name) && !browserOwned(from, name)) from.removeAttribute(name);
  }
  for (const { name, value } of to.attributes) {
    if (from.getAttribute(name) !== value && !browserOwned(from, name))
      from.setAttribute(name, value);
  }
}

function morphChildren(
  from: Element,
  to: Element,
  removed: Element[],
  drawn: Drawn | undefined,
): void {
  const byKey = new Map<string, Element>();
  for (const child of from.children) {
    const key = keyOf(child);
    if (key !== "") byKey.set(key, child);
  }

  let current: ChildNode | null = from.firstChild;
  // A copy, because each child moved into `from` leaves `to.childNodes`.
  for (const next of Array.from(to.childNodes)) {
    const key = next instanceof Element ? keyOf(next) : "";
    if (key !== "") {
      const match = byKey.get(key);
      if (match !== undefined && match !== current) {
        from.insertBefore(match, current);
        current = match;
      }
    }
    let kept: ChildNode;
    if (current === null) {
      from.append(next);
      kept = next;
    } else kept = morphChild(from, current, next, key, removed, drawn);
    let after = kept.nextSibling;
    const own = kept === current && kept instanceof HTMLTemplateElement ? drawn?.(kept) : undefined;
    if (own !== undefined)
      while (after instanceof Element && own.has(after)) after = after.nextSibling;
    current = after;
  }
  const rest: ChildNode[] = [];
  for (; current !== null; current = current.nextSibling) rest.push(current);
  for (const last of rest.toReversed()) {
    last.remove();
    if (last instanceof Element) removed.push(last);
  }
}

/** Whether `a` and `b`, neither of which has a key, stand for one element: the same tag and `name`. */
function like(a: Element, b: Element): boolean {
  return a.tagName === b.tagName && a.getAttribute("name") === b.getAttribute("name");
}

/**
 * The first element from `start` on that has no key and is `like` `model`.
 * The search stops at a `<template>`, because the elements the runtime drew
 * after one belong to it rather than to the siblings around it.
 */
function ahead(start: ChildNode | null, model: Element): Element | null {
  if (keyOf(model) !== "" || model instanceof HTMLTemplateElement) return null;
  for (let node = start; node !== null; node = node.nextSibling) {
    if (!(node instanceof Element)) continue;
    if (node instanceof HTMLTemplateElement) return null;
    if (keyOf(node) === "" && like(node, model)) return node;
  }
  return null;
}

/**
 * Morphs `current`, the child of `from` at the place of `next`, into
 * `next`, whose key is `key`, and returns the node now at that place.
 */
function morphChild(
  from: Element,
  current: ChildNode,
  next: ChildNode,
  key: string,
  removed: Element[],
  drawn: Drawn | undefined,
): ChildNode {
  if (current instanceof Element && next instanceof Element && keyOf(current) === key) {
    if (key !== "" || like(current, next)) return morphElement(current, next, removed, drawn);
  } else if (current.nodeType === Node.TEXT_NODE && next.nodeType === Node.TEXT_NODE) {
    if (current.textContent !== next.textContent) current.textContent = next.textContent;
    return current;
  }
  const match = next instanceof Element ? ahead(current, next) : null;
  if (match !== null && next instanceof Element) {
    // The siblings before the match left `to`, and the loop after it removes them.
    from.insertBefore(match, current);
    return morphElement(match, next, removed, drawn);
  }
  if (current instanceof Element && ahead(next.nextSibling, current) !== null) {
    // `next` is new before an element `to` still has, which keeps its place and its state.
    from.insertBefore(next, current);
    return next;
  }
  if (current instanceof Element && next instanceof Element && keyOf(current) === key)
    return morphElement(current, next, removed, drawn);
  current.replaceWith(next);
  if (current instanceof Element) removed.push(current);
  return next;
}
