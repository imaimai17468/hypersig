import { type Context, evaluate, type Host, resolve, run, type Slot } from "./eval.ts";
import { morph } from "./morph.ts";
import { readEventStream, type ServerEvent } from "./sse.ts";
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
  "every",
  "each",
  "if",
  "on-load",
  "on-key",
  ...BUBBLING.map((type) => `on-${type}`),
  ...DIRECT.map((type) => `on-${type}`),
];
const REACTIVE = ATTRIBUTES.map((name) => `[data-hs-${name}]`).join(",");

type Effect = () => void;
type Bind = [place: unknown, kind: string, value?: unknown];

interface Instance {
  readonly element: Element;
  readonly slot: Slot;
}

/** Connects the signals on `<body>` and the `data-hs-*` attributes of `root`. */
export class Runtime implements Host {
  readonly store: Store;
  readonly #root: Element;
  readonly #effects = new Map<Element, Effect[]>();
  readonly #scopes = new WeakMap<Element, readonly Slot[]>();
  readonly #started = new WeakSet<Element>();
  readonly #instanceRoots = new WeakSet<Element>();
  readonly #lists = new WeakMap<Element, Map<string, Instance>>();
  readonly #mounted = new WeakMap<Element, Element>();
  readonly #watched = new WeakMap<Element, string[]>();
  readonly #timers = new WeakMap<Element, Map<string, ReturnType<typeof setTimeout>>>();

  constructor(root: Element, initial: Values) {
    this.#root = root;
    this.store = new Store(initial);
    this.store.subscribe(() => this.#render());
    for (const type of BUBBLING) {
      root.addEventListener(type, (event) => this.#onEvent(type, event));
    }
    for (const type of DIRECT) {
      root.addEventListener(type, (event) => this.#onDirect(type, event), { capture: true });
    }
    this.scan(root, []);
    this.#render();
  }

  /** Registers the reactive attributes of `root` and every element below it. */
  scan(root: Element, scope: readonly Slot[]): void {
    const elements = root.matches(REACTIVE) ? [root] : [];
    elements.push(...root.querySelectorAll(REACTIVE));
    for (const element of elements) {
      const own = this.#scopes.get(element) ?? scope;
      this.#scopes.set(element, own);
      this.#effects.set(element, this.#effectsFor(element, own));
      if (this.#started.has(element)) continue;
      this.#started.add(element);
      const load = element.getAttribute("data-hs-on-load");
      if (load !== null) this.#runAttribute(load, element);
      const every = parse(element.getAttribute("data-hs-every"));
      if (Array.isArray(every)) this.#every(element, every[0] as number, every[1]);
    }
  }

  /** Calls an action with the current signals and applies the patches it streams back. */
  async call(
    method: string,
    path: string,
    args: unknown,
    pending: string | null,
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
    if (pending !== null) this.store.set(pending, true);
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
      if (pending !== null) this.store.set(pending, false);
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
        this.#render();
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

    const show = parse(element.getAttribute("data-hs-show"));
    if (show !== undefined && element instanceof HTMLElement) {
      effects.push(() => {
        element.style.display = value(show) ? "" : "none";
      });
    }
    const text = parse(element.getAttribute("data-hs-text"));
    if (text !== undefined) {
      effects.push(() => {
        const next = String(value(text));
        if (element.textContent !== next) element.textContent = next;
      });
    }
    const checked = parse(element.getAttribute("data-hs-checked"));
    if (checked !== undefined && element instanceof HTMLInputElement) {
      effects.push(() => {
        element.checked = Boolean(value(checked));
      });
    }
    for (const [name, when] of pairs(element, "data-hs-class")) {
      effects.push(() => element.classList.toggle(name, Boolean(value(when))));
    }
    for (const [name, expr] of pairs(element, "data-hs-attr")) {
      effects.push(() => {
        const next = String(value(expr));
        if (element.getAttribute(name) !== next) element.setAttribute(name, next);
      });
    }
    for (const [name, when] of pairs(element, "data-hs-toggle")) {
      effects.push(() => element.toggleAttribute(name, Boolean(value(when))));
    }
    for (const [name, expr] of pairs(element, "data-hs-style")) {
      if (!(element instanceof HTMLElement)) continue;
      effects.push(() => element.style.setProperty(name, String(value(expr))));
    }
    const bind = parse(element.getAttribute("data-hs-bind")) as Bind | undefined;
    if (bind !== undefined) effects.push(() => this.#writeControl(element, bind, ctx));

    const watched = parse(element.getAttribute("data-hs-effect"));
    if (Array.isArray(watched)) {
      effects.push(() => {
        const last = this.#watched.get(element) ?? [];
        const now = (watched as [unknown, unknown][]).map(([deps]) => JSON.stringify(value(deps)) ?? "null");
        this.#watched.set(element, now);
        now.forEach((key, i) => {
          if (last[i] !== key) this.#run((watched as [unknown, unknown][])[i]![1], ctx);
        });
      });
    }

    if (element instanceof HTMLTemplateElement) {
      const each = parse(element.getAttribute("data-hs-each"));
      if (Array.isArray(each)) effects.push(() => this.#renderList(element, each[0], each[1] as number, scope));
      const cond = parse(element.getAttribute("data-hs-if"));
      if (cond !== undefined) effects.push(() => this.#renderIf(element, Boolean(value(cond)), scope));
    }
    return effects;
  }

  #renderList(template: HTMLTemplateElement, place: unknown, depth: number, scope: readonly Slot[]): void {
    const proto = template.content.firstElementChild;
    const parent = template.parentNode;
    if (proto === null || parent === null) return;
    const ctx: Context = { store: this.store, scope };
    const path = resolve(place, ctx);
    const items = this.store.get(path);
    const keyAst = parse(proto.getAttribute("data-hs-key"));
    const previous = this.#lists.get(template) ?? new Map<string, Instance>();
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
      let created = false;
      if (instance === undefined || !instance.element.isConnected) {
        instance = { element: proto.cloneNode(true) as Element, slot: itemScope[depth]! };
        this.#instanceRoots.add(instance.element);
        created = true;
      } else {
        instance.slot.path = itemPath;
      }
      next.set(key, instance);
      if (anchor.nextSibling !== instance.element) parent.insertBefore(instance.element, anchor.nextSibling);
      if (created) this.scan(instance.element, [...scope.slice(0, depth), instance.slot]);
      anchor = instance.element;
    });

    for (const [key, instance] of previous) {
      if (next.get(key) !== instance) instance.element.remove();
    }
    this.#lists.set(template, next);
  }

  #renderIf(template: HTMLTemplateElement, visible: boolean, scope: readonly Slot[]): void {
    const current = this.#mounted.get(template);
    if (visible && (current === undefined || !current.isConnected)) {
      const proto = template.content.firstElementChild;
      if (proto === null || template.parentNode === null) return;
      const element = proto.cloneNode(true) as Element;
      this.#mounted.set(template, element);
      this.#instanceRoots.add(element);
      template.after(element);
      this.scan(element, scope);
    } else if (!visible && current !== undefined) {
      current.remove();
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

  #render(): void {
    for (const [element, effects] of this.#effects) {
      if (!element.isConnected) {
        this.#effects.delete(element);
        continue;
      }
      for (const effect of effects) {
        try {
          effect();
        } catch (error) {
          console.error("hypersig:", error);
        }
      }
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

  #every(element: Element, ms: number, stmt: unknown): void {
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

  #run(ast: unknown, ctx: Context): void {
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
