import { expect, it } from "vitest";
import { getPath, setPath } from "../../src/store/path.ts";

it("setPath copies each container on the way and leaves the value it was given unchanged", () => {
  const value = { rows: [{ id: 1, tags: ["a"] }], name: "n" };
  const next = setPath(value, "rows.0.tags.1", "b") as typeof value;
  expect(next).toEqual({ rows: [{ id: 1, tags: ["a", "b"] }], name: "n" });
  expect(value).toEqual({ rows: [{ id: 1, tags: ["a"] }], name: "n" });
  expect(next.rows).not.toBe(value.rows);
  expect(setPath(null, "a.b", 1)).toEqual({ a: { b: 1 } });
  expect(setPath(value, "", 2)).toBe(2);
});

it("getPath reads one, two, and more segments, and nothing through a scalar", () => {
  const value = { rows: [{ id: 1, tags: ["a"] }], n: 2 };
  expect(getPath(value, "n")).toBe(2);
  expect(getPath(value, "rows.0")).toEqual({ id: 1, tags: ["a"] });
  expect(getPath(value, "rows.0.tags.0")).toBe("a");
  expect(getPath(value, "n.x")).toBeUndefined();
  expect(getPath(value, "missing.0")).toBeUndefined();
  expect(getPath(null, "a.b")).toBeUndefined();
});
