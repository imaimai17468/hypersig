//! The page on what a page handler answers when it has no page to show,
//! and on the page `routes!` shows for a request that reaches no page.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::ErrorPages).title(site)),
        lead(site, Doc::Learn(Learn::ErrorPages).leads()),
        answers(site),
        own_page(site),
        missing(site),
        unserved(site),
        unchecked(site),
        next(site),
    ])
}

/// The types a page handler can return and the response each becomes.
fn answers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What a page handler returns",
                ja: "ページのハンドラが返すもの",
            },
        ),
        para(
            site,
            L {
                en: "A page handler returns anything that converts into a [`PageResponse`](api:hypersig/struct.PageResponse.html). A handler that only ever renders returns `Page<S>` or `Option<Page<S>>`. One that sometimes answers otherwise returns `PageResponse` and converts its usual page with `.into()`.",
                ja: "ページのハンドラは [`PageResponse`](api:hypersig/struct.PageResponse.html) に変換できる値を返します。常にページを描くハンドラは `Page<S>` か `Option<Page<S>>` を返します。ときどき別の応答をするハンドラは `PageResponse` を返し、通常のページを `.into()` で変換します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["The handler returns", "The response"],
                ja: &["ハンドラが返すもの", "応答"],
            },
            L {
                en: &[
                    &["`Page<S>`", "`200 OK` with the page"],
                    &[
                        "`Option<Page<S>>`",
                        "The page, or a `404 Not Found` for `None`",
                    ],
                    &[
                        "`PageResponse::with_status(status, page)`",
                        "The page with the status `status`",
                    ],
                    &["`PageResponse::not_found()`", "A `404 Not Found`"],
                    &[
                        "`PageResponse::redirect(url)`",
                        "A `303 See Other` to `url`",
                    ],
                    &[
                        "`(parts, response)`, with `parts: IntoResponseParts`",
                        "The response with the headers of `parts` added, such as a `Set-Cookie`",
                    ],
                    &[
                        "`Result<R, E>`, with `E: IntoResponse`",
                        "`R`'s response, or the error's, so the handler can use `?`",
                    ],
                ],
                ja: &[
                    &["`Page<S>`", "ページと `200 OK`"],
                    &["`Option<Page<S>>`", "ページ。`None` なら `404 Not Found`"],
                    &[
                        "`PageResponse::with_status(status, page)`",
                        "ページとステータス `status`",
                    ],
                    &["`PageResponse::not_found()`", "`404 Not Found`"],
                    &[
                        "`PageResponse::redirect(url)`",
                        "`url` への `303 See Other`",
                    ],
                    &[
                        "`(parts, response)`（`parts: IntoResponseParts`）",
                        "`Set-Cookie` のような `parts` のヘッダーを加えた応答",
                    ],
                    &[
                        "`Result<R, E>`（`E: IntoResponse`）",
                        "`R` の応答かエラーの応答。ハンドラで `?` を使えます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A page goes out with `Cache-Control: private, no-cache`, because it carries the visitor's signals. A `Cache-Control` in `parts` replaces it.",
                ja: "ページは訪問者のシグナルを含むので、`Cache-Control: private, no-cache` を付けて送ります。`parts` に `Cache-Control` を入れると、それに置き換わります。",
            },
        ),
    ])
}

/// `PageResponse::with_status` for a handler that cannot build its page.
fn own_page(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "An error page of your own",
                ja: "独自のエラーページ",
            },
        ),
        para(
            site,
            L {
                en: "A page handler that cannot build its page, such as one whose database does not answer, returns an error page of its own with [`PageResponse::with_status(status: StatusCode, page: Page<S>)`](api:hypersig/struct.PageResponse.html#method.with_status). The browser shows the page, and the status tells a crawler or a monitor that the request failed. One function that builds the error page lets every handler answer with it.",
                ja: "データベースが応答しないときのように、ページを組み立てられなかったハンドラは、[`PageResponse::with_status(status: StatusCode, page: Page<S>)`](api:hypersig/struct.PageResponse.html#method.with_status) で独自のエラーページを返します。ブラウザはそのページを表示し、ステータスはクローラーや監視の仕組みにリクエストの失敗を伝えます。エラーページを作る関数を 1 つ用意すれば、どのハンドラもそれで応答できます。",
            },
        ),
        show(Snippet::ErrorPage),
    ])
}

