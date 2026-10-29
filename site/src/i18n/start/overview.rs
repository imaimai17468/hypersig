//! The i18n overview: adding the crate, declaring the languages, the
//! smallest page in two languages, and a map of the section.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Overview).leads()),
        crate_setup(site),
        declare(site),
        first_page(site),
        checked(site),
        map(site),
    ])
}

fn crate_setup(site: Site) -> Part {
    parts([
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
                en: "Add `hypersig_i18n` next to `hypersig`, and `serde` with its `derive` feature for the language enum:",
                ja: "`hypersig` と並べて `hypersig_i18n` を追加します。言語の enum のために、`derive` の feature を有効にした `serde` も追加します。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = "0.14"
hypersig_i18n = "0.14"
serde = { version = "1", features = ["derive"] }"#,
        ),
        para(
            site,
            L {
                en: "The `axum` feature is on by default. It adds [`LocaleRouterExt`](api:hypersig_i18n/trait.LocaleRouterExt.html), whose `redirect_to_locale` answers `GET /` with a redirect to the page in the visitor's language. With `default-features = false`, the crate has no axum dependency and no `LocaleRouterExt`; the rest of the crate stays.",
                ja: "`axum` の feature は既定で有効です。この feature は [`LocaleRouterExt`](api:hypersig_i18n/trait.LocaleRouterExt.html) を追加し、その `redirect_to_locale` は `GET /` に対して訪問者の言語のページへのリダイレクトを返します。`default-features = false` にすると axum への依存と `LocaleRouterExt` がなくなり、それ以外はそのまま使えます。",
            },
        ),
    ])
}

fn declare(site: Site) -> Part {
    parts([
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
                en: "List the languages in an enum with `#[derive(Segment)]`, write a struct with one field per language, and implement [`Locale`](api:hypersig_i18n/trait.Locale.html) for the enum. `pick` returns the field of the language. The other pages of this section use this `Lang` and `ByLang`:",
                ja: "言語を `#[derive(Segment)]` を付けた enum に並べ、言語ごとに 1 つのフィールドを持つ構造体を書き、その enum に [`Locale`](api:hypersig_i18n/trait.Locale.html) を実装します。`pick` はその言語のフィールドを返します。このセクションのほかのページはここで宣言した `Lang` と `ByLang` を使います。",
            },
        ),
        show(Snippet::I18nDeclare),
        para(
            site,
            L {
                en: "With `#[serde(rename_all = \"lowercase\")]`, the variants are `en` and `ja` in a URL and in `<html lang>`, and the first variant, `En`, is the default language. [The Locale trait](i18n/locale/) covers the trait's bounds, the tag, and the default.",
                ja: "`#[serde(rename_all = \"lowercase\")]` によって、バリアントは URL でも `<html lang>` でも `en` と `ja` になり、最初のバリアントである `En` が既定の言語になります。トレイトの境界、タグ、既定の言語は [Locale トレイト](i18n/locale/) で説明します。",
            },
        ),
    ])
}

fn first_page(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A page in two languages",
                ja: "2 つの言語のページ",
            },
        ),
        para(
            site,
            L {
                en: "The language is the first segment of the route, so the handler receives it as a `Lang` and picks each text with it. `lang.tag()` sets `<html lang>`. `redirect_to_locale` sends a visitor who opens `/` to `/en` or `/ja`:",
                ja: "言語はルートの最初のセグメントなので、ハンドラは言語を `Lang` として受け取り、それを使って各文を選びます。`lang.tag()` は `<html lang>` を設定します。`redirect_to_locale` は `/` を開いた訪問者を `/en` か `/ja` へ送ります。",
            },
        ),
        show(Snippet::I18nHome),
        para(
            site,
            L {
                en: "For `GET /ja`, the parts of the document this code writes are:",
                ja: "`GET /ja` に返す文書のうち、このコードが書く部分は次のとおりです。",
            },
        ),
        code::block(Lang::Html, None, HOME_HTML),
        para(
            site,
            L {
                en: "`GET /fr` answers 404 before `home` runs, because `fr` is not a variant of `Lang`. `GET /` answers `307 Temporary Redirect` with `Vary: Accept-Language`, and its `Location` is `/ja` for a browser that sends `Accept-Language: ja-JP,ja;q=0.9` and `/en` for one that sends `fr`.",
                ja: "`GET /fr` には `home` を呼ぶ前に 404 を返します。`fr` は `Lang` のバリアントではないからです。`GET /` には `Vary: Accept-Language` 付きの `307 Temporary Redirect` を返し、その `Location` は `Accept-Language: ja-JP,ja;q=0.9` を送るブラウザには `/ja`、`fr` を送るブラウザには `/en` です。",
            },
        ),
    ])
}

