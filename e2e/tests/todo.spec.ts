import { url } from "../playwright.config.ts";
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
