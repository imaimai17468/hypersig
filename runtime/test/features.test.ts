import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
import { Runtime } from "../src/runtime.ts";

/** Mounts a fixture written by `crates/hypersig/tests/fixtures.rs`. */
function mount(name: string): Runtime {
  const path = resolve(__dirname, `fixtures/${name}.json`);
  const { html, signals } = JSON.parse(readFileSync(path, "utf8")) as { html: string; signals: Record<string, unknown> };
  const root = document.createElement("div");
  root.innerHTML = html;
  document.body.replaceChildren(root);
  return new Runtime(root, signals);
}

const tick = () => new Promise((resolve) => setTimeout(resolve, 0));
const $ = <E extends Element = HTMLElement>(selector: string) => document.querySelector<E>(selector)!;
const $$ = (selector: string) => [...document.querySelectorAll<HTMLElement>(selector)];

/** The store paths `runtime` reads from now on, whatever name it records each read under. */
function spyReads(runtime: Runtime): string[] {
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

function type(input: HTMLInputElement | HTMLSelectElement, value: string, event = "input"): void {
  input.value = value;
  input.dispatchEvent(new Event(event, { bubbles: true }));
}

function key(target: Element, name: string): void {
  target.dispatchEvent(new KeyboardEvent("keydown", { key: name, bubbles: true, cancelable: true }));
}

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

describe("lists", () => {
  it("renders items from a list signal and adds one on Enter", async () => {
    const runtime = mount("todos");
    expect($$("#list li").map((li) => li.textContent)).toEqual(["tea×"]);
    expect($("#left").textContent).toBe("1");

    type($<HTMLInputElement>("#draft"), "  milk ");
    key($("#draft"), "Enter");
    await tick();
    expect(runtime.store.get("todos")).toEqual([
      { id: 1, title: "tea", done: false },
      { id: 2, title: "milk", done: false },
    ]);
    expect($$("#list li label").map((label) => label.textContent)).toEqual(["tea", "milk"]);
    expect($<HTMLInputElement>("#draft").value).toBe("");
    expect($("#left").textContent).toBe("2");
  });

  it("binds a field of each item and keeps elements by key when an item before them goes", async () => {
    const runtime = mount("todos");
    runtime.store.set("todos", [
      { id: 1, title: "a", done: false },
      { id: 2, title: "b", done: false },
    ]);
    await tick();
    const second = $$("#list li")[1]!;
    const box = second.querySelector<HTMLInputElement>("input[type=checkbox]")!;
    box.checked = true;
    box.dispatchEvent(new Event("change", { bubbles: true }));
    await tick();
    expect(runtime.store.get("todos.1.done")).toBe(true);
    expect(second.classList.contains("done")).toBe(true);
    expect($("#clear").style.display).toBe("");

    $$("#list li .remove")[0]!.click();
    await tick();
    expect($$("#list li")).toEqual([second]);
    second.querySelector<HTMLInputElement>("input[type=checkbox]")!.click();
    await tick();
    expect(runtime.store.get("todos")).toEqual([{ id: 2, title: "b", done: false }]);
  });

  it("clears completed items and toggles all", async () => {
    const runtime = mount("todos");
    const all = $<HTMLInputElement>("#all");
    expect(all.checked).toBe(false);
    all.dispatchEvent(new Event("change", { bubbles: true }));
    await tick();
    expect(runtime.store.get("todos.0.done")).toBe(true);
    expect(all.checked).toBe(true);
    $("#clear").click();
    await tick();
    expect(runtime.store.get("todos")).toEqual([]);
    expect($$("#list li")).toEqual([]);
  });

  it("mounts an edit field on double click, focuses it, and removes it on Escape", async () => {
    const runtime = mount("todos");
    expect($$("#list input.edit")).toEqual([]);
    $("#list label").dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
    await tick();
    const edit = $<HTMLInputElement>("#list input.edit");
    expect(edit.value).toBe("tea");
    expect(document.activeElement).toBe(edit);
    type(edit, "green tea");
    expect(runtime.store.get("todos.0.title")).toBe("green tea");
    key(edit, "Escape");
    await tick();
    expect($$("#list input.edit")).toEqual([]);
    expect($("#list label").textContent).toBe("green tea");
  });

  it("keeps the list items and the field being edited when a server patch carries only the list's template", async () => {
    const runtime = mount("todos");
    $("#list label").dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
    await tick();
    const item = $("#list li");
    const edit = $<HTMLInputElement>("#list input.edit");
    edit.value = "green";
    // A fragment the server renders holds a list as its template alone, with no items.
    const fragment = $("#list").cloneNode(true) as Element;
    fragment.replaceChildren(fragment.querySelector("template")!);
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html: fragment.outerHTML }) });
    await tick();
    expect($("#list li")).toBe(item);
    expect(document.activeElement).toBe(edit);
    expect(edit.value).toBe("green");
    runtime.store.set("todos.0.title", "oolong");
    await tick();
    expect($$("#list li label").map((label) => label.textContent)).toEqual(["oolong"]);
  });
});

