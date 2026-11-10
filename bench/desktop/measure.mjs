// Starts each desktop build of the todo list repeatedly and measures how long
// after the process starts its window shows the first page, as README.md
// describes, then writes results/<date>-<system>-<arch>.{json,md}.
//
// Usage: node measure.mjs [app ...]

import { spawn, execFileSync } from "node:child_process";
import { mkdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import os from "node:os";
import { setTimeout as sleep } from "node:timers/promises";
import { fileURLToPath } from "node:url";
import electron from "electron";

// macOS checks a newly built executable on its first starts, which take
// seconds; five starts are past that.
const WARMUP = 5;
const RUNS = 20;
const TIMEOUT_MS = 30_000;
const here = (path) => fileURLToPath(new URL(path, import.meta.url));
const root = here("../../");
const exe = process.platform === "win32" ? ".exe" : "";

/** Each build: the command that starts it, and the files its size counts. */
const APPS = {
  hypersig: {
    command: here(`../../target/release/example-todo${exe}`),
    args: [],
    env: { HYPERSIG_DESKTOP_SMOKE: "1" },
    size: [here(`../../target/release/example-todo${exe}`)],
  },
  tauri: {
    command: here(`tauri/target/release/bench-desktop-tauri${exe}`),
    args: [],
    env: {},
    size: [here(`tauri/target/release/bench-desktop-tauri${exe}`)],
  },
  electron: {
    command: electron,
    args: [here("electron/main.mjs")],
    env: {},
    size: [here("node_modules/electron/dist"), here("electron"), here("web")],
  },
};

const percentile = (values, p) => {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.min(sorted.length - 1, Math.ceil((p / 100) * sorted.length) - 1)];
};

/** Starts `app` once and resolves to the milliseconds from spawn to its "first page shown" line. */
function once(app) {
  return new Promise((resolve, reject) => {
    const started = performance.now();
    const child = spawn(app.command, app.args, { cwd: root, env: { ...process.env, ...app.env } });
    let shown = null;
    let output = "";
    const timer = setTimeout(() => child.kill(), TIMEOUT_MS);
    child.stdout.on("data", (chunk) => {
      output += chunk;
      if (shown === null && /first page shown/.test(output)) shown = performance.now() - started;
    });
    child.stderr.on("data", (chunk) => (output += chunk));
    child.on("exit", (code) => {
      clearTimeout(timer);
      if (shown === null || code !== 0) reject(new Error(`exit ${code} without a shown page:\n${output}`));
      else resolve(shown);
    });
  });
}

/** The bytes of the files and directories in `paths`. */
function bytes(paths) {
  let total = 0;
  for (const path of paths) {
    const stat = statSync(path);
    if (stat.isDirectory()) {
      const kilobytes = execFileSync("du", ["-sk", path], { encoding: "utf8" }).split(/\s/)[0];
      total += Number(kilobytes) * 1024;
    } else total += stat.size;
  }
  return total;
}

async function measure(name) {
  const app = APPS[name];
  const first = await once(app);
  for (let k = 0; k < WARMUP; k++) await once(app);
  const runs = [];
  for (let k = 0; k < RUNS; k++) {
    runs.push(await once(app));
    // The previous window's processes finish closing before the next start.
    await sleep(300);
  }
  return {
    first: Math.round(first),
    median: Math.round(percentile(runs, 50)),
    p95: Math.round(percentile(runs, 95)),
    runs: runs.map(Math.round),
    bytes: bytes(app.size),
  };
}

const names = process.argv.slice(2).length > 0 ? process.argv.slice(2) : Object.keys(APPS);
const results = {};
for (const name of names) {
  process.stdout.write(`${name} … `);
  results[name] = await measure(name);
  console.log(`${results[name].median} ms median`);
}

const machine = {
  system: `${os.type()} ${os.release()}`,
  arch: os.arch(),
  cpu: os.cpus()[0]?.model ?? "unknown",
  memory: `${Math.round(os.totalmem() / 2 ** 30)} GiB`,
  node: process.version,
  electron: JSON.parse(readFileSync(here("node_modules/electron/package.json"), "utf8")).version,
};
const date = new Date().toISOString().slice(0, 10);
const stem = `results/${date}-${process.platform}-${os.arch()}`;
mkdirSync(here("results"), { recursive: true });
writeFileSync(here(`${stem}.json`), `${JSON.stringify({ date, machine, warmup: WARMUP, runs: RUNS, results }, null, 2)}\n`);
const rows = Object.entries(results).map(
  ([name, r]) => `| ${name} | ${r.first} | ${r.median} | ${r.p95} | ${(r.bytes / 2 ** 20).toFixed(1)} |`,
);
writeFileSync(
  here(`${stem}.md`),
  [
    `${date}, ${machine.system} ${machine.arch}, ${machine.cpu}, ${machine.memory}. ${RUNS} runs after ${WARMUP} warm-up runs.`,
    "",
    "| Build | First run (ms) | Median (ms) | p95 (ms) | Size (MiB) |",
    "|---|---|---|---|---|",
    ...rows,
    "",
  ].join("\n"),
);
console.log(`Wrote ${stem}.json and ${stem}.md`);
