import { test as base } from "@playwright/test";
import { url } from "../playwright.config.ts";
import { expect, test, watch } from "./support.ts";

test("one newsletter part serves two pages, each under its own field", async ({ page }) => {
  for (const [path, field] of [
    ["", "footer"],
    ["about", "shell.footer"],
  ] as const) {
    await page.goto(`${url("storefront")}${path}`);
    const email = page.getByRole("textbox", { name: "Email" });
    const sent = page.waitForRequest((request) => request.url().endsWith("/newsletter/join"));
    await email.fill(`ada+${path || "home"}@example.com`);
    await page.getByRole("button", { name: "Join" }).click();
    expect((await sent).headers()["hypersig-part"]).toBe(field);

    // The handler re-renders the part's fragment, whose binding reads the
    // address it patched under the same field.
    await expect(page.locator("#newsletter-joined")).toContainText(`Thanks, ada+${path || "home"}@example.com.`);

    // A call from inside the re-rendered fragment still names the field.
    const again = page.waitForRequest((request) => request.url().endsWith("/newsletter/another"));
    await page.getByRole("button", { name: "Add another address" }).click();
    expect((await again).headers()["hypersig-part"]).toBe(field);
    await expect(page.getByRole("textbox", { name: "Email" })).toHaveValue("");
  }
});

// The address is refused on purpose, so this test checks the failure it logs itself.
base("shows the newsletter's error inside the part", async ({ page }) => {
  const problems = watch(page);
  await page.goto(`${url("storefront")}about`);
  await page.getByRole("textbox", { name: "Email" }).fill("not-an-address");
  // The input's own validation would stop the submit, so the form is sent without it.
  await page.locator("form").evaluate((form) => form.setAttribute("novalidate", ""));
  await page.getByRole("button", { name: "Join" }).click();
  await expect(page.getByRole("alert")).toHaveText("An email address has an @ in it.");
  expect(problems.filter((problem) => problem.startsWith("422"))).toHaveLength(1);
});

// One save is refused on purpose, so this test checks the failure it logs itself.
base("marks only the wish whose save failed while several are in flight", async ({ page }) => {
  const problems = watch(page);
  await page.goto(url("storefront"));
  const wish = page.getByRole("textbox", { name: "Wish" });
  const name = `oolong-${Date.now()}`;
  for (const text of [name, `${name}-pot`, name]) {
    await wish.fill(text);
    await wish.press("Enter");
  }
  const items = page.locator("#wishes li");
  await expect(items).toHaveCount(3);
  // All three calls are in flight before the first answer arrives.
  await expect(items.filter({ hasText: "saving" })).toHaveCount(3);

  await expect(items.nth(0)).toHaveText(`${name} saved`);
  await expect(items.nth(1)).toHaveText(`${name}-pot saved`);
  await expect(items.nth(2)).toHaveText(`${name} ${name} is already on the list.`);
  expect(problems.filter((problem) => problem.startsWith("409"))).toHaveLength(1);
});
