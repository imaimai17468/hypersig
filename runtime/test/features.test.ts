import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
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
    await tick();
    expect($("#price-label").textContent).toBe("$1,250.00");

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

  it("binds an empty field to None in both directions for an Option signal", async () => {
    const runtime = mount("controls");
    expect($<HTMLInputElement>("#note").value).toBe("gift wrap");
    expect($<HTMLInputElement>("#limit").value).toBe("");

    type($<HTMLInputElement>("#note"), "");
    expect(runtime.store.get("note")).toBe(null);
    type($<HTMLInputElement>("#note"), "ribbon");
    expect(runtime.store.get("note")).toBe("ribbon");

    type($<HTMLInputElement>("#limit"), "12.9");
    expect(runtime.store.get("limit")).toBe(12);
    type($<HTMLInputElement>("#limit"), "  ");
    expect(runtime.store.get("limit")).toBe(null);
    type($<HTMLInputElement>("#quantity"), "");
    expect(runtime.store.get("quantity")).toBe(1);

    runtime.store.set("note", null);
    runtime.store.set("limit", 7);
    await tick();
    expect($<HTMLInputElement>("#note").value).toBe("");
    expect($<HTMLInputElement>("#limit").value).toBe("7");
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

describe("active descendant", () => {
  it("moves the signal among the shown options that are not disabled, and clicks the active one on Enter", async () => {
    const runtime = mount("combo");
    const field = $("#field");
    key(field, "ArrowDown");
    await tick();
    expect(runtime.store.get("active")).toBe("o0");
    key(field, "ArrowDown");
    await tick();
    expect(runtime.store.get("active")).toBe("o3");
    key(field, "ArrowDown");
    await tick();
    expect(runtime.store.get("active")).toBe("o0");
    key(field, "ArrowUp");
    await tick();
    expect(runtime.store.get("active")).toBe("o3");
    expect(runtime.store.get("downs")).toBe(3);
    key(field, "Enter");
    await tick();
    expect(runtime.store.get("picked")).toBe("o3");
  });

  it("starts from the last option on the up arrow and counts an option once it is shown", async () => {
    const runtime = mount("combo");
    const field = $<HTMLInputElement>("#field");
    type(field, "a");
    await tick();
    key(field, "ArrowUp");
    await tick();
    expect(runtime.store.get("active")).toBe("o3");
    key(field, "ArrowUp");
    await tick();
    expect(runtime.store.get("active")).toBe("o1");
  });

  it("leaves Enter alone while no option is active", async () => {
    const runtime = mount("combo");
    const event = new KeyboardEvent("keydown", { key: "Enter", bubbles: true, cancelable: true });
    $("#field").dispatchEvent(event);
    await tick();
    expect(event.defaultPrevented).toBe(false);
    expect(runtime.store.get("picked")).toBe("");
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
    $("#item0").dispatchEvent(new KeyboardEvent("keydown", { key: "Tab", shiftKey: true, bubbles: true, cancelable: true }));
    await tick();
    expect(hide).toHaveBeenCalledTimes(2);
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

  it("runs a timeout once, and not for an element that left the document", async () => {
    vi.useFakeTimers();
    const runtime = mount("timers");
    $("#gone").remove();
    await vi.advanceTimersByTimeAsync(350);
    expect(runtime.store.get("fired")).toBe(1);
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
    finish(new Response("You can keep 3 notes.\n", { status: 409 }));
    await vi.waitFor(() => expect(runtime.store.get("saving")).toBe(false));
    expect(runtime.store.get("error")).toBe("You can keep 3 notes.");
    expect(console.error).toHaveBeenCalledWith("hypersig: POST /search failed with 409: You can keep 3 notes.");
  });

  it("writes the status and its reason to the error when the failed answer has no body", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () => new Response(null, { status: 502, statusText: "Bad Gateway" })),
    );
    vi.spyOn(console, "error").mockImplementation(() => {});
    const runtime = mount("timers");
    $("#save").click();
    await vi.waitFor(() => expect(runtime.store.get("error")).toBe("502 Bad Gateway"));
    expect(console.error).toHaveBeenCalledWith("hypersig: POST /search failed with 502 Bad Gateway");
  });

  it("aborts the earlier latest call and applies only the newer answer", async () => {
    const answers: ((response: Response) => void)[] = [];
    const fetch = vi.fn(
      (_url: URL, init: RequestInit) =>
        new Promise<Response>((resolve, reject) => {
          init.signal?.addEventListener("abort", () => reject(new DOMException("aborted", "AbortError")));
          answers.push(resolve);
        }),
    );
    vi.stubGlobal("fetch", fetch);
    const runtime = mount("timers");
    type($<HTMLInputElement>("#find"), "t");
    type($<HTMLInputElement>("#find"), "te");
    await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(2));
    expect(fetch.mock.calls.map(([url]) => url.searchParams.get("args"))).toEqual(['"t"', '"te"']);
    expect(fetch.mock.calls[0]![1].signal!.aborted).toBe(true);
    answers[1]!(new Response('event: hs-signals\ndata: {"found":"te"}\n\n', { status: 200 }));
    answers[0]!(new Response('event: hs-signals\ndata: {"found":"t"}\n\n', { status: 200 }));
    await vi.waitFor(() => expect(runtime.store.get("finding")).toBe(false));
    expect(runtime.store.get("found")).toBe("te");
  });

  it("sends a reconnect call again after the connection fails", async () => {
    vi.useFakeTimers();
    vi.spyOn(console, "error").mockImplementation(() => {});
    const fetch = vi
      .fn()
      .mockRejectedValueOnce(new TypeError("Failed to fetch"))
      .mockResolvedValueOnce(new Response('event: hs-signals\ndata: {"found":"back"}\n\n', { status: 200 }));
    vi.stubGlobal("fetch", fetch);
    const runtime = mount("timers");
    $("#follow").click();
    await vi.advanceTimersByTimeAsync(0);
    expect(runtime.store.get("error")).toContain("Failed to fetch");
    await vi.advanceTimersByTimeAsync(1000);
    expect(fetch).toHaveBeenCalledTimes(2);
    expect(runtime.store.get("found")).toBe("back");
    expect(runtime.store.get("error")).toBe("");
  });
});

