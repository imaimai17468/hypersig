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
 * position. A `<template>` whose content changed is replaced whole, because
 * its content is not among its children. One that stays keeps the elements
 * `drawn` names for it, which `to` does not hold. A `<details>` or a
 * `<dialog>` keeps its `open` as the visitor or a script left it. Each element it takes out
 * of the document is pushed onto `removed`. One of them can be back in the
 * document by the time `morph` returns, where a later child with the same id
 * or foreign name moved it there.
 */
export function morph(from: Element, to: Element, removed: Element[] = [], drawn?: Drawn): Element {
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
    } else if (current instanceof Element && next instanceof Element && keyOf(current) === key) {
      kept = morph(current, next, removed, drawn);
    } else if (current.nodeType === Node.TEXT_NODE && next.nodeType === Node.TEXT_NODE) {
      if (current.textContent !== next.textContent) current.textContent = next.textContent;
      kept = current;
    } else {
      current.replaceWith(next);
      if (current instanceof Element) removed.push(current);
      kept = next;
    }
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
