//! The i18n page on `LocaleRouterExt::redirect_to_locale`: the `axum`
//! feature, the HTTP exchange of the redirect from `/`, and the route it
//! takes.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Redirect).leads()),
        redirect(site),
        exchange(site),
        route(site),
        unchecked(site),
        example(site),
        static_sites(site),
        next(site),
    ])
}

/// `redirect_to_locale` and the feature it needs.
fn redirect(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "redirect_to_locale",
                ja: "redirect_to_locale",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub trait LocaleRouterExt<St>: Sized {
    fn redirect_to_locale<L: Locale + Send + Sync>(self, home: Route<L>) -> Self;
}

impl<St: Clone + Send + Sync + 'static> LocaleRouterExt<St> for Router<St> { /* ... */ }",
        ),
        para(
            site,
            L {
                en: "[`LocaleRouterExt`](api:hypersig_i18n/trait.LocaleRouterExt.html) extends axum's `Router` with [`redirect_to_locale`](api:hypersig_i18n/trait.LocaleRouterExt.html#tymethod.redirect_to_locale), which adds a route for `GET /`. It reads `Accept-Language`, picks the language with [`negotiate`](i18n/negotiation/), and redirects to `home` in that language. It comes with the `axum` feature of `hypersig_i18n`, which is on by default. With `default-features = false`, the trait does not exist.",
                ja: "[`LocaleRouterExt`](api:hypersig_i18n/trait.LocaleRouterExt.html) は axum の `Router` に [`redirect_to_locale`](api:hypersig_i18n/trait.LocaleRouterExt.html#tymethod.redirect_to_locale) を加えます。このメソッドは `GET /` のルートを追加します。そのルートは `Accept-Language` を読み、[`negotiate`](i18n/negotiation/) で言語を選び、その言語の `home` へリダイレクトします。このトレイトは `hypersig_i18n` の `axum` の feature で使えるようになり、この feature は既定で有効です。`default-features = false` にすると、トレイトはありません。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "let app = pages(Router::new()).redirect_to_locale(HOME);",
        ),
    ])
}

/// The request and the response.
fn exchange(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTTP exchange",
                ja: "HTTP のやり取り",
            },
        ),
        para(
            site,
            L {
                en: "For a browser set to Japanese, with `HOME` at `/{lang}`:",
                ja: "日本語に設定したブラウザでは、`HOME` が `/{lang}` のとき次のようになります。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "GET / HTTP/1.1
Accept-Language: ja-JP,ja;q=0.9,en;q=0.8

HTTP/1.1 307 Temporary Redirect
location: /ja
vary: accept-language
content-length: 0",
        ),
        para(
            site,
            L {
                en: "The browser then requests `/ja`. A request without `Accept-Language` gets `location: /en`, the first variant.",
                ja: "ブラウザは続けて `/ja` をリクエストします。`Accept-Language` のないリクエストには最初のバリアントの `location: /en` を返します。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`307 Temporary Redirect` keeps `/` as the address to come back to. A browser or a search engine does not record `/ja` in its place, as it would for a `308`, so the next visit to `/` is negotiated again.",
                    ja: "`307 Temporary Redirect` は、`/` を次も訪れるアドレスとして残します。`308` のときのように、ブラウザや検索エンジンが `/` の代わりに `/ja` を記録することはないので、次に `/` を訪れたときも改めて言語を選びます。",
                },
                L {
                    en: "`vary: accept-language` tells caches that the response differs by that header, so a cache does not send a reader of English to `/ja`.",
                    ja: "`vary: accept-language` は、レスポンスがこのヘッダーで変わることをキャッシュに伝えます。そのため、キャッシュが英語を読む人を `/ja` へ送ることはありません。",
                },
            ],
        ),
    ])
}

/// The route `redirect_to_locale` takes, and its compile error.
fn route(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The route it takes",
                ja: "受け取るルート",
            },
        ),
        para(
            site,
            L {
                en: "`home` is a `Route<L>` whose only parameter is the language, because `/` holds nothing else to fill a parameter with. `L` is inferred from the route, so a route with more parameters, such as `Route<(Lang, i32)>`, makes `L` the tuple, which is not a `Locale`, and is E0277:",
                ja: "`/` には言語のほかにパラメータを埋める値がないので、`home` は言語だけをパラメータに持つ `Route<L>` です。`L` はルートから推論されます。そのため、`Route<(Lang, i32)>` のようにパラメータの多いルートを渡すと `L` がタプルになり、タプルは `Locale` ではないので E0277 になります。",
            },
        ),
        show(Snippet::I18nRedirectRoute),
    ])
}

/// What the redirect leaves to the program.
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
                    en: "The types do not check that no other route serves `/`. If two routes handle `GET /`, axum panics when the router is built, with `Overlapping method route`.",
                    ja: "ほかのルートが `/` を配信していないかどうかは、型では検査しません。2 つのルートが `GET /` を受け持つと、axum はルーターを組み立てるときに `Overlapping method route` でパニックします。",
                },
                L {
                    en: "The redirect covers `/` alone. A path without a language, such as `/members/7`, gets a 404 unless a handler of your own sends it on, as [Choosing the language](i18n/negotiation/#in-a-handler-of-your-own) shows.",
                    ja: "リダイレクトするのは `/` だけです。`/members/7` のように言語のないパスは、[言語の選び方](i18n/negotiation/#in-a-handler-of-your-own) のように独自のハンドラで送らない限り 404 になります。",
                },
            ],
        ),
    ])
}

/// A complete program with its requests.
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
                en: "A home page in English and Japanese, with the redirect from `/`. The requests at the end check the exchange above.",
                ja: "英語と日本語のホームページに、`/` からのリダイレクトを加えたものです。最後のリクエストで、上のやり取りを確かめます。",
            },
        ),
        show(Snippet::I18nRedirect),
    ])
}

/// Sites served as static files.
fn static_sites(site: Site) -> Part {
    note(
        site,
        L {
            en: "A site served as static files, such as this one, has no server to read `Accept-Language`, so it cannot redirect by language. Such a site serves its default language at `/` and links to the other languages from every page with a [language switcher](i18n/language-switcher/).",
            ja: "このサイトのように静的ファイルとして配信するサイトには `Accept-Language` を読むサーバーがないので、言語でリダイレクトできません。こうしたサイトは既定の言語を `/` で配信し、どのページからも [言語を切り替えるリンク](i18n/language-switcher/) でほかの言語へリンクします。",
        },
    )
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
                    en: "[Alternate pages](i18n/alternates/) tells search engines the address of each page in every language.",
                    ja: "[別の言語のページを示す](i18n/alternates/) では各ページのすべての言語のアドレスを検索エンジンに伝えます。",
                },
                L {
                    en: "[Plurals, numbers, and dates](i18n/formats/) writes values in the page's language.",
                    ja: "[複数形、数値、日付](i18n/formats/) では値をページの言語で書きます。",
                },
            ],
        ),
    ])
}