describe("call outcomes", () => {
  it("marks only the line whose call failed, with the values named when it was sent", async () => {
    const answers = new Map<string, (response: Response) => void>();
    const fetch = vi.fn(
      (_url: URL, init: RequestInit) => new Promise<Response>((resolve) => answers.set(String(init.body), resolve)),
    );
    vi.stubGlobal("fetch", fetch);
    vi.spyOn(console, "error").mockImplementation(() => {});
    const runtime = mount("outbox");
    const states = () => (runtime.store.get("letters") as { state: string }[]).map((letter) => letter.state);
    type($<HTMLInputElement>("#draft"), "first");
    $("#send").click();
    await tick();
    type($<HTMLInputElement>("#draft"), "second");
    $("#send").click();
    await vi.waitFor(() => expect(answers.size).toBe(2));
    expect(states()).toEqual(["sending", "sending"]);

    answers.get("1")!(new Response("The outbox is full.", { status: 409 }));
    answers.get("2")!(new Response("event: hs-signals\ndata: {}\n\n", { status: 200 }));
    await vi.waitFor(() => expect(states()).toEqual(["The outbox is full.", "sent"]));

    answers.clear();
    $$("#letters .edit")[0]!.click();
    expect(states()).toEqual(["sending", "sent"]);
    await vi.waitFor(() => expect(answers.size).toBe(1));
    answers.get("1")!(new Response("event: hs-signals\ndata: {}\n\n", { status: 200 }));
    await vi.waitFor(() => expect(states()).toEqual(["sent", "sent"]));
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
    $("#sort-desc").click();
    await tick();
    expect(ids()).toEqual([3, 2, 1]);
    expect($$("#lines li")).toEqual([rows[2], rows[1], rows[0]]);
    $("#sort").click();
    await tick();

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

describe("list patches", () => {
  /** The events of a stream fixture written by `crates/hypersig/tests/fixtures.rs`. */
  function events(name: string): { event: string; data: string }[] {
    const path = resolve(__dirname, `fixtures/${name}.json`);
    const { stream } = JSON.parse(readFileSync(path, "utf8")) as { stream: string };
    return stream
      .trim()
      .split("\n\n")
      .map((block) => {
        const [event, data] = block.split("\n");
        return { event: event!.slice("event: ".length), data: data!.slice("data: ".length) };
      });
  }

  const bodies = () => $$("#feed li").map((li) => li.textContent);

  it("appends, prepends, upserts, and removes items by key, keeping the elements of the items that stay", async () => {
    const runtime = mount("feed");
    await tick();
    const [one, two] = $$("#feed li");
    const [append, prepend, upsert, remove] = events("feed_patches");
    runtime.apply(append!);
    await tick();
    expect(bodies()).toEqual(["one", "two", "three"]);
    expect($$("#feed li").slice(0, 2)).toEqual([one, two]);
    runtime.apply(prepend!);
    runtime.apply(upsert!);
    await tick();
    expect(bodies()).toEqual(["zero", "one!", "two", "three", "four"]);
    expect($$("#feed li")[1]).toBe(one);
    runtime.apply(remove!);
    await tick();
    expect(bodies()).toEqual(["zero", "one!", "four"]);
    expect((runtime.store.get("notes") as { id: number }[]).map((note) => note.id)).toEqual([0, 1, 4]);
  });

  it("prepends items in the browser and scrolls the page back to the items that were in view", async () => {
    mount("feed");
    await tick();
    const [one] = $$("#feed li");
    const scroller = document.scrollingElement ?? document.documentElement;
    // The test DOM lays nothing out, so the first item reports the 40 pixels the two new items above it would push it down.
    vi.spyOn(one!, "getBoundingClientRect")
      .mockReturnValueOnce(new DOMRect(0, 0, 100, 20))
      .mockReturnValue(new DOMRect(0, 40, 100, 20));
    const scrolled = vi.spyOn(scroller, "scrollTop", "set");
    $("#older").click();
    await tick();
    expect(bodies()).toEqual(["minus one", "zero", "one", "two"]);
    expect($$("#feed li")[2]).toBe(one);
    expect(scrolled).toHaveBeenCalledWith(40);
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

  it("runs focus and blur only when focus enters or leaves the element", () => {
    const runtime = mount("nested");
    const [first, second] = $$("#field input");
    const move = (type: string, target: Element, related: Element | null) =>
      target.dispatchEvent(new FocusEvent(type, { bubbles: true, relatedTarget: related }));
    move("focusin", first!, document.body);
    expect(runtime.store.get("focused")).toBe(true);

    // From the first input to the second, as an arrow key moves between radios.
    runtime.store.set("focused", "unchanged");
    move("focusout", first!, second!);
    move("focusin", second!, first!);
    expect(runtime.store.get("focused")).toBe("unchanged");

    move("focusout", second!, document.body);
    expect(runtime.store.get("focused")).toBe(false);
    move("focusin", second!, null);
    expect(runtime.store.get("focused")).toBe(true);
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
  for (const old of document.head.querySelectorAll("style")) old.remove();
  document.head.append(...parsed.head.querySelectorAll("style"));
  for (const { name } of [...document.body.attributes]) document.body.removeAttribute(name);
  for (const { name, value } of [...parsed.body.attributes]) document.body.setAttribute(name, value);
  document.title = parsed.title;
}

function start(): Runtime {
  return new Runtime(document.body, JSON.parse(document.body.getAttribute("data-hs-signals")!) as Record<string, unknown>);
}

describe("shortcuts", () => {
  const primary = /Mac|iPhone|iPad|iPod/.test(navigator.platform) ? { metaKey: true } : { ctrlKey: true };

  function press(target: Element, key: string, init: KeyboardEventInit = {}): KeyboardEvent {
    const event = new KeyboardEvent("keydown", { key, bubbles: true, cancelable: true, ...init });
    target.dispatchEvent(event);
    return event;
  }

  it("runs a page shortcut while nothing has focus, and matches its modifiers exactly", () => {
    serve("shortcuts_page");
    const runtime = start();
    expect(press(document.body, "k", primary).defaultPrevented).toBe(true);
    expect(runtime.store.get("palette")).toBe(1);
    press(document.body, "k");
    press(document.body, "k", { ...primary, altKey: true });
    expect(runtime.store.get("palette")).toBe(1);
    press(document.body, "/");
    expect(runtime.store.get("palette")).toBe(11);
  });

  it("leaves the keys a text field types with to the field, unless the shortcut holds a modifier", () => {
    serve("shortcuts_page");
    const runtime = start();
    const field = $("#field");
    expect(press(field, "s").defaultPrevented).toBe(false);
    expect(press(field, "/").defaultPrevented).toBe(false);
    expect(runtime.store.get("saves")).toBe(0);
    expect(runtime.store.get("palette")).toBe(0);
    press($("#section"), "s");
    expect(runtime.store.get("saves")).toBe(1);

    press(field, "S", { ctrlKey: true, shiftKey: true });
    expect(runtime.store.get("saves")).toBe(101);
    press(field, "s", { ctrlKey: true });
    expect(runtime.store.get("saves")).toBe(101);
    // The section has handlers, none for this key, so the page's runs.
    press(field, "k", primary);
    expect(runtime.store.get("palette")).toBe(1);
  });

  it("gives a field its own handlers, and the nearest handler that matches stops the search", () => {
    serve("shortcuts_page");
    const runtime = start();
    press($("#notes"), "Escape");
    expect(runtime.store.get("escapes")).toBe(1);
    press($("#own"), "s");
    expect(runtime.store.get("saves")).toBe(1000);
    expect(press($("#own"), "Enter").defaultPrevented).toBe(false);
    expect(runtime.store.get("newlines")).toBe(0);
    press($("#own"), "Enter", { shiftKey: true });
    expect(runtime.store.get("newlines")).toBe(1);
  });
});

describe("server rendering", () => {
  it("shows list items, counts, and visibility before the runtime starts, and the runtime adopts them", async () => {
    serve("todos_page");
    const item = $("#list li");
    expect($$("#list li label").map((label) => label.textContent)).toEqual(["tea"]);
    expect($("#left").textContent).toBe("1");
    expect($("#clear").hasAttribute("data-hs-hide")).toBe(true);
    expect($("#clear").getAttribute("style")).toBeNull();
    expect(getComputedStyle($("#clear")).display).toBe("none");

    const runtime = start();
    await tick();
    expect($("#clear").hasAttribute("data-hs-hide")).toBe(false);
    expect($("#clear").style.display).toBe("none");
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
    expect($<HTMLInputElement>("#note").value).toBe("gift wrap");
    expect($<HTMLInputElement>("#limit").hasAttribute("value")).toBe(false);
    expect($<HTMLSelectElement>("#size").value).toBe('"Small"');
    expect($<HTMLInputElement>("#small").checked).toBe(true);
    expect($<HTMLInputElement>("#large").checked).toBe(false);
    expect($("#buy").getAttribute("title")).toBe("5.00");
    expect($("#buy").style.getPropertyValue("width")).toBe("1rem");
    expect($("#total").textContent).toBe("small");
    expect($("#price-label").textContent).toBe("$2,500.00");
  });
});

describe("links", () => {
  it("writes the same percent-encoded hrefs on the server as the runtime does, and follows a changed parameter", async () => {
    serve("links_page");
    const hrefs = () => $$("a").map((a) => a.getAttribute("href"));
    const served = hrefs();
    expect(served).toEqual([
      "/",
      "/teams/red%20team/members/1",
      "/teams/red%20team?open=true&page=1&q=a%20b%26c%3Dd%23%C3%A9",
      "/teams/caf%C3%A9%2F%C3%BC%3F%26%23/members/-2",
      "/teams/caf%C3%A9%2F%C3%BC%3F%26%23?open=true&page=-2&q=a%20b%26c%3Dd%23%C3%A9",
      "/search?open=true&page=2&q=a%20b%26c%3Dd%23%C3%A9",
      "/search?color=red&color=dark%20blue&open=true&page=2&q=a%20b%26c%3Dd%23%C3%A9&tag=a%2Bb",
      "/files/docs/a%20b/%C3%BC%3F.md",
      "/files/docs/a%20b/%C3%BC%3F.md",
    ]);

    const runtime = mount("links");
    await tick();
    expect(hrefs()).toEqual(served);
    runtime.store.set("members.0.team", "blue");
    runtime.store.set("filter.tag", null);
    runtime.store.set("filter.page", 3);
    runtime.store.set("filter.color", ["blue"]);
    await tick();
    expect($$("#members a")[0]!.getAttribute("href")).toBe("/teams/blue/members/1");
    expect($$("#members a")[1]!.getAttribute("href")).toBe("/teams/blue?open=true&page=1&q=a%20b%26c%3Dd%23%C3%A9");
    expect($("#search-bound").getAttribute("href")).toBe("/search?color=blue&open=true&page=3&q=a%20b%26c%3Dd%23%C3%A9");
    runtime.store.set("filter.color", []);
    await tick();
    expect($("#search-bound").getAttribute("href")).toBe("/search?open=true&page=3&q=a%20b%26c%3Dd%23%C3%A9");
    runtime.store.set("file", "x/y z/");
    await tick();
    expect($("#file-bound").getAttribute("href")).toBe("/files/x/y%20z/");

    const assign = vi.spyOn(location, "assign").mockImplementation(() => {});
    ($$("#members button")[1] as HTMLButtonElement).click();
    expect(assign).toHaveBeenCalledWith(served[3]);
    assign.mockRestore();
  });

  it("pushes and replaces the URL a link names, and the URL a server event names, as the server writes it", async () => {
    history.replaceState(null, "", "/");
    const runtime = mount("links");
    const shown = () => location.pathname + location.search;
    const length = history.length;
    $("#push").click();
    await tick();
    expect(shown()).toBe($("#search-bound").getAttribute("href"));
    expect(history.length).toBe(length + 1);
    $("#replace").click();
    await tick();
    expect(shown()).toBe($("#file-bound").getAttribute("href"));
    expect(history.length).toBe(length + 1);

    const { stream } = JSON.parse(readFileSync(resolve(__dirname, "fixtures/history_patch.json"), "utf8")) as { stream: string };
    const events = stream
      .trim()
      .split("\n\n")
      .map((block) => {
        const [event, data] = block.split("\n");
        return { event: event!.slice("event: ".length), data: data!.slice("data: ".length) };
      });
    runtime.apply(events[0]!);
    expect(shown()).toBe("/search?open=true&page=2&q=a%20b%26c%3Dd%23%C3%A9");
    expect(history.length).toBe(length + 2);
    runtime.apply(events[1]!);
    expect(shown()).toBe("/teams/red%20team/members/1");
    expect(history.length).toBe(length + 2);
  });
});

describe("value operations", () => {
  const texts = () => $$("#values span, #dates span").map((span) => [span.id, span.textContent]);

  it("write the same text on the server and in the browser", async () => {
    serve("values_page");
    const served = texts();
    expect(served.every(([, text]) => text !== "")).toBe(true);
    start();
    await tick();
    expect(texts()).toEqual(served);
  });

  it("write dates and relative times with Intl, and leave the browser's zone to the runtime", async () => {
    serve("values_page");
    expect($("#date-local").textContent).toBe("");
    expect(Object.fromEntries(texts())).toMatchObject({
      "date-en-full": "Monday, January 5, 2026 at 3:04:05 PM",
      "date-en-medium": "Jan 5, 2026, 3:04 PM",
      "date-ja-full": "2026年1月5日月曜日 15:04:05",
      "date-tokyo": "2026/01/06 0:04",
      "ago-en": "3 minutes ago",
      "ago-ja": "3 分前",
    });
    const runtime = start();
    await tick();
    expect($("#date-local").textContent).toBe(
      new Intl.DateTimeFormat("en", { dateStyle: "medium", timeStyle: "short" }).format(1_767_625_445_000),
    );
    runtime.store.set("now", 1_767_625_445_000 + 2 * 86_400_000);
    await tick();
    expect([$("#ago-en").textContent, $("#ago-ja").textContent]).toEqual(["2 days ago", "2 日前"]);
  });

  it("pick a plural form for the count", async () => {
    const runtime = mount("values");
    await tick();
    expect([$("#items-en").textContent, $("#items-ja").textContent]).toEqual(["1 item", "1 点"]);
    runtime.store.set("count", 3);
    await tick();
    expect([$("#items-en").textContent, $("#items-ja").textContent]).toEqual(["3 items", "3 点"]);
  });

  it("test the characters of a string by ASCII class", async () => {
    const runtime = mount("values");
    await tick();
    expect(Object.fromEntries(texts())).toMatchObject({ "zip-digits": "yes", "code-digits": "no", "code-upper": "yes", "code-space": "yes" });
    runtime.store.set("zip", "15ー0001");
    runtime.store.set("code", "１２");
    await tick();
    expect(Object.fromEntries(texts())).toMatchObject({ "zip-digits": "no", "code-upper": "no", "code-space": "no" });
    runtime.store.set("zip", "");
    await tick();
    expect($("#zip-digits").textContent).toBe("yes");
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

describe("title", () => {
  it("is written by the server and rewritten when a computed signal it reads changes", async () => {
    serve("computed_page");
    expect(document.title).toBe("6 doubled");
    const runtime = start();
    await tick();
    expect(document.title).toBe("6 doubled");

    runtime.store.set("b", 4);
    await vi.waitFor(() => expect(document.title).toBe("10 doubled"));
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

describe("lists and conditions inside svg", () => {
  const heights = () => $$("#plot g > rect").map((rect) => rect.getAttribute("height"));
  const marks = () => $$("#plot g").map((g) => g.querySelectorAll("line").length);

  async function exercise(runtime: Runtime): Promise<void> {
    expect(document.querySelectorAll("#plot template").length).toBeGreaterThan(0);
    for (const template of document.querySelectorAll("#plot template")) expect(template).toBeInstanceOf(HTMLTemplateElement);
    expect(heights()).toEqual(["10", "20"]);
    expect(marks()).toEqual([2, 0]);
    expect($("#extent").textContent).toBe("7 10 3 30");
    expect($$("#plot g > rect").every((rect) => rect.namespaceURI === "http://www.w3.org/2000/svg")).toBe(true);

    runtime.store.set("bars", [
      { id: 2, height: 25, marks: [3] },
      { id: 1, height: 10, marks: [1, 2] },
      { id: 3, height: 5, marks: [] },
    ]);
    await tick();
    expect(heights()).toEqual(["25", "10", "5"]);
    expect($("#extent").textContent).toBe("8 10 2 40");
    expect($("#second").textContent).toBe("1");
    $("#collect").click();
    $("#collect").click();
    await tick();
    expect(runtime.store.get("collected")).toEqual([2, 1, 3, 2, 1, 3]);
    expect($("#collected").textContent).toBe("6");
    expect(marks()).toEqual([1, 2, 0]);
    expect($$("#plot g line").map((line) => line.getAttribute("y1"))).toEqual(["3", "1", "2"]);

    expect($("#frame").style.display).toBe("none");
    expect(document.querySelector("#dot")).toBeNull();
    $("#mark").click();
    await tick();
    expect($("#frame").style.display).toBe("");
    expect(document.querySelector("#dot")?.namespaceURI).toBe("http://www.w3.org/2000/svg");
  }

  it("renders them from templates the parser read as svg elements", async () => {
    await exercise(mount("plot"));
  });

  it("adopts the items the server rendered", async () => {
    serve("plot_page");
    expect($("#extent").textContent).toBe("7 10 3 30");
    expect($("#second").textContent).toBe("2");
    const first = $$("#plot > g")[0];
    const runtime = start();
    await tick();
    expect($$("#plot > g")[0]).toBe(first);
    await exercise(runtime);
  });
});

describe("forms", () => {
  // The runtimes that earlier tests started with `start()` listen on `document.body`, and
  // they would run this form's submit handler against their own stores.
  beforeEach(() => document.body.replaceWith(document.createElement("body")));

  const blur = (target: Element) => target.dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
  const submit = () => $("#join").dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));

  it("shows a rule's message once the field is left, and tracks whether the values changed", async () => {
    mount("join");
    expect($("#email-error").textContent).toBe("");
    expect($("#dirty").style.display).toBe("none");

    type($<HTMLInputElement>("#email"), "ada");
    await tick();
    expect($("#email-error").textContent).toBe("");
    expect($("#dirty").style.display).toBe("");
    blur($("#email"));
    await tick();
    expect($("#email-error").textContent).toBe("That is not an email address.");

    type($<HTMLInputElement>("#email"), "");
    await tick();
    expect($("#email-error").textContent).toBe("Enter an email address.");
    type($<HTMLInputElement>("#email"), "ada@example.com");
    await tick();
    expect($("#email-error").textContent).toBe("");
    expect($("#dirty").style.display).toBe("none");
  });

  it("touches every field on submit and calls the action only once every rule passes", async () => {
    const fetch = vi.fn(() => Promise.resolve(new Response("", { status: 200 })));
    vi.stubGlobal("fetch", fetch);
    const runtime = mount("join");
    submit();
    await tick();
    expect(fetch).not.toHaveBeenCalled();
    expect(runtime.store.get("signup.submitted")).toBe(true);
    expect($("#password-error").textContent).toBe("Use 8 or more characters.");

    type($<HTMLInputElement>("#password"), "correct horse");
    await tick();
    expect($("#password-error").textContent).toBe("");
    expect($("#valid").style.display).toBe("");
    submit();
    await tick();
    expect(fetch).toHaveBeenCalledTimes(1);
    const [, init] = fetch.mock.calls[0] as unknown as [string, RequestInit];
    expect(JSON.parse(init.body as string)).toEqual({ email: "ada@example.com", password: "correct horse" });
  });

  it("shows a message the server sent until the next submit, and reset puts the starting values back", async () => {
    vi.stubGlobal("fetch", vi.fn(() => Promise.resolve(new Response("", { status: 200 }))));
    const runtime = mount("join");
    runtime.store.set("signup.errors", { email: "That address is taken." });
    await tick();
    expect($("#email-error").textContent).toBe("That address is taken.");
    submit();
    await tick();
    expect($("#email-error").textContent).toBe("");

    type($<HTMLInputElement>("#email"), "grace");
    $("#reset").click();
    await tick();
    expect($<HTMLInputElement>("#email").value).toBe("ada@example.com");
    expect(runtime.store.get("signup.touched")).toEqual({});
    expect(runtime.store.get("signup.submitted")).toBe(false);
    expect($("#password-error").textContent).toBe("");
  });

  it("renders the form on the server with no message and the runtime adopts it", async () => {
    serve("join_page");
    expect($("#email-error").textContent).toBe("");
    expect(getComputedStyle($("#dirty")).display).toBe("none");
    expect(getComputedStyle($("#valid")).display).toBe("none");
    start();
    type($<HTMLInputElement>("#email"), "x");
    blur($("#email"));
    await tick();
    expect($("#email-error").textContent).toBe("That is not an email address.");
  });
});

describe("uploads", () => {
  // The runtimes that earlier tests started with `start()` listen on `document.body`, and
  // they would send this button's upload a second time.
  beforeEach(() => document.body.replaceWith(document.createElement("body")));

  /** Puts `files` in the file input, as picking them would. jsdom has no `DataTransfer` to build a `FileList`. */
  function pick(files: File[]): void {
    const input = $<HTMLInputElement>("#pictures");
    Object.defineProperty(input, "files", { value: files, configurable: true });
  }

  it("sends the arguments and the picked files as multipart with the upload header", async () => {
    const fetch = vi.fn((_url: URL, _init: RequestInit) =>
      Promise.resolve(new Response('event: hs-signals\ndata: {"saved":"2"}\n\n', { status: 200 })),
    );
    vi.stubGlobal("fetch", fetch);
    const runtime = mount("upload");
    type($<HTMLInputElement>("#caption"), "beach");
    pick([new File(["one"], "a.png", { type: "image/png" }), new File(["two"], "b.txt")]);
    $("#send").click();
    await vi.waitFor(() => expect(runtime.store.get("saved")).toBe("2"));
    const [url, init] = fetch.mock.calls[0]!;
    expect(url.pathname).toBe("/pictures");
    expect(init.method).toBe("POST");
    expect(init.headers).toEqual({ accept: "text/event-stream", "hypersig-upload": "1" });
    const body = init.body as FormData;
    expect(await (body.get("args") as Blob).text()).toBe('"beach"');
    const files = body.getAll("files") as File[];
    expect(files.map((file) => file.name)).toEqual(["a.png", "b.txt"]);
    expect(await files[0]!.text()).toBe("one");
    expect(runtime.store.get("uploading")).toBe(false);
  });

  it("looks up the file input apart from a node ref of the same name", async () => {
    const fetch = vi.fn((_url: URL, _init: RequestInit) => Promise.resolve(new Response("", { status: 200 })));
    vi.stubGlobal("fetch", fetch);
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    mount("upload");
    pick([new File(["one"], "a.png", { type: "image/png" })]);
    $("#send").click();
    await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
    expect(((fetch.mock.calls[0]![1].body as FormData).getAll("files") as File[]).map((file) => file.name)).toEqual(["a.png"]);
    $("#focus").click();
    await vi.waitFor(() => expect(document.activeElement).toBe($("#note")));
    expect(warn).not.toHaveBeenCalled();
  });

  it("sends no file part when nothing is picked, and empties the input after sending", async () => {
    const fetch = vi.fn((_url: URL, _init: RequestInit) => Promise.resolve(new Response("", { status: 200 })));
    vi.stubGlobal("fetch", fetch);
    mount("upload");
    $<HTMLInputElement>("#pictures").value = "";
    $("#send").click();
    await vi.waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
    expect((fetch.mock.calls[0]![1].body as FormData).getAll("files")).toEqual([]);
    await tick();
    expect($<HTMLInputElement>("#pictures").value).toBe("");
  });

  it("writes a refused upload to the error signal", async () => {
    vi.stubGlobal("fetch", vi.fn(() => Promise.resolve(new Response("the upload is longer than its limit", { status: 413 }))));
    vi.spyOn(console, "error").mockImplementation(() => {});
    const runtime = mount("upload");
    $("#send").click();
    await vi.waitFor(() => expect(runtime.store.get("error")).toBe("the upload is longer than its limit"));
    expect(runtime.store.get("uploading")).toBe(false);
  });
});
