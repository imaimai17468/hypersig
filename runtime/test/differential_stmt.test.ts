import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, it } from "vitest";
import { run } from "../src/eval.ts";
import { Store } from "../src/store.ts";
import { host } from "./eval/context.ts";
import { same } from "./same.ts";

/**
 * The file `crates/hypersig/src/testing/differential.rs` writes: statements,
 * the signals they run against, the list item a statement runs inside, and
 * the signals whose value the simulation changed.
 */
interface Differential {
  readonly run: number;
  readonly groups: readonly {
    readonly signals: Record<string, unknown>;
    readonly cases: readonly (readonly [
      ast: unknown,
      slot: string | null,
      changed: Record<string, unknown>,
    ])[];
  }[];
}

const fixture = JSON.parse(
  readFileSync(resolve(__dirname, "fixtures/differential_stmt.json"), "utf8"),
) as Differential;

/** The signals after the runtime runs `ast` against `signals`, inside the list item at `slot` where one is given. */
function browserSignals(
  ast: unknown,
  slot: string | null,
  signals: Record<string, unknown>,
): unknown {
  const store = new Store(structuredClone(signals));
  try {
    run(ast, { store, scope: slot === null ? [] : [{ path: slot }] }, host());
  } catch (error) {
    return { threw: String(error) };
  }
  return store.untracked(() => store.get(""));
}

/** A known disagreement: the finding it belongs to, and the cases it covers. */
interface Skip {
  readonly finding: string;
  readonly covers: (ast: unknown) => boolean;
}

/** Whether `ast` holds `["fixed", value, digits]` with at least `least` digits, outside a literal's value. */
function fixedTo(ast: unknown, least: number): boolean {
  if (!Array.isArray(ast) || ast[0] === "lit") return false;
  if (ast[0] === "fixed" && (ast[2] as number) >= least) return true;
  return ast.some((part) => fixedTo(part, least));
}

const skips: readonly Skip[] = [
  {
    // With `rec.v` at -1.778429746627808e-15, `toFixed(100)` writes the
    // value's exact digits, and the server writes those of the next double,
    // because a value exactly halfway at that many digits is rounded by
    // printing the next double, whose digits differ before the 100th.
    finding: "fixed with many digits rounds a value exactly halfway differently (#127)",
    covers: (ast) => fixedTo(ast, 17),
  },
];

/** Every case of the file run in the browser, and the ones whose signals differ from the simulation's. */
function compare(): { count: number; disagreements: unknown[]; used: Set<Skip> } {
  const disagreements: unknown[] = [];
  const used = new Set<Skip>();
  let count = 0;
  for (const { signals, cases } of fixture.groups)
    for (const [ast, slot, changed] of cases) {
      count += 1;
      const server = { ...signals, ...changed };
      const browser = browserSignals(ast, slot, signals);
      if (same(server, browser)) continue;
      const skip = skips.find((candidate) => candidate.covers(ast));
      if (skip === undefined) disagreements.push({ ast, slot, server, browser });
      else used.add(skip);
    }
  return { count, disagreements, used };
}

it("runs every statement to the signals the simulation left", () => {
  const { count, disagreements, used } = compare();
  expect(count).toBe(fixture.run);
  expect(disagreements.slice(0, 5)).toEqual([]);
  // A skip that no longer covers a disagreement is stale and goes.
  expect(skips.filter((skip) => !used.has(skip)).map((skip) => skip.finding)).toEqual([]);
});
