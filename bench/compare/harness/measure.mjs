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
const LATENCIES = [0, 50];

const median = (values) => {
  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
};

/** Ids of active todos spread over the list, `offset` apart so two scenarios never share one. */
const activeIds = (offset) =>
  Array.from({ length: WARMUP + RUNS }, (_, k) => {
    let id = 40 + offset + k * 105;
    while (id % 3 === 0) id++;
    return id;
  });

function trackNetwork(page) {
  const inflight = new Set();
  const finished = [];
  page.on("request", (request) => inflight.add(request));
  const done = async (request) => {
    inflight.delete(request);
    try {
      const sizes = await request.sizes();
      finished.push(sizes.requestHeadersSize + sizes.requestBodySize + sizes.responseHeadersSize + sizes.responseBodySize);
    } catch {
      // the request failed before any size was known
    }
  };
  page.on("requestfinished", done);
  page.on("requestfailed", (request) => inflight.delete(request));
  return {
    async idle(quiet = 100) {
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

async function interactions(browser, name, latency) {
  const context = await browser.newContext();
  await context.addInitScript(instrument);
  const page = await context.newPage();
  const net = trackNetwork(page);
  const cdp = await context.newCDPSession(page);
  await cdp.send("Network.enable");
  await cdp.send("Network.emulateNetworkConditions", { offline: false, latency, downloadThroughput: -1, uploadThroughput: -1 });
  await reset(name);
  await page.goto(url(name), { waitUntil: "load" });
  await net.idle(1000);

  const count = () => page.evaluate(() => Number(window.__bench.count()));
  const runs = async (scenario) => {
    const values = [];
    for (let k = 0; k < WARMUP + RUNS; k++) {
      const times = await scenario(k);
      if (k >= WARMUP) values.push(times);
    }
    return { dom: median(values.map((t) => t.dom)), frame: median(values.map((t) => t.frame)) };
  };
  const result = {};

  const input = page.locator("input.new-todo");
  await input.click();
  result.type = await runs(() => measureOne(page, net, { kind: "type" }, () => page.keyboard.press("x")));
  await input.fill("");
  await net.idle();

  const toggles = activeIds(0);
  let bytes = [];
  result.toggle = await runs(async (k) => {
    const title = `todo ${toggles[k]}`;
    const left = await count();
    const box = page.locator("ul.todo-list > li", { has: page.getByText(title, { exact: true }) }).locator("input.toggle");
    await box.scrollIntoViewIfNeeded();
    net.take();
    const ms = await measureOne(page, net, { kind: "toggle", title, done: true, count: String(left - 1) }, () => box.click());
    bytes.push(net.take());
    return ms;
  });
  result.toggleBytes = median(bytes.slice(WARMUP));

  result.add = await runs(async (k) => {
    const title = `bench add ${k}`;
    const left = await count();
    await input.fill(title);
    await net.idle();
    return measureOne(page, net, { kind: "add", title, count: String(left + 1) }, () => input.press("Enter"));
  });

  const deletes = activeIds(50);
  result.delete = await runs(async (k) => {
    const title = `todo ${deletes[k]}`;
    const left = await count();
    const item = page.locator("ul.todo-list > li", { has: page.getByText(title, { exact: true }) });
    await item.hover();
    return measureOne(page, net, { kind: "delete", title, count: String(left - 1) }, () => item.locator("button.destroy").click());
  });

  const all = await page.evaluate(() => window.__bench.visible());
  const active = await count();
  result.filter = await runs(async (k) => {
    const target = k % 2 === 0 ? "Active" : "All";
    const link = page.locator(".filters a", { hasText: target });
    const check = { kind: "filter", visible: target === "Active" ? active : all };
    return measureOne(page, net, check, () => link.click());
  });

  await context.close();
  return result;
}

async function load(browser, name) {
  const fcp = [];
  const long = [];
  const loaded = [];
  let scripts = [];
  for (let k = 0; k < LOAD_RUNS; k++) {
    await reset(name);
    const context = await browser.newContext();
    await context.addInitScript(() => {
      window.__perf = { fcp: null, long: 0 };
      new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) if (entry.name === "first-contentful-paint") window.__perf.fcp = entry.startTime;
      }).observe({ type: "paint", buffered: true });
      new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) window.__perf.long += entry.duration;
      }).observe({ type: "longtask", buffered: true });
    });
    const page = await context.newPage();
    const cdp = await context.newCDPSession(page);
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: true });
    await page.goto(url(name), { waitUntil: "load" });
    await sleep(2000);
    const perf = await page.evaluate(() => ({
      ...window.__perf,
      load: performance.getEntriesByType("navigation")[0].loadEventEnd,
      scripts: performance
        .getEntriesByType("resource")
        .filter((entry) => /\.(m?js|wasm)(\?|$)/.test(new URL(entry.name).pathname) || entry.initiatorType === "script")
        .map((entry) => entry.name),
    }));
    fcp.push(perf.fcp);
    long.push(perf.long);
    loaded.push(perf.load);
    scripts = perf.scripts;
    await context.close();
  }
  await reset(name);
  const html = Buffer.from(await (await fetch(url(name))).arrayBuffer());
  let scriptBytes = 0;
  for (const src of new Set(scripts)) {
    scriptBytes += gzipSync(Buffer.from(await (await fetch(src)).arrayBuffer())).length;
  }
  return {
    htmlGzip: gzipSync(html).length,
    scriptGzip: scriptBytes,
    fcp: median(fcp),
    load: median(loaded),
    longTasks: median(long),
  };
}