describe("controls", () => {
  it("binds numbers, enum selects, and radios", async () => {
    const runtime = mount("controls");
    expect($<HTMLInputElement>("#quantity").value).toBe("1");
    expect($<HTMLSelectElement>("#size").value).toBe('"Small"');
    expect($<HTMLInputElement>("#small").checked).toBe(true);

    type($<HTMLInputElement>("#quantity"), "3.7");
    expect(runtime.store.get("quantity")).toBe(3);
    type($<HTMLInputElement>("#price"), "1.25");
    expect(runtime.store.get("price")).toBe(1.25);

    type($<HTMLSelectElement>("#size"), '"Large"', "change");
    await tick();
    expect(runtime.store.get("size")).toBe("Large");
    expect($<HTMLInputElement>("#large").checked).toBe(true);
    expect($("#total").textContent).toBe("large");

    const small = $<HTMLInputElement>("#small");
    small.checked = true;
    small.dispatchEvent(new Event("change", { bubbles: true }));
    await tick();
    expect($<HTMLSelectElement>("#size").value).toBe('"Small"');
  });

  it("binds attributes, boolean attributes, and styles", async () => {
    const runtime = mount("controls");
    const buy = $<HTMLButtonElement>("#buy");
    expect(buy.hasAttribute("disabled")).toBe(false);
    expect(buy.getAttribute("title")).toBe("5.00");
    expect(buy.style.getPropertyValue("width")).toBe("1rem");
    runtime.store.set("quantity", 0);
    await tick();
    expect(buy.hasAttribute("disabled")).toBe(true);
    expect(buy.style.getPropertyValue("width")).toBe("0rem");
  });
});

describe("roving focus", () => {
  it("moves focus between the items of its own group, skips a disabled item, and wraps", async () => {
    const runtime = mount("roving");
    await tick();
    expect($("#tab0").getAttribute("tabindex")).toBe("0");
    expect($("#tab1").getAttribute("tabindex")).toBe("-1");

    $("#tab0").focus();
    key($("#tab0"), "ArrowRight");
    expect(document.activeElement?.id).toBe("tab1");
    await tick();
    expect(runtime.store.get("tab")).toBe(1);
    expect($("#tab1").getAttribute("tabindex")).toBe("0");

    key($("#tab1"), "ArrowRight");
    expect(document.activeElement?.id).toBe("tab3");
    key($("#tab3"), "ArrowLeft");
    expect(document.activeElement?.id).toBe("tab1");
    key($("#tab1"), "End");
    expect(document.activeElement?.id).toBe("tab3");
    key($("#tab3"), "Home");
    expect(document.activeElement?.id).toBe("tab0");
    key($("#tab0"), "ArrowLeft");
    expect(document.activeElement?.id).toBe("tab3");
    key($("#tab3"), "ArrowUp");
    expect(document.activeElement?.id).toBe("tab3");
  });

  it("leaves a key to the handler that took it and to a held modifier", () => {
    const runtime = mount("roving");
    $("#tab3").focus();
    key($("#tab3"), "ArrowRight");
    expect(document.activeElement?.id).toBe("tab3");
    expect(runtime.store.get("taken")).toBe(1);

    $("#tab0").focus();
    $("#tab0").dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowRight", altKey: true, bubbles: true, cancelable: true }));
    expect(document.activeElement?.id).toBe("tab0");
  });

  it("gives a nested group its own items and keys", () => {
    mount("roving");
    $("#item0").focus();
    key($("#item0"), "ArrowDown");
    expect(document.activeElement?.id).toBe("item1");
    key($("#item1"), "ArrowDown");
    expect(document.activeElement?.id).toBe("item0");
    key($("#item0"), "ArrowRight");
    expect(document.activeElement?.id).toBe("item0");
  });
});

