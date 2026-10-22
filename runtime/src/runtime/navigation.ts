/** The URL the page shows, and whether the browser is loading another page. */
export class Navigation {
  /** Whether the browser is loading another page, after which server events are not applied. */
  leaving = false;
  /** The path and query of the URL the page shows, which `history` changes without loading a page. */
  #shown = location.pathname + location.search;

  /** Loads the page at `url`, as following a link to it would. */
  navigate(url: string): void {
    this.leaving = true;
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
    this.leaving = true;
    location.reload();
  }
}
