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

  it("debounces input and reruns an effect when its dependency changes", async () => {
    vi.useFakeTimers();
    const runtime = mount("timers");
    expect(runtime.store.get("effects")).toBe(1);
    type($<HTMLInputElement>("#query"), "t");
    await vi.advanceTimersByTimeAsync(20);
    type($<HTMLInputElement>("#query"), "te");
    await vi.advanceTimersByTimeAsync(20);
    expect(runtime.store.get("searched")).toBe("");
    await vi.advanceTimersByTimeAsync(60);
    expect(runtime.store.get("searched")).toBe("te");
    expect(runtime.store.get("effects")).toBe(3);
  });

  it("sets the pending flag during a call and the error after a failed one", async () => {
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