describe("node refs", () => {
  it("hides an open popover and leaves a closed one alone", async () => {
    mount("roving");
    const menu = $("#menu");
    const hide = vi.fn();
    menu.hidePopover = hide;
    let open = false;
    const matches = menu.matches.bind(menu);
    vi.spyOn(menu, "matches").mockImplementation((selector: string) => (selector === ":popover-open" ? open : matches(selector)));
    key($("#item0"), "Tab");
    await tick();
    expect(hide).not.toHaveBeenCalled();
    open = true;
    key($("#item0"), "Tab");
    await tick();
    expect(hide).toHaveBeenCalledOnce();
  });

  it("shows a closed popover and leaves an open one alone", async () => {
    mount("roving");
    const menu = $("#menu");
    menu.setAttribute("popover", "auto");
    const show = vi.fn();
    menu.showPopover = show;
    let open = true;
    const matches = menu.matches.bind(menu);
    vi.spyOn(menu, "matches").mockImplementation((selector: string) => (selector === ":popover-open" ? open : matches(selector)));
    key($("#item0"), "Enter");
    await tick();
    expect(show).not.toHaveBeenCalled();
    open = false;
    key($("#item0"), "Enter");
    await tick();
    expect(show).toHaveBeenCalledOnce();
  });
});

describe("timers, effects, and calls", () => {
  it("runs an interval while the element is in the document", async () => {
    vi.useFakeTimers();
    const runtime = mount("timers");
    await vi.advanceTimersByTimeAsync(350);
    expect(runtime.store.get("ticks")).toBe(3);
    $("#clock").remove();
    await vi.advanceTimersByTimeAsync(300);
    expect(runtime.store.get("ticks")).toBe(3);
  });

  it("debounces input, runs an on-signal-change handler per change, and reruns an effect from what it reads", async () => {
    vi.useFakeTimers();
    const runtime = mount("timers");
    expect(runtime.store.get("effects")).toBe(0);
    type($<HTMLInputElement>("#query"), "t");
    await vi.advanceTimersByTimeAsync(20);
    type($<HTMLInputElement>("#query"), "te");
    await vi.advanceTimersByTimeAsync(20);
    expect(runtime.store.get("searched")).toBe("");
    expect(runtime.store.get("echo")).toBe("TE");
    await vi.advanceTimersByTimeAsync(60);
    expect(runtime.store.get("searched")).toBe("te");
    expect(runtime.store.get("effects")).toBe(2);
    runtime.store.set("ticks", 99);
    await vi.advanceTimersByTimeAsync(0);
    expect(runtime.store.get("effects")).toBe(2);
  });

  it("sets the indicator during a call and the error after a failed one", async () => {
    let finish: (response: Response) => void = () => {};
    vi.stubGlobal("fetch", vi.fn(() => new Promise<Response>((resolve) => (finish = resolve))));
    vi.spyOn(console, "error").mockImplementation(() => {});
    const runtime = mount("timers");
    $("#save").click();
    await tick();
    expect(runtime.store.get("saving")).toBe(true);
    finish(new Response("no", { status: 500 }));
    await vi.waitFor(() => expect(runtime.store.get("saving")).toBe(false));
    expect(runtime.store.get("error")).toBe("500: no");
  });
});

