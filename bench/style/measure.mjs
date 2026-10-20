// Builds each styling of TodoMVC-2000, renders the pages, and writes the
// bytes each page sends to results/latest.json and results/latest.md.
//
// Run with `pnpm measure` from bench/style.

import { execFileSync } from "node:child_process";
import { createServer } from "node:http";
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { performance } from "node:perf_hooks";
import { gzipSync } from "node:zlib";
import { chromium } from "@playwright/test";
import { transformSync } from "@babel/core";
import stylexBabel from "@stylexjs/babel-plugin";
import { vanillaExtractPlugin } from "@vanilla-extract/esbuild-plugin";
import * as esbuild from "esbuild";

const require = createRequire(import.meta.url);
const bin = (name) => join("node_modules", ".bin", name);
const run = (cmd, args, cwd = ".") => execFileSync(cmd, args, { cwd, stdio: "inherit" });
const minify = (css) => esbuild.transformSync(css, { loader: "css", minify: true }).code;
const dir = (name) => {
  mkdirSync(join("out", name), { recursive: true });
  return join("out", name);
};
const writeClasses = (name, classes) =>
  writeFileSync(join(dir(name), "classes.json"), JSON.stringify(classes, null, 2));

/// Each tool writes `out/<name>/classes.json` and `out/<name>/style.css`.
const tools = {
  tailwind() {
    run(bin("tailwindcss"), ["-i", "tailwind/input.css", "-o", join(dir("tailwind"), "style.css"), "--minify"]);
    copyFileSync("tailwind/classes.json", join(dir("tailwind"), "classes.json"));
  },
  async panda() {
    run(join("..", bin("panda")), ["codegen", "--silent"], "panda");
    run(join("..", bin("panda")), ["cssgen", "--minify", "--silent", "--outfile", join("..", dir("panda"), "style.css")], "panda");
    writeClasses("panda", (await import("./panda/styles.ts")).classes);
  },
  async stylex() {
    const source = readFileSync("stylex/styles.ts", "utf8");
    const result = transformSync(source, {
      filename: join(process.cwd(), "stylex/styles.ts"),
      presets: [require.resolve("@babel/preset-typescript")],
      plugins: [[stylexBabel, { dev: false, runtimeInjection: false, unstable_moduleResolution: { type: "commonJS" } }]],
    });
    const compiled = join(dir("stylex"), "styles.mjs");
    writeFileSync(compiled, result.code);
    const css = readFileSync("stylex/base.css", "utf8") + stylexBabel.processStylexRules(result.metadata.stylex);
    writeFileSync(join(dir("stylex"), "style.css"), minify(css));
    writeClasses("stylex", (await import(`./${compiled}`)).classes);
  },
  async "vanilla-extract"() {
    const out = dir("vanilla-extract");
    await esbuild.build({
      entryPoints: ["vanilla-extract/classes.ts"],
      bundle: true,
      format: "esm",
      platform: "node",
      minify: true,
      outfile: join(out, "classes.mjs"),
      plugins: [vanillaExtractPlugin({ identifiers: "short" })],
    });
    copyFileSync(join(out, "classes.css"), join(out, "style.css"));
    writeClasses("vanilla-extract", (await import(`./${join(out, "classes.mjs")}`)).classes);
  },
  plain() {
    writeFileSync(join(dir("plain"), "style.css"), minify(readFileSync("../compare/shared/todomvc.css", "utf8")));
    copyFileSync("plain/classes.json", join(dir("plain"), "classes.json"));
  },
};

const buildMs = {};
for (const [name, build] of Object.entries(tools)) {
  const start = performance.now();
  await build();
  buildMs[name] = performance.now() - start;
}
run("cargo", ["build", "--release", "--quiet"]);
run("cargo", ["run", "--release", "--quiet", "--", ...Object.keys(tools)]);

const CPU_SLOWDOWN = 4;
const RUNS = 5;
const median = (values) => values.toSorted((a, b) => a - b)[Math.floor(values.length / 2)];
const types = { ".html": "text/html", ".css": "text/css" };
const server = createServer((req, res) => {
  const path = join("out", decodeURIComponent(new URL(req.url, "http://x").pathname));
  try {
    const body = readFileSync(path.endsWith("/") ? join(path, "index.html") : path);
    res.writeHead(200, { "content-type": types[path.slice(path.lastIndexOf("."))] ?? "text/html" });
    res.end(body);
  } catch {
    res.writeHead(404).end();
  }
}).listen(4300);
const browser = await chromium.launch({ channel: "chrome", headless: true });

