import { expect, it } from "vitest";
import { $$, tick } from "../../dom.ts";
import { mount } from "../fixture.ts";

it("resolves item fields and counts one list deep inside another", async () => {
  const runtime = mount("nested");
  expect($$("#rows button").map((b) => b.textContent)).toEqual(["a/x", "a/y", "b/z"]);
  expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "0"]);

  $$("#rows button")[2]!.click();
  await tick();
  expect(runtime.store.get("rows.1.tags.0.on")).toBe(true);
  expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "1"]);
  expect($$("#rows button")[2]!.classList.contains("on")).toBe(true);

  const [first, second] = runtime.store.get("rows") as unknown[];
  runtime.store.set("rows", [second, first]);
  await tick();
  expect($$("#rows button").map((b) => b.textContent)).toEqual(["b/z", "a/x", "a/y"]);
  $$("#rows button")[2]!.click();
  await tick();
  expect(runtime.store.get("rows.1.tags.1.on")).toBe(true);
  expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "2"]);
});
