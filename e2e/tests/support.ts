import { test as base, expect, type Page } from "@playwright/test";

/** Records console errors, page errors, and 4xx/5xx responses on a page. */
export function watch(page: Page): string[] {
  const problems: string[] = [];
  page.on("console", (message) => {
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
