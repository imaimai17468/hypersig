//! The i18n page on texts that take values: a function per language, with
//! the same argument types in every translation.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Messages).leads()),
        functions(site),
        argument_types(site),
        closures(site),
        unchecked(site),
        example(site),
    ])
}

fn functions(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A function per language",
                ja: "言語ごとの関数",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "const PAGE_OF: ByLang<fn(u32, u32) -> String> = ByLang { en: |page, total| ..., ja: |page, total| ... };\nlang.pick(PAGE_OF)(2, 5)   // String",
        ),
        para(
            site,
            L {
                en: "A text that puts values inside it is a `ByLang` of `fn` pointers, one function per language. `pick` returns the function of the language, and calling it formats the text. Each translation places the values where its language puts them, so `Page 2 of 5` and `5 ページ中 2 ページ目` take the same arguments in a different order:",
                ja: "値を中に入れる文は `fn` ポインタの `ByLang` にし、言語ごとに 1 つの関数を書きます。`pick` はその言語の関数を返し、それを呼ぶと文ができます。訳ごとに、その言語での位置に値を置くので、`Page 2 of 5` と `5 ページ中 2 ページ目` は同じ引数を違う順に使います。",
            },
        ),
        show(Snippet::I18nMessage),
        para(
            site,
            L {
                en: "Only the chosen function runs, so the other translations are never formatted. A text written as `ByLang { en: format!(..), ja: format!(..) }` formats every translation before `pick` drops all but one.",
                ja: "実行されるのは選ばれた関数だけなので、ほかの訳は組み立てられません。`ByLang { en: format!(..), ja: format!(..) }` と書いた文は、`pick` が 1 つを残して捨てる前に、すべての訳を組み立てます。",
            },
        ),
    ])
}

fn argument_types(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Arguments typed alike in every translation",
                ja: "どの訳でも同じ引数の型",
            },
        ),
        para(
            site,
            L {
                en: "The type `fn(u32, u32) -> String` belongs to the whole text, so every translation takes the same arguments of the same types, and every caller passes them. A translation that takes an `i32` where the text says `u32` is E0308:",
                ja: "型 `fn(u32, u32) -> String` は文全体に属するので、どの訳も同じ型の同じ引数を受け取り、呼び出す側はどれにもそれを渡します。文が `u32` と決めているのに `i32` を受け取る訳は E0308 になります。",
            },
        ),
        show(Snippet::I18nMessageArg),
        para(
            site,
            L {
                en: "The call is checked once for every language: `lang.pick(PAGE_OF)(\"2\", 5)` is E0308 whichever language `lang` holds.",
                ja: "呼び出しはすべての言語に対して一度に検査されます。`lang.pick(PAGE_OF)(\"2\", 5)` は、`lang` がどの言語でも E0308 になります。",
            },
        ),
    ])
}

fn closures(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Why the type is written",
                ja: "型を書く理由",
            },
        ),
        para(
            site,
            L {
                en: "Each closure has a type of its own, so two closures cannot be the one `T` of a `ByLang` without the type written out. Left to inference, the second closure is E0308:",
                ja: "クロージャはそれぞれ固有の型を持つので、型を書かなければ 2 つのクロージャを `ByLang` の 1 つの `T` にはできません。型を推論に任せると、2 つ目のクロージャが E0308 になります。",
            },
        ),
        show(Snippet::I18nTwoClosures),
        para(
            site,
            L {
                en: "With `ByLang<fn(u32) -> String>` written, both closures coerce to the same `fn` pointer. Only a closure that captures nothing coerces, so a translation that reads a local variable, such as `|n| format!(\"{n} in {city}\")`, is E0308 too. Pass the value as another argument instead.",
                ja: "`ByLang<fn(u32) -> String>` と書くと、どちらのクロージャも同じ `fn` ポインタに変換されます。変換できるのは何もキャプチャしないクロージャだけなので、`|n| format!(\"{n} in {city}\")` のようにローカル変数を読む訳も E0308 になります。その値は引数として渡してください。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What stays unchecked",
                ja: "検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Whether a translation uses each argument. A translation that leaves out `total` compiles and shows a sentence without it.",
                    ja: "訳が各引数を使っているか。`total` を使わない訳もコンパイルは通り、それを含まない文が表示されます。",
                },
                L {
                    en: "Two arguments of the same type in the wrong order. `|page, total|` written as `|total, page|` in one translation compiles and swaps the numbers in that language.",
                    ja: "同じ型の 2 つの引数の順序。ある訳で `|page, total|` を `|total, page|` と書いてもコンパイルは通り、その言語でだけ数が入れ替わります。",
                },
                L {
                    en: "Plural forms. `{n} items` reads `1 items` for 1. [Plurals, numbers, and dates](i18n/formats/) picks the form by the number.",
                    ja: "複数形。`{n} items` は 1 のとき `1 items` になります。数によって形を選ぶ方法は [複数形、数値、日付](i18n/formats/) で説明します。",
                },
            ],
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example",
                ja: "例",
            },
        ),
        para(
            site,
            L {
                en: "A page of an archive that shows its position, with a 404 for a page past the last:",
                ja: "アーカイブのページで現在の位置を示し、最後より後ろのページには 404 を返す例です。",
            },
        ),
        show(Snippet::I18nMessagePage),
        para(
            site,
            L {
                en: "`GET /ja/archive/3` writes `<p>12 ページ中 3 ページ目</p>`. The text is written once by the server; to show a number the browser changes, use the expressions of [Plurals, numbers, and dates](i18n/formats/). Next, that page covers plural forms and the formats of numbers and dates.",
                ja: "`GET /ja/archive/3` は `<p>12 ページ中 3 ページ目</p>` を書きます。この文はサーバーが 1 回書くものです。ブラウザで変わる数を表示するには、[複数形、数値、日付](i18n/formats/) の式を使います。次はそのページで、複数形と、数値や日付の書式を説明します。",
            },
        ),
    ])
}
