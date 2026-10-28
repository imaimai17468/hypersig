#!/usr/bin/env bash
# Refuses the first `gh pr create` of a branch whose diff changes Rust once,
# so that the beautiful-rust plugin's rust-review skill reads the diff before a
# person does. The marker lets the same command through on the next try.
#
# PreToolUse on Bash: reads the tool call on stdin and prints the decision as JSON.
set -euo pipefail

input=$(cat)
command=$(jq -r '.tool_input.command // ""' <<<"$input")
grep -Eq '(^|[;&|[:space:]])gh[[:space:]]+pr[[:space:]]+create' <<<"$command" || exit 0

root=${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}
session=$(jq -r '.session_id // "none"' <<<"$input")
state="${TMPDIR:-/tmp}/hypersig-review"
mkdir -p "$state"
branch=$(git -C "$root" rev-parse --abbrev-ref HEAD 2>/dev/null || echo none)
marker="$state/$session.${branch//\//_}"
if [[ ! -e $marker ]] && git -C "$root" diff --name-only origin/main...HEAD 2>/dev/null | grep -q '\.rs$'; then
  touch "$marker"
  jq -n --arg reason "This branch changes Rust files. Before opening the pull request, invoke the beautiful-rust:rust-review skill on the diff against origin/main, fix its Must fix findings and the Should fix findings that apply, and commit. AGENTS.md wins where the skill disagrees (unsafe, serde behind a feature, public &str parameters), and its verification list replaces the skill's commands. Then run the same command again." \
    '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "deny", permissionDecisionReason: $reason}}'
fi
