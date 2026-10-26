---
description: "How a sentence is built and what a document carries, in comments, instruction documents, commit messages, PR bodies, and replies, in English and Japanese: which tool checks which pattern, plain words, what a sentence is allowed to be about, precision, where the decision sits, and format"
alwaysApply: true
---

# Prose

This governs every text a person reads: replies to the user, plans, reports, commit messages, PR descriptions, review comments, code comments, and instruction documents. Check a reply against these rules before sending it, including where the sentence names a category in one word or hands a decision back. Code Practices settles what such prose may take as its subject, and Knowledge Currency whether its claims are verified. This file settles how the sentence is built and what the document carries.

Each rule names a kind of construction and the test that finds its members in either language. An example shows the kind, and a phrase that passes the test falls under the rule whether or not it resembles the example.

## Tools

**Two tools find the fixed forms.** A rule below that a tool checks names the tool and keeps only what the tool leaves.

- `yomiyasu` checks Japanese. `.claude/hooks/lint-prose.py` runs its `yomiyasu_lint.py` after each edit on the Japanese the edit wrote: Markdown that is mostly Japanese, and the `ja: "..."` strings of `.rs` files. The lint reports dashes, negated contrasts, fixed openers and closers, metaphorical verbs, stock nouns, literal translations, heading and colon shapes, bold, emoji, and runs of fragments. Its intensifier and nominalization counts measure a change between two versions, so they run only in `scripts/yomiyasu_check.py run --source OLD --final NEW` on a revision.
- The `humanizer` skill covers English. It has no linter, so apply it to English you wrote before committing. The same hook counts the two English rules that can be counted: the em dash budget, and a phrase negated before its claim.
- The hook adds its findings to the context and never blocks the edit. Fix a finding where a rule below applies, and leave it in a quotation or a deliberate choice. It skips `.claude/`, whose files quote the forms they forbid.

**This file wins where a tool disagrees.** Four disagreements are known:

- yomiyasu leaves spelling variants as written, where this repository spells each term the way the rest of the site does.
- yomiyasu's `sentence_end_repetition` flags three endings in a row of `です` or `ます`, which the polite register below produces in most passages. The hook drops those findings.
- humanizer §8 removes every em dash from English, where this file allows a budget.
- humanizer §19 removes bold labels, where a normative document here opens each rule with a bold lead (Format below).

## Plain words

**Use the plain word for what happens, in language the reader already has.** A figurative verb, a shorthand, or a coined term reads as insight and substitutes an impression for the mechanism. Name the condition and the consequence separately, each with its own literal verb: "is not detected", "fails", "is skipped". yomiyasu flags the common Japanese metaphorical verbs and humanizer §3 the English sayings.

**Cut an evaluative or promotional modifier.** The class is the adjectives, adverbs, and verbs that rate a thing rather than describe it: claims of strength, completeness, smoothness, necessity, or centrality, in either language. Test a word by deleting it. When the sentence then claims the same checkable fact, the word was a rating and goes. When the fact changes, the word narrowed the meaning and stays. humanizer §12 and §16 and yomiyasu's `slop_vocabulary` list this year's favorites, and the deletion test decides the rest.

**Prefer the shorter, older word.** A Latinate or Sino-Japanese compound where a native verb says the same (`use` over `utilize`, `する` over `実施する`) adds length and stiffness. So does a multi-word connective that one preposition replaces (`to` over `in order to`).

**Let the verb carry the meaning.** Two constructions hide it. A copula substitute is a verb that stands in for `is` or `has` (humanizer §18). A light-verb construction puts the action in a noun and hands it a verb with no content, such as `make a decision` for `decided`, or `〜を行う` for `する`. Test it by turning the noun back into a verb. When the sentence keeps its meaning, use the verb.

**Report your own action with its object.** A verb of gesture with the thing moved left out (`畳んだ`) tells the reader that something happened and not what. Write which thing you changed and what it now is: which function you deleted, which default went from what to what.

**Name the actor.** An agentless passive and an inanimate subject given an agent's verb (`the check ensures X`, `〜が担保される`) both leave out who or what acts, which is how an unverified claim gets in without anyone owning it. Put the actor in the subject: which function, which gate, which person. Where the actor is the reader, write the imperative. humanizer §11 and three fixed phrases of yomiyasu's `metaphor_verb` catch some of these.

**Cut an intensifying adverb.** The class is the degree adverbs and truth-emphasizers that raise the temperature of a claim without changing it (English `really`, Japanese `非常に`). Test it by deleting the adverb: the claim keeps its content and loses only its loudness. No check finds these in a single text. A hedge that carries real uncertainty is a different class, and Precision below protects it.

