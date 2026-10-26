#!/usr/bin/env python3
"""Lints the prose an edit just wrote and hands the findings back as context.

PostToolUse on Edit, Write, and MultiEdit: reads the tool call on stdin and,
when it finds something, prints JSON whose additionalContext lists it. The
findings are candidates for review, so the hook never blocks and always exits 0.

Japanese text goes to the yomiyasu plugin's yomiyasu_lint.py, plus the two
checks of .claude/rules/prose.md that yomiyasu does not make. English text gets
the two checks of prose.md that can be counted. The humanizer skill and a
reading by eye cover the rest.
"""

import json
import os
import re
import subprocess
import sys

MAX_FINDINGS = 20
EM_DASHES_PER_1000_WORDS = 5

# A string literal of the site's L { en, ja }, each of which sits on one line.
# The `&[...]` arrays of table headers are left out.
SITE_TEXT = re.compile(r'\b(en|ja):\s*"((?:[^"\\]|\\.)*)"')
LINE_COMMENT = re.compile(r"^\s*//[/!]?\s?(.*)$")
FENCE = re.compile(r"^\s*(```|~~~)")
INLINE_CODE = re.compile(r"`[^`\n]*`")
JAPANESE = re.compile(r"[\u3040-\u30ff\u4e00-\u9fff]")
LATIN_WORD = re.compile(r"[A-Za-z]+(?:'[A-Za-z]+)?")

# A `not` after an auxiliary negates the verb and the `but` joins two clauses
# ("hypersig does not read it, but sqlx checks it"), so the first pattern skips it.
NEGATION_CONTRAST = [
    re.compile(r"(?<!\bdo )(?<!\bdoes )(?<!\bdid )(?<!\bcan )(?<!\bcould )(?<!\bwill )(?<!\bwould )"
               r"(?<!\bshould )(?<!\bmust )(?<!\bmay )(?<!\bmight )\bnot (?:just |only |merely |simply )?[^.;:!?\n]{1,80}?,? but (?:also |rather )?\w", re.I),
    re.compile(r"\b(?:it|this|that)(?:'s| is) not [^.;!?\n]{1,60}[,;] (?:it|this|that)(?:'s| is)\b", re.I),
    re.compile(r"\b(?:isn't|is not|aren't|are not) [^.!?\n]{1,60}\. (?:It|This|That|They)(?:'s|'re| is| are)\b"),
]

# An i-adjective, or the adjective-like auxiliaries ない and たい, directly before です.
I_ADJECTIVE_DESU = re.compile(r"[\u3041-\u3093\u4e00-\u9fff]{1,4}いです")
# Nouns and na-adjectives that end in い, where です is regular.
I_ENDING_NOUNS = ("違い", "扱い", "使い", "思い", "問い", "行い", "願い", "払い", "合い", "迷い", "勢い", "嫌い",
                  "幸い", "きれい", "きらい", "綺麗", "せい", "ぐらい", "くらい")
ADVERSATIVE = re.compile(r"^(?:ただし|ただ、|一方で|一方、|とはいえ|しかし|ですが|でも、|もっとも|それでも|にもかかわらず|現実的には|とはいうものの)")
JA_SENTENCE = re.compile(r"[^。！？!?\n]+[。！？!?]?")


def sentence_around(text, match, ends):
    """The sentence of `text` that holds `match`, found between the sentence ends in `ends`."""
    start = max(text.rfind(end, 0, match.start()) for end in ends + "\n") + 1
    stops = [i for i in (text.find(end, match.end()) for end in ends + "\n") if i >= 0]
    return text[start:min(stops) + 1 if stops else len(text)].strip()


def written_text(tool_input):
    if "content" in tool_input:
        return tool_input.get("content") or ""
    if "edits" in tool_input:
        return "\n".join(edit.get("new_string", "") for edit in tool_input.get("edits") or [])
    return tool_input.get("new_string") or ""


def split_source(path, text):
    """Returns the (english, japanese) prose a file of this kind holds in `text`."""
    english, japanese = [], []
    if path.endswith(".rs") or path.endswith(".ts"):
        for line in text.splitlines():
            for lang, literal in SITE_TEXT.findall(line):
                (japanese if lang == "ja" else english).append(literal.replace('\\"', '"'))
            comment = LINE_COMMENT.match(line)
            if comment:
                english.append(comment.group(1))
    elif path.endswith(".md"):
        fenced = False
        for line in text.splitlines():
            if FENCE.match(line):
                fenced = not fenced
                continue
            if not fenced:
                english.append(INLINE_CODE.sub("", line))
        # A Markdown file is in one language. README.ja.md and the Japanese skill
        # files carry kana on most lines, where an English file quotes a word at most.
        lines = [line for line in english if line.strip()]
        if lines and sum(bool(JAPANESE.search(line)) for line in lines) * 2 > len(lines):
            english, japanese = [], english
    return "\n".join(english), "\n".join(japanese)


