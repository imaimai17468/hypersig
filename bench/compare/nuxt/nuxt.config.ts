import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

// Every app in the benchmark inlines the same stylesheet. It is read here at
// build time and handed to the server as a virtual module, so it reaches the
// HTML without also being bundled into the client JavaScript.
const css = readFileSync(fileURLToPath(new URL("../shared/todomvc.css", import.meta.url)), "utf8");

export default defineNuxtConfig({
  compatibilityDate: "2026-10-01",
  telemetry: false,
  devtools: { enabled: false },
  app: {
    head: {
      title: "Nuxt TodoMVC",
      htmlAttrs: { lang: "en" },
    },
  },
  nitro: {
    virtual: {
      "#todomvc-css": `export default ${JSON.stringify(css)};`,
    },
  },
});
