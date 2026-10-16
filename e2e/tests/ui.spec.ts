import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test.beforeEach(async ({ page }) => {
  await page.goto(url("ui"));
});

const background = (page: import("@playwright/test").Page) =>
  page.locator("#app").evaluate((app) => getComputedStyle(app).backgroundColor);

test("the tokens follow the system scheme until a signal picks one", async ({ page }) => {
  await page.emulateMedia({ colorScheme: "light" });
  expect(await background(page)).toBe("rgb(255, 255, 255)");
  await page.emulateMedia({ colorScheme: "dark" });
  expect(await background(page)).toBe("rgb(9, 9, 11)");
  await page.getByLabel("Theme").selectOption({ label: "Light" });
  await expect.poll(() => background(page)).toBe("rgb(255, 255, 255)");
  await page.emulateMedia({ colorScheme: "light" });
  await page.getByLabel("Theme").selectOption({ label: "Dark" });
  await expect.poll(() => background(page)).toBe("rgb(9, 9, 11)");
});

test("a text field is labeled, described, and marked invalid by the server's error", async ({ page }) => {
  const email = page.getByRole("textbox", { name: "Email" });
  await expect(email).toHaveAccessibleDescription("We send receipts here.");
  await expect(email).toHaveAttribute("aria-invalid", "false");
  await expect(email).toHaveAttribute("required", "");

  await page.getByRole("textbox", { name: "Name" }).fill("Ada");
  await email.fill("ada@taken.example");
  await page.getByRole("button", { name: "Save" }).click();
  await expect(email).toHaveAttribute("aria-invalid", "true");
  await expect(email).toHaveAccessibleDescription("We send receipts here. That email is already in use.");
  await expect(email).toHaveCSS("border-top-color", "rgb(231, 0, 11)");

  await email.fill("ada@example.com");
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.locator("#saved")).toHaveText("Saved Ada.");
  await expect(email).toHaveAttribute("aria-invalid", "false");
});

test("a checkbox, a switch, and radios keep their native roles and keys", async ({ page }) => {
  const newsletter = page.getByRole("checkbox", { name: "Email me about new releases" });
  await expect(newsletter).not.toBeChecked();
  await page.getByText("Email me about new releases").click();
  await expect(newsletter).toBeChecked();

  const notifications = page.getByRole("switch", { name: "Notifications" });
  await expect(notifications).toBeChecked();
  await notifications.focus();
  await page.keyboard.press("Space");
  await expect(notifications).not.toBeChecked();

  const plan = page.getByRole("group", { name: "Plan" });
  await expect(plan.getByRole("radio", { name: "Pro" })).toBeChecked();
  await plan.getByRole("radio", { name: "Pro" }).focus();
  await page.keyboard.press("ArrowDown");
  await expect(plan.getByRole("radio", { name: "Team" })).toBeChecked();
  await expect(plan.getByRole("radio", { name: "Team" })).toBeFocused();

  await page.getByLabel("Region").selectOption({ label: "Frankfurt" });
  await expect(page.getByLabel("Region")).toHaveValue('"Frankfurt"');
});

test("tabs take one Tab stop and move with the arrow keys, Home, and End", async ({ page }) => {
  const list = page.getByRole("tablist", { name: "Settings" });
  const tab = (name: string) => list.getByRole("tab", { name });
  await expect(tab("Account")).toHaveAttribute("aria-selected", "true");
  await expect(tab("Password")).toHaveAttribute("tabindex", "-1");

  await tab("Account").focus();
  await page.keyboard.press("ArrowRight");
  await expect(tab("Password")).toBeFocused();
  await expect(tab("Password")).toHaveAttribute("aria-selected", "true");
  await expect(page.getByRole("tabpanel", { name: "Password" })).toHaveText("Passwords are at least twelve characters.");
  await expect(page.getByRole("tabpanel", { name: "Account" })).toBeHidden();

  await page.keyboard.press("End");
  await expect(tab("Billing")).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(tab("Account")).toBeFocused();
  await page.keyboard.press("Home");
  await expect(tab("Account")).toHaveAttribute("aria-selected", "true");

  await page.keyboard.press("Tab");
  await expect(page.getByRole("tabpanel", { name: "Account" })).toBeFocused();
  await page.keyboard.press("Shift+Tab");
  await expect(tab("Account")).toBeFocused();

  await tab("Billing").click();
  await expect(tab("Billing")).toHaveAttribute("aria-selected", "true");
});

