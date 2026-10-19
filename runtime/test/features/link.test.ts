import { afterEach, expect, it, vi } from "vitest";
import { $, $$, tick } from "../dom.ts";
import { events, mount, serve } from "./fixture.ts";

afterEach(() => {
  vi.restoreAllMocks();
});

const hrefs = () => $$("a").map((a) => a.getAttribute("href"));
const shown = () => location.pathname + location.search;

it("writes the same percent-encoded hrefs on the server as the runtime does, and follows a changed parameter", async () => {
  serve("links_page");
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
  expect($$("#members a")[1]!.getAttribute("href")).toBe(
    "/teams/blue?open=true&page=1&q=a%20b%26c%3Dd%23%C3%A9",
  );
  expect($("#search-bound").getAttribute("href")).toBe(
    "/search?color=blue&open=true&page=3&q=a%20b%26c%3Dd%23%C3%A9",
  );
  runtime.store.set("filter.color", []);
  await tick();
  expect($("#search-bound").getAttribute("href")).toBe(
    "/search?open=true&page=3&q=a%20b%26c%3Dd%23%C3%A9",
  );
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
  const length = history.length;
  $("#push").click();
  await tick();
  expect(shown()).toBe($("#search-bound").getAttribute("href"));
  expect(history.length).toBe(length + 1);
  $("#replace").click();
  await tick();
  expect(shown()).toBe($("#file-bound").getAttribute("href"));
  expect(history.length).toBe(length + 1);

  const [push, replace] = events("history_patch");
  runtime.apply(push!);
  expect(shown()).toBe("/search?open=true&page=2&q=a%20b%26c%3Dd%23%C3%A9");
  expect(history.length).toBe(length + 2);
  runtime.apply(replace!);
  expect(shown()).toBe("/teams/red%20team/members/1");
  expect(history.length).toBe(length + 2);
});
