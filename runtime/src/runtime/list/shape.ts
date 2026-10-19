import { type Compiled, compile, type Slot } from "../../eval.ts";
import { placeRead, type StaticRead, staticReads } from "../../reads.ts";
import { join } from "../../store/path.ts";
import { positionOf } from "../../store/change.ts";
import { bound, planOf, sources } from "../plan.ts";
import { comparison } from "../selector.ts";
import { data, parse, type Spec, specOf } from "../spec.ts";
import { CONDITIONS, type Write, writes } from "../write.ts";

/**
 * One binding of an element inside a list item, which the item's `Row` runs
 * in place of an `Effect` of its own. The reads it can make are known from
 * its AST, so the runtime neither records them as it runs nor registers each
 * item's copy of it in the `DepIndex`.
 */
export interface Part {
  /**
   * What it writes, or for a `<template>` what it renders after itself: `if`
   * a condition, which it shows or removes, and `each` a nested list over a
   * field of the item.
   */
  readonly kind: Write | "bind" | "if" | "each";
  /** The class, attribute, or CSS property it writes. */
  readonly name: string;
  /** The expression, for `bind` the `Bind`, and for `each` the list's place and item depth. */
  readonly ast: unknown;
  /** The expression built by `compile`, null for `bind` and `each`. */
  readonly value: Compiled | null;
  /** For `["eq", ["sig", path], other]` where `other` reads only the item: the signal and `other`, built by `compile`. */
  readonly selector: { readonly path: string; readonly other: Compiled } | null;
  /** For `if` and `each`, the element its template holds. */
  readonly proto: Element | null;
}

/** What the items of one `<template>` share, worked out once from its element. */
export interface Shape {
  readonly parts: readonly Part[];
  /** The parts of each element of the template whose bindings are all parts. */
  readonly partsOf: ReadonlyMap<Element, readonly number[]>;
  /** The item fields each part reads, `""` for the whole item. */
  readonly fields: readonly (readonly string[])[];
  /**
   * The parts whose field reads are shallow: the `each` parts, which run
   * again when the list at the field is replaced or resized. A change inside
   * one of its items reaches that item's own row.
   */
  readonly shallow: number;
  /** The reads outside the item, each with the parts that make it. */
  readonly external: readonly (readonly [StaticRead, number])[];
  /** The parts that read the item's position. */
  readonly moved: number;
  readonly all: number;
  /** The item fields the key reads, where it reads nothing else, or null. */
  readonly keyFields: readonly string[] | null;
  /**
   * Whether every binding of an item is a part and its key reads only the
   * item, so nothing in the `DepIndex` reads under the item's slot name.
   */
  readonly whole: boolean;
  /** `maskFor`'s answers, by change. */
  readonly masks: Map<string, number>;
}

/** The parts of a template's elements as `shapeOf` collects them, before it adds what the key reads. */
interface Collected {
  readonly parts: Part[];
  readonly fields: string[][];
  readonly partsOf: Map<Element, number[]>;
  readonly external: Map<string, [StaticRead, number]>;
  moved: number;
  shallow: number;
  /** The number of elements with a reactive attribute. */
  reactive: number;
}

const shapes = new WeakMap<Element, Shape>();

/** The parts of the items built from `proto`, a list template at item depth `depth`. */
export function shapeOf(proto: Element, depth: number): Shape {
  let shape = shapes.get(proto);
  if (shape !== undefined) return shape;
  const found = collect(proto, depth);
  const keyAst = parse(data(proto).hsKey);
  const keyFields = keyAst === undefined ? null : fieldsOf(keyAst, depth);
  shape = {
    parts: found.parts,
    partsOf: found.partsOf,
    fields: found.fields,
    shallow: found.shallow,
    external: [...found.external.values()],
    moved: found.moved,
    all: found.parts.length === 0 ? 0 : 2 ** found.parts.length - 1,
    keyFields,
    // The bindings inside a condition that an `if` part shows are effects of their own, which read under the item's slot name,
    // and the items of a nested list are filed by their paths, which `relocate` gives them again when this item moves.
    whole:
      found.partsOf.size === found.reactive &&
      (keyAst === undefined || keyFields !== null) &&
      !found.parts.some(
        (part) =>
          part.kind === "each" || (part.kind === "if" && part.proto !== null && bound(part.proto)),
      ),
    masks: new Map(),
  };
  shapes.set(proto, shape);
  return shape;
}

/** The item fields `keyAst` reads, where it reads nothing but fields of the item at `depth`. */
function fieldsOf(keyAst: unknown, depth: number): string[] | null {
  const reads = staticReads(keyAst);
  return reads !== null && reads.every((read) => read.kind === "item" && read.depth === depth)
    ? reads.map((read) => (read as { field: string }).field)
    : null;
}

