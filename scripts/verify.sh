#!/usr/bin/env bash
# Runs the checks a change needs before it is reported complete. The files
# changed since the branch left origin/main, committed or not, decide which
# checks run. Each check's log goes to target/verify/<name>.log, and the last
# lines print a table of every check as passed, failed, or not run with the
# reason. The script exits non-zero when a check failed.
#
#   scripts/verify.sh           checks for the changed files
#   scripts/verify.sh --all     every check
set -uo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"
nightly=nightly-2026-10-14
logs=target/verify
mkdir -p "$logs"

all=false
[[ ${1:-} == --all ]] && all=true

base=$(git merge-base origin/main HEAD)
changed=$( {
  git diff --name-only "$base"
  git ls-files --others --exclude-standard
} | sort -u)

# Whether a changed file matches the extended regex $1.
touches() {
  $all || grep -Eq "$1" <<<"$changed"
}

results=()
failed=false

# Runs the command after the name, logs its output, and records the outcome.
check() {
  local name=$1
  shift
  printf '== %s\n' "$name"
  if "$@" >"$logs/$name.log" 2>&1; then
    results+=("passed   $name")
  else
    results+=("FAILED   $name (see $logs/$name.log)")
    tail -n 30 "$logs/$name.log"
    failed=true
  fi
}

skip() {
  results+=("not run  $1: $2")
}

fresh_doc() {
  rm -rf target/doc && RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
}

site_links() {
  [[ -d target/doc ]] || fresh_doc || return 1
  cargo run -p site -- --out target/site --rustdoc target/doc
}

runtime_bundle() {
  (cd runtime && pnpm build) && git diff --exit-code crates/hypersig/assets
}

public_api() {
  for crate in hypersig hypersig_ui hypersig_style hypersig_i18n; do
    cargo +"$nightly" rustdoc -p "$crate" --all-features -- -Z unstable-options --output-format json || return 1
  done
  cargo +"$nightly" test -p hypersig --test public_api -- --ignored
}

# The benchmark of the branch and of the commit it left origin/main from,
# whose medians the commit body carries.
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

rust='\.rs$|\.stderr$|Cargo\.(toml|lock)$|rust-toolchain\.toml$|^\.cargo/'
if $all || grep -E "$rust" <<<"$changed" | grep -vq '^site/'; then
  check fmt cargo fmt --all --check
  check clippy cargo clippy --workspace --all-targets -- -D warnings
  check test cargo test --workspace
  check no-default-features cargo check -p hypersig --no-default-features
  check doc fresh_doc
  check site-links site_links
elif touches '^site/'; then
  check fmt cargo fmt --all --check
  check clippy-site cargo clippy -p site --all-targets -- -D warnings
  check test-site cargo test -p site
  check site-links site_links
else
  skip rust "no Rust file changed"
fi

if touches '^README(\.ja)?\.md$' && ! touches "$rust"; then
  check readme-doctests cargo test -p hypersig --doc
fi

wire='^runtime/|^crates/hypersig/src/(expr|html|page|patch|route)(\.rs$|/)|^crates/hypersig/tests/fixtures\.rs$'
if touches "$wire"; then
  check runtime-check bash -c 'cd runtime && pnpm check'
  check runtime-test bash -c 'cd runtime && pnpm test'
  check runtime-bundle runtime_bundle
else
  skip runtime "neither runtime/ nor the wire format changed"
fi

if touches '^runtime/src/|^crates/hypersig/src/html(\.rs$|/)|^examples/|^e2e/|^crates/hypersig_cli/templates/'; then
  if docker info >/dev/null 2>&1; then
    check e2e-check bash -c 'cd e2e && pnpm check'
    check e2e-test bash -c 'cd e2e && pnpm test'
  else
    skip e2e "Docker is not running (start it with colima start)"
  fi
else
  skip e2e "neither the runtime, the HTML builder, nor an example changed"
fi

if touches '^crates/hypersig/src/(html|page|ssr|expr)(\.rs$|/)'; then
  check bench bench
else
  skip bench "html, page, ssr, and expr are unchanged"
fi

if touches '^crates/hypersig/src/(ssr|signal|expr)(\.rs$|/)'; then
  check miri cargo +"$nightly" miri test -p hypersig --lib
else
  skip miri "ssr, signal, and expr are unchanged"
fi

if touches 'Cargo\.(toml|lock)$'; then
  check udeps cargo +"$nightly" udeps --workspace --all-targets
else
  skip udeps "no manifest changed"
fi

if touches '^crates/(hypersig|hypersig_ui|hypersig_style|hypersig_i18n)/src/'; then
  check public-api public_api
else
  skip public-api "no library source changed"
fi

if touches '^examples/postgres/(queries|migrations)/'; then
  if [[ -n ${DATABASE_URL:-} ]]; then
    check sqlx bash -c 'cargo sqlx migrate run --source examples/postgres/migrations && cd examples/postgres && cargo sqlx prepare --check'
  else
    skip sqlx "DATABASE_URL is not set"
  fi
else
  skip sqlx "no query or migration changed"
fi

printf '\n'
printf '%s\n' "${results[@]}"
! $failed