describe("list statements and browser values", () => {
  it("creates, updates, swaps, and sorts items, and reads random numbers and the clock", async () => {
    const runtime = mount("table");
    const ids = () => (runtime.store.get("lines") as { id: number }[]).map((line) => line.id);
    $("#run").click();
    await tick();
    expect(ids()).toEqual([1, 2, 3]);
    expect(runtime.store.get("next_id")).toBe(4);
    const labels = () => $$("#lines li").map((li) => li.textContent);
    expect(labels().every((label) => label === "a" || label === "b")).toBe(true);

    const before = labels();
    $("#update").click();
    await tick();
    expect(labels()).toEqual([`${before[0]} !`, before[1], `${before[2]} !`]);

    const rows = $$("#lines li");
    $("#swap").click();
    await tick();
    expect(ids()).toEqual([3, 2, 1]);
    expect($$("#lines li")).toEqual([rows[2], rows[1], rows[0]]);
    $("#sort").click();
    await tick();
    expect(ids()).toEqual([1, 2, 3]);
    expect($$("#lines li")).toEqual(rows);

    const lines = Array.from({ length: 10 }, (_, index) => ({ id: index + 1, label: "a" }));
    runtime.store.set("lines", lines);
    await tick();
    const ten = $$("#lines li");
    const moves = vi.spyOn($("#lines"), "insertBefore");
    runtime.store.set("lines", lines.map((line, index) => lines[index === 1 ? 8 : index === 8 ? 1 : index]!));
    await tick();
    expect($$("#lines li")).toEqual(ten.map((_, index) => ten[index === 1 ? 8 : index === 8 ? 1 : index]));
    expect(moves).toHaveBeenCalledTimes(2);

    $("#stamp").click();
    expect(Math.abs((runtime.store.get("stamp") as number) - Date.now())).toBeLessThan(1000);
  });
});

describe("nested lists and direct events", () => {
  it("resolves item fields and counts one list deep inside another", async () => {
    const runtime = mount("nested");
    expect($$("#rows button").map((b) => b.textContent)).toEqual(["a/x", "a/y", "b/z"]);
    expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "0"]);

    $$("#rows button")[2]!.click();
    await tick();
    expect(runtime.store.get("rows.1.tags.0.on")).toBe(true);
    expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "1"]);
    expect($$("#rows button")[2]!.classList.contains("on")).toBe(true);

    const [first, second] = runtime.store.get("rows") as unknown[];
    runtime.store.set("rows", [second, first]);
    await tick();
    expect($$("#rows button").map((b) => b.textContent)).toEqual(["b/z", "a/x", "a/y"]);
    $$("#rows button")[2]!.click();
    await tick();
    expect(runtime.store.get("rows.1.tags.1.on")).toBe(true);
    expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "2"]);
  });

  it("runs mouseenter and mouseleave on the element itself, and focus on focusin", async () => {
    const runtime = mount("nested");
    const row = $$("#rows > li")[1]!;
    row.dispatchEvent(new MouseEvent("mouseenter"));
    expect(runtime.store.get("hovered")).toBe("b");
    row.querySelector("button")!.dispatchEvent(new MouseEvent("mouseenter"));
    expect(runtime.store.get("hovered")).toBe("b");
    row.dispatchEvent(new MouseEvent("mouseleave"));
    expect(runtime.store.get("hovered")).toBe("");

    $("#field input").dispatchEvent(new FocusEvent("focusin", { bubbles: true }));
    expect(runtime.store.get("focused")).toBe(true);
    $("#field input").dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
    expect(runtime.store.get("focused")).toBe(false);
  });
});

