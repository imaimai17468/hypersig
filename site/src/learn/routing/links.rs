//! The page on links, URLs written on the server, redirects, and navigation.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Links).title(site)),
        lead(site, Doc::Learn(Learn::Links).leads()),
        links(site),
        written(site),
        link_errors(site),
        redirects(site),
        unchecked(site),
        next(site),
    ])
}

/// `link`, `link_with`, and `El::href`.
fn links(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Links",
                ja: "リンク",
            },
        ),
        para(
            site,
            L {
                en: "A [`Link<S>`](api:hypersig/struct.Link.html) points at a page the router serves, for an element on the page whose signals are `S`. `Route::link(&self) -> Link<S>` builds one for a route without parameters. `Route::link_with(&self, args) -> Link<S>` builds one for a route with them, where `args` are expressions of the route's types, as the arguments of an action call are. A route with search parameters takes `(P, Q)`, or `Q` alone where `P` is `()`.",
                ja: "[`Link<S>`](api:hypersig/struct.Link.html) は、ルーターが配信するページを指すリンクで、シグナルが `S` のページの要素に使います。パラメータのないルートでは `Route::link(&self) -> Link<S>` がリンクを作ります。パラメータのあるルートでは `Route::link_with(&self, args) -> Link<S>` が作ります。`args` は、アクションを呼び出すときの引数と同じく、ルートの型の式です。検索パラメータのあるルートは `(P, Q)` を受け取り、`P` が `()` なら `Q` だけを受け取ります。",
            },
        ),
        para(
            site,
            L {
                en: "[`El::href`](api:hypersig/html/struct.El.html#method.href) takes the link. It also takes a [`Url`](api:hypersig/struct.Url.html) written on the server, and a [`SafeUrl`](api:hypersig/struct.SafeUrl.html) for an address the router does not serve.",
                ja: "リンクは [`El::href`](api:hypersig/html/struct.El.html#method.href) に渡します。`El::href` はサーバーで書いた [`Url`](api:hypersig/struct.Url.html) と、ルーターが配信しないアドレスのための [`SafeUrl`](api:hypersig/struct.SafeUrl.html) も受け取ります。",
            },
        ),
        show(Snippet::Links),
    ])
}

/// The HTML a fixed link and a bound link write.
fn written(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "What a link writes",
                ja: "リンクが書き出すもの",
            },
        ),
        para(
            site,
            L {
                en: "Literals give a fixed `href`. A parameter that reads a signal or a list item is evaluated in the browser and kept up to date: `Page::render` writes the URL of the signals' first values into `href`, and the browser rewrites `href` each time a signal the link reads changes. Both sides percent-encode a value as `encodeURIComponent` does, so the first HTML and the browser write the same URL. In the sample above, the page's first HTML holds `<a href=\"/teams/red/members/7\">`.",
                ja: "リテラルを渡すと `href` は固定になります。シグナルやリストの項目を読むパラメータは、ブラウザで評価され、値が変わるたびに更新されます。`Page::render` はシグナルの最初の値から作った URL を `href` に書き、ブラウザはリンクが読むシグナルが変わるたびに `href` を書き直します。両側とも `encodeURIComponent` と同じ方法で値をパーセントエンコードするので、最初の HTML とブラウザは同じ URL を書きます。上のサンプルでは、ページの最初の HTML に `<a href=\"/teams/red/members/7\">` が入ります。",
            },
        ),
    ])
}

/// The mistakes `Link` and `El::href` refuse.
fn link_errors(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査すること",
            },
        ),
        para(
            site,
            L {
                en: "`El::href` does not take a string, so a URL cannot be written by hand:",
                ja: "`El::href` は文字列を受け取らないので、URL を手で書くことはできません。",
            },
        ),
        show(Snippet::WrongHref),
        bullets(
            site,
            &[
                L {
                    en: "`link` exists only on `Route<(), ()>`, so calling it on a route with parameters is error E0599, \"no method named `link`\".",
                    ja: "`link` は `Route<(), ()>` にしかないので、パラメータのあるルートで呼ぶとエラー E0599（\"no method named `link`\"）になります。",
                },
                L {
                    en: "A parameter of the wrong type, such as `MEMBER.link_with(\"seven\")` for a `Route<i32>`, is error E0277, \"`&str` is not an expression of type `i32`\".",
                    ja: "`Route<i32>` に対する `MEMBER.link_with(\"seven\")` のように型の違うパラメータは、エラー E0277（\"`&str` is not an expression of type `i32`\"）になります。",
                },
            ],
        ),
    ])
}

