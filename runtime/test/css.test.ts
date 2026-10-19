import { beforeEach, describe, expect, it } from "vitest";
import { addRules } from "../src/css.ts";

const keys = () =>
  [...document.head.querySelectorAll<HTMLStyleElement>("style[data-hs-css]")].map(
    (style) => style.dataset.hsCss,
  );

beforeEach(() => {
  document.head.replaceChildren();
});

describe("addRules", () => {
  it("creates the first bucket at the start of head, before the page's own styles", () => {
    const own = document.createElement("style");
    document.head.append(own);
    addRules([["0 k--00", ".a{}"]]);
    expect((document.head.firstElementChild as HTMLElement | null)?.dataset.hsCss).toBe("0 k--00");
  });

  it("keeps buckets in sort-key order and appends a partial overlap to its bucket", () => {
    addRules([
      ["0 k01--00", ".md{}"],
      ["1 k--00", ".when{}"],
    ]);
    addRules([
      ["0 k--00", ".a{}"],
      ["0 k01--00", ".md{}.md2{}"],
      ["0 k-4019-00", ".hover{}"],
    ]);
    expect(keys()).toEqual(["0 k--00", "0 k-4019-00", "0 k01--00", "1 k--00"]);
    expect(document.head.querySelector('style[data-hs-css="0 k01--00"]')?.textContent).toBe(
      ".md{}.md{}.md2{}",
    );
  });

  it("gives a new bucket the nonce of the runtime's script, and none without one", () => {
    addRules([["0 k--00", ".a{}"]]);
    expect(document.head.querySelector("style")!.hasAttribute("nonce")).toBe(false);
    const script = document.createElement("script");
    script.setAttribute("nonce", "n0nce");
    document.head.append(script);
    addRules([["1 k--00", ".b{}"]]);
    const added = document.head.querySelector<HTMLStyleElement>('style[data-hs-css="1 k--00"]')!;
    expect(added.getAttribute("nonce")).toBe("n0nce");
  });
});
