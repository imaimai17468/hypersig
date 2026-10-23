import { url } from "../playwright.config.ts";
import { test as baseTest } from "@playwright/test";
import { expect, test } from "./support.ts";

test("a note goes to the notes service and the page shows the service's list", async ({ page }) => {
  await page.goto(url("api"));
  const draft = page.getByPlaceholder("Write a note");
  await draft.fill("call the service");
  await draft.press("Enter");
  await expect(draft).toHaveValue("");
  await expect(page.locator("#note-list li").getByText("call the service")).toBeVisible();

  await page.reload();
  await expect(page.locator("#note-list li").getByText("call the service")).toBeVisible();
});

// The refused note answers 422 by design, so this test runs without the check of `./support.ts`.
baseTest("the service's refusal reaches the page as its message, and the draft stays", async ({ page }) => {
  await page.goto(url("api"));
  const draft = page.getByPlaceholder("Write a note");
  const long = "x".repeat(81);
  await draft.fill(long);
  await draft.press("Enter");
  await expect(page.getByRole("alert")).toHaveText("A note holds 1 to 80 characters.");
  await expect(draft).toHaveValue(long);

  await draft.fill("short enough");
  await draft.press("Enter");
  await expect(page.getByRole("alert")).toBeHidden();
});
