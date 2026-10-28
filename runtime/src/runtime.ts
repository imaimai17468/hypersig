import type { CallOptions, Host } from "./eval.ts";
import type { ServerEvent } from "./sse.ts";
import { Store } from "./store.ts";
import { type Values } from "./store/value.ts";
import { Actions } from "./runtime/action.ts";
import { Bindings } from "./runtime/binding.ts";
import { Conditions } from "./runtime/condition.ts";
import { Lists } from "./runtime/list.ts";
import { Navigation } from "./runtime/navigation.ts";
import { Render } from "./runtime/render.ts";

/**
 * Connects the signals on `<body>` and the `data-hs-*` attributes of `root`.
 * Each part of the work is an object of its own, and they reach each other
 * through this one.
 */
export class Runtime implements Host {
  readonly root: Element;
  readonly store: Store;
  readonly render = new Render(this);
  readonly bindings = new Bindings(this);
  readonly lists = new Lists(this);
  readonly conditions = new Conditions(this);
  readonly actions = new Actions(this);
  readonly navigation = new Navigation();

  constructor(root: Element, initial: Values) {
    this.root = root;
    this.store = new Store(initial);
    this.store.subscribe((changes) => this.render.run(changes));
    this.store.derive((changes) => this.bindings.computed.derive(changes));
    this.bindings.start(root);
    this.render.run(new Map());
  }

  /** Applies one event of an action's answer. */
  apply(event: ServerEvent): void {
    this.actions.patches.apply(event);
  }

  /** Answers Back and Forward. */
  popped(): void {
    this.navigation.popped();
  }

  call(
    method: string,
    path: string,
    args: () => unknown,
    indicator: string | null,
    error: string | null,
    options: CallOptions,
  ): Promise<void> {
    return this.actions.calls.call(method, path, args, indicator, error, options);
  }

  dom(op: string, name: string, origin: Element | undefined): void {
    this.bindings.refs.dom(op, name, origin);
  }

  clearFiles(name: string, origin: Element | undefined): void {
    this.bindings.refs.clearFiles(name, origin);
  }

  navigate(url: string): void {
    this.navigation.navigate(url);
  }

  history(url: string, replace: boolean): void {
    this.navigation.history(url, replace);
  }

  debounce(origin: Element | undefined, key: string, ms: number, action: () => void): void {
    this.bindings.timers.debounce(origin, key, ms, action);
  }

  transition(update: () => void): void {
    // The browser calls the callback once it has taken its picture of the
    // page, and takes the second picture once the callback returns, so the
    // writes render inside it rather than in the store's microtask.
    if (typeof document.startViewTransition !== "function") {
      update();
      return;
    }
    document.startViewTransition(() => {
      try {
        update();
      } catch (error) {
        console.error("hypersig:", error);
      } finally {
        this.store.flush();
      }
    });
  }
}
