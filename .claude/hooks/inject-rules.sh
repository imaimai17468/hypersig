#!/usr/bin/env bash
# Puts .claude/rules/rust.md and .claude/rules/prose.md into the model's context
# right before the tool call they govern. The whole file goes in the first time
# an agent of a session needs it, and a short checklist on each later call,
# because the whole file on every edit would fill the context with copies.
#
# PreToolUse: reads the tool call on stdin and prints the context as JSON.
# SessionStart: forgets which files were sent, so that after a compaction or a
# clear, which drop the earlier copy from the context, the next call sends the
# whole file again.
set -euo pipefail

input=$(cat)
root=${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/../.." && pwd)}
session=$(jq -r '.session_id // "none"' <<<"$input")
state="${TMPDIR:-/tmp}/hypersig-rules"
mkdir -p "$state"

if [[ $(jq -r '.hook_event_name // ""' <<<"$input") == SessionStart ]]; then
  rm -f "$state/$session".*
  exit 0
fi

# A subagent starts with a context of its own, so it gets its own first copy.
agent=$(jq -r '.agent_id // "main"' <<<"$input")
tool=$(jq -r '.tool_name // ""' <<<"$input")
path=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // ""' <<<"$input")
command=$(jq -r '.tool_input.command // ""' <<<"$input")

kinds=()
case $tool in
  Edit | Write | MultiEdit | NotebookEdit)
    case $path in
      *.rs) kinds+=(rust) ;;
    esac
    # Comments and doc comments are prose, and the site's pages hold their text in `.rs` files.
    case $path in
      *.rs | *.md | *.ts | *.toml | *.yml) kinds+=(prose) ;;
    esac
    ;;
  Bash)
    # A commit message, a pull request, and a review comment are prose a person reads.
    if grep -Eq '(^|[;&|[:space:]])(git[[:space:]]+commit|gh[[:space:]]+pr[[:space:]]+(create|edit|comment|review)|gh[[:space:]]+api[^;&|]*comments)' <<<"$command"; then
      kinds+=(prose)
    fi
    ;;
esac

[[ ${#kinds[@]} -eq 0 ]] && exit 0

checklist_rust='Rust checklist (.claude/rules/rust.md, sent in full earlier in this context):
- No public parameter that takes &str or String for a signal path, an element id, a URL, an expression, an event name, or CSS. No compile error turned into a runtime check.
- No unsafe. Override a lint with #[expect(..., reason = "...")], never #[allow].
- A broken invariant panics; bad input returns Result.
- No trait with one implementor and no generic caller. No function that only forwards its arguments. Borrow before cloning.
- A comment gives the reason for the code directly below it, or a fact that code cannot show. No history, no restating the line.
- A new public item has a summary under 15 words and # Examples / # Errors / # Panics where they apply. A promised compile error gets a trybuild case in the same commit.'

checklist_prose='Prose checklist (.claude/rules/prose.md, sent in full earlier in this context):
- Japanese: yomiyasu lint runs after each edit (lint-prose.py); fix what applies. Polite です/ます, no dash, no i-adjective + です, no two adversative connectives in a row.
- English: apply the humanizer skill before committing. Em dashes at most 5 per 1000 words (prose.md wins over humanizer here).
- Delete-to-test: an evaluative modifier or intensifying adverb whose deletion leaves the same checkable fact goes. A copula substitute or light verb becomes the plain verb.
- Named actor, the action with its object, the specific fact. Cut metadiscourse (a sentence about the document itself).
- Claim first, no negated alternative before it, no reveal, no closing aphorism, two items rather than three.
- Open with what was decided. Keep a hedge that carries real uncertainty; give a guarantee its condition.
- Commit messages and pull requests are in English, start with feat/fix/refactor/test/docs/chore, and say why for a fix or refactor.'

context=""
for kind in "${kinds[@]}"; do
  marker="$state/$session.$agent.$kind"
  if [[ -e $marker ]]; then
    var="checklist_$kind"
    context+="${!var}"$'\n\n'
  else
    context+="The rules below govern the call you are about to make. Follow them; where AGENTS.md is more specific, AGENTS.md wins."$'\n\n'
    context+=$(cat "$root/.claude/rules/$kind.md")$'\n\n'
    touch "$marker"
  fi
done

jq -n --arg context "$context" '{hookSpecificOutput: {hookEventName: "PreToolUse", additionalContext: $context}}'
