/** The checks of the property test in `morph.property.test.ts`, run on one pair of trees. */

import { morph } from "../../src/morph.ts";
import { FOREIGN, build, type Case } from "./generate.ts";

function escape(text: string): string {
  return text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
}

function ownsOpen(element: Element): boolean {
  return element.localName === "details" || element.localName === "dialog";
}

/**
 * A canonical form of `node`: attributes sorted, because `morph` sets a new
 * attribute after the ones an element already has. It leaves out what `morph`
 * keeps on purpose, which the checks compare on their own: `open` on a
 * `<details>` or `<dialog>`, a foreign element's other attributes and
 * children, and the elements in `skip`.
 */
function canonical(node: Node, skip: ReadonlySet<Element>): string {
  if (node instanceof Text) return escape(node.data);
  if (!(node instanceof Element)) return "";
  const foreign = node.hasAttribute(FOREIGN);
  const attrs = [...node.attributes]
    .filter(({ name }) => !(name === "open" && ownsOpen(node)))
    .filter(({ name }) => !foreign || name === FOREIGN || name === "id")
    .map(({ name, value }) => `${name}="${escape(value)}"`)
    .toSorted();
  const open = `<${[node.localName, ...attrs].join(" ")}>`;
  if (foreign) return `${open}</${node.localName}>`;
  if (node instanceof HTMLTemplateElement) return `${open}${node.innerHTML}</template>`;
  let inner = "";
  // Adjacent text nodes read as one, as they would once serialized and parsed again.
  for (const child of node.childNodes)
    if (!(child instanceof Element && skip.has(child))) inner += canonical(child, skip);
  return `${open}${inner}</${node.localName}>`;
}

function keyOf(element: Element): string {
  if (element.id !== "") return `#${element.id}`;
  const foreign = element.getAttribute(FOREIGN);
  return foreign === null ? "" : `@${foreign}`;
}

function templates(list: readonly Element[]): HTMLTemplateElement[] {
  return list.filter((child) => child instanceof HTMLTemplateElement);
}

function like(list: readonly Element[], model: Element): Element[] {
  return list.filter(
    (child) =>
      child.tagName === model.tagName && child.getAttribute("name") === model.getAttribute("name"),
  );
}

/**
 * Adds to `unkeyed` the children of `from` without a key that `morph`
 * matches all the same: the one `<template>` of a parent where `to` has one
 * with the same attributes and content, and an element whose tag and `name` no sibling
 * shares in either tree, among siblings with no `<template>`, where the
 * lookahead stops.
 */
function unkeyedStay(from: Element, to: Element, unkeyed: Set<Element>): void {
  const mine = [...from.children].filter((child) => keyOf(child) === "");
  const yours = [...to.children].filter((child) => keyOf(child) === "");
  const [template, ...more] = templates(mine);
  const [other, ...others] = templates(yours);
  if (
    template !== undefined &&
    other !== undefined &&
    more.length === 0 &&
    others.length === 0 &&
    template.innerHTML === other.innerHTML &&
    template.getAttributeNames().join() === other.getAttributeNames().join() &&
    template
      .getAttributeNames()
      .every((name) => template.getAttribute(name) === other.getAttribute(name))
  )
    unkeyed.add(template);
  if (templates([...from.children]).length > 0) return;
  for (const child of mine)
    if (
      child.hasAttribute("name") &&
      like(mine, child).length === 1 &&
      like(yours, child).length === 1
    )
      unkeyed.add(child);
}

/**
 * Adds to `stay` the elements of `from` that a morph into `to` must keep in
 * the document: the root, and each child with a key that the same parent in
 * `to` also has on an element of the same tag, under a parent that is itself
 * kept.
 */
function mustStay(from: Element, to: Element, stay: Set<Element>, unkeyed: Set<Element>): void {
  stay.add(from);
  if (from instanceof HTMLTemplateElement || from.hasAttribute(FOREIGN)) return;
  const theirs = new Map<string, Element>();
  for (const child of to.children) {
    const key = keyOf(child);
    if (key !== "") theirs.set(key, child);
  }
  for (const child of from.children) {
    const match = theirs.get(keyOf(child));
    if (keyOf(child) === "" || match === undefined || match.tagName !== child.tagName) continue;
    if (child.hasAttribute(FOREIGN) && child.getAttribute(FOREIGN) !== match.getAttribute(FOREIGN))
      stay.add(child);
    else mustStay(child, match, stay, unkeyed);
  }
  unkeyedStay(from, to, unkeyed);
}

let observer: MutationObserver | undefined;
let disconnected: Node[] = [];

function drain(): void {
  if (observer === undefined) return;
  for (const record of observer.takeRecords()) disconnected.push(...record.removedNodes);
}

/**
 * A stand-in for `Element.moveBefore`, which happy-dom lacks: `insertBefore`,
 * with the removal it records left out, because a browser's `moveBefore`
 * keeps the element in the document while it moves.
 */
export function installMoveBefore(): () => void {
  Object.defineProperty(Element.prototype, "moveBefore", {
    configurable: true,
    value(this: Element, node: Node, child: Node | null): void {
      drain();
      this.insertBefore(node, child);
      observer?.takeRecords();
    },
  });
  return () => {
    Reflect.deleteProperty(Element.prototype, "moveBefore");
  };
}

/**
 * `root` and the elements under it. A walk rather than `querySelectorAll`,
 * whose results happy-dom 21 keeps alive, which over 10,000 cases fills the heap.
 */
function elements(root: Element, into: Element[]): Element[] {
  into.push(root);
  for (const child of root.children) elements(child, into);
  return into;
}