/// `else HANDLER` on the first line of `routes!`.
fn missing(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Pages for missing paths",
                ja: "見つからないパスのページ",
            },
        ),
        para(
            site,
            L {
                en: "`fn pages(Router<State>) else missing;` on the first line of `routes!` names a handler for each page request that reaches no page:",
                ja: "`routes!` の 1 行目に `fn pages(Router<State>) else missing;` と書くと、どのページにも届かなかったページのリクエストを受け取るハンドラを指定できます。対象は次のリクエストです。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A path no route matches, which gets a 404.",
                    ja: "どのルートにも合わないパス。404 になります。",
                },
                L {
                    en: "A path whose segments do not parse into its route's parameters, which gets a 404.",
                    ja: "セグメントをルートのパラメータに解析できないパス。404 になります。",
                },
                L {
                    en: "A handler that answers `None` or `PageResponse::not_found()`, which gets a 404.",
                    ja: "`None` か `PageResponse::not_found()` を返したハンドラ。404 になります。",
                },
                L {
                    en: "A query string that does not deserialize into its route's search parameters, which gets a 400.",
                    ja: "ルートの検索パラメータにデシリアライズできないクエリ文字列。400 になります。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The handler receives an [`Unserved`](api:hypersig/struct.Unserved.html) where a page's handler receives its parameters, and may take an extractor as a page's handler does. Its page is sent with the status the `Unserved` names, unless it comes from `PageResponse::with_status`. A call to an action, a stream, or an upload at a path no route matches still gets a 404 with an empty body, because a page is no answer to a call.",
                ja: "ハンドラは、ページのハンドラがパラメータを受け取る位置で [`Unserved`](api:hypersig/struct.Unserved.html) を受け取ります。ページのハンドラと同じく、エクストラクタも受け取れます。返したページは `Unserved` が示すステータスで送ります。ただし `PageResponse::with_status` で作ったページはそのステータスで送ります。どのルートにも合わないパスへのアクション、ストリーム、アップロードの呼び出しには、ページが呼び出しへの応答にならないので、本文のない 404 を返します。",
            },
        ),
        para(
            site,
            L {
                en: "The handler takes an `Unserved`, so one written for a page fails to compile:",
                ja: "ハンドラは `Unserved` を受け取るので、ページ用に書いたハンドラはコンパイルできません。",
            },
        ),
        show(Snippet::WrongUnserved),
    ])
}

/// `Unserved::status`, `Unserved::within`, and its `Display`.
fn unserved(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Reading the request that reached no page",
                ja: "どのページにも届かなかったリクエストを読む",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`Unserved::status(&self) -> StatusCode` is the status the page goes out with: 404, or 400 for a query string.",
                    ja: "`Unserved::status(&self) -> StatusCode` はページを送るときのステータスで、404 か、クエリ文字列なら 400 です。",
                },
                L {
                    en: "`Unserved::within(&self, route: Route<P, Q>) -> Option<P>` reads the parameters of `route` from the start of the path. A literal segment must be equal, and a `{name}` segment takes the request's segment at its place, percent-decoded. A site under `/{lang}` shows the page in the language of the path this way.",
                    ja: "`Unserved::within(&self, route: Route<P, Q>) -> Option<P>` は、パスの先頭から `route` のパラメータを読みます。固定のセグメントは等しくなければならず、`{name}` のセグメントはその位置にあるリクエストのセグメントをパーセントデコードして受け取ります。`/{lang}` の下にあるサイトはこの方法でパスの言語のページを出せます。",
                },
                L {
                    en: "Its `Display` is one fixed sentence per reason, naming the path, which the page can show the visitor, such as \"no page is served at `/en/oders/3`\".",
                    ja: "`Display` は理由ごとに決まった 1 文で、パスを含みます。\"no page is served at `/en/oders/3`\" のように、訪問者に見せられる文です。",
                },
                L {
                    en: "For a query string, its `Error::source` is the reader's message, which quotes the query and names the types of the search parameters. It belongs in a log.",
                    ja: "クエリ文字列の場合、`Error::source` はクエリを引用し検索パラメータの型を挙げたメッセージです。訪問者ではなくログに向けたものです。",
                },
            ],
        ),
        show(Snippet::MissingPage),
    ])
}

/// What `else` leaves to the program.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What stays unchecked",
                ja: "検査されないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Without `else`, a 404 has an empty body, and a 400 for a query string has a one-line text body.",
                    ja: "`else` がなければ、404 の本文は空で、クエリ文字列への 400 の本文は 1 行の文です。",
                },
                L {
                    en: "The function `routes!` writes registers the `else` handler as the router's fallback. A later `Router::fallback` replaces it for paths no route matches, and merging the router with another that has a fallback panics, as axum does for any two fallbacks.",
                    ja: "`routes!` が書く関数は `else` のハンドラをルーターのフォールバックとして登録します。あとから `Router::fallback` を呼ぶと、どのルートにも合わないパスについてはそれに置き換わります。フォールバックを持つ別のルーターと `merge` すると、axum が 2 つのフォールバックに対してそうするのと同じくパニックします。",
                },
                L {
                    en: "An `else` handler that answers `None` has no page to fall back on, and the request gets a 404 with an empty body.",
                    ja: "`else` のハンドラが `None` を返すと、代わりに出すページがないので、そのリクエストには本文のない 404 を返します。",
                },
                L {
                    en: "`PageResponse::with_status` takes any status, so nothing checks that an error page goes out with an error status.",
                    ja: "`PageResponse::with_status` はどのステータスでも受け取るので、エラーページがエラーのステータスで送られるかどうかは検査されません。",
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
                    en: "[Serving under a path prefix](learn/base-path/) moves every page, including the error pages, under a prefix such as `/app`.",
                    ja: "[パスの接頭辞の下で配信する](learn/base-path/) ではエラーページを含むすべてのページを `/app` のような接頭辞の下に移します。",
                },
                L {
                    en: "[Errors and logging](learn/errors/) covers the errors of actions and how to log them.",
                    ja: "アクションのエラーとそのログの取り方は [エラーとログ](learn/errors/) で説明します。",
                },
            ],
        ),
    ])
}
