import { expect, it } from "vitest";
import { readEventStream, type ServerEvent } from "../src/sse.ts";

function streamOf(...chunks: string[]): ReadableStream<Uint8Array> {
  const encoder = new TextEncoder();
  return new ReadableStream({
    start(controller) {
      for (const chunk of chunks) controller.enqueue(encoder.encode(chunk));
      controller.close();
    },
  });
}

it("reassembles events split across chunks", async () => {
  const events: ServerEvent[] = [];
  await readEventStream(
    streamOf("event: hs-sig", "nals\ndata: {\"a\":1}\n\n: keep-alive\n\nevent: hs-element\ndata: {\"html\":\"<p id=x></p>\"}\n\n"),
    (event) => events.push(event),
  );
  expect(events).toEqual([
    { event: "hs-signals", data: '{"a":1}' },
    { event: "hs-element", data: '{"html":"<p id=x></p>"}' },
  ]);
});
