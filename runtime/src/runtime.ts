import { evaluate, run } from "./eval.ts";
import { morph } from "./morph.ts";
import { readEventStream, type ServerEvent } from "./sse.ts";
import { Store, type Values } from "./store.ts";

const EVENTS = ["click", "input", "change", "submit"] as const;
const REACTIVE = "[data-hs-show],[data-hs-text],[data-hs-bind],[data-hs-class],[data-hs-on-load]";

type Effect = () => void;

/** Connects the signals on `<body>` and the `data-hs-*` attributes of `root`. */
export class Runtime {
  readonly store: Store;
  readonly #root: Element;
  readonly #effects = new Map<Element, Effect[]>();
  readonly #loaded = new WeakSet<Element>();

  constructor(root: Element, initial: Values) {
    this.#root = root;
    this.store = new Store(initial);
    this.store.subscribe(() => this.#render());
    for (const type of EVENTS) {
      root.addEventListener(type, (event) => this.#onEvent(type, event));
    }
    this.scan(root);
    this.#render();
  }

  /** Registers the reactive attributes of `root` and every element below it. */
  scan(root: Element): void {
    const elements = root.matches(REACTIVE) ? [root] : [];
    elements.push(...root.querySelectorAll(REACTIVE));
    for (const element of elements) {
      this.#effects.set(element, this.#effectsFor(element));
      const load = element.getAttribute("data-hs-on-load");
      if (load !== null && !this.#loaded.has(element)) {
        this.#loaded.add(element);
        this.#runAttribute(load);
      }
    }
  }

  /** Calls an action with the current signals and applies the patches it streams back. */
  readonly call = async (method: string, path: string, args: unknown): Promise<void> => {
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
    const response = await fetch(url, init);
    if (!response.ok || response.body === null) {
      const detail = await response.text();
      this.#root.dispatchEvent(
        new CustomEvent("hs-error", { detail: { status: response.status, message: detail } }),
      );
      console.error(`hypersig: ${method} ${path} answered ${response.status}: ${detail}`);
      return;
    }
    await readEventStream(response.body, (event) => this.apply(event));
  };

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
        this.scan(morph(target, next));
        this.#render();
        return;
      }
      default:
        console.warn(`hypersig: ignored server event "${event.event}"`);
    }
  }

  #effectsFor(element: Element): Effect[] {
    const effects: Effect[] = [];
    const show = parse(element.getAttribute("data-hs-show"));
    if (show !== undefined && element instanceof HTMLElement) {
      effects.push(() => {
        element.style.display = evaluate(show, this.store) ? "" : "none";
      });
    }
    const text = parse(element.getAttribute("data-hs-text"));
    if (text !== undefined) {
      effects.push(() => {
        const value = String(evaluate(text, this.store));
        if (element.textContent !== value) element.textContent = value;
      });
    }
    const classes = parse(element.getAttribute("data-hs-class"));
    if (Array.isArray(classes)) {
      for (const [name, when] of classes as [string, unknown][]) {
        effects.push(() => {
          element.classList.toggle(name, Boolean(evaluate(when, this.store)));
        });
      }
    }
    const bind = element.getAttribute("data-hs-bind");
    if (bind !== null && element instanceof HTMLInputElement) {
      effects.push(() => {
        const value = this.store.get(bind);
        if (typeof value === "boolean") {
          if (element.checked !== value) element.checked = value;
        } else {
          const text = value === undefined || value === null ? "" : String(value);
          if (element.value !== text) element.value = text;
        }
      });
    }
    return effects;
  }

  #render(): void {
    for (const [element, effects] of this.#effects) {
      if (!element.isConnected) {
        this.#effects.delete(element);
        continue;
      }
      for (const effect of effects) effect();
    }
  }

  #onEvent(type: (typeof EVENTS)[number], event: Event): void {
    const target = event.target;
    if (!(target instanceof Element)) return;
    if ((type === "input" || type === "change") && target instanceof HTMLInputElement) {
      const bind = target.getAttribute("data-hs-bind");
      if (bind !== null) {
        const current = this.store.get(bind);
        this.store.set(bind, typeof current === "boolean" ? target.checked : target.value);
      }
    }
    const attribute = `data-hs-on-${type}`;
    const handler = target.closest(`[${attribute}]`);
    if (handler === null || !this.#root.contains(handler)) return;
    if (type === "submit") event.preventDefault();
    const stmt = handler.getAttribute(attribute);
    if (stmt !== null) this.#runAttribute(stmt);
  }

  #runAttribute(source: string): void {
    run(JSON.parse(source), this.store, this.call).catch((error: unknown) => {
      console.error("hypersig:", error);
    });
  }
}

function parse(source: string | null): unknown {
  return source === null ? undefined : JSON.parse(source);
}
