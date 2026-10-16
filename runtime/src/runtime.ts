import { type Context, evaluate, type Host, resolve, resolveRead, run, type Slot } from "./eval.ts";
import { morph } from "./morph.ts";
import { placeRead, type StaticRead, staticReads } from "./reads.ts";
import { readEventStream, type ServerEvent } from "./sse.ts";
import { DepIndex, type Effect, type Selector } from "./deps.ts";
import { ChangeLookup, type Changes, equal, join, overlaps, positionOf, type Reads, Store, type Values } from "./store.ts";

/** Events handled by one listener on the root, matched to the closest element with a handler. */
const BUBBLING = ["click", "dblclick", "input", "change", "submit", "keydown", "focusin", "focusout"] as const;
/** Events that do not bubble, caught in the capture phase and matched to the target only. */
const DIRECT = ["mouseenter", "mouseleave"] as const;

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
  "each",
  "if",
  "on-load",
  "on-key",
  ...BUBBLING.map((type) => `on-${type}`),
  ...DIRECT.map((type) => `on-${type}`),
];
const REACTIVE = ATTRIBUTES.map((name) => `[data-hs-${name}]`).join(",");

type Bind = [place: unknown, kind: string, value?: unknown];

/**
 * How long one task may spend taking over server-rendered items before it
 * yields, in milliseconds. A task over 50 ms counts as blocking the page.
 */
const ADOPT_BUDGET_MS = 30;

interface Instance {
  readonly element: Element;
  readonly slot: Slot;
  /** False for an item the server rendered, until the runtime registers its bindings. */
  scanned: boolean;
  /** The item's key as its key expression evaluated it, before a duplicate is told apart by position. */
  key: string;
  /** What evaluating this item's key read, which `watch` is registered under. */
  keyReads: Reads;
  /**
   * Reruns the list when something its key read changes. The key's reads are
   * registered under it rather than under the list's own binding, so a
   * render of the list does not record every item's key reads again.
   */
  watch: Effect | null;
  /**
   * Whether the key read nothing but the item's own fields, so another item
   * with the same value has the same key wherever it sits.
   */
  local: boolean;
  /** The item's value when the list last matched it, to tell a move from a change. */
  value: unknown;
  readonly row: Row;
}

/**
 * One binding of an element inside a list item, which the item's `Row` runs
 * in place of an `Effect` of its own. The reads it can make are known from
 * its AST, so the runtime neither records them as it runs nor registers each
 * item's copy of it in the `DepIndex`.
 */
interface Part {
  /** What it writes, or `if` for a condition's `<template>`, which it shows or removes. */
  readonly kind: "show" | "text" | "checked" | "class" | "attr" | "toggle" | "style" | "bind" | "if";
  /** The class, attribute, or CSS property it writes. */
  readonly name: string;
  /** The expression, or for `bind` the `Bind`. */
  readonly ast: unknown;
  /** For `["eq", ["sig", path], other]` where `other` reads only the item: the signal and `other`. */
  readonly selector: { readonly path: string; readonly other: unknown } | null;
  /** For `if`, the element its template holds. */
  readonly proto: Element | null;
}

/** What the items of one `<template>` share, worked out once from its element. */
interface Shape {
  readonly parts: readonly Part[];
  /** The parts of each element of the template whose bindings are all parts. */
  readonly partsOf: ReadonlyMap<Element, readonly number[]>;
  /** The item fields each part reads, `""` for the whole item. */
  readonly fields: readonly (readonly string[])[];
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

/** A list item's parts, and which of them the next render runs. */
interface Row {
  /** The item's root element. */
  readonly element: Element;
  readonly slot: Slot;
  readonly shape: Shape;
  readonly template: HTMLTemplateElement;
  /** The element each part writes, by part, once the item is mounted. */
  nodes: readonly (Element | undefined)[] | null;
  ctx: Context | null;
  /** The parts to run, one bit each. */
  dirty: number;
  queued: boolean;
  /** The key of the value each selector part compared, where it filed the row in its `Selection`. */
  keys: (string | undefined)[] | null;
}

/** The items whose selector part compared a value under each key, kept for one list. */
interface Selection {
  readonly path: string;
  /** The key of the signal's value when the selection last looked. */
  current: string | undefined;
  readonly byKey: Map<string, Row | Row[]>;
}

/** The bindings a list keeps for its items' reads outside them. */
interface ListParts {
  readonly list: Effect;
  readonly effects: readonly Effect[];
  /** By part, for the selector parts. */
  readonly selections: readonly (Selection | undefined)[];
}

/** The key a selector part files an item under when its value is not a JSON scalar, which every change of the signal reaches. */
const ANY = "*";
const NO_READS: Reads = new Map();

const STATE = Symbol("hypersig");

/**
 * What the runtime keeps about one element. It lives on the element, so it
 * goes when the element does, without a table that keeps its size after the
 * elements it held are gone.
 */
interface ElementState {
  /**
   * The template element whose `data-hs-*` attributes this element, part of
   * a list item or a condition, carries its bindings from. The server writes
   * those copies without the attributes, and a patch that morphs a clone
   * removes them.
   */
  source?: Element;
  scope?: readonly Slot[];
  effects?: Effect[];
  /** Whether its on-load and on-interval statements have started. */
  started?: boolean;
  /** Whether it is the root of a list item or a condition. */
  root?: boolean;
  /** For such a root, the elements with bindings inside it, outside its nested lists and conditions. */
  members?: readonly Element[];
  /** For the root of a list item, the binding that watches the item's key. */
  watch?: Effect;
  /** For the root of a list item, its parts. */
  row?: Row;
}

type Stateful = Element & { [STATE]?: ElementState };

function own(element: Element): ElementState | undefined {
  return (element as Stateful)[STATE];
}

function state(element: Element): ElementState {
  return ((element as Stateful)[STATE] ??= {});
}

/** Connects the signals on `<body>` and the `data-hs-*` attributes of `root`. */
export class Runtime implements Host {
  readonly store: Store;
  readonly #root: Element;
  readonly #index = new DepIndex();
  /** Bindings to run in the current or next render: new ones, and ones whose item moved. */
  #fresh: Effect[] = [];
  #ran = new Set<Effect>();
  /** The changes the current render is running for. */
  #changes: Changes = new Map();
  /** The selector the binding being run recorded through `#compare`. */
  #selector: Selector | null = null;
  #nextId = 0;
  readonly #lists = new WeakMap<Element, Map<string, Instance>>();
  /**
   * The slots of the rendered list items at each store path: one, or one per
   * list where several lists show the same signal. The bindings of an item
   * record their reads under its slot's name, so a change written at the
   * item's path is looked up here to reach them.
   */
  readonly #slots = new Map<string, Slot | Slot[]>();
  /** The root element of the item each slot belongs to. */
  readonly #owners = new WeakMap<Slot, Element>();
  #nextSlot = 0;
  /** Whether a patch has replaced elements since the last `#sweep`, which may have taken list items with them. */
  #patched = false;
  readonly #mounted = new WeakMap<Element, Element>();
  readonly #timers = new WeakMap<Element, Map<string, ReturnType<typeof setTimeout>>>();
  /**
   * Registers the bindings of a server-rendered item that is still waiting
   * for its turn, by its root. The item already shows its values, so it
   * waits for a later task, unless an event reaches it first.
   */
  readonly #waiting = new Map<Element, () => void>();
  #draining = false;
  /**
   * Items and conditions taken out of the document, elements a patch took
   * out, and bindings found outside it, whose registrations `#sweep` removes
   * after the next paint.
   * The runtime never puts a removed element back, so until then they only
   * hold memory, and a render that reaches them skips them.
   */
  #removed: Element[] = [];
  #stale = new Set<Effect>();
  #sweeping = false;
  /** The rows of the list items, by slot name, which the changes `#named` names reach. */
  readonly #rows = new Map<string, Row>();
  /** The rows with parts to run, in the order they were marked. */
  #queue: Row[] = [];
  readonly #listParts = new WeakMap<HTMLTemplateElement, ListParts>();

