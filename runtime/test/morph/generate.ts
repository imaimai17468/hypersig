/** The generator of the property test in `morph.property.test.ts`: seeded pairs of trees. */

/** The attribute `El::foreign` writes, which `morph` keeps an element by. */
export const FOREIGN = "data-hs-foreign";

/** A node of a generated tree, which `build` turns into DOM nodes. */
export type Model =
  | { kind: "text"; text: string }
  | {
      kind: "element";
      tag: string;
      attrs: [string, string][];
      children: Model[];
      /** The content of a `<template>`. */
      content?: string;
      /** How many elements the runtime drew after this `<template>`, on the `from` side only. */
      drawn?: number;
    };

export type ElementModel = Extract<Model, { kind: "element" }>;

/** mulberry32, so that a seed names one sequence of cases on every machine. */
export function random(seed: number): () => number {
  let state = seed >>> 0;
  return () => {
    state = (state + 0x6d2b79f5) >>> 0;
    let t = state;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4_294_967_296;
  };
}

const IDS = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"];
const NAMES = ["n1", "n2", "n3"];
const FOREIGN_NAMES = ["f1", "f2", "f3"];
// Tags the HTML serializer and the DOM agree on at any depth, so a tree built here reads back as built.
const TAGS = ["div", "section", "span", "em", "label"];
const TEMPLATES = ["a", "b", "<i>x</i>"];
const TEXTS = ["x", "y", "z"];

const MAX_DEPTH = 3;
const MAX_CHILDREN = 6;

class Gen {
  constructor(
    readonly rand: () => number,
    readonly used: Set<string>,
  ) {}

  int(n: number): number {
    return Math.floor(this.rand() * n);
  }

  pick<T>(items: readonly T[]): T {
    return items[this.int(items.length)]!;
  }

  chance(p: number): boolean {
    return this.rand() < p;
  }

  /** A name from `pool` that the tree does not use yet, because a page holds each id and node ref once. */
  fresh(pool: readonly string[]): string | undefined {
    const free = pool.filter((id) => !this.used.has(id));
    if (free.length === 0) return undefined;
    const id = this.pick(free);
    this.used.add(id);
    return id;
  }

  children(depth: number, from: boolean): Model[] {
    if (depth >= MAX_DEPTH) return [];
    const count = this.int(MAX_CHILDREN + 1);
    const out: Model[] = [];
    for (let i = 0; i < count; i++) out.push(this.node(depth + 1, from));
    return out;
  }

  node(depth: number, from: boolean): Model {
    const roll = this.int(16);
    if (roll < 2) return { kind: "text", text: this.pick(TEXTS) };
    if (roll < 5) return this.element(this.pick(TAGS), [], depth, from);
    if (roll < 9) {
      const id = this.fresh(IDS);
      return this.element(this.pick(TAGS), id === undefined ? [] : [["id", id]], depth, from);
    }
    if (roll < 11) {
      const attrs: [string, string][] = [["name", this.pick(NAMES)]];
      const id = this.chance(0.3) ? this.fresh(IDS) : undefined;
      if (id !== undefined) attrs.unshift(["id", id]);
      if (this.chance(0.3)) attrs.push(["value", this.pick(TEXTS)]);
      return { kind: "element", tag: "input", attrs, children: [] };
    }
    if (roll < 12) {
      // The server writes a template with no id, so `morph` matches it by position.
      const attrs: [string, string][] = [[this.chance(0.5) ? "data-hs-each" : "data-hs-if", "[]"]];
      const template: ElementModel = {
        kind: "element",
        tag: "template",
        attrs,
        children: [],
        content: this.pick(TEMPLATES),
      };
      if (from && this.chance(0.6)) template.drawn = 1 + this.int(2);
      return template;
    }
    if (roll < 14) {
      const attrs: [string, string][] = [];
      const id = this.chance(0.5) ? this.fresh(IDS) : undefined;
      if (id !== undefined) attrs.push(["id", id]);
      if (this.chance(0.5)) attrs.push(["open", ""]);
      return this.element(this.chance(0.7) ? "details" : "dialog", attrs, depth, from);
    }
    const name = this.fresh(FOREIGN_NAMES);
    if (name === undefined) return { kind: "text", text: this.pick(TEXTS) };
    const attrs: [string, string][] = [[FOREIGN, name]];
    const id = this.chance(0.3) ? this.fresh(IDS) : undefined;
    if (id !== undefined) attrs.unshift(["id", id]);
    return { kind: "element", tag: "div", attrs, children: [] };
  }

  element(tag: string, attrs: [string, string][], depth: number, from: boolean): ElementModel {
    if (this.chance(0.3)) attrs.push(["class", this.pick(TEXTS)]);
    return { kind: "element", tag, attrs, children: this.children(depth, from) };
  }
}

export function attr(model: ElementModel, name: string): string | undefined {
  return model.attrs.find(([key]) => key === name)?.[1];
}

