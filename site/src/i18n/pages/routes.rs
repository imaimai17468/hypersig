//! The i18n page on putting the language in the route: the first segment of
//! a `Route<(Lang, ...)>`, what the handler receives, and the 404 for a
//! language the enum lacks.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Pages).leads()),
        route(site),
        handler(site),
        not_found(site),
        urls(site),
        unchecked(site),
        example(site),
        next(site),
    ])
}

/// The language as the first parameter of `routes!`.
fn route(site: Site) -> Part {
    parts([
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
                en: "A [`Locale`](api:hypersig_i18n/trait.Locale.html) enum derives [`Segment`](api:hypersig/trait.Segment.html), so it can be a parameter of a [`Route`](api:hypersig/struct.Route.html). Put it first, in the first segment of the path, and give the page's other parameters after it:",
                ja: "[`Locale`](api:hypersig_i18n/trait.Locale.html) の列挙型は [`Segment`](api:hypersig/trait.Segment.html) を derive しているので、[`Route`](api:hypersig/struct.Route.html) のパラメータにできます。パスの最初のセグメントに置いてパラメータの先頭にし、ページのほかのパラメータをその後に並べます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route<Lang> = \"/{lang}\" => home;
    const MEMBER: Route<(Lang, i32)> = \"/{lang}/members/{id}\" => member;
}",
        ),
        para(
            site,
            L {
                en: "A page with no other parameter takes `Route<Lang>`. A page with more takes a tuple whose first value is the language, as `Route<(Lang, i32)>` does. The segment holds the string serde writes for the variant, the language's tag, so with `#[serde(rename_all = \"lowercase\")]` the paths are `/en/members/7` and `/ja/members/7`.",
                ja: "ほかにパラメータのないページは `Route<Lang>` です。パラメータがほかにもあるページは、`Route<(Lang, i32)>` のように最初の値が言語のタプルになります。セグメントには serde がバリアントに対して書く文字列、つまりその言語のタグが入ります。`#[serde(rename_all = \"lowercase\")]` を付けた場合、パスは `/en/members/7` と `/ja/members/7` になります。",
            },
        ),
    ])
}

/// What the handler takes.
fn handler(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the handler receives",
                ja: "ハンドラが受け取るもの",
            },
        ),
        para(
            site,
            L {
                en: "The handler takes the parameters as one value of the route's type, already parsed, so the language arrives as a `Lang` variant beside the other values. It passes the variant to `pick` for each text and to `Page::lang`:",
                ja: "ハンドラはパラメータを解析済みのルートの型の値 1 つとして受け取ります。そのため、言語はほかの値と並んで `Lang` のバリアントとして届きます。ハンドラはこのバリアントを、文ごとの `pick` と `Page::lang` に渡します。",
            },
        ),
        show(Snippet::I18nRoute),
        para(
            site,
            L {
                en: "The handler never sees a language the enum lacks, so a `match` on `lang` lists only the enum's variants and needs no fallback arm.",
                ja: "enum にない言語がハンドラに届くことはありません。そのため、`lang` に対する `match` は enum のバリアントだけを並べればよく、どれにも当たらないときの腕は要りません。",
            },
        ),
    ])
}

/// The 404 for a segment that names no variant.
fn not_found(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A language the enum lacks",
                ja: "enum にない言語",
            },
        ),
        para(
            site,
            L {
                en: "A first segment that names no variant gets a 404, and the handler does not run. Without an `else` handler in `routes!`, the response has an empty body:",
                ja: "最初のセグメントがどのバリアントにも当たらないリクエストには 404 を返し、ハンドラは動きません。`routes!` に `else` のハンドラがなければ、レスポンスの本文は空です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "GET /fr/members/7 HTTP/1.1

HTTP/1.1 404 Not Found
x-content-type-options: nosniff
content-length: 0",
        ),
        para(
            site,
            L {
                en: "The match is exact, so `/EN/members/7` and `/en-US/members/7` get the same 404. [Error pages](learn/error-pages/) shows how to send a page of your own with it.",
                ja: "照合は完全一致なので、`/EN/members/7` と `/en-US/members/7` にも同じ 404 を返します。404 と一緒に独自のページを送る方法は [エラーページ](learn/error-pages/) で説明します。",
            },
        ),
    ])
}

