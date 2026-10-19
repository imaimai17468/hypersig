/** Events handled by one listener on the root, matched to the closest element with a handler. */
export const BUBBLING = [
  "click",
  "dblclick",
  "input",
  "change",
  "submit",
  "keydown",
  "focusin",
  "focusout",
] as const;
/** Events that do not bubble, caught in the capture phase and matched to the target only. */
export const DIRECT = ["mouseenter", "mouseleave"] as const;

const ATTRIBUTES = [
  "show",
  "text",
  "checked",
  "bind",
  "class",
  "attr",
  "toggle",
  "style",
  "effect",
  "on-signal",
  "on-interval",
  "on-timeout",
  "each",
  "if",
  "on-load",
  "on-visible",
  "on-key",
  "on-custom",
  ...BUBBLING.map((type) => `on-${type}`),
  ...DIRECT.map((type) => `on-${type}`),
];
/** A selector that matches an element with any attribute the runtime binds. */
export const REACTIVE = ATTRIBUTES.map((name) => `[data-hs-${name}]`).join(",");

/** A two-way binding of a form control: the place it writes, how it reads the value, and a radio button's value. */
export type Bind = [place: unknown, kind: string, value?: unknown];

/** The parsed `data-hs-*` attributes an element builds its bindings from. */
export interface Spec {
  readonly show: unknown;
  readonly text: unknown;
  readonly checked: unknown;
  readonly classes: readonly [string, unknown][];
  readonly attrs: readonly [string, unknown][];
  readonly toggles: readonly [string, unknown][];
  readonly styles: readonly [string, unknown][];
  readonly bind: Bind | undefined;
  readonly effects: readonly unknown[];
  readonly watches: readonly [unknown, unknown][];
  /** The element a `<template>` holds, which its list items and condition are built from. */
  readonly proto: Element | null;
  readonly each: [place: unknown, depth: number] | undefined;
  readonly cond: unknown;
}

const parsed = new Map<string, unknown>();

/**
 * `JSON.parse(source)`, remembered, so the items of a list share one AST per
 * attribute and `evaluate` builds each expression once. The result is never
 * modified, because the store copies a value before changing it.
 */
export function parse(source: string | null | undefined): unknown {
  if (source === null || source === undefined) return undefined;
  let value = parsed.get(source);
  if (value === undefined) {
    value = JSON.parse(source);
    if (parsed.size > 10_000) parsed.clear();
    parsed.set(source, value);
  }
  return value;
}

/**
 * The `data-*` attributes of `element`, which the runtime reads by their
 * `dataset` names: `hsEach` for `data-hs-each`. Elements outside HTML, SVG,
 * and MathML have none.
 */
export function data(element: Element): DOMStringMap {
  return (element as Partial<HTMLOrSVGElement>).dataset ?? {};
}

function pairs(value: unknown): [string, unknown][] {
  return Array.isArray(value) ? (value as [string, unknown][]) : [];
}

export function readSpec(element: Element): Spec {
  const set = data(element);
  const effects = parse(set.hsEffect);
  const watches = parse(set.hsOnSignal);
  const each = parse(set.hsEach);
  return {
    show: parse(set.hsShow),
    text: parse(set.hsText),
    checked: parse(set.hsChecked),
    classes: pairs(parse(set.hsClass)),
    attrs: pairs(parse(set.hsAttr)),
    toggles: pairs(parse(set.hsToggle)),
    styles: pairs(parse(set.hsStyle)),
    bind: parse(set.hsBind) as Bind | undefined,
    effects: Array.isArray(effects) ? effects : [],
    watches: Array.isArray(watches) ? (watches as [unknown, unknown][]) : [],
    proto: element instanceof HTMLTemplateElement ? element.content.firstElementChild : null,
    each: Array.isArray(each) ? (each as [unknown, number]) : undefined,
    cond: parse(set.hsIf),
  };
}

const specs = new WeakMap<Element, Spec>();

/**
 * `readSpec(source)`, remembered for an element inside a `<template>`, whose
 * attributes no patch changes, so the items built from it share one spec.
 */
export function specOf(source: Element): Spec {
  let spec = specs.get(source);
  if (spec === undefined) {
    spec = readSpec(source);
    specs.set(source, spec);
  }
  return spec;
}
