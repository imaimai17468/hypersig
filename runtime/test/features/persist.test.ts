import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, expect, it, vi } from "vitest";
import { keep, type Persisted, restore } from "../../src/persist.ts";
import { Runtime } from "../../src/runtime.ts";
import type { Values } from "../../src/store/value.ts";
import { $, tick } from "../dom.ts";
import { serve } from "./fixture.ts";

const KEY = "hypersig:/dark";

afterEach(() => {
  localStorage.clear();
  delete document.documentElement.dataset.hsRestoring;
  Reflect.deleteProperty(document, "startViewTransition");
  // A runtime listens on `<body>`, so a fresh one keeps this test's clicks
  // away from the runtimes of the tests before it.
  document.documentElement.replaceChild(document.createElement("body"), document.body);
});

/** Whether `<html>` carries the mark that hides the page until the runtime starts. */
function restoring(): boolean {
  return document.documentElement.dataset.hsRestoring !== undefined;
}

/** Starts the runtime as `runtime/src/index.ts` does, over the stored values. */
function startPersisted(): Runtime {
  const body = document.body;
  const persisted = JSON.parse(body.dataset.hsPersist!) as Persisted;
  const runtime = new Runtime(
    body,
    restore(JSON.parse(body.dataset.hsSignals!) as Values, persisted),
  );
  keep(runtime.store, persisted);
  return runtime;
}

it("starts from the stored value, renders it, and shows the page", () => {
  serve("persisted_page");
  localStorage.setItem(KEY, "true");
  document.documentElement.dataset.hsRestoring = "";
  expect($("#dark").textContent).toBe("false");
  const runtime = startPersisted();
  expect(runtime.store.get("dark")).toBe(true);
  expect($("#dark").textContent).toBe("true");
  expect(document.body.classList.length).toBeGreaterThan(0);
  expect(restoring()).toBe(false);
});

it("stores each write of a persisted signal and leaves the others out", async () => {
  serve("persisted_page");
  expect(document.body.classList.length).toBe(0);
  const runtime = startPersisted();
  $("#flip").click();
  await tick();
  expect(runtime.store.get("clicks")).toBe(1);
  expect(localStorage.getItem(KEY)).toBe("true");
  expect(localStorage.length).toBe(1);
  expect(document.body.classList.length).toBeGreaterThan(0);
});

it("ignores a stored value that does not parse or has another JSON type", () => {
  serve("persisted_page");
  localStorage.setItem(KEY, "{");
  expect(startPersisted().store.get("dark")).toBe(false);
  serve("persisted_page");
  localStorage.setItem(KEY, '"dark"');
  expect(startPersisted().store.get("dark")).toBe(false);
});

it("renders the statement's writes inside the browser's view transition", async () => {
  serve("persisted_page");
  const seen: string[] = [];
  const startViewTransition = vi.fn<(update: () => void) => void>((update) => {
    seen.push($("#dark").textContent!);
    update();
    seen.push($("#dark").textContent!);
  });
  Object.defineProperty(document, "startViewTransition", {
    value: startViewTransition,
    configurable: true,
  });
  const runtime = startPersisted();
  $("#flip").click();
  expect(startViewTransition).toHaveBeenCalledOnce();
  expect(seen).toEqual(["false", "true"]);
  await tick();
  expect(runtime.store.get("clicks")).toBe(1);
});

it("runs the statement without a transition where the browser has none", async () => {
  serve("persisted_page");
  const runtime = startPersisted();
  $("#flip").click();
  await tick();
  expect($("#dark").textContent).toBe("true");
  expect(runtime.store.get("clicks")).toBe(1);
});

/** Runs the script `Page::persist` writes into `<head>`, which a parsed fixture does not run. */
function runHeadScript(): void {
  const { document: html } = JSON.parse(
    readFileSync(resolve(__dirname, "../fixtures/persisted_page.json"), "utf8"),
  ) as { document: string };
  const parsed = new DOMParser().parseFromString(html, "text/html");
  const script = parsed.head.querySelector("script:not([type])")!;
  new Function(script.textContent!)();
}

it("hides the page before it renders only where a stored value differs from the initial one", () => {
  runHeadScript();
  expect(restoring()).toBe(false);
  localStorage.setItem(KEY, "false");
  runHeadScript();
  expect(restoring()).toBe(false);
  localStorage.setItem(KEY, "true");
  runHeadScript();
  expect(restoring()).toBe(true);
});