describe("dependency tracking", () => {
  it("does not recheck list keys when a field outside the key changes", async () => {
    const runtime = mount("todos");
    runtime.store.set("todos", [
      { id: 1, title: "a", done: false },
      { id: 2, title: "b", done: false },
    ]);
    await tick();
    const get = runtime.store.get.bind(runtime.store);
    const reads = spyReads(runtime);
    $$("#list li input[type=checkbox]")[1]!.click();
    await tick();
    expect(runtime.store.get("todos.1.done")).toBe(true);
    expect(reads.filter((path) => path.endsWith(".id"))).toEqual([]);
    expect($("#left").textContent).toBe("1");

    reads.length = 0;
    runtime.store.set("todos.0.id", 7);
    await tick();
    expect(reads).toEqual(expect.arrayContaining(["todos.0.id", "todos.1.id"]));
    $$("#list li .remove")[0]!.click();
    await tick();
    expect($$("#list li label").map((label) => label.textContent)).toEqual(["b"]);
  });


  it("does not rerun the bindings of the other items when an item is pushed", async () => {
    const runtime = mount("todos");
    runtime.store.set("todos", [
      { id: 1, title: "a", done: false },
      { id: 2, title: "b", done: false },
    ]);
    await tick();
    const get = runtime.store.get.bind(runtime.store);
    const reads = spyReads(runtime);
    runtime.store.set("todos", [...(get("todos") as unknown[]), { id: 3, title: "c", done: false }]);
    await tick();
    expect($$("#list li label").map((label) => label.textContent)).toEqual(["a", "b", "c"]);
    expect(reads.filter((path) => /^todos\.[01]\.(id|title)$/.test(path))).toEqual([]);
    expect(reads.filter((path) => path.endsWith(".title"))).toEqual(["todos.2.title"]);

    runtime.store.set("todos.0.id", 9);
    await tick();
    runtime.store.set("todos", (get("todos") as { id: number }[]).slice(1));
    await tick();
    expect($$("#list li label").map((label) => label.textContent)).toEqual(["b", "c"]);
  });

  it("reruns only the bindings that read a changed signal", async () => {
    const runtime = mount("todos");
    runtime.store.set("todos", [
      { id: 1, title: "a", done: false },
      { id: 2, title: "b", done: false },
    ]);
    await tick();
    const labels = $$("#list label");
    const written = new Set<Node>();
    const observer = new MutationObserver((records) => {
      for (const record of records) written.add(record.target);
    });
    for (const label of labels) observer.observe(label, { childList: true, characterData: true, subtree: true });
    runtime.store.set("draft", "typing");
    await tick();
    expect(written.size).toBe(0);
    runtime.store.set("todos.1.title", "B");
    await tick();
    expect(labels.map((label) => label.textContent)).toEqual(["a", "B"]);
    expect([...written]).toEqual([labels[1]]);
    observer.disconnect();
  });
});

/** Loads a document written by `Page::render`, without starting the runtime. */
function serve(name: string): void {
  const path = resolve(__dirname, `fixtures/${name}.json`);
  const { document: html } = JSON.parse(readFileSync(path, "utf8")) as { document: string };
  const parsed = new DOMParser().parseFromString(html, "text/html");
  document.body.replaceChildren(...parsed.body.childNodes);
  for (const old of document.head.querySelectorAll("style[data-hs-css]")) old.remove();
  document.head.append(...parsed.head.querySelectorAll("style[data-hs-css]"));
  for (const { name } of [...document.body.attributes]) document.body.removeAttribute(name);
  for (const { name, value } of [...parsed.body.attributes]) document.body.setAttribute(name, value);
}

function start(): Runtime {
  return new Runtime(document.body, JSON.parse(document.body.getAttribute("data-hs-signals")!) as Record<string, unknown>);
}

describe("server rendering", () => {
  it("shows list items, counts, and visibility before the runtime starts, and the runtime adopts them", async () => {
    serve("todos_page");
    const item = $("#list li");
    expect($$("#list li label").map((label) => label.textContent)).toEqual(["tea"]);
    expect($("#left").textContent).toBe("1");
    expect($("#clear").style.display).toBe("none");

    const runtime = start();
    await tick();
    expect($$("#list li")).toEqual([item]);
    item.querySelector<HTMLInputElement>("input[type=checkbox]")!.click();
    await tick();
    expect(runtime.store.get("todos.0.done")).toBe(true);
    expect(item.classList.contains("done")).toBe(true);
    expect($("#left").textContent).toBe("0");

    item.querySelector("label")!.dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
    await tick();
    expect(item.querySelector<HTMLInputElement>("input.edit")!.value).toBe("tea");
  });

  it("registers the bindings of server-rendered items in a later task and shows changes made before it", async () => {
    serve("todos_page");
    const item = $("#list li");
    const runtime = start();
    runtime.store.set("todos.0.done", true);
    await Promise.resolve();
    expect(item.classList.contains("done")).toBe(false);
    await tick();
    expect(item.classList.contains("done")).toBe(true);
    expect($$("#list li")).toEqual([item]);
  });

  it("registers a waiting item's bindings at once when an event reaches it", async () => {
    serve("todos_page");
    const item = $("#list li");
    const runtime = start();
    item.querySelector<HTMLInputElement>("input[type=checkbox]")!.click();
    expect(runtime.store.get("todos.0.done")).toBe(true);
    await Promise.resolve();
    expect(item.classList.contains("done")).toBe(true);

    serve("nested_page");
    const nested = start();
    $$("#rows button")[2]!.click();
    expect(nested.store.get("rows.1.tags.0.on")).toBe(true);
  });

  it("adopts nested list items with their own item scope", async () => {
    serve("nested_page");
    expect($$("#rows button").map((b) => b.textContent)).toEqual(["a/x", "a/y", "b/z"]);
    expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "0"]);
    expect($$("#rows button")[0]!.classList.contains("on")).toBe(true);
    const buttons = $$("#rows button");

    const runtime = start();
    await tick();
    expect($$("#rows button")).toEqual(buttons);
    buttons[2]!.click();
    await tick();
    expect(runtime.store.get("rows.1.tags.0.on")).toBe(true);
    expect($$("#rows .count").map((s) => s.textContent)).toEqual(["1", "1"]);
  });

  it("writes form values, attributes, and styles on the server", () => {
    serve("controls_page");
    expect($<HTMLInputElement>("#quantity").value).toBe("1");
    expect($<HTMLInputElement>("#price").value).toBe("2.5");
    expect($<HTMLSelectElement>("#size").value).toBe('"Small"');
    expect($<HTMLInputElement>("#small").checked).toBe(true);
    expect($<HTMLInputElement>("#large").checked).toBe(false);
    expect($("#buy").getAttribute("title")).toBe("5.00");
    expect($("#buy").style.getPropertyValue("width")).toBe("1rem");
    expect($("#total").textContent).toBe("small");
  });
});

