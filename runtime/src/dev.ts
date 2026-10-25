// The code a page served by a debug build runs, which `pnpm build` writes to
// crates/hypersig/assets/dev.js apart from the runtime, so that a page
// served by a release build loads none of it.
import type { Runtime } from "./runtime.ts";
import type { Navigation } from "./runtime/navigation.ts";
import { readEventStream } from "./sse.ts";
import { equal, type Values } from "./store/value.ts";

/**
 * Starts the runtime with `run` and reloads the page once another server
 * answers the stream at `stream`.
 *
 * The runtime starts from `initial`, the values the server wrote, with the
 * signals the visitor changed before the last reload put back where
 * `restoreSignals` allows. Just before the next reload, the page keeps the
 * signals that differ from what the server last wrote.
 */
export function start(run: (values: Values) => Runtime, initial: Values, stream: string): void {
  const shape = document.body.dataset.hsShape;
  const runtime = run(shape === undefined ? initial : restoreSignals(shape, initial));
  const changed = followServer(runtime, initial);
  reloadOnRestart(stream, runtime.navigation, () => {
    if (shape !== undefined) keepSignals(shape, changed());
  });
}

/**
 * Follows the values the server writes into `runtime`, starting from
 * `initial`, and returns a function that reads the top-level signals whose
 * values differ from the server's last write of them.
 *
 * Each event of an action's answer or of a stream passes through
 * `runtime.actions.patches.apply`, so the values each event leaves behind
 * are the server's. The store copies each container it writes into, so a
 * top-level signal that an event changed holds a new value afterwards.
 */
export function followServer(runtime: Runtime, initial: Values): () => Values {
  const server: Values = { ...initial };
  const patches = runtime.actions.patches;
  const apply = patches.apply.bind(patches);
  patches.apply = (event) => {
    const before = runtime.store.get("") as Values;
    apply(event);
    const after = runtime.store.get("") as Values;
    for (const key of Object.keys(after)) if (after[key] !== before[key]) server[key] = after[key];
  };
  return () => {
    const values = runtime.store.get("") as Values;
    const changed: Values = {};
    for (const key of Object.keys(server)) {
      if (!equal(values[key], server[key])) changed[key] = values[key];
    }
    return changed;
  };
}

/** The `sessionStorage` key the signals wait under while the page reloads. */
const KEPT = "hypersig-dev";

/** What a page leaves for the page its reload loads. */
interface Kept {
  /** The path and query the page showed. */
  route: string;
  /** The `SHAPE` of the page's signal struct. */
  shape: string;
  /** The top-level signals the visitor changed. */
  values: Values;
}

/** The path and query of the URL the page shows. */
function route(): string {
  return location.pathname + location.search;
}

/**
 * Leaves the signals the visitor changed in `sessionStorage` for the page
 * the reload loads, beside the route and the struct's `shape`.
 */
export function keepSignals(shape: string, values: Values): void {
  const kept: Kept = { route: route(), shape, values };
  try {
    sessionStorage.setItem(KEPT, JSON.stringify(kept));
  } catch {
    // Storage that is full or blocked keeps nothing, and the reloaded page
    // starts from the server's values.
  }
}

/**
 * The values a reloaded page starts with: `initial`, the server's, with
 * the signals `keepSignals` left put back over it.
 *
 * They are put back only where the page that left them showed the same
 * route and its struct had the same `shape`, and they are removed as they
 * are read, so a later visit starts from the server's values.
 */
export function restoreSignals(shape: string, initial: Values): Values {
  let kept: Kept;
  try {
    const text = sessionStorage.getItem(KEPT);
    if (text === null) return initial;
    sessionStorage.removeItem(KEPT);
    kept = JSON.parse(text) as Kept;
  } catch {
    return initial;
  }
  if (kept.route !== route() || kept.shape !== shape) return initial;
  const values = { ...initial };
  for (const key of Object.keys(initial)) if (key in kept.values) values[key] = kept.values[key];
  return values;
}

/**
 * How long to wait before asking again, in milliseconds. `cargo hypersig dev`
 * stops the old server only once the new build is written and starts the new
 * one at once, so the new server listens soon after the old stream ends.
 */
const RETRY_MS = 50;

/**
 * How long to wait before asking again once no server has answered for
 * `IDLE_AFTER_MS`, in milliseconds. Each refused request writes an error to
 * the console, so a server that stays down is asked less often.
 */
const IDLE_RETRY_MS = 1000;

/** How long no server answers before the page asks every `IDLE_RETRY_MS`, in milliseconds. */
const IDLE_AFTER_MS = 3000;

/**
 * How long one request holds the stream before the page closes it and asks
 * again, in milliseconds. A server that shuts down gracefully waits for its
 * open responses, and this stream ends only when the server stops, so the
 * page closes it for the server to finish.
 */
const HOLD_MS = 2000;

/**
 * Keeps the stream at `url` open and reloads the page once a server other
 * than the one that rendered it answers there.
 *
 * The URL names the server that rendered the page, which holds the stream
 * open until it stops, and a server started in its place answers the same
 * URL with `hs-reload` at once. A failed request or a stream that ends means
 * that no server is answering, so the page asks again. A `404` means the
 * server serves no such stream, as one does that serves the runtime itself,
 * and the page stops asking. Each request is closed after `HOLD_MS`, so
 * that a server stopped with Ctrl+C waits for it no longer than that.
 * `keep` runs just before the reload.
 */
export function reloadOnRestart(
  url: string,
  navigation: Navigation,
  keep: () => void = () => {},
): void {
  // When the page lost the server, while no server has answered since.
  let refusedSince: number | undefined;
  const ask = async (): Promise<void> => {
    if (navigation.leaving) return;
    let reload = false;
    const hold = new AbortController();
    const timer = setTimeout(() => hold.abort(), HOLD_MS);
    try {
      const response = await fetch(url, {
        headers: { Accept: "text/event-stream" },
        signal: hold.signal,
      });
      refusedSince = undefined;
      if (response.status === 404) return;
      if (response.ok && response.body !== null) {
        await readEventStream(response.body, (event) => {
          if (event.event === "hs-reload") reload = true;
        });
      }
    } catch {
      // The server stopped, had not started listening yet, or held the
      // stream for `HOLD_MS`.
      if (!hold.signal.aborted) refusedSince ??= Date.now();
    } finally {
      clearTimeout(timer);
    }
    if (reload) {
      keep();
      navigation.reload();
      return;
    }
    const idle = refusedSince !== undefined && Date.now() - refusedSince >= IDLE_AFTER_MS;
    setTimeout(() => void ask(), idle ? IDLE_RETRY_MS : RETRY_MS);
  };
  void ask();
}
