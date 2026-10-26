import { afterEach, expect, it, vi } from "vitest";
import { $, type } from "../dom.ts";
import type { Fetch } from "../runtime/mount.ts";
import { mount } from "./fixture.ts";

afterEach(() => {
  vi.unstubAllGlobals();
});

it("reads and writes a part's signals under its field, and tells the server the field", async () => {
  const fetch = vi.fn<Fetch>(() =>
    Promise.resolve(
      new Response(
        'event: hs-signals\ndata: {"footer.status":"Subscribed"}\n\nevent: hs-end\ndata: {}\n\n',
        {
          status: 200,
          headers: { "content-type": "text/event-stream" },
        },
      ),
    ),
  );
  vi.stubGlobal("fetch", fetch);
  const runtime = mount("storefront");
  expect($<HTMLInputElement>("#email").value).toBe("ada@example.com");
  type($<HTMLInputElement>("#email"), "grace@example.com");
  expect(runtime.store.get("footer.email")).toBe("grace@example.com");
  $("#subscribe").click();
  await vi.waitFor(() => expect($("#status").textContent).toBe("Subscribed"));
  const [, init] = fetch.mock.calls[0]!;
  expect((init.headers as Record<string, string>)["hypersig-part"]).toBe("footer");
  expect(init.body).toBe('"grace@example.com"');
});
