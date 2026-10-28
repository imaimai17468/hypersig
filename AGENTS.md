# Project Instructions

hypersig is a server-driven hypermedia library for Rust. The server owns the state, renders HTML, and streams patches over Server-Sent Events. A small JavaScript runtime in the browser applies them and keeps a store of signals for local reactivity. Every string the browser acts on (signal names, attribute expressions, action URLs, page links, patch target ids, node ref names) is produced from a Rust type, so a mistake in any of them is a compile error.

That last sentence is the project's thesis. A change that lets a user write one of those strings by hand again, or that turns a compile error into a runtime failure, needs the user's own words.

## Layout

| Path | What lives there |
|---|---|
| `crates/hypersig` | The library: `Signals`, `Sig`, `Expr`, `Stmt`, the HTML builder, `Action`, `Route`, `Fragment`, `Patches`, the axum integration, and the embedded runtime |
| `crates/hypersig_proc` | `#[derive(Signals)]`, `#[derive(Segment)]`, and `#[derive(Search)]` as a thin `proc-macro = true` shim |
| `crates/hypersig_proc_impl` | The token transformations behind the derives, with their snapshot tests |
| `crates/hypersig_style` | Typed atomic CSS: `Style`, `Sheet`, the value types, and the code `tests/generate.rs` writes from the MDN data in `data/`. Regenerate it with `HYPERSIG_GENERATE=overwrite cargo test -p hypersig_style --test generate` |
| `crates/hypersig_i18n` | Typed translations: the `Locale` trait, `negotiate` over `Accept-Language`, and the redirect from `/` to a locale's page |
| `crates/hypersig_ui` | Components styled after shadcn/ui on native elements and the WAI-ARIA patterns, with trybuild cases under `tests/ui/fail` |
| `crates/hypersig_cli` | `cargo hypersig new`, `cargo hypersig add`, `cargo hypersig dev`, and `cargo hypersig elements`, which writes a Rust module from a library's `custom-elements.json` and whose expected output for the manifest under `tests/elements/` is `tests/elements/expected.rs`, with the deploy templates under `templates/` compiled into the binary. Each template's manifest is `Cargo.toml.template`, because `cargo package` leaves out a directory that holds a `Cargo.toml`. `stage.sh` writes a template's project under `target/templates` with its hypersig pointed at this repository, for CI's lint and the e2e tests. `steps/` holds the code `add` prints, which the deploy pages show |
| `crates/hypersig/benches/render.rs` | divan benchmarks of `Page::render` over list pages, with allocation counts |
| `crates/hypersig/tests/ui` | trybuild cases. Each `fail/*.rs` is a mistake that must not compile |
| `crates/hypersig/tests/fixtures.rs` | Builds pages with the Rust builder and writes them to `runtime/test/fixtures/`, which the runtime tests load |
| `runtime/` | The browser runtime in TypeScript. `pnpm build` writes `crates/hypersig/assets/hypersig.js`, its Brotli copy `hypersig.js.br`, and `dev.js` from `src/dev.ts`, which only a debug build serves, all committed so that `cargo build` needs no Node |
| `assets/` | The hypersig icon `hypersig.svg`, and the link preview pictures `og-en.png` and `og-ja.png` drawn from `og-en.svg` and `og-ja.svg`, which the site serves. Redraw a PNG after changing its SVG with `"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless --hide-scrollbars --window-size=1200,630 --screenshot=assets/og-en.png "file://$PWD/assets/og-en.svg"` |
| `examples/` | Runnable axum applications |
| `e2e/` | Playwright tests that start every example and drive it in Chrome |
| `site/` | The documentation site, built with hypersig and `hypersig_ui` and published to GitHub Pages by `.github/workflows/pages.yml`. `cargo run -p site -- --out DIR --base ""` writes it for a local preview. Its code samples are doctests under `site/snippets/`, programs under `site/examples/`, or live demos under `site/src/demo/`. Every page renders once per language of `site/src/lang.rs`, English at the root and Japanese under `ja/`, and a text of a page is an `L { en, ja }`, so a change to the English text changes the Japanese text in the same commit |

The wire format between the two halves is defined twice, once in `crates/hypersig/src/expr.rs`, `html.rs`, `page.rs`, `patch.rs`, and `route.rs` and once in `runtime/src/`. A change to one is a change to both in the same commit. `tests/fixtures.rs` fails when a committed fixture differs from what the builder writes now, and the tests under `runtime/test/features/` run those fixtures, so a change on one side that the other side does not follow fails one of the two test suites. Regenerate the fixtures with `HYPERSIG_FIXTURES=overwrite cargo test -p hypersig --test fixtures`, then run `pnpm test`.

