// Measures every app in `apps.mjs` as SPEC.md describes and writes results/latest.{json,md}.
//
// Usage: node harness/measure.mjs [app ...]

import { execSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import os from "node:os";
import { setTimeout as sleep } from "node:timers/promises";
import { gzipSync } from "node:zlib";
import { chromium } from "@playwright/test";
import lighthouse from "lighthouse";
import { APPS, reset, start, url } from "./apps.mjs";
import { instrument } from "./page.mjs";

const LATENCY_MS = 50;
const CPU_SLOWDOWN = 4;
const SESSIONS = 3;
const WARMUP = 3;
const RUNS = 15;
const DEBUG_PORT = 9333;

const median = (values) => {
  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
};

/** Lighthouse's mobile run, with its CPU slowdown and request latency but no bandwidth cap. */
const lighthouseConfig = {
  extends: "lighthouse:default",
  settings: {
    onlyAudits: ["first-contentful-paint", "total-blocking-time"],
    throttlingMethod: "devtools",
    throttling: {
      requestLatencyMs: LATENCY_MS,
      downloadThroughputKbps: 0,
      uploadThroughputKbps: 0,
      cpuSlowdownMultiplier: CPU_SLOWDOWN,
    },
  },
};

async function load(name) {
  const runs = [];
  for (let k = 0; k < SESSIONS; k++) {
    await reset(name);
    const result = await lighthouse(url(name), { port: DEBUG_PORT, logLevel: "error", output: "json" }, lighthouseConfig);
    const audits = result.lhr.audits;
    runs.push({
      fcp: audits["first-contentful-paint"].numericValue,
      tbt: audits["total-blocking-time"].numericValue,
    });
  }
  return { fcp: median(runs.map((r) => r.fcp)), tbt: median(runs.map((r) => r.tbt)) };
}

async function code(browser, name) {
  const page = await browser.newPage();
  await page.goto(url(name), { waitUntil: "load" });
  await sleep(1500);
  const scripts = await page.evaluate(() =>
    performance
      .getEntriesByType("resource")
      .filter((entry) => /\.(m?js|wasm)$/.test(new URL(entry.name).pathname) || entry.initiatorType === "script")
      .map((entry) => entry.name),
  );
  await page.close();
  const bytes = { js: 0, wasm: 0 };
  for (const src of new Set(scripts)) {
    const size = gzipSync(Buffer.from(await (await fetch(src)).arrayBuffer())).length;
    bytes[new URL(src).pathname.endsWith(".wasm") ? "wasm" : "js"] += size;
  }
  return bytes;
}

async function waitIdle(page, quiet = 150) {
  let inflight = 0;
  const up = () => inflight++;
  const down = () => inflight--;
  page.on("request", up);
  page.on("requestfinished", down);
  page.on("requestfailed", down);
  let since = Date.now();
  while (Date.now() - since < quiet) {
    if (inflight > 0) since = Date.now();
    await sleep(10);
  }
  page.off("request", up);
  page.off("requestfinished", down);
  page.off("requestfailed", down);
}

/**
 * One page session of 15 toggles and 15 adds after warm-ups. Returns its INP,
 * the worst interaction as Web Vitals defines it for fewer than 50
 * interactions, and the median time until a toggle shows in the DOM.
 */
async function session(browser, name) {
  const context = await browser.newContext();
  await context.addInitScript(instrument);
  await context.addInitScript(() => {
    window.__interactions = new Map();
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        if (entry.interactionId === 0) continue;
        const known = window.__interactions.get(entry.interactionId) ?? 0;
        window.__interactions.set(entry.interactionId, Math.max(known, entry.duration));
      }
    }).observe({ type: "event", durationThreshold: 16, buffered: true });
  });
  const page = await context.newPage();
  const cdp = await context.newCDPSession(page);
  await cdp.send("Network.enable");
  await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: LATENCY_MS, downloadThroughput: -1, uploadThroughput: -1 });
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: CPU_SLOWDOWN });
  await reset(name);
  await page.goto(url(name), { waitUntil: "load" });
  await waitIdle(page, 1500);
  const count = () => page.evaluate(() => Number(window.__bench.count()));
  const shown = [];

  for (let k = 0; k < WARMUP + RUNS; k++) {
    if (k === WARMUP) await page.evaluate(() => window.__interactions.clear());
    let id = 40 + k * 105;
    while (id % 3 === 0) id++;
    const title = `todo ${id}`;
    const left = await count();
    const box = page.locator("ul.todo-list > li", { has: page.getByText(title, { exact: true }) }).locator("input.toggle");
    await box.scrollIntoViewIfNeeded();
    await page.evaluate((c) => {
      window.__result = window.__bench.arm(c);
    }, { kind: "toggle", title, count: String(left - 1) });
    await box.click();
    const ms = await page.evaluate(() => window.__result);
    if (k >= WARMUP) shown.push(ms);
    await waitIdle(page);
  }

  const input = page.locator("input.new-todo");
  for (let k = 0; k < RUNS; k++) {
    const title = `bench add ${k}`;
    const left = await count();
    await input.fill(title);
    await waitIdle(page);
    await page.evaluate((c) => {
      window.__result = window.__bench.arm(c);
    }, { kind: "add", title, count: String(left + 1) });
    await input.press("Enter");
    await page.evaluate(() => window.__result);
    await waitIdle(page);
  }

  const durations = await page.evaluate(() => [...window.__interactions.values()]);
  await context.close();
  return { inp: durations.length === 0 ? 0 : Math.max(...durations), shown: median(shown) };
}