  constructor(root: Element, initial: Values) {
    this.#root = root;
    this.store = new Store(initial);
    this.store.subscribe((changes) => this.#render(changes));
    for (const type of BUBBLING) {
      root.addEventListener(type, (event) => this.#onEvent(type, event));
    }
    for (const type of DIRECT) {
      root.addEventListener(type, (event) => this.#onDirect(type, event), { capture: true });
    }
    this.#computed(root);
    this.scan(root, []);
    this.#render(new Map());
  }

  /** Registers the computed signals on `root`, before any element, so they run first. */
  #computed(root: Element): void {
    const computed = parse(root.getAttribute("data-hs-computed"));
    if (!Array.isArray(computed)) return;
    const ctx: Context = { store: this.store, scope: [] };
    const effects = (computed as [string, unknown][]).map(([path, expr]): Effect => ({
      id: this.#nextId++,
      element: root,
      run: () => {
        const value = evaluate(expr, ctx);
        this.store.untracked(() => this.store.set(path, value));
      },
      deps: null,
      nodes: new Map(),
      selector: null,
    }));
    this.#fresh.push(...effects);
  }

  /**
   * Registers the reactive attributes of `root` and every element below it,
   * apart from the elements of list items and conditions, which `#mount` registers.
   */
  scan(root: Element, scope: readonly Slot[]): void {
    const elements = root.matches(REACTIVE) ? [root] : [];
    elements.push(...root.querySelectorAll(REACTIVE));
    for (const element of elements) {
      if (own(element)?.source !== undefined) continue;
      this.#register(element, own(element)?.scope ?? scope);
    }
  }

  /**
   * Registers the bindings of `root`, an item or condition built from
   * `proto` by cloning it or by the server, finding its elements by their
   * position in `proto`. The bindings of a list item that are parts of its
   * `row` are left to the row, which runs them all in the next render.
   */
  #mount(root: Element, proto: Element, scope: readonly Slot[], row?: Row): void {
    const members: Element[] = [];
    const nodes = row === undefined ? [] : new Array<Element | undefined>(row.shape.parts.length);
    locate(planOf(proto), root, (element, source, reactive) => {
      state(element).source = source;
      if (source.hasAttribute("data-hs-ref") && !element.hasAttribute("data-hs-ref")) {
        element.setAttribute("data-hs-ref", source.getAttribute("data-hs-ref")!);
      }
      if (!reactive) return;
      members.push(element);
      const parts = row?.shape.partsOf.get(source);
      if (parts === undefined) this.#register(element, scope);
      else for (const part of parts) nodes[part] = element;
    });
    state(root).members = members;
    if (row === undefined) return;
    // `#scopeOf` finds an item's scope here for an event on an element whose bindings are all parts.
    state(root).scope = scope;
    row.nodes = nodes;
    row.ctx = { store: this.store, scope };
    this.#mark(row, row.shape.all);
  }

  #register(element: Element, scope: readonly Slot[]): void {
    const bound = state(element);
    bound.scope = scope;
    for (const old of bound.effects ?? []) this.#index.remove(old);
    const effects = this.#effectsFor(element, scope);
    bound.effects = effects;
    this.#fresh.push(...effects);
    if (bound.started) return;
    bound.started = true;
    const load = this.#attr(element, "data-hs-on-load");
    if (load !== null) this.#runAttribute(load, element);
    const interval = parse(this.#attr(element, "data-hs-on-interval"));
    if (Array.isArray(interval)) this.#interval(element, interval[0] as number, interval[1]);
  }

  /** The `data-hs-*` attribute `name` of `element`, read from its template element where it has one. */
  #attr(element: Element, name: string): string | null {
    return (own(element)?.source ?? element).getAttribute(name);
  }

  /** The closest element from `element` up whose attribute `name` is set, with its value. */
  #closest(element: Element, name: string): [Element, string] | null {
    for (let at: Element | null = element; at !== null; at = at.parentElement) {
      const value = this.#attr(at, name);
      if (value !== null) return this.#root.contains(at) ? [at, value] : null;
    }
    return null;
  }

  /** Calls an action with `args` and applies the patches it streams back. */
  async call(
    method: string,
    path: string,
    args: unknown,
    indicator: string | null,
    error: string | null,
  ): Promise<void> {
    const url = new URL(path, document.baseURI);
    const body = JSON.stringify(args ?? null);
    const init: RequestInit = { method, headers: { accept: "text/event-stream" } };
    if (method === "GET") {
      if (args !== null && args !== undefined) url.searchParams.set("args", body);
    } else {
      init.headers = { accept: "text/event-stream", "content-type": "application/json" };
      init.body = body;
    }
    if (indicator !== null) this.store.set(indicator, true);
    try {
      const response = await fetch(url, init);
      if (!response.ok || response.body === null) {
        const detail = await response.text();
        this.#fail(method, path, `${response.status}: ${detail}`, error, response.status);
        return;
      }
      if (error !== null) this.store.set(error, "");
      await readEventStream(response.body, (event) => this.apply(event));
    } catch (cause) {
      this.#fail(method, path, String(cause), error, 0);
    } finally {
      if (indicator !== null) this.store.set(indicator, false);
    }
  }