def yomiyasu_lint():
    config = os.environ.get("CLAUDE_CONFIG_DIR") or os.path.join(os.path.expanduser("~"), ".claude")
    try:
        with open(os.path.join(config, "plugins", "installed_plugins.json"), encoding="utf-8") as file:
            installs = json.load(file).get("plugins", {}).get("yomiyasu@yomiyasu", [])
    except (OSError, ValueError, AttributeError):
        return None
    for install in installs:
        script = os.path.join(install.get("installPath", ""), "skills", "yomiyasu", "scripts", "yomiyasu_lint.py")
        if os.path.isfile(script):
            return script
    return None


def lint_japanese(text):
    findings = []
    script = yomiyasu_lint()
    if script:
        env = dict(os.environ, PYTHONDONTWRITEBYTECODE="1")
        try:
            run = subprocess.run([sys.executable, script, "--json", "-"], input=text, capture_output=True,
                                 text=True, encoding="utf-8", timeout=6, env=env)
            result = json.loads(run.stdout)
        except (OSError, ValueError, subprocess.SubprocessError):
            result = {}
        explained = set()
        for finding in result.get("findings", []):
            rule = finding.get("rule", "")
            # An edit is a fragment of its document, so a density over it says nothing about the document.
            if rule in ("excess_list", "excess_bold"):
                continue
            # The polite register prose.md asks for ends most sentences in です or ます.
            if rule == "sentence_end_repetition" and re.search("「(です|ます)」", finding.get("message", "")):
                continue
            line = f"yomiyasu {rule}: {finding.get('snippet', '').strip()}"
            if rule not in explained:
                explained.add(rule)
                line += f"\n    ({finding.get('message', '')})"
            findings.append(line)
    else:
        findings.append("yomiyasu is not installed, so only the two checks below ran on this Japanese text.")
    for match in I_ADJECTIVE_DESU.finditer(text):
        if not match.group(0)[:-2].endswith(I_ENDING_NOUNS):
            findings.append(f"prose.md i-adjective + です: {sentence_around(text, match, '。！？')}")
    sentences = [s.strip() for s in JA_SENTENCE.findall(text) if s.strip()]
    for before, after in zip(sentences, sentences[1:]):
        if ADVERSATIVE.match(before) and ADVERSATIVE.match(after):
            findings.append(f"prose.md adversatives back to back: {before} {after}")
    return findings


def lint_english(text, whole_file):
    findings = []
    dashes = text.count("—")
    if dashes:
        words = len(LATIN_WORD.findall(whole_file)) or 1
        total = whole_file.count("—")
        if total * 1000 > EM_DASHES_PER_1000_WORDS * words:
            findings.append(f"prose.md em dash budget: the file now has {total} em dashes in {words} words, "
                            f"over {EM_DASHES_PER_1000_WORDS} per 1000; this edit wrote {dashes}.")
    for pattern in NEGATION_CONTRAST:
        for match in pattern.finditer(text):
            findings.append(f"prose.md negation before the claim: {sentence_around(text, match, '.!?')}")
    return findings


def main():
    try:
        call = json.load(sys.stdin)
    except ValueError:
        return
    tool_input = call.get("tool_input") or {}
    path = tool_input.get("file_path") or ""
    root = os.environ.get("CLAUDE_PROJECT_DIR") or call.get("cwd") or ""
    relative = os.path.relpath(path, root) if root and os.path.isabs(path) else path
    # The rule and skill files quote the forms they forbid.
    if relative.startswith((".claude" + os.sep, "target" + os.sep, "..")):
        return
    english, japanese = split_source(relative, written_text(tool_input))
    if not english.strip() and not japanese.strip():
        return
    findings = []
    if japanese.strip():
        findings += lint_japanese(japanese)
    if english.strip():
        try:
            with open(path, encoding="utf-8") as file:
                whole_english, _ = split_source(relative, file.read())
        except OSError:
            whole_english = english
        findings += lint_english(english, whole_english)
    if not findings:
        return
    shown = findings[:MAX_FINDINGS]
    if len(findings) > MAX_FINDINGS:
        shown.append(f"... and {len(findings) - MAX_FINDINGS} more.")
    context = (f"Prose candidates in the text this edit wrote to {relative}. Each is a candidate: "
               "fix it where .claude/rules/prose.md applies, and leave a quotation or a deliberate choice.\n- "
               + "\n- ".join(shown))
    print(json.dumps({"hookSpecificOutput": {"hookEventName": "PostToolUse", "additionalContext": context}},
                     ensure_ascii=False))


if __name__ == "__main__":
    try:
        main()
    except Exception:  # A broken lint must not stop the edit that triggered it.
        pass
    sys.exit(0)
