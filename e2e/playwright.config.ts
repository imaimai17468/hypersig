import { defineConfig } from "@playwright/test";

/** Each example runs as its own server on its own port, built once in release mode. */
const examples = { counter: 3100, todo: 3101, todomvc: 3102, showcase: 3103, style: 3104, ui: 3105, signup: 3106, auth: 3107, storefront: 3108, prefix: 3109 } as const;

export const url = (example: keyof typeof examples): string => `http://127.0.0.1:${examples[example]}/`;

export default defineConfig({
  testDir: "tests",
  fullyParallel: false,
  workers: 1,
  forbidOnly: Boolean(process.env.CI),
  reporter: process.env.CI ? "github" : "list",
  use: {
    // Locally, drive the installed Chrome. CI installs Playwright's Chromium.
    ...(process.env.CI ? {} : { channel: "chrome" }),
    headless: true,
  },
  webServer: Object.entries(examples).map(([name, port]) => ({
    command: `cargo run --release --quiet -p example-${name}`,
    cwd: "..",
    env: { ADDR: `127.0.0.1:${port}` },
    url: `http://127.0.0.1:${port}/`,
    timeout: 600_000,
    reuseExistingServer: false,
  })),
});