  /** Runs a DOM method on the element a node ref names, after the next render. */
  dom(op: string, name: string, origin: Element | undefined): void {
    queueMicrotask(() =>
      queueMicrotask(() => {
        let target = this.#findRef(name, origin);
        if (target === null && this.#waiting.size > 0) {
          for (const mount of this.#waiting.values()) mount();
          target = this.#findRef(name, origin);
        }
        if (target === null) {
          console.warn(`hypersig: no element with node ref "${name}"`);
          return;
        }
        if (op === "focus" && target instanceof HTMLElement) target.focus();
        else if (op === "blur" && target instanceof HTMLElement) target.blur();
        else if (op === "select" && (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement)) target.select();
        else if (op === "scroll") target.scrollIntoView({ block: "nearest" });
      }),
    );
  }

  /** Runs `action` once no call with the same key on `origin` has arrived for `ms`. */
  debounce(origin: Element | undefined, key: string, ms: number, action: () => void): void {
    const owner = origin ?? this.#root;
    let timers = this.#timers.get(owner);
    if (timers === undefined) {
      timers = new Map();
      this.#timers.set(owner, timers);
    }
    clearTimeout(timers.get(key));
    timers.set(
      key,
      setTimeout(() => {
        timers.delete(key);
        if (owner.isConnected) action();
      }, ms),
    );
  }

  /** Applies one server event. */
  apply(event: ServerEvent): void {
    switch (event.event) {
      case "hs-signals":
        this.store.merge(JSON.parse(event.data) as Values);
        return;
      case "hs-element": {
        const { html } = JSON.parse(event.data) as { html: string };
        const template = document.createElement("template");
        template.innerHTML = html;
        const next = template.content.firstElementChild;
        if (next === null || next.id === "") {
          console.error("hypersig: an element patch needs a root element with an id");
          return;
        }
        const target = document.getElementById(next.id);
        if (target === null) {
          console.error(`hypersig: no element with id "${next.id}" to patch`);
          return;
        }
        const morphed = morph(target, next, this.#removed);
        this.scan(morphed, this.#scopeOf(morphed.parentElement));
        this.#render(new Map());
        this.#patched = true;
        this.#sweepLater();
        return;
      }
      default:
        console.warn(`hypersig: ignored server event "${event.event}"`);
    }
  }

  #context(element: Element, scope: readonly Slot[]): Context {
    return { store: this.store, scope, origin: element };
  }

  #effectsFor(element: Element, scope: readonly Slot[]): Effect[] {
    const source = own(element)?.source;
    const spec = source === undefined ? readSpec(element) : specOf(source);
    const ctx = this.#context(element, scope);
    const value = (ast: unknown) => evaluate(ast, ctx);
    const truth = (ast: unknown): boolean => {
      const compared = comparison(ast);
      return compared === null ? Boolean(value(ast)) : this.#compare(compared[0], compared[1], ctx);
    };
    const effects: Effect[] = [];
    const add = (run: () => void): Effect => {
      const effect: Effect = { id: this.#nextId++, element, run, deps: null, nodes: new Map(), selector: null };
      effects.push(effect);
      return effect;
    };

    for (const [kind, name, ast] of writes(spec)) {
      if (!writable(element, kind)) continue;
      add(() => write(element, kind, name, CONDITIONS.has(kind) ? truth(ast) : value(ast)));
    }
    const bind = spec.bind;
    if (bind !== undefined) add(() => this.#writeControl(element, bind, ctx));

    for (const stmt of spec.effects) add(() => this.#runTracked(stmt, ctx));
    for (const [place, stmt] of spec.watches) {
      let first = true;
      add(() => {
        this.store.getAs(resolve(place, ctx), resolveRead(place, ctx));
        if (!first) this.#run(stmt, ctx);
        first = false;
      });
    }

    if (element instanceof HTMLTemplateElement) {
      const { proto, each, cond } = spec;
      if (each !== undefined) {
        const list: Effect = add(() => this.#renderList(element, proto, each[0], each[1], scope, list));
      }
      if (cond !== undefined) add(() => this.#renderIf(element, proto, Boolean(value(cond)), scope));
    }
    return effects;
  }

  #renderList(template: HTMLTemplateElement, proto: Element | null, place: unknown, depth: number, scope: readonly Slot[], list: Effect): void {
    const parent = template.parentNode;
    if (proto === null || parent === null) return;
    const listCtx: Context = { store: this.store, scope };
    const path = resolve(place, listCtx);
    const read = resolveRead(place, listCtx);
    const value = this.store.getShallowAs(path, read);
    const items = Array.isArray(value) ? value : [];
    const keyAst = parse(proto.getAttribute("data-hs-key"));
    const shape = shapeOf(proto, depth);
    this.#partsOf(template, shape, scope.slice(0, depth), list);
    const known = this.#lists.get(template);
    const created = new Set<Instance>();
    /** The previous items by position and, built on first use, by value, to find one whose key holds for a value. */
    let placed: Instance[] | null = null;
    let byValue: Map<unknown, Instance> | null = null;
    const sameValue = (previous: ReadonlyMap<string, Instance>, index: number): Instance | undefined => {
      placed ??= [...previous.values()];
      const item = items[index];
      const at = placed[index];
      if (at !== undefined && at.local && Object.is(at.value, item)) return at;
      if (byValue === null) {
        byValue = new Map();
        for (const instance of placed) if (instance.local && !byValue.has(instance.value)) byValue.set(instance.value, instance);
      }
      // `match` gives an instance its new value as it goes, so the one found may no longer hold `item`.
      const found = byValue.get(item);
      return found !== undefined && found.local && Object.is(found.value, item) ? found : undefined;
    };
    /** The key `match` filed its last item under. */
    let matched = "";
    /** The slot and context a key that reads only the item is evaluated in, moved to each item in turn. */
    const keySlot: Slot = { path: "" };
    let keyCtx: Context | null = null;
    const match = (index: number, previous: ReadonlyMap<string, Instance>, next: Map<string, Instance>, appending = false): Instance => {
      const itemPath = join(path, String(index));
      const itemAt = read === path ? itemPath : join(read, String(index));
      let key = `#${index}`;
      let keyReads: Reads = new Map();
      /** The name `keyReads` read the item under, where they read nothing else. */
      let named: string | null = null;
      if (keyAst !== undefined) {
        // An appended item has no previous item at its position, and is rarely one moved from elsewhere in the list.
        const same = appending ? undefined : sameValue(previous, index);
        if (same !== undefined) {
          key = same.key;
          keyReads = same.keyReads;
          named = same.slot.read!;
        } else if (shape.keyFields !== null) {
          // A key that reads only the item's fields is watched through the item's row, so its reads are not recorded.
          keyCtx ??= { store: this.store, scope: [...scope.slice(0, depth), keySlot] };
          keySlot.path = itemPath;
          if (itemAt === itemPath) delete keySlot.read;
          else keySlot.read = itemAt;
          key = JSON.stringify(this.store.untracked(() => evaluate(keyAst, keyCtx!)));
          keyReads = NO_READS;
          named = itemAt;
        } else {
          // The key reads the item by its path, or by a name below the enclosing item's, so the list reruns when they change.
          const keySlot: Slot = itemAt === itemPath ? { path: itemPath } : { path: itemPath, read: itemAt };
          keyReads = this.store.track(() => {
            key = JSON.stringify(evaluate(keyAst, { store: this.store, scope: [...scope.slice(0, depth), keySlot] }));
          });
          const local = [...keyReads.keys()].every((at) => at !== positionOf(itemAt) && (at === itemAt || at.startsWith(`${itemAt}.`)));
          if (local) named = itemAt;
        }
      }
      const local = named !== null;
      const raw = key;
      if (next.has(key)) {
        console.warn(`hypersig: duplicate key ${key} in a list; matching it by position`);
        key = `${key}#${index}`;
      }
      let instance = previous.get(key);
      if (instance === undefined || !instance.element.isConnected) {
        const slot: Slot = { path: itemPath, read: `@${this.#nextSlot++}` };
        this.#file(itemPath, slot);
        // A key that read only the item reads it under the item's name, so the item can move without registering it again.
        if (named !== null) keyReads = rebased(keyReads, named, slot.read!);
        const element = proto.cloneNode(true) as Element;
        instance = { element, slot, scanned: true, key: raw, keyReads, watch: null, local, value: items[index], row: this.#row(element, slot, shape, template) };
        this.#owners.set(slot, instance.element);
        state(instance.element).root = true;
        created.add(instance);
        this.#watchKey(instance, list);
      } else {
        instance.key = raw;
        if (named !== null && named !== instance.slot.read) keyReads = rebased(keyReads, named, instance.slot.read!);
        if (instance.keyReads !== keyReads || instance.watch === null) {
          instance.keyReads = keyReads;
          this.#watchKey(instance, list);
        }
        instance.local = local;
        const changed = !Object.is(instance.value, items[index]);
        const name = instance.slot.read!;
        if (instance.slot.path !== itemPath) {
          const from = instance.slot.path;
          this.#place(instance.slot, itemPath);
          if (instance.scanned && !shape.whole) {
            this.#relocate(instance.element, from, itemPath);
            if (changed) this.#index.within(name, this.#again);
            else this.#index.at(positionOf(name), this.#again);
          }
          this.#mark(instance.row, changed ? shape.all : shape.moved);
        } else if (changed) {
          if (instance.scanned && !shape.whole) this.#index.within(name, this.#again);
          this.#mark(instance.row, shape.all);
        }
        instance.value = items[index];
      }
      next.set(key, instance);
      matched = key;
      return instance;
    };
    const register = (order: readonly Instance[]): void => {
      for (const instance of order) {
        if (created.has(instance)) this.#mount(instance.element, proto, [...scope.slice(0, depth), instance.slot], instance.row);
        else if (!instance.scanned && !this.#waiting.has(instance.element)) this.#wait(instance, proto, [...scope.slice(0, depth), instance.slot]);
      }
    };

    if (known !== undefined && appended(this.#changes, path, known, items.length)) {
      let anchor: Node = template;
      for (const instance of known.values()) anchor = instance.element;
      const next = known;
      const added: Instance[] = [];
      for (let index = known.size; index < items.length; index++) added.push(match(index, known, next, true));
      const fragment = document.createDocumentFragment();
      for (const instance of added) fragment.append(instance.element);
      parent.insertBefore(fragment, anchor.nextSibling);
      register(added);
      this.#lists.set(template, next);
      return;
    }

    const touched = known === undefined ? null : replaced(this.#changes, path, known, items.length);
    if (known !== undefined && touched !== null) {
      const was = [...known.values()];
      const wasKeys = [...known.keys()];
      const keys = [...wasKeys];
      const order = [...was];
      const end = was.at(-1)?.element.nextSibling ?? null;
      const filed = new Map<string, Instance>();
      for (let index = 0; index < was.length; index++) if (!touched.has(index)) filed.set(keys[index]!, was[index]!);
      const at = [...touched].sort((a, b) => a - b);
      for (const index of at) {
        order[index] = match(index, known, filed);
        keys[index] = matched;
      }
      const next = new Map<string, Instance>();
      order.forEach((instance, index) => next.set(keys[index]!, instance));
      for (const index of at) if (next.get(wasKeys[index]!) !== was[index]) this.#unmountItem(was[index]!);
      for (let i = at.length - 1; i >= 0; i--) {
        const index = at[i]!;
        const element = order[index]!.element;
        const before = index + 1 < order.length ? order[index + 1]!.element : end;
        if (element.parentNode !== parent || element.nextSibling !== before) parent.insertBefore(element, before);
      }
      register(at.map((index) => order[index]!));
      this.#lists.set(template, next);
      return;
    }

    const previous = known ?? this.#adoptItems(template, shape);
    const next = new Map<string, Instance>();
    const order = items.map((_, index) => match(index, previous, next));
    for (const [key, instance] of previous) {
      if (next.get(key) !== instance) this.#unmountItem(instance);
    }
    arrange(parent, template, previous, order);
    register(order);
    this.#lists.set(template, next);
  }

  /** Registers the bindings of the server-rendered `instance` in a later task. */
  #wait(instance: Instance, proto: Element, scope: readonly Slot[]): void {
    this.#waiting.set(instance.element, () => {
      this.#waiting.delete(instance.element);
      if (instance.scanned || !instance.element.isConnected) return;
      instance.scanned = true;
      this.#mount(instance.element, proto, scope, instance.row);
    });
    if (this.#draining) return;
    this.#draining = true;
    setTimeout(() => this.#drain(), 0);
  }

  /** Registers waiting items until the budget runs out, then yields to the browser and continues. */
  #drain(): void {
    const end = performance.now() + ADOPT_BUDGET_MS;
    while (this.#waiting.size > 0 && performance.now() < end) {
      let count = 0;
      for (const mount of this.#waiting.values()) {
        mount();
        if (++count === 50) break;
      }
      this.#render(new Map());
    }
    if (this.#waiting.size > 0) setTimeout(() => this.#drain(), 0);
    else this.#draining = false;
  }

  /**
   * Registers the waiting items that contain `element`, outermost first, so
   * an event finds their bindings. Only the bindings of nested lists and
   * conditions run here, to reach the items inside them. The rest run in the
   * next render, so a binding does not write a control while the event that
   * changes it is still being dispatched.
   */
  #adoptAround(element: Element): void {
    while (this.#waiting.size > 0) {
      let outermost: Element | undefined;
      for (let at: Element | null = element; at !== null; at = at.parentElement) {
        if (this.#waiting.has(at)) outermost = at;
      }
      if (outermost === undefined) return;
      this.#waiting.get(outermost)!();
      for (const member of own(outermost)?.members ?? []) {
        if (member instanceof HTMLTemplateElement) for (const effect of own(member)?.effects ?? []) this.#runEffect(effect);
      }
      const row = own(outermost)?.row;
      row?.nodes?.forEach((node, index) => {
        if (row.shape.parts[index]!.kind === "if" && node !== undefined) this.store.untracked(() => this.#runPart(row, index));
      });
    }
  }

  /** Takes over the items the server rendered right after `template`, keyed as it keyed them. */
  #adoptItems(template: HTMLTemplateElement, shape: Shape): Map<string, Instance> {
    const adopted = new Map<string, Instance>();
    for (let at = template.nextElementSibling; at !== null && at.hasAttribute("data-hs-ssr"); at = at.nextElementSibling) {
      if (own(at)?.root === true) break;
      state(at).root = true;
      const key = at.getAttribute("data-hs-ssr")!;
      const slot: Slot = { path: "", read: `@${this.#nextSlot++}` };
      this.#owners.set(slot, at);
      adopted.set(key, { element: at, slot, scanned: false, key, keyReads: new Map(), watch: null, local: false, value: undefined, row: this.#row(at, slot, shape, template) });
    }
    return adopted;
  }

  #renderIf(template: HTMLTemplateElement, proto: Element | null, visible: boolean, scope: readonly Slot[]): void {
    let current = this.#mounted.get(template);
    const rendered = template.nextElementSibling;
    if (current === undefined && rendered?.getAttribute("data-hs-ssr") === "" && own(rendered)?.root !== true) {
      state(rendered).root = true;
      if (visible && proto !== null) {
        this.#mounted.set(template, rendered);
        this.#mount(rendered, proto, scope);
        return;
      }
      this.#unmount(rendered);
    }
    current = this.#mounted.get(template);
    if (visible && (current === undefined || !current.isConnected)) {
      if (proto === null || template.parentNode === null) return;
      const element = proto.cloneNode(true) as Element;
      this.#mounted.set(template, element);
      state(element).root = true;
      template.after(element);
      this.#mount(element, proto, scope);
    } else if (!visible && current !== undefined) {
      this.#unmount(current);
      this.#mounted.delete(template);
    }
  }

  #writeControl(element: Element, [place, kind, choice]: Bind, ctx: Context): void {
    const current = this.store.getAs(resolve(place, ctx), resolveRead(place, ctx));
    if (!(element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement || element instanceof HTMLSelectElement)) return;
    switch (kind) {
      case "bool":
        if (element instanceof HTMLInputElement && element.checked !== current) element.checked = Boolean(current);
        return;
      case "radio":
        if (element instanceof HTMLInputElement) element.checked = equal(current, choice);
        return;
      case "int":
      case "float":
        if (Number(element.value) !== current || element.value === "") element.value = String(current ?? "");
        return;
      case "json": {
        const next = JSON.stringify(current);
        if (element.value !== next) element.value = next;
        return;
      }
      default: {
        const next = current === undefined || current === null ? "" : String(current);
        if (element.value !== next) element.value = next;
      }
    }
  }

  #readControl(element: Element, type: string): void {
    const bind = parse(this.#attr(element, "data-hs-bind")) as Bind | undefined;
    if (bind === undefined) return;
    if (!(element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement || element instanceof HTMLSelectElement)) return;
    const [place, kind, choice] = bind;
    const path = resolve(place, this.#context(element, this.#scopeOf(element)));
    const raw = element.value;
    switch (kind) {
      case "bool":
        if (type === "change" && element instanceof HTMLInputElement) this.store.set(path, element.checked);
        return;
      case "radio":
        if (type === "change" && element instanceof HTMLInputElement && element.checked) this.store.set(path, choice);
        return;
      case "int":
      case "float": {
        const number = Number(raw);
        if (raw.trim() === "" || !Number.isFinite(number)) return;
        this.store.set(path, kind === "int" ? Math.trunc(number) : number);
        return;
      }
      case "json":
        if (type === "change") this.store.set(path, JSON.parse(raw));
        return;
      default:
        if (type === "input" || element instanceof HTMLSelectElement) this.store.set(path, raw);
    }
  }

  /** Runs the bindings that `changes` affects, oldest first, then every binding that has not run yet. */
  #render(changes: Changes): void {
    this.#ran = new Set();
    this.#changes = changes;
    const named = this.#named(changes);
    const found = this.#index.affected(named);
    this.#touchRows(named);
    if (changes.size > 0) {
      let lookup: ChangeLookup | null = null;
      this.#index.selected(
        (path) => (lookup ??= new ChangeLookup(changes)).affects(path, true),
        (path) => keyOf(this.store.untracked(() => this.store.get(path))),
        found,
      );
    }
    const affected = [...found].sort((a, b) => a.id - b.id);
    for (const effect of affected) this.#runEffect(effect);
    while (this.#fresh.length > 0 || this.#queue.length > 0) {
      while (this.#fresh.length > 0) {
        const batch = this.#fresh;
        this.#fresh = [];
        for (const effect of batch) this.#runEffect(effect);
      }
      this.#flushRows();
    }
    // The bindings that ran hold their elements, which may have left the document.
    this.#ran = new Set();
    this.#changes = new Map();
  }

  #runEffect(effect: Effect): void {
    if (this.#ran.has(effect)) return;
    this.#ran.add(effect);
    if (!effect.element.isConnected) {
      this.#stale.add(effect);
      this.#sweepLater();
      return;
    }
    this.#selector = null;
    const reads = this.store.track(() => {
      try {
        effect.run();
      } catch (error) {
        console.error("hypersig:", error);
      }
    });
    this.#index.set(effect, reads);
    this.#index.select(effect, this.#selector);
    this.#selector = null;
  }

  /**
   * Evaluates `["eq", ["sig", path], other]` for the binding being run. Where
   * `other` is a JSON scalar that does not read `path`, it records a selector
   * in place of reading `path`, so a change there runs only the bindings
   * whose result it can flip.
   */
  #compare(path: string, other: unknown, ctx: Context): boolean {
    let value: unknown;
    const reads = this.store.track(() => {
      value = evaluate(other, ctx);
    });
    this.store.replay(reads);
    const scalar = value === null || typeof value === "string" || typeof value === "number" || typeof value === "boolean";
    if (!scalar || [...reads.keys()].some((read) => overlaps(read, path))) return equal(this.store.get(path), value);
    const current = this.store.untracked(() => this.store.get(path));
    this.#selector = { path, key: keyOf(value), current: keyOf(current) };
    return equal(current, value);
  }

  /**
   * `changes` with each change written inside a rendered list item added
   * again under the item's slot name, where the bindings inside the item
   * recorded their reads. A change under a list that `changes` replaces is
   * left to that list, which compares its items when it renders.
   */
  #named(changes: Changes): Changes {
    if (this.#slots.size === 0) return changes;
    let named: Map<string, boolean> | null = null;
    for (const [path, inside] of changes) {
      for (let dot = path.indexOf("."); ; dot = path.indexOf(".", dot + 1)) {
        const prefix = dot === -1 ? path : path.slice(0, dot);
        const filed = this.#slots.get(prefix);
        if (filed !== undefined) {
          if (changes.has(prefix.slice(0, prefix.lastIndexOf(".")))) break;
          named ??= new Map(changes);
          for (const slot of Array.isArray(filed) ? filed : [filed]) {
            const name = slot.read! + path.slice(prefix.length);
            named.set(name, named.get(name) === true || inside);
          }
        }
        if (dot === -1) break;
      }
    }
    return named ?? changes;
  }

  /** Makes the row of a list item whose root is `element`, filed under its slot's name. */
  #row(element: Element, slot: Slot, shape: Shape, template: HTMLTemplateElement): Row {
    const row: Row = { element, slot, shape, template, nodes: null, ctx: null, dirty: 0, queued: false, keys: null };
    this.#rows.set(slot.read!, row);
    state(element).row = row;
    return row;
  }

  /**
   * The bindings of the list at `template` for the reads its items' parts make
   * outside the items, made again when the list's own binding is.
   */
  #partsOf(template: HTMLTemplateElement, shape: Shape, outer: readonly Slot[], list: Effect): ListParts {
    const existing = this.#listParts.get(template);
    if (existing !== undefined && existing.list === list) return existing;
    this.#dropParts(template);
    if (existing !== undefined) {
      // The rows filed themselves in the selections being replaced, so they run every part again to file themselves in the new ones.
      for (const instance of this.#lists.get(template)?.values() ?? []) {
        instance.row.keys = null;
        this.#mark(instance.row, shape.all);
      }
    }
    const effects: Effect[] = [];
    const selections: (Selection | undefined)[] = [];
    const watch = (reads: Reads, changed: () => void): void => {
      let started = false;
      effects.push({
        id: this.#nextId++,
        element: template,
        run: () => {
          this.store.replay(reads);
          if (started) changed();
          started = true;
        },
        deps: null,
        nodes: new Map(),
        selector: null,
      });
    };
    for (const [read, mask] of shape.external) {
      const name = readName(read, outer);
      watch(new Map([[name, true]]), () => {
        for (const instance of this.#lists.get(template)?.values() ?? []) this.#mark(instance.row, mask);
      });
    }
    shape.parts.forEach((part, index) => {
      if (part.selector === null) return;
      const selection: Selection = { path: part.selector.path, current: undefined, byKey: new Map() };
      selections[index] = selection;
      const now = (): string => keyOf(this.store.untracked(() => this.store.get(selection.path)));
      selection.current = now();
      watch(new Map([[selection.path, true]]), () => {
        const was = selection.current!;
        const key = now();
        selection.current = key;
        const reach = (filed: Row | Row[] | undefined): void => {
          for (const row of filed === undefined ? [] : Array.isArray(filed) ? filed : [filed]) this.#mark(row, 1 << index);
        };
        reach(selection.byKey.get(ANY));
        if (key === was) return;
        reach(selection.byKey.get(was));
        reach(selection.byKey.get(key));
      });
    });
    const parts: ListParts = { list, effects, selections };
    this.#listParts.set(template, parts);
    this.#fresh.push(...effects);
    return parts;
  }

  #dropParts(template: HTMLTemplateElement): void {
    const parts = this.#listParts.get(template);
    if (parts === undefined) return;
    for (const effect of parts.effects) this.#index.remove(effect);
    this.#listParts.delete(template);
  }

  #dropRow(row: Row): void {
    if (this.#rows.get(row.slot.read!) === row) this.#rows.delete(row.slot.read!);
    const keys = row.keys;
    if (keys === null) return;
    const selections = this.#listParts.get(row.template)?.selections ?? [];
    keys.forEach((key, index) => {
      const selection = selections[index];
      if (key !== undefined && selection !== undefined) unfileRow(selection, key, row);
    });
  }

  /** Has the next render run the parts of `row` in `mask`. */
  #mark(row: Row, mask: number): void {
    if (mask === 0) return;
    row.dirty |= mask;
    if (row.queued) return;
    row.queued = true;
    this.#queue.push(row);
  }

  /** Marks the parts of each row that a change `#named` named under the row's slot can reach. */
  #touchRows(named: Changes): void {
    if (this.#rows.size === 0) return;
    for (const [path, inside] of named) {
      if (!path.startsWith("@")) continue;
      const dot = path.indexOf(".");
      const row = this.#rows.get(dot === -1 ? path : path.slice(0, dot));
      if (row === undefined) continue;
      const rest = dot === -1 ? "" : path.slice(dot + 1);
      const { shape } = row;
      const list = this.#listParts.get(row.template)?.list;
      if (list !== undefined && shape.keyFields !== null && shape.keyFields.some((field) => reaches(field, rest, inside))) this.#again(list);
      this.#mark(row, maskFor(shape, rest, inside));
    }
  }

  /** Runs the marked parts of the queued rows. Their reads are known already, so none are recorded. */
  #flushRows(): void {
    this.store.untracked(() => {
      while (this.#queue.length > 0) {
        const batch = this.#queue;
        this.#queue = [];
        for (const row of batch) {
          const mask = row.dirty;
          row.dirty = 0;
          row.queued = false;
          if (row.ctx === null || !row.element.isConnected) continue;
          for (let index = 0; mask >>> index !== 0; index++) {
            if ((mask & (1 << index)) === 0) continue;
            try {
              this.#runPart(row, index);
            } catch (error) {
              console.error("hypersig:", error);
            }
          }
        }
      }
    });
  }

  #runPart(row: Row, index: number): void {
    const element = row.nodes![index];
    if (element === undefined) return;
    const part = row.shape.parts[index]!;
    if (part.kind === "bind") this.#writeControl(element, part.ast as Bind, row.ctx!);
    else if (part.kind === "if") this.#renderIf(element as HTMLTemplateElement, part.proto, this.#holds(row, index), row.ctx!.scope);
    else write(element, part.kind, part.name, CONDITIONS.has(part.kind) ? this.#holds(row, index) : evaluate(part.ast, row.ctx!));
  }

  /** Whether the condition of a show, class, or toggle part holds, filing a selector part's row under the value it compared. */
  #holds(row: Row, index: number): boolean {
    const part = row.shape.parts[index]!;
    const ctx = row.ctx!;
    if (part.selector === null) return Boolean(evaluate(part.ast, ctx));
    const other = evaluate(part.selector.other, ctx);
    const scalar = other === null || typeof other === "string" || typeof other === "number" || typeof other === "boolean";
    const key = scalar ? keyOf(other) : ANY;
    const keys = (row.keys ??= []);
    const selection = this.#listParts.get(row.template)?.selections[index];
    if (selection !== undefined && keys[index] !== key) {
      if (keys[index] !== undefined) unfileRow(selection, keys[index], row);
      const filed = selection.byKey.get(key);
      selection.byKey.set(key, filed === undefined ? row : Array.isArray(filed) ? [...filed, row] : [filed, row]);
      keys[index] = key;
    }
    return equal(this.store.get(part.selector.path), other);
  }

  /** Registers the reads of `instance`'s key under its watch, which reruns `list` when they change. */
  #watchKey(instance: Instance, list: Effect): void {
    if (instance.keyReads.size === 0 && instance.watch === null) return;
    if (instance.watch === null) {
      const watch: Effect = {
        id: this.#nextId++,
        element: instance.element,
        run: () => {
          this.store.replay(instance.keyReads);
          this.#again(list);
        },
        deps: null,
        nodes: new Map(),
        selector: null,
      };
      instance.watch = watch;
      state(instance.element).watch = watch;
    }
    this.#index.set(instance.watch, instance.keyReads);
  }

