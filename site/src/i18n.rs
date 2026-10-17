//! The i18n section: typed translations from `hypersig_i18n`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, title};

/// The article of `page`.
pub fn content(site: Site, page: I18nPage) -> Part {
    let head = title(Doc::I18n(page).title(site));
    match page {
        I18nPage::Overview => overview(site, head),
        I18nPage::Pages => pages(site, head),
        I18nPage::Negotiation => negotiation(site, head),
    }
}

fn overview(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "`hypersig_i18n` keeps each text with its translations in one value, so a text without one of its translations fails to compile.",
                ja: "`hypersig_i18n` は、1 つの文とその訳を 1 つの値にまとめて持ちます。どれかの言語の訳が欠けた文は、コンパイルエラーになります。",
            },
        ),
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
                en: "The crate lives in the hypersig repository and is not on crates.io yet. Add it next to `hypersig` from Git. Its `axum` feature, on by default, adds the redirect from `/` that [Choosing the language](i18n/negotiation/) describes.",
                ja: "このクレートは hypersig のリポジトリにあり、まだ crates.io には公開していません。`hypersig` と並べて Git から追加します。既定で有効な `axum` の feature は、[言語の選び方](i18n/negotiation/) で説明する `/` からのリダイレクトを足します。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = { git = "https://github.com/imaimai17468/hypersig" }
hypersig_i18n = { git = "https://github.com/imaimai17468/hypersig" }"#,
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
                en: "List the languages in an enum with `#[derive(Segment)]`, and implement [`Locale`](api:hypersig_i18n/trait.Locale.html) for it. `Locale::Text` names a struct with one field per language, and `pick` returns the field of the language. The string serde writes for a variant is its BCP 47 tag, which `tag` returns, so `#[serde(rename_all = \"lowercase\")]` gives `en` and `ja`.",
                ja: "言語を `#[derive(Segment)]` を付けた enum に並べ、その enum に [`Locale`](api:hypersig_i18n/trait.Locale.html) を実装します。`Locale::Text` には言語ごとに 1 つのフィールドを持つ構造体を指定し、`pick` はその言語のフィールドを返します。serde がバリアントに書く文字列がその言語の BCP 47 のタグで、`tag` はその文字列を返します。そのため `#[serde(rename_all = \"lowercase\")]` を付けると、タグは `en` と `ja` になります。",
            },
        ),
        show(Snippet::I18nLocale),
        heading(
            site,
            L {
                en: "A missing translation is a compile error",
                ja: "訳の欠けはコンパイルエラーになる",
            },
        ),
        para(
            site,
            L {
                en: "A text is a struct literal, and Rust requires every field of one, so a text without its Japanese fails to compile. A language added to the enum fails to compile in two places until it is handled: the `match` in `pick`, and the struct, whose new field every text then has to give.",
                ja: "文は構造体のリテラルで、Rust はリテラルにすべてのフィールドを求めます。そのため、日本語の訳がない文はコンパイルエラーになります。enum に言語を足したときも、2 か所がコンパイルエラーになります。1 つは `pick` の `match` で、もう 1 つは構造体です。構造体にフィールドを足すと、すべての文がそのフィールドを書く必要があります。",
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
                    en: "`Locale::ALL` lists the languages for `negotiate`, and the compiler does not check that it lists every variant. A variant left out of it is never chosen.",
                    ja: "`Locale::ALL` は `negotiate` が選ぶ言語の一覧で、すべてのバリアントが並んでいるかはコンパイラが検査しません。一覧から漏れたバリアントは、選ばれることがありません。",
                },
                L {
                    en: "A translation is checked to exist, and not to be correct. A Japanese field that holds English compiles.",
                    ja: "検査するのは訳があることで、訳が正しいことではありません。日本語のフィールドに英語を書いても、コンパイルは通ります。",
                },
                L {
                    en: "Plural rules and the formats of numbers and dates are left to the application, or to a library such as ICU4X.",
                    ja: "複数形の規則や、数値と日付の書式は扱いません。アプリで書くか、ICU4X のようなライブラリを使ってください。",
                },
            ],
        ),
        note(
            site,
            L {
                en: "This site is built the same way: every page is one Rust function, rendered once in English and once in Japanese, and each of its texts is an `L { en, ja }`.",
                ja: "このサイトも同じ作りです。どのページも 1 つの Rust の関数で、英語と日本語で 1 回ずつ描き、文はどれも `L { en, ja }` で書いています。",
            },
        ),
    ])
}

