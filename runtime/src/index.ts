import { keep, type Persisted, restore } from "./persist.ts";
import { Runtime } from "./runtime.ts";
import type { Values } from "./store/value.ts";

function start(): void {
  const body = document.body;
  const persisted = JSON.parse(body.dataset.hsPersist ?? "[]") as Persisted;
  const initial = restore(JSON.parse(body.dataset.hsSignals ?? "{}") as Values, persisted);
  /** Starts the runtime over the page's signals, set to `values`. */
  const run = (values: Values): Runtime => {
    const runtime = new Runtime(document.body, values);
    keep(runtime.store, persisted);
    window.addEventListener("popstate", () => runtime.popped());
    return runtime;
  };
  // Only a page served by a debug build names the stream. That server serves
  // the dev script at `dev.js` beside the stream, resolved against the page,
  // because a relative import resolves against this script, which a page may
  // load from another origin.
  const stream = body.dataset.hsDev;
  if (stream === undefined) {
    run(initial);
    return;
  }
  import(new URL("dev.js", new URL(stream, document.baseURI)).href).then(
    (dev: typeof import("./dev.ts")) => dev.start(run, initial, stream),
    () => run(initial),
  );
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", start, { once: true });
} else {
  start();
}
