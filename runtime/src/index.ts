import { Runtime } from "./runtime.ts";
import type { Values } from "./store/value.ts";

function start(): void {
  const body = document.body;
  const initial = JSON.parse(body.dataset.hsSignals ?? "{}") as Values;
  const runtime = new Runtime(body, initial);
  window.addEventListener("popstate", () => runtime.popped());
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", start, { once: true });
} else {
  start();
}
