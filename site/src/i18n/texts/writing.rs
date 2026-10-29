//! The i18n page on writing a text: a struct literal with a field per
//! language, where texts live, and texts of types other than strings.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Texts).leads()),
        literal(site),
        new_language(site),
        placement(site),
        other_types(site),
        unchecked(site),
        example(site),
    ])
}

fn literal(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A text is a struct literal",
                ja: "文は構造体のリテラル",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "ByLang { en: \"Settings\", ja: \"設定\" }   // ByLang<&str>\nlang.pick(text)                            // &str",
        ),
        para(
            site,
            L {
                en: "A text is a value of the `Text<T>` of [`Locale`](api:hypersig_i18n/trait.Locale.html), here the `ByLang` of [Declare the languages](i18n/overview/#declare-the-languages), and [`Locale::pick`](api:hypersig_i18n/trait.Locale.html#tymethod.pick) returns the translation for one language. Rust requires a struct literal to give every field, so a text without its Japanese fails to compile with E0063:",
                ja: "文は [`Locale`](api:hypersig_i18n/trait.Locale.html) の `Text<T>` の値で、ここでは [言語を宣言する](i18n/overview/#declare-the-languages) の `ByLang` です。[`Locale::pick`](api:hypersig_i18n/trait.Locale.html#tymethod.pick) は 1 つの言語の訳を返します。Rust は構造体のリテラルにすべてのフィールドを書くよう求めるので、日本語のない文は E0063 でコンパイルエラーになります。",
            },
        ),
        show(Snippet::I18nMissing),
        para(
            site,
            L {
                en: "What `pick` returns goes wherever text goes: `El::text`, `Page::new`, `format!`. Text set with `El::text` is escaped like any other, so a `<` in a translation appears on the page as the character `<`.",
                ja: "`pick` が返す値は `El::text`、`Page::new`、`format!` など、文字列を渡せるところならどこにでも渡せます。`El::text` で設定した文はほかの文字列と同じくエスケープされるので、訳の中の `<` はページに文字の `<` として表示されます。",
            },
        ),
    ])
}

fn new_language(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Adding a language",
                ja: "言語を追加する",
            },
        ),
        para(
            site,
            L {
                en: "To add French, add the variant `Fr` to `Lang` and the field `fr` to `ByLang`. Then the `match` in `pick` fails with E0004 until it has the arm for `Fr`, and every text fails with E0063 until it has its `fr` field. `cargo build` lists each text that still needs a translation:",
                ja: "フランス語を追加するには、`Lang` にバリアント `Fr` を、`ByLang` にフィールド `fr` を追加します。すると、`pick` の `match` は `Fr` の分岐を書くまで E0004 になり、どの文も `fr` のフィールドを書くまで E0063 になります。`cargo build` が、まだ訳が必要な文を 1 つずつ挙げます。",
            },
        ),
        show(Snippet::I18nNewLanguage),
        para(
            site,
            L {
                en: "Adding the field to `ByLang` is what makes every text fail. A variant added to `Lang` without the field fails only in `pick`, where there is no field to return.",
                ja: "すべての文をエラーにするのは、`ByLang` へのフィールドの追加です。フィールドを追加せずに `Lang` にバリアントだけを追加すると、返すフィールドがない `pick` だけがエラーになります。",
            },
        ),
    ])
}