## Edit loop

While editing, build with [blaze](https://github.com/indexable-inc/blaze), which compiles only the functions an edit changed: `blaze build`, `blaze run -p <package>`, and `blaze test -p <crate>`. After an edit to a function body in `html.rs`, `blaze build -p site` took 0.7 to 0.9 s on one Mac where `cargo build` took 5.7 to 7.2 s. Install it with `curl -fsSL https://raw.githubusercontent.com/indexable-inc/blaze/main/install.sh | sh`, and leave `blaze enable` off, because it links `cargo clippy` to blaze's own 24 lints.

blaze builds with its own toolchain (Rust nightly-2026-10-27 for blaze 0.20261027.0), not the 1.99 that `rust-toolchain.toml` pins, so run `cargo check` before reporting, and run the list below with cargo. `cargo test --doc` stays on cargo while editing too: blaze 0.20261027.0 took 105 s for `hypersig`'s 128 doctests where cargo took 17 s, and failed the two marked `compile_fail`, in which it found no error.

The tests that start another program run outside blaze 0.1.0's sandbox, by `[package.metadata.blaze.sandbox]` in `crates/hypersig/Cargo.toml`, `crates/hypersig_ui/Cargo.toml` (trybuild starts cargo), and `crates/hypersig_cli/Cargo.toml` (its tests start `cargo-hypersig`).

## Verification before completion

No hook runs these. Run them yourself before reporting a change complete:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` (this includes the trybuild cases)
- `cargo check -p hypersig --no-default-features`, because the axum integration is behind a feature
- `rm -rf target/doc && RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, which fails on a broken intra-doc link. rustdoc adds each run's entries to `target/doc/search.index` without removing the previous ones, and every crate it documents rewrites that index, so on a kept `target/doc` the step slows with each run (on one machine, 28 MB of index made the step take 573 seconds, and from empty it was 3.3 MB and under two minutes). A kept `target/doc` also still holds the pages of items that no longer exist, which the site step below would accept as link targets
- `cargo run -p site -- --out target/site --rustdoc target/doc` after that, which fails when a page of the documentation site links to an item the rustdoc output no longer has. The site's reference and component pages copy the first sentence of each doc comment, some of them shortened, so a change to one updates `site/src/reference.rs` or `site/src/components.rs` too
- `cargo bench -p hypersig --bench render` before and after the change, where it touches `html.rs`, `page.rs`, `ssr.rs`, or `expr.rs`. The commit body carries both medians.
- In `runtime/`: `pnpm check` (the type check, `oxfmt --check`, and `oxlint` with the categories `.oxlintrc.json` enables) and `pnpm test`, where the change touches `runtime/` or the wire format. A lint finding is fixed in the code, never silenced with a disable comment or by turning the rule off
- In `runtime/`: `pnpm build`, then `git diff --exit-code crates/hypersig/assets`, so the committed bundle matches the source
- `cd examples/postgres && cargo sqlx prepare --check`, with `DATABASE_URL` naming a PostgreSQL that `cargo sqlx migrate run --source examples/postgres/migrations` has migrated, where the change touches a query under `examples/postgres/queries/` or a migration. The `sqlx::query!` macros build from `examples/postgres/.sqlx`, because `.cargo/config.toml` sets `SQLX_OFFLINE=true`, so a changed query fails to compile until `cargo sqlx prepare` writes that directory again
- In `e2e/`: `pnpm check` and `pnpm test`, where the change touches the runtime, the HTML builder, or an example. It drives the installed Google Chrome, both locally and on CI, where GitHub's runner image has it, and runs PostgreSQL for `examples/postgres` with Docker, which the runner image also has and which has to be running locally.

A trybuild `.stderr` file records the compiler's message. Regenerate it with `TRYBUILD=overwrite cargo test -p hypersig --test ui` only after reading the new message and confirming it still names the mistake the case exists for.

`rust-toolchain.toml` pins the compiler and installs `rust-src`, and CI uses the same version and components, because the trybuild `.stderr` files hold rustc's exact output and rustc quotes standard library source only where `rust-src` is present. Raising the pin is one commit that changes `rust-toolchain.toml`, `rust-version` in `Cargo.toml`, the CI toolchain, and every regenerated `.stderr` together.

CI also runs `cargo udeps --workspace --all-targets` and `cargo miri test -p hypersig --lib` on the nightly pinned in `.github/workflows/ci.yml`. Run them locally with `cargo +nightly-2026-10-14 ...` when a change adds a dependency or touches `ssr.rs`, `signal.rs`, or `expr.rs`. `hypersig_proc_impl` is left out of miri because insta spawns a process to find the workspace, which miri cannot do.

The same nightly job checks the public API with `crates/hypersig/tests/public_api.rs`: no string parameter where a type belongs, no third type parameter, no `Arc`, `Box`, `Rc`, or `RefCell`, one public path per item, third-party types only from its `THIRD_PARTY` list, and a first doc sentence under 15 words. Run it when a change adds or changes a public item: write the rustdoc JSON of each library crate with `cargo +nightly-2026-10-14 rustdoc -p <crate> --all-features -- -Z unstable-options --output-format json` for `hypersig`, `hypersig_ui`, `hypersig_style` and `hypersig_i18n`, then `cargo +nightly-2026-10-14 test -p hypersig --test public_api -- --ignored`. A new string parameter or a long first doc sentence fails it until `tests/public_api_strings.txt` or `tests/public_api_summaries.txt` lists it. `HYPERSIG_PUBLIC_API=overwrite` rewrites both lists; read the diff, because each new string parameter is either checked when it is built or a place where a hand-written string reaches the browser.

Report a step you could not run as "not run", never as "passed".

## Rules

- The `beautiful-rust` plugin, enabled in `.claude/settings.json`, loads its `rust-best-practices` skill on every Rust task, and its `rust-review` skill reviews a branch's Rust diff before the pull request opens. `.claude/hooks/review-before-pr.sh` refuses the first `gh pr create` of a branch whose commits change a `.rs` file, with a reason that asks for that review, and lets the same command through on the next try. The skills are written for any Rust project, so this file wins where they disagree. Three disagreements are known: the skill allows `unsafe` for three reasons, where this workspace forbids it; it puts serde impls behind a `serde` feature, where `serde`'s traits sit in the public API unconditionally; and its checklist accepts a public `&str` parameter, where a public function that takes `&str` for a signal path, an element id, or an expression breaks this project's thesis. The verification commands of both skills, and the lints `rust-review` offers, are replaced by the list under Verification before completion and `[workspace.lints]`.
- The `yomiyasu` plugin checks Japanese a person reads. Invoke its skill on Japanese you write or revise, naming the document type: `manual` for the site and the READMEs, `pr` for a pull request, `promo` for an announcement.
- The `humanizer` plugin checks English. Apply its skill to English a person reads before committing.
- `README.md` (English, which crates.io shows) and `README.ja.md` carry the same facts, code, and tables. A change to one changes the other in the same commit, and `lib.rs` compiles the Rust code block of each as a doctest.

## Knowledge Currency

Your training data goes stale. `syn` is at 3.x here and `axum` at 0.8.x, and both differ from the versions most examples on the web were written against. Before writing an import path or an API call from memory, read the pinned version's source under `~/.cargo/registry/src/` or its docs.rs page for that exact version. The language features and standard library APIs the pinned compiler offers are listed in `~/.rustup/toolchains/<channel in rust-toolchain.toml>-<host>/share/doc/rust/html/releases.md`, which `rust-toolchain.toml` installs, so read it before choosing an older idiom over a newer API. Where you cannot confirm a fact, write that into the sentence it limits.

## Commits

- One commit has one purpose. Stage explicit paths, never `git add -A` or `git add .`.
- The first line starts with `feat` / `fix` / `refactor` / `test` / `docs` / `chore` and states what improves. The body says why for a `fix` or `refactor`.
- Commit messages, pull request titles and bodies, and review comments are in English, because a user who follows a change from a release reads them, and most users do not read Japanese. The release notes are the titles of the merged pull requests, so a Japanese title reaches them unchanged. Commits made before this rule stay as they are.
- End with a `Co-Authored-By:` trailer crediting the current model.

## Code Practices

**Let the types and the structure carry each restriction.** When the reader has to be stopped from calling something or passing something, give the code a shape in which that use fails to compile: a method that exists only on the type where it is valid, or an argument of the function that owns the decision. `each_keyed` takes the key as an argument for this reason, so a key can sit only on the root of a list template. Where you reach for a comment, a runtime check, or a doc line to warn a caller, read it as a sign that the shape is still missing, and change the shape first. A restriction lives in prose only where the type system cannot hold it within this project's limits, such as the two-type-parameter ceiling, and then the prose names what goes unchecked and what happens when it is broken.

**A comment describes the code directly below it.** It gives the reason that code has its shape, or a fact about it the code cannot show by itself, such as an assumption about the browser or where the value below was produced. It speaks about that code alone, in terms a reader of that line can check there. History belongs in the commit message, and another file's behavior in that file.

**Decide what the work needs and act on it.** Hand back only a decision that is the user's to make: taste, risk tolerance, and anything that reaches outside this repository, such as publishing a crate or changing the GitHub repository's settings.