/// `Route::url`, `PageResponse::redirect`, `Patches::navigate`, and `Link::navigate`.
fn redirects(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Redirects and navigation",
                ja: "リダイレクトとページの移動",
            },
        ),
        para(
            site,
            L {
                en: "The server writes a page's URL with `Route::url(&self, args) -> Url`. It takes the route's parameter values themselves, in the shape `link_with` takes, and encodes them as a link does. `Url::as_str` and `Display` give the text.",
                ja: "サーバーは `Route::url(&self, args) -> Url` でページの URL を書きます。`url` は `link_with` と同じ形でルートのパラメータの値そのものを受け取り、リンクと同じ方法でエンコードします。文字列は `Url::as_str` か `Display` で取り出します。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A page handler that sends the browser elsewhere returns a `PageResponse`, built with `PageResponse::redirect(url: Url)`, which answers `303 See Other`. Its usual page converts with `.into()`.",
                    ja: "ブラウザを別のページへ送るページのハンドラは `PageResponse` を返します。`PageResponse::redirect(url: Url)` で作り、`303 See Other` で応答します。通常のページは `.into()` で変換します。",
                },
                L {
                    en: "An action does the same with `Patches::navigate(url: Url)`. The browser follows it once the patches before it are applied, and the patches after it in the same answer are not applied.",
                    ja: "アクションでは `Patches::navigate(url: Url)` で同じことをします。ブラウザはそれより前の差分を適用してから移動し、同じ応答でそれより後の差分は適用しません。",
                },
                L {
                    en: "In the browser, `Link::navigate(self) -> Stmt<S>` is a statement that loads the linked page, as following the link would. It is for a handler that should leave the page after it runs other statements.",
                    ja: "ブラウザでは、`Link::navigate(self) -> Stmt<S>` がリンク先のページを読み込む文です。リンクをたどったときと同じ動きをします。ほかの文を実行してからページを離れるハンドラに使います。",
                },
            ],
        ),
        show(Snippet::Navigation),
    ])
}

/// What links leave to the program.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What links leave unchecked",
                ja: "リンクが検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A link is a plain `<a href>`, so following it loads the next page from the server, and the signals of the page you leave are gone.",
                    ja: "リンクは通常の `<a href>` なので、たどると次のページをサーバーから読み込み、離れたページのシグナルは失われます。",
                },
                L {
                    en: "A link to another site or to a `#fragment` is a [`SafeUrl`](api:hypersig/struct.SafeUrl.html), which [Values the types do not check](learn/unchecked-values/) covers. It checks the URL's scheme, and nothing checks that the address it names exists.",
                    ja: "ほかのサイトや `#fragment` へのリンクには [`SafeUrl`](api:hypersig/struct.SafeUrl.html) を使います。これは [型で検査しない値](learn/unchecked-values/) で説明します。`SafeUrl` が検査するのは URL のスキームだけで、その先のアドレスがあるかどうかは検査されません。",
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
                    en: "[Pages under a page](learn/child-pages/) links to a page whose path continues another's.",
                    ja: "[子のページ](learn/child-pages/) では別のページのパスの続きにあるページへリンクします。",
                },
                L {
                    en: "[Search parameters](learn/search-params/) changes the URL in the address bar without loading a page.",
                    ja: "[検索パラメータ](learn/search-params/) ではページを読み込まずにアドレスバーの URL を変えます。",
                },
                L {
                    en: "[Error pages](learn/error-pages/) answers with a status of your choice.",
                    ja: "[エラーページ](learn/error-pages/) では選んだステータスで応答します。",
                },
            ],
        ),
    ])
}
