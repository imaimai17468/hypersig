/**
 * Updates `from` in place until it matches `to`, keeping elements that are
 * already there so focus and scroll position survive. Children with an `id`
 * are matched by id, the rest by position.
 */
export function morph(from: Element, to: Element): Element {
  if (from.tagName !== to.tagName) {
    from.replaceWith(to);
    return to;
  }
  syncAttributes(from, to);
  morphChildren(from, to);
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

function morphChildren(from: Element, to: Element): void {
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
      morph(current, next);
    } else if (current.nodeType === Node.TEXT_NODE && next.nodeType === Node.TEXT_NODE) {
      if (current.textContent !== next.textContent) current.textContent = next.textContent;
    } else {
      current.replaceWith(next);
    }
  });
  while (from.childNodes.length > targets.length) from.lastChild?.remove();
}
