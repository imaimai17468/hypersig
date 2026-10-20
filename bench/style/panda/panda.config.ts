import { defineConfig } from "@pandacss/dev";

export default defineConfig({
  presets: ["@pandacss/preset-base"],
  preflight: false,
  include: ["./styles.ts"],
  outdir: "styled-system",
  outExtension: "mjs",
  forceImportExtension: true,
  globalCss: {
    "*": { boxSizing: "border-box" },
    body: {
      font: "14px 'Helvetica Neue', Helvetica, Arial, sans-serif",
      background: "#f5f5f5",
      color: "#111",
      margin: "0",
    },
  },
});
