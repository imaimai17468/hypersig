import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Runtime } from "../../src/runtime.ts";

/** One event of a stream, as `Runtime.apply` takes it. */
export interface StreamEvent {
  event: string;
  data: string;
}

/** Reads a fixture written by `crates/hypersig/tests/fixtures.rs`. */
function read<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(__dirname, `../fixtures/${name}.json`), "utf8")) as T;
}

/** Mounts a fixture written by `crates/hypersig/tests/fixtures.rs`. */
export function mount(name: string): Runtime {
  const { html, signals } = read<{ html: string; signals: Record<string, unknown> }>(name);
  const root = document.createElement("div");
  root.innerHTML = html;
  document.body.replaceChildren(root);
  return new Runtime(root, signals);
}

/** The events of a stream fixture written by `crates/hypersig/tests/fixtures.rs`. */
export function events(name: string): StreamEvent[] {
  const { stream } = read<{ stream: string }>(name);
  return stream
    .trim()
    .split("\n\n")
    .map((block) => {
      const [event, data] = block.split("\n");
      return { event: event!.slice("event: ".length), data: data!.slice("data: ".length) };
    });
}

/** Loads a document written by `Page::render`, without starting the runtime. */
export function serve(name: string): void {
  const { document: html } = read<{ document: string }>(name);
  const parsed = new DOMParser().parseFromString(html, "text/html");
  document.body.replaceChildren(...parsed.body.childNodes);
  for (const old of document.head.querySelectorAll("style")) old.remove();
  document.head.append(...parsed.head.querySelectorAll("style"));
  for (const { name: attribute } of Array.from(document.body.attributes))
    document.body.removeAttribute(attribute);
  for (const { name: attribute, value } of Array.from(parsed.body.attributes))
    document.body.setAttribute(attribute, value);
  document.title = parsed.title;
}

/** Starts the runtime on the document `serve` loaded, with the signals its body carries. */
export function start(): Runtime {
  return new Runtime(
    document.body,
    JSON.parse(document.body.dataset.hsSignals!) as Record<string, unknown>,
  );
}
