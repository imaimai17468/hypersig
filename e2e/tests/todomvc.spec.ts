import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test.beforeEach(async ({ request }) => {
  await request.post(`${url("todomvc")}save`, {
    data: { todos: [], next_id: 1, draft: "", filter: "All", editing: null, edit_text: "", left: 0, completed: 0 },
  });
});

test("adds, completes, filters, edits, and clears todos in the browser", async ({ page }) => {
  await page.goto(url("todomvc"));
  const input = page.getByPlaceholder("What needs to be done?");
  const visible = page.locator(".todo-list li").filter({ visible: true });
  const count = page.locator(".todo-count");
  const filter = (name: string) => page.locator(".filters a").getByText(name, { exact: true });
  await expect(page.locator(".main")).toBeHidden();

  for (const title of ["buy milk", "walk dog", "write code"]) {
    await input.fill(title);
    await input.press("Enter");
  }
  await expect(visible.locator("label")).toHaveText(["buy milk", "walk dog", "write code"]);
  await expect(count).toHaveText("3 items left");
  await expect(input).toHaveValue("");

  await visible.nth(1).locator(".toggle").check();
  await expect(count).toHaveText("2 items left");
  await expect(visible.nth(1)).toHaveClass(/completed/);

  await filter("Active").click();
  await expect(visible.locator("label")).toHaveText(["buy milk", "write code"]);
  await filter("Completed").click();
  await expect(visible.locator("label")).toHaveText(["walk dog"]);
  await filter("All").click();

  const edit = page.locator(".todo-list .edit");
  await visible.nth(0).locator("label").dblclick();
  await expect(edit).toBeFocused();
  await edit.fill("buy oat milk");
  await edit.press("Enter");
  await expect(edit).toHaveCount(0);
  await visible.nth(2).locator("label").dblclick();
  await edit.fill("nope");
  await edit.press("Escape");
  await expect(edit).toHaveCount(0);
  await expect(visible.locator("label")).toHaveText(["buy oat milk", "walk dog", "write code"]);
  await visible.nth(2).locator("label").dblclick();
  await edit.fill("");
  await page.locator("h1").click();
  await expect(visible.locator("label")).toHaveText(["buy oat milk", "walk dog"]);

  await page.locator("label[for=toggle-all]").click();
  await expect(count).toHaveText("0 items left");
  await expect(page.locator("#toggle-all")).toBeChecked();
  await page.locator("label[for=toggle-all]").click();
  await expect(count).toHaveText("2 items left");
  await visible.nth(0).locator(".toggle").check();
  await expect(count).toHaveText("1 item left");
  await page.locator(".clear-completed").click();
  await expect(visible.locator("label")).toHaveText(["walk dog"]);

  await visible.nth(0).hover();
  await visible.nth(0).locator(".destroy").click();
  await expect(page.locator(".main")).toBeHidden();
});

test("keeps the list across a reload and serves it as HTML without JavaScript", async ({ page, browser }) => {
  await page.goto(url("todomvc"));
  const input = page.getByPlaceholder("What needs to be done?");
  for (const title of ["one", "two"]) {
    await input.fill(title);
    await input.press("Enter");
  }
  await page.locator(".todo-list li").first().locator(".toggle").check();
  await expect.poll(async () => {
    const response = await page.request.get(url("todomvc"));
    return (await response.text()).includes("two");
  }).toBe(true);

  const plain = await browser.newContext({ javaScriptEnabled: false });
  const html = await plain.newPage();
  await html.goto(url("todomvc"));
  await expect(html.locator(".todo-list li label")).toHaveText(["one", "two"]);
  await expect(html.locator(".todo-list li").first()).toHaveClass(/completed/);
  await expect(html.locator(".todo-count")).toHaveText("1 item left");
  await expect(html.locator(".clear-completed")).toBeVisible();
  await plain.close();

  await page.reload();
  await expect(page.locator(".todo-list li")).toHaveCount(2);
  await input.fill("three");
  await input.press("Enter");
  await expect(page.locator(".todo-list li label")).toHaveText(["one", "two", "three"]);
});
