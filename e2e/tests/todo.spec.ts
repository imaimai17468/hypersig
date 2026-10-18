import { url } from "../playwright.config.ts";
import { test as baseTest } from "@playwright/test";
import { expect, test, watch } from "./support.ts";

test("two tabs see each other's changes over the update stream", async ({ page: a, context }) => {
  const b = await context.newPage();
  const problems = watch(b);
  await a.goto(url("todo"));
  await b.goto(url("todo"));
  const items = (page: typeof a) => page.locator("#todo-list li");

  const draft = a.getByPlaceholder("What needs doing?");
  await draft.fill("buy tea");
  await draft.press("Enter");
  // The reply to an add clears the draft. Typing before it arrives loses the text.
  await expect(draft).toHaveValue("");
  await expect(items(a)).toHaveCount(1);
  await draft.fill("write docs");
  await a.getByPlaceholder("What needs doing?").press("Enter");
  await expect(items(b)).toHaveCount(2);
  await expect(b.locator("#summary-left")).toHaveText("2 left");
  // The items arrived in a patch, so their rules did too.
  const title = (page: typeof a, n: number) => items(page).nth(n).locator("span");
  await expect(items(b).nth(0)).toHaveCSS("display", "flex");
  await expect(title(b, 0)).toHaveCSS("text-decoration-line", "none");

  await items(b).nth(0).getByLabel("toggle").click();
  await expect(title(a, 0)).toHaveCSS("text-decoration-line", "line-through");
  await expect(title(a, 0)).toHaveCSS("color", "rgb(136, 136, 136)");
  await items(a).nth(1).getByLabel("toggle").click();
  await expect(b.locator("#summary-done")).toHaveText("All done.");

  await items(b).nth(0).getByLabel("remove").click();
  await expect(items(a)).toHaveCount(1);
  expect(problems).toEqual([]);
});

// The refused connection logs an error by design, so this test runs without the check of `./support.ts`.
baseTest("the update stream connects again after its first connection is refused", async ({ page: a, context }) => {
  let attempts = 0;
  await a.route("**/todos/updates", async (route) => {
    attempts += 1;
    if (attempts === 1) await route.abort("connectionrefused");
    else await route.continue();
  });
  await a.goto(url("todo"));
  await expect.poll(() => attempts, { timeout: 5000 }).toBe(2);

  const b = await context.newPage();
  await b.goto(url("todo"));
  const draft = b.getByPlaceholder("What needs doing?");
  await draft.fill("seen after a reconnect");
  await draft.press("Enter");
  await expect(a.locator("#todo-list li").getByText("seen after a reconnect")).toBeVisible();
});

test("a todo's title links to its own page, and an unknown id gets 404", async ({ page, request }) => {
  await page.goto(url("todo"));
  const draft = page.getByPlaceholder("What needs doing?");
  await draft.fill("read the spec");
  await draft.press("Enter");
  await page.locator("#todo-list li").getByRole("link", { name: "read the spec" }).click();
  await expect(page).toHaveURL(/\/todos\/\d+$/);
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("read the spec");
  await expect(page.locator("#status-open")).toBeVisible();
  await expect(page.locator("#status")).toBeHidden();

  await page.getByRole("link", { name: "All todos" }).click();
  await expect(page).toHaveURL(url("todo"));
  await expect(page.locator("#todo-list li").getByRole("link", { name: "read the spec" })).toBeVisible();

  expect((await request.get(`${url("todo")}todos/999999`)).status()).toBe(404);
  expect((await request.get(`${url("todo")}todos/seven`)).status()).toBe(404);
});
