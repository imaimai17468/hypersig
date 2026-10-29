//! The i18n page on alternate pages: `LocalePage::alternates`, the
//! `<link rel="alternate">` tags it writes, `Origin`, and the one-off
//! `Page::alternate` and `Page::alternate_default`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Alternates).leads()),
        alternates(site),
        origin(site),
        one_off(site),
        unchecked(site),
        next(site),
    ])
}

/// `LocalePage::alternates` and the tags it writes.
fn alternates(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "LocalePage::alternates",
                ja: "LocalePage::alternates",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub trait LocalePage: Sized {
    fn alternates<L: Locale>(self, origin: Origin, url: impl Fn(L) -> Url) -> Self;
}

impl<S: Signals> LocalePage for Page<S> { /* ... */ }",
        ),
        para(
            site,
            L {
                en: "[`LocalePage::alternates`](api:hypersig_i18n/trait.LocalePage.html#tymethod.alternates) takes the site's [`Origin`](api:hypersig/struct.Origin.html) and a function from a language to the page's [`Url`](api:hypersig/struct.Url.html) in that language, which is the route's `url` with the language changed. It calls the function once per variant of [`locales`](api:hypersig_i18n/fn.locales.html) and writes one `<link rel=\"alternate\">` into `<head>` for each, with the variant's `tag` as `hreflang`. It writes one more with `hreflang=\"x-default\"` at the first variant, the default language:",
                ja: "[`LocalePage::alternates`](api:hypersig_i18n/trait.LocalePage.html#tymethod.alternates) は、サイトの [`Origin`](api:hypersig/struct.Origin.html) と、言語からその言語のページの [`Url`](api:hypersig/struct.Url.html) を返す関数を受け取ります。関数はルートの `url` の言語を変えたものです。`alternates` は [`locales`](api:hypersig_i18n/fn.locales.html) のバリアントごとに関数を呼び、それぞれのバリアントの `tag` を `hreflang` にした `<link rel=\"alternate\">` を `<head>` に 1 つずつ書きます。最後に、最初のバリアント、つまり既定の言語に `hreflang=\"x-default\"` を付けたものをもう 1 つ書きます。",
            },
        ),
        show(Snippet::I18nAlternates),
        para(
            site,
            L {
                en: "The page in either language writes the same three tags into `<head>`:",
                ja: "どちらの言語のページも `<head>` に同じ 3 つのタグを書きます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<link rel="alternate" hreflang="en" href="https://example.com/en/posts/3">
<link rel="alternate" hreflang="ja" href="https://example.com/ja/posts/3">
<link rel="alternate" hreflang="x-default" href="https://example.com/en/posts/3">"#,
        ),
        para(
            site,
            L {
                en: "A search engine reads these tags to show each searcher the page in their language, and the page at `x-default` to a searcher whose language none of them names. Browsers do nothing visible with them.",
                ja: "検索エンジンはこのタグを読み、検索した人にその人の言語のページを結果に出します。どの `hreflang` にも当たらない言語の人には `x-default` のページを出します。ブラウザの表示は変わりません。",
            },
        ),
        para(
            site,
            L {
                en: "The function returns a `Url`, which only a route builds, so a path written by hand is E0308:",
                ja: "関数が返すのは `Url` で、`Url` を作れるのはルートだけです。そのため、手で書いたパスは E0308 になります。",
            },
        ),
        show(Snippet::I18nAlternatesString),
    ])
}

