#!/usr/bin/env bash
# Creates the project of the host named by $1 at target/templates/$1 with
# `cargo hypersig new`, as a reader would get it, and points its hypersig and
# hypersig_ui at this repository's crates, so the tests build a template
# against the code they test rather than crates.io. Cargo warns that the
# hypersig_ui patch goes unused in a template that does not depend on it.
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
name=$1
dest="$root/target/templates/$name"

# `new` writes only a directory that does not exist, so it writes a fresh one
# beside the copy, whose target and build stay, so that a second run is quick.
fresh="$root/target/templates/.new/my-$name"
rm -rf "$fresh"
cargo run --quiet --manifest-path "$root/Cargo.toml" -p hypersig_cli -- \
  new "$fresh" --host "$name" > /dev/null
mkdir -p "$dest"
find "$dest" -mindepth 1 -maxdepth 1 ! -name target ! -name build -exec rm -rf {} +
tar -C "$fresh" -cf - . | tar -C "$dest" -xf -
rm -rf "$fresh"
mkdir -p "$dest/.cargo"
cat > "$dest/.cargo/config.toml" <<CONFIG
[patch.crates-io]
hypersig = { path = "$root/crates/hypersig" }
hypersig_ui = { path = "$root/crates/hypersig_ui" }
CONFIG
echo "$dest"