function collect(proto: Element, depth: number): Collected {
  const found: Collected = {
    parts: [],
    fields: [],
    partsOf: new Map(),
    external: new Map(),
    moved: 0,
    shallow: 0,
    reactive: 0,
  };
  sources(planOf(proto), (source) => {
    found.reactive++;
    const parts = partsFor(source, depth);
    // A row runs at most 30 parts, one bit each in a small integer.
    if (parts === null || found.parts.length + parts.length > 30) return;
    found.partsOf.set(
      source,
      parts.map(([part, reads]) => add(found, part, reads, depth)),
    );
  });
  return found;
}

/** Adds `part`, which makes `reads`, to `found`, and returns its index. */
function add(found: Collected, part: Part, reads: readonly StaticRead[], depth: number): number {
  const index = found.parts.push(part) - 1;
  if (part.kind === "each") found.shallow |= 1 << index;
  const fields: string[] = [];
  for (const read of reads) {
    if (read.kind === "item" && read.depth === depth) fields.push(read.field);
    else if (read.kind === "index" && read.depth === depth) found.moved |= 1 << index;
    else {
      const id =
        read.kind === "sig"
          ? `s ${read.path}`
          : read.kind === "item"
            ? `i ${read.depth} ${read.field}`
            : `x ${read.depth}`;
      const entry = found.external.get(id);
      if (entry === undefined) found.external.set(id, [read, 1 << index]);
      else entry[1] |= 1 << index;
    }
  }
  found.fields.push(fields);
  return index;
}

/** The statements that start when an element is registered, which keep its bindings effects of their own. */
const STARTED = ["hsOnLoad", "hsOnInterval", "hsOnTimeout", "hsOnVisible", "hsOnCustom"];

/**
 * The parts of the bindings of `source`, an element of a list template at
 * item depth `depth`, with the reads each can make. Null where it carries a
 * binding that stays an `Effect`: a statement, a nested list or condition,
 * or an expression whose reads are not known from its AST.
 */
function partsFor(source: Element, depth: number): [Part, StaticRead[]][] | null {
  const spec = specOf(source);
  if (spec.effects.length > 0 || spec.watches.length > 0) return null;
  const set = data(source);
  if (STARTED.some((key) => set[key] !== undefined)) return null;
  const list = listPart(source, spec, depth);
  if (list === null) return null;
  const conditions = conditionParts(source, spec, depth);
  if (conditions === null) return null;
  const control = bindPart(spec, depth);
  if (control === null) return null;
  return [...list, ...conditions, ...control];
}

/** The `each` part of `source`, a nested list over a field of this item, or null where the list is not one. */
function listPart(source: Element, spec: Spec, depth: number): [Part, StaticRead[]][] | null {
  if (spec.each === undefined) return [];
  // A rerun its items' keys ask for marks the part through `Row.lists`.
  const read = placeRead(spec.each[0]);
  if (
    read?.kind !== "item" ||
    read.depth !== depth ||
    spec.proto === null ||
    !(source instanceof HTMLTemplateElement)
  )
    return null;
  const part: Part = {
    kind: "each",
    name: "",
    ast: spec.each,
    value: null,
    selector: null,
    proto: spec.proto,
  };
  return [[part, [read]]];
}

/** The parts of the bindings of `spec` that write or show a condition, or null where one reads what no part can. */
function conditionParts(source: Element, spec: Spec, depth: number): [Part, StaticRead[]][] | null {
  const conditions: [Part["kind"], string, unknown][] = writes(spec);
  if (spec.cond !== undefined && source instanceof HTMLTemplateElement)
    conditions.push(["if", "", spec.cond]);
  const found: [Part, StaticRead[]][] = [];
  for (const [kind, name, ast] of conditions) {
    const reads = staticReads(ast);
    if (reads === null || reads.some((read) => read.kind !== "sig" && read.depth > depth))
      return null;
    const compared = CONDITIONS.has(kind) ? comparison(ast) : null;
    const otherReads = compared === null ? null : staticReads(compared[1]);
    const selects =
      otherReads !== null &&
      otherReads.every((read) => read.kind === "item" && read.depth === depth);
    const selector = selects ? { path: compared![0], other: compile(compared![1]) } : null;
    const part: Part = { kind, name, ast, value: compile(ast), selector, proto: spec.proto };
    found.push([part, selects ? otherReads : reads]);
  }
  return found;
}

/** The `bind` part of `spec`, or null where it binds a place no part can read. */
function bindPart(spec: Spec, depth: number): [Part, StaticRead[]][] | null {
  if (spec.bind === undefined) return [];
  const read = placeRead(spec.bind[0]);
  if (read === null || (read.kind === "item" && read.depth > depth)) return null;
  const part: Part = {
    kind: "bind",
    name: "",
    ast: spec.bind,
    value: null,
    selector: null,
    proto: null,
  };
  return [[part, [read]]];
}

/** The name a read outside a list's items is recorded under, for a list inside the items `outer`. */
export function readName(read: StaticRead, outer: readonly Slot[]): string {
  if (read.kind === "sig") return read.path;
  const slot = outer[read.depth]!;
  const name = slot.read ?? slot.path;
  return read.kind === "index" ? positionOf(name) : join(name, read.field);
}