/** One case in the document, with what the checks read from it before the morph. */
interface Before {
  from: Element;
  to: Element;
  drawn: Map<HTMLTemplateElement, Set<Element>>;
  expected: string;
  stay: Set<Element>;
  unkeyed: Set<Element>;
  elements: Element[];
  toElements: Set<Element>;
  /** Whether each `<details>` and `<dialog>` of `from` had `open`. */
  open: Map<Element, boolean>;
  /** The foreign name and the HTML of each foreign element of `from`. */
  foreign: Map<Element, [string | null, string]>;
}

function prepare(test: Case): Before {
  document.body.replaceChildren();
  const from = document.createElement("div");
  from.id = "root";
  const drawn = new Map<HTMLTemplateElement, Set<Element>>();
  const counter = { next: 0 };
  build(test.from, from, drawn, counter, true);
  document.body.append(from);
  const to = document.createElement("div");
  to.id = "root";
  build(test.to, to, new Map(), counter, false);
  const stay = new Set<Element>();
  const unkeyed = new Set<Element>();
  mustStay(from, to, stay, unkeyed);
  const before = elements(from, []);
  const open = new Map(
    before.filter((element) => ownsOpen(element)).map((el) => [el, el.hasAttribute("open")]),
  );
  const foreign = new Map<Element, [string | null, string]>();
  for (const element of before)
    if (element.hasAttribute(FOREIGN))
      foreign.set(element, [element.getAttribute(FOREIGN), element.outerHTML]);
  const expected = canonical(to, new Set());
  const toElements = new Set(elements(to, []));
  return { from, to, drawn, expected, stay, unkeyed, elements: before, toElements, open, foreign };
}

/** What a morph did: the element it returned, what it pushed onto `removed`, and what left the document. */
interface After {
  morphed: Element;
  removed: Element[];
  left: Set<Node>;
}

function run(before: Before): After {
  observer = new MutationObserver(() => {});
  observer.observe(document.body, { subtree: true, childList: true });
  disconnected = [];
  const removed: Element[] = [];
  try {
    const morphed = morph(before.from, before.to, removed, (template) =>
      before.drawn.get(template),
    );
    drain();
    // Without `moveBefore`, `move` takes an element out to move it, as the doc comment of `morph` allows.
    return {
      morphed,
      removed,
      left: new Set("moveBefore" in Element.prototype ? disconnected : []),
    };
  } finally {
    observer.disconnect();
    observer = undefined;
  }
}

/** Property 1: the page reads as `to`, apart from what `morph` keeps on purpose. */
function content(before: Before, after: After): string[] {
  const problems: string[] = [];
  const skip = new Set<Element>();
  for (const [template, items] of before.drawn) {
    if (!template.isConnected) continue;
    let next = template.nextElementSibling;
    for (const item of items) {
      if (next !== item)
        problems.push(`drawn element ${item.outerHTML} no longer follows its template`);
      skip.add(item);
      next = next?.nextElementSibling ?? null;
    }
  }
  const actual = canonical(after.morphed, skip);
  if (actual !== before.expected)
    problems.push(`content differs\n  want ${before.expected}\n  got  ${actual}`);
  for (const [element, was] of before.open)
    if (element.isConnected && element.hasAttribute("open") !== was)
      problems.push(`${element.localName}${keyOf(element)} changed open`);
  for (const [element, [name, html]] of before.foreign)
    // One that `to` gives another foreign name, or none, under the same id, morphs as any other element.
    if (element.isConnected && element.getAttribute(FOREIGN) === name && element.outerHTML !== html)
      problems.push(`foreign element ${keyOf(element)} changed: ${element.outerHTML}`);
  return problems;
}

/** Property 2: an element `to` still has under the same parent never leaves the document. */
function kept(before: Before, after: After): string[] {
  const problems: string[] = [];
  const gone = (element: Element) => after.left.has(element) || !element.isConnected;
  for (const element of before.stay)
    if (gone(element))
      problems.push(`kept element ${element.localName}${keyOf(element)} left the document`);
  for (const element of before.unkeyed) {
    if (!(element instanceof HTMLTemplateElement)) {
      if (gone(element)) problems.push(`${element.localName} matched by name left the document`);
      continue;
    }
    if (gone(element)) problems.push("template matched by position left the document");
    for (const item of before.drawn.get(element) ?? [])
      if (gone(item)) problems.push("template item matched by position left the document");
  }
  return problems;
}

/** Property 3: `removed` names exactly the elements that are no longer in the document. */
function collected(before: Before, after: After): string[] {
  const problems: string[] = [];
  const gone = new Set(after.removed);
  if (gone.size !== after.removed.length) problems.push("removed holds an element twice");
  for (const element of after.removed) {
    if (element.isConnected)
      problems.push(`removed holds ${element.localName}${keyOf(element)}, still in the document`);
    if (before.toElements.has(element))
      problems.push(`removed holds ${element.localName}${keyOf(element)} from to`);
  }
  for (const element of before.elements) {
    if (element.isConnected) continue;
    let top: Element | null = element;
    while (top !== null && !gone.has(top)) top = top.parentElement;
    if (top === null)
      problems.push(
        `${element.localName}${keyOf(element)} left the document without being in removed`,
      );
  }
  return problems;
}

/** Morphs one case in the document and returns what broke, or an empty list. */
export function check(test: Case): string[] {
  const before = prepare(test);
  const after = run(before);
  return [...content(before, after), ...kept(before, after), ...collected(before, after)];
}