  /** Queues `effect` to run again in this render. */
  readonly #again = (effect: Effect): void => {
    this.#ran.delete(effect);
    this.#fresh.push(effect);
  };

  /** Files `slot` under `path`, where its item now sits. */
  #place(slot: Slot, path: string): void {
    this.#unfile(slot);
    slot.path = path;
    this.#file(path, slot);
  }

  #file(path: string, slot: Slot): void {
    const filed = this.#slots.get(path);
    if (filed === undefined) this.#slots.set(path, slot);
    else if (Array.isArray(filed)) filed.push(slot);
    else if (filed !== slot) this.#slots.set(path, [filed, slot]);
  }

  /** Takes `slot` out of `#slots`, where it is filed under its path. */
  #unfile(slot: Slot): void {
    const filed = this.#slots.get(slot.path);
    if (filed === slot) {
      this.#slots.delete(slot.path);
    } else if (Array.isArray(filed)) {
      const rest = filed.filter((other) => other !== slot);
      if (rest.length === 0) this.#slots.delete(slot.path);
      else this.#slots.set(slot.path, rest.length === 1 ? rest[0]! : rest);
    }
  }

  /** Gives the items of the lists inside `root`, whose item moved from `from` to `to`, their new paths. */
  #relocate(root: Element, from: string, to: string): void {
    const prefix = `${from}.`;
    for (const element of own(root)?.members ?? []) {
      if (!(element instanceof HTMLTemplateElement)) continue;
      for (const instance of this.#lists.get(element)?.values() ?? []) {
        if (!instance.slot.path.startsWith(prefix)) continue;
        this.#place(instance.slot, to + instance.slot.path.slice(from.length));
        this.#relocate(instance.element, from, to);
      }
      const mounted = this.#mounted.get(element);
      if (mounted !== undefined) this.#relocate(mounted, from, to);
    }
  }

  /** Removes a list item, and its slot from `#slots`. */
  #unmountItem(instance: Instance): void {
    this.#unfile(instance.slot);
    this.#unmount(instance.element);
  }

  /** Removes `root` from the document, and the bindings inside it after the next paint. */
  #unmount(root: Element): void {
    root.remove();
    this.#removed.push(root);
    this.#sweepLater();
  }

  #sweepLater(): void {
    if (this.#sweeping) return;
    this.#sweeping = true;
    // A frame callback runs right before a paint, so the timer it sets runs after one. A hidden page paints nothing.
    const after = (): void => void setTimeout(() => this.#sweep(), 0);
    if (document.hidden) after();
    else requestAnimationFrame(after);
  }

  /** Forgets the bindings of the removed items and conditions, and of the bindings found outside the document. */
  #sweep(): void {
    this.#sweeping = false;
    const forget = (element: Element): void => {
      const bound = own(element);
      if (bound === undefined) return;
      for (const effect of bound.effects ?? []) this.#index.remove(effect);
      if (bound.watch !== undefined) this.#index.remove(bound.watch);
      if (bound.row !== undefined) this.#dropRow(bound.row);
      if (element instanceof HTMLTemplateElement) this.#dropParts(element);
      delete bound.effects;
      delete bound.watch;
      delete bound.row;
    };
    const unfile = (root: Element): void => {
      for (const element of own(root)?.members ?? []) {
        if (!(element instanceof HTMLTemplateElement)) continue;
        for (const instance of this.#lists.get(element)?.values() ?? []) {
          this.#unfile(instance.slot);
          forget(instance.element);
          unfile(instance.element);
        }
        const mounted = this.#mounted.get(element);
        if (mounted !== undefined) unfile(mounted);
      }
    };
    if (this.#patched) {
      this.#patched = false;
      // A patch removes the elements it replaces without unmounting them, so the items among them are found by their slots.
      for (const filed of [...this.#slots.values()]) {
        for (const slot of Array.isArray(filed) ? [...filed] : [filed]) {
          const owner = this.#owners.get(slot);
          if (owner === undefined || owner.isConnected) continue;
          this.#unfile(slot);
          this.#removed.push(owner);
        }
      }
    }
    for (const root of this.#removed) {
      // A patch puts back an element it removed where a later child with the same id moves it.
      if (root.isConnected) continue;
      if (own(root)?.root === true) {
        forget(root);
        this.#visit(root, forget);
        unfile(root);
      } else {
        // An element a patch removed has no list of its members. The slots of the items below it were taken out above.
        forget(root);
        for (const element of root.querySelectorAll("*")) forget(element);
      }
    }
    this.#removed = [];
    for (const effect of this.#stale) if (!effect.element.isConnected) forget(effect.element);
    this.#stale = new Set();
  }

  /** Calls `visit` on each element with bindings inside the item or condition `root`, nested ones included. */
  #visit(root: Element, visit: (element: Element) => void): void {
    for (const element of own(root)?.members ?? []) {
      visit(element);
      if (!(element instanceof HTMLTemplateElement)) continue;
      for (const instance of this.#lists.get(element)?.values() ?? []) this.#visit(instance.element, visit);
      const mounted = this.#mounted.get(element);
      if (mounted !== undefined) this.#visit(mounted, visit);
    }
  }

  #onEvent(type: (typeof BUBBLING)[number], event: Event): void {
    const target = event.target;
    if (!(target instanceof Element)) return;
    this.#adoptAround(target);
    if (type === "input" || type === "change") this.#readControl(target, type);
    if (type === "keydown" && event instanceof KeyboardEvent) this.#onKey(target, event);
    const found = this.#closest(target, `data-hs-on-${type}`);
    if (found === null) return;
    if (type === "submit") event.preventDefault();
    this.#runAttribute(found[1], found[0]);
  }

  #onKey(target: Element, event: KeyboardEvent): void {
    const found = this.#closest(target, "data-hs-on-key");
    if (found === null) return;
    const [handler, source] = found;
    const keys = parse(source) as [string, unknown][];
    for (const [key, stmt] of keys) {
      if (key !== event.key) continue;
      event.preventDefault();
      this.#run(stmt, this.#context(handler, this.#scopeOf(handler)));
    }
  }

  #onDirect(type: (typeof DIRECT)[number], event: Event): void {
    const target = event.target;
    if (!(target instanceof Element)) return;
    this.#adoptAround(target);
    const stmt = this.#attr(target, `data-hs-on-${type}`);
    if (stmt !== null) this.#runAttribute(stmt, target);
  }

  #interval(element: Element, ms: number, stmt: unknown): void {
    const timer = setInterval(() => {
      if (!element.isConnected) {
        clearInterval(timer);
        return;
      }
      this.#run(stmt, this.#context(element, this.#scopeOf(element)));
    }, ms);
  }

  #scopeOf(element: Element | null): readonly Slot[] {
    for (let at = element; at !== null; at = at.parentElement) {
      const scope = own(at)?.scope;
      if (scope !== undefined) return scope;
    }
    return [];
  }

  #findRef(name: string, origin: Element | undefined): Element | null {
    const selector = `[data-hs-ref="${CSS.escape(name)}"]`;
    for (let at = origin ?? null; at !== null; at = at.parentElement) {
      if (own(at)?.root !== true) continue;
      const found = at.matches(selector) ? at : at.querySelector(selector);
      if (found !== null) return found;
    }
    return this.#root.querySelector(selector);
  }

  #fail(method: string, path: string, message: string, error: string | null, status: number): void {
    if (error !== null) this.store.set(error, message);
    this.#root.dispatchEvent(new CustomEvent("hs-error", { detail: { status, message } }));
    console.error(`hypersig: ${method} ${path} failed with ${message}`);
  }

  #runAttribute(source: string, element: Element): void {
    this.#run(parse(source), this.#context(element, this.#scopeOf(element)));
  }

  /** Runs a statement whose reads do not belong to the binding being rendered. */
  #run(ast: unknown, ctx: Context): void {
    this.store.untracked(() => this.#runTracked(ast, ctx));
  }

  /** Runs a statement, recording the reads of its synchronous part in the enclosing binding. */
  #runTracked(ast: unknown, ctx: Context): void {
    run(ast, ctx, this).catch((error: unknown) => {
      console.error("hypersig:", error);
    });
  }
}

