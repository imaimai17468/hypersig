export interface ServerEvent {
  readonly event: string;
  readonly data: string;
}

/** Reads a `text/event-stream` body and calls `onEvent` for each complete event. */
export async function readEventStream(
  body: ReadableStream<Uint8Array>,
  onEvent: (event: ServerEvent) => void,
): Promise<void> {
  const reader = body.getReader();
  const decoder = new TextDecoder();
  let buffer = "";
  let event = "message";
  let data: string[] = [];

  const dispatch = () => {
    if (data.length > 0) onEvent({ event, data: data.join("\n") });
    event = "message";
    data = [];
  };

  for (;;) {
    const { value, done } = await reader.read();
    if (done) break;
    buffer += decoder.decode(value, { stream: true });
    const lines = buffer.split(/\r\n|\r|\n/);
    buffer = lines.pop() ?? "";
    for (const line of lines) {
      if (line === "") dispatch();
      else if (line.startsWith(":")) continue;
      else {
        const colon = line.indexOf(":");
        const field = colon === -1 ? line : line.slice(0, colon);
        const raw = colon === -1 ? "" : line.slice(colon + 1);
        const content = raw.startsWith(" ") ? raw.slice(1) : raw;
        if (field === "event") event = content;
        else if (field === "data") data.push(content);
      }
    }
  }
  dispatch();
}
