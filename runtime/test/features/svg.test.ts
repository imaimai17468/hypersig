import { expect, it } from "vitest";
import type { Runtime } from "../../src/runtime.ts";
import { $, $$, tick } from "../dom.ts";
import { mount, serve, start } from "./fixture.ts";

const heights = () => $$("#plot g > rect").map((rect) => rect.getAttribute("height"));
const marks = () => $$("#plot g").map((g) => g.querySelectorAll("line").length);

/** Checks the plot `runtime` rendered, then changes its bars and checks the lists and conditions again. */
async function exercise(runtime: Runtime): Promise<void> {
  expect(document.querySelectorAll("#plot template").length).toBeGreaterThan(0);
  for (const template of document.querySelectorAll("#plot template"))
    expect(template).toBeInstanceOf(HTMLTemplateElement);
  expect(heights()).toEqual(["10", "20"]);
  expect(marks()).toEqual([2, 0]);
  expect($("#extent").textContent).toBe("7 10 3 30");
  expect(
    $$("#plot g > rect").every((rect) => rect.namespaceURI === "http://www.w3.org/2000/svg"),
  ).toBe(true);

  runtime.store.set("bars", [
    { id: 2, height: 25, marks: [3] },
    { id: 1, height: 10, marks: [1, 2] },
    { id: 3, height: 5, marks: [] },
  ]);
  await tick();
  expect(heights()).toEqual(["25", "10", "5"]);
  expect($("#extent").textContent).toBe("8 10 2 40");
  expect($("#second").textContent).toBe("1");
  $("#collect").click();
  $("#collect").click();
  await tick();
  expect(runtime.store.get("collected")).toEqual([2, 1, 3, 2, 1, 3]);
  expect($("#collected").textContent).toBe("6");
  expect(marks()).toEqual([1, 2, 0]);
  expect($$("#plot g line").map((line) => line.getAttribute("y1"))).toEqual(["3", "1", "2"]);

  expect($("#frame").style.display).toBe("none");
  expect(document.querySelector("#dot")).toBeNull();
  $("#mark").click();
  await tick();
  expect($("#frame").style.display).toBe("");
  expect(document.querySelector("#dot")?.namespaceURI).toBe("http://www.w3.org/2000/svg");
}

it("renders lists and conditions inside svg from templates the parser read as svg elements", async () => {
  expect.hasAssertions();
  await exercise(mount("plot"));
});

it("adopts the items inside svg that the server rendered", async () => {
  serve("plot_page");
  expect($("#extent").textContent).toBe("7 10 3 30");
  expect($("#second").textContent).toBe("2");
  const first = $$("#plot > g")[0];
  const runtime = start();
  await tick();
  expect($$("#plot > g")[0]).toBe(first);
  await exercise(runtime);
});
