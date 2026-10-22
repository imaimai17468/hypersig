import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test("counts in the browser, doubles on the function, and receives the function's clock", async ({ page }) => {
  await page.goto(url("vercel"));
  const count = page.locator("span");
  await expect(count).toHaveText("0");

  for (let i = 0; i < 3; i++) await page.getByRole("button", { name: "+1" }).click();
  await expect(count).toHaveText("3");
  await page.getByRole("button", { name: "Double on the function" }).click();
  await expect(count).toHaveText("6");

  const clock = page.locator("p").last();
  const first = await clock.textContent();
  await expect(clock).not.toHaveText(first ?? "");
});
