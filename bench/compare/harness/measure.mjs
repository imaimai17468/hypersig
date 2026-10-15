// Measures every app in `apps.mjs` as SPEC.md describes and writes results/latest.{json,md}.
//
// Usage: node harness/measure.mjs [app ...]

import { execSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import os from "node:os";
import { setTimeout as sleep } from "node:timers/promises";
import { gzipSync } from "node:zlib";
import { chromium } from "@playwright/test";
import { APPS, reset, start, url } from "./apps.mjs";
import { instrument } from "./page.mjs";

const WARMUP = 3;
const RUNS = 15;
const LOAD_RUNS = 10;
const LATENCY_MS = 50;

const median = (values) => {
  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
};

function trackNetwork(page) {
  const inflight = new Set();
  const finished = [];
  page.on("request", (request) => inflight.add(request));
  page.on("requestfinished", async (request) => {
    inflight.delete(request);
    try {
      const sizes = await request.sizes();
      finished.push(sizes.requestHeadersSize + sizes.requestBodySize + sizes.responseHeadersSize + sizes.responseBodySize);
    } catch {
      // the request failed before any size was known
    }
  });
  // Chrome reports a fetch whose response nobody reads as aborted even after
  // the response arrived, so count what was sent and received from the headers.
  page.on("requestfailed", async (request) => {
    inflight.delete(request);
    const headerBytes = (headers) => headers.reduce((sum, { name, value }) => sum + name.length + value.length + 4, 0);
    let bytes = headerBytes(await request.headersArray()) + (request.postDataBuffer()?.length ?? 0);
    const response = await request.response().catch(() => null);
    if (response !== null) bytes += headerBytes(await response.headersArray());
    finished.push(bytes);
  });
  return {
    async idle(quiet = 150) {
      let since = Date.now();
      while (Date.now() - since < quiet) {
        if (inflight.size > 0) since = Date.now();
        await sleep(10);
      }
    },
    take() {
      return finished.splice(0).reduce((sum, bytes) => sum + bytes, 0);
    },
  };
}

async function measureOne(page, net, check, act) {
  await page.evaluate((c) => {
    window.__result = window.__bench.arm(c);
  }, check);
  await act();
  const ms = await Promise.race([
    page.evaluate(() => window.__result),
    sleep(10_000).then(() => {
      throw new Error(`timed out waiting for ${JSON.stringify(check)}`);
    }),
  ]);
  await net.idle();
  return ms;
}

async function updates(browser, name) {
  const context = await browser.newContext();
  await context.addInitScript(instrument);
  const page = await context.newPage();
  const net = trackNetwork(page);
  const cdp = await context.newCDPSession(page);
  await cdp.send("Network.enable");
  await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: LATENCY_MS, downloadThroughput: -1, uploadThroughput: -1 });
  await reset(name);
  await page.goto(url(name), { waitUntil: "load" });
  await net.idle(1500);
  const count = () => page.evaluate(() => Number(window.__bench.count()));

  const toggle = [];
  const bytes = [];
  for (let k = 0; k < WARMUP + RUNS; k++) {
    let id = 40 + k * 105;
    while (id % 3 === 0) id++;
    const title = `todo ${id}`;
    const left = await count();
    const box = page.locator("ul.todo-list > li", { has: page.getByText(title, { exact: true }) }).locator("input.toggle");
    await box.scrollIntoViewIfNeeded();
    net.take();
    const ms = await measureOne(page, net, { kind: "toggle", title, count: String(left - 1) }, () => box.click());
    if (k >= WARMUP) {
      toggle.push(ms);
      bytes.push(net.take());
    }
  }

  const add = [];
  const input = page.locator("input.new-todo");
  for (let k = 0; k < WARMUP + RUNS; k++) {
    const title = `bench add ${k}`;
    const left = await count();
    await input.fill(title);
    await net.idle();
    const ms = await measureOne(page, net, { kind: "add", title, count: String(left + 1) }, () => input.press("Enter"));
    if (k >= WARMUP) add.push(ms);
  }

  await context.close();
  return { toggle: median(toggle), add: median(add), toggleBytes: median(bytes) };
}

async function load(browser, name) {
  const fcp = [];
  let scripts = [];
  for (let k = 0; k < LOAD_RUNS; k++) {
    await reset(name);
    const context = await browser.newContext();
    const page = await context.newPage();
    const cdp = await context.newCDPSession(page);
    await cdp.send("Network.enable");
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: true });
    await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: LATENCY_MS, downloadThroughput: -1, uploadThroughput: -1 });
    await page.goto(url(name), { waitUntil: "load" });
    await sleep(1500);
    const perf = await page.evaluate(() => ({
      fcp: performance.getEntriesByName("first-contentful-paint")[0]?.startTime ?? null,
      scripts: performance
        .getEntriesByType("resource")
        .filter((entry) => /\.(m?js|wasm)$/.test(new URL(entry.name).pathname) || entry.initiatorType === "script")
        .map((entry) => entry.name),
    }));
    fcp.push(perf.fcp);
    scripts = perf.scripts;
    await context.close();
  }
  let code = 0;
  for (const src of new Set(scripts)) {
    code += gzipSync(Buffer.from(await (await fetch(src)).arrayBuffer())).length;
  }
  return { code, fcp: median(fcp) };
}

const names = process.argv.slice(2).length > 0 ? process.argv.slice(2) : Object.keys(APPS);
const browser = await chromium.launch({ channel: "chrome", headless: true });
const results = {
  date: new Date().toISOString(),
  commit: execSync("git rev-parse --short HEAD").toString().trim(),
  chrome: browser.version(),
  machine: `${os.cpus()[0].model}, ${os.platform()} ${os.release()}`,
  latencyMs: LATENCY_MS,
  apps: {},
};
for (const name of names) {
  const stop = await start(name);
  try {
    results.apps[name] = { ...(await load(browser, name)), ...(await updates(browser, name)) };
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
const ms = (value) => `${value < 10 ? value.toFixed(1) : Math.round(value)} ms`;
const md = `# TodoMVC-2000 results

${results.date}, commit ${results.commit}, Chrome ${results.chrome}, ${results.machine}. Medians, with ${LATENCY_MS} ms added to every request. \`SPEC.md\` defines each column.

| App | Code sent to the browser (gzip) | First Contentful Paint | Toggle one item | Add one item | Bytes per toggle |
|---|---|---|---|---|---|
${names
  .map((name) => {
    const a = results.apps[name];
    return `| ${name} | ${kb(a.code)} | ${ms(a.fcp)} | ${ms(a.toggle)} | ${ms(a.add)} | ${kb(a.toggleBytes)} |`;
  })
  .join("\n")}
`;
writeFileSync(new URL("latest.md", dir), md);
console.log(md);
