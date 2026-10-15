import { type Context, evaluate, type Host, resolve, run, type Slot } from "./eval.ts";
import { morph } from "./morph.ts";
import { readEventStream, type ServerEvent } from "./sse.ts";
import { DepIndex, type Effect } from "./deps.ts";
import { type Changes, equal, join, type Reads, Store, type Values } from "./store.ts";

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
  /** What evaluating this item's key read, recorded again when the key is not reevaluated. */
  keyReads: Reads;
}

/** Connects the signals on `<body>` and the `data-hs-*` attributes of `root`. */
export class Runtime implements Host {
  readonly store: Store;
  readonly #root: Element;
  readonly #effects = new Map<Element, Effect[]>();
  readonly #index = new DepIndex();
  /** Bindings to run in the current or next render: new ones, and ones whose item moved. */
  #fresh: Effect[] = [];
  #ran = new Set<Effect>();
  /** The changes the current render is running for. */
  #changes: Changes = new Map();
  #nextId = 0;
  readonly #scopes = new WeakMap<Element, readonly Slot[]>();
  readonly #started = new WeakSet<Element>();
  readonly #instanceRoots = new WeakSet<Element>();
  readonly #lists = new WeakMap<Element, Map<string, Instance>>();
  readonly #mounted = new WeakMap<Element, Element>();
  readonly #timers = new WeakMap<Element, Map<string, ReturnType<typeof setTimeout>>>();
  /**
   * The template element whose `data-hs-*` attributes an element of a list
   * item or a condition carries its bindings from. The server writes those
   * copies without the attributes, and a patch that morphs a clone removes them.
   */
  readonly #sources = new WeakMap<Element, Element>();
  /** The elements with bindings inside each item or condition root, outside its nested ones. */
  readonly #members = new WeakMap<Element, readonly Element[]>();
  /**
   * Registers the bindings of a server-rendered item that is still waiting
   * for its turn, by its root. The item already shows its values, so it
   * waits for a later task, unless an event reaches it first.
   */
  readonly #waiting = new Map<Element, () => void>();
  #draining = false;

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
      if (this.#sources.has(element)) continue;
      this.#register(element, this.#scopes.get(element) ?? scope);
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
      this.#sources.set(element, source);
      if (source.hasAttribute("data-hs-ref") && !element.hasAttribute("data-hs-ref")) {
        element.setAttribute("data-hs-ref", source.getAttribute("data-hs-ref")!);
      }
      if (!source.matches(REACTIVE)) return;
      members.push(element);
      this.#register(element, scope);
    });
    this.#members.set(root, members);
  }

  #register(element: Element, scope: readonly Slot[]): void {
    this.#scopes.set(element, scope);
    for (const old of this.#effects.get(element) ?? []) this.#index.remove(old);
    const effects = this.#effectsFor(element, scope);
    this.#effects.set(element, effects);
    this.#fresh.push(...effects);
    if (this.#started.has(element)) return;
    this.#started.add(element);
    const load = this.#attr(element, "data-hs-on-load");
    if (load !== null) this.#runAttribute(load, element);
    const interval = parse(this.#attr(element, "data-hs-on-interval"));
    if (Array.isArray(interval)) this.#interval(element, interval[0] as number, interval[1]);
  }

  /** The `data-hs-*` attribute `name` of `element`, read from its template element where it has one. */
  #attr(element: Element, name: string): string | null {
    return (this.#sources.get(element) ?? element).getAttribute(name);
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
    const source = this.#sources.get(element);
    const spec = source === undefined ? readSpec(element) : specOf(source);
    const ctx = this.#context(element, scope);
    const value = (ast: unknown) => evaluate(ast, ctx);
    const effects: Effect[] = [];
    const add = (run: () => void): void => {
      effects.push({ id: this.#nextId++, element, run, deps: null, nodes: new Map() });
    };

    const { show, text, checked } = spec;
    if (show !== undefined && element instanceof HTMLElement) {
      add(() => {
        const next = value(show) ? "" : "none";
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
        const on = Boolean(value(when));
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
        const on = Boolean(value(when));
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
    const match = (index: number, previous: ReadonlyMap<string, Instance>, next: Map<string, Instance>): Instance => {
      const itemPath = join(path, String(index));
      const itemScope = [...scope.slice(0, depth), { path: itemPath }];
      let key = `#${index}`;
      let keyReads: Reads = new Map();
      if (keyAst !== undefined) {
        keyReads = this.store.track(() => {
          key = JSON.stringify(evaluate(keyAst, { store: this.store, scope: itemScope }));
        });
        this.store.replay(keyReads);
      }
      if (next.has(key)) {
        console.warn(`hypersig: duplicate key ${key} in a list; matching it by position`);
        key = `${key}#${index}`;
      }
      let instance = previous.get(key);
      if (instance === undefined || !instance.element.isConnected) {
        instance = { element: proto.cloneNode(true) as Element, slot: itemScope[depth]!, scanned: true, keyReads };
        this.#instanceRoots.add(instance.element);
        created.add(instance);
      } else {
        instance.keyReads = keyReads;
        if (instance.slot.path !== itemPath) {
          instance.slot.path = itemPath;
          if (instance.scanned) this.#requeue(instance.element);
        }
      }
      next.set(key, instance);
      return instance;
    };
    const register = (order: readonly Instance[]): void => {
      for (const instance of order) {
        const itemScope = [...scope.slice(0, depth), instance.slot];
        if (created.has(instance)) this.#mount(instance.element, proto, itemScope);
        else if (!instance.scanned && !this.#waiting.has(instance.element)) this.#wait(instance, proto, itemScope);
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
      for (const member of this.#members.get(outermost) ?? []) {
        if (member instanceof HTMLTemplateElement) for (const effect of this.#effects.get(member) ?? []) this.#runEffect(effect);
      }
    }
  }

  /** Takes over the items the server rendered right after `template`, keyed as it keyed them. */
  #adoptItems(template: HTMLTemplateElement): Map<string, Instance> {
    const adopted = new Map<string, Instance>();
    for (let at = template.nextElementSibling; at !== null && at.hasAttribute("data-hs-ssr"); at = at.nextElementSibling) {
      if (this.#instanceRoots.has(at)) break;
      this.#instanceRoots.add(at);
      adopted.set(at.getAttribute("data-hs-ssr")!, { element: at, slot: { path: "" }, scanned: false, keyReads: new Map() });
    }
    return adopted;
  }

  #renderIf(template: HTMLTemplateElement, proto: Element | null, visible: boolean, scope: readonly Slot[]): void {
    let current = this.#mounted.get(template);
    const rendered = template.nextElementSibling;
    if (current === undefined && rendered?.getAttribute("data-hs-ssr") === "" && !this.#instanceRoots.has(rendered)) {
      this.#instanceRoots.add(rendered);
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
      this.#instanceRoots.add(element);
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
    const affected = [...this.#index.affected(changes)].sort((a, b) => a.id - b.id);
    for (const effect of affected) this.#runEffect(effect);
    while (this.#fresh.length > 0) {
      const batch = this.#fresh;
      this.#fresh = [];
      for (const effect of batch) this.#runEffect(effect);
    }
  }

  #runEffect(effect: Effect): void {
    if (this.#ran.has(effect)) return;
    this.#ran.add(effect);
    if (!effect.element.isConnected) {
      this.#index.remove(effect);
      this.#effects.delete(effect.element);
      return;
    }
    const reads = this.store.track(() => {
      try {
        effect.run();
      } catch (error) {
        console.error("hypersig:", error);
      }
    });
    this.#index.set(effect, reads);
  }

  /** Reruns the bindings inside `root`, whose item now sits at a different path. */
  #requeue(root: Element): void {
    this.#visit(root, (element) => {
      for (const effect of this.#effects.get(element) ?? []) {
        this.#ran.delete(effect);
        this.#fresh.push(effect);
      }
    });
  }

  /** Removes `root` from the document and forgets the bindings inside it. */
  #unmount(root: Element): void {
    root.remove();
    this.#visit(root, (element) => {
      for (const effect of this.#effects.get(element) ?? []) this.#index.remove(effect);
      this.#effects.delete(element);
    });
  }

  /** Calls `visit` on each element with bindings inside the item or condition `root`, nested ones included. */
  #visit(root: Element, visit: (element: Element) => void): void {
    for (const element of this.#members.get(root) ?? []) {
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
      const scope = this.#scopes.get(at);
      if (scope !== undefined) return scope;
    }
    return [];
  }

  #findRef(name: string, origin: Element | undefined): Element | null {
    const selector = `[data-hs-ref="${CSS.escape(name)}"]`;
    for (let at = origin ?? null; at !== null; at = at.parentElement) {
      if (!this.#instanceRoots.has(at)) continue;
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
  const children = [...copy.children].filter((child) => !child.hasAttribute("data-hs-ssr"));
  for (const [index, child] of plan.children) {
    const target = children[index];
    if (target !== undefined && target.tagName === child.tag) locate(child, target, found);
  }
}

/**
 * Puts the elements of `order` right after `template` in that order, moving
 * as few as it can. The items of `previous` still in the document keep their
 * place where their old positions increase along `order`, the longest such
 * run, and the rest are inserted around them.
 */
function arrange(parent: Node, template: Element, previous: ReadonlyMap<string, Instance>, order: readonly Instance[]): void {
  const positions = new Map<Instance, number>();
  let last: Element | null = null;
  for (const instance of previous.values()) {
    if (instance.element.parentNode !== parent) continue;
    positions.set(instance, positions.size);
    last = instance.element;
  }
  const stay = increasing(order.map((instance) => positions.get(instance) ?? -1));
  let before: Node | null = last === null ? template.nextSibling : last.nextSibling;
  for (let index = order.length - 1; index >= 0; index--) {
    const element = order[index]!.element;
    if (!stay.has(index)) parent.insertBefore(element, before);
    before = element;
  }
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
  let index = 0;
  for (const instance of known.values()) {
    if (instance.slot.path !== join(path, String(index++)) || !instance.element.isConnected) return false;
  }
  const prefix = `${path}.`;
  for (const changed of changes.keys()) {
    if (!changed.startsWith(prefix)) continue;
    const segment = changed.slice(prefix.length).split(".", 1)[0]!;
    if (!(Number(segment) >= known.size)) return false;
  }
  return true;
}
