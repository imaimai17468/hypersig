import { expect, it } from "vitest";
import { check, installMoveBefore } from "./morph/check.ts";
import { generate, random, show, type ElementModel, type Model } from "./morph/generate.ts";

/*
 * A property test of `morph` over seeded pairs of trees, each morphed in the
 * document. The page reads as `to`, apart from what `morph` keeps on
 * purpose. An element that `to` still has under the same parent never leaves
 * the document. `removed` holds exactly the elements that left it.
 */

/**
 * The findings this test found and `morph` still has, each with the problems
 * of `check` it stands for. A case whose every problem belongs to one of
 * these passes, and each must still occur, so a fix to `morph` fails this
 * test until its entry is deleted here.
 */
const KNOWN = [
  {
    finding:
      "a <template> inserted before an input with no id stops the lookahead, so the input is replaced (#117)",
    matches: (problem: string) => problem.startsWith("input matched by name"),
  },
];

const SEED = 20_261_028;
const PAIRS = 10_000;

/**
 * The cases that broke in a way `KNOWN` does not list, the first three of
 * them, and the findings of `KNOWN` that no case had.
 */
function run(): { failures: string[]; unseen: string[] } {
  const rand = random(SEED);
  const seen = new Set<string>();
  const failures: string[] = [];
  for (let i = 0; i < PAIRS; i++) {
    const test = generate(rand);
    const unknown = check(test).filter((problem) => {
      const known = KNOWN.find(({ matches }) => matches(problem));
      if (known !== undefined) seen.add(known.finding);
      return known === undefined;
    });
    if (unknown.length > 0 && failures.length < 3)
      failures.push(`case ${i}\n${show(test)}\n${unknown.join("\n")}`);
  }
  const unseen = KNOWN.map(({ finding }) => finding).filter((finding) => !seen.has(finding));
  return { failures, unseen };
}

it(`morphs ${PAIRS} generated pairs where the browser has moveBefore`, () => {
  const uninstall = installMoveBefore();
  let result: ReturnType<typeof run>;
  try {
    result = run();
  } finally {
    uninstall();
  }
  expect(result.failures).toEqual([]);
  expect(result.unseen).toEqual([]);
}, 60_000);

it(`morphs ${PAIRS} generated pairs where the browser lacks moveBefore`, () => {
  expect("moveBefore" in Element.prototype).toBe(false);
  const { failures, unseen } = run();
  expect(failures).toEqual([]);
  expect(unseen).toEqual([]);
}, 60_000);

function template(content: string): ElementModel {
  return {
    kind: "element",
    tag: "template",
    attrs: [["data-hs-each", "[]"]],
    children: [],
    content,
  };
}

it("keeps a template and its items when an element before it is removed", () => {
  // from: <div id="root"><p></p><template data-hs-each="[]">a</template><span data-drawn="0">item</span></div>
  // to:   <div id="root"><template data-hs-each="[]">a</template></div>
  const from: Model[] = [
    { kind: "element", tag: "p", attrs: [], children: [] },
    { ...template("a"), drawn: 1 },
  ];
  expect(check({ from, to: [template("a")] })).toEqual([]);
});

it("keeps a template and its items when an element is inserted before it", () => {
  // from: <div id="root"><template data-hs-each="[]">a</template><span data-drawn="0">item</span></div>
  // to:   <div id="root"><p></p><template data-hs-each="[]">a</template></div>
  const to: Model[] = [{ kind: "element", tag: "p", attrs: [], children: [] }, template("a")];
  expect(check({ from: [{ ...template("a"), drawn: 1 }], to })).toEqual([]);
});

it.fails("keeps an input with no id when a template is inserted before it", () => {
  // from: <div id="root"><input name="q"></div>
  // to:   <div id="root"><p></p><template data-hs-each="[]">a</template><input name="q"></div>
  const input: Model = { kind: "element", tag: "input", attrs: [["name", "q"]], children: [] };
  const to: Model[] = [
    { kind: "element", tag: "p", attrs: [], children: [] },
    template("a"),
    input,
  ];
  expect(check({ from: [input], to })).toEqual([]);
});
