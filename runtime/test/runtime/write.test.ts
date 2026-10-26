import { expect, it } from "vitest";
import { safeUrl } from "../../src/runtime/write.ts";

// The same cases as the tests of `crates/hypersig/src/href.rs`.
it("keeps a URL whose scheme cannot run a script", () => {
  const refused = [
    "https://example.com/a?b#c",
    "HTTP://example.com",
    "mailto:a@example.com",
    "/logo.png",
    "#top",
    "?page=2",
    "",
    "//cdn.example/a.js",
    "docs/a:b",
    "./a:b",
  ].filter((url) => !safeUrl(url));
  expect(refused).toEqual([]);
});

it("refuses a URL that could run a script", () => {
  const kept = [
    "javascript:alert(1)",
    "JavaScript:alert(1)",
    "java\tscript:alert(1)",
    " javascript:alert(1)",
    "data:text/html,<script>alert(1)</script>",
    "vbscript:x",
    "blob:https://example.com/x",
    "a b",
  ].filter((url) => safeUrl(url));
  expect(kept).toEqual([]);
});
