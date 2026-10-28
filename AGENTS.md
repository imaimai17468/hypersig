# Project Instructions

hypersig is a server-driven hypermedia library for Rust. The server owns the state, renders HTML, and streams patches over Server-Sent Events. A small JavaScript runtime in the browser applies them and keeps a store of signals for local reactivity. Every string the browser acts on (signal names, attribute expressions, action URLs, page links, patch target ids, node ref names) is produced from a Rust type, so a mistake in any of them is a compile error.

That last sentence is the project's thesis. A change that lets a user write one of those strings by hand again, or that turns a compile error into a runtime failure, needs the user's own words.

## Procedures

- **While editing Rust**, build with blaze, which recompiles only the functions an edit changed: `blaze build`, `blaze run -p <package>`, `blaze test -p <crate>`. Run doctests with `cargo test --doc`, because blaze finds no error in a `compile_fail` doctest. Leave `blaze enable` off, because it replaces `cargo clippy` with blaze's own lints.
- **Before reporting a change complete**, run `scripts/verify.sh`. It runs fmt, clippy, and the tests of the packages the change touched since `origin/main`, the site's link check, the runtime's checks, and the benchmark where those files changed, and prints a table of each as passed, failed, or not run with the reason. Report that table as it printed. CI runs the rest on the pull request (the whole workspace's tests, rustdoc from empty, e2e, the templates, miri, udeps, the public API check, and sqlx), so wait for it to pass before merging.
- **Before writing an import path or an API call from memory**, read the pinned version's source under `~/.cargo/registry/src/` or its docs.rs page for that version (`syn` is 3.x and `axum` 0.8.x here). The language and standard library features of the pinned compiler are in `~/.rustup/toolchains/1.99-<host>/share/doc/rust/html/releases.md`. Where you cannot confirm a fact, say so in the sentence it limits.
- **When changing the wire format**, which is defined in `crates/hypersig/src/{expr,html,page,patch,route}.rs` and again in `runtime/src/`, change both sides in one commit, then run `HYPERSIG_FIXTURES=overwrite cargo test -p hypersig --test fixtures` and commit the fixtures under `runtime/test/fixtures/`.
- **When changing `runtime/`**, run `pnpm build` there and commit `crates/hypersig/assets/`, so that `cargo build` needs no Node. Fix a lint finding in the code, never with a disable comment or by turning the rule off.
- **When changing `Page::render` or the code it calls** (`html.rs`, `page.rs`, `ssr.rs`, `expr.rs`), put the two benchmark medians `scripts/verify.sh` writes to `target/verify/bench-*.txt` in the commit body.
- **When a trybuild `.stderr` changes**, read the new message and confirm it still names the mistake the case exists for, then write it with `TRYBUILD=overwrite cargo test -p hypersig --test ui`.
- **When adding or changing a public item**, keep its first doc sentence under 15 words. The public API check fails on a new string parameter or a longer sentence until `crates/hypersig/tests/public_api_strings.txt` or `public_api_summaries.txt` lists it. `HYPERSIG_PUBLIC_API=overwrite` rewrites both lists; read the diff, because each new string parameter is either checked when it is built or a place where a hand-written string reaches the browser. The site's reference and component pages copy each first sentence, so update `site/src/reference.rs` or `site/src/components.rs` with it.
- **When changing a page's text**, change both halves of its `L { en, ja }` in one commit. Invoke the yomiyasu skill on the Japanese with the document type `manual` (`pr` for a pull request, `promo` for an announcement), and the humanizer skill on English a person reads.
- **When changing `README.md`**, change `README.ja.md` in the same commit with the same facts, code, and tables.
- **When changing the MDN data in `crates/hypersig_style/data/`**, regenerate the style code with `HYPERSIG_GENERATE=overwrite cargo test -p hypersig_style --test generate`.
- **When changing `assets/og-en.svg` or `og-ja.svg`**, redraw its PNG with `"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless --hide-scrollbars --window-size=1200,630 --screenshot=assets/og-en.png "file://$PWD/assets/og-en.svg"`.
- **When changing a deploy template under `crates/hypersig_cli/templates/`**, name its manifest `Cargo.toml.template`, because `cargo package` leaves out a directory that holds a `Cargo.toml`. `crates/hypersig_cli/stage.sh` writes a template's project under `target/templates` for CI's lint and the e2e tests.
- **When changing a query or a migration of `examples/postgres`**, run `cargo sqlx prepare` there with `DATABASE_URL` set, because `.cargo/config.toml` sets `SQLX_OFFLINE=true` and the queries build from `examples/postgres/.sqlx`.
- **When raising the Rust version**, change `rust-toolchain.toml`, `rust-version` in `Cargo.toml`, the CI toolchain, and every regenerated `.stderr` in one commit, because the `.stderr` files hold rustc's exact output.
- **Before opening a pull request that changes Rust**, run the beautiful-rust plugin's `rust-review` skill on the diff and fix what it finds. `.claude/hooks/review-before-pr.sh` refuses the first `gh pr create` until then. Skip its suggestions of `unsafe` (`Cargo.toml` forbids it), of a `serde` feature (serde's traits are in the public API unconditionally), and of a public `&str` parameter (the thesis above), and its verification commands, which `scripts/verify.sh` replaces.

## Commits

- One commit has one purpose. Stage explicit paths, never `git add -A` or `git add .`.
- The first line starts with `feat` / `fix` / `refactor` / `test` / `docs` / `chore` and states what improves. The body says why for a `fix` or `refactor`.
- Commit messages, pull request titles and bodies, and review comments are in English, because a user who follows a change from a release reads them, and most users do not read Japanese. The release notes are the titles of the merged pull requests, so a Japanese title reaches them unchanged. Commits made before this rule stay as they are.
- End with a `Co-Authored-By:` trailer crediting the current model.

## Code Practices

**Let the types and the structure carry each restriction.** When the reader has to be stopped from calling something or passing something, give the code a shape in which that use fails to compile: a method that exists only on the type where it is valid, or an argument of the function that owns the decision. `each_keyed` takes the key as an argument for this reason, so a key can sit only on the root of a list template. Where you reach for a comment, a runtime check, or a doc line to warn a caller, read it as a sign that the shape is still missing, and change the shape first. A restriction lives in prose only where the type system cannot hold it within this project's limits, such as the two-type-parameter ceiling, and then the prose names what goes unchecked and what happens when it is broken.

**A comment describes the code directly below it.** It gives the reason that code has its shape, or a fact about it the code cannot show by itself, such as an assumption about the browser or where the value below was produced. It speaks about that code alone, in terms a reader of that line can check there. History belongs in the commit message, and another file's behavior in that file.

**Decide what the work needs and act on it.** Hand back only a decision that is the user's to make: taste, risk tolerance, and anything that reaches outside this repository, such as publishing a crate or changing the GitHub repository's settings.
