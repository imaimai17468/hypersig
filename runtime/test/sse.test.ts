import { readFileSync } from "node:fs";
import { resolve } from "node:path";
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
    streamOf(
      "event: hs-sig",
      'nals\ndata: {"a":1}\n\n: keep-alive\n\nevent: hs-element\ndata: {"html":"<p id=x></p>"}\n\n',
    ),
    (event) => events.push(event),
  );
  expect(events).toEqual([
    { event: "hs-signals", data: '{"a":1}' },
    { event: "hs-element", data: '{"html":"<p id=x></p>"}' },
  ]);
});

it("drops an event the body ends inside, before its blank line", async () => {
  const events: ServerEvent[] = [];
  await readEventStream(
    streamOf('event: hs-signals\ndata: {"a":1}\n\nevent: hs-signals\ndata: {"a":'),
    (event) => events.push(event),
  );
  expect(events).toEqual([{ event: "hs-signals", data: '{"a":1}' }]);
});

it("cancels the body when a handler throws, so the connection closes", async () => {
  let cancelled = false;
  const body = new ReadableStream<Uint8Array>({
    start(controller) {
      controller.enqueue(new TextEncoder().encode("data: 1\n\n"));
    },
    cancel() {
      cancelled = true;
    },
  });
  await expect(
    readEventStream(body, () => {
      throw new Error("handler failed");
    }),
  ).rejects.toThrow("handler failed");
  expect(cancelled).toBe(true);
});

/** Reads `bytes` handed over in the chunks that end at `cuts`. */
async function readCut(bytes: Uint8Array, cuts: readonly number[]): Promise<ServerEvent[]> {
  const events: ServerEvent[] = [];
  const ends = [...cuts, bytes.length];
  const body = new ReadableStream<Uint8Array>({
    start(controller) {
      let from = 0;
      for (const end of ends) {
        controller.enqueue(bytes.slice(from, end));
        from = end;
      }
      controller.close();
    },
  });
  await readEventStream(body, (event) => events.push(event));
  return events;
}

it("reads the same events wherever the body is cut into two chunks or many", async () => {
  // The batches the Rust builder writes, and one with characters UTF-8
  // writes in two, three, and four bytes, so that a cut falls inside one.
  const streams = ["feed_patches", "history_patch", "styled_patch", "widgets_patch"].map(
    (name) =>
      (
        JSON.parse(readFileSync(resolve(__dirname, `fixtures/${name}.json`), "utf8")) as {
          stream: string;
        }
      ).stream,
  );
  streams.push(
    `event: hs-signals\ndata: ${JSON.stringify({ note: "é 日本語 😀\n<b>&amp;</b>", n: -0.5 })}\n\n` +
      `event: hs-element\ndata: ${JSON.stringify({ html: '<p id="x">\n\n😀</p>' })}\n\n` +
      "event: hs-end\ndata: {}\n\n",
  );
  const bodies = streams.map((stream) => new TextEncoder().encode(stream));
  const whole = await Promise.all(bodies.map((bytes) => readCut(bytes, [])));
  expect(Math.min(...whole.map((events) => events.length))).toBeGreaterThan(1);
  const cuts = bodies.flatMap((bytes, body) => {
    const offsets = Array.from({ length: bytes.length - 1 }, (_, index) => index + 1);
    return [offsets, ...offsets.map((offset) => [offset])].map((at) => ({ bytes, at, body }));
  });
  const read = await Promise.all(cuts.map(({ bytes, at }) => readCut(bytes, at)));
  expect(read).toEqual(cuts.map(({ body }) => whole[body]));
});
