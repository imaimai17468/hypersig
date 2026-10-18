import { test as base } from "@playwright/test";
import { url } from "../playwright.config.ts";
import { expect, test, watch } from "./support.ts";

// The page fails two calls on purpose, so this test checks the failures it logs itself.
base("signs in with a cookie, shows the handler's error, and asks to sign in again once the session ends", async ({ page, context }) => {
  const problems = watch(page);
  await page.goto(url("auth"));
  await expect(page).toHaveURL(`${url("auth")}login`);

  const name = page.getByRole("textbox", { name: "Name", exact: true });
  const password = page.getByRole("textbox", { name: "Password", exact: true });
  await name.fill("ada");
  await password.fill("babbage");
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page.locator("#ui-credentials-value-password-error")).toHaveText("The name or the password is wrong.");

  await password.fill("lovelace");
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page).toHaveURL(url("auth"));
  await expect(page.locator("#member")).toHaveText("Signed in as ada");
  const session = (await context.cookies()).find((cookie) => cookie.name === "session");
  expect(session?.httpOnly).toBe(true);

  const note = page.getByRole("textbox", { name: "Note", exact: true });
  const notes = page.locator("#notes li");
  for (const text of ["one", "two", "three"]) {
    await note.fill(text);
    await page.getByRole("button", { name: "Add note" }).click();
    await expect(notes.last()).toHaveText(text);
  }
  await expect(note).toHaveValue("");
  await note.fill("four");
  await page.getByRole("button", { name: "Add note" }).click();
  await expect(page.locator("#problem")).toHaveText("409: You can keep 3 notes.");
  await expect(notes).toHaveCount(3);

  await context.clearCookies();
  await page.getByRole("button", { name: "Add note" }).click();
  await expect(page.locator("#problem")).toHaveText("401: Your session has ended. Sign in again.");
  await page.reload();
  await expect(page).toHaveURL(`${url("auth")}login`);

  const notesUrl = `${url("auth")}notes`;
  expect(problems.filter((problem) => problem.startsWith("4"))).toEqual([`409 ${notesUrl}`, `401 ${notesUrl}`]);
  expect(problems.filter((problem) => problem.startsWith("hypersig:"))).toEqual([
    "hypersig: POST /notes failed with 409: You can keep 3 notes.",
    "hypersig: POST /notes failed with 401: Your session has ended. Sign in again.",
  ]);
  expect(problems.filter((problem) => !problem.startsWith("4") && !problem.startsWith("hypersig:") && !problem.startsWith("Failed to load resource"))).toEqual([]);
});

test("signs out, which removes the cookie and the session", async ({ page, context }) => {
  await page.goto(`${url("auth")}login`);
  await page.getByRole("textbox", { name: "Name", exact: true }).fill("grace");
  await page.getByRole("textbox", { name: "Password", exact: true }).fill("hopper");
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page.locator("#member")).toHaveText("Signed in as grace");

  await page.getByRole("button", { name: "Sign out" }).click();
  await expect(page).toHaveURL(`${url("auth")}login`);
  expect((await context.cookies()).some((cookie) => cookie.name === "session")).toBe(false);
  await page.goto(url("auth"));
  await expect(page).toHaveURL(`${url("auth")}login`);
});

test("refuses an action called from another site or with a body that is not JSON", async ({ request }) => {
  const signIn = `${url("auth")}sign-in`;
  const credentials = JSON.stringify({ name: "ada", password: "lovelace" });

  const plain = await request.post(signIn, { data: credentials, headers: { "content-type": "text/plain" } });
  expect(plain.status()).toBe(415);

  const crossSite = await request.post(signIn, {
    data: credentials,
    headers: { "content-type": "application/json", "sec-fetch-site": "cross-site" },
  });
  expect(crossSite.status()).toBe(403);
  expect(crossSite.headers()["set-cookie"]).toBeUndefined();

  const sameOrigin = await request.post(signIn, {
    data: credentials,
    headers: { "content-type": "application/json", "sec-fetch-site": "same-origin" },
  });
  expect(sameOrigin.status()).toBe(200);
  expect(sameOrigin.headers()["set-cookie"]).toContain("HttpOnly");
});