const names = process.argv.slice(2).length > 0 ? process.argv.slice(2) : Object.keys(APPS);
const browser = await chromium.launch({ channel: "chrome", headless: true });
const results = {
  date: new Date().toISOString(),
  commit: execSync("git rev-parse --short HEAD").toString().trim(),
  chrome: browser.version(),
  machine: `${os.cpus()[0].model}, ${os.platform()} ${os.release()}`,
  apps: {},
};
for (const name of names) {
  const stop = await start(name);
  try {
    const entry = { load: await load(browser, name) };
    for (const latency of LATENCIES) entry[`rtt${latency}`] = await interactions(browser, name, latency);
    results.apps[name] = entry;
    console.log(name, JSON.stringify(entry));
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
const both = (t) => `${ms(t.dom)} / ${ms(t.frame)}`;
const rows = (pick) => names.map((name) => `| ${name} | ${pick(results.apps[name]).join(" | ")} |`).join("\n");
const md = `# TodoMVC-2000 results

${results.date}, commit ${results.commit}, Chrome ${results.chrome}, ${results.machine}. Medians; see \`SPEC.md\` for what each column measures. Each latency cell is the time until the DOM shows the result, then the time until the next frame is painted.

## Load

| App | HTML (gzip) | Scripts (gzip) | First Contentful Paint | \`load\` event | Long tasks |
|---|---|---|---|---|---|
${rows((a) => [kb(a.load.htmlGzip), kb(a.load.scriptGzip), ms(a.load.fcp), ms(a.load.load), ms(a.load.longTasks)])}

## Interaction latency, localhost

| App | Type a character | Toggle | Add | Delete | Switch filter | Bytes per toggle |
|---|---|---|---|---|---|---|
${rows((a) => [both(a.rtt0.type), both(a.rtt0.toggle), both(a.rtt0.add), both(a.rtt0.delete), both(a.rtt0.filter), kb(a.rtt0.toggleBytes)])}

## Interaction latency, 50 ms added per request

| App | Type a character | Toggle | Add | Delete | Switch filter |
|---|---|---|---|---|---|
${rows((a) => [both(a.rtt50.type), both(a.rtt50.toggle), both(a.rtt50.add), both(a.rtt50.delete), both(a.rtt50.filter)])}
`;
writeFileSync(new URL("latest.md", dir), md);
console.log(md);
