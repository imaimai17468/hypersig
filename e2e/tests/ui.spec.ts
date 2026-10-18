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

test("a number field is a labeled spin button that starts at its signal and keeps to its range", async ({ page }) => {
  const seats = page.getByRole("spinbutton", { name: "Seats" });
  await expect(seats).toHaveValue("2");
  await expect(seats).toHaveAccessibleDescription("Up to 8 seats.");
  await expect(seats).toHaveAttribute("inputmode", "numeric");
  await seats.fill("8");
  await seats.press("ArrowUp");
  await expect(seats).toHaveValue("8");
  await seats.fill("1");
  await seats.press("ArrowDown");
  await expect(seats).toHaveValue("1");
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

test("a page shortcut opens a dialog with its statement", async ({ page }) => {
  await page.locator("h1").first().click();
  await page.keyboard.press("ControlOrMeta+k");
  const dialog = page.getByRole("dialog", { name: "Edit profile" });
  await expect(dialog).toBeVisible();
  await expect(dialog.getByRole("textbox", { name: "Display name" })).toBeFocused();
  await page.keyboard.type("k");
  await expect(dialog.getByRole("textbox", { name: "Display name" })).toHaveValue(/k$/);
  await page.keyboard.press("Escape");
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

const payments = (page: import("@playwright/test").Page) => page.getByRole("table", { name: "Payments" });
const dataTable = (page: import("@playwright/test").Page) => page.locator("section", { has: payments(page) });
const emails = (page: import("@playwright/test").Page) =>
  payments(page).locator("tbody tr:visible td:nth-child(4)").allTextContents();

test("a data table sorts by a column header, ascending then descending", async ({ page }) => {
  const header = payments(page).getByRole("columnheader", { name: "Email" });
  await expect(header).toHaveAttribute("aria-sort", "none");
  await expect(header.locator("svg:visible")).toHaveCount(1);
  await header.getByRole("button").click();
  await expect(header).toHaveAttribute("aria-sort", "ascending");
  await expect(header.locator("svg:visible")).toHaveCount(1);
  await expect(header.locator("svg:visible path").first()).toHaveAttribute("d", "m5 12 7-7 7 7");
  expect(await emails(page)).toEqual([
    "abe45@example.com",
    "ava67@example.com",
    "carmella@example.com",
    "emma12@example.com",
    "jason78@example.com",
  ]);
  await header.getByRole("button").click();
  await expect(header).toHaveAttribute("aria-sort", "descending");
  expect((await emails(page))[0]).toBe("silas22@example.com");

  const amount = payments(page).getByRole("columnheader", { name: "Amount" });
  await amount.getByRole("button").click();
  await expect(amount).toHaveAttribute("aria-sort", "ascending");
  await expect(header).toHaveAttribute("aria-sort", "none");
  expect((await emails(page))[0]).toBe("liam34@example.com");
});

test("a data table pages through the rows the filter passes", async ({ page }) => {
  const range = page.getByText("1–5 / 12");
  await expect(range).toBeVisible();
  const next = dataTable(page).getByRole("button", { name: "Next page" });
  const previous = dataTable(page).getByRole("button", { name: "Previous page" });
  await expect(previous).toBeDisabled();
  await next.click();
  await expect(page.getByText("6–10 / 12")).toBeVisible();
  expect(await emails(page)).toHaveLength(5);
  await next.click();
  await expect(page.getByText("11–12 / 12")).toBeVisible();
  expect(await emails(page)).toEqual(["noah90@example.com", "ava67@example.com"]);
  await expect(next).toBeDisabled();

  const filter = page.getByRole("searchbox", { name: "Filter payments" });
  await filter.fill("s");
  await expect(page.getByText("1–5 / 9")).toBeVisible();
  await next.click();
  await expect(page.getByText("6–9 / 9")).toBeVisible();
  expect(await emails(page)).toEqual([
    "sarah23@example.com",
    "olivia56@example.com",
    "noah90@example.com",
    "ava67@example.com",
  ]);

  await filter.fill("LI");
  await expect(page.getByText("1–2 / 2")).toBeVisible();
  expect(await emails(page)).toEqual(["olivia56@example.com", "liam34@example.com"]);
  await filter.fill("zzz");
  await expect(payments(page).getByText("No results.")).toBeVisible();
  await expect(page.getByText("0–0 / 0")).toBeVisible();
});

test("a data table selects rows under a tri-state header checkbox", async ({ page }) => {
  const all = payments(page).getByRole("checkbox", { name: "Select all rows" });
  const rows = payments(page).getByRole("checkbox", { name: "Select row" });
  await expect(all).toHaveAttribute("aria-checked", "false");
  await all.click();
  await expect(all).toHaveAttribute("aria-checked", "true");
  await expect(rows.first()).toBeChecked();
  await payments(page).locator("tbody tr:visible").first().locator("label, span").first().click();
  await expect(rows.first()).not.toBeChecked();
  await expect(all).toHaveAttribute("aria-checked", "mixed");
  await all.click();
  await expect(all).toHaveAttribute("aria-checked", "true");
  await all.focus();
  await page.keyboard.press("Space");
  await expect(all).toHaveAttribute("aria-checked", "false");
  await expect(rows.first()).not.toBeChecked();
});

test("a data table shows a row's details under it and keeps them with the row when sorting", async ({ page }) => {
  const toggle = payments(page).getByRole("button", { name: "Show row details" }).first();
  await expect(toggle).toHaveAttribute("aria-expanded", "false");
  const details = payments(page).getByText("Payment 1 from ken99@example.com is success.");
  await expect(details).toBeHidden();
  await toggle.click();
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
  await expect(details).toBeVisible();
  // Sorted by email, ken99 is the sixth row, the first of the second page.
  await payments(page).getByRole("columnheader", { name: "Email" }).getByRole("button").click();
  await expect(details).toBeHidden();
  await dataTable(page).getByRole("button", { name: "Next page" }).click();
  const group = payments(page).locator("tbody", { hasText: "ken99@example.com" }).first();
  await expect(group.locator("tr").nth(1)).toHaveText("Payment 1 from ken99@example.com is success.");
  await expect(details).toBeVisible();
  await group.getByRole("button", { name: "Show row details" }).click();
  await expect(details).toBeHidden();
});

test("a data table hides a column while its signal is false", async ({ page }) => {
  const status = payments(page).getByRole("columnheader", { name: "Status" });
  await expect(status).toBeVisible();
  await page.getByRole("switch", { name: "Show the status column" }).click({ force: true });
  await expect(status).toBeHidden();
  await expect(payments(page).locator("tbody tr:visible").first().locator("td:visible")).toHaveCount(4);
});

test("a data table returns to its first page when the list shrinks past the page shown", async ({ page }) => {
  const next = dataTable(page).getByRole("button", { name: "Next page" });
  await next.click();
  await next.click();
  await expect(page.getByText("11–12 / 12")).toBeVisible();
  for (const row of await payments(page).getByRole("checkbox", { name: "Select row" }).all()) {
    if (await row.isVisible()) await row.check({ force: true });
  }
  await page.getByRole("button", { name: "Delete selected" }).click();
  await expect(page.getByText("1–5 / 10")).toBeVisible();
  expect(await emails(page)).toHaveLength(5);
});

const orders = (page: import("@playwright/test").Page) => page.locator("section", { has: page.getByRole("heading", { name: "Server table" }) });
const orderEmails = (page: import("@playwright/test").Page) =>
  orders(page).locator("tbody tr:visible td:nth-child(3)").allTextContents();

test("a server table asks the server for the next page", async ({ page }) => {
  await expect(orders(page).getByText("1–5 / 42")).toBeVisible();
  expect(await orderEmails(page)).toEqual([
    "customer01@example.com",
    "customer02@example.com",
    "customer03@example.com",
    "customer04@example.com",
    "customer05@example.com",
  ]);
  const previous = orders(page).getByRole("button", { name: "Previous page" });
  await expect(previous).toBeDisabled();
  await orders(page).getByRole("button", { name: "Next page" }).click();
  await expect(orders(page).getByText("6–10 / 42")).toBeVisible();
  await expect(orders(page).locator("tbody tr:visible td:nth-child(3)").first()).toHaveText("customer06@example.com");
  await expect(orders(page).locator("tbody tr:visible")).toHaveCount(5);
  await previous.click();
  await expect(orders(page).locator("tbody tr:visible td:nth-child(3)").first()).toHaveText("customer01@example.com");
});

test("a server table sorts on the server and returns to the first page", async ({ page }) => {
  await orders(page).getByRole("button", { name: "Next page" }).click();
  await expect(orders(page).getByText("6–10 / 42")).toBeVisible();
  const amount = orders(page).getByRole("columnheader", { name: "Amount" });
  await amount.getByRole("button").click();
  await expect(amount).toHaveAttribute("aria-sort", "ascending");
  await expect(orders(page).getByText("1–5 / 42")).toBeVisible();
  await expect(orders(page).locator("tbody tr:visible td:nth-child(4)").first()).toHaveText("$30.50");
  await amount.getByRole("button").click();
  await expect(amount).toHaveAttribute("aria-sort", "descending");
  await expect(orders(page).locator("tbody tr:visible td:nth-child(4)").first()).toHaveText("$990.50");
  await expect(payments(page).locator("tbody tr:visible td:nth-child(5)").first()).toHaveText("$316.00");
});

test("a server table keeps the selected keys while the visitor moves between pages", async ({ page }) => {
  const all = orders(page).getByRole("checkbox", { name: "Select all rows" });
  const rows = orders(page).getByRole("checkbox", { name: "Select row" });
  const count = orders(page).locator("#chosen-orders");
  await rows.first().check({ force: true });
  await expect(count).toHaveText("1 selected");
  await expect(all).toHaveAttribute("aria-checked", "mixed");
  await orders(page).getByRole("button", { name: "Next page" }).click();
  await expect(orders(page).getByText("6–10 / 42")).toBeVisible();
  await expect(all).toHaveAttribute("aria-checked", "false");
  await all.click();
  await expect(count).toHaveText("6 selected");
  await expect(all).toHaveAttribute("aria-checked", "true");
  await orders(page).getByRole("button", { name: "Previous page" }).click();
  await expect(orders(page).getByText("1–5 / 42")).toBeVisible();
  await expect(rows.first()).toBeChecked();
  await expect(all).toHaveAttribute("aria-checked", "mixed");
  await orders(page).getByRole("button", { name: "Next page" }).click();
  await all.click();
  await expect(count).toHaveText("1 selected");
});

test("a table with a maximum height scrolls its rows under a header that stays in place", async ({ page }) => {
  const frame = orders(page).locator("div:has(> table)");
  const scrolled = await frame.evaluate((element) => {
    element.scrollTop = element.scrollHeight;
    return element.scrollTop;
  });
  expect(scrolled).toBeGreaterThan(0);
  const head = (await orders(page).locator("thead").boundingBox())!;
  const box = (await frame.boundingBox())!;
  expect(Math.abs(head.y - box.y)).toBeLessThan(2);
});

test("a server table filters on the server once typing pauses", async ({ page }) => {
  await orders(page).getByRole("searchbox", { name: "Filter orders" }).fill("customer1");
  await expect(orders(page).getByText("1–5 / 10")).toBeVisible();
  expect(await orderEmails(page)).toEqual([
    "customer10@example.com",
    "customer11@example.com",
    "customer12@example.com",
    "customer13@example.com",
    "customer14@example.com",
  ]);
  await orders(page).getByRole("searchbox", { name: "Filter orders" }).fill("nobody");
  await expect(orders(page).getByText("No results.")).toBeVisible();
  await expect(orders(page).getByText("0–0 / 0")).toBeVisible();
});

const bars = (page: import("@playwright/test").Page) => page.locator("#bar-chart svg rect");

test("a bar chart draws a bar per row and series, and a statement moves one", async ({ page }) => {
  await expect(bars(page)).toHaveCount(12);
  const january = bars(page).first();
  await expect(january).toHaveAttribute("height", "46.500");
  await page.getByRole("button", { name: "Add 50 desktop visitors in the first month" }).click();
  await expect(january).toHaveAttribute("height", "59.000");
  await expect(page.getByRole("table", { name: "Visitors (bars)" }).getByRole("row", { name: /January/ })).toContainText("236");
});

test("a chart refits its scale to the rows a server patch sends", async ({ page }) => {
  const ticks = () => page.locator("#bar-chart figure > div > div:first-child span:visible").allTextContents();
  expect(await ticks()).toEqual(["0", "100", "200", "300", "400"]);
  await page.getByRole("button", { name: "Show the second half" }).click();
  await expect(page.getByRole("table", { name: "Visitors (bars)" }).getByRole("rowheader")).toHaveText([
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ]);
  expect(await ticks()).toEqual(["0", "250", "500", "750", "1,000"]);
  await expect(bars(page).first()).toHaveAttribute("height", "52.000");
});

test("a bar chart draws a negative value down from the line at 0", async ({ page }) => {
  const ticks = page.locator("#lead-chart figure > div > div:first-child span:visible");
  await expect(ticks).toHaveText(["-200", "-100", "0", "100", "200"]);
  const lead = page.locator("#lead-chart svg rect");
  await expect(lead.nth(0)).toHaveAttribute("y", "23.500");
  await expect(lead.nth(0)).toHaveAttribute("height", "26.500");
  await expect(lead.nth(3)).toHaveAttribute("y", "50.000");
  await expect(lead.nth(3)).toHaveAttribute("height", "29.250");
});

test("a stacked bar chart puts each series on the ones before it", async ({ page }) => {
  const rects = page.locator("#stacked-chart svg rect");
  await expect(rects).toHaveCount(12);
  await expect(rects.nth(0)).toHaveAttribute("y", "69.000");
  await expect(rects.nth(0)).toHaveAttribute("height", "31.000");
  await expect(rects.nth(6)).toHaveAttribute("y", "55.667");
  await expect(rects.nth(6)).toHaveAttribute("height", "13.333");
  await expect(rects.nth(6)).toHaveAttribute("x", await rects.nth(0).getAttribute("x") ?? "");
});

test("a chart labels every nth row once the rows outnumber its labels", async ({ page }) => {
  await expect(page.locator("#stacked-chart figure > div > div:last-child span:visible")).toHaveText(["January", "March", "May"]);
});

test("a horizontal bar chart runs its bars from the left and its scale along the bottom", async ({ page }) => {
  const january = page.locator("#horizontal-chart svg rect").first();
  await expect(january).toHaveAttribute("x", "0.000");
  await expect(january).toHaveAttribute("width", "46.500");
  await expect(january).toHaveAttribute("y", "1.667");
  await expect(january).toHaveAttribute("height", "6.667");
  await expect(page.locator("#horizontal-chart figure > div > div:last-child span:visible")).toHaveText(["0", "100", "200", "300", "400"]);
  await expect(page.locator("#horizontal-chart figure > div > div:first-child span:visible")).toHaveText([
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
  ]);
});

test("a donut chart draws each row as an arc as long as its share", async ({ page }) => {
  const donut = page.locator("#donut-chart");
  await expect(donut.locator("figure > div > div span")).toHaveText("925");
  const arcs = donut.locator("svg circle[pathLength]");
  await expect(arcs).toHaveCount(5);
  await expect(arcs.nth(0)).toHaveAttribute("stroke-dasharray", "29.730 70.270");
  await expect(arcs.nth(0)).toHaveAttribute("stroke-dashoffset", "0.000");
  await expect(arcs.nth(1)).toHaveAttribute("stroke-dashoffset", "-29.730");
  const colors = await arcs.evaluateAll((circles) => circles.map((circle) => getComputedStyle(circle).stroke));
  expect(new Set(colors).size).toBe(5);
  await expect(page.getByRole("table", { name: "Visitors by browser" }).getByRole("row", { name: /Safari/ })).toContainText("200");
});

test("a line chart joins each row to the next and draws nothing past the last", async ({ page }) => {
  const segments = page.locator("#line-chart svg g");
  await expect(segments).toHaveCount(12);
  await expect(page.locator("#line-chart svg g:visible")).toHaveCount(10);
  await expect(page.locator("#area-chart svg polygon:visible")).toHaveCount(10);
});

test("a chart shows a row's values under the pointer", async ({ page }) => {
  const plot = page.locator("#bar-chart svg");
  const box = (await plot.boundingBox())!;
  await page.mouse.move(box.x + box.width * 0.25, box.y + box.height * 0.6);
  await expect(page.locator("#bar-chart").locator("div:visible", { hasText: /^February/ }).first()).toContainText("Desktop305");
});
