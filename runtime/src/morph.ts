/**
 * The elements the runtime drew right after a `<template>`: the items of a
 * list or the content of a condition. A server patch carries the template
 * alone, so `morph` keeps these where it keeps their template.
 */
export type Drawn = (template: HTMLTemplateElement) => ReadonlySet<Element> | undefined;

/**
 * Updates `from` in place until it matches `to`, keeping elements that are
 * already there so focus and scroll position survive. Children with an `id`
 * are matched by id, the rest by position. A `<template>` whose content
 * changed is replaced whole, because its content is not among its children.
 * One that stays keeps the elements `drawn` names for it, which `to` does
 * not hold. Each element it takes out of the document is pushed onto
 * `removed`. One of them can be back in the document by the time `morph`
 * returns, where a later child with the same id moved it there.
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
  syncAttributes(from, to);
  // A `<template>` that stays has the same content, which `morphChildren` must not walk as children.
  if (!(from instanceof HTMLTemplateElement)) morphChildren(from, to, removed, drawn);
  return from;
}

function syncAttributes(from: Element, to: Element): void {
  // A copy, because removing an attribute changes `from.attributes` while it is walked.
  for (const { name } of Array.from(from.attributes)) {
    if (!to.hasAttribute(name)) from.removeAttribute(name);
  }
  for (const { name, value } of to.attributes) {
    if (from.getAttribute(name) !== value) from.setAttribute(name, value);
  }
}

function morphChildren(
  from: Element,
  to: Element,
  removed: Element[],
  drawn: Drawn | undefined,
): void {
  const byId = new Map<string, Element>();
  for (const child of from.children) if (child.id !== "") byId.set(child.id, child);

  let current: ChildNode | null = from.firstChild;
  // A copy, because each child moved into `from` leaves `to.childNodes`.
  for (const next of Array.from(to.childNodes)) {
    if (next instanceof Element && next.id !== "") {
      const match = byId.get(next.id);
      if (match !== undefined && match !== current) {
        from.insertBefore(match, current);
        current = match;
      }
    }
    let kept: ChildNode;
    if (current === null) {
      from.append(next);
      kept = next;
    } else if (current instanceof Element && next instanceof Element && current.id === next.id) {
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
