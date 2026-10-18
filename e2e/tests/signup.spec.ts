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
  await page.getByRole("button", { name: "Create account" }).click();
  await expect(emailError).toHaveText("That address is already registered.");

  await email.fill(`ada-${Date.now()}@example.com`);
  await page.getByRole("button", { name: "Create account" }).click();
  await expect(page.locator("#welcome")).toHaveText("Welcome, Ada.");
  await expect(email).toHaveValue("");
  await expect(emailError).toHaveText("");
  await expect(page.locator("#unsaved")).toBeHidden();
});
