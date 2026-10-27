---
name: bug-hunt
description: "Run a round of finding and fixing bugs in this repository with a scope and an end agreed first, and fix only what ordinary use hits. Invoke when the user asks to look for bugs, review the repository for problems, or start another round of bug fixing."
---

# Bug hunt

A read-through review of this repository keeps producing findings for as long as someone keeps reading, and a finding found by reading says nothing about whether a user ever meets it. This skill gives a round of bug fixing an end the user agreed to, and fixes only the findings that ordinary use of hypersig hits.

## 1. Agree the scope and the end before reading

Write the proposed scope to the user in prose and wait for an answer (no option picker). A scope names:

- **The kinds of bug it covers**, each with the check that settles it. Prefer a check a machine runs over a stated range of input, because only that supports the sentence "this scope has no other bug of this kind". A reading of a module supports only "reading found these".
- **The end of the round**: the checks pass and the findings they turned up are fixed, or the listed modules have been read once. Nothing outside the scope is reported as a finding of this round.

Checks that fit this repository, where the two halves of the wire format and the type-level guarantees are the usual sources of bugs:

| Kind of bug | Check |
|---|---|
| The renderer and the runtime disagree on a value | Feed the same expressions and signals to `ssr.rs` and `runtime/src/eval.ts` and compare the results |
| A typed link reaches no handler | Property test: for routes and parameters, the path `url()` writes is routed back to that route's handler with the same parameters |
| A patch loses state the page had | Compare `morph(from, to)` with `to` rendered fresh, over generated pairs of trees, and check that kept elements stay connected |
| Something compiles that the thesis says must not | A trybuild case per kind of string the browser acts on |

## 2. Qualify each finding before fixing it

A finding goes forward only with both of these written down:

- **A reproduction**: a failing test, a request, or a browser run that shows the wrong output, the panic, or the lost state. A finding reasoned out but not shown stays a note.
- **The usage sentence**: who hits it, in what ordinary use of hypersig, and how often. For example: "A site with Japanese pages writes a route literal in Japanese, and every link to it answers 404." Where the sentence needs an unusual setup, such as user data in a CSS value under a CSP nonce, or two enum variants renamed to one string, it is an edge case.

Then decide:

- **Fix** a finding that has a reproduction and that ordinary use hits.
- **File** an edge case as a low-priority issue with its reproduction, and do not fix it in this round.
- **A breaking change** (a signature, or code that compiled before and now does not) is only for a finding that ordinary use hits. Before 1.0 a break is allowed, but each one costs every user an edit at the next release.

## 3. Report the end

Close the round with what the scope covered, what each check found, and what was fixed or filed, and say that the scope holds no other finding. Where a check could not run, say so and leave that part of the scope open, rather than calling the round finished.

## Calibration from the review of 2026-10-27

The review filed as #97 to #114 was a read-through without an agreed end, and about half of its 18 findings met the bar above.

- **Ordinary use hits them:** a route literal in Japanese answering 404 (#97), the English text of the dialog, toast, and combobox on Japanese pages (#108), morph reloading an iframe in a row with an id (#103), the duplicate-id check taking 26 ms per patch of 1,000 rows (#102), `cargo hypersig elements` writing a module that does not compile for a component with `poster` (#104), and the published crates without `LICENSE` (#100).
- **Edge cases:** a value that is not UTF-8 under an unknown query key (#105), an unbalanced `[` in a bound style under a CSP nonce (#106), a repeated `each_keyed` key (#99), Tab with the pointer resting on the combobox list (#109), `build.build-dir` set (#111), a leading newline in `textarea` (#113), and two `Segment` variants with one segment (#112). #112 added a breaking change for an edge case.
