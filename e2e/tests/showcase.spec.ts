import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test.beforeEach(async ({ page }) => {
  await page.goto(url("showcase"));
});

test("each placement of a component keeps its own state", async ({ page }) => {
  const bodies = page.locator(".disclosure p");
  await expect(bodies.nth(0)).toBeHidden();
  await expect(bodies.nth(1)).toBeVisible();
  await page.locator(".disclosure-title").getByText("Shipping", { exact: true }).click();
  await page.locator(".disclosure-title").getByText("Returns", { exact: true }).click();
  await expect(bodies.nth(0)).toBeVisible();
  await expect(bodies.nth(1)).toBeHidden();
  await expect(page.locator(".disclosure-title").nth(0)).toHaveAttribute("aria-expanded", "true");

  const stepper = (i: number) => page.locator(".stepper").nth(i);
  await expect(stepper(1).getByLabel("fewer")).toBeDisabled();
  await stepper(1).getByLabel("more").click();
  await stepper(0).getByLabel("fewer").click();
  await expect(page.locator(".total")).toHaveText("2 guests");
  await expect(stepper(0).getByLabel("fewer")).toBeDisabled();
});

test("an interval runs while started", async ({ page }) => {
  await page.locator(".start").click();
  await expect(page.locator(".start")).toHaveText("Stop");
  await expect(page.locator(".reset")).toBeDisabled();
  await expect.poll(async () => parseFloat((await page.locator(".elapsed").textContent()) ?? "0")).toBeGreaterThan(0.3);
  await page.locator(".start").click();
  const stopped = await page.locator(".elapsed").textContent();
  await page.waitForTimeout(300);
  await expect(page.locator(".elapsed")).toHaveText(stopped ?? "");
  await page.locator(".reset").click();
  await expect(page.locator(".elapsed")).toHaveText("0.0 s");
});

test("search waits for a pause in typing and shows an indicator", async ({ page }) => {
  const field = page.getByPlaceholder("Search fruit");
  await page.locator(".focus").click();
  await expect(field).toBeFocused();
  const searches: string[] = [];
  page.on("request", (request) => {
    if (request.url().endsWith("/search")) searches.push(request.url());
  });
  await page.keyboard.type("berr", { delay: 30 });
  await expect(page.locator(".searching")).toBeVisible();
  await expect(page.locator("#results li")).toHaveText(["blackberry", "blueberry"]);
  await expect(page.locator(".searching")).toBeHidden();
  expect(searches).toHaveLength(1);
  await page.keyboard.press("Escape");
  await expect(field).toHaveValue("");
  await expect(page.locator("#results li")).toHaveCount(13);
});

test("a form derives its total and disables submit while invalid or in flight", async ({ page }) => {
  await page.locator(".quantity").fill("3");
  await expect(page.locator(".price")).toHaveText("9.00 USD");
  await page.locator(".size").selectOption({ label: "Large" });
  await expect(page.locator(".price")).toHaveText("13.50 USD");
  await page.locator(".gift").check();
  await page.locator(".note").fill("x".repeat(41));
  await expect(page.locator(".count")).toHaveText("41 / 40");
  await expect(page.locator(".place")).toBeDisabled();
  await page.locator(".note").fill("ribbon 🎀");
  await expect(page.locator(".count")).toHaveText("8 / 40");
  await page.locator(".place").click();
  await expect(page.locator(".place")).toHaveText("Placing…");
  await expect(page.locator(".status")).toHaveText("Ordered 3 large, gift wrapped.");
  await expect(page.locator(".place")).toHaveText("Place order");
});
