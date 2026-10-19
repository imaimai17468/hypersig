import { AstError, type Builder, compile } from "../../eval.ts";

/**
 * A number written as `format_number` in `crates/hypersig/src/ssr.rs` writes it: rounded by `toFixed`,
 * its whole part grouped by threes, a minus sign only where a digit is not zero, then the affixes.
 */
function formatNumber(value: number, [digits, group, point, prefix, suffix]: NumberFormat): string {
  if (!Number.isFinite(value)) return `${prefix}${String(value)}${suffix}`;
  const fixed = Math.abs(value).toFixed(digits);
  const sign = value < 0 && /[1-9]/u.test(fixed) ? "-" : "";
  if (fixed.includes("e")) return `${sign}${prefix}${fixed}${suffix}`;
  const [whole = "", fraction = ""] = fixed.split(".");
  const grouped = whole.replaceAll(/\B(?=(\d{3})+$)/gu, () => group);
  return `${sign}${prefix}${grouped}${fraction === "" ? "" : point + fraction}${suffix}`;
}

/** The digits after the point, the group separator, the decimal point, and the prefix and suffix of `["format",...]`. */
type NumberFormat = [digits: number, group: string, point: string, prefix: string, suffix: string];

/**
 * The ASCII character classes of `CharClass` in `crates/hypersig/src/expr.rs`, by code point.
 * A code point outside ASCII is in no class.
 */
const classes: Record<string, (unit: number) => boolean> = {
  digit: (u) => u >= 0x30 && u <= 0x39,
  hex: (u) => (u >= 0x30 && u <= 0x39) || (u >= 0x41 && u <= 0x46) || (u >= 0x61 && u <= 0x66),
  alpha: (u) => (u >= 0x41 && u <= 0x5a) || (u >= 0x61 && u <= 0x7a),
  alnum: (u) => (u >= 0x30 && u <= 0x39) || (u >= 0x41 && u <= 0x5a) || (u >= 0x61 && u <= 0x7a),
  upper: (u) => u >= 0x41 && u <= 0x5a,
  lower: (u) => u >= 0x61 && u <= 0x7a,
  // Rust's `is_ascii_whitespace`, which leaves out the vertical tab.
  space: (u) => u === 0x20 || u === 0x09 || u === 0x0a || u === 0x0c || u === 0x0d,
};

/** Whether every (`all`) or some character of `text` is in `test`. */
function testChars(text: string, test: (unit: number) => boolean, all: boolean): boolean {
  for (const char of text) {
    if (test(char.codePointAt(0)!) !== all) return !all;
  }
  return all;
}

type DateStyle = "full" | "long" | "medium" | "short";

/** The units `["reltime",...]` picks from, largest first, with their length in milliseconds, as `UNITS` in `crates/hypersig/src/date.rs` lists them. */
const relativeUnits: readonly (readonly [number, Intl.RelativeTimeFormatUnit])[] = [
  [31_536_000_000, "year"],
  [2_592_000_000, "month"],
  [86_400_000, "day"],
  [3_600_000, "hour"],
  [60_000, "minute"],
  [1000, "second"],
];

/** The categories `Intl.PluralRules` picks from, in the order `["plural",...]` lists their forms. */
const pluralCategories: readonly string[] = ["zero", "one", "two", "few", "many", "other"];

/** The `Intl` options of `["date",...]`: a fixed offset shifts the time and writes it as UTC, and no offset leaves the browser's own zone. */
function dateOptions(
  dateStyle: DateStyle | null,
  timeStyle: DateStyle | null,
  offset: number | null,
): Intl.DateTimeFormatOptions {
  const options: Intl.DateTimeFormatOptions = {};
  if (dateStyle !== null) options.dateStyle = dateStyle;
  if (timeStyle !== null) options.timeStyle = timeStyle;
  if (offset !== null) options.timeZone = "UTC";
  return options;
}

/** A builder of `chars_all` when `all`, or of `chars_any`. */
function chars(all: boolean): Builder {
  return ([valueAst, name], ast) => {
    const [value, test] = [compile(valueAst), classes[name as string]];
    if (test === undefined) throw new AstError(ast);
    return (ctx) => testChars(value(ctx) as string, test, all);
  };
}

/** The expressions that write a number, a date, or a plural form as text, and those that test a text's characters. */
export const texts: Record<string, Builder> = {
  fixed: ([valueAst, digits]) => {
    const value = compile(valueAst);
    return (ctx) => (value(ctx) as number).toFixed(digits as number);
  },
  format: ([valueAst, ...format]) => {
    const value = compile(valueAst);
    return (ctx) => formatNumber(value(ctx) as number, format as NumberFormat);
  },
  date: ([valueAst, locale, dateStyle, timeStyle, offset]) => {
    const value = compile(valueAst);
    const shift = ((offset as number | null) ?? 0) * 60_000;
    const format = new Intl.DateTimeFormat(
      locale as string,
      dateOptions(
        dateStyle as DateStyle | null,
        timeStyle as DateStyle | null,
        offset as number | null,
      ),
    );
    return (ctx) => format.format((value(ctx) as number) + shift);
  },
  reltime: ([valueAst, nowAst, locale]) => {
    const [value, now] = [compile(valueAst), compile(nowAst)];
    const format = new Intl.RelativeTimeFormat(locale as string);
    return (ctx) => {
      const millis = (value(ctx) as number) - (now(ctx) as number);
      const [span, unit] =
        relativeUnits.find(([length]) => Math.abs(millis) >= length) ?? relativeUnits.at(-1)!;
      return format.format(Math.trunc(millis / span), unit);
    };
  },
  plural: ([valueAst, locale, formAsts], ast) => {
    const value = compile(valueAst);
    const rules = new Intl.PluralRules(locale as string);
    const forms = (formAsts as unknown[]).map((form) => compile(form));
    return (ctx) => {
      const form = forms[pluralCategories.indexOf(rules.select(value(ctx) as number))];
      if (form === undefined) throw new AstError(ast);
      return form(ctx);
    };
  },
  chars_all: chars(true),
  chars_any: chars(false),
};
