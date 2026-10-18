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
                en: "`hypersig_i18n` keeps a text and its translations together in one value, so a text missing a translation into any language fails to compile.",
                ja: "`hypersig_i18n` は 1 つの文とその訳を 1 つの値にまとめて持ちます。そのため、どれかの言語の訳が欠けた文はコンパイルエラーになります。",
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
                en: "The crate is in the hypersig repository and is not published on crates.io yet. Add it next to `hypersig` from Git. Its `axum` feature, on by default, adds the redirect from `/` that [Choosing the language](i18n/negotiation/) describes.",
                ja: "このクレートは hypersig のリポジトリにあり、まだ crates.io には公開していません。`hypersig` と並べて Git から追加します。既定で有効な `axum` の feature は、[言語の選び方](i18n/negotiation/) で説明する `/` からのリダイレクトを追加します。",
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
                en: "List the languages in an enum with `#[derive(Segment)]`, and implement [`Locale`](api:hypersig_i18n/trait.Locale.html) for it. `Locale::Text` names a struct with one field per language, and `pick` returns the field of the language. serde serializes each variant to the language's BCP 47 tag, which `tag` returns, so with `#[serde(rename_all = \"lowercase\")]` the tags are `en` and `ja`.",
                ja: "言語を `#[derive(Segment)]` を付けた enum に並べ、その enum に [`Locale`](api:hypersig_i18n/trait.Locale.html) を実装します。`Locale::Text` には言語ごとに 1 つのフィールドを持つ構造体を指定し、`pick` はその言語のフィールドを返します。serde がバリアントをシリアライズした文字列がその言語の BCP 47 のタグになり、`tag` はその文字列を返します。そのため `#[serde(rename_all = \"lowercase\")]` を付けると、タグは `en` と `ja` になります。",
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
                en: "A text is a struct literal, and Rust requires a literal to give every field, so a text without its Japanese fails to compile. A language added to the enum also causes compile errors in two places until it is handled. One is the `match` in `pick`, and the other is the struct. Every text has to give the field added to the struct.",
                ja: "文は構造体のリテラルで、Rust はリテラルにすべてのフィールドを書くよう求めます。そのため、日本語の訳がない文はコンパイルエラーになります。enum に言語を足したときも、その言語への対応を書くまで 2 か所がコンパイルエラーになります。1 つは `pick` の `match` で、もう 1 つは構造体です。構造体に足したフィールドは、すべての文で書く必要があります。",
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
                    en: "`Locale::ALL` lists the languages for `negotiate`, and the compiler does not check that it lists every variant. A variant missing from it is never chosen.",
                    ja: "`Locale::ALL` は `negotiate` が選ぶ言語の一覧で、すべてのバリアントが並んでいるかはコンパイラが検査しません。一覧にないバリアントが選ばれることはありません。",
                },
                L {
                    en: "The check is that a translation exists, not that it is correct. A Japanese field that holds English compiles.",
                    ja: "検査するのは訳があることで、訳が正しいことではありません。日本語のフィールドに英語を書いても、コンパイルは通ります。",
                },
                L {
                    en: "Plural rules and the formats of numbers and dates are not handled. Write them in the application, or use a library such as ICU4X.",
                    ja: "複数形の規則や、数値と日付の書式は扱いません。アプリで書くか、ICU4X のようなライブラリを使ってください。",
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

fn pages(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "The language is the first segment of each page's route, so the page in another language is the same route with only the language changed.",
                ja: "言語は各ページのルートの最初のセグメントに置きます。そのため、別の言語のページは言語だけを変えた同じルートになります。",
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
                en: "A link to the same page in another language is made by passing `link_with` the other language with the same remaining parameters, so the route checks it as it checks other links. A language switcher lists `Locale::ALL` and links, in each language, to the page the reader is on.",
                ja: "同じページの別の言語へのリンクは、`link_with` に別の言語とそのほかの同じパラメータを渡して作ります。そのため、ほかのリンクと同じくルートがこのリンクを検査します。言語を切り替える UI は `Locale::ALL` を並べ、言語ごとに読者が今いるページへリンクします。",
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
                en: "[`Page::lang`](api:hypersig/struct.Page.html#method.lang) writes the tag into `<html lang>`. The browser uses it to decide hyphenation, the voice a screen reader uses, and the fonts, so Japanese text set with `lang=\"ja\"` gets Japanese glyphs.",
                ja: "[`Page::lang`](api:hypersig/struct.Page.html#method.lang) はタグを `<html lang>` に書きます。ブラウザはこの属性を見て、ハイフネーション、スクリーンリーダーの読み上げの声、使うフォントを決めます。そのため、`lang=\"ja\"` の付いた日本語の文は日本語の字形で表示されます。",
            },
        ),
        para(
            site,
            L {
                en: "[`LocalePage::alternates`](api:hypersig_i18n/trait.LocalePage.html#tymethod.alternates) takes an `Origin` and a function from a language to the page's `Url`, as in `|lang| POST.url((lang, id))`. It writes one `<link rel=\"alternate\" hreflang>` per language of `Locale::ALL`, and an `x-default` at the first language, so a search engine shows each reader the page in their language.",
                ja: "[`LocalePage::alternates`](api:hypersig_i18n/trait.LocalePage.html#tymethod.alternates) は `Origin` と、`|lang| POST.url((lang, id))` のように言語からページの `Url` を返す関数を受け取ります。`Locale::ALL` の言語ごとに `<link rel=\"alternate\" hreflang>` を 1 つ書き、最初の言語には `x-default` も書きます。検索エンジンはこれを見て、読者の言語のページを結果に出します。",
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
                en: "A visitor who opens `/` is redirected to the page in the language their browser asks for.",
                ja: "`/` を開いた訪問者を、ブラウザが求める言語のページへリダイレクトします。",
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
                en: "[`negotiate`](api:hypersig_i18n/fn.negotiate.html) reads an `Accept-Language` value and returns the best language of `Locale::ALL`. It tries the ranges from the highest `q` to the lowest, and in the order written where two have the same `q`. A range first matches a tag equal to it, ignoring case. Next it matches a tag equal to the range with its last subtags dropped, so `ja-JP` matches `ja`. Last it matches a tag that starts with the range, so `en` matches `en-US`. `*`, an empty header, and a header with no match give the first language of `Locale::ALL`.",
                ja: "[`negotiate`](api:hypersig_i18n/fn.negotiate.html) は `Accept-Language` の値を読み、`Locale::ALL` の中から最も合う言語を返します。範囲は `q` の大きい順に試し、`q` が同じものは書かれた順に試します。範囲はまず、大文字と小文字を区別せずに範囲と等しいタグに一致します。次に、範囲の後ろのサブタグを削った形と等しいタグに一致するので、`ja-JP` は `ja` に一致します。最後に、範囲で始まるタグに一致するので、`en` は `en-US` に一致します。`*`、空のヘッダー、一致する言語のないヘッダーには、`Locale::ALL` の最初の言語を返します。",
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
                en: "With the `axum` feature, [`redirect_to_locale`](api:hypersig_i18n/trait.LocaleRouterExt.html#tymethod.redirect_to_locale) responds to `GET /` with a `307 Temporary Redirect` to a route whose only parameter is the language. The response includes `Vary: Accept-Language`, because the language redirected to can differ between browsers.",
                ja: "`axum` の feature を有効にすると、[`redirect_to_locale`](api:hypersig_i18n/trait.LocaleRouterExt.html#tymethod.redirect_to_locale) は `GET /` に対して、言語だけをパラメータに持つルートへの `307 Temporary Redirect` を返します。ブラウザによってリダイレクト先の言語が変わることがあるので、レスポンスには `Vary: Accept-Language` を付けます。",
            },
        ),
        show(Snippet::I18nNegotiate),
        note(
            site,
            L {
                en: "A site served as static files, such as this one, has no server to read `Accept-Language`. Such a site puts its default language at the root and links to the others from the header.",
                ja: "このサイトのように静的ファイルとして配信するサイトには、`Accept-Language` を読むサーバーがありません。こうしたサイトでは既定の言語をルートに置き、ほかの言語へはヘッダーからリンクします。",
            },
        ),
    ])
}
