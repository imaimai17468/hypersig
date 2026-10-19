import { expect, it } from "vitest";
import { $, key, tick } from "../dom.ts";
import { mount } from "./fixture.ts";

it("moves focus between the items of its own group, skips a disabled item, and wraps", async () => {
  const runtime = mount("roving");
  await tick();
  expect($("#tab0").getAttribute("tabindex")).toBe("0");
  expect($("#tab1").getAttribute("tabindex")).toBe("-1");

  $("#tab0").focus();
  key($("#tab0"), "ArrowRight");
  expect(document.activeElement?.id).toBe("tab1");
  await tick();
  expect(runtime.store.get("tab")).toBe(1);
  expect($("#tab1").getAttribute("tabindex")).toBe("0");

  key($("#tab1"), "ArrowRight");
  expect(document.activeElement?.id).toBe("tab3");
  key($("#tab3"), "ArrowLeft");
  expect(document.activeElement?.id).toBe("tab1");
  key($("#tab1"), "End");
  expect(document.activeElement?.id).toBe("tab3");
  key($("#tab3"), "Home");
  expect(document.activeElement?.id).toBe("tab0");
  key($("#tab0"), "ArrowLeft");
  expect(document.activeElement?.id).toBe("tab3");
  key($("#tab3"), "ArrowUp");
  expect(document.activeElement?.id).toBe("tab3");
});

it("leaves a key to the handler that took it and to a held modifier", () => {
  const runtime = mount("roving");
  $("#tab3").focus();
  key($("#tab3"), "ArrowRight");
  expect(document.activeElement?.id).toBe("tab3");
  expect(runtime.store.get("taken")).toBe(1);

  $("#tab0").focus();
  $("#tab0").dispatchEvent(
    new KeyboardEvent("keydown", {
      key: "ArrowRight",
      altKey: true,
      bubbles: true,
      cancelable: true,
    }),
  );
  expect(document.activeElement?.id).toBe("tab0");
});

it("gives a nested group its own items and keys", () => {
  mount("roving");
  $("#item0").focus();
  key($("#item0"), "ArrowDown");
  expect(document.activeElement?.id).toBe("item1");
  key($("#item1"), "ArrowDown");
  expect(document.activeElement?.id).toBe("item0");
  key($("#item0"), "ArrowRight");
  expect(document.activeElement?.id).toBe("item0");
});
