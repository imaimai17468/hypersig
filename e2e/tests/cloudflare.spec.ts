import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test("counts in the browser, doubles on the worker, and receives the worker's clock", async ({ page }) => {
  await page.goto(url("cloudflare"));
  const count = page.locator("span");
  await expect(count).toHaveText("0");

  for (let i = 0; i < 3; i++) await page.getByRole("button", { name: "+1" }).click();
  await expect(count).toHaveText("3");
  await page.getByRole("button", { name: "Double on the worker" }).click();
  await expect(count).toHaveText("6");

  const clock = page.locator("p").last();
  const first = await clock.textContent();
  await expect(clock).not.toHaveText(first ?? "");
});
