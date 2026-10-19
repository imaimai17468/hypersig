import { expect, it } from "vitest";
import { $, $$, tick } from "../dom.ts";
import { mount, serve, start } from "./fixture.ts";

const texts = () => $$("#values span, #dates span").map((span) => [span.id, span.textContent]);

it("writes the same text on the server and in the browser", async () => {
  serve("values_page");
  const served = texts();
  expect(served.every(([, text]) => text !== "")).toBe(true);
  start();
  await tick();
  expect(texts()).toEqual(served);
});

it("writes dates and relative times with Intl, and leaves the browser's zone to the runtime", async () => {
  serve("values_page");
  expect($("#date-local").textContent).toBe("");
  expect(Object.fromEntries(texts())).toMatchObject({
    "date-en-full": "Monday, January 5, 2026 at 3:04:05 PM",
    "date-en-medium": "Jan 5, 2026, 3:04 PM",
    "date-ja-full": "2026年1月5日月曜日 15:04:05",
    "date-tokyo": "2026/01/06 0:04",
    "ago-en": "3 minutes ago",
    "ago-ja": "3 分前",
  });
  const runtime = start();
  await tick();
  expect($("#date-local").textContent).toBe(
    new Intl.DateTimeFormat("en", { dateStyle: "medium", timeStyle: "short" }).format(
      1_767_625_445_000,
    ),
  );
  runtime.store.set("now", 1_767_625_445_000 + 2 * 86_400_000);
  await tick();
  expect([$("#ago-en").textContent, $("#ago-ja").textContent]).toEqual(["2 days ago", "2 日前"]);
});

it("labels each variant of an enum", async () => {
  const runtime = mount("values");
  await tick();
  expect($("#status").textContent).toBe("In review");
  runtime.store.set("status", "closed");
  await tick();
  expect($("#status").textContent).toBe("Closed");
  runtime.store.set("status", "open");
  await tick();
  expect($("#status").textContent).toBe("Open");
});

it("picks a plural form for the count", async () => {
  const runtime = mount("values");
  await tick();
  expect([$("#items-en").textContent, $("#items-ja").textContent]).toEqual(["1 item", "1 点"]);
  runtime.store.set("count", 3);
  await tick();
  expect([$("#items-en").textContent, $("#items-ja").textContent]).toEqual(["3 items", "3 点"]);
});

it("tests the characters of a string by ASCII class", async () => {
  const runtime = mount("values");
  await tick();
  expect(Object.fromEntries(texts())).toMatchObject({
    "zip-digits": "yes",
    "code-digits": "no",
    "code-upper": "yes",
    "code-space": "yes",
  });
  runtime.store.set("zip", "15ー0001");
  runtime.store.set("code", "１２");
  await tick();
  expect(Object.fromEntries(texts())).toMatchObject({
    "zip-digits": "no",
    "code-upper": "no",
    "code-space": "no",
  });
  runtime.store.set("zip", "");
  await tick();
  expect($("#zip-digits").textContent).toBe("yes");
});
