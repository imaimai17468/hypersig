import { url } from "../playwright.config.ts";
import { expect, test } from "./support.ts";

test.beforeEach(async ({ page }) => {
  await page.goto(url("showcase"));
});

test("each placement of a component keeps its own state", async ({ page }) => {
  const bodies = page.locator(".disclosure p");
  await expect(bodies.nth(0)).toBeHidden();
  await expect(bodies.nth(1)).toBeVisible();
  await page.locator(".disclosure-title").getByText("Shipping", { exact: true }).click();
  await page.locator(".disclosure-title").getByText("Returns", { exact: true }).click();
  await expect(bodies.nth(0)).toBeVisible();
  await expect(bodies.nth(1)).toBeHidden();
  await expect(page.locator(".disclosure-title").nth(0)).toHaveAttribute("aria-expanded", "true");

  const stepper = (i: number) => page.locator(".stepper").nth(i);
  await expect(stepper(1).getByLabel("fewer")).toBeDisabled();
  await stepper(1).getByLabel("more").click();
  await stepper(0).getByLabel("fewer").click();
  await expect(page.locator(".total")).toHaveText("2 guests");
  await expect(stepper(0).getByLabel("fewer")).toBeDisabled();
});

test("an interval runs while started", async ({ page }) => {
  await page.locator(".start").click();
  await expect(page.locator(".start")).toHaveText("Stop");
  await expect(page.locator(".reset")).toBeDisabled();
  await expect.poll(async () => parseFloat((await page.locator(".elapsed").textContent()) ?? "0")).toBeGreaterThan(0.3);
  await page.locator(".start").click();
  const stopped = await page.locator(".elapsed").textContent();
  await page.waitForTimeout(300);
  await expect(page.locator(".elapsed")).toHaveText(stopped ?? "");
  await page.locator(".reset").click();
  await expect(page.locator(".elapsed")).toHaveText("0.0 s");
});

test("search waits for a pause in typing and shows an indicator", async ({ page }) => {
  const field = page.getByPlaceholder("Search fruit");
  await page.locator(".focus").click();
  await expect(field).toBeFocused();
  const searches: string[] = [];
  page.on("request", (request) => {
    if (request.url().endsWith("/search")) searches.push(request.url());
  });
  await page.keyboard.type("berr", { delay: 30 });
  await expect(page.locator(".searching")).toBeVisible();
  await expect(page.locator("#results li")).toHaveText(["blackberry", "blueberry"]);
  await expect(page.locator(".searching")).toBeHidden();
  expect(searches).toHaveLength(1);
  await page.keyboard.press("Escape");
  await expect(field).toHaveValue("");
  await expect(page.locator("#results li")).toHaveCount(13);
});

test("each answered search is a history entry, Back loads the earlier search from the server, and the title follows the query", async ({ page }) => {
  const field = page.getByPlaceholder("Search fruit");
  const shown = () => {
    const at = new URL(page.url());
    return at.pathname + at.search;
  };
  await expect(page).toHaveTitle("hypersig showcase");
  await field.click();
  await page.keyboard.type("berr", { delay: 30 });
  await expect(page.locator("#results li")).toHaveText(["blackberry", "blueberry"]);
  await expect.poll(shown).toBe("/?q=berr");
  await expect(page).toHaveTitle("berr - hypersig showcase");
  await page.keyboard.press("ControlOrMeta+A");
  await page.keyboard.type("ap", { delay: 30 });
  await expect(page.locator("#results li")).toHaveText(["apple", "apricot", "grape"]);
  await expect.poll(shown).toBe("/?q=ap");
  // The URL changed without loading a page, so the field keeps its focus.
  await expect(field).toBeFocused();
  await page.evaluate(() => Object.assign(window, { before: true }));

  const loaded = page.waitForEvent("load");
  await page.goBack();
  await loaded;
  expect(shown()).toBe("/?q=berr");
  expect(await page.evaluate(() => "before" in window)).toBe(false);
  await expect(field).toHaveValue("berr");
  await expect(page.locator("#results li")).toHaveText(["blackberry", "blueberry"]);
  await expect(page).toHaveTitle("berr - hypersig showcase");

  await field.click();
  await page.keyboard.press("Escape");
  await expect.poll(shown).toBe("/");
  await expect(page.locator("#results li")).toHaveCount(13);
  await expect(page).toHaveTitle("hypersig showcase");
  const back = page.waitForEvent("load");
  await page.goBack();
  await back;
  expect(shown()).toBe("/?q=berr");
});