fn placement(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Where texts live",
                ja: "文を置く場所",
            },
        ),
        para(
            site,
            L {
                en: "A text used in one place is written where it is used, inside the function that renders it. A text shared by several pages is a `const`, which can sit in a module of its own. `pick` takes the text by value, and each use of a `const` is a new value, so one constant can be picked any number of times. A `static` cannot be passed by value, so texts are `const`s, not `static`s:",
                ja: "1 か所でしか使わない文は、それを描画する関数の中の使う場所に書きます。複数のページで使う文は `const` にし、専用のモジュールにまとめることもできます。`pick` は文を値で受け取り、`const` は使うたびに新しい値になるので、1 つの定数を何度でも `pick` に渡せます。`static` は値として渡せないので、文は `static` ではなく `const` にします。",
            },
        ),
        show(Snippet::I18nTextConsts),
        para(
            site,
            L {
                en: "A text in a `let` binding is moved by the first `pick`, and a second `pick` of it is E0382. Write it as a `const`, or derive `Clone` and `Copy` on `ByLang`.",
                ja: "`let` で束縛した文は最初の `pick` でムーブされるので、もう一度 `pick` に渡すと E0382 になります。`const` にするか、`ByLang` に `Clone` と `Copy` を derive してください。",
            },
        ),
    ])
}

fn other_types(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Texts that are not strings",
                ja: "文字列以外の文",
            },
        ),
        para(
            site,
            L {
                en: "`Text<T>` takes any `T`, so a value that differs per language is written the same way: a [`SafeUrl`](api:hypersig/struct.SafeUrl.html) to the documentation in each language, a `NumberFormat`, or an element. A sentence with a link inside is an element per language, because the link sits at a different place in each:",
                ja: "`Text<T>` はどの `T` でも受け取るので、言語ごとに違う値は同じ方法で書けます。言語ごとのドキュメントへの [`SafeUrl`](api:hypersig/struct.SafeUrl.html)、`NumberFormat`、要素などです。中にリンクを含む文は、言語ごとにリンクの位置が違うので、言語ごとの要素にします。",
            },
        ),
        show(Snippet::I18nTextValues),
        para(
            site,
            L {
                en: "For `Lang::Ja`, the page's body holds:",
                ja: "`Lang::Ja` のとき、ページの body には次の要素が入ります。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<p>まず<a href="https://example.com/ja/guide">ガイド</a>を読んでください。</p>"#,
        ),
        para(
            site,
            L {
                en: "A struct literal evaluates every field, so both elements are built and `pick` drops the English one. That costs little for a short element. Where building a translation is expensive, make each field a function, as [Texts with values](i18n/messages/) does, so only the chosen one runs.",
                ja: "構造体のリテラルはすべてのフィールドを評価するので、2 つの要素がどちらも作られ、`pick` は英語の方を捨てます。短い要素なら負担はほとんどありません。訳を作るのに時間がかかる場合は、[値を含む文](i18n/messages/) のように各フィールドを関数にすれば、選ばれた訳だけが実行されます。",
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
                    en: "The content of a translation. A Japanese field that holds English, an empty string, or a translation that says something else compiles, and the page shows it as written.",
                    ja: "訳の中身。日本語のフィールドに英語や空文字列を書いても、意味の違う訳を書いても、コンパイルは通り、ページには書いたとおりに表示されます。",
                },
                L {
                    en: "Text outside a `ByLang`. A string literal passed straight to `El::text` compiles and is shown in every language.",
                    ja: "`ByLang` の外にある文。文字列リテラルを直接 `El::text` に渡してもコンパイルは通り、どの言語でもその文字列が表示されます。",
                },
                L {
                    en: "The `T` of a text is the same in every language, so each translation has the same type, but not the same length or shape: a German label longer than its English one can overflow a button.",
                    ja: "文の `T` はどの言語でも同じなので、訳の型は同じになりますが、長さや形までは揃いません。英語より長いドイツ語のラベルはボタンからはみ出すことがあります。",
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
                en: "A page whose title and paragraph are texts, from [A page in two languages](i18n/overview/#a-page-in-two-languages):",
                ja: "[2 つの言語のページ](i18n/overview/#a-page-in-two-languages) の、タイトルと段落を文で書いたページです。",
            },
        ),
        show(Snippet::I18nHome),
        para(
            site,
            L {
                en: "Next, [Texts with values](i18n/messages/) covers a text that puts a number or a name inside it.",
                ja: "次の [値を含む文](i18n/messages/) で数値や名前を中に入れる文を説明します。",
            },
        ),
    ])
}
