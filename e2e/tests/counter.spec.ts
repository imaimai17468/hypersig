import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test("counts in the browser, saves to the server, and resets from the server", async ({ page }) => {
  await page.goto(url("counter"));
  const count = page.locator(".count");
  await expect(count).toHaveText("0");
  await expect(page.getByRole("button", { name: "Reset" })).toBeHidden();

  for (let i = 0; i < 11; i++) await page.getByRole("button", { name: "+" }).click();
  await expect(count).toHaveText("11");
  await expect(count).toHaveClass(/big/);
  await page.getByPlaceholder("why this number?").fill("demo");
  await expect(page.locator("p").first()).toHaveText("demo → 11");

  await page.getByRole("button", { name: "Save to server" }).click();
  await expect(page.locator(".status")).toHaveText("Saved 11 (demo).");

  await page.getByRole("button", { name: "Reset" }).click();
  await expect(count).toHaveText("0");
  await expect(page.locator(".status")).toHaveText("Reset by the server.");
});