fn pages(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "The language is the first segment of each page's route, so a page in another language is the same route with another locale.",
                ja: "言語は各ページのルートの最初のセグメントに置きます。そのため、別の言語のページは、言語だけを変えた同じルートです。",
            },
        ),
        heading(
            site,
            L {
                en: "The language in the route",
                ja: "ルートに言語を入れる",
            },
        ),
        para(
            site,
            L {
                en: "A `Locale` is a [`Segment`](api:hypersig/trait.Segment.html), so it can be the first parameter of a route, as in `Route<(Lang, i32)>` for `\"/{lang}/members/{id}\"`. The handler receives the language with the other parameters, and a request for a language the enum does not have gets a 404 before the handler runs.",
                ja: "`Locale` は [`Segment`](api:hypersig/trait.Segment.html) なので、`\"/{lang}/members/{id}\"` に対する `Route<(Lang, i32)>` のように、ルートの最初のパラメータにできます。ハンドラはほかのパラメータと一緒に言語を受け取ります。enum にない言語へのリクエストには、ハンドラを呼ぶ前に 404 を返します。",
            },
        ),
        show(Snippet::I18nPages),
        heading(
            site,
            L {
                en: "Links to the other languages",
                ja: "ほかの言語へのリンク",
            },
        ),
        para(
            site,
            L {
                en: "A link to the same page in another language is `link_with` with the other locale and the same parameters, so the route checks it as it checks every link. A switcher lists `Locale::ALL` and links each language to the page the reader is on.",
                ja: "同じページの別の言語へのリンクは、言語だけを変え、ほかのパラメータはそのまま渡した `link_with` です。そのため、ほかのリンクと同じくルートがそのリンクを検査します。言語の切り替えは `Locale::ALL` を並べ、それぞれの言語で今のページへリンクします。",
            },
        ),
        heading(
            site,
            L {
                en: "The language of the page",
                ja: "ページの言語",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::lang`](api:hypersig/struct.Page.html#method.lang) writes the tag into `<html lang>`. The browser reads it for hyphenation, for the voice a screen reader uses, and for the fonts it picks, so Japanese text set with `lang=\"ja\"` gets Japanese glyphs.",
                ja: "[`Page::lang`](api:hypersig/struct.Page.html#method.lang) はタグを `<html lang>` に書きます。ブラウザはこの属性を見て、ハイフネーション、スクリーンリーダーの読み上げの声、使うフォントを決めます。そのため、`lang=\"ja\"` の付いた日本語の文は日本語の字形で表示されます。",
            },
        ),
    ])
}

fn negotiation(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "A visitor who opens `/` is sent to the page in the language their browser asks for.",
                ja: "`/` を開いた訪問者を、ブラウザが求める言語のページへ送ります。",
            },
        ),
        heading(
            site,
            L {
                en: "negotiate",
                ja: "negotiate",
            },
        ),
        para(
            site,
            L {
                en: "[`negotiate`](api:hypersig_i18n/fn.negotiate.html) reads an `Accept-Language` value and returns the best language of `Locale::ALL`. It tries the ranges from the highest `q` to the lowest, and in the order written where two weigh the same. A range matches a tag that equals it, ignoring case, then a tag that equals it once its last subtags are dropped, so `ja-JP` matches `ja`, and then a tag that starts with it, so `en` matches `en-US`. `*`, an empty header, and a header with no match give the first language of `Locale::ALL`.",
                ja: "[`negotiate`](api:hypersig_i18n/fn.negotiate.html) は `Accept-Language` の値を読み、`Locale::ALL` の中から最も合う言語を返します。範囲は `q` の大きい順に試し、同じ重みのものは書かれた順に試します。範囲は、まず大文字と小文字を区別せずに同じタグと一致し、次に後ろのサブタグを削った形で一致します。そのため `ja-JP` は `ja` と一致します。最後に、その範囲で始まるタグと一致するので、`en` は `en-US` と一致します。`*`、空のヘッダー、一致する言語のないヘッダーには、`Locale::ALL` の最初の言語を返します。",
            },
        ),
        heading(
            site,
            L {
                en: "The redirect from /",
                ja: "/ からのリダイレクト",
            },
        ),
        para(
            site,
            L {
                en: "With the `axum` feature, [`redirect_to_locale`](api:hypersig_i18n/trait.LocaleRouterExt.html#tymethod.redirect_to_locale) answers `GET /` with a `307 Temporary Redirect` to a route whose only parameter is the language. The response carries `Vary: Accept-Language`, because another browser may be sent to another language.",
                ja: "`axum` の feature を有効にすると、[`redirect_to_locale`](api:hypersig_i18n/trait.LocaleRouterExt.html#tymethod.redirect_to_locale) は `GET /` に、言語だけをパラメータに持つルートへの `307 Temporary Redirect` で答えます。別のブラウザは別の言語へ送られることがあるので、レスポンスには `Vary: Accept-Language` を付けます。",
            },
        ),
        show(Snippet::I18nNegotiate),
        note(
            site,
            L {
                en: "A site served as static files, such as this one, has no server to read `Accept-Language`. It puts its default language at the root and links to the others from the header.",
                ja: "このサイトのように静的ファイルとして配信するサイトには、`Accept-Language` を読むサーバーがありません。既定の言語をルートに置き、ほかの言語へはヘッダーからリンクします。",
            },
        ),
    ])
}
