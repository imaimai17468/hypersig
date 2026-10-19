import { Runtime } from "../../src/runtime.ts";

/** The signature of `fetch` as the runtime calls it. */
export type Fetch = (url: URL, init: RequestInit) => Promise<Response>;

export function mount(html: string, signals: Record<string, unknown>): Runtime {
  const root = document.createElement("main");
  root.innerHTML = html;
  document.body.replaceChildren(root);
  return new Runtime(root, signals);
}

/** The store paths `runtime` reads from now on, whatever name it records each read under. */
export function spyReads(runtime: Runtime): string[] {
  const reads: string[] = [];
  const { store } = runtime;
  const get = store.get.bind(store);
  const getAs = store.getAs.bind(store);
  store.get = (path: string) => {
    reads.push(path);
    return get(path);
  };
  store.getAs = (path: string, read: string) => {
    reads.push(path);
    return getAs(path, read);
  };
  const getIn = store.getIn.bind(store);
  store.getIn = (base: string, field: string, read: string) => {
    reads.push(field === "" ? base : `${base}.${field}`);
    return getIn(base, field, read);
  };
  return reads;
}

export function sseResponse(body: string): Response {
  return new Response(body, { status: 200, headers: { "content-type": "text/event-stream" } });
}

/** The bindings the runtime keeps for `element`, under the symbol `hypersig`. */
export function bound(element: Element): { effects?: unknown[]; watch?: unknown; row?: unknown } {
  const key = Object.getOwnPropertySymbols(element).find(
    (symbol) => symbol.description === "hypersig",
  )!;
  return (
    element as unknown as Record<symbol, { effects?: unknown[]; watch?: unknown; row?: unknown }>
  )[key]!;
}
