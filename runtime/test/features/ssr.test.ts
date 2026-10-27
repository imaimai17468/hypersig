import { expect, it } from "vitest";
import { $, $$, tick } from "../dom.ts";
import { serve, start } from "./fixture.ts";

it("shows list items, counts, and visibility before the runtime starts, and the runtime adopts them", async () => {
  serve("todos_page");
  const item = $("#list li");
  expect($$("#list li label").map((label) => label.textContent)).toEqual(["tea"]);
  expect($("#left").textContent).toBe("1");
  expect($("#clear").dataset.hsHide).toBeDefined();
  expect($("#clear").getAttribute("style")).toBeNull();
  expect(getComputedStyle($("#clear")).display).toBe("none");

  const runtime = start();
  await tick();
  expect($("#clear").dataset.hsHide).toBeUndefined();
  expect($("#clear").style.display).toBe("none");
  expect($$("#list li")).toEqual([item]);
  item.querySelector<HTMLInputElement>("input[type=checkbox]")!.click();
  await tick();
  expect(runtime.store.get("todos.0.done")).toBe(true);
  expect(item.classList.contains("done")).toBe(true);
  expect($("#left").textContent).toBe("0");

  item.querySelector("label")!.dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
  await tick();
  expect(item.querySelector<HTMLInputElement>("input.edit")!.value).toBe("tea");
});

it("registers the bindings of server-rendered items in a later task and shows changes made before it", async () => {
  serve("todos_page");
  const item = $("#list li");
  const runtime = start();
  runtime.store.set("todos.0.done", true);
  await Promise.resolve();
  expect(item.classList.contains("done")).toBe(false);
  await tick();
  expect(item.classList.contains("done")).toBe(true);
  expect($$("#list li")).toEqual([item]);
});

it("registers a waiting item's bindings at once when an event reaches it", async () => {
  serve("todos_page");
  const item = $("#list li");
  const runtime = start();
  item.querySelector<HTMLInputElement>("input[type=checkbox]")!.click();
  expect(runtime.store.get("todos.0.done")).toBe(true);
  await Promise.resolve();
  expect(item.classList.contains("done")).toBe(true);

  serve("nested_page");
  const nested = start();
  $$("#rows button")[2]!.click();
  expect(nested.store.get("rows.1.tags.0.on")).toBe(true);
});

it("adopts nested list items with their own item scope", async () => {
  serve("nested_page");
  expect($$("#rows button").map((b) => b.textContent)).toEqual(["a/x", "a/y", "b/z"]);
  expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "0"]);
  expect($$("#rows button")[0]!.classList.contains("on")).toBe(true);
  const buttons = $$("#rows button");

  const runtime = start();
  await tick();
  expect($$("#rows button")).toEqual(buttons);
  buttons[2]!.click();
  await tick();
  expect(runtime.store.get("rows.1.tags.0.on")).toBe(true);
  expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "1"]);
});

it("writes form values, attributes, and styles on the server", () => {
  serve("controls_page");
  expect($<HTMLInputElement>("#quantity").value).toBe("1");
  expect($<HTMLInputElement>("#price").value).toBe("2.5");
  expect($<HTMLInputElement>("#note").value).toBe("gift wrap");
  expect($<HTMLInputElement>("#limit").hasAttribute("value")).toBe(false);
  expect($<HTMLInputElement>("#due").value).toBe("2026-01-05");
  expect($<HTMLInputElement>("#until").hasAttribute("value")).toBe(false);
  expect($<HTMLSelectElement>("#size").value).toBe('"Small"');
  expect($<HTMLInputElement>("#small").checked).toBe(true);
  expect($<HTMLInputElement>("#large").checked).toBe(false);
  expect($("#buy").getAttribute("title")).toBe("5.00");
  expect($("#buy").style.getPropertyValue("width")).toBe("1rem");
  expect($("#total").textContent).toBe("small");
  expect($("#price-label").textContent).toBe("$2,500.00");
});
