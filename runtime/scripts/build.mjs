// Builds crates/hypersig/assets/hypersig.js from src/.
import { writeFileSync } from "node:fs";
import { build } from "esbuild";
import { minify } from "terser";

const out = new URL("../../crates/hypersig/assets/hypersig.js", import.meta.url);
const banner = "// hypersig runtime. Built from runtime/src by pnpm build; do not edit.";

const bundled = await build({
  entryPoints: [new URL("../src/index.ts", import.meta.url).pathname],
  bundle: true,
  format: "esm",
  target: "es2022",
  minify: true,
  legalComments: "none",
  write: false,
});
// esbuild minifies in one pass. terser's further passes inline and drop what the first pass left behind.
const minified = await minify(bundled.outputFiles[0].text, {
  module: true,
  ecma: 2020,
  compress: { passes: 3 },
  format: { preamble: banner },
});
const js = Buffer.from(`${minified.code}\n`);
writeFileSync(out, js);
console.log(`hypersig.js ${js.length} B`);
