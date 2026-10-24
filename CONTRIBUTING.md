# Contributing

Bug reports, questions, and pull requests are welcome. Write them in English or Japanese.

## Reporting a bug

Open an [issue](https://github.com/imaimai17468/hypersig/issues) with the hypersig version, the smallest code that shows the problem, what you expected, and what happened. When the browser is involved, add the browser's name and version and any error in its console.

## Making a change

hypersig's rule is that every string the browser acts on comes from a Rust type, so a mistake in one is a compile error. A change that lets a user write one of those strings by hand, or that turns a compile error into a failure at run time, is unlikely to be merged. Open an issue first when a change adds public API, so the shape can be settled before the code is written.

The tools a change needs:

- Rust: `rust-toolchain.toml` pins the compiler, and rustup installs it on the first `cargo` command.
- Node.js 22 and pnpm 10, only for a change under `runtime/` or `e2e/`.
- Google Chrome and Docker, only for the Playwright tests under `e2e/`.

[`AGENTS.md`](AGENTS.md) describes the repository layout, the checks to run before opening a pull request, and the commit message format. The browser runtime and the Rust crates define the wire format between them twice, and the section there on the fixtures explains how a change to one side is kept in step with the other.