test("an exclusive accordion keeps one section open", async ({ page }) => {
  const first = page.getByText("Does it need JavaScript?");
  const second = page.getByText("Can I restyle it?");
  await first.click();
  await expect(page.getByText("The accordion is <details>")).toBeVisible();
  await second.focus();
  await page.keyboard.press("Enter");
  await expect(page.getByText("Set the tokens in a Theme")).toBeVisible();
  await expect(page.getByText("The accordion is <details>")).toBeHidden();
});

test("a dialog keeps focus off the inert page, closes on Escape, and returns focus to its button", async ({ page }) => {
  const open = page.getByRole("button", { name: "Edit profile" });
  await open.click();
  const dialog = page.getByRole("dialog", { name: "Edit profile" });
  await expect(dialog).toBeVisible();
  await expect(dialog).toHaveAccessibleDescription("Make changes to your profile, then press Done.");
  await expect(dialog.getByRole("textbox", { name: "Display name" })).toBeFocused();
  // Past its last control, Tab leaves the document for the browser's own
  // controls, which headless Chrome reports as focus on <body>.
  const visited = new Set<string>();
  for (let i = 0; i < 8; i++) {
    await page.keyboard.press("Tab");
    const where = await dialog.evaluate((el) =>
      el.contains(document.activeElement) ? document.activeElement!.textContent || document.activeElement!.id : document.activeElement === document.body ? "body" : "page",
    );
    expect(where).not.toBe("page");
    visited.add(where);
  }
  expect(visited).toContain("Done");
  await page.keyboard.press("Escape");
  await expect(dialog).toBeHidden();
  await expect(open).toBeFocused();

  await open.click();
  await dialog.getByRole("button", { name: "Close" }).click();
  await expect(dialog).toBeHidden();
});

test("an alert dialog stays open on a click outside and closes with its buttons", async ({ page }) => {
  await page.getByRole("button", { name: "Delete project" }).click();
  const dialog = page.getByRole("alertdialog", { name: "Delete this project?" });
  await expect(dialog).toBeVisible();
  await page.mouse.click(5, 5);
  await expect(dialog).toBeVisible();
  await dialog.getByRole("button", { name: "Delete" }).click();
  await expect(dialog).toBeHidden();
  await expect(page.locator("#status")).toHaveText("Deleted the project.");
});

test("a menu opens under its button, moves with the arrow keys, and closes", async ({ page }) => {
  const trigger = page.getByRole("button", { name: "Project actions" });
  const menu = page.getByRole("menu", { name: "Project actions" });
  const item = (name: string) => menu.getByRole("menuitem", { name });
  await expect(menu).toBeHidden();

  await trigger.click();
  await expect(menu).toBeVisible();
  await expect(item("Rename")).toBeFocused();
  const [button, list] = [await trigger.boundingBox(), await menu.boundingBox()];
  expect(list!.y).toBeGreaterThanOrEqual(button!.y + button!.height);
  expect(Math.abs(list!.x - button!.x)).toBeLessThan(1);

  await page.keyboard.press("ArrowDown");
  await expect(item("Duplicate")).toBeFocused();
  await page.keyboard.press("End");
  await expect(item("Delete")).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await expect(item("Rename")).toBeFocused();
  await page.keyboard.press("Escape");
  await expect(menu).toBeHidden();
  await expect(trigger).toBeFocused();

  await page.keyboard.press("Enter");
  await expect(item("Rename")).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(menu).toBeHidden();
  await expect(trigger).toBeFocused();

  await trigger.click();
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Enter");
  await expect(menu).toBeHidden();
  await expect(page.locator("#status")).toHaveText("Archived the project.");
  await trigger.click();
  await expect(item("Archive")).toBeDisabled();
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await expect(item("Delete")).toBeFocused();
});

test("a progress bar reports its value", async ({ page }) => {
  const bar = page.getByRole("progressbar", { name: "Upload" });
  await expect(bar).toHaveAttribute("aria-valuenow", "40");
  await page.locator("#more").click();
  await expect(bar).toHaveAttribute("aria-valuenow", "50");
  for (let i = 0; i < 7; i++) await page.locator("#more").click();
  await expect(bar).toHaveAttribute("aria-valuenow", "100");
});