/// The parts of the document `Snippet::I18nHome` renders for `/ja` that
/// the snippet's code writes, each of which the snippet asserts.
const HOME_HTML: &str = r#"<html lang="ja">
<title>ようこそ</title>
<h1>ようこそ</h1>
<p>こんにちは。</p>"#;

fn checked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査すること",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A text without one of its translations fails to compile (E0063), because a struct literal has to give every field.",
                    ja: "訳が 1 つでも欠けた文はコンパイルエラー（E0063）になります。構造体のリテラルにはすべてのフィールドを書く必要があるからです。",
                },
                L {
                    en: "A language added to the enum fails to compile in `pick` (E0004) until its `match` has the new arm, and in every text (E0063) until it has the new field.",
                    ja: "enum に追加した言語は、`pick` の `match` に新しい分岐を書くまで（E0004）、またすべての文に新しいフィールドを書くまで（E0063）コンパイルエラーになります。",
                },
                L {
                    en: "A text that takes values has the same argument types in every language (E0308).",
                    ja: "値を受け取る文の引数の型は、どの言語でも同じです（E0308）。",
                },
                L {
                    en: "A link to a page in a language the enum does not have cannot be written, because the route takes a `Lang`.",
                    ja: "ルートが `Lang` を受け取るので、enum にない言語のページへのリンクは書けません。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The compiler does not read the translations. A Japanese field that holds English compiles, and so does a translation that says something else.",
                ja: "コンパイラは訳の中身を読みません。日本語のフィールドに英語を書いても、意味の違う訳を書いても、コンパイルは通ります。",
            },
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

fn map(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The pages of this section",
                ja: "このセクションのページ",
            },
        ),
        table_of(
            site,
            L {
                en: &["Page", "Covers"],
                ja: &["ページ", "内容"],
            },
            L {
                en: &[
                    &[
                        "[The Locale trait](i18n/locale/)",
                        "The bounds, `pick`, `tag`, `locales`, and the default language",
                    ],
                    &[
                        "[Writing a text](i18n/texts/)",
                        "Texts as struct literals, where they live, and texts of other types",
                    ],
                    &[
                        "[Texts with values](i18n/messages/)",
                        "A function per language, with the same argument types in each",
                    ],
                    &[
                        "[Plurals, numbers, and dates](i18n/formats/)",
                        "`Expr::plural`, `NumberFormat`, and `DateFormat` chosen per language",
                    ],
                    &[
                        "[Pages in each language](i18n/pages/)",
                        "The language as the first segment of each route",
                    ],
                    &[
                        "[Links to other languages](i18n/language-switcher/)",
                        "A link to the same page in another language",
                    ],
                    &[
                        "[The language of the page](i18n/page-language/)",
                        "`Page::lang` and `<html lang>`",
                    ],
                    &[
                        "[Alternate pages](i18n/alternates/)",
                        "`<link rel=\"alternate\" hreflang>` for search engines",
                    ],
                    &[
                        "[Choosing the language](i18n/negotiation/)",
                        "`negotiate` and the `Accept-Language` header",
                    ],
                    &[
                        "[The redirect from /](i18n/redirect/)",
                        "`redirect_to_locale`",
                    ],
                ],
                ja: &[
                    &[
                        "[Locale トレイト](i18n/locale/)",
                        "トレイトの境界、`pick`、`tag`、`locales`、既定の言語",
                    ],
                    &[
                        "[文を書く](i18n/texts/)",
                        "構造体のリテラルとしての文、文を置く場所、文字列以外の型の文",
                    ],
                    &[
                        "[値を含む文](i18n/messages/)",
                        "言語ごとの関数と、どの言語でも同じ引数の型",
                    ],
                    &[
                        "[複数形、数値、日付](i18n/formats/)",
                        "言語ごとに選ぶ `Expr::plural`、`NumberFormat`、`DateFormat`",
                    ],
                    &[
                        "[言語ごとのページ](i18n/pages/)",
                        "各ルートの最初のセグメントとしての言語",
                    ],
                    &[
                        "[ほかの言語へのリンク](i18n/language-switcher/)",
                        "同じページの別の言語へのリンク",
                    ],
                    &[
                        "[ページの言語](i18n/page-language/)",
                        "`Page::lang` と `<html lang>`",
                    ],
                    &[
                        "[別の言語のページを示す](i18n/alternates/)",
                        "検索エンジンに向けた `<link rel=\"alternate\" hreflang>`",
                    ],
                    &[
                        "[言語の選び方](i18n/negotiation/)",
                        "`negotiate` と `Accept-Language` ヘッダー",
                    ],
                    &[
                        "[/ からのリダイレクト](i18n/redirect/)",
                        "`redirect_to_locale`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Next, [The Locale trait](i18n/locale/) covers what the language enum has to implement and what each method returns.",
                ja: "次の [Locale トレイト](i18n/locale/) で、言語の enum が実装するものと、各メソッドが返すものを説明します。",
            },
        ),
    ])
}
