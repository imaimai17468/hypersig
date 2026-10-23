import { defineConfig } from "@playwright/test";

/** Each example runs as its own server on its own port, built once in release mode. */
const examples = { counter: 3100, todo: 3101, todomvc: 3102, showcase: 3103, style: 3104, ui: 3105, signup: 3106, auth: 3107, storefront: 3108, prefix: 3109 } as const;

/** Each example here keeps its state in a database file, which its server starts without. */
const databases = { database: 3112 } as const;

/** The file the `database` example opens, relative to the repository's root. */
export const databasePath = "target/e2e/database.db";

/** The port of the second `database` server, which its spec starts on the same file. */
export const secondDatabasePort = 3113;

/** The `postgres` example and the second server its spec starts on the same database. */
const postgres = { postgres: 3121 } as const;
export const secondPostgresPort = 3122;

/** PostgreSQL, which Docker runs for the `postgres` example under this name and port. */
export const postgresContainer = "hypersig-e2e-postgres";
export const postgresUrl = "postgres://postgres:hypersig@127.0.0.1:3120/postgres";

/** The `redis` example and the second server its spec starts on the same Redis. */
const redis = { redis: 3126 } as const;
export const secondRedisPort = 3127;

/** Redis, which Docker runs for the `redis` example under this name and port. */
export const redisContainer = "hypersig-e2e-redis";
export const redisUrl = "redis://127.0.0.1:3125/";

/** The `api` example, and the stand-in notes service it calls on `notesBackendPort`. */
const apis = { api: 3124 } as const;
const notesBackendPort = 3123;

/** Each worker runs under `wrangler dev` on its own port. */
const workers = { cloudflare: 3110 } as const;

/** Each Vercel Function runs as its own binary, which listens where `VERCEL_DEV_PORT` says. */
const functions = { vercel: 3111 } as const;

const ports = { ...examples, ...databases, ...postgres, ...redis, ...apis, ...workers, ...functions };

export const url = (example: keyof typeof ports): string => `http://127.0.0.1:${ports[example]}/`;

export default defineConfig({
  testDir: "tests",
  globalTeardown: "./teardown.ts",
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
    // A fresh PostgreSQL in Docker, which the example connects to once its
    // server accepts TCP. The image's first start runs a server on the Unix
    // socket alone while it sets up the database, so a check over TCP waits
    // for the server that stays. `globalTeardown` removes the container.
    {
      command: [
        `docker rm -f ${postgresContainer} >/dev/null 2>&1;`,
        `docker run -d --rm --name ${postgresContainer} -e POSTGRES_PASSWORD=hypersig -p 127.0.0.1:3120:5432 postgres:18 >/dev/null &&`,
        `until docker exec ${postgresContainer} pg_isready -h 127.0.0.1 -U postgres >/dev/null 2>&1; do sleep 1; done &&`,
        "cargo run --release --quiet -p example-postgres",
      ].join(" "),
      cwd: "..",
      env: { ADDR: `127.0.0.1:${postgres.postgres}`, DATABASE_URL: postgresUrl },
      url: `http://127.0.0.1:${postgres.postgres}/`,
      timeout: 600_000,
      reuseExistingServer: false,
    },
    // A fresh Redis in Docker, which `globalTeardown` removes too.
    {
      command: [
        `docker rm -f ${redisContainer} >/dev/null 2>&1;`,
        `docker run -d --rm --name ${redisContainer} -p 127.0.0.1:3125:6379 redis:8 >/dev/null &&`,
        `until docker exec ${redisContainer} redis-cli ping >/dev/null 2>&1; do sleep 1; done &&`,
        "cargo run --release --quiet -p example-redis",
      ].join(" "),
      cwd: "..",
      env: { ADDR: `127.0.0.1:${redis.redis}`, REDIS_URL: redisUrl },
      url: `http://127.0.0.1:${redis.redis}/`,
      timeout: 600_000,
      reuseExistingServer: false,
    },
    {
      command: "cargo run --release --quiet -p example-api --bin notes-backend",
      cwd: "..",
      env: { ADDR: `127.0.0.1:${notesBackendPort}`, API_TOKEN: "e2e-token" },
      port: notesBackendPort,
      timeout: 600_000,
      reuseExistingServer: false,
    },
    {
      command: "cargo run --release --quiet -p example-api",
      cwd: "..",
      env: { ADDR: `127.0.0.1:${apis.api}`, API_URL: `http://127.0.0.1:${notesBackendPort}`, API_TOKEN: "e2e-token" },
      url: `http://127.0.0.1:${apis.api}/`,
      timeout: 600_000,
      reuseExistingServer: false,
    },
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
