// Builds crates/hypersig/assets/hypersig.js from src/ and writes its Brotli copy next to it,
// and builds crates/hypersig/assets/dev.js, which only a debug build serves, from src/dev.ts.
import { writeFileSync } from "node:fs";
import { brotliCompressSync, constants } from "node:zlib";
import { build } from "esbuild";
import { minify } from "terser";

const assets = new URL("../../crates/hypersig/assets/", import.meta.url);

/** Bundles and minifies the module at `entry`, under the comment `banner`. */
async function bundle(entry, banner) {
  const bundled = await build({
    entryPoints: [new URL(entry, import.meta.url).pathname],
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
  return Buffer.from(`${minified.code}\n`);
}

const js = await bundle(
  "../src/index.ts",
  "// hypersig runtime. Built from runtime/src by pnpm build; do not edit.",
);
writeFileSync(new URL("hypersig.js", assets), js);
writeFileSync(
  new URL("hypersig.js.br", assets),
  brotliCompressSync(js, {
    params: {
      [constants.BROTLI_PARAM_MODE]: constants.BROTLI_MODE_TEXT,
      [constants.BROTLI_PARAM_QUALITY]: constants.BROTLI_MAX_QUALITY,
      [constants.BROTLI_PARAM_SIZE_HINT]: js.length,
    },
  }),
);
console.log(`hypersig.js ${js.length} B`);

const dev = await bundle(
  "../src/dev.ts",
  "// hypersig dev script. Built from runtime/src/dev.ts by pnpm build; do not edit.",
);
writeFileSync(new URL("dev.js", assets), dev);
console.log(`dev.js ${dev.length} B`);
