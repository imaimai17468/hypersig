import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test.beforeEach(async ({ page }) => {
  await page.goto(url("style"));
});

test("a signal switches the theme's tokens and the recipe's variants", async ({ page }) => {
  const app = page.locator("#app");
  const button = page.locator("#switch-theme");
  await expect(app).toHaveCSS("background-color", "rgb(255, 255, 255)");
  await button.click();
  await expect(app).toHaveCSS("background-color", "rgb(13, 17, 23)");
  await expect(button).toHaveText("Light theme");
  await button.click();
  await expect(app).toHaveCSS("background-color", "rgb(255, 255, 255)");

  await expect(button).toHaveCSS("font-size", "14px");
  await expect(button).toHaveCSS("color", "rgb(255, 255, 255)");
  await page.locator("#large").click();
  await expect(button).toHaveCSS("font-size", "20px");
  await page.locator("#outline").click();
  await expect(button).toHaveCSS("color", "rgb(47, 111, 235)");
  await expect(button).toHaveCSS("background-color", "rgba(0, 0, 0, 0)");
});

test("the grid follows the viewport, and group and peer follow other elements", async ({ page }) => {
  const columns = () => page.locator("#cards").evaluate((el) => getComputedStyle(el).gridTemplateColumns.split(" ").length);
  await page.setViewportSize({ width: 600, height: 900 });
  await expect.poll(columns).toBe(1);
  await page.setViewportSize({ width: 1100, height: 900 });
  await expect.poll(columns).toBe(3);

  const card = page.locator("#cards > div").first();
  const action = card.getByText("Order");
  await page.mouse.move(0, 0);
  await expect(action).toBeHidden();
  await card.hover();
  await expect(action).toBeVisible();

  await expect(page.locator("#details")).toBeHidden();
  await page.getByLabel("Show details").check();
  await expect(page.locator("#details")).toBeVisible();
});

test("rules a patch brings take their place among the page's rules", async ({ page }) => {
  await expect(page.locator("#spinner")).toHaveCSS("animation-name", "spin");

  // The page has rules under `lg` and none under `md`. The patch's `md` rule
  // has to land before the page's `lg` rules, or it would win at 1100px.
  await page.setViewportSize({ width: 1100, height: 900 });
  await page.locator("#load").click();
  const loaded = page.locator("#more > div");
  await expect(loaded).toHaveCount(3);
  const share = async () => {
    const [card, row] = await Promise.all([loaded.first().boundingBox(), page.locator("#more").boundingBox()]);
    return Math.round((card!.width / row!.width) * 100);
  };
  await expect.poll(share).toBe(25);
  await page.setViewportSize({ width: 800, height: 900 });
  await expect.poll(share).toBe(50);
  await page.setViewportSize({ width: 600, height: 900 });
  await expect.poll(share).toBe(100);
});

test("the page runs under a policy that allows styles only by nonce", async ({ page }) => {
  await page.addInitScript(() => {
    const seen: string[] = [];
    (window as unknown as { violations: string[] }).violations = seen;
    document.addEventListener("securitypolicyviolation", (event) => seen.push(`${event.violatedDirective} ${event.blockedURI}`));
  });
  const response = await page.goto(url("style"));
  const policy = response!.headers()["content-security-policy"] ?? "";
  expect(policy).toMatch(/style-src 'nonce-[0-9a-f]{32}'/);
  expect(policy).not.toContain("unsafe-inline");

  // The note is hidden on the server, under a rule that outranks its own `display`.
  const note = page.locator("#dark-note");
  await expect(note).toBeHidden();
  await page.locator("#switch-theme").click();
  await expect(note).toBeVisible();
  await expect(note).toHaveCSS("display", "flex");
  await page.locator("#switch-theme").click();
  await expect(note).toBeHidden();

  // The rules a patch brings go into a `<style>` the runtime adds with the nonce.
  await page.setViewportSize({ width: 1100, height: 900 });
  await page.locator("#load").click();
  await expect(page.locator("#more > div").first()).toHaveCSS("padding-top", "16px");

  expect(await page.evaluate(() => (window as unknown as { violations: string[] }).violations)).toEqual([]);
});
