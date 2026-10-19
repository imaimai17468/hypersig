import type { Runtime } from "../runtime.ts";
import { Attributes } from "./binding/attribute.ts";
import { Computed } from "./binding/computed.ts";
import { Controls } from "./binding/control.ts";
import { Events } from "./binding/event.ts";
import { Refs } from "./binding/ref.ts";
import { Statements } from "./binding/statement.ts";
import { upgradeTemplates } from "./binding/template.ts";
import { Timers } from "./binding/timer.ts";

/** What ties the page's `data-hs-*` attributes to the store: bindings, handlers, statements, timers, and node refs. */
export class Bindings {
  readonly attributes: Attributes;
  readonly statements: Statements;
  readonly controls: Controls;
  readonly timers: Timers;
  readonly events: Events;
  readonly computed: Computed;
  readonly refs: Refs;

  constructor(rt: Runtime) {
    this.attributes = new Attributes(rt);
    this.statements = new Statements(rt);
    this.controls = new Controls(rt);
    this.timers = new Timers(rt);
    this.events = new Events(rt);
    this.computed = new Computed(rt);
    this.refs = new Refs(rt);
  }

  /** Listens on `root`, registers the computed signals and the title on it, then every binding inside it. */
  start(root: Element): void {
    this.events.listen(root);
    this.computed.register(root);
    upgradeTemplates(root);
    this.attributes.scan(root, []);
  }
}
