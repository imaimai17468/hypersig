import { test as base, expect, type Page } from "@playwright/test";

/** What the page logs for a style WebKit refuses on a `<select>` of its own accord. */
const SELECT_STYLE = "hypersig-e2e: WebKit refused its own inline style on a <select>";

/**
 * Records console errors, page errors, and 4xx/5xx responses on a page.
 *
 * WebKit draws a `<select>` with an inline style of its own, which a
 * `style-src` of nonces alone refuses: it logs "Refused to apply a
 * stylesheet" and fires a `style-src-attr` violation at the `<select>`,
 * whose own `style` attribute is unset. That error is left out, and any
 * other stays.
 */
export function watch(page: Page): string[] {
  const problems: string[] = [];
  void page.addInitScript((marker) => {
    document.addEventListener("securitypolicyviolation", (event) => {
      const target = event.target;
      if (event.effectiveDirective === "style-src-attr" && target instanceof HTMLSelectElement && !target.hasAttribute("style"))
        console.debug(marker);
    });
  }, SELECT_STYLE);
  page.on("console", (message) => {
    if (message.text() === SELECT_STYLE) {
      const refused = problems.map((problem) => problem.startsWith("Refused to apply a stylesheet")).lastIndexOf(true);
      if (refused !== -1) problems.splice(refused, 1);
      return;
    }
    if (message.type() === "error" || message.type() === "warning") problems.push(message.text());
  });
  page.on("pageerror", (error) => problems.push(String(error)));
  page.on("response", (response) => {
    if (response.status() >= 400) problems.push(`${response.status()} ${response.url()}`);
  });
  return problems;
}

/** `test` whose `page` fails the test if the page logged an error or got an error response. */
export const test = base.extend({
  page: async ({ page }, use) => {
    const problems = watch(page);
    await use(page);
    expect(problems).toEqual([]);
  },
});

export { expect };
