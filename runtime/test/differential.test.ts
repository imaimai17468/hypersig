import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it } from "vitest";
import { evaluate } from "../src/eval.ts";
import { context } from "./eval/context.ts";

/** The file `crates/hypersig/src/ssr/differential.rs` writes: trees, the signals they read, and the value the server gave each. */
interface Differential {
  readonly evaluated: number;
  readonly groups: readonly {
    readonly signals: Record<string, unknown>;
    readonly cases: readonly (readonly [ast: unknown, server: unknown])[];
  }[];
}

const fixture = JSON.parse(
  readFileSync(resolve(__dirname, "fixtures/differential.json"), "utf8"),
) as Differential;

/**
 * Whether the browser shows `browser` as it shows `server`. A number is compared
 * as `String` writes it, which is the text a binding shows and the digits
 * `JSON.stringify` sends, so -0 and 0 are the same value. A difference that a
 * later operation would expose, such as `1 / x`, sits inside the same tree and
 * shows in its value.
 */
function same(server: unknown, browser: unknown): boolean {
  if (typeof server === "number")
    return typeof browser === "number" && String(server) === String(browser);
  if (Array.isArray(server))
    return (
      Array.isArray(browser) &&
      server.length === browser.length &&
      server.every((item, index) => same(item, browser[index]))
    );
  if (server !== null && typeof server === "object") {
    if (browser === null || typeof browser !== "object" || Array.isArray(browser)) return false;
    const fields = browser as Record<string, unknown>;
    const names = Object.keys(server);
    return (
      names.length === Object.keys(fields).length &&
      names.every(
        (name) => name in fields && same((server as Record<string, unknown>)[name], fields[name]),
      )
    );
  }
  return server === browser;
}

/** A known disagreement: the finding it belongs to, and the cases it covers. */
interface Skip {
  readonly finding: string;
  readonly covers: (ast: unknown) => boolean;
}

/** Whether `ast` holds the operation `op` outside a literal's value. */
function uses(ast: unknown, op: string): boolean {
  if (!Array.isArray(ast) || ast[0] === "lit") return false;
  return ast[0] === op || ast.some((part) => uses(part, op));
}

const skips: readonly Skip[] = [
  {
    // `["pow",["lit",1000000],["lit",7]]` is 1e42 on the server and
    // 9.999999999999999e+41 in V8, and `["log10",["lit",0.8854563236236572]]`
    // is -0.05283285613065316 against -0.05283285613065315. The server calls
    // its platform's `pow` and `log10`, and V8 its own, and the two round the
    // last bit differently. The header of `crates/hypersig/src/ssr.rs` names it.
    finding: "pow and log10 round the last bit differently on the server and in V8",
    covers: (ast) => uses(ast, "pow") || uses(ast, "log10"),
  },
];

/** One case where the two sides disagree. */
interface Disagreement {
  readonly ast: unknown;
  readonly signals: Record<string, unknown>;
  readonly server: unknown;
  readonly browser: unknown;
}

function browserValue(ast: unknown, signals: Record<string, unknown>): unknown {
  try {
    return evaluate(ast, context(structuredClone(signals)));
  } catch (error) {
    return { threw: String(error) };
  }
}

/** Every case of the file evaluated in the browser, and the disagreements no skip covers. */
function compare(): { count: number; disagreements: Disagreement[]; used: Set<Skip> } {
  const disagreements: Disagreement[] = [];
  const used = new Set<Skip>();
  let count = 0;
  for (const { signals, cases } of fixture.groups) {
    for (const [ast, server] of cases) {
      count += 1;
      const browser = browserValue(ast, signals);
      if (same(server, browser)) continue;
      const skip = skips.find((candidate) => candidate.covers(ast));
      if (skip === undefined) disagreements.push({ ast, signals, server, browser });
      else used.add(skip);
    }
  }
  return { count, disagreements, used };
}

it("evaluates every tree to the value the server wrote into the first HTML", () => {
  const { count, disagreements, used } = compare();
  expect(count).toBe(fixture.evaluated);
  expect(disagreements.slice(0, 5)).toEqual([]);
  // A skip that no longer covers a disagreement is stale and goes.
  expect(skips.filter((skip) => !used.has(skip)).map((skip) => skip.finding)).toEqual([]);
});
