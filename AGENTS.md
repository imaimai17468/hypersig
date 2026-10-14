# Project Instructions

hypersig is a server-driven hypermedia library for Rust. The server owns the state, renders HTML, and streams patches over Server-Sent Events. A small JavaScript runtime in the browser applies them and keeps a store of signals for local reactivity. The idea is Datastar's. What this repository adds is that every string Datastar asks you to write by hand (signal names, attribute expressions, action URLs, patch target ids) is produced from a Rust type, so a mistake in any of them is a compile error.

That last sentence is the project's thesis. A change that lets a user write one of those strings by hand again, or that turns a compile error into a runtime failure, needs the user's own words.

## Layout

| Path | What lives there |
|---|---|
| `crates/hypersig` | The library: `Signals`, `Sig`, `Expr`, `Stmt`, the HTML builder, `Action`, `Fragment`, `Patches`, the axum integration, and the embedded runtime |
| `crates/hypersig_proc` | `#[derive(Signals)]` as a thin `proc-macro = true` shim |
| `crates/hypersig_proc_impl` | The token transformation behind the derive, with its snapshot tests |
| `crates/hypersig/tests/ui` | trybuild cases. Each `fail/*.rs` is a mistake that must not compile |
| `runtime/` | The browser runtime in TypeScript. `pnpm build` writes `crates/hypersig/assets/hypersig.js`, which is committed so that `cargo build` needs no Node |
| `examples/` | Runnable axum applications |

The wire format between the two halves is defined twice, once in `crates/hypersig/src/expr.rs` and `patch.rs` and once in `runtime/src/`. A change to one is a change to both in the same commit.

## Verification before completion

No hook runs these. Run them yourself before reporting a change complete:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` (this includes the trybuild cases)
- `cargo check -p hypersig --no-default-features`, because the axum integration is behind a feature
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, which fails on a broken intra-doc link
- In `runtime/`: `pnpm check` and `pnpm test`, where the change touches `runtime/` or the wire format
- In `runtime/`: `pnpm build`, then `git diff --exit-code crates/hypersig/assets`, so the committed bundle matches the source

A trybuild `.stderr` file records the compiler's message. Regenerate it with `TRYBUILD=overwrite cargo test -p hypersig --test ui` only after reading the new message and confirming it still names the mistake the case exists for.

`rust-toolchain.toml` pins the compiler and installs `rust-src`, and CI uses the same version and components, because the trybuild `.stderr` files hold rustc's exact output and rustc quotes standard library source only where `rust-src` is present. Raising the pin is one commit that changes `rust-toolchain.toml`, `rust-version` in `Cargo.toml`, the CI toolchain, and every regenerated `.stderr` together.

Report a step you could not run as "not run", never as "passed".

## Rules

- `.claude/rules/rust.md` covers every `.rs` file. It is the short list of guidelines this codebase leans on.
- `.claude/skills/rust-guidelines/` holds the full Pragmatic Rust Guidelines. Invoke the skill where a decision is not covered by the rule file: a new public type, an error type, a new crate, an `unsafe` block.
- `.claude/rules/prose.md` covers every text a person reads.

## Knowledge Currency

Your training data goes stale. `syn` is at 3.x here and `axum` at 0.8.x, and both differ from the versions most examples on the web were written against. Before writing an import path or an API call from memory, read the pinned version's source under `~/.cargo/registry/src/` or its docs.rs page for that exact version. Where you cannot confirm a fact, write that into the sentence it limits.

## Commits

- One commit has one purpose. Stage explicit paths, never `git add -A` or `git add .`.
- The first line starts with `feat` / `fix` / `refactor` / `test` / `docs` / `chore` and states what improves. The body is in Japanese, and a `fix` or `refactor` says why.
- End with a `Co-Authored-By:` trailer crediting the current model.

## Code Practices

**Make the wrong thing fail to compile.** Where you are about to write a comment, a runtime check, or a doc line so that a caller does not misuse an API, change the types first. A comment is the fallback when the type system genuinely cannot carry the rule, and then the comment says what the type system cannot check.

**Comments explain the code directly below them and nothing else.** No narration, no restating the obvious, no issue numbers, no pointers to another file's behavior.

**Decide what the work needs and act on it.** Hand back only a decision that is the user's to make: taste, risk tolerance, and anything that reaches outside this repository, such as publishing a crate or changing the GitHub repository's settings.
