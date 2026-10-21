#!/usr/bin/env bash
# Copies the template named by $1 to target/templates/$1, as a reader would
# get it, and points its hypersig at this repository's crate, so the tests
# build a template against the code they test rather than crates.io.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
name=$1
dest="$root/target/templates/$name"

mkdir -p "$dest"
# Keeps the copy's target, build, and node_modules, so a second run is quick.
find "$dest" -mindepth 1 -maxdepth 1 ! -name target ! -name build ! -name node_modules -exec rm -rf {} +
tar -C "$root/templates/$name" --exclude ./target --exclude ./build --exclude ./node_modules \
  --exclude ./.wrangler --exclude ./Cargo.lock -cf - . | tar -C "$dest" -xf -
mkdir -p "$dest/.cargo"
cat > "$dest/.cargo/config.toml" <<CONFIG
[patch.crates-io]
hypersig = { path = "$root/crates/hypersig" }
CONFIG
echo "$dest"