const names = process.argv.slice(2).length > 0 ? process.argv.slice(2) : Object.keys(APPS);
const browser = await chromium.launch({ channel: "chrome", headless: true, args: [`--remote-debugging-port=${DEBUG_PORT}`] });
const results = {
  date: new Date().toISOString(),
  commit: execSync("git rev-parse --short HEAD").toString().trim(),
  chrome: browser.version(),
  machine: `${os.cpus()[0].model}, ${os.platform()} ${os.release()}`,
  conditions: { latencyMs: LATENCY_MS, cpuSlowdown: CPU_SLOWDOWN },
  apps: {},
};
for (const name of names) {
  const stop = await start(name);
  try {
    const sessions = [];
    for (let k = 0; k < SESSIONS; k++) sessions.push(await session(browser, name));
    results.apps[name] = {
      ...(await code(browser, name)),
      ...(await load(name)),
      inp: median(sessions.map((s) => s.inp)),
      toggleShown: median(sessions.map((s) => s.shown)),
    };
    console.log(name, JSON.stringify(results.apps[name]));
  } finally {
    stop();
  }
}
await browser.close();

const dir = new URL("../results/", import.meta.url);
mkdirSync(dir, { recursive: true });
writeFileSync(new URL("latest.json", dir), `${JSON.stringify(results, null, 2)}\n`);

const kb = (bytes) => `${(bytes / 1024).toFixed(1)} KB`;
const ms = (value) => `${Math.round(value)} ms`;
const inp = (value) => (value < 16 ? "< 16 ms" : ms(value));
const md = `# TodoMVC-2000 results

${results.date}, commit ${results.commit}, Chrome ${results.chrome}, ${results.machine}. CPU slowed ${CPU_SLOWDOWN}x and ${LATENCY_MS} ms added to every request. Medians of ${SESSIONS} runs.

| | ${names.join(" | ")} |
|---|${names.map(() => "---").join("|")}|
| JS (gzip) | ${names.map((n) => kb(results.apps[n].js)).join(" | ")} |
| Wasm (gzip) | ${names.map((n) => (results.apps[n].wasm === 0 ? "-" : kb(results.apps[n].wasm))).join(" | ")} |
| FCP | ${names.map((n) => ms(results.apps[n].fcp)).join(" | ")} |
| TBT | ${names.map((n) => ms(results.apps[n].tbt)).join(" | ")} |
| INP | ${names.map((n) => inp(results.apps[n].inp)).join(" | ")} |
`;
writeFileSync(new URL("latest.md", dir), md);
console.log(md);
