import { type Context, evaluate, type Host, resolve, run, type Slot } from "./eval.ts";
import { morph } from "./morph.ts";
import { readEventStream, type ServerEvent } from "./sse.ts";
import { DepIndex, type Effect } from "./deps.ts";
import { equal, join, Store, type Values } from "./store.ts";

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

interface Instance {
  readonly element: Element;
  readonly slot: Slot;
  /** False for an element the server rendered, until the runtime scans it with its item scope. */
  scanned: boolean;
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
  #nextId = 0;
  readonly #scopes = new WeakMap<Element, readonly Slot[]>();
  readonly #started = new WeakSet<Element>();
  readonly #instanceRoots = new WeakSet<Element>();
  readonly #lists = new WeakMap<Element, Map<string, Instance>>();
  readonly #mounted = new WeakMap<Element, Element>();
  readonly #timers = new WeakMap<Element, Map<string, ReturnType<typeof setTimeout>>>();

  constructor(root: Element, initial: Values) {
    this.#root = root;
    this.store = new Store(initial);
    this.store.subscribe((changed) => this.#render(changed));
    for (const type of BUBBLING) {
      root.addEventListener(type, (event) => this.#onEvent(type, event));
    }
    for (const type of DIRECT) {
      root.addEventListener(type, (event) => this.#onDirect(type, event), { capture: true });
    }
    this.scan(root, []);
    this.#render(new Set());
  }

  /** Registers the reactive attributes of `root` and every element below it. */
  scan(root: Element, scope: readonly Slot[]): void {
    const elements = root.matches(REACTIVE) ? [root] : [];
    elements.push(...root.querySelectorAll(REACTIVE));
    for (const element of elements) {
      const rendered = element.closest("[data-hs-ssr]");
      if (rendered !== null && !this.#instanceRoots.has(rendered)) continue;
      const own = this.#scopes.get(element) ?? scope;
      this.#scopes.set(element, own);
      for (const old of this.#effects.get(element) ?? []) this.#index.remove(old);
      const effects = this.#effectsFor(element, own);
      this.#effects.set(element, effects);
      this.#fresh.push(...effects);
      if (this.#started.has(element)) continue;
      this.#started.add(element);
      const load = element.getAttribute("data-hs-on-load");
      if (load !== null) this.#runAttribute(load, element);
      const interval = parse(element.getAttribute("data-hs-on-interval"));
      if (Array.isArray(interval)) this.#interval(element, interval[0] as number, interval[1]);
    }
  }

  /** Calls an action with the current signals and applies the patches it streams back. */
  async call(
    method: string,
    path: string,
    args: unknown,
    indicator: string | null,
    error: string | null,
  ): Promise<void> {
    const url = new URL(path, document.baseURI);
    if (args !== null && args !== undefined) url.searchParams.set("args", JSON.stringify(args));
    const signals = JSON.stringify(this.store.snapshot());
    const init: RequestInit = { method, headers: { accept: "text/event-stream" } };
    if (method === "GET") {
      url.searchParams.set("signals", signals);
    } else {
      init.headers = { accept: "text/event-stream", "content-type": "application/json" };
      init.body = signals;
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
        const target = this.#findRef(name, origin);
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
        this.#render(new Set());
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
    const ctx = this.#context(element, scope);
    const value = (ast: unknown) => evaluate(ast, ctx);
    const effects: Effect[] = [];
    const add = (run: () => void): void => {
      effects.push({ id: this.#nextId++, element, run, deps: null, nodes: [] });
    };

    const show = parse(element.getAttribute("data-hs-show"));
    if (show !== undefined && element instanceof HTMLElement) {
      add(() => {
        element.style.display = value(show) ? "" : "none";
      });
    }
    const text = parse(element.getAttribute("data-hs-text"));
    if (text !== undefined) {
      add(() => {
        const next = String(value(text));
        if (element.textContent !== next) element.textContent = next;
      });
    }
    const checked = parse(element.getAttribute("data-hs-checked"));
    if (checked !== undefined && element instanceof HTMLInputElement) {
      add(() => {
        element.checked = Boolean(value(checked));
      });
    }
    for (const [name, when] of pairs(element, "data-hs-class")) {
      add(() => element.classList.toggle(name, Boolean(value(when))));
    }
    for (const [name, expr] of pairs(element, "data-hs-attr")) {
      add(() => {
        const next = String(value(expr));
        if (element.getAttribute(name) !== next) element.setAttribute(name, next);
      });
    }
    for (const [name, when] of pairs(element, "data-hs-toggle")) {
      add(() => element.toggleAttribute(name, Boolean(value(when))));
    }
    for (const [name, expr] of pairs(element, "data-hs-style")) {
      if (!(element instanceof HTMLElement)) continue;
      add(() => element.style.setProperty(name, String(value(expr))));
    }
    const bind = parse(element.getAttribute("data-hs-bind")) as Bind | undefined;
    if (bind !== undefined) add(() => this.#writeControl(element, bind, ctx));

    const effect = parse(element.getAttribute("data-hs-effect"));
    if (Array.isArray(effect)) {
      for (const stmt of effect) add(() => this.#runTracked(stmt, ctx));
    }
    const watches = parse(element.getAttribute("data-hs-on-signal"));
    if (Array.isArray(watches)) {
      for (const [place, stmt] of watches as [unknown, unknown][]) {
        let first = true;
        add(() => {
          this.store.get(resolve(place, ctx));
          if (!first) this.#run(stmt, ctx);
          first = false;
        });
      }
    }

    if (element instanceof HTMLTemplateElement) {
      const each = parse(element.getAttribute("data-hs-each"));
      if (Array.isArray(each)) add(() => this.#renderList(element, each[0], each[1] as number, scope));
      const cond = parse(element.getAttribute("data-hs-if"));
      if (cond !== undefined) add(() => this.#renderIf(element, Boolean(value(cond)), scope));
    }
    return effects;
  }

  #renderList(template: HTMLTemplateElement, place: unknown, depth: number, scope: readonly Slot[]): void {
    const proto = template.content.firstElementChild;
    const parent = template.parentNode;
    if (proto === null || parent === null) return;
    const ctx: Context = { store: this.store, scope };
    const path = resolve(place, ctx);
    const items = this.store.getShallow(path);
    const keyAst = parse(proto.getAttribute("data-hs-key"));
    const previous = this.#lists.get(template) ?? this.#adoptItems(template);
    const next = new Map<string, Instance>();
    let anchor: Node = template;

    (Array.isArray(items) ? items : []).forEach((_, index) => {
      const itemPath = join(path, String(index));
      const itemScope = [...scope.slice(0, depth), { path: itemPath }];
      let key = keyAst === undefined ? `#${index}` : JSON.stringify(evaluate(keyAst, { store: this.store, scope: itemScope }));
      if (next.has(key)) {
        console.warn(`hypersig: duplicate key ${key} in a list; matching it by position`);
        key = `${key}#${index}`;
      }
      let instance = previous.get(key);
      if (instance === undefined || !instance.element.isConnected) {
        instance = { element: proto.cloneNode(true) as Element, slot: itemScope[depth]!, scanned: false };
        this.#instanceRoots.add(instance.element);
      } else {
        if (instance.slot.path !== itemPath) {
          instance.slot.path = itemPath;
          this.#requeue(instance.element);
        }
      }
      next.set(key, instance);
      if (anchor.nextSibling !== instance.element) parent.insertBefore(instance.element, anchor.nextSibling);
      if (!instance.scanned) {
        instance.scanned = true;
        this.scan(instance.element, [...scope.slice(0, depth), instance.slot]);
      }
      anchor = instance.element;
    });

    for (const [key, instance] of previous) {
      if (next.get(key) !== instance) this.#unmount(instance.element);
    }
    this.#lists.set(template, next);
  }

  /** Takes over the items the server rendered right after `template`, keyed as it keyed them. */
  #adoptItems(template: HTMLTemplateElement): Map<string, Instance> {
    const adopted = new Map<string, Instance>();
    for (let at = template.nextElementSibling; at !== null && at.hasAttribute("data-hs-ssr"); at = at.nextElementSibling) {
      if (this.#instanceRoots.has(at)) break;
      this.#instanceRoots.add(at);
      adopted.set(at.getAttribute("data-hs-ssr")!, { element: at, slot: { path: "" }, scanned: false });
    }
    return adopted;
  }

  #renderIf(template: HTMLTemplateElement, visible: boolean, scope: readonly Slot[]): void {
    let current = this.#mounted.get(template);
    const rendered = template.nextElementSibling;
    if (current === undefined && rendered?.getAttribute("data-hs-ssr") === "" && !this.#instanceRoots.has(rendered)) {
      this.#instanceRoots.add(rendered);
      if (visible) {
        this.#mounted.set(template, rendered);
        this.scan(rendered, scope);
        return;
      }
      this.#unmount(rendered);
    }
    current = this.#mounted.get(template);
    if (visible && (current === undefined || !current.isConnected)) {
      const proto = template.content.firstElementChild;
      if (proto === null || template.parentNode === null) return;
      const element = proto.cloneNode(true) as Element;
      this.#mounted.set(template, element);
      this.#instanceRoots.add(element);
      template.after(element);
      this.scan(element, scope);
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
    const bind = parse(element.getAttribute("data-hs-bind")) as Bind | undefined;
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

  /** Runs the bindings that read a path in `changed`, oldest first, then every binding that has not run yet. */
  #render(changed: ReadonlySet<string>): void {
    this.#ran = new Set();
    const affected = [...this.#index.affected(changed)].sort((a, b) => a.id - b.id);
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
    for (const element of [root, ...root.querySelectorAll(REACTIVE)]) {
      for (const effect of this.#effects.get(element) ?? []) {
        this.#ran.delete(effect);
        this.#fresh.push(effect);
      }
    }
  }

  /** Removes `root` from the document and forgets the bindings inside it. */
  #unmount(root: Element): void {
    root.remove();
    for (const element of [root, ...root.querySelectorAll(REACTIVE)]) {
      for (const effect of this.#effects.get(element) ?? []) this.#index.remove(effect);
      this.#effects.delete(element);
    }
  }

  #onEvent(type: (typeof BUBBLING)[number], event: Event): void {
    const target = event.target;
    if (!(target instanceof Element)) return;
    if (type === "input" || type === "change") this.#readControl(target, type);
    if (type === "keydown" && event instanceof KeyboardEvent) this.#onKey(target, event);
    const attribute = `data-hs-on-${type}`;
    const handler = target.closest(`[${attribute}]`);
    if (handler === null || !this.#root.contains(handler)) return;
    if (type === "submit") event.preventDefault();
    const stmt = handler.getAttribute(attribute);
    if (stmt !== null) this.#runAttribute(stmt, handler);
  }

  #onKey(target: Element, event: KeyboardEvent): void {
    const handler = target.closest("[data-hs-on-key]");
    if (handler === null || !this.#root.contains(handler)) return;
    const keys = parse(handler.getAttribute("data-hs-on-key")) as [string, unknown][];
    for (const [key, stmt] of keys) {
      if (key !== event.key) continue;
      event.preventDefault();
      this.#run(stmt, this.#context(handler, this.#scopeOf(handler)));
    }
  }

  #onDirect(type: (typeof DIRECT)[number], event: Event): void {
    const target = event.target;
    if (!(target instanceof Element)) return;
    const stmt = target.getAttribute(`data-hs-on-${type}`);
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
    this.#run(JSON.parse(source), this.#context(element, this.#scopeOf(element)));
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

function parse(source: string | null): unknown {
  return source === null ? undefined : JSON.parse(source);
}

function pairs(element: Element, attribute: string): [string, unknown][] {
  const value = parse(element.getAttribute(attribute));
  return Array.isArray(value) ? (value as [string, unknown][]) : [];
}
