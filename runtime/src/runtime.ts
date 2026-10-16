import { type Context, evaluate, type Host, resolve, run, type Slot } from "./eval.ts";
import { morph } from "./morph.ts";
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
  /** What evaluating this item's key read, recorded again when the key is not reevaluated. */
  keyReads: Reads;
  /**
   * Whether the key read nothing but the item's own fields, so another item
   * with the same value has the same key wherever it sits.
   */
  local: boolean;
  /** The item's value when the list last matched it, to tell a move from a change. */
  value: unknown;
}

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
  /** `#changes` indexed for `#rebase`, built the first time a render needs it. */
  #lookup: ChangeLookup | null = null;
  #nextId = 0;
  readonly #lists = new WeakMap<Element, Map<string, Instance>>();
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
   * Items and conditions taken out of the document, and bindings found
   * outside it, whose registrations `#sweep` removes after the next paint.
   * The runtime never puts a removed element back, so until then they only
   * hold memory, and a render that reaches them skips them.
   */
  #removed: Element[] = [];
  #stale = new Set<Effect>();
  #sweeping = false;

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
   * position in `proto`.
   */
  #mount(root: Element, proto: Element, scope: readonly Slot[]): void {
    const members: Element[] = [];
    locate(planOf(proto), root, (element, source) => {
      state(element).source = source;
      if (source.hasAttribute("data-hs-ref") && !element.hasAttribute("data-hs-ref")) {
        element.setAttribute("data-hs-ref", source.getAttribute("data-hs-ref")!);
      }
      if (!source.matches(REACTIVE)) return;
      members.push(element);
      this.#register(element, scope);
    });
    state(root).members = members;
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
        const morphed = morph(target, next);
        this.scan(morphed, this.#scopeOf(morphed.parentElement));
        this.#render(new Map());
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
    const add = (run: () => void): void => {
      effects.push({ id: this.#nextId++, element, run, deps: null, nodes: new Map(), selector: null });
    };

    const { show, text, checked } = spec;
    if (show !== undefined && element instanceof HTMLElement) {
      add(() => {
        const next = truth(show) ? "" : "none";
        if (element.style.display !== next) element.style.display = next;
      });
    }
    if (text !== undefined) {
      add(() => {
        const next = String(value(text));
        if (element.textContent !== next) element.textContent = next;
      });
    }
    if (checked !== undefined && element instanceof HTMLInputElement) {
      add(() => {
        element.checked = Boolean(value(checked));
      });
    }
    for (const [name, when] of spec.classes) {
      add(() => {
        const on = truth(when);
        if (element.classList.contains(name) !== on) element.classList.toggle(name, on);
      });
    }
    for (const [name, expr] of spec.attrs) {
      add(() => {
        const next = String(value(expr));
        if (element.getAttribute(name) !== next) element.setAttribute(name, next);
      });
    }
    for (const [name, when] of spec.toggles) {
      add(() => {
        const on = truth(when);
        if (element.hasAttribute(name) !== on) element.toggleAttribute(name, on);
      });
    }
    for (const [name, expr] of spec.styles) {
      if (!(element instanceof HTMLElement)) continue;
      add(() => element.style.setProperty(name, String(value(expr))));
    }
    const bind = spec.bind;
    if (bind !== undefined) add(() => this.#writeControl(element, bind, ctx));

    for (const stmt of spec.effects) add(() => this.#runTracked(stmt, ctx));
    for (const [place, stmt] of spec.watches) {
      let first = true;
      add(() => {
        this.store.get(resolve(place, ctx));
        if (!first) this.#run(stmt, ctx);
        first = false;
      });
    }

    if (element instanceof HTMLTemplateElement) {
      const { proto, each, cond } = spec;
      if (each !== undefined) add(() => this.#renderList(element, proto, each[0], each[1], scope));
      if (cond !== undefined) add(() => this.#renderIf(element, proto, Boolean(value(cond)), scope));
    }
    return effects;
  }

  #renderList(template: HTMLTemplateElement, proto: Element | null, place: unknown, depth: number, scope: readonly Slot[]): void {
    const parent = template.parentNode;
    if (proto === null || parent === null) return;
    const path = resolve(place, { store: this.store, scope });
    const value = this.store.getShallow(path);
    const items = Array.isArray(value) ? value : [];
    const keyAst = parse(proto.getAttribute("data-hs-key"));
    const known = this.#lists.get(template);
    const created = new Set<Instance>();
    const itemScope = (itemPath: string): Slot[] => [...scope.slice(0, depth), { path: itemPath }];
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
    const match = (index: number, previous: ReadonlyMap<string, Instance>, next: Map<string, Instance>): Instance => {
      const itemPath = join(path, String(index));
      let key = `#${index}`;
      let keyReads: Reads = new Map();
      let local = false;
      if (keyAst !== undefined) {
        const same = sameValue(previous, index);
        if (same !== undefined) {
          key = same.key;
          keyReads = same.slot.path === itemPath ? same.keyReads : rebased(same.keyReads, same.slot.path, itemPath);
          local = true;
        } else {
          const slot = itemPath;
          keyReads = this.store.track(() => {
            key = JSON.stringify(evaluate(keyAst, { store: this.store, scope: itemScope(slot) }));
          });
          local = [...keyReads.keys()].every((read) => read !== positionOf(slot) && (read === slot || read.startsWith(`${slot}.`)));
        }
        this.store.replay(keyReads);
      }
      const raw = key;
      if (next.has(key)) {
        console.warn(`hypersig: duplicate key ${key} in a list; matching it by position`);
        key = `${key}#${index}`;
      }
      let instance = previous.get(key);
      if (instance === undefined || !instance.element.isConnected) {
        instance = { element: proto.cloneNode(true) as Element, slot: { path: itemPath }, scanned: true, key: raw, keyReads, local, value: items[index] };
        state(instance.element).root = true;
        created.add(instance);
      } else {
        instance.key = raw;
        instance.keyReads = keyReads;
        instance.local = local;
        if (instance.slot.path !== itemPath) {
          const from = instance.slot.path;
          instance.slot.path = itemPath;
          if (instance.scanned && Object.is(instance.value, items[index])) this.#rebase(instance.element, from, itemPath);
          else if (instance.scanned) this.#requeue(instance.element);
        }
        instance.value = items[index];
      }
      next.set(key, instance);
      matched = key;
      return instance;
    };
    const register = (order: readonly Instance[]): void => {
      for (const instance of order) {
        if (created.has(instance)) this.#mount(instance.element, proto, [...scope.slice(0, depth), instance.slot]);
        else if (!instance.scanned && !this.#waiting.has(instance.element)) this.#wait(instance, proto, [...scope.slice(0, depth), instance.slot]);
      }
    };

    if (known !== undefined && appended(this.#changes, path, known, items.length)) {
      let anchor: Node = template;
      for (const instance of known.values()) {
        this.store.replay(instance.keyReads);
        anchor = instance.element;
      }
      const next = new Map(known);
      const added: Instance[] = [];
      for (let index = known.size; index < items.length; index++) added.push(match(index, known, next));
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
      order.forEach((instance, index) => {
        next.set(keys[index]!, instance);
        if (!touched.has(index)) this.store.replay(instance.keyReads);
      });
      for (const index of at) if (next.get(wasKeys[index]!) !== was[index]) this.#unmount(was[index]!.element);
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

    const previous = known ?? this.#adoptItems(template);
    const next = new Map<string, Instance>();
    const order = items.map((_, index) => match(index, previous, next));
    for (const [key, instance] of previous) {
      if (next.get(key) !== instance) this.#unmount(instance.element);
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
      this.#mount(instance.element, proto, scope);
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
    }
  }

  /** Takes over the items the server rendered right after `template`, keyed as it keyed them. */
  #adoptItems(template: HTMLTemplateElement): Map<string, Instance> {
    const adopted = new Map<string, Instance>();
    for (let at = template.nextElementSibling; at !== null && at.hasAttribute("data-hs-ssr"); at = at.nextElementSibling) {
      if (own(at)?.root === true) break;
      state(at).root = true;
      const key = at.getAttribute("data-hs-ssr")!;
      adopted.set(key, { element: at, slot: { path: "" }, scanned: false, key, keyReads: new Map(), local: false, value: undefined });
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
    const current = this.store.get(resolve(place, ctx));
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
    this.#lookup = null;
    const found = this.#index.affected(changes);
    if (changes.size > 0) {
      this.#index.selected(
        (path) => (this.#lookup ??= new ChangeLookup(changes)).affects(path, true),
        (path) => keyOf(this.store.untracked(() => this.store.get(path))),
        found,
      );
    }
    const affected = [...found].sort((a, b) => a.id - b.id);
    for (const effect of affected) this.#runEffect(effect);
    while (this.#fresh.length > 0) {
      const batch = this.#fresh;
      this.#fresh = [];
      for (const effect of batch) this.#runEffect(effect);
    }
    // The bindings that ran hold their elements, which may have left the document.
    this.#ran = new Set();
    this.#changes = new Map();
    this.#lookup = null;
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

  /** Reruns the bindings inside `root`, whose item now sits at a different path. */
  #requeue(root: Element): void {
    this.#visit(root, (element) => {
      for (const effect of own(element)?.effects ?? []) {
        this.#ran.delete(effect);
        this.#fresh.push(effect);
      }
    });
  }

  /**
   * Moves the reads of the bindings inside `root` from the item path `from`
   * to `to`, for an item whose value moved there unchanged, so they do not
   * run again. A binding still runs again where it read the item's position
   * or where `changes` reach something else it read or the path its selector compares.
   */
  #rebase(root: Element, from: string, to: string): void {
    const prefix = `${from}.`;
    const move = (path: string): string => (path === from || path.startsWith(prefix) ? to + path.slice(from.length) : path);
    this.#lookup ??= new ChangeLookup(this.#changes);
    const lookup = this.#lookup;
    for (const element of own(root)?.members ?? []) {
      for (const effect of own(element)?.effects ?? []) {
        if (effect.deps === null) continue;
        let rerun = effect.selector !== null && lookup.affects(effect.selector.path, true);
        for (const [path, deep] of effect.deps) {
          if (move(path) === path ? lookup.affects(path, deep) : path === positionOf(from)) rerun = true;
        }
        this.#index.move(effect, move);
        if (rerun) {
          this.#ran.delete(effect);
          this.#fresh.push(effect);
        } else {
          this.#ran.add(effect);
        }
      }
      if (!(element instanceof HTMLTemplateElement)) continue;
      for (const instance of this.#lists.get(element)?.values() ?? []) {
        instance.slot.path = move(instance.slot.path);
        instance.keyReads = new Map([...instance.keyReads].map(([path, deep]) => [move(path), deep]));
        this.#rebase(instance.element, from, to);
      }
      const mounted = this.#mounted.get(element);
      if (mounted !== undefined) this.#rebase(mounted, from, to);
    }
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
      for (const effect of bound?.effects ?? []) this.#index.remove(effect);
      if (bound !== undefined) delete bound.effects;
    };
    for (const root of this.#removed) this.#visit(root, forget);
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
  readonly children: readonly (readonly [number, Plan])[];
}

const plans = new WeakMap<Element, Plan>();

function planOf(proto: Element): Plan {
  let plan = plans.get(proto);
  if (plan === undefined) {
    plan = build(proto) ?? { tag: proto.tagName, source: null, children: [] };
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
  const carries = element.matches(REACTIVE) || element.hasAttribute("data-hs-ref");
  return carries || children.length > 0 ? { tag: element.tagName, source: carries ? element : null, children } : null;
}

/**
 * Calls `found` with each element of `copy` that `plan` names, and the
 * template element it was built from. A child that carries `data-hs-ssr` was
 * rendered for a nested list or condition, has no counterpart in the
 * template, and is skipped when counting positions.
 */
function locate(plan: Plan, copy: Element, found: (element: Element, source: Element) => void): void {
  if (plan.source !== null) found(copy, plan.source);
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

/** `reads`, made by the item at `from`, as the item at `to` would have made them. */
function rebased(reads: Reads, from: string, to: string): Reads {
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
 * compared, because `#renderList` and `#rebase` give the items of one list
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