const parsed = new Map<string, unknown>();

/**
 * `JSON.parse(source)`, remembered, so the items of a list share one AST per
 * attribute and `evaluate` builds each expression once. The result is never
 * modified, because the store copies a value before changing it.
 */
function parse(source: string | null): unknown {
  if (source === null) return undefined;
  let value = parsed.get(source);
  if (value === undefined) {
    value = JSON.parse(source);
    if (parsed.size > 10_000) parsed.clear();
    parsed.set(source, value);
  }
  return value;
}

/** The key `DepIndex.select` files a value under. */
function keyOf(value: unknown): string {
  return JSON.stringify(value) ?? "undefined";
}

/** The page signal path and the other operand of `ast`, where `ast` compares a page signal for equality. */
function comparison(ast: unknown): [path: string, other: unknown] | null {
  if (!Array.isArray(ast) || ast[0] !== "eq" || ast.length !== 3) return null;
  const [, a, b] = ast as [string, unknown, unknown];
  const signal = (side: unknown): string | null =>
    Array.isArray(side) && side[0] === "sig" && typeof side[1] === "string" ? side[1] : null;
  const left = signal(a);
  if (left !== null) return [left, b];
  const right = signal(b);
  return right === null ? null : [right, a];
}

function pairs(element: Element, attribute: string): [string, unknown][] {
  const value = parse(element.getAttribute(attribute));
  return Array.isArray(value) ? (value as [string, unknown][]) : [];
}

