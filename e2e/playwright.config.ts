import { defineConfig } from "@playwright/test";

/** Each example runs as its own server on its own port, built once in release mode. */
const examples = { counter: 3100, todo: 3101, todomvc: 3102, showcase: 3103, style: 3104, ui: 3105, signup: 3106, auth: 3107, storefront: 3108, prefix: 3109 } as const;

/** Each example here keeps its state in a database file, which its server starts without. */
const databases = { database: 3112 } as const;

/** The file the `database` example opens, relative to the repository's root. */
export const databasePath = "target/e2e/database.db";

/** The port of the second `database` server, which its spec starts on the same file. */
export const secondDatabasePort = 3113;

/** Each worker runs under `wrangler dev` on its own port. */
const workers = { cloudflare: 3110 } as const;

/** Each Vercel Function runs as its own binary, which listens where `VERCEL_DEV_PORT` says. */
const functions = { vercel: 3111 } as const;

const ports = { ...examples, ...databases, ...workers, ...functions };

export const url = (example: keyof typeof ports): string => `http://127.0.0.1:${ports[example]}/`;

export default defineConfig({
  testDir: "tests",
  // Each spec file drives its own example, so files run side by side, and the
  // tests of one file run in order because they share that server's state.
  fullyParallel: false,
  workers: 4,
  forbidOnly: Boolean(process.env.CI),
  reporter: process.env.CI ? "github" : "list",
  use: {
    // Drives the Google Chrome already installed, here and on GitHub's
    // runner image, so neither installs a browser.
    channel: "chrome",
    headless: true,
  },
  webServer: [
    ...Object.entries(examples).map(([name, port]) => ({
      command: `cargo run --release --quiet -p example-${name}`,
      cwd: "..",
      env: { ADDR: `127.0.0.1:${port}` },
      url: `http://127.0.0.1:${port}/`,
      timeout: 600_000,
      reuseExistingServer: false,
    })),
    // The tests of a file run in order on one server, so a run that started
    // from the previous run's file would find its todos.
    ...Object.entries(databases).map(([name, port]) => ({
      command: `mkdir -p target/e2e && rm -f ${databasePath}* && cargo run --release --quiet -p example-${name}`,
      cwd: "..",
      env: { ADDR: `127.0.0.1:${port}`, DATABASE_PATH: databasePath },
      url: `http://127.0.0.1:${port}/`,
      timeout: 600_000,
      reuseExistingServer: false,
    })),
    // The Cloudflare template runs in workerd, the Workers runtime, which
    // wrangler starts after building the worker with `worker-build`. The
    // project is created first with `cargo hypersig new`, with its hypersig
    // pointed at this repository.
    {
      command: `dir=$(crates/hypersig_cli/stage.sh cloudflare) && cd "$dir" && "$OLDPWD/e2e/node_modules/.bin/wrangler" dev --ip 127.0.0.1 --port ${workers.cloudflare}`,
      cwd: "..",
      env: { WRANGLER_SEND_METRICS: "false" },
      url: `http://127.0.0.1:${workers.cloudflare}/`,
      timeout: 600_000,
      reuseExistingServer: false,
    },
    // Outside Vercel, `vercel_runtime::run` serves plain HTTP through the same
    // `VercelLayer` the deployed function uses, so the template runs without
    // the Vercel CLI or an account.
    {
      command: `dir=$(crates/hypersig_cli/stage.sh vercel) && cd "$dir" && cargo run --release --quiet`,
      cwd: "..",
      env: { VERCEL_DEV_PORT: String(functions.vercel) },
      url: `http://127.0.0.1:${functions.vercel}/`,
      timeout: 600_000,
      reuseExistingServer: false,
    },
  ],
});
