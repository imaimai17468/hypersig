import { expect, it } from "vitest";
import { $, tick, type } from "../dom.ts";
import { mount } from "./fixture.ts";

it("binds numbers, enum selects, and radios", async () => {
  const runtime = mount("controls");
  expect($<HTMLInputElement>("#quantity").value).toBe("1");
  expect($<HTMLSelectElement>("#size").value).toBe('"Small"');
  expect($<HTMLInputElement>("#small").checked).toBe(true);

  type($<HTMLInputElement>("#quantity"), "3.7");
  expect(runtime.store.get("quantity")).toBe(3);
  type($<HTMLInputElement>("#price"), "1.25");
  expect(runtime.store.get("price")).toBe(1.25);
  await tick();
  expect($("#price-label").textContent).toBe("$1,250.00");

  type($<HTMLSelectElement>("#size"), '"Large"', "change");
  await tick();
  expect(runtime.store.get("size")).toBe("Large");
  expect($<HTMLInputElement>("#large").checked).toBe(true);
  expect($("#total").textContent).toBe("large");

  const small = $<HTMLInputElement>("#small");
  small.checked = true;
  small.dispatchEvent(new Event("change", { bubbles: true }));
  await tick();
  expect($<HTMLSelectElement>("#size").value).toBe('"Small"');
});

it("binds an empty field to None in both directions for an Option signal", async () => {
  const runtime = mount("controls");
  expect($<HTMLInputElement>("#note").value).toBe("gift wrap");
  expect($<HTMLInputElement>("#limit").value).toBe("");

  type($<HTMLInputElement>("#note"), "");
  expect(runtime.store.get("note")).toBe(null);
  type($<HTMLInputElement>("#note"), "ribbon");
  expect(runtime.store.get("note")).toBe("ribbon");

  type($<HTMLInputElement>("#limit"), "12.9");
  expect(runtime.store.get("limit")).toBe(12);
  type($<HTMLInputElement>("#limit"), "  ");
  expect(runtime.store.get("limit")).toBe(null);
  type($<HTMLInputElement>("#quantity"), "");
  expect(runtime.store.get("quantity")).toBe(1);

  runtime.store.set("note", null);
  runtime.store.set("limit", 7);
  await tick();
  expect($<HTMLInputElement>("#note").value).toBe("");
  expect($<HTMLInputElement>("#limit").value).toBe("7");
});

it("binds attributes, boolean attributes, and styles", async () => {
  const runtime = mount("controls");
  const buy = $<HTMLButtonElement>("#buy");
  expect(buy.hasAttribute("disabled")).toBe(false);
  expect(buy.getAttribute("title")).toBe("5.00");
  expect(buy.style.getPropertyValue("width")).toBe("1rem");
  runtime.store.set("quantity", 0);
  await tick();
  expect(buy.hasAttribute("disabled")).toBe(true);
  expect(buy.style.getPropertyValue("width")).toBe("0rem");
});
