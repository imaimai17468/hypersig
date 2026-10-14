import { Runtime } from "./runtime.ts";
import type { Values } from "./store.ts";

function start(): void {
  const body = document.body;
  const initial = JSON.parse(body.getAttribute("data-hs-signals") ?? "{}") as Values;
  new Runtime(body, initial);
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", start, { once: true });
} else {
  start();
}
