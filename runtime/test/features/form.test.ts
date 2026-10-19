import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { $, tick, type } from "../dom.ts";
import type { Fetch } from "../runtime/mount.ts";
import { mount, serve, start } from "./fixture.ts";

// A runtime started with `start()` listens on `document.body`, and it would run this form's
// submit handler against its own store.
beforeEach(() => document.body.replaceWith(document.createElement("body")));

afterEach(() => {
  vi.unstubAllGlobals();
});

function blur(target: Element): void {
  target.dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
}

function submit(): void {
  $("#join").dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
}

it("shows a rule's message once the field is left, and tracks whether the values changed", async () => {
  mount("join");
  expect($("#email-error").textContent).toBe("");
  expect($("#dirty").style.display).toBe("none");

  type($<HTMLInputElement>("#email"), "ada");
  await tick();
  expect($("#email-error").textContent).toBe("");
  expect($("#dirty").style.display).toBe("");
  blur($("#email"));
  await tick();
  expect($("#email-error").textContent).toBe("That is not an email address.");

  type($<HTMLInputElement>("#email"), "");
  await tick();
  expect($("#email-error").textContent).toBe("Enter an email address.");
  type($<HTMLInputElement>("#email"), "ada@example.com");
  await tick();
  expect($("#email-error").textContent).toBe("");
  expect($("#dirty").style.display).toBe("none");
});

it("touches every field on submit and calls the action only once every rule passes", async () => {
  const fetch = vi.fn<Fetch>(() => Promise.resolve(new Response("", { status: 200 })));
  vi.stubGlobal("fetch", fetch);
  const runtime = mount("join");
  submit();
  await tick();
  expect(fetch).not.toHaveBeenCalled();
  expect(runtime.store.get("signup.submitted")).toBe(true);
  expect($("#password-error").textContent).toBe("Use 8 or more characters.");

  type($<HTMLInputElement>("#password"), "correct horse");
  await tick();
  expect($("#password-error").textContent).toBe("");
  expect($("#valid").style.display).toBe("");
  submit();
  await tick();
  expect(fetch).toHaveBeenCalledTimes(1);
  const [, init] = fetch.mock.calls[0]!;
  expect(JSON.parse(init.body as string)).toEqual({
    email: "ada@example.com",
    password: "correct horse",
  });
});

it("shows a message the server sent until the next submit, and reset puts the starting values back", async () => {
  vi.stubGlobal(
    "fetch",
    vi.fn<Fetch>(() => Promise.resolve(new Response("", { status: 200 }))),
  );
  const runtime = mount("join");
  runtime.store.set("signup.errors", { email: "That address is taken." });
  await tick();
  expect($("#email-error").textContent).toBe("That address is taken.");
  submit();
  await tick();
  expect($("#email-error").textContent).toBe("");

  type($<HTMLInputElement>("#email"), "grace");
  $("#reset").click();
  await tick();
  expect($<HTMLInputElement>("#email").value).toBe("ada@example.com");
  expect(runtime.store.get("signup.touched")).toEqual({});
  expect(runtime.store.get("signup.submitted")).toBe(false);
  expect($("#password-error").textContent).toBe("");
});

it("renders the form on the server with no message and the runtime adopts it", async () => {
  serve("join_page");
  expect($("#email-error").textContent).toBe("");
  expect(getComputedStyle($("#dirty")).display).toBe("none");
  expect(getComputedStyle($("#valid")).display).toBe("none");
  start();
  type($<HTMLInputElement>("#email"), "x");
  blur($("#email"));
  await tick();
  expect($("#email-error").textContent).toBe("That is not an email address.");
});
