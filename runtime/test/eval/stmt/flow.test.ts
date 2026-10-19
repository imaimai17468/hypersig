import { expect, it } from "vitest";
import { run } from "../../../src/eval.ts";
import { context, host } from "../context.ts";

it("sets signals in order and awaits calls", async () => {
  const ctx = context({ count: 0 });
  const h = host();
  await run(
    [
      "seq",
      [
        ["set", "count", ["add", ["sig", "count"], ["lit", 1]]],
        ["call", "POST", "/save", ["lit", 5], "busy", null],
        ["set", "count", ["lit", 9]],
      ],
    ],
    ctx,
    h,
  );
  expect(h.call).toHaveBeenCalledWith("POST", "/save", expect.any(Function), "busy", null, {
    latest: false,
    reconnect: false,
    connected: false,
    origin: undefined,
  });
  expect(h.call.mock.calls[0]![2]()).toBe(5);
  expect(ctx.store.get("count")).toBe(9);
});

it("runs the branch the condition picks", async () => {
  const ctx = context({ n: 10 });
  await run(
    ["if", ["gt", ["sig", "n"], ["lit", 9]], ["set", "n", ["lit", 0]], ["seq", []]],
    ctx,
    host(),
  );
  expect(ctx.store.get("n")).toBe(0);
});
