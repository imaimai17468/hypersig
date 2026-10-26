/**
 * How long the page waits to unload after it asks to leave, before it counts
 * the request as one that kept the page, such as a URL that answers with a
 * download or with 204 No Content.
 */
const UNLOAD_MS = 3000;

/** The URL the page shows, and whether the browser is loading another page. */
export class Navigation {
  /** Whether the browser is loading another page, after which server events are not applied. */
  leaving = false;
  /** The path and query of the URL the page shows, which `history` changes without loading a page. */
  #shown = location.pathname + location.search;

  /** Loads the page at `url`, as following a link to it would. */
  navigate(url: string): void {
    this.#leave();
    location.assign(url);
  }

  /**
   * Shows `url` in the address bar without loading it, as a new history
   * entry unless `replace`. A URL equal to the current one adds no entry.
   */
  history(url: string, replace: boolean): void {
    if (this.leaving) return;
    const next = new URL(url, document.baseURI);
    if (!replace && next.href === location.href) return;
    if (replace) history.replaceState(null, "", next);
    else history.pushState(null, "", next);
    this.#shown = next.pathname + next.search;
  }

  /**
   * Loads the page at the URL Back or Forward moved to, since the server
   * renders a page from its URL. A move between two entries of one path and
   * query, such as to a `#fragment`, keeps the page.
   */
  popped(): void {
    if (this.leaving || location.pathname + location.search === this.#shown) return;
    this.reload();
  }

  /** Loads the page at the URL it shows again. */
  reload(): void {
    this.#leave();
    location.reload();
  }

  /**
   * Stops applying server events until the page unloads. The browser fires
   * no `pagehide` for a URL it downloads or one answered with 204, and the
   * page stays, so `leaving` turns false again after `UNLOAD_MS` unless
   * `pagehide` fired by then. A page the back-forward cache restores fires
   * `pageshow` with `persisted`, and applies events again from then on.
   */
  #leave(): void {
    if (this.leaving) return;
    this.leaving = true;
    const restore = (event: PageTransitionEvent): void => {
      if (!event.persisted) return;
      removeEventListener("pageshow", restore);
      this.leaving = false;
    };
    const unload = (): void => {
      clearTimeout(stay);
      addEventListener("pageshow", restore);
    };
    const stay = setTimeout(() => {
      removeEventListener("pagehide", unload);
      this.leaving = false;
    }, UNLOAD_MS);
    addEventListener("pagehide", unload, { once: true });
  }
}