describe("computed signals", () => {
  it("are computed on the server, recomputed in order in the browser, and rerun readers only when their value changes", async () => {
    serve("computed_page");
    expect($("#sum").textContent).toBe("3");
    expect($("#doubled").textContent).toBe("6");
    const runtime = start();
    await tick();

    runtime.store.set("a", 5);
    await vi.waitFor(() => expect($("#doubled").textContent).toBe("14"));
    expect($("#sum").textContent).toBe("7");

    const written = new Set<Node>();
    const observer = new MutationObserver((records) => {
      for (const record of records) written.add(record.target);
    });
    observer.observe($("#doubled"), { childList: true, characterData: true, subtree: true });
    runtime.store.set("a", 4);
    runtime.store.set("b", 3);
    await tick();
    await tick();
    expect(runtime.store.get("sum")).toBe(7);
    expect(written.size).toBe(0);
    observer.disconnect();
  });
});

describe("atomic css", () => {
  /** The one event in a stream fixture written by `Patches::to_event_stream`. */
  function patch(name: string): { event: string; data: string } {
    const path = resolve(__dirname, `fixtures/${name}.json`);
    const { stream } = JSON.parse(readFileSync(path, "utf8")) as { stream: string };
    const [event, data] = stream.trim().split("\n");
    return { event: event!.slice("event: ".length), data: data!.slice("data: ".length) };
  }

  const buckets = () => [...document.head.querySelectorAll<HTMLStyleElement>("style[data-hs-css]")].map((style) => style.dataset.hsCss);

  it("switches a toggled style's classes with its condition", async () => {
    serve("styled_page");
    start();
    const box = $("#box");
    expect(box.className).toBe("md:width_50%");
    box.click();
    await tick();
    expect(box.classList.contains("when:color_crimson")).toBe(true);
    box.click();
    await tick();
    expect(box.classList.contains("when:color_crimson")).toBe(false);
  });

  it("puts a patch's rule buckets at their place in the order", async () => {
    serve("styled_page");
    const runtime = start();
    expect(buckets()).toEqual(["0 k01-----00", "1 k-----00"]);
    runtime.apply(patch("styled_patch"));
    await tick();
    expect(buckets()).toEqual(["0 k-----00", "0 k-----01", "0 k----4019-00", "0 k01-----00", "1 k-----00", "2 pulse"]);
    expect($("#slot").className).toBe("width_100% padding_4px animation-name_pulse hover:color_blue");
    const base = document.head.querySelector<HTMLStyleElement>('style[data-hs-css="0 k-----00"]')!;
    const once = base.textContent;
    runtime.apply(patch("styled_patch"));
    await tick();
    expect(base.textContent).toBe(once);
    expect(buckets()).toHaveLength(6);
  });
});
