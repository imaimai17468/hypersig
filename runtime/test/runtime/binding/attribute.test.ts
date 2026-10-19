import { afterEach, expect, it, vi } from "vitest";
import { $, tick } from "../../dom.ts";
import { mount } from "../mount.ts";

afterEach(() => {
  vi.unstubAllGlobals();
});

type Entry = { target: Element; isIntersecting: boolean };

/** Stands in for `IntersectionObserver`, which the test DOM lacks, and records the elements it watches. */
function observeVisibility() {
  let callback!: (entries: Entry[]) => void;
  const observed: Element[] = [];
  const unobserved: Element[] = [];
  class Observer {
    constructor(report: (entries: unknown[], observer: Observer) => void) {
      callback = (entries) => report(entries, this);
    }
    observe(target: Element) {
      observed.push(target);
    }
    unobserve(target: Element) {
      unobserved.push(target);
    }
  }
  vi.stubGlobal("IntersectionObserver", Observer);
  // The runtime builds its observer when it starts, after this returns.
  const report = (entries: Entry[]) => callback(entries);
  return { report, observed, unobserved };
}

it("renders text and show from signals, and reacts to a click", async () => {
  mount(
    `<span id="t" data-hs-text='["str",["sig","count"]]'></span>` +
      `<p id="p" data-hs-show='["gt",["sig","count"],["lit",0]]'>positive</p>` +
      `<button id="b" data-hs-on-click='["set","count",["add",["sig","count"],["lit",1]]]'><i id="icon">+</i></button>`,
    { count: 0 },
  );
  expect($("#t").textContent).toBe("0");
  expect($("#p").style.display).toBe("none");

  $("#icon").click();
  await tick();
  expect($("#t").textContent).toBe("1");
  expect($("#p").style.display).toBe("");
});

it("counts the items of a list with one read of the list, and still reads the signals the test compares with", async () => {
  const runtime = mount(
    `<b id="n" data-hs-text='["str",["count","rows",0,["eq",["item",0,"done"],["sig","want"]]]]'></b>`,
    {
      rows: [false, true, false].map((done) => ({ done })),
      want: false,
    },
  );
  await tick();
  expect($("#n").textContent).toBe("2");
  runtime.store.set("rows.1.done", false);
  await tick();
  expect($("#n").textContent).toBe("3");
  runtime.store.set("want", true);
  await tick();
  expect($("#n").textContent).toBe("0");
  runtime.store.set("rows", [...(runtime.store.get("rows") as unknown[]), { done: true }]);
  await tick();
  expect($("#n").textContent).toBe("1");
});

it("runs an on-visible statement each time the element comes into view, and stops watching it once it leaves the page", () => {
  const { report, observed, unobserved } = observeVisibility();
  const runtime = mount(
    `<p id="end" data-hs-on-visible='["set","loads",["add",["sig","loads"],["lit",1]]]'></p>`,
    { loads: 0 },
  );
  const end = $("#end");
  expect(observed).toEqual([end]);
  report([{ target: end, isIntersecting: true }]);
  report([{ target: end, isIntersecting: false }]);
  report([{ target: end, isIntersecting: true }]);
  expect(runtime.store.get("loads")).toBe(2);
  end.remove();
  report([{ target: end, isIntersecting: false }]);
  expect(unobserved).toEqual([end]);
});
