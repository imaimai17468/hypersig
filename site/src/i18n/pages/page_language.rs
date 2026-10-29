//! The i18n page on `Page::lang`: the `LanguageTag` it takes from
//! `Locale::tag`, the `<html lang>` it writes, and the panics around it.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::PageLanguage).leads()),
        lang(site),
        browser(site),
        tags(site),
        locale_tag(site),
        render_check(site),
        next(site),
    ])
}

/// `Page::lang` and the HTML it writes.
fn lang(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Page::lang",
                ja: "Page::lang",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: Signals> Page<S> {
    pub fn lang(self, tag: impl Into<LanguageTag>) -> Self;
}

pub trait Locale {
    fn tag(self) -> LanguageTag;
    // ...
}",
        ),
        para(
            site,
            L {
                en: "[`Page::lang`](api:hypersig/struct.Page.html#method.lang) writes `tag` into the `lang` attribute of `<html>`. It takes a [`LanguageTag`](api:hypersig/struct.LanguageTag.html), or an [`IntlLocale`](api:hypersig/enum.IntlLocale.html), which converts into one. In a page whose route holds the language, the tag comes from [`Locale::tag`](api:hypersig_i18n/trait.Locale.html#method.tag), which returns the string serde writes for the variant, the same text as the route's first segment:",
                ja: "[`Page::lang`](api:hypersig/struct.Page.html#method.lang) は `tag` を `<html>` の `lang` 属性に書きます。受け取るのは [`LanguageTag`](api:hypersig/struct.LanguageTag.html) か、それに変換できる [`IntlLocale`](api:hypersig/enum.IntlLocale.html) です。ルートに言語を持つページでは、タグを [`Locale::tag`](api:hypersig_i18n/trait.Locale.html#method.tag) から取ります。`tag` は serde がバリアントに対して書く文字列、つまりルートの最初のセグメントと同じ文字列を返します。",
            },
        ),
        show(Snippet::I18nLang),
        para(
            site,
            L {
                en: "The first of these renders:",
                ja: "最初のページは次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<!doctype html><html lang="ja"><head>…</head><body>…</body></html>"#,
        ),
        para(
            site,
            L {
                en: "A page without `lang` has no `lang` attribute, and the browser guesses the language from the text.",
                ja: "`lang` を指定しないページには `lang` 属性がなく、ブラウザは文から言語を推測します。",
            },
        ),
    ])
}

/// What the browser does with `lang`.
fn browser(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the browser does with it",
                ja: "ブラウザがこの属性で決めること",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A screen reader picks the voice and the pronunciation rules of the language.",
                    ja: "スクリーンリーダーはその言語の声と発音の規則を選びます。",
                },
                L {
                    en: "The browser picks fonts and glyphs for the language. Kanji shared by Japanese and Chinese take their Japanese shapes under `lang=\"ja\"`.",
                    ja: "ブラウザはその言語に合うフォントと字形を選びます。日本語と中国語で共通の漢字は、`lang=\"ja\"` の下では日本語の字形になります。",
                },
                L {
                    en: "`hyphens: auto` hyphenates words by the language's rules, and does nothing on a page without `lang`.",
                    ja: "`hyphens: auto` はその言語の規則で単語にハイフンを入れます。`lang` のないページでは何もしません。",
                },
            ],
        ),
    ])
}

