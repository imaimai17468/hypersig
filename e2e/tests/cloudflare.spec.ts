import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test("counts in the browser, doubles on the worker, and receives the worker's clock", async ({ page }) => {
  await page.goto(url("cloudflare"));
  const count = page.locator("span");
  // stage.sh writes the project afresh, without the `.wrangler` that holds
  // the local D1 database, so nothing is saved yet.
  await expect(count).toHaveText("0");

  for (let i = 0; i < 3; i++) await page.getByRole("button", { name: "+1" }).click();
  await expect(count).toHaveText("3");
  await page.getByRole("button", { name: "Double on the worker" }).click();
  await expect(count).toHaveText("6");

  const clock = page.locator("p").last();
  const first = await clock.textContent();
  await expect(clock).not.toHaveText(first ?? "");
});

test("saves the count in D1, and the page reads it back on the next load", async ({ page }) => {
  await page.goto(url("cloudflare"));
  const count = page.locator("span");
  for (let i = 0; i < 2; i++) await page.getByRole("button", { name: "+1" }).click();
  const saved = page.waitForResponse((response) => response.url().endsWith("/save"));
  await page.getByRole("button", { name: "Save in D1" }).click();
  expect((await saved).status()).toBe(200);
  await expect(page.getByRole("alert")).toBeHidden();

  await page.reload();
  await expect(count).toHaveText("2");
});

// `wrangler dev` runs the worker in one isolate, so this drives the path
// through the hub that a save takes to the other isolates of a deployment,
// with both tabs served by the same isolate.
test("a save in one tab shows in another tab without a reload", async ({ page, context }) => {
  const other = await context.newPage();
  await page.goto(url("cloudflare"));
  await other.goto(url("cloudflare"));

  // The stream of `other` reads the count only after it subscribes, so once
  // the first save shows there the second can arrive only through the hub.
  for (const clicks of [5, 3]) {
    for (let i = 0; i < clicks; i++) await page.getByRole("button", { name: "+1" }).click();
    const count = (await page.locator("span").textContent()) ?? "";
    await page.getByRole("button", { name: "Save in D1" }).click();
    await expect(page.locator("strong")).toHaveText(count);
    await expect(other.locator("strong")).toHaveText(count);
  }
});