test("a statement opens a dialog as a modal and another closes it", async ({ page }) => {
  const dialog = page.getByRole("dialog", { name: "Commands" });
  await expect(dialog).toBeHidden();
  await page.locator(".open-palette").click();
  await expect(dialog).toBeVisible();
  expect(await dialog.evaluate((element) => element.matches(":modal"))).toBe(true);
  await expect(page.getByPlaceholder("Type a command")).toBeFocused();
  await page.locator(".close-palette").click();
  await expect(dialog).toBeHidden();
  await expect(page.locator(".open-palette")).toBeFocused();
});

test("a page shortcut runs from anywhere and leaves the characters a field types", async ({ page }) => {
  const field = page.getByPlaceholder("Search fruit");
  await page.keyboard.press("/");
  await expect(field).toBeFocused();
  await page.keyboard.type("b/k");
  await expect(field).toHaveValue("b/k");

  const dialog = page.getByRole("dialog", { name: "Commands" });
  await page.keyboard.press("ControlOrMeta+k");
  await expect(dialog).toBeVisible();
  const command = page.getByPlaceholder("Type a command");
  await expect(command).toBeFocused();
  await page.keyboard.type("k/");
  await expect(command).toHaveValue("k/");
  await page.keyboard.press("Escape");
  await expect(dialog).toBeHidden();
  await expect(field).toBeFocused();

  await page.locator("h1").click();
  await page.keyboard.press("ControlOrMeta+k");
  await expect(dialog).toBeVisible();
});

test("a list loads the next numbers each time the element after it scrolls into view", async ({ page }) => {
  const numbers = page.locator("#numbers li");
  const loads: string[] = [];
  page.on("request", (request) => {
    if (new URL(request.url()).pathname === "/more") loads.push(new URL(request.url()).searchParams.get("args") ?? "");
  });
  await expect(numbers).toHaveCount(20);
  expect(loads).toEqual([]);
  for (const count of [40, 60, 80, 100]) {
    await page.locator("#numbers .more").scrollIntoViewIfNeeded();
    await expect(numbers).toHaveCount(count);
  }
  await expect(page.locator("#numbers .end")).toBeVisible();
  await expect(page.locator("#numbers .more")).toBeHidden();
  expect(loads).toEqual(["20", "40", "60", "80"]);
});

test("a slow answer to an earlier search does not overwrite the answer to a later one", async ({ page }) => {
  const field = page.getByPlaceholder("Search fruit");
  const sent: string[] = [];
  const aborted: string[] = [];
  page.on("requestfailed", (request) => {
    if (request.url().endsWith("/search")) aborted.push(request.postData() ?? "");
  });
  await page.route("**/search", async (route) => {
    const query = route.request().postData() ?? "";
    sent.push(query);
    // Holds the first search back until the second has been answered.
    if (sent.length === 1) await new Promise((resolve) => setTimeout(resolve, 1500));
    await route.continue().catch(() => {});
  });
  await field.pressSequentially("b");
  await expect.poll(() => sent).toEqual(['"b"']);
  await field.pressSequentially("err");
  await expect(page.locator("#results li")).toHaveText(["blackberry", "blueberry"]);
  await expect.poll(() => aborted).toEqual(['"b"']);
  await page.waitForTimeout(1700);
  await expect(page.locator("#results li")).toHaveText(["blackberry", "blueberry"]);
  await expect(page.locator(".searching")).toBeHidden();
});

test("a form derives its total and disables submit while invalid or in flight", async ({ page }) => {
  await page.locator(".quantity").fill("3");
  await expect(page.locator(".price")).toHaveText("9.00 USD");
  await page.locator(".size").selectOption({ label: "Large" });
  await expect(page.locator(".price")).toHaveText("13.50 USD");
  await page.locator(".gift").check();
  await page.locator(".note").fill("x".repeat(41));
  await expect(page.locator(".count")).toHaveText("41 / 40");
  await expect(page.locator(".place")).toBeDisabled();
  await page.locator(".note").fill("ribbon 🎀");
  await expect(page.locator(".count")).toHaveText("8 / 40");
  await page.locator(".place").click();
  await expect(page.locator(".place")).toHaveText("Placing…");
  await expect(page.locator(".status")).toHaveText("Ordered 3 large, gift wrapped.");
  await expect(page.locator(".place")).toHaveText("Place order");
});

