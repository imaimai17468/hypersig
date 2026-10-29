//! The i18n section: typed translations from `hypersig_i18n`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, title};

mod choosing;
mod pages;

/// The article of `page`.
pub fn content(site: Site, page: I18nPage) -> Part {
    let head = title(Doc::I18n(page).title(site));
    match page {
        // Get started
        I18nPage::Overview => overview(site, head),
        // Texts
        I18nPage::Locale | I18nPage::Texts | I18nPage::Messages | I18nPage::Formats => {
            overview(site, head)
        }
        // Pages in each language
        I18nPage::Pages => pages::routes::page(site, head),
        I18nPage::LanguageSwitcher => pages::language_switcher::page(site, head),
        I18nPage::PageLanguage => pages::page_language::page(site, head),
        I18nPage::Alternates => pages::alternates::page(site, head),
        // Choosing the language
        I18nPage::Negotiation => choosing::negotiation::page(site, head),
        I18nPage::Redirect => choosing::redirect::page(site, head),
    }
}

fn overview(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::I18n(I18nPage::Overview).leads()),
        heading(
            site,
            L {
                en: "Add the crate",
                ja: "クレートを追加する",
            },
        ),
        para(
            site,
            L {
                en: "Add it next to `hypersig`. Its `axum` feature, on by default, adds the redirect from `/` that [Choosing the language](i18n/negotiation/) describes.",
                ja: "`hypersig` と並べて追加します。既定で有効な `axum` の feature は、[言語の選び方](i18n/negotiation/) で説明する `/` からのリダイレクトを追加します。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = "0.14"
hypersig_i18n = "0.14""#,
        ),
        heading(
            site,
            L {
                en: "Declare the languages",
                ja: "言語を宣言する",
            },
        ),
        para(
            site,
            L {
                en: "List the languages in an enum with `#[derive(Segment)]`, and implement [`Locale`](api:hypersig_i18n/trait.Locale.html) for it. `Locale::Text` names a struct with one field per language, and `pick` returns the field of the language. serde serializes each variant to the language's BCP 47 tag, which `tag` returns, so with `#[serde(rename_all = \"lowercase\")]` the tags are `en` and `ja`. The first variant is the default language.",
                ja: "言語を `#[derive(Segment)]` を付けた enum に並べ、その enum に [`Locale`](api:hypersig_i18n/trait.Locale.html) を実装します。`Locale::Text` には言語ごとに 1 つのフィールドを持つ構造体を指定し、`pick` はその言語のフィールドを返します。serde がバリアントをシリアライズした文字列がその言語の BCP 47 のタグになり、`tag` はその文字列を返します。そのため `#[serde(rename_all = \"lowercase\")]` を付けると、タグは `en` と `ja` になります。最初のバリアントが既定の言語です。",
            },
        ),
        show(Snippet::I18nLocale),
        heading(
            site,
            L {
                en: "A missing translation fails to compile",
                ja: "訳が欠けるとコンパイルエラーになる",
            },
        ),
        para(
            site,
            L {
                en: "A text is a struct literal, and Rust requires a literal to give every field, so a text without its Japanese fails to compile. A language added to the enum also fails to compile in two places until it is handled. One is the `match` in `pick`, and the other is the struct, because every text has to give the field added to it. The sample below uses the `Lang` and `ByLang` of [Declare the languages](i18n/overview/#declare-the-languages).",
                ja: "文は構造体のリテラルで、Rust はリテラルにすべてのフィールドを書くよう求めます。そのため、日本語の訳がない文はコンパイルエラーになります。enum に言語を追加したときも、その言語への対応を書くまで 2 か所がコンパイルエラーになります。1 つは `pick` の `match` で、もう 1 つは構造体です。構造体に追加したフィールドは、すべての文で書く必要があります。下のサンプルは [言語を宣言する](i18n/overview/#declare-the-languages) の `Lang` と `ByLang` を使います。",
            },
        ),
        show(Snippet::I18nMissing),
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
                    en: "The compiler checks only that a translation exists. A Japanese field that holds English compiles.",
                    ja: "検査するのは訳があることで、訳が正しいことではありません。日本語のフィールドに英語を書いても、コンパイルは通ります。",
                },
                L {
                    en: "`hypersig_i18n` picks texts and leaves plural forms and the formats of numbers and dates to `hypersig`: `Expr::plural` with `PluralCategory`, `NumberFormat`, and `DateFormat`, which [Expr](reference/expr/#numbers-and-text) lists.",
                    ja: "`hypersig_i18n` が扱うのは文の選択です。複数形と、数値や日付の書式には `hypersig` の `Expr::plural` と `PluralCategory`、`NumberFormat`、`DateFormat` を使います。一覧は [Expr](reference/expr/#numbers-and-text) にあります。",
                },
            ],
        ),
        note(
            site,
            L {
                en: "This site is built the same way. Every page is one Rust function, rendered once in English and once in Japanese, and each of its texts is written as an `L { en, ja }`.",
                ja: "このサイトも同じ方法で作っています。どのページも 1 つの Rust の関数で、英語と日本語で 1 回ずつ描画し、文はどれも `L { en, ja }` で書いています。",
            },
        ),
    ])
}