/// Loads the page under the CPU slowdown, then marks every item done by
/// adding the classes a done title has and reads the layout.
async function browse(name) {
  const page = await browser.newPage();
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("Emulation.setCPUThrottlingRate", { rate: CPU_SLOWDOWN });
  await cdp.send("Performance.enable");
  await page.goto(`http://127.0.0.1:4300/${name}/`, { waitUntil: "load" });
  const { metrics } = await cdp.send("Performance.getMetrics");
  const metric = (key) => metrics.find((m) => m.name === key).value * 1000;
  const fcp = await page.evaluate(
    () =>
      new Promise((resolve) =>
        new PerformanceObserver((list) => {
          const entry = list.getEntriesByName("first-contentful-paint")[0];
          if (entry) resolve(entry.startTime);
        }).observe({ type: "paint", buffered: true }),
      ),
  );
  const toggle = await page.evaluate(() => {
    // Item 3 is done and item 1 is not, so the classes item 3 has on top are
    // the ones each styling adds to a done item.
    const items = [...document.querySelectorAll("ul > li[data-hs-ssr]")];
    const parts = (li) => [li, li.querySelector("label")];
    const added = parts(items[2]).map((el, i) => {
      const before = parts(items[0])[i].classList;
      return [...el.classList].filter((c) => !before.contains(c));
    });
    const start = performance.now();
    for (const item of items) parts(item).forEach((el, i) => added[i].length && el.classList.add(...added[i]));
    document.body.offsetHeight;
    return performance.now() - start;
  });
  await page.close();
  return { fcp, styleLayout: metric("RecalcStyleDuration") + metric("LayoutDuration"), toggle };
}

const gzip = (text) => gzipSync(Buffer.from(text)).length;
const styleBlocks = /<style[^>]*>([\s\S]*?)<\/style>/g;
const names = ["hypersig", ...Object.keys(tools)];
const apps = {};
for (const name of names) {
  const html = readFileSync(join("out", name, "index.html"), "utf8");
  const inline = [...html.matchAll(styleBlocks)].map((m) => m[1]).join("");
  let file = "";
  try {
    file = readFileSync(join("out", name, "style.css"), "utf8");
  } catch {}
  const markup = html.replace(styleBlocks, (m, css) => m.replace(css, ""));
  const runs = [];
  for (let i = 0; i < RUNS; i++) runs.push(await browse(name));
  apps[name] = {
    build: buildMs[name] ?? null,
    render: JSON.parse(readFileSync(join("out", name, "render.json"), "utf8")).ms,
    fcp: median(runs.map((r) => r.fcp)),
    styleLayout: median(runs.map((r) => r.styleLayout)),
    toggle: median(runs.map((r) => r.toggle)),
    css: gzip(inline) + (file ? gzip(file) : 0),
    cssRaw: Buffer.byteLength(inline) + Buffer.byteLength(file),
    html: gzip(markup),
    sent: gzip(html) + (file ? gzip(file) : 0),
  };
}

await browser.close();
server.close();

const versions = Object.fromEntries(
  ["@tailwindcss/cli", "@pandacss/dev", "@stylexjs/stylex", "@vanilla-extract/css", "esbuild"].map((p) => [
    p,
    JSON.parse(readFileSync(join("node_modules", p, "package.json"), "utf8")).version,
  ]),
);
const commit = execFileSync("git", ["rev-parse", "--short", "HEAD"]).toString().trim();
const chrome = browser.version();
const results = { date: new Date().toISOString(), commit, chrome, cpuSlowdown: CPU_SLOWDOWN, runs: RUNS, versions, apps };
mkdirSync("results", { recursive: true });
writeFileSync("results/latest.json", JSON.stringify(results, null, 2) + "\n");

const kb = (n) => `${(n / 1024).toFixed(1)} KB`;
const ms = (n) => (n === null ? "None" : `${n.toFixed(n < 10 ? 1 : 0)} ms`);
const row = (label, key, unit = kb) => `| ${label} | ${names.map((n) => unit(apps[n][key])).join(" | ")} |`;
const md = `# Styling TodoMVC-2000

Measured at ${results.date}, commit ${commit}, in Chrome ${chrome} with the CPU slowed ${CPU_SLOWDOWN}x. Browser values are medians of ${RUNS} runs, and the render time is the median of 50. ${Object.entries(versions)
  .map(([p, v]) => `${p} ${v}`)
  .join(", ")}.

| | ${names.join(" | ")} |
|---|${names.map(() => "---|").join("")}
${row("CSS (gzip)", "css")}
${row("HTML without CSS (gzip)", "html")}
${row("Sent in all (gzip)", "sent")}
${row("Build of the CSS", "build", ms)}
${row("Server render", "render", ms)}
${row("First Contentful Paint", "fcp", ms)}
${row("Style and layout on load", "styleLayout", ms)}
${row("Marking 2000 items done", "toggle", ms)}
`;
writeFileSync("results/latest.md", md);
console.log(md);
