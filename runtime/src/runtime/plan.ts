import { NAMES } from "./binding/ref.ts";
import { data, REACTIVE } from "./spec.ts";

/** Where the elements that carry an attribute sit inside a template's element, by element child index. */
export interface Plan {
  readonly tag: string;
  readonly source: Element | null;
  /** Whether `source` carries a reactive attribute, rather than a node ref alone. */
  readonly reactive: boolean;
  readonly children: readonly (readonly [number, Plan])[];
}

const plans = new WeakMap<Element, Plan>();

export function planOf(proto: Element): Plan {
  let plan = plans.get(proto);
  if (plan === undefined) {
    plan = build(proto) ?? { tag: proto.tagName, source: null, reactive: false, children: [] };
    plans.set(proto, plan);
  }
  return plan;
}

function build(element: Element): Plan | null {
  const children: [number, Plan][] = [];
  Array.from(element.children).forEach((child, index) => {
    const plan = build(child);
    if (plan !== null) children.push([index, plan]);
  });
  const reactive = element.matches(REACTIVE);
  const carries = reactive || NAMES.some((attribute) => element.hasAttribute(attribute));
  return carries || children.length > 0
    ? { tag: element.tagName, source: carries ? element : null, reactive, children }
    : null;
}

/**
 * Calls `found` with each element of `copy` that `plan` names, and the
 * template element it was built from. Where the server `rendered` `copy`, a
 * child that carries `data-hs-ssr` was rendered for a nested list or
 * condition, has no counterpart in the template, and is skipped when
 * counting positions, and a child whose tag differs is left out. A clone of
 * the template, not yet rendered into, has the template's children only.
 */
export function locate(
  plan: Plan,
  copy: Element,
  rendered: boolean,
  found: (element: Element, source: Element, reactive: boolean) => void,
): void {
  if (plan.source !== null) found(copy, plan.source, plan.reactive);
  if (plan.children.length === 0) return;
  // Walks the siblings, because reading `children` makes the element keep an HTMLCollection.
  let at = copy.firstElementChild;
  let position = 0;
  for (const [index, child] of plan.children) {
    for (; at !== null; at = at.nextElementSibling) {
      if (rendered && data(at).hsSsr !== undefined) continue;
      if (position++ === index) break;
    }
    if (at === null) return;
    if (!rendered || at.tagName === child.tag) locate(child, at, rendered, found);
    at = at.nextElementSibling;
  }
}

/** Calls `visit` with each element of `plan` that carries a reactive attribute, in the order `locate` finds them. */
export function sources(plan: Plan, visit: (source: Element) => void): void {
  if (plan.source !== null && plan.reactive) visit(plan.source);
  for (const [, child] of plan.children) sources(child, visit);
}

/** Whether `proto` or an element inside it carries a reactive attribute. */
export function bound(proto: Element): boolean {
  let found = false;
  sources(planOf(proto), () => {
    found = true;
  });
  return found;
}
