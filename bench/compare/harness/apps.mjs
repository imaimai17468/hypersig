// The apps under test, how to start each one, and where it listens.

import { execFileSync, spawn } from "node:child_process";
import { setTimeout as sleep } from "node:timers/promises";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("..", import.meta.url));

export const APPS = {
  // The binary embeds crates/hypersig/assets/hypersig.js, so it is rebuilt before each start
  // to pick up a runtime built after the last cargo build. Cargo returns at once when nothing changed.
  hypersig: {
    port: 4100,
    build: ["cargo", "build", "--release", "--quiet", "--manifest-path", "hypersig/Cargo.toml"],
    cmd: ["hypersig/target/release/bench-hypersig"],
  },
  // `topcoat asset bundle` builds the release binary and writes the runtime script it
  // embeds as an asset!() into target/release/assets, where the binary loads it from.
  // The CLI has no --manifest-path, so it runs inside topcoat/.
  topcoat: {
    port: 4104,
    build: ["sh", "-c", "cd topcoat && topcoat asset bundle --release"],
    cmd: ["topcoat/target/release/bench-topcoat"],
  },
  react: { port: 4105, cmd: ["npm", "--prefix", "react", "run", "start"] },
  next: { port: 4103, cmd: ["npm", "--prefix", "next", "run", "start"] },
  "tanstack-start": { port: 4106, cmd: ["npm", "--prefix", "tanstack-start", "run", "start"] },
  nuxt: { port: 4107, cmd: ["npm", "--prefix", "nuxt", "run", "start"] },
  datastar: { port: 4102, cmd: ["datastar/target/release/bench-datastar"] },
  // The server reads its address and the directory that `cargo leptos build`
  // writes the wasm and its JavaScript into from these variables, which
  // `cargo leptos serve` would otherwise set from Cargo.toml.
  leptos: {
    port: 4108,
    cmd: ["leptos/target/release/bench-leptos"],
    env: { LEPTOS_SITE_ADDR: "127.0.0.1:4108", LEPTOS_SITE_ROOT: "leptos/target/site", LEPTOS_ENV: "PROD" },
  },
  // `dx build` writes the server beside a public/ directory holding the wasm
  // and its JavaScript, which the server finds next to its own executable.
  dioxus: {
    port: 4109,
    cmd: ["dioxus/target/dx/bench-dioxus/release/web/server"],
    env: { IP: "127.0.0.1", PORT: "4109" },
  },
};

export const url = (name) => `http://127.0.0.1:${APPS[name].port}/`;

/** Starts the app, waits until it answers, and returns a function that stops it. */
export async function start(name) {
  const app = APPS[name];
  if (app.build) execFileSync(app.build[0], app.build.slice(1), { cwd: root, stdio: "inherit" });
  const child = spawn(app.cmd[0], app.cmd.slice(1), {
    cwd: root,
    env: { ...process.env, ...app.env },
    stdio: ["ignore", "ignore", "inherit"],
    detached: true,
  });
  for (let attempt = 0; attempt < 200; attempt++) {
    try {
      const response = await fetch(url(name));
      if (response.ok) break;
    } catch {
      // not listening yet
    }
    if (attempt === 199) throw new Error(`${name} did not start on port ${app.port}`);
    await sleep(100);
  }
  return () => {
    try {
      process.kill(-child.pid, "SIGTERM");
    } catch {
      // already gone
    }
  };
}

export async function reset(name) {
  const response = await fetch(new URL("bench-reset", url(name)), { method: "POST" });
  if (response.status !== 204) throw new Error(`${name}: /bench-reset answered ${response.status}`);
}