**Anchor a term the reader may not hold to one they do, once.** Keep the file name, the type, the command in the report, because a reader given only the outcome cannot check it or carry it into the next conversation, and every later report then restarts from nothing. On first use, tie the term to a word the reader has used themselves, a place on screen, or work they did by hand. Use the name alone after that. Where you cannot tell what the reader holds, ask.

**When handing a decision back, state the goal, where it stands, what blocks it, and how the options differ, in that order.** The blocker is the one thing the reader cannot reconstruct alone, so it must be a fact rather than an impression.

## Say the specific thing

**Apply the portability test.** A sentence that could move unchanged to another repository, another module, or another company is filler. Replace it with a fact, a mechanism, a number, a file path, or a consequence that belongs to this subject alone. `The implications are significant` and `速度が向上する` name nothing.

**Repeat the right word rather than rotating synonyms.** Calling one thing a gate, then a check, then a guard reads as variety and costs the reader the identity of the thing. Pick the term and keep it. Where one English word covers two concepts in a text written in both languages, split it in the English first, because a translation that splits it only in Japanese leaves the English ambiguous and the two languages out of step.

**Use a universal quantifier only for what was checked.** `every` or `すべて` used to mean "many" claim more than was measured. A directive is the exception, because `never hand-edit it` states the rule's force rather than a measurement.

**Write one claim once.** Restating a point in new words makes a passage feel thorough and gives the reader nothing new, and so does a summary placed right after the passage it summarizes. Where a text circles the same claim more than twice, cut the repetitions.

## What the sentence is about

Ask of every sentence whether it updates **the situation** or **the document**.

A sentence that updates the situation carries something new about the thing under discussion: what the code does, what a measurement returned, what someone decided, what is still undecided. Keep it.

A sentence that updates the document is metadiscourse: a frame marker that announces a topic, a preview of what comes next, a summary of what came before, or an evaluation of the text's own claim (how much weight it deserves, whether it surprises). Delete it and read across the gap. Where the logic now jumps, rewrite it as the situation-side fact it was gesturing at. yomiyasu flags the fixed Japanese openers and closers of this kind, and humanizer §2, §4, §24, and §25 the English ones.

Four document-side forms survive, and only at a boundary such as an opening or a close:

- Rejecting a misreading, with the misreading quoted exactly. A bare announcement that a misreading exists, with nothing quoted, does not qualify.
- Setting a question that a later passage answers.
- A request to the reader, such as a scope caveat.
- Opening and closing the frame of an example.

Cutting a document-side sentence down to a crisp declarative does not save it, because it then reads like a considered remark, and that cut is the most common way this rule gets evaded. Settle what the sentence is about before judging how it sounds.

## Sentence shape

Each shape below is read as machine-written, and each also costs the reader something specific. Density is what gets noticed, so budget them per file rather than per sentence, and measure before calling a file clean.

**Em dash: 5 per 1000 words of English.** The hook counts it per file after each edit that writes one. Past the budget the dash carries work that punctuation should refuse, giving a subordinate clause the same weight as the main clause, so the reader cannot tell the instruction from its reason. Where the right side restates the left, delete it. Where it adds a condition, give it its own sentence. Japanese allows none (Japanese below).

**Put the claim first, without a negated alternative before it.** yomiyasu's `negative_parallelism`, the hook's English pattern, and humanizer §1 and §5 find the forms. Keep the negated half only where it is a misreading the reader would actually reach: quote it, and add the ground for rejecting it, which a counterfactual often supplies (`もしAなら〜だったはずだ`).

**Do not balance a pair of clauses around a semicolon, and do not close on an aphorism.** The symmetry reads as insight and resists being checked, and a final polished line turns a finished argument back into a slogan (humanizer §2 and §3). End on the clearest concrete sentence the text already contains. Where a balanced half is worth keeping, keep it only because it changes which way the reader decides a borderline case.

**Write the point where the run-up would go.** A colon followed by a dramatic completion, a question you answer yourself, and a phrase that announces a point is coming all delay a sentence you could simply write (humanizer §4, yomiyasu `trailing_colon` and `meta_filler`). Use the colon for a list, a label, or a quotation.

**Put the word that answers the previous sentence near the front of the next one.** Where it arrives at the end, the reader holds the whole sentence unplaced until it lands, and a run of them reads as a list of facts with the connections left out. The same gap opens when the subject changes between adjacent sentences with nothing announcing the change.

**Prefer two parallel items to three.** humanizer §6 covers the forced triad and yomiyasu's `count_declaration` the announced count. An enumeration of things that genuinely number three is exempt.

**State an effect rather than its importance.** The class is the predicates that grade significance (an adjective of importance, a cleft that opens with one, a phrase calling a fact proof of something larger) and the trailing participle or `〜を示している` that attaches meaning to a fact without stating it (humanizer §13 and §15, yomiyasu `meta_filler`). Where the effect is worth naming, name it. Otherwise the phrase goes.