/** The parsed `data-hs-*` attributes an element builds its bindings from. */
interface Spec {
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

function readSpec(element: Element): Spec {
  const effects = parse(element.getAttribute("data-hs-effect"));
  const watches = parse(element.getAttribute("data-hs-on-signal"));
  const each = parse(element.getAttribute("data-hs-each"));
  return {
    show: parse(element.getAttribute("data-hs-show")),
    text: parse(element.getAttribute("data-hs-text")),
    checked: parse(element.getAttribute("data-hs-checked")),
    classes: pairs(element, "data-hs-class"),
    attrs: pairs(element, "data-hs-attr"),
    toggles: pairs(element, "data-hs-toggle"),
    styles: pairs(element, "data-hs-style"),
    bind: parse(element.getAttribute("data-hs-bind")) as Bind | undefined,
    effects: Array.isArray(effects) ? effects : [],
    watches: Array.isArray(watches) ? (watches as [unknown, unknown][]) : [],
    proto: element instanceof HTMLTemplateElement ? element.content.firstElementChild : null,
    each: Array.isArray(each) ? (each as [unknown, number]) : undefined,
    cond: parse(element.getAttribute("data-hs-if")),
  };
}

const specs = new WeakMap<Element, Spec>();

/**
 * `readSpec(source)`, remembered for an element inside a `<template>`, whose
 * attributes no patch changes, so the items built from it share one spec.
 */
function specOf(source: Element): Spec {
  let spec = specs.get(source);
  if (spec === undefined) {
    spec = readSpec(source);
    specs.set(source, spec);
  }
  return spec;
}

/** Where the elements that carry an attribute sit inside a template's element, by element child index. */
interface Plan {
  readonly tag: string;
  readonly source: Element | null;
  /** Whether `source` carries a reactive attribute, rather than a node ref alone. */
  readonly reactive: boolean;
  readonly children: readonly (readonly [number, Plan])[];
}

const plans = new WeakMap<Element, Plan>();

function planOf(proto: Element): Plan {
  let plan = plans.get(proto);
  if (plan === undefined) {
    plan = build(proto) ?? { tag: proto.tagName, source: null, reactive: false, children: [] };
    plans.set(proto, plan);
  }
  return plan;
}

function build(element: Element): Plan | null {
  const children: [number, Plan][] = [];
  [...element.children].forEach((child, index) => {
    const plan = build(child);
    if (plan !== null) children.push([index, plan]);
  });
  const reactive = element.matches(REACTIVE);
  const carries = reactive || element.hasAttribute("data-hs-ref");
  return carries || children.length > 0 ? { tag: element.tagName, source: carries ? element : null, reactive, children } : null;
}

/**
 * Calls `found` with each element of `copy` that `plan` names, and the
 * template element it was built from. A child that carries `data-hs-ssr` was
 * rendered for a nested list or condition, has no counterpart in the
 * template, and is skipped when counting positions.
 */
function locate(plan: Plan, copy: Element, found: (element: Element, source: Element, reactive: boolean) => void): void {
  if (plan.source !== null) found(copy, plan.source, plan.reactive);
  if (plan.children.length === 0) return;
  // Walks the siblings, because reading `children` makes the element keep an HTMLCollection.
  let at = copy.firstElementChild;
  let position = 0;
  for (const [index, child] of plan.children) {
    for (; at !== null; at = at.nextElementSibling) {
      if (at.hasAttribute("data-hs-ssr")) continue;
      if (position++ === index) break;
    }
    if (at === null) return;
    if (at.tagName === child.tag) locate(child, at, found);
    at = at.nextElementSibling;
  }
}

type Write = Exclude<Part["kind"], "bind" | "if">;

/** The kinds of binding whose expression is a condition, which `#compare` or a selector part can answer. */
const CONDITIONS: ReadonlySet<Part["kind"]> = new Set(["show", "class", "toggle", "if"]);

/** The bindings of `spec` that write to the element, in the order the runtime runs them. */
function writes(spec: Spec): [kind: Write, name: string, ast: unknown][] {
  const found: [Write, string, unknown][] = [];
  if (spec.show !== undefined) found.push(["show", "", spec.show]);
  if (spec.text !== undefined) found.push(["text", "", spec.text]);
  if (spec.checked !== undefined) found.push(["checked", "", spec.checked]);
  for (const [name, when] of spec.classes) found.push(["class", name, when]);
  for (const [name, expr] of spec.attrs) found.push(["attr", name, expr]);
  for (const [name, when] of spec.toggles) found.push(["toggle", name, when]);
  for (const [name, expr] of spec.styles) found.push(["style", name, expr]);
  return found;
}

/** Whether a binding of `kind` can write to `element`: `show` and `style` need an inline style, `checked` an input. */
function writable(element: Element, kind: Write): boolean {
  if (kind === "show" || kind === "style") return element instanceof HTMLElement;
  return kind !== "checked" || element instanceof HTMLInputElement;
}

/** Writes `value` to `element` as a binding of `kind` does, where it differs from what the element shows and `writable` holds. */
function write(element: Element, kind: Write, name: string, value: unknown): void {
  if (!writable(element, kind)) return;
  switch (kind) {
    case "show": {
      const next = value ? "" : "none";
      if ((element as HTMLElement).style.display !== next) (element as HTMLElement).style.display = next;
      return;
    }
    case "text": {
      const next = String(value);
      if (element.textContent !== next) element.textContent = next;
      return;
    }
    case "checked":
      (element as HTMLInputElement).checked = Boolean(value);
      return;
    case "class":
      if (element.classList.contains(name) !== value) element.classList.toggle(name, Boolean(value));
      return;
    case "attr": {
      const next = String(value);
      if (element.getAttribute(name) !== next) element.setAttribute(name, next);
      return;
    }
    case "toggle":
      if (element.hasAttribute(name) !== value) element.toggleAttribute(name, Boolean(value));
      return;
    case "style":
      (element as HTMLElement).style.setProperty(name, String(value));
  }
}

const shapes = new WeakMap<Element, Shape>();

/** The parts of the items built from `proto`, a list template at item depth `depth`. */
function shapeOf(proto: Element, depth: number): Shape {
  let shape = shapes.get(proto);
  if (shape !== undefined) return shape;
  const parts: Part[] = [];
  const fields: string[][] = [];
  const partsOf = new Map<Element, number[]>();
  const external = new Map<string, [StaticRead, number]>();
  let moved = 0;
  let reactive = 0;
  sources(planOf(proto), (source) => {
    reactive++;
    const found = partsFor(source, depth);
    // A row runs at most 30 parts, one bit each in a small integer.
    if (found === null || parts.length + found.length > 30) return;
    const indices: number[] = [];
    for (const [part, reads] of found) {
      const index = parts.push(part) - 1;
      indices.push(index);
      const own: string[] = [];
      for (const read of reads) {
        if (read.kind === "item" && read.depth === depth) own.push(read.field);
        else if (read.kind === "index" && read.depth === depth) moved |= 1 << index;
        else {
          const id = read.kind === "sig" ? `s ${read.path}` : read.kind === "item" ? `i ${read.depth} ${read.field}` : `x ${read.depth}`;
          const entry = external.get(id);
          if (entry === undefined) external.set(id, [read, 1 << index]);
          else entry[1] |= 1 << index;
        }
      }
      fields.push(own);
    }
    partsOf.set(source, indices);
  });
  const keyAst = parse(proto.getAttribute("data-hs-key"));
  const keyReads = keyAst === undefined ? [] : staticReads(keyAst);
  const keyFields =
    keyReads !== null && keyReads.every((read) => read.kind === "item" && read.depth === depth)
      ? keyReads.map((read) => (read as { field: string }).field)
      : null;
  shape = {
    parts,
    partsOf,
    fields,
    external: [...external.values()],
    moved,
    all: parts.length === 0 ? 0 : 2 ** parts.length - 1,
    keyFields: keyAst === undefined ? null : keyFields,
    // The bindings inside a condition that an `if` part shows are effects of their own, which read under the item's slot name.
    whole: partsOf.size === reactive && (keyAst === undefined || keyFields !== null) && !parts.some((part) => part.kind === "if" && part.proto !== null && bound(part.proto)),
    masks: new Map(),
  };
  shapes.set(proto, shape);
  return shape;
}

/** Whether `proto` or an element inside it carries a reactive attribute. */
function bound(proto: Element): boolean {
  let found = false;
  sources(planOf(proto), () => (found = true));
  return found;
}

/** Calls `visit` with each element of `plan` that carries a reactive attribute, in the order `locate` finds them. */
function sources(plan: Plan, visit: (source: Element) => void): void {
  if (plan.source !== null && plan.reactive) visit(plan.source);
  for (const [, child] of plan.children) sources(child, visit);
}

/**
 * The parts of the bindings of `source`, an element of a list template at
 * item depth `depth`, with the reads each can make. Null where it carries a
 * binding that stays an `Effect`: a statement, a nested list or condition,
 * or an expression whose reads are not known from its AST.
 */
function partsFor(source: Element, depth: number): [Part, StaticRead[]][] | null {
  const spec = specOf(source);
  if (spec.effects.length > 0 || spec.watches.length > 0 || spec.each !== undefined) return null;
  if (source.hasAttribute("data-hs-on-load") || source.hasAttribute("data-hs-on-interval")) return null;
  const found: [Part, StaticRead[]][] = [];
  const conditions: [Part["kind"], string, unknown][] = writes(spec);
  if (spec.cond !== undefined && source instanceof HTMLTemplateElement) conditions.push(["if", "", spec.cond]);
  for (const [kind, name, ast] of conditions) {
    const reads = staticReads(ast);
    if (reads === null || reads.some((read) => read.kind !== "sig" && read.depth > depth)) return null;
    const compared = CONDITIONS.has(kind) ? comparison(ast) : null;
    const otherReads = compared === null ? null : staticReads(compared[1]);
    if (otherReads !== null && otherReads.every((read) => read.kind === "item" && read.depth === depth)) {
      found.push([{ kind, name, ast, selector: { path: compared![0], other: compared![1] }, proto: spec.proto }, otherReads]);
    } else {
      found.push([{ kind, name, ast, selector: null, proto: spec.proto }, reads]);
    }
  }
  if (spec.bind !== undefined) {
    const read = placeRead(spec.bind[0]);
    if (read === null || (read.kind === "item" && read.depth > depth)) return null;
    found.push([{ kind: "bind", name: "", ast: spec.bind, selector: null, proto: null }, [read]]);
  }
  return found;
}

/** The name a read outside a list's items is recorded under, for a list inside the items `outer`. */
function readName(read: StaticRead, outer: readonly Slot[]): string {
  if (read.kind === "sig") return read.path;
  const slot = outer[read.depth]!;
  const name = slot.read ?? slot.path;
  return read.kind === "index" ? positionOf(name) : join(name, read.field);
}

/** Whether a change at `rest` inside an item, the whole value there when `inside`, can change a deep read of its field `field`. */
function reaches(field: string, rest: string, inside: boolean): boolean {
  if (field === "" || rest === field || rest.startsWith(`${field}.`)) return true;
  return inside && (rest === "" || field.startsWith(`${rest}.`));
}

/** The parts of `shape` that a change at `rest` inside an item can reach. */
function maskFor(shape: Shape, rest: string, inside: boolean): number {
  const id = `${inside ? 1 : 0}${rest}`;
  let mask = shape.masks.get(id);
  if (mask === undefined) {
    mask = 0;
    shape.fields.forEach((own, index) => {
      if (own.some((field) => reaches(field, rest, inside))) mask! |= 1 << index;
    });
    // A change inside a nested list names the position it is at, so the changes seen can keep growing.
    if (shape.masks.size > 1000) shape.masks.clear();
    shape.masks.set(id, mask);
  }
  return mask;
}

function unfileRow(selection: Selection, key: string, row: Row): void {
  const filed = selection.byKey.get(key);
  if (filed === row) selection.byKey.delete(key);
  else if (Array.isArray(filed)) {
    const rest = filed.filter((other) => other !== row);
    if (rest.length === 0) selection.byKey.delete(key);
    else selection.byKey.set(key, rest.length === 1 ? rest[0]! : rest);
  }
}

/**
 * Puts the elements of `order` right after `template` in that order, moving
 * as few as it can. The items of `previous` still in the document keep their
 * place where their old positions increase along `order`, the longest such
 * run, and the rest are inserted around them.
 */
function arrange(parent: Node, template: Element, previous: ReadonlyMap<string, Instance>, order: readonly Instance[]): void {
  const placed: Instance[] = [];
  for (const instance of previous.values()) if (instance.element.parentNode === parent) placed.push(instance);
  // The items that keep their place at either end stay without entering the run below.
  let start = 0;
  while (start < order.length && start < placed.length && order[start] === placed[start]) start++;
  let end = 0;
  while (end < order.length - start && end < placed.length - start && order[order.length - 1 - end] === placed[placed.length - 1 - end]) end++;
  const positions = new Map<Instance, number>();
  for (let at = start; at < placed.length - end; at++) positions.set(placed[at]!, at);
  const stay = increasing(order.slice(start, order.length - end).map((instance) => positions.get(instance) ?? -1));
  const last = placed.at(-1)?.element;
  let before: Node | null = end > 0 ? order[order.length - end]!.element : last === undefined ? template.nextSibling : last.nextSibling;
  for (let index = order.length - 1 - end; index >= start; index--) {
    const element = order[index]!.element;
    if (!stay.has(index - start)) parent.insertBefore(element, before);
    before = element;
  }
}

/** `reads`, made by a key that read only the item, under the name `from`, as if it had read it under `to`. */
function rebased(reads: Reads, from: string, to: string): Reads {
  if (reads.size === 0) return reads;
  const moved: Reads = new Map();
  for (const [path, deep] of reads) moved.set(to + path.slice(from.length), deep);
  return moved;
}

/** The indices of a longest strictly increasing run of the values of `values` that are not -1. */
function increasing(values: readonly number[]): Set<number> {
  /** `tails[k]` is the index ending the run of length k + 1 whose last value is smallest. */
  const tails: number[] = [];
  const previous = new Array<number>(values.length).fill(-1);
  values.forEach((value, index) => {
    if (value === -1) return;
    let low = 0;
    let high = tails.length;
    while (low < high) {
      const mid = (low + high) >> 1;
      if (values[tails[mid]!]! < value) low = mid + 1;
      else high = mid;
    }
    if (low > 0) previous[index] = tails[low - 1]!;
    tails[low] = index;
  });
  const run = new Set<number>();
  for (let at = tails.length === 0 ? -1 : tails[tails.length - 1]!; at !== -1; at = previous[at]!) run.add(at);
  return run;
}

/**
 * Whether `changes` only appended items to the list at `path`, which `known`
 * rendered at its previous length, so the items it holds are unchanged and
 * still sit at the same paths.
 */
function appended(changes: Changes, path: string, known: ReadonlyMap<string, Instance>, length: number): boolean {
  if (changes.get(path) !== false || length < known.size) return false;
  const prefix = `${path}.`;
  for (const changed of changes.keys()) {
    if (!changed.startsWith(prefix)) continue;
    const segment = changed.slice(prefix.length).split(".", 1)[0]!;
    if (!(Number(segment) >= known.size)) return false;
  }
  return inPlace(path, known);
}

/**
 * The positions at which `changes` put other values into the list at `path`,
 * which `known` rendered at the same length, where it changed nothing else of
 * the list. The items at the other positions are unchanged and still sit at
 * the same paths. Null where the changes did more, or reached most items.
 */
function replaced(changes: Changes, path: string, known: ReadonlyMap<string, Instance>, length: number): Set<number> | null {
  if (changes.get(path) !== false || length !== known.size) return null;
  const prefix = `${path}.`;
  const touched = new Set<number>();
  for (const changed of changes.keys()) {
    if (!changed.startsWith(prefix)) continue;
    const index = Number(changed.slice(prefix.length).split(".", 1)[0]);
    if (!(Number.isInteger(index) && index >= 0 && index < length)) return null;
    touched.add(index);
  }
  return touched.size * 2 <= length && inPlace(path, known) ? touched : null;
}

/**
 * Whether the items of `known` are in the document, each at the path of its
 * position in the list at `path`. Only the first and the last path are
 * compared, because `#renderList` and `#relocate` give the items of one list
 * their paths together, so they agree with each other.
 */
function inPlace(path: string, known: ReadonlyMap<string, Instance>): boolean {
  let last: Instance | undefined;
  for (const instance of known.values()) {
    if (last === undefined && instance.slot.path !== join(path, "0")) return false;
    if (!instance.element.isConnected) return false;
    last = instance;
  }
  return last === undefined || last.slot.path === join(path, String(known.size - 1));
}
