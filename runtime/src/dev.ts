import type { Navigation } from "./runtime/navigation.ts";
import { readEventStream } from "./sse.ts";

/** How long to wait before asking a server that did not answer again, in milliseconds. */
const RETRY_MS = 250;

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
 */
export function reloadOnRestart(url: string, navigation: Navigation): void {
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
      if (response.status === 404) return;
      if (response.ok && response.body !== null) {
        await readEventStream(response.body, (event) => {
          if (event.event === "hs-reload") reload = true;
        });
      }
    } catch {
      // The server stopped, had not started listening yet, or held the
      // stream for `HOLD_MS`.
    } finally {
      clearTimeout(timer);
    }
    if (reload) navigation.reload();
    else setTimeout(() => void ask(), RETRY_MS);
  };
  void ask();
}
