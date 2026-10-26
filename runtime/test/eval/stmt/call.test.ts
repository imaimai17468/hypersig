import { expect, it } from "vitest";
import { run } from "../../../src/eval.ts";
import { context, host } from "../context.ts";

it("passes the flags after the error signal to the call", async () => {
  const h = host();
  await run(
    [
      "call",
      "GET",
      "/find",
      ["lit", "tea"],
      null,
      null,
      ["latest", "reconnect", "connected", "exclusive"],
      { timeout: 5000 },
    ],
    context({}),
    h,
  );
  expect(h.call).toHaveBeenCalledWith("GET", "/find", expect.any(Function), null, null, {
    latest: true,
    reconnect: true,
    connected: true,
    exclusive: true,
    timeout: 5000,
    origin: undefined,
  });
});
