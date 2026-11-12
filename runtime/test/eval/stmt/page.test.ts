import { expect, it } from "vitest";
import { run } from "../../../src/eval.ts";
import { context, host } from "../context.ts";

it("hands the evaluated URL to the host to navigate", async () => {
  const h = host();
  await run(
    ["navigate", ["concat", ["lit", "/members/"], ["uri", ["sig", "id"]]]],
    context({ id: "a b" }),
    h,
  );
  expect(h.navigate).toHaveBeenCalledWith("/members/a%20b");
});

it("hands the evaluated URL to the host to push or replace in the address bar", async () => {
  const h = host();
  const url = ["concat", ["lit", "/fruit"], ["query", ["lit", { q: "a b" }]]];
  await run(["history", "push", url], context({}), h);
  await run(["history", "replace", url], context({}), h);
  expect(h.history.mock.calls).toEqual([
    ["/fruit?q=a%20b", false],
    ["/fruit?q=a%20b", true],
  ]);
});

it("hands the evaluated position to the host to place an element", async () => {
  const h = host();
  await run(["dom-at", "show-popover", "menu", ["sig", "at"]], context({ at: { x: 3, y: 4 } }), h);
  expect(h.domAt).toHaveBeenCalledWith("show-popover", "menu", { x: 3, y: 4 }, undefined);
});
