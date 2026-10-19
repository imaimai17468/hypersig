import { vi } from "vitest";
import type { Context, Host } from "../../src/eval.ts";
import { Store } from "../../src/store.ts";

export function context(values: Record<string, unknown>): Context {
  return { store: new Store(values), scope: [] };
}

/** The signals the expressions of the tests read, which no expression writes. */
export const ctx = context({ count: 3, name: " Tea ", panel: { open: true }, filter: "Active" });

export function host() {
  return {
    call: vi.fn<Host["call"]>(() => Promise.resolve()),
    dom: vi.fn<Host["dom"]>(),
    clearFiles: vi.fn<Host["clearFiles"]>(),
    navigate: vi.fn<Host["navigate"]>(),
    history: vi.fn<Host["history"]>(),
    debounce: vi.fn<Host["debounce"]>(),
  } satisfies Host;
}
