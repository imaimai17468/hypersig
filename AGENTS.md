# Project Instructions

hypersig is a server-driven hypermedia library for Rust. The server owns the state, renders HTML, and streams patches over Server-Sent Events. A small JavaScript runtime in the browser applies them and keeps a store of signals for local reactivity. Every string the browser acts on (signal names, attribute expressions, action URLs, page links, patch target ids, node ref names) is produced from a Rust type, so a mistake in any of them is a compile error.

That last sentence is the project's thesis. A change that lets a user write one of those strings by hand again, or that turns a compile error into a runtime failure, needs the user's own words.

## Layout

| Path | What lives there |
|---|---|
| `crates/hypersig` | The library: `Signals`, `Sig`, `Expr`, `Stmt`, the HTML builder, `Action`, `Route`, `Fragment`, `Patches`, the axum integration, and the embedded runtime |
| `crates/hypersig_proc` | `#[derive(Signals)]` as a thin `proc-macro = true` shim |
| `crates/hypersig_proc_impl` | The token transformation behind the derive, with its snapshot tests |
| `crates/hypersig_style` | Typed atomic CSS: `Style`, `Sheet`, the value types, and the code `tests/generate.rs` writes from the MDN data in `data/`. Regenerate it with `HYPERSIG_GENERATE=overwrite cargo test -p hypersig_style --test generate` |
| `crates/hypersig_ui` | Components styled after shadcn/ui on native elements and the WAI-ARIA patterns, with trybuild cases under `tests/ui/fail` |
| `crates/hypersig/benches/render.rs` | divan benchmarks of `Page::render` over list pages, with allocation counts |
| `crates/hypersig/tests/ui` | trybuild cases. Each `fail/*.rs` is a mistake that must not compile |
| `crates/hypersig/tests/fixtures.rs` | Builds pages with the Rust builder and writes them to `runtime/test/fixtures/`, which the runtime tests load |
| `runtime/` | The browser runtime in TypeScript. `pnpm build` writes `crates/hypersig/assets/hypersig.js`, which is committed so that `cargo build` needs no Node |
| `examples/` | Runnable axum applications |
| `e2e/` | Playwright tests that start every example and drive it in Chrome |
| `site/` | The documentation site, built with hypersig and `hypersig_ui` and published to GitHub Pages by `.github/workflows/pages.yml`. `cargo run -p site -- --out DIR --base ""` writes it for a local preview. Its code samples are doctests under `site/snippets/`, programs under `site/examples/`, or live demos under `site/src/demo/` |

The wire format between the two halves is defined twice, once in `crates/hypersig/src/expr.rs`, `html.rs`, `page.rs`, `patch.rs`, and `route.rs` and once in `runtime/src/`. A change to one is a change to both in the same commit. `tests/fixtures.rs` fails when a committed fixture differs from what the builder writes now, and `runtime/test/features.test.ts` runs those fixtures, so a change on one side that the other side does not follow fails one of the two test suites. Regenerate the fixtures with `HYPERSIG_FIXTURES=overwrite cargo test -p hypersig --test fixtures`, then run `pnpm test`.

## Verification before completion

