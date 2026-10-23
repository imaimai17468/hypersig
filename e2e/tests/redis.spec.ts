import { spawn, type ChildProcess } from "node:child_process";
import { fileURLToPath } from "node:url";
import type { Page } from "@playwright/test";
import { redisUrl, secondRedisPort as port, url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

const items = (page: Page) => page.locator("#todo-list li");

test("todos are added, toggled, and removed through Redis", async ({ page }) => {
  await page.goto(url("redis"));
  const draft = page.getByPlaceholder("What needs doing?");
  await draft.fill("buy tea");
  await draft.press("Enter");
  await expect(draft).toHaveValue("");
  await draft.fill("write docs");
  await draft.press("Enter");
  await expect(items(page)).toHaveCount(2);
  await expect(page.locator("#summary-left")).toHaveText("2 left");

  await items(page).nth(0).getByLabel("toggle").click();
  await expect(page.locator("#summary-left")).toHaveText("1 left");
  await page.reload();
  await expect(items(page).nth(0).locator("span")).toHaveCSS("text-decoration-line", "line-through");

  await items(page).nth(1).getByLabel("remove").click();
  await expect(items(page)).toHaveCount(1);
  await page.locator("#todo-list li").getByRole("link", { name: "buy tea" }).click();
  await expect(page.locator("#status")).toBeVisible();
});

/** A second server process on the Redis the first one uses. */
test.describe("a second process on the same Redis", () => {
  let second: ChildProcess | undefined;

  test.beforeAll(async () => {
    const root = fileURLToPath(new URL("../..", import.meta.url));
    // The web server step built this binary with `cargo run --release`.
    second = spawn("target/release/example-redis", [], {
      cwd: root,
      env: { ...process.env, ADDR: `127.0.0.1:${port}`, REDIS_URL: redisUrl },
      stdio: "ignore",
    });
    await expect
      .poll(() => fetch(`http://127.0.0.1:${port}/`).then((response) => response.status, () => 0), { timeout: 10_000 })
      .toBe(200);
  });

  test.afterAll(() => {
    second?.kill();
  });

  test("an open tab of one process shows a write through the other at once", async ({ page, context }) => {
    const other = await context.newPage();
    await page.goto(url("redis"));
    await other.goto(`http://127.0.0.1:${port}/`);

    const draft = page.getByPlaceholder("What needs doing?");
    await draft.fill("seen by both");
    await draft.press("Enter");
    const written = items(other).filter({ hasText: "seen by both" });
    await expect(written).toBeVisible();

    await written.getByLabel("toggle").click();
    await expect(items(page).filter({ hasText: "seen by both" }).locator("span")).toHaveCSS(
      "text-decoration-line",
      "line-through",
    );
  });
});