/// `Route::url` and the compile error for a missing language.
fn urls(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The URL of a page in one language",
                ja: "ある言語のページの URL",
            },
        ),
        para(
            site,
            L {
                en: "[`Route::url`](api:hypersig/struct.Route.html#method.url) takes the same tuple and returns a [`Url`](api:hypersig/struct.Url.html), the path the server redirects to or writes into a header: `MEMBER.url((Lang::Ja, 7))` is `/ja/members/7`. [`Route::link_with`](api:hypersig/struct.Route.html#method.link_with) takes it too and writes an `href`, which [Links to other languages](i18n/language-switcher/) covers.",
                ja: "[`Route::url`](api:hypersig/struct.Route.html#method.url) は同じタプルを受け取り、[`Url`](api:hypersig/struct.Url.html) を返します。サーバーがリダイレクト先にしたりヘッダーに書いたりするパスです。`MEMBER.url((Lang::Ja, 7))` は `/ja/members/7` になります。[`Route::link_with`](api:hypersig/struct.Route.html#method.link_with) も同じタプルを受け取って `href` を書きます。こちらは [ほかの言語へのリンク](i18n/language-switcher/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "Every URL of the page needs its language. Leaving it out, as a site that adds languages to existing pages easily does, is E0308:",
                ja: "ページの URL にはどれも言語が必要です。既存のページに言語を追加したサイトでは言語を書き忘れやすいのですが、言語を省くと E0308 になります。",
            },
        ),
        show(Snippet::I18nUrlWithoutLang),
    ])
}

/// What the route leaves to the program.
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
                    en: "Nothing checks that every page puts the language in its route. A page declared as `Route<i32>` at `/members/{id}` compiles and is served in one language only.",
                    ja: "すべてのページがルートに言語を入れているかどうかは検査しません。`/members/{id}` に `Route<i32>` として宣言したページもコンパイルでき、1 つの言語だけで配信されます。",
                },
                L {
                    en: "`/` names no language, so it gets a 404 unless something serves it. [The redirect from /](i18n/redirect/) sends it to the page in the visitor's language.",
                    ja: "`/` は言語を指さないので、何かが配信しない限り 404 になります。[/ からのリダイレクト](i18n/redirect/) は `/` を訪問者の言語のページへ送ります。",
                },
            ],
        ),
    ])
}

/// A complete page in two languages, with its requests.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A complete example",
                ja: "完全な例",
            },
        ),
        para(
            site,
            L {
                en: "A member page in English and Japanese. The requests at the end get the page in Japanese, a 404 for `fr` and for `EN`, and a 404 for a member that `member` does not know.",
                ja: "英語と日本語のメンバーのページです。最後のリクエストの結果は日本語のページ、`fr` と `EN` に対する 404、`member` が知らないメンバーに対する 404 です。",
            },
        ),
        show(Snippet::I18nPagesExample),
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
                    en: "[Links to other languages](i18n/language-switcher/) links to this page in each language.",
                    ja: "[ほかの言語へのリンク](i18n/language-switcher/) ではこのページの各言語へリンクします。",
                },
                L {
                    en: "[The language of the page](i18n/page-language/) writes the language into `<html lang>`.",
                    ja: "[ページの言語](i18n/page-language/) では言語を `<html lang>` に書きます。",
                },
                L {
                    en: "[Choosing the language](i18n/negotiation/) picks the language from the request's `Accept-Language`.",
                    ja: "[言語の選び方](i18n/negotiation/) ではリクエストの `Accept-Language` から言語を選びます。",
                },
            ],
        ),
    ])
}
