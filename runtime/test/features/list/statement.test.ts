import { afterEach, expect, it, vi } from "vitest";
import { $, $$, tick } from "../../dom.ts";
import { mount } from "../fixture.ts";

afterEach(() => {
  vi.restoreAllMocks();
});

const labels = () => $$("#lines li").map((li) => li.textContent);

it("creates, updates, swaps, and sorts items, and reads random numbers and the clock", async () => {
  const runtime = mount("table");
  const ids = () => (runtime.store.get("lines") as { id: number }[]).map((line) => line.id);
  $("#run").click();
  await tick();
  expect(ids()).toEqual([1, 2, 3]);
  expect(runtime.store.get("next_id")).toBe(4);
  for (const label of labels()) expect(["a", "b"]).toContain(label);

  const before = labels();
  $("#update").click();
  await tick();
  expect(labels()).toEqual([`${before[0]} !`, before[1], `${before[2]} !`]);

  const rows = $$("#lines li");
  $("#swap").click();
  await tick();
  expect(ids()).toEqual([3, 2, 1]);
  expect($$("#lines li")).toEqual([rows[2], rows[1], rows[0]]);
  $("#sort").click();
  await tick();
  expect(ids()).toEqual([1, 2, 3]);
  expect($$("#lines li")).toEqual(rows);
  $("#sort-desc").click();
  await tick();
  expect(ids()).toEqual([3, 2, 1]);
  expect($$("#lines li")).toEqual([rows[2], rows[1], rows[0]]);
  $("#sort").click();
  await tick();

  const lines = Array.from({ length: 10 }, (_, index) => ({ id: index + 1, label: "a" }));
  runtime.store.set("lines", lines);
  await tick();
  const ten = $$("#lines li");
  const moves = vi.spyOn($("#lines"), "insertBefore");
  runtime.store.set("lines", lines.with(1, lines[8]!).with(8, lines[1]!));
  await tick();
  expect($$("#lines li")).toEqual(ten.with(1, ten[8]!).with(8, ten[1]!));
  expect(moves).toHaveBeenCalledTimes(2);

  $("#stamp").click();
  expect(Math.abs((runtime.store.get("stamp") as number) - Date.now())).toBeLessThan(1000);
});
