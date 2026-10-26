export interface ServerEvent {
  readonly event: string;
  readonly data: string;
}

/**
 * Reads a `text/event-stream` body and calls `onEvent` for each complete
 * event. An event the body ends inside, before its blank line, is dropped,
 * as the EventSource specification drops it, because a connection cut in the
 * middle of an event leaves only part of its data. Reading stops once
 * `signal` aborts.
 */
export async function readEventStream(
  body: ReadableStream<Uint8Array>,
  onEvent: (event: ServerEvent) => void,
  signal?: AbortSignal,
): Promise<void> {
  const reader = body.getReader();
  // A body that does not come from a `fetch` given the same signal is not ended by the abort, so the reader is cancelled.
  const stop = (): void => {
    reader.cancel().catch(() => null);
  };
  signal?.addEventListener("abort", stop, { once: true });
  const take = parser(onEvent);
  // `reader.read()` answers in the shape of an iterator's `next`, so the chunks can be walked with `for await`.
  const chunks: AsyncIterable<Uint8Array> = {
    [Symbol.asyncIterator]: () => ({ next: () => reader.read() }),
  };
  try {
    for await (const chunk of chunks) take(chunk);
  } finally {
    signal?.removeEventListener("abort", stop);
    // An `onEvent` that throws leaves the loop with the body still open, and
    // the connection stays open until the reader cancels it.
    stop();
  }
}

/** A function that takes the chunks of a body in order and calls `onEvent` for each event they complete. */
function parser(onEvent: (event: ServerEvent) => void): (chunk: Uint8Array) => void {
  const decoder = new TextDecoder();
  let buffer = "";
  let event = "message";
  let data: string[] = [];

  const dispatch = () => {
    if (data.length > 0) onEvent({ event, data: data.join("\n") });
    event = "message";
    data = [];
  };

  return (chunk) => {
    buffer += decoder.decode(chunk, { stream: true });
    const lines = buffer.split(/\r\n|\r|\n/u);
    buffer = lines.pop() ?? "";
    for (const line of lines) {
      if (line === "") dispatch();
      else if (!line.startsWith(":")) {
        const colon = line.indexOf(":");
        const field = colon === -1 ? line : line.slice(0, colon);
        const raw = colon === -1 ? "" : line.slice(colon + 1);
        const content = raw.startsWith(" ") ? raw.slice(1) : raw;
        if (field === "event") event = content;
        else if (field === "data") data.push(content);
      }
    }
  };
}