**Name the source or drop the sentence.** An appeal to an unnamed authority or consensus implies backing that nothing gives (humanizer §17).

**Vary the run.** Three or more long assertions in a row, or a run of short flat declaratives, both read as generated. Break the run with a sentence of the other length. humanizer §7 covers repeated openings, and yomiyasu's `fragment_run` the run of short evaluations.

## Precision

These decide whether the sentence says only as much as was checked, where Knowledge Currency decides whether it was.

**Keep a hedge that carries real uncertainty.** A modal or evidential (`かもしれない`, `だろう`, "appears to") is removable only where it weakens something the text has already established. Where it marks an unverified possibility, an inference from a log, or a doubt the reader would raise, flattening it into an assertion makes the text wrong. humanizer §9 cuts stacked qualifiers, which is the same test applied to a pile of them.

**Do not collapse distinct things into one word.** Separate decisions, separate causes, and separate kinds of failure stay separate. Where several of them do reduce to one thing, say so in a sentence before naming it.

**State the mechanism when claiming a cause.** `AだとBになる` with the reason omitted is an assertion the reader cannot check.

**Give the condition of a detection, a prevention, or a guarantee.** Written unconditionally it claims more than any check delivers: `〜が成り立つときに限り`, `〜しやすい`.

**Narrow the claim to what the example supports.** Where the example carries only part of it, the claim moves rather than the example.

## What the document includes

These decide what gets written and where it sits, where the sections above decide how each sentence is built.

**Open with what was decided.** A plan, a report, a PR body, or a review carries the decision and the reason it beat the alternative in its first lines, and the reasoning that produced it follows (humanizer §26 for a reply). Announcing the conclusion in place of stating it is the run-up Sentence shape refuses.

**Give a point the room its consequence earns.** A judgment that could have gone the other way takes the space it needs, and a premise no reader would dispute takes one line or none. Where every point runs to the same length, the reader is left to work out which one decided the design.

**An option ruled out by a stated requirement gets that requirement and nothing else.** Writing its advantages, or balancing them against a drawback, gives a case nobody could adopt the same length as the design you adopted. Where several options were weighed, the ones you dropped go in a table of the option and the reason it lost, and the prose covers the option you took.

**Cut a section the decision survives without.** Ask it of each section once the document is written. A background, a glossary, an audience note, or a list of future extensions is written where the reader cannot follow the decision without it, and dropped where a template was the only thing asking for it.

## Format

**Format follows the content.** A bullet list is for items that are genuinely parallel, and an argument that moves from one step to the next belongs in prose, so numbering paragraphs (`The first wall is`, `The second wall is`) to disguise a list as prose fails both ways. A heading needs more than two sentences under it. humanizer §19 and §20 and yomiyasu's bold, list, heading, and emoji rules cover decoration.

**A label names an action, or points at a literal.** A heading, a table column, or a bullet lead holds no room for a mechanism, so a slot that asks for one noun takes the nearest image instead. `溶かした先` names nothing that happened where `入れた節` names the action, and a file path or a section title cannot become an image at all.

**A bold lead names a rule.** In the normative documents here it opens the paragraph and carries the rule's name, which is why these paragraphs have one. Bold scattered mid-sentence marks nothing, because emphasis everywhere is emphasis nowhere.

## Japanese

**Write no dash in running text or in a heading.** yomiyasu's `dash_*` rules flag the em dash, the horizontal bar, and the box-drawing rule. Write a parenthetical with `（）`, and split a restatement into two sentences or join it with a comma. The en dash in a range or in a compound name such as `Curry–Howard` is exempt, as is anything inside a code block.

**Write a heading as one noun phrase.** Two elements joined by a colon or a rule read as a label and its gloss (yomiyasu `heading_colon` finds the colon form).

**End a polite clause on a verb, a noun with `です`, or a na-adjective with `です`.** An i-adjective followed directly by `です` reads as spoken, and the hook flags it, together with `ないです` and `たいです`. Read it as a symptom that the sentence has come loose from the ones around it, and rewrite the passage rather than the ending alone.

**Do not open two sentences in a row with an adversative or concessive connective.** The class is the connectives that turn against the sentence before (逆接) or grant a point before qualifying it (譲歩), and the hook flags two in a row. Back to back they balance the text without moving it.

## Calibration

These rules cut what repeats and what overclaims. They do not license flattening a text toward a neutral middle, and applying them to someone else's writing means the minimum effective edit rather than a rewrite.

**Before cutting a flagged phrase, check whether cutting it loses meaning.** Where it does, it is content, and it stays or gets reworded. Where the author would defend it, it is a choice rather than a formula, and it stays.

**Match the register you are writing in.** A reply to the user, a commit message, and a rule are held to the same tests and read nothing alike. Bluntness, humor, and a first-person admission survive every rule here when they are the writer's own.
