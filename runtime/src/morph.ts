/**
 * Updates `from` in place until it matches `to`, keeping elements that are
 * already there so focus and scroll position survive. Children with an `id`
 * are matched by id, the rest by position. A `<template>` whose content
 * changed is replaced whole, because its content is not among its children.
 * Each element it takes out of the document is pushed onto `removed`. One
 * of them can be back in the document by the time `morph` returns, where a
 * later child with the same id moved it there.
 */
export function morph(from: Element, to: Element, removed: Element[] = []): Element {
  if (
    from.tagName !== to.tagName ||
    (from instanceof HTMLTemplateElement && from.innerHTML !== (to as HTMLTemplateElement).innerHTML)
  ) {
    from.replaceWith(to);
    removed.push(from);
    return to;
  }
  syncAttributes(from, to);
  morphChildren(from, to, removed);
  return from;
}

function syncAttributes(from: Element, to: Element): void {
  for (const { name } of [...from.attributes]) {
    if (!to.hasAttribute(name)) from.removeAttribute(name);
  }
  for (const { name, value } of [...to.attributes]) {
    if (from.getAttribute(name) !== value) from.setAttribute(name, value);
  }
}

function morphChildren(from: Element, to: Element, removed: Element[]): void {
  const byId = new Map<string, Element>();
  for (const child of from.children) if (child.id !== "") byId.set(child.id, child);

  const targets = [...to.childNodes];
  targets.forEach((next, index) => {
    let current: ChildNode | null = from.childNodes[index] ?? null;
    if (next instanceof Element && next.id !== "") {
      const match = byId.get(next.id);
      if (match !== undefined && match !== current) {
        from.insertBefore(match, current);
        current = match;
      }
    }
    if (current === null) {
      from.appendChild(next);
    } else if (current instanceof Element && next instanceof Element && current.id === next.id) {
      morph(current, next, removed);
    } else if (current.nodeType === Node.TEXT_NODE && next.nodeType === Node.TEXT_NODE) {
      if (current.textContent !== next.textContent) current.textContent = next.textContent;
    } else {
      current.replaceWith(next);
      if (current instanceof Element) removed.push(current);
    }
  });
  while (from.childNodes.length > targets.length) {
    const last = from.lastChild!;
    last.remove();
    if (last instanceof Element) removed.push(last);
  }
}
