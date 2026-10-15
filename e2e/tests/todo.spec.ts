import { url } from "../playwright.config.ts";
import { expect, test, watch } from "./support.ts";

test("two tabs see each other's changes over the update stream", async ({ page: a, context }) => {
  const b = await context.newPage();
  const problems = watch(b);
  await a.goto(url("todo"));
  await b.goto(url("todo"));
  const items = (page: typeof a) => page.locator("#todo-list li");

  await a.getByPlaceholder("What needs doing?").fill("buy tea");
  await a.getByPlaceholder("What needs doing?").press("Enter");
  await a.getByPlaceholder("What needs doing?").fill("write docs");
  await a.getByPlaceholder("What needs doing?").press("Enter");
  await expect(items(b)).toHaveCount(2);
  await expect(b.locator(".summary").filter({ visible: true })).toHaveText("2 left");

  await items(b).nth(0).locator(".check").click();
  await expect(items(a).nth(0)).toHaveClass("done");
  await items(a).nth(1).locator(".check").click();
  await expect(b.locator(".summary").filter({ visible: true })).toHaveText("All done.");

  await items(b).nth(0).locator(".remove").click();
  await expect(items(a)).toHaveCount(1);
  expect(problems).toEqual([]);
});
