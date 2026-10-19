// Builds crates/hypersig/assets/hypersig.js from src/ and writes its Brotli copy next to it.
import { writeFileSync } from "node:fs";
import { brotliCompressSync, constants } from "node:zlib";
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
writeFileSync(
  new URL("hypersig.js.br", out),
  brotliCompressSync(js, {
    params: {
      [constants.BROTLI_PARAM_MODE]: constants.BROTLI_MODE_TEXT,
      [constants.BROTLI_PARAM_QUALITY]: constants.BROTLI_MAX_QUALITY,
      [constants.BROTLI_PARAM_SIZE_HINT]: js.length,
    },
  }),
);
console.log(`hypersig.js ${js.length} B`);
