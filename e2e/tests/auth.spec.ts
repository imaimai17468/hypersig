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
  await name.fill("nobody");
  await password.fill("lovelace");
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page.locator("#ui-credentials-value-password-error")).toHaveText("The name or the password is wrong.");

  await name.fill("ada");
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
  await expect(page.locator("#problem")).toHaveText("You can keep 3 notes.");
  await expect(notes).toHaveCount(3);

  await context.clearCookies();
  await page.getByRole("button", { name: "Add note" }).click();
  await expect(page.locator("#problem")).toHaveText("Your session has ended. Sign in again.");
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

// The page answers 404 on purpose, so this test checks the failures it logs itself.
base("shows the router's page with a 404 for a path that names no page", async ({ page }) => {
  const problems = watch(page);
  const response = await page.goto(`${url("auth")}nowhere`);
  expect(response?.status()).toBe(404);
  await expect(page.locator("#missing")).toHaveText("There is no page here.");
  await expect(page.getByText("no page is served at `/nowhere`")).toBeVisible();
  await page.getByRole("link", { name: "Back to your notes" }).click();
  await expect(page).toHaveURL(`${url("auth")}login`);
  expect(problems.filter((problem) => !problem.startsWith("404") && !problem.startsWith("Failed to load resource"))).toEqual([]);
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

test("gives the session a new id at sign-in, so an id planted in the browser is not signed in", async ({ page, context, browser }) => {
  const signIn = async (name: string, password: string) => {
    await page.goto(`${url("auth")}login`);
    await page.getByRole("textbox", { name: "Name", exact: true }).fill(name);
    await page.getByRole("textbox", { name: "Password", exact: true }).fill(password);
    await page.getByRole("button", { name: "Sign in" }).click();
    await expect(page.locator("#member")).toHaveText(`Signed in as ${name}`);
  };
  const session = async () => (await context.cookies()).find((cookie) => cookie.name === "session");

  await signIn("grace", "hopper");
  const planted = await session();
  expect(planted).toMatchObject({ httpOnly: true, secure: true, sameSite: "Lax" });
  const other = await browser.newContext();
  await other.addCookies([planted!]);

  await signIn("ada", "lovelace");
  expect((await session())?.value).not.toBe(planted!.value);
  const planter = await other.newPage();
  await planter.goto(url("auth"));
  await expect(planter).toHaveURL(`${url("auth")}login`);
  await other.close();
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

/** The eight bytes every PNG file starts with, followed by a little more. */
const png = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 1, 2, 3, 4]);

// The page refuses one upload on purpose, so this test checks the failures it logs itself.
base("uploads a picture from the file input and shows the handler's refusal of another file", async ({ page }) => {
  const problems = watch(page);
  await page.goto(`${url("auth")}login`);
  await page.getByRole("textbox", { name: "Name", exact: true }).fill("ada");
  await page.getByRole("textbox", { name: "Password", exact: true }).fill("lovelace");
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page.locator("#member")).toHaveText("Signed in as ada");

  const file = page.getByLabel("Picture");
  await file.setInputFiles({ name: "cat.png", mimeType: "image/png", buffer: png });
  await page.getByRole("button", { name: "Upload picture" }).click();
  await expect(page.locator("#picture")).toHaveText("Your picture: cat.png, 12 bytes");
  await expect(file).toHaveValue("");

  await file.setInputFiles({ name: "notes.png", mimeType: "image/png", buffer: Buffer.from("not a picture") });
  await page.getByRole("button", { name: "Upload picture" }).click();
  await expect(page.locator("#picture-problem")).toHaveText("Choose one PNG or JPEG picture.");
  await expect(page.locator("#picture")).toHaveText("Your picture: cat.png, 12 bytes");

  await page.reload();
  await expect(page.locator("#picture")).toHaveText("Your picture: cat.png, 12 bytes");
  expect(problems.filter((problem) => problem.startsWith("hypersig:"))).toEqual([
    "hypersig: POST /picture failed with 422: Choose one PNG or JPEG picture.",
  ]);
  expect(problems.filter((problem) => !problem.startsWith("4") && !problem.startsWith("hypersig:") && !problem.startsWith("Failed to load resource"))).toEqual([]);
});

test("refuses an upload without the runtime's header, from another site, or over its limit", async ({ request }) => {
  const upload = `${url("auth")}picture`;
  const multipart = { files: { name: "cat.png", mimeType: "image/png", buffer: png } };

  const fromForm = await request.post(upload, { multipart });
  expect(fromForm.status()).toBe(403);

  const crossSite = await request.post(upload, { multipart, headers: { "hypersig-upload": "1", "sec-fetch-site": "cross-site" } });
  expect(crossSite.status()).toBe(403);

  const large = { files: { name: "big.png", mimeType: "image/png", buffer: Buffer.alloc(80 * 1024) } };
  const tooLarge = await request.post(upload, { multipart: large, headers: { "hypersig-upload": "1" } });
  expect(tooLarge.status()).toBe(413);

  // With the header and within the limit, the request reaches the member extractor, which finds no session.
  const signedOut = await request.post(upload, { multipart, headers: { "hypersig-upload": "1", accept: "text/event-stream" } });
  expect(signedOut.status()).toBe(401);
});
