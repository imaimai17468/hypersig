---
name: bug-hunt
description: "Run a round of finding and fixing bugs and usability problems in this repository, with a scope and an end agreed first, and keep only what ordinary use hits. Invoke when the user asks to look for bugs or improvements, review the repository for problems, or start another round."
---

# Bug hunt

Reading the repository yields findings for as long as someone keeps reading, and ideas for improvement never run out. A finding found by reading says nothing about whether a user meets it. This skill gives a round an end the user agreed to and keeps only the findings that ordinary use of hypersig hits.

## 1. Agree the scope and the end before starting

Write the proposed scope to the user in prose and wait for an answer (no option picker). A scope names one or both sources of findings, each with its end:

- **Checks for kinds of bug.** Each kind comes with the check that settles it. Prefer a check a machine runs over a stated range of input, because only that supports the sentence "this scope has no other bug of this kind". A reading of a module supports only "reading found these". The round ends when the checks pass and their findings are fixed, or when the listed modules have been read once.
- **Tasks done with the public API.** Each is a piece of work a user of hypersig does, done here through the public API alone. Improvements come only from this source, because a task meets the API's friction and a reading does not. The round ends when the tasks are done.

Nothing outside the scope is reported as a finding of this round.

Checks that fit this repository, where the two halves of the wire format and the type-level guarantees are the usual sources of bugs:

| Kind of bug | Check |
|---|---|
| The renderer and the runtime disagree on a value | Feed the same expressions and signals to `ssr.rs` and `runtime/src/eval.ts` and compare the results |
| A typed link reaches no handler | Property test: for routes and parameters, the path `url()` writes is routed back to that route's handler with the same parameters |
| A patch loses state the page had | Compare `morph(from, to)` with `to` rendered fresh, over generated pairs of trees, and check that kept elements stay connected |
| Something compiles that the thesis says must not | A trybuild case per kind of string the browser acts on |

Tasks that fit this repository:

| Task | What it uses |
|---|---|
| Write a site page and its demos under `site/src/` | The components or crate the page documents, and the claims its text makes |
| Start a project from a template under `crates/hypersig_cli/templates/` and run it with `cargo hypersig dev` | The CLI and the template |
| Add a feature to an example under `examples/`, such as a page, an action, or a stream | The routes, signals, and patches the feature needs |

## 2. Qualify each finding before acting on it

A finding goes forward only with these three written down:

- **The evidence.** For a bug, a reproduction: a failing test, a request, or a browser run that shows the wrong output, the panic, or the lost state. For an improvement, what the task ran into: the workaround it wrote, the point where it stopped, or a difference from a sibling API that it relied on. A finding reasoned out but not shown stays a note. An idea the task did not run into is not recorded.
- **The usage sentence.** Who hits it, in what ordinary use of the public API, and how often. For example: "A site with Japanese pages writes a route literal in Japanese, and every link to it answers 404." Where the sentence needs an unusual setup, such as user data in a CSS value under a CSP nonce, two enum variants renamed to one string, or a hand-written impl of a `#[doc(hidden)]` item, it is an edge case.
- **The consequence**, as one of these levels:
  1. Wrong output, a panic, a dropped connection, or lost state.
  2. The user writes a workaround, an API behaves differently from its sibling, or the docs lead to code that fails.
  3. Only a tool notices, such as an HTML or ARIA validator.
  4. No user is affected, such as docs for a case the public API cannot reach.

A finding about the machinery that guards a feature (a test, CI, the docs' build) goes forward only when a bug has already got past that machinery.

Then decide:

- **Fix** a finding that ordinary use hits at level 1 or 2.
- **Fold in** a level-3 finding that a few lines fix, in a commit on the same code. Otherwise drop it.
- **Drop** a level-4 finding and an edge case. List each in the report of the round, and file an issue only for a panic, lost data, or a security problem.
- **File** a finding to fix whose fix needs a redesign larger than the round, with its evidence, the usage sentence, and what the redesign involves.
- **Break** a signature, or code that compiled before, only for a finding that ordinary use hits. Before 1.0 a break is allowed, but each one costs every user an edit at the next release.

## 3. Report the end

Close the round with what the scope covered, what each check and task found, what was fixed, folded in, filed, or dropped, and that the scope holds no other finding. Where a check or a task could not run, say so and leave that part of the scope open, rather than calling the round finished.

## Calibration

### The review of 2026-10-27

The review filed as #97 to #114 was a read-through without an agreed end, and about half of its 18 findings met the bar above.

- **Ordinary use hits them:** a route literal in Japanese answering 404 (#97), the English text of the dialog, toast, and combobox on Japanese pages (#108), morph reloading an iframe in a row with an id (#103), the duplicate-id check taking 26 ms per patch of 1,000 rows (#102), `cargo hypersig elements` writing a module that does not compile for a component with `poster` (#104), and the published crates without `LICENSE` (#100).
- **Edge cases:** a value that is not UTF-8 under an unknown query key (#105), an unbalanced `[` in a bound style under a CSP nonce (#106), a repeated `each_keyed` key (#99), Tab with the pointer resting on the combobox list (#109), `build.build-dir` set (#111), a leading newline in `textarea` (#113), and two `Segment` variants with one segment (#112). #112 added a breaking change for an edge case.

### The docs work of 2026-11-02

Writing the i18n and Components pages (#177, #178) filed #179 to #187. The task-based source found every one of them.

- **Fix:** list markers and underlines in the navigation components (#186, level 1), a panic in a page handler dropping the connection (#179, level 1), a page with no signals failing clippy so that it needs a dummy field (#183, level 2), a second trigger panicking for `Menu` and `Popover` but not for `Dialog` and `Tooltip` (#185, level 2), and `Accordion::exclusive` saying an id from `Id::with_key` is unchecked where `Page::render` panics on it (#187, level 2).
- **Fold in:** `aria-activedescendant=""` while no combobox option is active (#184, level 3).
- **Drop:** `TableState::sorting` returning `Some` for a `DataTable`'s state under an integer key (#187, an edge case), the `# Panics` section of `negotiate`, which only an empty hand-written `Variants::VARIANTS` reaches (#180, level 4), and a check of the error codes of `compile_fail` snippets, which no wrong snippet has got past yet (#181, machinery).
