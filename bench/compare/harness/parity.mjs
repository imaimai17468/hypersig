// Checks that each app behaves as SPEC.md describes, before any of it is measured.
//
// Usage: node harness/parity.mjs [app ...]

import assert from "node:assert/strict";
import { setTimeout as sleep } from "node:timers/promises";
import { chromium } from "@playwright/test";
import { APPS, reset, start, url } from "./apps.mjs";

async function check(page, name) {
  const count = () => page.locator(".todo-count strong").textContent().then((t) => t.trim());
  const item = (title) => page.locator("ul.todo-list > li", { has: page.getByText(title, { exact: true }) });
  const visible = () => page.evaluate(() => [...document.querySelectorAll("ul.todo-list > li")].filter((li) => li.offsetParent !== null).length);
  const until = async (what, fn) => {
    for (let attempt = 0; attempt < 100; attempt++) {
      if (await fn()) return;
      await sleep(50);
    }
    assert.fail(`${name}: ${what}`);
  };

  await reset(name);
  await page.goto(url(name), { waitUntil: "load" });
  await sleep(1000);
  assert.equal(await count(), "1334", `${name}: initial count`);
  assert.equal(await page.locator("ul.todo-list > li").count(), 2000, `${name}: initial items`);
  assert.equal(await item("todo 3").getAttribute("class").then((c) => (c ?? "").includes("completed")), true, `${name}: todo 3 starts completed`);

  await page.locator("input.new-todo").fill("parity add");
  await page.locator("input.new-todo").press("Enter");
  await until("add shows the item and the count", async () => (await count()) === "1335" && (await item("parity add").count()) === 1);
  assert.equal(await page.locator("input.new-todo").inputValue(), "", `${name}: add clears the input`);

  await page.locator("input.new-todo").fill("   ");
  await page.locator("input.new-todo").press("Enter");
  await sleep(300);
  assert.equal(await count(), "1335", `${name}: blank text adds nothing`);
  await page.locator("input.new-todo").fill("");

  await item("todo 1").locator("input.toggle").click();
  await until("toggle marks the item and updates the count", async () =>
    (await count()) === "1334" && ((await item("todo 1").getAttribute("class")) ?? "").includes("completed"));

  await item("todo 2").hover();
  await item("todo 2").locator("button.destroy").click();
  await until("delete removes the item and updates the count", async () => (await count()) === "1333" && (await item("todo 2").count()) === 0);

  await page.locator(".filters a", { hasText: "Active" }).click();
  await until("Active shows only active items", async () => (await visible()) === 1333);
  await page.locator(".filters a", { hasText: "Completed" }).click();
  await until("Completed shows only completed items", async () => (await visible()) === 667);
  await page.locator(".filters a", { hasText: "All" }).click();
  await until("All shows every item", async () => (await visible()) === 2000);

  await sleep(500);
  await page.reload({ waitUntil: "load" });
  await sleep(1000);
  assert.equal(await count(), "1333", `${name}: the server kept the count`);
  assert.equal(await item("parity add").count(), 1, `${name}: the server kept the added item`);
  assert.equal(await item("todo 2").count(), 0, `${name}: the server kept the delete`);
  assert.equal(((await item("todo 1").getAttribute("class")) ?? "").includes("completed"), true, `${name}: the server kept the toggle`);
}

const names = process.argv.slice(2).length > 0 ? process.argv.slice(2) : Object.keys(APPS);
const browser = await chromium.launch({ channel: "chrome", headless: true });
let failed = false;
for (const name of names) {
  const stop = await start(name);
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  try {
    await check(page, name);
    assert.deepEqual(errors, [], `${name}: page errors`);
    console.log(`${name}: ok`);
  } catch (error) {
    failed = true;
    console.log(`${name}: FAILED ${error.message}`);
  } finally {
    await page.close();
    stop();
  }
}
await browser.close();
process.exit(failed ? 1 : 0);
