/**
 * Whether the browser shows `browser` as it shows `server`. A number is compared
 * as `String` writes it, which is the text a binding shows and the digits
 * `JSON.stringify` sends, so -0 and 0 are the same value. A difference that a
 * later operation would expose, such as `1 / x`, sits inside the same tree and
 * shows in its value.
 */
export function same(server: unknown, browser: unknown): boolean {
  if (typeof server === "number")
    return typeof browser === "number" && String(server) === String(browser);
  if (Array.isArray(server))
    return (
      Array.isArray(browser) &&
      server.length === browser.length &&
      server.every((item, index) => same(item, browser[index]))
    );
  if (server !== null && typeof server === "object") {
    if (browser === null || typeof browser !== "object" || Array.isArray(browser)) return false;
    const fields = browser as Record<string, unknown>;
    const names = Object.keys(server);
    return (
      names.length === Object.keys(fields).length &&
      names.every(
        (name) => name in fields && same((server as Record<string, unknown>)[name], fields[name]),
      )
    );
  }
  return server === browser;
}
