//! The i18n page on linking to the same page in another language: `link_with`
//! with another locale, a switcher over `locales`, and the attributes on its
//! links.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::LanguageSwitcher).leads()),
        link(site),
        locales(site),
        attributes(site),
        wrong(site),
        unchecked(site),
        next(site),
    ])
}

/// `link_with` with another locale.
fn link(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A link with another language",
                ja: "別の言語へのリンク",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<P: Params, Q: Search> Route<P, Q> {
    pub fn link_with<S>(&self, args: impl IntoExpr<S, Q::Args<P>>) -> Link<S>;
}",
        ),
        para(
            site,
            L {
                en: "[`Route::link_with`](api:hypersig/struct.Route.html#method.link_with) takes the route's parameters and returns a [`Link`](api:hypersig/struct.Link.html) for `El::href`. For a route whose first parameter is the language, the same page in another language is the same call with only the language changed: on `/ja/members/7`, `MEMBER.link_with((&Lang::En, id))` writes `href=\"/en/members/7\"`. The language goes by reference, as any enum constant in a link does, because `IntoExpr` is implemented for `&T`. The route builds the path, so a link to another language cannot point at a page the route does not serve.",
                ja: "[`Route::link_with`](api:hypersig/struct.Route.html#method.link_with) はルートのパラメータを受け取り、`El::href` に渡す [`Link`](api:hypersig/struct.Link.html) を返します。最初のパラメータが言語のルートでは、別の言語の同じページへのリンクは、言語だけを変えた同じ呼び出しになります。`/ja/members/7` で `MEMBER.link_with((&Lang::En, id))` と書くと、`href=\"/en/members/7\"` になります。`IntoExpr` は `&T` に実装されているので、リンクに渡す列挙型の定数はどれも参照で渡し、言語も同じく参照で渡します。パスはルートが組み立てるので、別の言語へのリンクがルートの配信しないページを指すことはありません。",
            },
        ),
    ])
}

/// A switcher over every locale.
fn locales(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Listing every language",
                ja: "すべての言語を並べる",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn locales<L: Locale>() -> &'static [L];",
        ),
        para(
            site,
            L {
                en: "[`locales`](api:hypersig_i18n/fn.locales.html) returns every variant of the enum in declaration order, the default first. A language switcher maps over it and links each language to the page the reader is on, with the same remaining parameters. A language added to the enum appears in the switcher without a change to it:",
                ja: "[`locales`](api:hypersig_i18n/fn.locales.html) は enum のすべてのバリアントを宣言の順に返し、既定の言語が先頭に来ます。言語を切り替える UI はこれを `map` でたどり、言語ごとに、読者が今いるページへ残りのパラメータを変えずにリンクします。enum に言語を追加すると、UI を変えなくてもその言語が並びます。",
            },
        ),
        show(Snippet::I18nSwitcher),
        para(
            site,
            L {
                en: "`switcher(Lang::Ja, 7)` writes:",
                ja: "`switcher(Lang::Ja, 7)` は次の HTML を書きます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<nav>
  <ul>
    <li><a href="/en/members/7" hreflang="en" lang="en">English</a></li>
    <li><a href="/ja/members/7" hreflang="ja" lang="ja" aria-current="page">日本語</a></li>
  </ul>
</nav>"#,
        ),
    ])
}

/// What `hreflang`, `lang`, and `aria-current` do on the links.
fn attributes(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The attributes on each link",
                ja: "リンクに付ける属性",
            },
        ),
        para(
            site,
            L {
                en: "`link_with` writes only `href`. The switcher adds three attributes with `El::attr` and the constants of [`html::attr`](api:hypersig/html/attr/index.html):",
                ja: "`link_with` が書くのは `href` だけです。上の UI は `El::attr` と [`html::attr`](api:hypersig/html/attr/index.html) の定数で 3 つの属性を加えます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Attribute", "Value", "What reads it"],
                ja: &["属性", "値", "読むもの"],
            },
            L {
                en: &[
                    &[
                        "`lang`",
                        "The tag of the link's language, from `tag`",
                        "A screen reader reads `日本語` in a Japanese voice on an English page, and the browser picks Japanese glyphs for it",
                    ],
                    &[
                        "`hreflang`",
                        "The tag of the page the link opens",
                        "It states the language of the target. Browsers show nothing for it",
                    ],
                    &[
                        "`aria-current`",
                        "`page`, on the link to the language the reader is in",
                        "A screen reader announces the link as the current page",
                    ],
                ],
                ja: &[
                    &[
                        "`lang`",
                        "`tag` が返す、リンクの言語のタグ",
                        "英語のページでも、スクリーンリーダーは `日本語` を日本語の声で読み、ブラウザは日本語の字形で表示します",
                    ],
                    &[
                        "`hreflang`",
                        "リンク先のページの言語のタグ",
                        "リンク先の言語を示します。ブラウザの表示は変わりません",
                    ],
                    &[
                        "`aria-current`",
                        "読者が今読んでいる言語へのリンクに `page`",
                        "スクリーンリーダーがそのリンクを現在のページとして読み上げます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each link's text is in its own language, `English` and `日本語`, so a reader who cannot read the current page still finds their language.",
                ja: "各リンクの文はリンク先の言語で書きます（`English` と `日本語`）。今のページを読めない読者も、自分の言語を見つけられます。",
            },
        ),
    ])
}

/// The compile error for a link without the remaining parameters.
fn wrong(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査すること",
            },
        ),
        para(
            site,
            L {
                en: "The link takes the whole tuple of the route. A link that gives only the language, and so would lose the page's other parameters, is E0277:",
                ja: "リンクはルートのタプル全体を受け取ります。言語だけを渡してページのほかのパラメータを落としたリンクは E0277 になります。",
            },
        ),
        show(Snippet::I18nSwitcherWrong),
    ])
}

/// What the switcher leaves to the program.
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
                    en: "The values of `El::attr` are text. A misspelled `aria-current` value, such as `pgae`, is written as given, and a screen reader treats a value it does not know as `true`, announcing the link as current without saying it is the page.",
                    ja: "`El::attr` の値は文字列です。`pgae` のように綴りを間違えた `aria-current` の値もそのまま書かれます。スクリーンリーダーは知らない値を `true` として扱うので、そのリンクを「現在のもの」と読み上げますが、ページであるとは伝えません。",
                },
                L {
                    en: "Nothing checks that the remaining parameters are the reader's. A switcher that passes a fixed id instead of the page's links every language to that one member.",
                    ja: "残りのパラメータが読者の今いるページのものかどうかは検査しません。ページの id の代わりに固定の id を渡した UI は、どの言語でもその 1 人のメンバーにリンクします。",
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
                    en: "[The language of the page](i18n/page-language/) writes the page's own language into `<html lang>`.",
                    ja: "[ページの言語](i18n/page-language/) ではページ自身の言語を `<html lang>` に書きます。",
                },
                L {
                    en: "[Alternate pages](i18n/alternates/) tells search engines the address of the page in each language.",
                    ja: "[別の言語のページを示す](i18n/alternates/) では各言語のページのアドレスを検索エンジンに伝えます。",
                },
                L {
                    en: "[Links and redirects](learn/links/) covers `link_with` for any route.",
                    ja: "[リンクとリダイレクト](learn/links/) ではどのルートにも使える `link_with` を説明します。",
                },
            ],
        ),
    ])
}
