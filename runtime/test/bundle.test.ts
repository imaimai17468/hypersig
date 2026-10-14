import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it } from "vitest";

it("the built bundle evaluates no strings as code", () => {
  const bundle = readFileSync(resolve(__dirname, "../../crates/hypersig/assets/hypersig.js"), "utf8");
  expect(bundle.length).toBeGreaterThan(1000);
  expect(bundle).not.toMatch(/\beval\s*\(/);
  expect(bundle).not.toMatch(/new\s+Function\b/);
  expect(bundle).not.toMatch(/\bFunction\s*\(/);
});
