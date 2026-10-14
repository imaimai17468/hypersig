---
name: rust-guidelines
description: "Look up the full Pragmatic Rust Guidelines by ID or topic. Invoke before adding a public type, an error type, a crate, a feature flag, a macro, or an unsafe block, and whenever .claude/rules/rust.md does not settle a Rust design question."
---

# Rust guidelines

`references/pragmatic-rust-guidelines.md` is Microsoft's Pragmatic Rust Guidelines, concatenated into one file (MIT, copyright line at its top). It is about 34k tokens, so do not read it whole.

1. Find the guideline. Every guideline has a heading of the form `## Title (M-ID) { #M-ID }`. List them with `grep -n '^## ' references/pragmatic-rust-guidelines.md`, or search a topic with `grep -n -i 'error\|panic' ...`.
2. Read that section only, from its heading to the next `## `.
3. Where the section links to the Rust API Guidelines (`C-...` IDs), the checklist is at https://rust-lang.github.io/api-guidelines/checklist.html.
4. Apply it. Where it conflicts with `.claude/rules/rust.md`, the rule file wins, because it records a decision made for this repository. Name the conflict in your report.

Refresh the reference with:

```sh
curl -sL https://microsoft.github.io/rust-guidelines/agents/all.txt -o .claude/skills/rust-guidelines/references/pragmatic-rust-guidelines.md
```
