---
name: japanese-prose
description: "Write Japanese a person reads, in natural polite form (です・ます): README.ja.md, replies to the user, release notes. Covers register, sentences that read as Japanese rather than as English carried over, and translating between README.md and README.ja.md. Invoke before writing or revising Japanese prose."
---

# Japanese prose

`.claude/rules/prose.md` settles what a sentence may be about and the shapes that read as generated, in both languages, and its Japanese section covers dashes, `い` adjectives before `です`, and adversatives. This skill adds three things it leaves open: the polite register, sentences that read as Japanese rather than as English carried over, and how the two READMEs stay one document. Where the two disagree, `prose.md` wins.

## Register

**Write in 丁寧語 for a reader you do not know.** End sentences with `です` and `ます`, and keep that ending through the whole document, tables and list items included. A list item that is a noun phrase stays a noun phrase (`シグナル名の綴り違い`), and a list item that is a sentence ends in `ます`.

**Give each verb one layer of politeness.** `できます`, `お使いください`, `ご覧ください` carry the respect the sentence needs. A second layer on top (`させていただきます`, `おっしゃられる`, `〜となっております`) adds length without adding respect, so write the single form.

| Two layers | One layer |
|---|---|
| ご確認いただけますと幸いです | ご確認ください |
| 対応させていただきました | 対応しました |
| こちらが例となっております | 例を示します |

**Use 尊敬語 and 謙譲語 only where a person is the subject.** Most sentences in a README have the library, a type, or a command as the subject, and those take plain 丁寧語: `Page::render は初期値で式を評価します`. Respect forms belong to sentences about the reader (`ブラウザで開いてください`) or about the writer.

**`い` adjectives take a verb or a noun before `です`.** `prose.md` flags `難しいです`, and the polite register meets it most often. Rewrite the sentence around what the reader does or what the thing has: `設定が多いです` becomes `設定項目が 12 個あります`.

## Japanese sentence shape

These describe how a natural Japanese sentence carries the meaning that an English sentence carries with a different shape. Each pair shows one mechanism, and a word from a pair stays where the sentence needs it.

**The subject is something that acts, and the verb is what it does.** English lets a concept be the subject (`This design enables fast updates`). In Japanese, name the thing that runs and its action: `ランタイムは変更されたパスの束縛だけを評価し直すので、更新が速く終わります`.

**State an ability with the potential form.** `書けます`, `動かせます`, `使えます` carry what `〜することができます` carries in half the length.

**Let a particle or a plain verb carry the relation.** `において`, `に関して`, `によって`, `を行う` mirror English prepositions and `perform`. `サーバー側において評価を行います` reads as `サーバーで評価します`.

**Split a long modifier into its own sentence.** An English relative clause becomes a modifier that the reader holds unplaced until the noun arrives. Where the modifier runs past one clause, state it as the sentence before: `ランタイムは束縛ごとに読んだパスを記録します。変更があると、そのパスからたどれる束縛だけを評価し直します。`

**Drop the pronoun the context already supplies.** `それは`, `これにより`, `彼ら` follow English `it` and `this`. A Japanese sentence leaves the subject out when the previous sentence named it.

**Write the claim at the strength it was checked.** `〜と言えるでしょう`, `〜と考えられます` soften a fact into an opinion. A measured number or a tested behavior takes `です` or `ます`, and a real doubt keeps `かもしれません` (as `prose.md` Precision says).

**Keep the technical term the reader will type.** An identifier, a command, or a type name stays in code font exactly as written. A concept that has a settled Japanese word takes it (`型`, `束縛`, `差分`). A concept without one keeps the English word in katakana or as written, anchored once on first use: `シグナル（ページの状態を表す値）`.

**Put a half-width space between full-width and half-width text.** `Rust の型`, `約 5 KB`, `18 件`. A full-width parenthesis (`（）`) and full-width punctuation (`、。`) take no space.

## Two READMEs, one document

`README.md` is English and is the one crates.io shows. `README.ja.md` is Japanese. Each opens with a link to the other.

**Write each version from the meaning of the paragraph.** Read the source paragraph, decide what it tells the reader, then write that in the target language in the shape that language uses. A sentence-by-sentence translation carries the source's word order and subjects across, which is how English-shaped Japanese and Japanese-shaped English both start.

**Keep the facts identical.** Code, identifiers, commands, numbers, units, measurement conditions, and the rows of every table match between the two files. A comment in a shell block is written in the language of its README. `lib.rs` compiles the Rust code block of each README as a doctest, so a code change made to one file only fails there.

**Change both in one commit.** A section added to one README is added to the other in the same commit, so neither falls behind.

## Before finishing

Search the Japanese text for these and reread each hit. A hit marks a sentence to read again, and keeping it is fine where the sentence needs it.

```sh
rg -n 'いです|ことができ|において|に関して|によって|を行[いうえっ]|させていただ|となっており|と言えるでしょう|さまざま|様々|ですね|いかがでし|してみましょう|—|―' README.ja.md
```

Then read the file once from the top for the register: every sentence ends in `です` or `ます`, and no paragraph reads as a translation of an English paragraph.

Run the `yomiyasu` plugin's check on the old and the new text (`run`, then `verify`), as `AGENTS.md` describes. Its lint does not report metaphors, so read the text once more for a thing written as a person, such as a function that 「名指しする」 a handler or an action that 「断る」 a request, and write what happens instead.
