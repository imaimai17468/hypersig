import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { defineConfig, type Plugin } from "vite";
import react from "@vitejs/plugin-react";

// Every app in the benchmark inlines the same stylesheet. This reads the shared
// file at build time so it stays the single source.
const sharedCss = fileURLToPath(new URL("../shared/todomvc.css", import.meta.url));

function inlineSharedCss(): Plugin {
  return {
    name: "inline-shared-css",
    transformIndexHtml() {
      return [{ tag: "style", children: readFileSync(sharedCss, "utf8"), injectTo: "head" }];
    },
  };
}

export default defineConfig({
  plugins: [react(), inlineSharedCss()],
});
