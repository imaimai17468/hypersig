/**
 * Measures where `element` sits in its scroll container, and returns the
 * function that scrolls the container so that `element` is back there.
 *
 * Chrome's scroll anchoring (`overflow-anchor`) keeps the content in view in
 * place when items are added above it, but not while the container is
 * scrolled to the very top, which is where a list that loads older items on
 * scrolling up is when it loads them. Where the browser has already
 * compensated, `element` has not moved, and nothing is scrolled.
 */
export function keepInView(element: Element): () => void {
  const scroller = scrollerOf(element);
  const before = element.getBoundingClientRect().top;
  return () => {
    const moved = element.getBoundingClientRect().top - before;
    if (moved !== 0) scroller.scrollTop += moved;
  };
}

/** The nearest ancestor of `element` that scrolls vertically, or the document's scroller. */
function scrollerOf(element: Element): Element {
  for (let at = element.parentElement; at !== null; at = at.parentElement) {
    const overflow = getComputedStyle(at).overflowY;
    if (overflow === "auto" || overflow === "scroll") return at;
  }
  return document.scrollingElement ?? document.documentElement;
}
