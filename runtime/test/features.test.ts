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

  it("rebuilds list items after a server patch replaces the fragment around them", async () => {
    const runtime = mount("todos");
    const html = $("#list").outerHTML;
    const list = html.slice(0, html.lastIndexOf("</template>") + "</template>".length) + "</ul>";
    runtime.apply({ event: "hs-element", data: JSON.stringify({ html: list }) });
    await tick();
    expect($$("#list li label").map((label) => label.textContent)).toEqual(["tea"]);
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
  document.body.setAttribute("data-hs-signals", parsed.body.getAttribute("data-hs-signals")!);
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
