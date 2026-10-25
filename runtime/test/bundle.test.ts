import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { brotliDecompressSync } from "node:zlib";
import { expect, it } from "vitest";

it.each(["hypersig.js", "dev.js"])("the built %s evaluates no strings as code", (file) => {
  const bundle = readFileSync(resolve(__dirname, "../../crates/hypersig/assets", file), "utf8");
  expect(bundle.length).toBeGreaterThan(1000);
  expect(bundle).not.toMatch(/\beval\s*\(/u);
  expect(bundle).not.toMatch(/new\s+Function\b/u);
  expect(bundle).not.toMatch(/\bFunction\s*\(/u);
});

it("the runtime carries none of the dev script", () => {
  const bundle = readFileSync(
    resolve(__dirname, "../../crates/hypersig/assets/hypersig.js"),
    "utf8",
  );
  expect(bundle).not.toContain("hs-reload");
  expect(bundle).not.toContain("sessionStorage");
});

it("the committed Brotli copy decompresses to the built bundle", () => {
  const assets = resolve(__dirname, "../../crates/hypersig/assets");
  const bundle = readFileSync(resolve(assets, "hypersig.js"));
  expect(brotliDecompressSync(readFileSync(resolve(assets, "hypersig.js.br")))).toEqual(bundle);
});