test("a listbox takes one Tab stop and selects the option the arrow keys reach", async ({ page }) => {
  const list = page.getByRole("listbox", { name: "Font" });
  const sans = list.getByRole("option", { name: "Sans serif" });
  const serif = list.getByRole("option", { name: "Serif", exact: true });
  const mono = list.getByRole("option", { name: "Monospace" });
  await expect(sans).toHaveAttribute("aria-selected", "true");
  await page.getByLabel("Region").focus();
  await page.keyboard.press("Tab");
  await expect(sans).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await expect(serif).toBeFocused();
  await expect(serif).toHaveAttribute("aria-selected", "true");
  await expect(sans).toHaveAttribute("aria-selected", "false");
  await page.keyboard.press("End");
  await expect(mono).toBeFocused();
  await expect(mono).toHaveAttribute("tabindex", "0");
  await expect(serif).toHaveAttribute("tabindex", "-1");
  await page.keyboard.press("Tab");
  await expect(page.getByRole("combobox", { name: "Country" })).toBeFocused();
});

test("a combobox keeps focus in its field while the arrow keys reach the options it shows", async ({ page }) => {
  const field = page.getByRole("combobox", { name: "Country" });
  const list = page.getByRole("listbox", { name: "Suggestions" });
  await expect(field).toHaveAttribute("aria-expanded", "false");
  await expect(list).toBeHidden();

  await field.fill("an");
  await expect(field).toHaveAttribute("aria-expanded", "true");
  await expect(list.getByRole("option")).toHaveText(["Canada", "France", "Germany", "Japan"]);
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await expect(field).toBeFocused();
  const france = list.getByRole("option", { name: "France" });
  await expect(field).toHaveAttribute("aria-activedescendant", (await france.getAttribute("id"))!);
  await expect(france).toHaveAttribute("aria-selected", "true");
  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("ArrowUp");
  await expect(list.getByRole("option", { name: "Japan" })).toHaveAttribute("aria-selected", "true");
  await page.keyboard.press("Enter");
  await expect(field).toHaveValue("Japan");
  await expect(field).toHaveAttribute("aria-expanded", "false");

  await page.keyboard.press("Escape");
  await expect(field).toHaveValue("");
  await page.keyboard.press("ArrowDown");
  await list.getByRole("option", { name: "Norway" }).click();
  await expect(field).toHaveValue("Norway");
  await expect(field).toBeFocused();
  await expect(list).toBeHidden();

  await field.fill("zz");
  await expect(page.getByText("No matches")).toBeVisible();
  await expect(list).toBeHidden();
  await page.keyboard.press("Tab");
  await expect(field).toHaveAttribute("aria-expanded", "false");
});

test("a slider is a native range with a value text", async ({ page }) => {
  const slider = page.getByRole("slider", { name: "Volume" });
  await expect(slider).toHaveValue("40");
  await slider.focus();
  await page.keyboard.press("ArrowRight");
  await expect(slider).toHaveValue("45");
  await expect(slider).toHaveAttribute("aria-valuetext", "45 %");
  await page.keyboard.press("End");
  await expect(slider).toHaveValue("100");
});

test("a tooltip describes its button and opens on hover and focus", async ({ page }) => {
  const add = page.getByRole("button", { name: "Add item" });
  const tip = page.getByRole("tooltip");
  await expect(add).toHaveAccessibleDescription("Adds an empty row at the end");
  await expect(tip).toBeHidden();
  await add.hover();
  await expect(tip).toBeVisible();
  await tip.hover();
  await expect(tip).toBeVisible();
  await page.mouse.move(0, 0);
  await expect(tip).toBeHidden();
  await add.focus();
  await expect(tip).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(tip).toBeHidden();
});

test("a toast is announced in the notifications region and stays until dismissed", async ({ page }) => {
  const region = page.getByRole("region", { name: "Notifications" });
  await expect(region.getByRole("list")).toHaveAttribute("aria-live", "polite");
  await page.locator("#toast").click();
  await page.locator("#toast-error").click();
  await expect(region.getByRole("listitem")).toHaveCount(2);
  await expect(region.getByRole("listitem").first()).toContainText("Event created");
  await region.getByRole("listitem").first().getByRole("button", { name: "Dismiss" }).click();
  await expect(region.getByRole("listitem")).toHaveCount(1);
  await expect(region.getByRole("listitem")).toContainText("Upload failed");
});
