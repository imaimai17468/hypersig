import { expect, it } from "vitest";
import { AstError, evaluate } from "../../../src/eval.ts";
import { ctx } from "../context.ts";

const plain = (value: number) => evaluate(["format", ["lit", value], 0, ",", ".", "", ""], ctx);
const at = Date.UTC(2026, 0, 5, 15, 4, 5);
const ago = (millis: number, locale = "en") =>
  evaluate(["reltime", ["lit", at + millis], ["lit", at], locale], ctx);
const forms = ["z", "1", "2", "f", "m", "o"].map((form) => ["lit", form]);
const plural = (locale: string, n: number) => evaluate(["plural", ["lit", n], locale, forms], ctx);

it("formats numbers as ssr.rs does", () => {
  expect(plain(0)).toBe("0");
  expect(plain(999)).toBe("999");
  expect(plain(1000)).toBe("1,000");
  expect(plain(-1234567)).toBe("-1,234,567");
  expect(plain(-0.4)).toBe("0");
  expect(evaluate(["format", ["lit", -1234.5], 2, ",", ".", "$", ""], ctx)).toBe("-$1,234.50");
  expect(evaluate(["format", ["lit", 1234567.891], 2, ".", ",", "", " €"], ctx)).toBe(
    "1.234.567,89 €",
  );
  expect(evaluate(["format", ["lit", 12345], 0, "", ".", "", "%"], ctx)).toBe("12345%");
  expect(evaluate(["format", ["lit", Number.NaN], 0, ",", ".", "¥", ""], ctx)).toBe("¥NaN");
  expect(evaluate(["format", ["lit", 2e21], 0, ",", ".", "", ""], ctx)).toBe("2e+21");
});

it("writes dates in a fixed offset and relative times as date.rs does", () => {
  expect(evaluate(["date", ["lit", at], "en", "medium", "short", 0], ctx)).toBe(
    "Jan 5, 2026, 3:04 PM",
  );
  expect(evaluate(["date", ["lit", at], "ja", null, "medium", 540], ctx)).toBe("0:04:05");
  expect(evaluate(["date", ["lit", at], "en", "long", null, -300], ctx)).toBe("January 5, 2026");
  expect(ago(-500)).toBe("0 seconds ago");
  expect(ago(59_999)).toBe("in 59 seconds");
  expect(ago(-3_600_000)).toBe("1 hour ago");
  expect(ago(29 * 86_400_000)).toBe("in 29 days");
  expect(ago(-31 * 86_400_000, "ja")).toBe("1 か月前");
  expect(ago(400 * 86_400_000, "ja")).toBe("1 年後");
});

it("picks the plural form as ssr.rs does", () => {
  expect([1, -1, 0, 1.5, 2].map((n) => plural("en", n))).toEqual(["1", "1", "o", "o", "o"]);
  expect(plural("ja", 1)).toBe("o");
});

it("tests characters by ASCII class as ssr.rs does", () => {
  expect(evaluate(["chars_all", ["lit", "0123"], "digit"], ctx)).toBe(true);
  expect(evaluate(["chars_all", ["lit", ""], "digit"], ctx)).toBe(true);
  expect(evaluate(["chars_any", ["lit", ""], "digit"], ctx)).toBe(false);
  expect(evaluate(["chars_all", ["lit", "１"], "digit"], ctx)).toBe(false);
  expect(evaluate(["chars_all", ["lit", "aF9"], "hex"], ctx)).toBe(true);
  expect(evaluate(["chars_any", ["lit", "a\vb"], "space"], ctx)).toBe(false);
  expect(evaluate(["chars_any", ["lit", "a\fb"], "space"], ctx)).toBe(true);
  expect(evaluate(["chars_all", ["lit", "aZ"], "alpha"], ctx)).toBe(true);
  expect(evaluate(["chars_any", ["lit", "😀"], "alnum"], ctx)).toBe(false);
  expect(() => evaluate(["chars_all", ["lit", "a"], "emoji"], ctx)).toThrow(AstError);
});
