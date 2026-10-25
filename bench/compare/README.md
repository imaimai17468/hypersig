# Comparing hypersig with Topcoat, Next.js, React, TanStack Start, Nuxt, Datastar, Leptos, and Dioxus

The same 2000-item TodoMVC in nine frameworks, checked for identical behavior and measured in one Chrome harness. `SPEC.md` is the contract every app follows and defines each measurement. `results/latest.md` holds the latest numbers.

## Build

From this directory:

```sh
cargo build --release --manifest-path datastar/Cargo.toml
(cd topcoat && topcoat asset bundle --release)
(cd next && npm ci && npm run build)
(cd react && npm ci && npm run build)
(cd tanstack-start && npm ci && npm run build)
(cd nuxt && npm ci && npm run build)
(cd leptos && cargo leptos build --release)
(cd dioxus && dx build --release --web)
pnpm install
```

The Topcoat build needs `cargo install topcoat-cli --version 0.10.0`, the Leptos build `cargo install cargo-leptos --version 0.3.11 --locked`, and the Dioxus build `cargo install dioxus-cli --version 0.7.10 --locked`. The last two compile the browser half to `wasm32-unknown-unknown`, which `rustup target add wasm32-unknown-unknown` installs. `topcoat asset bundle` builds the release binary and writes the runtime script beside it in `topcoat/target/release/assets`, where the binary loads it from.

The harness builds the hypersig app itself with `cargo build --release` each time it starts it, because the binary embeds `crates/hypersig/assets/hypersig.js` and would otherwise keep serving the runtime from its last build. The other apps do not depend on this repository's runtime and are started from whatever the commands above last built.

## Run

```sh
node harness/parity.mjs           # every app behaves as SPEC.md says
node harness/measure.mjs          # writes results/latest.json and results/latest.md
node harness/measure.mjs datastar # one app only
```

The harness starts each app itself, one at a time, and drives the Chrome installed on the machine. A full run takes about an hour. CI does not run it, because it needs Node, the Topcoat, Leptos, and Dioxus CLIs, and a local Chrome.