function setAttr(model: ElementModel, name: string, value: string | undefined): void {
  model.attrs = model.attrs.filter(([key]) => key !== name);
  if (value !== undefined) model.attrs.push([name, value]);
}

function idsOf(models: readonly Model[], into: Set<string>): Set<string> {
  for (const model of models) {
    if (model.kind !== "element") continue;
    for (const name of ["id", FOREIGN]) {
      const value = attr(model, name);
      if (value !== undefined) into.add(value);
    }
    idsOf(model.children, into);
  }
  return into;
}

/** A copy of `from`'s children with the deletions, moves and edits a server render might make. */
function edit(gen: Gen, children: readonly Model[], depth: number): Model[] {
  let out: Model[] = [];
  for (const child of children) {
    if (gen.chance(0.2)) continue;
    if (child.kind === "text") {
      out.push({ kind: "text", text: gen.chance(0.2) ? gen.pick(TEXTS) : child.text });
      continue;
    }
    const copy: ElementModel = {
      kind: "element",
      tag: child.tag,
      attrs: child.attrs.map(([key, value]) => [key, value]),
      children: edit(gen, child.children, depth + 1),
    };
    if (child.content !== undefined)
      copy.content = gen.chance(0.3) ? gen.pick(TEMPLATES) : child.content;
    if (gen.chance(0.08) && TAGS.includes(copy.tag)) copy.tag = gen.pick(TAGS);
    if (gen.chance(0.2)) setAttr(copy, "class", gen.chance(0.5) ? gen.pick(TEXTS) : undefined);
    if ((copy.tag === "details" || copy.tag === "dialog") && gen.chance(0.3))
      setAttr(copy, "open", attr(copy, "open") === undefined ? "" : undefined);
    if (copy.tag === "input" && gen.chance(0.2)) setAttr(copy, "value", gen.pick(TEXTS));
    out.push(copy);
  }
  if (gen.chance(0.3)) {
    for (let i = out.length - 1; i > 0; i--) {
      const j = gen.int(i + 1);
      [out[i], out[j]] = [out[j]!, out[i]!];
    }
  }
  return out;
}

function insert(gen: Gen, children: Model[], depth: number): Model[] {
  for (const child of children)
    if (
      child.kind === "element" &&
      child.content === undefined &&
      child.tag !== "input" &&
      attr(child, FOREIGN) === undefined
    )
      child.children = insert(gen, child.children, depth + 1);
  if (depth >= MAX_DEPTH) return children;
  const out = [...children];
  const count = gen.int(3);
  for (let i = 0; i < count && out.length < MAX_CHILDREN; i++)
    out.splice(gen.int(out.length + 1), 0, gen.node(depth + 1, false));
  return out;
}

/** One pair of trees: the children of `from`'s root and of `to`'s root. */
export interface Case {
  from: Model[];
  to: Model[];
}

export function generate(rand: () => number): Case {
  const from = new Gen(rand, new Set()).children(0, true);
  if (rand() < 0.15) return { from, to: new Gen(rand, new Set()).children(0, false) };
  const gen = new Gen(rand, new Set());
  const edited = edit(gen, from, 0);
  idsOf(edited, gen.used);
  return { from, to: insert(gen, edited, 0) };
}

/** The DOM nodes of `models`, and the elements drawn after each `<template>`. */
export function build(
  models: readonly Model[],
  parent: Element,
  drawn: Map<HTMLTemplateElement, Set<Element>>,
  counter: { next: number },
  from: boolean,
): void {
  for (const model of models) {
    if (model.kind === "text") {
      parent.append(document.createTextNode(model.text));
      continue;
    }
    const element = document.createElement(model.tag);
    for (const [key, value] of model.attrs) element.setAttribute(key, value);
    parent.append(element);
    if (element instanceof HTMLTemplateElement) {
      element.innerHTML = model.content ?? "";
      const items = new Set<Element>();
      for (let i = 0; i < (model.drawn ?? 0); i++) {
        const item = document.createElement("span");
        item.dataset["drawn"] = String(counter.next++);
        item.textContent = "item";
        parent.append(item);
        items.add(item);
      }
      if (items.size > 0) drawn.set(element, items);
      continue;
    }
    build(model.children, element, drawn, counter, from);
    // A script owns a foreign element's attributes and children, as a map library does.
    if (from && attr(model, FOREIGN) !== undefined) {
      element.classList.add("owned");
      element.append(document.createElement("canvas"));
    }
  }
}

function html(models: readonly Model[], from: boolean): string {
  const root = document.createElement("div");
  root.id = "root";
  build(models, root, new Map(), { next: 0 }, from);
  return root.outerHTML;
}

/** The HTML of a case's two trees, for a report of a failure. */
export function show(test: Case): string {
  return `from: ${html(test.from, true)}\nto:   ${html(test.to, false)}`;
}
