import { test as base } from "@playwright/test";
import { url } from "../playwright.config.ts";
import { expect, test, watch } from "./support.ts";

const app = `${url("prefix")}app`;

test("follows links, calls an action, and loads the runtime under the prefix", async ({ page }) => {
  const runtime = page.waitForResponse((response) => response.url().includes("/_hypersig/runtime."));
  await page.goto(url("prefix"));
  await expect(page).toHaveURL(app);
  const loaded = await runtime;
  expect(new URL(loaded.url()).pathname).toMatch(/^\/app\/_hypersig\/runtime\.[0-9a-f]{16}\.js$/);
  expect(loaded.headers()["cache-control"]).toBe("public, max-age=31536000, immutable");

  await page.getByRole("link", { name: "Coffee" }).click();
  await expect(page).toHaveURL(`${app}/items/1`);
  await expect(page.locator("h1")).toHaveText("Coffee");
  const cart = page.locator("#cart span");
  const before = Number(await cart.textContent());
  await page.getByRole("button", { name: "Add to cart" }).click();
  await expect(cart).toHaveText(String(before + 1));

  await page.getByRole("link", { name: "All items" }).click();
  await expect(page).toHaveURL(app);
  await expect(page.locator("#items li")).toHaveCount(3);
});

// The page answers 404 on purpose, so this test checks the failures it logs itself.
base("shows the shop's 404 page for a path under the prefix that names no page", async ({ page }) => {
  const problems = watch(page);
  const response = await page.goto(`${app}/items/9`);
  expect(response?.status()).toBe(404);
  await expect(page.locator("h1")).toHaveText("Not found");
  await expect(page.locator("#problem")).toHaveText("the handler of `/items/9` answered with no page");
  await page.getByRole("link", { name: "All items" }).click();
  await expect(page).toHaveURL(app);
  expect(problems.filter((problem) => !problem.startsWith("404") && !problem.startsWith("Failed to load resource"))).toEqual([]);
});
