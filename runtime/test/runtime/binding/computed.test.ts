import { afterEach, expect, it, vi } from "vitest";
import { $ } from "../../dom.ts";
import { Runtime } from "../../../src/runtime.ts";

afterEach(() => {
  vi.restoreAllMocks();
});

it("runs the steps after a write whose computed signal fails", () => {
  const error = vi.spyOn(console, "error").mockImplementation(() => {});
  const root = document.createElement("main");
  // `chars_all` reads its text with `for...of`, which throws for `null`.
  root.dataset.hsComputed = JSON.stringify([["digits", ["chars_all", ["sig", "code"], "digit"]]]);
  root.innerHTML = `<button data-hs-on-click='["seq",[["set","code",["lit",null]],["set","after",["lit",true]]]]'>go</button>`;
  document.body.replaceChildren(root);
  const runtime = new Runtime(root, { code: "12", digits: true, after: false });
  $("button").click();
  expect(runtime.store.get("after")).toBe(true);
  expect(error).toHaveBeenCalled();
});
