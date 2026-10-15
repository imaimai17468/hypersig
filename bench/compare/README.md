# Comparing hypersig with Leptos, Next.js, React, TanStack Start, Nuxt, and htmx

The same 2000-item TodoMVC in seven frameworks, checked for identical behavior and measured in one Chrome harness. `SPEC.md` is the contract every app follows and defines each measurement. `results/latest.md` holds the latest numbers.

## Build

From this directory:

```sh
cargo build --release --manifest-path hypersig/Cargo.toml
cargo build --release --manifest-path htmx/Cargo.toml
(cd leptos && cargo leptos build --release)
(cd next && npm ci && npm run build)
(cd react && npm ci && npm run build)
(cd tanstack-start && npm ci && npm run build)
(cd nuxt && npm ci && npm run build)
pnpm install
```

The Leptos build needs `rustup target add wasm32-unknown-unknown` and `cargo install cargo-leptos --version 0.3.11`. Where `RUSTUP_TOOLCHAIN` is set in the shell, it overrides the repository's `rust-toolchain.toml`, so unset it for that build.

## Run

```sh
node harness/parity.mjs           # every app behaves as SPEC.md says
node harness/measure.mjs          # writes results/latest.json and results/latest.md
node harness/measure.mjs htmx     # one app only
```

The harness starts each app itself, one at a time, and drives the Chrome installed on the machine. A full run takes about ten minutes. CI does not run it, because it needs Node, the wasm toolchain, and a local Chrome.