No hook runs these. Run them yourself before reporting a change complete:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` (this includes the trybuild cases)
- `cargo check -p hypersig --no-default-features`, because the axum integration is behind a feature
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, which fails on a broken intra-doc link
- `cargo run -p site -- --out target/site --rustdoc target/doc` after that, which fails when a page of the documentation site links to an item the rustdoc output no longer has. The site's reference and component pages copy the first sentence of each doc comment, some of them shortened, so a change to one updates `site/src/reference.rs` or `site/src/components.rs` too
- `cargo bench -p hypersig --bench render` before and after the change, where it touches `html.rs`, `page.rs`, `ssr.rs`, or `expr.rs`. The commit body carries both medians.
- In `runtime/`: `pnpm check` and `pnpm test`, where the change touches `runtime/` or the wire format
- In `runtime/`: `pnpm build`, then `git diff --exit-code crates/hypersig/assets`, so the committed bundle matches the source
- In `e2e/`: `pnpm check` and `pnpm test`, where the change touches the runtime, the HTML builder, or an example. Locally it drives the installed Chrome; CI installs Playwright's Chromium.

A trybuild `.stderr` file records the compiler's message. Regenerate it with `TRYBUILD=overwrite cargo test -p hypersig --test ui` only after reading the new message and confirming it still names the mistake the case exists for.

`rust-toolchain.toml` pins the compiler and installs `rust-src`, and CI uses the same version and components, because the trybuild `.stderr` files hold rustc's exact output and rustc quotes standard library source only where `rust-src` is present. Raising the pin is one commit that changes `rust-toolchain.toml`, `rust-version` in `Cargo.toml`, the CI toolchain, and every regenerated `.stderr` together.

CI also runs `cargo udeps --workspace --all-targets` and `cargo miri test -p hypersig --lib` on the nightly pinned in `.github/workflows/ci.yml`. Run them locally with `cargo +nightly-2026-10-14 ...` when a change adds a dependency or touches `ssr.rs`, `signal.rs`, or `expr.rs`. `hypersig_proc_impl` is left out of miri because insta spawns a process to find the workspace, which miri cannot do.

Report a step you could not run as "not run", never as "passed".

## Rules

- `.claude/rules/rust.md` covers every `.rs` file. It is the short list of guidelines this codebase leans on.
- `.claude/skills/rust-guidelines/` holds the full Pragmatic Rust Guidelines. Invoke the skill where a decision is not covered by the rule file: a new public type, an error type, a new crate, an `unsafe` block.
- `.claude/rules/prose.md` covers every text a person reads.
- `.claude/skills/japanese-prose/` holds the polite Japanese register and the rules that keep `README.md` and `README.ja.md` one document. Invoke it before writing Japanese prose that a person reads.

## Knowledge Currency

Your training data goes stale. `syn` is at 3.x here and `axum` at 0.8.x, and both differ from the versions most examples on the web were written against. Before writing an import path or an API call from memory, read the pinned version's source under `~/.cargo/registry/src/` or its docs.rs page for that exact version. The language features and standard library APIs the pinned compiler offers are listed in `~/.rustup/toolchains/<channel in rust-toolchain.toml>-<host>/share/doc/rust/html/releases.md`, which `rust-toolchain.toml` installs, so read it before choosing an older idiom over a newer API. Where you cannot confirm a fact, write that into the sentence it limits.

## Commits

- One commit has one purpose. Stage explicit paths, never `git add -A` or `git add .`.
- The first line starts with `feat` / `fix` / `refactor` / `test` / `docs` / `chore` and states what improves. The body is in Japanese, and a `fix` or `refactor` says why.
- End with a `Co-Authored-By:` trailer crediting the current model.

## Code Practices

**Let the types and the structure carry each restriction.** When the reader has to be stopped from calling something or passing something, give the code a shape in which that use fails to compile: a method that exists only on the type where it is valid, or an argument of the function that owns the decision. `each_keyed` takes the key as an argument for this reason, so a key can sit only on the root of a list template. Where you reach for a comment, a runtime check, or a doc line to warn a caller, read it as a sign that the shape is still missing, and change the shape first. A restriction lives in prose only where the type system cannot hold it within this project's limits, such as the two-type-parameter ceiling, and then the prose names what goes unchecked and what happens when it is broken.

**A comment describes the code directly below it.** It gives the reason that code has its shape, or a fact about it the code cannot show by itself, such as an assumption about the browser or where the value below was produced. It speaks about that code alone, in terms a reader of that line can check there. History belongs in the commit message, and another file's behavior in that file.

**Decide what the work needs and act on it.** Hand back only a decision that is the user's to make: taste, risk tolerance, and anything that reaches outside this repository, such as publishing a crate or changing the GitHub repository's settings.