/// `Origin::new` and what it checks.
fn origin(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The origin",
                ja: "オリジン",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Origin {
    pub const fn new(origin: &'static str) -> Self;
    pub const fn under(self, prefix: &'static str) -> Self;
}",
        ),
        para(
            site,
            L {
                en: "Search engines read `href` here only as an absolute URL, and a `Url` is a path, so `alternates` writes each one after an `Origin`. [`Origin::new`](api:hypersig/struct.Origin.html#method.new) takes `http://` or `https://`, a host, and an optional port, and nothing after them. [`Origin::under`](api:hypersig/struct.Origin.html#method.under) adds the path the site is served under, such as `/blog`. Both panic on anything else, which in a `const` is E0080:",
                ja: "検索エンジンはここの `href` を絶対 URL としてしか読まず、`Url` はパスです。そのため `alternates` はそれぞれの `Url` を `Origin` の後ろに書きます。[`Origin::new`](api:hypersig/struct.Origin.html#method.new) が受け取るのは `http://` か `https://`、ホスト、省略できるポートで、その後ろには何も書けません。[`Origin::under`](api:hypersig/struct.Origin.html#method.under) は `/blog` のように、サイトを配信するパスを加えます。どちらもそれ以外の値ではパニックし、`const` の中ではそれが E0080 になります。",
            },
        ),
        show(Snippet::OriginPath),
        para(
            site,
            L {
                en: "An origin read from configuration at startup goes through [`Origin::parse`](api:hypersig/struct.Origin.html#method.parse), which returns an error in place of the panic.",
                ja: "起動時に設定から読むオリジンには [`Origin::parse`](api:hypersig/struct.Origin.html#method.parse) を使います。パニックの代わりにエラーを返します。",
            },
        ),
    ])
}

/// `Page::alternate` and `Page::alternate_default`.
fn one_off(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "One alternate at a time",
                ja: "代替ページを 1 つずつ書く",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: Signals> Page<S> {
    pub fn alternate(self, origin: Origin, url: &Url, hreflang: impl Into<LanguageTag>) -> Self;
    pub fn alternate_default(self, origin: Origin, url: &Url) -> Self;
}",
        ),
        para(
            site,
            L {
                en: "`alternates` covers pages whose route holds the language. For a page whose versions sit at routes of their own, such as `/about` and `/ja/kaisha`, [`Page::alternate`](api:hypersig/struct.Page.html#method.alternate) writes one `<link rel=\"alternate\">` per call, and [`Page::alternate_default`](api:hypersig/struct.Page.html#method.alternate_default) writes the `x-default`. `hreflang` takes a `LanguageTag` or an `IntlLocale`, as `Page::lang` does:",
                ja: "`alternates` は、ルートに言語を持つページのためのものです。`/about` と `/ja/kaisha` のように言語ごとに別のルートを持つページには、[`Page::alternate`](api:hypersig/struct.Page.html#method.alternate) を使います。1 回の呼び出しで `<link rel=\"alternate\">` を 1 つ書き、[`Page::alternate_default`](api:hypersig/struct.Page.html#method.alternate_default) は `x-default` を書きます。`hreflang` には `Page::lang` と同じく `LanguageTag` か `IntlLocale` を渡します。",
            },
        ),
        show(Snippet::I18nAlternateOne),
    ])
}

/// What alternates leave to the program.
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
                    en: "Nothing checks that the function returns the same page. A function that returns `HOME.url(lang)` for every page tells search engines that every page is the home page in another language, and they may drop the pages from their results.",
                    ja: "関数が同じページを返すかどうかは検査しません。どのページでも `HOME.url(lang)` を返す関数は、すべてのページが別の言語のホームだと検索エンジンに伝えることになり、ページが検索結果から外れることがあります。",
                },
                L {
                    en: "Search engines use the tags only when each page in the set lists all of them, its own included. Call `alternates` on the page in every language.",
                    ja: "検索エンジンがこのタグを使うのは、組になるページがどれも、自分自身を含むすべてのタグを書いているときだけです。`alternates` はすべての言語のページで呼んでください。",
                },
                L {
                    en: "Nothing checks that the origin is where the site is served. A wrong host sends search engines to pages that are not the site's.",
                    ja: "オリジンがサイトを配信する場所かどうかは検査しません。ホストを間違えると、検索エンジンはサイトのものではないページを見に行きます。",
                },
            ],
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
                    en: "[Choosing the language](i18n/negotiation/) picks the language a visitor's browser asks for.",
                    ja: "[言語の選び方](i18n/negotiation/) では訪問者のブラウザが求める言語を選びます。",
                },
                L {
                    en: "[The page and its head](learn/page/) covers the other tags `Page` writes into `<head>`.",
                    ja: "[ページと head](learn/page/) では `Page` が `<head>` に書くほかのタグを説明します。",
                },
            ],
        ),
    ])
}
