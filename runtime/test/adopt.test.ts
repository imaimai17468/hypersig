import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it } from "vitest";
import { Runtime } from "../src/runtime.ts";
import { tick } from "./dom.ts";

/** The file `crates/hypersig/tests/adopt.rs` writes: documents `Page::render` served for generated pages. */
const { pages } = JSON.parse(readFileSync(resolve(__dirname, "fixtures/adopt.json"), "utf8")) as {
  pages: readonly string[];
};

/** What a reader of the page sees of one element: its attributes and its own text. */
interface Seen {
  readonly element: Element;
  readonly attributes: Record<string, string>;
  readonly text: string;
  readonly value: string | boolean | undefined;
}

function see(element: Element): Seen {
  const attributes: Record<string, string> = {};
  for (const { name, value } of Array.from(element.attributes)) attributes[name] = value;
  const text = Array.from(element.childNodes, (node) =>
    node.nodeType === Node.TEXT_NODE ? node.textContent : "\u0000",
  ).join("");
  const value =
    element instanceof HTMLInputElement
      ? element.type === "checkbox"
        ? element.checked
        : element.value
      : element instanceof HTMLTextAreaElement
        ? element.value
        : undefined;
  return { element, attributes, text, value };
}

/**
 * The differences between what the server wrote and what the runtime shows
 * after it starts. The server marks an element it hid with `data-hs-hide`,
 * which the runtime turns into `display: none`, so those two are left out.
 */
function differences(before: readonly Seen[]): string[] {
  const found: string[] = [];
  for (const old of before) {
    const where = `<${old.element.tagName.toLowerCase()}> ${JSON.stringify(old.attributes)}`;
    if (!old.element.isConnected) {
      found.push(`replaced ${where}`);
      continue;
    }
    const now = see(old.element);
    const hidden = "data-hs-hide" in old.attributes;
    for (const name of new Set([...Object.keys(old.attributes), ...Object.keys(now.attributes)])) {
      if (name === "data-hs-hide" || (name === "style" && hidden)) continue;
      if (old.attributes[name] !== now.attributes[name])
        found.push(`${where} ${name}: ${old.attributes[name]} -> ${now.attributes[name]}`);
    }
    if (hidden && (old.element as HTMLElement).style.display !== "none")
      found.push(`${where} shown`);
    if (old.text !== now.text)
      found.push(`${where} text: ${JSON.stringify(old.text)} -> ${JSON.stringify(now.text)}`);
    if (old.value !== now.value)
      found.push(`${where} value: ${JSON.stringify(old.value)} -> ${JSON.stringify(now.value)}`);
  }
  const kept = new Set(before.map(({ element }) => element));
  for (const element of document.body.querySelectorAll("*"))
    if (!kept.has(element)) found.push(`added ${element.outerHTML.slice(0, 200)}`);
  return found;
}

/** Puts one document into the page as the browser parses it. */
function load(html: string): void {
  const parsed = new DOMParser().parseFromString(html, "text/html");
  document.body.replaceChildren(...parsed.body.childNodes);
  for (const { name } of Array.from(document.body.attributes)) document.body.removeAttribute(name);
  for (const { name, value } of Array.from(parsed.body.attributes))
    document.body.setAttribute(name, value);
}

/** Starts the runtime on the document `load` put into the page. */
function start(): Runtime {
  return new Runtime(
    document.body,
    JSON.parse(document.body.dataset.hsSignals!) as Record<string, unknown>,
  );
}

it.each(pages.map((html, page) => [page, html] as const))(
  "starts on generated page %i without changing what the server wrote",
  async (_, html) => {
    load(html);
    const before = Array.from(document.body.querySelectorAll("*"), see);
    start();
    await tick();
    await tick();
    expect(differences(before)).toEqual([]);
  },
);
