import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test("checks the rules as the visitor leaves each field, and again on submit", async ({ page }) => {
  await page.goto(url("signup"));
  const name = page.getByRole("textbox", { name: "Name", exact: true });
  const email = page.getByRole("textbox", { name: "Email", exact: true });
  const nameError = page.locator("#ui-signup-value-name-error");
  const emailError = page.locator("#ui-signup-value-email-error");
  const passwordError = page.locator("#ui-signup-value-password-error");

  await email.fill("ada");
  await expect(emailError).toHaveText("");
  await expect(page.locator("#unsaved")).toBeVisible();
  await email.blur();
  await expect(emailError).toHaveText("An email address has an @ in it.");
  await expect(email).toHaveAttribute("aria-invalid", "true");
  await expect(nameError).toHaveText("");

  await page.getByRole("button", { name: "Create account" }).click();
  await expect(nameError).toHaveText("Enter your name.");
  await expect(passwordError).toHaveText("Use 8 or more characters.");

  await page.getByRole("button", { name: "Clear" }).click();
  await expect(email).toHaveValue("");
  await expect(emailError).toHaveText("");
  await expect(page.locator("#unsaved")).toBeHidden();
  await expect(name).toHaveValue("");
});

test("shows the server's message for a taken address, then signs up and starts over", async ({ page }) => {
  await page.goto(url("signup"));
  const email = page.getByRole("textbox", { name: "Email", exact: true });
  const emailError = page.locator("#ui-signup-value-email-error");

  await page.getByRole("textbox", { name: "Name", exact: true }).fill("Ada");
  await email.fill("taken@example.com");
  await page.getByRole("textbox", { name: "Password", exact: true }).fill("correct horse");
  await page.getByRole("textbox", { name: "Confirm password", exact: true }).fill("correct horse");
  await page.getByRole("radio", { name: "Team" }).check();
  await page.getByRole("button", { name: "Create account" }).click();
  await expect(emailError).toHaveText("That address is already registered.");

  await email.fill(`ada-${Date.now()}@example.com`);
  await page.getByRole("button", { name: "Create account" }).click();
  await expect(page.locator("#welcome")).toHaveText("Welcome, Ada.");
  await expect(email).toHaveValue("");
  await expect(emailError).toHaveText("");
  await expect(page.locator("#unsaved")).toBeHidden();
});

test("touches the plan when focus leaves the radio group, not while the arrow keys move inside it", async ({ page }) => {
  await page.goto(url("signup"));
  const group = page.getByRole("radiogroup", { name: "Plan" });
  const planError = page.locator("#ui-signup-value-plan-error");
  // Records every value aria-invalid takes, so a message shown for a moment is caught too.
  await group.evaluate((element) => {
    const seen: string[] = [];
    (window as unknown as { seen: string[] }).seen = seen;
    new MutationObserver(() => seen.push(element.getAttribute("aria-invalid") ?? "")).observe(element, {
      attributeFilter: ["aria-invalid"],
    });
  });

  await page.getByRole("radio", { name: "Personal" }).focus();
  await page.keyboard.press("ArrowDown");
  await expect(page.getByRole("radio", { name: "Team" })).toBeChecked();
  await expect(page.getByRole("radio", { name: "Team" })).toBeFocused();
  await page.keyboard.press("ArrowUp");
  await expect(page.getByRole("radio", { name: "Personal" })).toBeChecked();
  await expect(planError).toHaveText("");
  expect(await page.evaluate(() => (window as unknown as { seen: string[] }).seen)).not.toContain("true");

  await page.keyboard.press("Tab");
  await expect(planError).toHaveText("");
  await expect(group).toHaveAttribute("aria-invalid", "false");
});

test("shows the plan's message when focus leaves the group with nothing chosen", async ({ page }) => {
  await page.goto(url("signup"));
  const planError = page.locator("#ui-signup-value-plan-error");
  await page.getByRole("radio", { name: "Personal" }).focus();
  await expect(planError).toHaveText("");
  await page.keyboard.press("Tab");
  await expect(planError).toHaveText("Choose a plan.");
  await expect(page.getByRole("radiogroup", { name: "Plan" })).toHaveAttribute("aria-invalid", "true");
});
