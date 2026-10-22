import { reloadOnRestart } from "./dev.ts";
import { Runtime } from "./runtime.ts";
import type { Values } from "./store/value.ts";

function start(): void {
  const body = document.body;
  const initial = JSON.parse(body.dataset.hsSignals ?? "{}") as Values;
  const runtime = new Runtime(body, initial);
  window.addEventListener("popstate", () => runtime.popped());
  // Only a page served by a debug build names the stream.
  const dev = body.dataset.hsDev;
  if (dev !== undefined) void reloadOnRestart(dev, runtime.navigation);
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", start, { once: true });
} else {
  start();
}
