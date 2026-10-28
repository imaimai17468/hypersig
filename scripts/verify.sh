#!/usr/bin/env bash
# Runs the quick checks of the packages a change touched, before it is
# reported complete. The files changed since the branch left origin/main,
# committed or not, decide which checks run. CI runs the rest on every pull
# request: the whole workspace's tests, rustdoc, e2e, the templates, miri,
# udeps, the public API check, and sqlx. Each check's log goes to
# target/verify/<name>.log, and the last lines print a table of every check
# as passed, failed, or not run with the reason. The script exits non-zero
# when a check failed.
set -uo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"
logs=target/verify
mkdir -p "$logs"

base=$(git merge-base origin/main HEAD)
changed=$( {
  git diff --name-only "$base"
  git ls-files --others --exclude-standard
} | sort -u)

# Whether a changed file matches the extended regex $1.
touches() {
  grep -Eq "$1" <<<"$changed"
}

results=()
failed=false

# Runs the command after the name, logs its output, and records the outcome.
check() {
  local name=$1
  shift
  printf '== %s\n' "$name"
  local start=$SECONDS
  if "$@" >"$logs/$name.log" 2>&1; then
    results+=("passed   $name ($((SECONDS - start)) s)")
  else
    results+=("FAILED   $name (see $logs/$name.log)")
    tail -n 30 "$logs/$name.log"
    failed=true
  fi
}

skip() {
  results+=("not run  $1: $2")
}

# The workspace packages that hold a changed file, from cargo's own list of
# each package's directory.
packages=$(cargo metadata --no-deps --format-version 1 | python3 -c '
import json, os, sys
dirs = {os.path.dirname(p["manifest_path"]): p["name"] for p in json.load(sys.stdin)["packages"]}
root = sys.argv[1]
found = set()
for line in sys.argv[2].splitlines():
    path = os.path.join(root, line)
    owner = max((d for d in dirs if path.startswith(d + "/")), key=len, default=None)
    if owner:
        found.add(dirs[owner])
print(" ".join(sorted(found)))
' "$root" "$changed")
flags=()
for package in $packages; do
  flags+=(-p "$package")
done

if touches '\.rs$|\.stderr$|Cargo\.(toml|lock)$'; then
  check fmt cargo fmt --all --check
fi

if touches '^Cargo\.(toml|lock)$|^rust-toolchain\.toml$|^\.cargo/'; then
  check clippy cargo clippy --workspace --all-targets -- -D warnings
  skip test "the workspace manifest changed, so CI runs every package's tests"
elif [[ ${#flags[@]} -gt 0 ]]; then
  check clippy cargo clippy "${flags[@]}" --all-targets -- -D warnings
  check test cargo test "${flags[@]}"
else
  skip rust "no package changed"
fi

if touches '^README(\.ja)?\.md$' && [[ " $packages " != *" hypersig "* ]]; then
  check readme-doctests cargo test -p hypersig --doc
fi

# The link check reads target/doc as it is, which CI writes from empty.
if touches '^site/'; then
  if [[ -d target/doc ]]; then
    check site-links cargo run -p site -- --out target/site --rustdoc target/doc
  else
    check site-links bash -c 'RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps && cargo run -p site -- --out target/site --rustdoc target/doc'
  fi
fi

if touches '^runtime/|^crates/hypersig/src/(expr|html|page|patch|route)(\.rs$|/)|^crates/hypersig/tests/fixtures\.rs$'; then
  check runtime-check bash -c 'cd runtime && pnpm check'
  check runtime-test bash -c 'cd runtime && pnpm test'
  check runtime-bundle bash -c 'cd runtime && pnpm build && git diff --exit-code ../crates/hypersig/assets'
fi

# CI does not run the benchmark, so the branch and the commit it left
# origin/main from both run here, and the commit body carries the medians.
bench() {
  local tree=$logs/base
  git worktree remove --force "$tree" 2>/dev/null
  git worktree add --detach "$tree" "$base" &&
    (cd "$tree" && CARGO_TARGET_DIR="$root/target/verify-base" cargo bench -p hypersig --bench render) >"$logs/bench-base.txt" 2>&1 &&
    cargo bench -p hypersig --bench render >"$logs/bench-branch.txt" 2>&1
  local status=$?
  git worktree remove --force "$tree" 2>/dev/null
  printf 'medians: %s and %s\n' "$logs/bench-base.txt" "$logs/bench-branch.txt"
  return $status
}
if touches '^crates/hypersig/src/(html|page|ssr|expr)(\.rs$|/)'; then
  check bench bench
fi

printf '\n'
printf '%s\n' "${results[@]}"
! $failed
