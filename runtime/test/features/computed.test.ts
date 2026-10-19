import { expect, it, vi } from "vitest";
import { $, tick } from "../dom.ts";
import { serve, start } from "./fixture.ts";

it("computes signals on the server, recomputes them in order in the browser, and reruns readers only when their value changes", async () => {
  serve("computed_page");
  expect($("#sum").textContent).toBe("3");
  expect($("#doubled").textContent).toBe("6");
  const runtime = start();
  await tick();

  runtime.store.set("a", 5);
  await vi.waitFor(() => expect($("#doubled").textContent).toBe("14"));
  expect($("#sum").textContent).toBe("7");

  const written = new Set<Node>();
  const observer = new MutationObserver((records) => {
    for (const record of records) written.add(record.target);
  });
  observer.observe($("#doubled"), { childList: true, characterData: true, subtree: true });
  runtime.store.set("a", 4);
  runtime.store.set("b", 3);
  await tick();
  await tick();
  expect(runtime.store.get("sum")).toBe(7);
  expect(written.size).toBe(0);
  observer.disconnect();
});

it("writes the title on the server and rewrites it when a computed signal it reads changes", async () => {
  serve("computed_page");
  expect(document.title).toBe("6 doubled");
  const runtime = start();
  await tick();
  expect(document.title).toBe("6 doubled");

  runtime.store.set("b", 4);
  await vi.waitFor(() => expect(document.title).toBe("10 doubled"));
});