test("typed styles follow the viewport, a container, tokens a signal switches, and a signal's value", async ({ page }) => {
  const card = page.locator("#card");
  const frame = card.locator("..");
  const widthRatio = async () => {
    const [c, f] = await Promise.all([card.boundingBox(), frame.boundingBox()]);
    return Math.round((c!.width / f!.width) * 100);
  };

  // The breakpoint rule comes after the base rule, so it wins from 48rem.
  await page.setViewportSize({ width: 600, height: 800 });
  await expect.poll(widthRatio).toBe(100);
  await page.setViewportSize({ width: 1000, height: 800 });
  await expect.poll(widthRatio).toBe(50);

  // The note measures the card, which is narrower on the wider viewport.
  await expect(card.locator("p")).toHaveCSS("font-weight", "400");
  await page.setViewportSize({ width: 600, height: 800 });
  await expect(card.locator("p")).toHaveCSS("font-weight", "700");

  await expect(card).toHaveCSS("background-color", "rgb(246, 246, 246)");
  await page.locator("#theme").click();
  await expect(card).toHaveCSS("background-color", "rgb(34, 39, 51)");
  await expect(card).toHaveCSS("color", "rgb(238, 238, 238)");
  await page.locator("#theme").click();
  await expect(card).toHaveCSS("background-color", "rgb(246, 246, 246)");

  const badge = page.locator("#badge");
  await expect(badge).toHaveCSS("color", "rgb(128, 128, 128)");
  await page.locator("#loud").click();
  await expect(badge).toHaveText("LOUD");
  await expect(badge).toHaveCSS("color", "rgb(220, 20, 60)");
  await page.locator("#calm").click();
  await expect(badge).toHaveCSS("color", "rgb(128, 128, 128)");

  await expect(page.locator("#spinner")).toHaveCSS("animation-name", "spin");
});

test("the server adds, changes, and removes notes of the feed one at a time", async ({ page }) => {
  const items = page.locator(".feed li");
  const notes = page.locator(".feed li span");
  await expect(notes).toHaveCount(20);
  await items.first().evaluate((li) => li.setAttribute("data-kept", ""));
  const answers: Promise<string>[] = [];
  page.on("response", (response) => {
    if (response.url().includes("/feed/")) answers.push(response.text());
  });

  await page.locator(".newer").click();
  await expect(notes.last()).toHaveText("Note 61");
  await items.nth(1).locator(".star").click();
  await expect(notes.nth(1)).toHaveText("Note 42 (starred)");
  await items.nth(2).locator(".dismiss").click();
  await expect(notes).toHaveCount(20);
  await expect(notes.nth(2)).toHaveText("Note 44");
  // The element of the first note stayed, so the list changed in place.
  await expect(items.first()).toHaveAttribute("data-kept", "");
  // Each answer carries at most the one note it changes.
  const bodies = await Promise.all(answers);
  expect(bodies).toHaveLength(3);
  for (const body of bodies) expect(body.match(/"id":/g)?.length ?? 0).toBeLessThanOrEqual(1);
});

test("loading older notes keeps the notes in view where they were", async ({ page }) => {
  const feed = page.locator(".feed");
  const items = page.locator(".feed li");
  const top = (id: number) =>
    page
      .locator(".feed li span")
      .getByText(`Note ${id}`, { exact: true })
      .evaluate((span) => span.getBoundingClientRect().top);
  const scrollTop = () => feed.evaluate((element) => element.scrollTop);
  // `dispatchEvent` clicks without scrolling the button into view first.
  const older = () => page.locator(".older").dispatchEvent("click");

  // Chrome's own scroll anchoring does nothing while the feed is at the very top.
  expect(await scrollTop()).toBe(0);
  const first = await top(41);
  await older();
  await expect(items).toHaveCount(25);
  expect(await top(41)).toBeCloseTo(first, 0);
  expect(await scrollTop()).toBeGreaterThan(0);

  // Scrolled partway, Chrome anchors the content itself, and the runtime scrolls no further.
  await feed.evaluate((element) => (element.scrollTop -= 10));
  const partway = await top(36);
  await older();
  await expect(items).toHaveCount(30);
  expect(await top(36)).toBeCloseTo(partway, 0);
});