/// What `LanguageTag` checks, and its compile error.
fn tags(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What a LanguageTag checks",
                ja: "LanguageTag が検査すること",
            },
        ),
        para(
            site,
            L {
                en: "A `LanguageTag` has the shape of a BCP 47 tag: subtags of one to eight ASCII letters and digits, separated by `-`, the first of two, three, or five to eight letters. `ja`, `en-US`, and `pt-BR` pass. `en_US`, which the browser does not read as a language, does not. `Page::lang` takes no `&str`, so a tag written in the program goes through `LanguageTag::new`, whose panic in a `const` is E0080:",
                ja: "`LanguageTag` は BCP 47 のタグの形をしています。ASCII の英字と数字 1〜8 文字のサブタグを `-` でつなぎ、最初のサブタグは 2、3、5〜8 文字の英字です。`ja`、`en-US`、`pt-BR` は通り、ブラウザが言語として読まない `en_US` は通りません。`Page::lang` は `&str` を受け取らないので、プログラムに書いたタグは `LanguageTag::new` を通ります。`LanguageTag::new` のパニックは `const` の中では E0080 になります。",
            },
        ),
        show(Snippet::I18nTagConst),
        para(
            site,
            L {
                en: "Only the shape is checked. Whether the language exists in the IANA registry is not, so `xx` passes.",
                ja: "検査するのは形だけです。言語が IANA の登録簿にあるかどうかは検査しないので、`xx` も通ります。",
            },
        ),
    ])
}

/// The panic of `Locale::tag`.
fn locale_tag(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A segment that is not a tag",
                ja: "タグの形をしていないセグメント",
            },
        ),
        para(
            site,
            L {
                en: "`Locale::tag` reads the tag at run time, from the string serde writes, so the compiler does not see it. A variant renamed to a string that is not shaped as a tag compiles, serves its pages at that segment, and panics when `tag` reads it:",
                ja: "`Locale::tag` はタグを実行時に serde が書く文字列から読むので、コンパイラはそのタグを見ません。タグの形をしていない文字列に rename したバリアントはコンパイルでき、そのセグメントでページも配信されます。そして `tag` がそれを読んだときにパニックします。",
            },
        ),
        show(Snippet::I18nTagPanic),
        para(
            site,
            L {
                en: "In a page handler the panic happens on every request for that language. It drops the connection before a response is written, and the browser shows its own error for a failed connection. Name each variant with its tag, through `rename_all = \"lowercase\"` or `#[serde(rename = \"pt-BR\")]`.",
                ja: "ページのハンドラの中では、その言語へのリクエストのたびにパニックします。パニックはレスポンスを書く前に接続を切るので、ブラウザは接続に失敗したときの自身のエラーを表示します。各バリアントには `rename_all = \"lowercase\"` や `#[serde(rename = \"pt-BR\")]` でタグの名前を付けてください。",
            },
        ),
    ])
}

/// The debug-build check in `Page::render`.
fn render_check(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The check in a debug build",
                ja: "デバッグビルドでの検査",
            },
        ),
        para(
            site,
            L {
                en: "In a debug build, [`Page::render`](api:hypersig/struct.Page.html#method.render) panics where `lang` names the language of an `IntlLocale` and the page formats its plurals and dates only in other `IntlLocale`s, as a page with `lang=\"ja\"` that formats only with `IntlLocale::En` does. A release build skips the check. [Plurals, numbers, and dates](i18n/formats/) shows how to pick the `IntlLocale` from the page's language.",
                ja: "デバッグビルドでは、`lang` が `IntlLocale` のある言語を指していて、ページが複数形と日付をほかの `IntlLocale` だけで書いていると、[`Page::render`](api:hypersig/struct.Page.html#method.render) がパニックします。`lang=\"ja\"` のページが `IntlLocale::En` だけで書式を整える場合がこれに当たります。リリースビルドではこの検査をしません。ページの言語から `IntlLocale` を選ぶ方法は [複数形、数値、日付](i18n/formats/) で説明します。",
            },
        ),
    ])
}

/// The pages after this one.
fn next(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "[Alternate pages](i18n/alternates/) writes the same tags into `hreflang` for search engines.",
                    ja: "[別の言語のページを示す](i18n/alternates/) では同じタグを検索エンジン向けの `hreflang` に書きます。",
                },
                L {
                    en: "[Links to other languages](i18n/language-switcher/) puts `lang` on the links of a language switcher.",
                    ja: "[ほかの言語へのリンク](i18n/language-switcher/) では言語を切り替えるリンクに `lang` を付けます。",
                },
            ],
        ),
    ])
}
