//! The page on what a handler reads from the request and writes beside its
//! answer: the request a call sends, extractors, headers such as cookies,
//! and the requests an action refuses.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, subheading, table_of, title};

/// The article.
pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Requests).title(site)),
        lead(site, Doc::Learn(Learn::Requests).leads()),
        the_request(site),
        extractors(site),
        own_extractor(site),
        headers(site),
        refused(site),
        unchecked(site),
        next(site),
    ])
}

/// The request the runtime sends for a call.
fn the_request(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What a call sends",
                ja: "呼び出しが送るもの",
            },
        ),
        para(
            site,
            L {
                en: "A call to an [`Action<S, A>`](api:hypersig/struct.Action.html) sends the arguments `A` and nothing else. A `post` action sends them as a JSON body, and a `get` action as JSON in the `args` query parameter. Every call asks for `text/event-stream` and carries a `hypersig-call` header. A call of `JOIN` from [Forms](learn/forms/) looks like this:",
                ja: "[`Action<S, A>`](api:hypersig/struct.Action.html) の呼び出しは引数 `A` だけを送ります。`post` のアクションは引数を JSON の本文で、`get` のアクションは `args` のクエリパラメータに入れた JSON で送ります。どの呼び出しも `text/event-stream` を求め、`hypersig-call` のヘッダーを付けます。[フォーム](learn/forms/) の `JOIN` の呼び出しは次のようになります。",
            },
        ),
        code::block(
            Lang::JavaScript,
            None,
            "POST /join\naccept: text/event-stream\ncontent-type: application/json\n\n{\"email\":\"ada@example.com\",\"password\":\"correct horse\"}",
        ),
        para(
            site,
            L {
                en: "A handler that needs several values takes a tuple, which the page passes as a tuple of expressions. The cookies, the `User-Agent`, and the rest of the headers come with the request too, and a handler reads them through an extractor. [`Action::request`](api:hypersig/struct.Action.html#method.request) builds the same request for a test.",
                ja: "複数の値が要るハンドラはタプルを受け取り、ページは式のタプルを渡します。Cookie、`User-Agent` などのヘッダーもリクエストと一緒に届き、ハンドラはそれをエクストラクタで読みます。テストでは [`Action::request`](api:hypersig/struct.Action.html#method.request) が同じリクエストを作ります。",
            },
        ),
    ])
}

/// The two forms of a handler, `HeaderMap`, and the compile error for an extractor in the wrong place.
fn extractors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Extractors and headers",
                ja: "エクストラクタとヘッダー",
            },
        ),
        para(
            site,
            L {
                en: "An extractor is an axum type that reads a part of the request, such as its headers or a cookie. A page or action handler takes the state and its arguments, and may take one [`FromRequestParts`](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html) extractor between them. [`Handler`](api:hypersig/trait.Handler.html) is implemented for these two forms, where `St` is the router's state and `A` the page's parameters or the action's arguments:",
                ja: "エクストラクタはヘッダーや Cookie などリクエストの一部を読む axum の型です。ページとアクションのハンドラは状態と引数を受け取り、そのあいだに [`FromRequestParts`](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html) のエクストラクタを 1 つ受け取れます。[`Handler`](api:hypersig/trait.Handler.html) は次の 2 つの形に実装されています。`St` はルーターの状態、`A` はページのパラメータかアクションの引数です。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "async fn handler(state: St, args: A) -> O\nasync fn handler(state: St, extractor: E, args: A) -> O",
        ),
        para(
            site,
            L {
                en: "`E` can be `HeaderMap`, a tuple of extractors, which counts as one, or the application's own type, such as the signed-in member read from a session cookie. The extractor runs before the handler, and when it fails, its rejection is the response and the handler does not run.",
                ja: "`E` には `HeaderMap`、1 つとして数えられるエクストラクタのタプル、セッションの Cookie から読んだサインイン中のメンバーのようなアプリケーションの型が使えます。エクストラクタはハンドラより先に動き、失敗するとその拒否が応答になって、ハンドラは動きません。",
            },
        ),
        show(Snippet::Headers),
        para(
            site,
            L {
                en: "An extractor after the arguments, or two extractors outside a tuple, matches neither form, and `routes!` does not compile:",
                ja: "引数のあとのエクストラクタや、タプルにしていない 2 つのエクストラクタはどちらの形にも合わないので、`routes!` がコンパイルできません。",
            },
        ),
        show(Snippet::ExtractorOrder),
    ])
}

/// An extractor that reads the session cookie and refuses a visitor who is signed out.
fn own_extractor(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "An extractor of your own",
                ja: "自作のエクストラクタ",
            },
        ),
        para(
            site,
            L {
                en: "The extractor below reads the member from the session cookie and refuses a request without one. Its rejection depends on who sent the request, which [`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) tells from the `Accept` header: the runtime asks for `text/event-stream` when it calls an action, and a browser loading a page asks for HTML.",
                ja: "下のエクストラクタはセッションの Cookie からメンバーを読み、Cookie のないリクエストを拒否します。拒否の応答はリクエストを送ったものによって変わり、[`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) が `Accept` ヘッダーからそれを見分けます。ランタイムはアクションを呼ぶときに `text/event-stream` を求め、ページを読み込むブラウザは HTML を求めます。",
            },
        ),
        show(Snippet::Extractors),
        para(
            site,
            L {
                en: "An extractor that guards actions answers a signed-out request with a status such as 401, which the page reads through [`Call::status`](api:hypersig/struct.Call.html#method.status). The browser's `fetch` follows a redirect, so an action answered with one receives the sign-in page's HTML. The runtime counts that answer as a failure with status 0, which the page cannot tell from a dropped connection.",
                ja: "アクションを守るエクストラクタは、サインインしていないリクエストに 401 などのステータスで応答します。ページはこのステータスを [`Call::status`](api:hypersig/struct.Call.html#method.status) で読めます。ブラウザの `fetch` はリダイレクトをたどるので、リダイレクトで応答したアクションはサインインのページの HTML を受け取ります。ランタイムはこの応答をステータス 0 の失敗として扱うので、ページからは接続が切れた場合と見分けられません。",
            },
        ),
        para(
            site,
            L {
                en: "The cookie in this sample is read by hand to show the extractor, and [Signing in](learn/sessions/) keeps the session with `tower-sessions`.",
                ja: "このサンプルはエクストラクタを示すために Cookie を手で読んでいます。セッションを `tower-sessions` で持つ方法は [サインインとセッション](learn/sessions/) にあります。",
            },
        ),
    ])
}

/// Headers sent beside a page or patches.
fn headers(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Headers beside the answer",
                ja: "応答と一緒に送るヘッダー",
            },
        ),
        para(
            site,
            L {
                en: "A pair of headers and a page or patches, such as `([(SET_COOKIE, value)], patches)` in `sign_out` above, sends the headers with the answer. The first half is any axum [`IntoResponseParts`](https://docs.rs/axum/0.8/axum/response/trait.IntoResponseParts.html), so an array of name and value pairs works, and so does a `HeaderMap`.",
                ja: "上の `sign_out` の `([(SET_COOKIE, value)], patches)` のように、ヘッダーとページや差分を組にすると、ヘッダーが応答と一緒に送られます。組の前半は axum の [`IntoResponseParts`](https://docs.rs/axum/0.8/axum/response/trait.IntoResponseParts.html) なら何でもよいので、名前と値の組の配列も `HeaderMap` も使えます。",
            },
        ),
        para(
            site,
            L {
                en: "A page goes out with `Cache-Control: private, no-cache`, because it carries the visitor's signals. A `Cache-Control` the handler returns beside the page replaces it.",
                ja: "ページは訪問者のシグナルを持つので、`Cache-Control: private, no-cache` を付けて送られます。ハンドラがページと一緒に `Cache-Control` を返すと、それに置き換わります。",
            },
        ),
    ])
}

/// The requests the router refuses before the handler runs.
fn refused(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Requests an action refuses",
                ja: "アクションが拒否するリクエスト",
            },
        ),
        para(
            site,
            L {
                en: "Before its handler runs, an action refuses the requests a page on another site could send with the visitor's cookie, and [Security](learn/security/) lists them. The refusal is a [`RequestError`](api:hypersig/struct.RequestError.html), whose response has one of these statuses and one fixed sentence as its `text/plain` body:",
                ja: "アクションは、別のサイトのページが訪問者の Cookie を付けて送れるリクエストを、ハンドラが動く前に拒否します。拒否するリクエストの一覧は [セキュリティ](learn/security/) にあります。拒否は [`RequestError`](api:hypersig/struct.RequestError.html) で、その応答は次のどれかのステータスと、決まった 1 文の `text/plain` の本文を持ちます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Status", "Request"],
                ja: &["ステータス", "リクエスト"],
            },
            L {
                en: &[
                    &[
                        "403 Forbidden",
                        "A `Sec-Fetch-Site` other than `same-origin`, or a `get` call without the `hypersig-call` header",
                    ],
                    &[
                        "415 Unsupported Media Type",
                        "A `post` call whose body is not `application/json`",
                    ],
                    &[
                        "400 Bad Request",
                        "Arguments that do not deserialize into `A`",
                    ],
                ],
                ja: &[
                    &[
                        "403 Forbidden",
                        "`same-origin` 以外の `Sec-Fetch-Site`、または `hypersig-call` のヘッダーのない `get` の呼び出し",
                    ],
                    &[
                        "415 Unsupported Media Type",
                        "本文が `application/json` でない `post` の呼び出し",
                    ],
                    &["400 Bad Request", "`A` にデシリアライズできない引数"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A request without `Sec-Fetch-Site` passes. Some browsers still let such a request reach a GET action, so use POST for an action that changes something. The `auth` example in the repository signs in with a cookie session end to end. [Uploads](learn/uploads/) lists the refusals of an upload.",
                ja: "`Sec-Fetch-Site` のないリクエストは通ります。ブラウザによってはそうしたリクエストが GET のアクションに届くので、何かを変更するアクションには POST を使ってください。リポジトリの `auth` の例は Cookie のセッションでのサインインを最初から最後まで示します。アップロードの拒否は [アップロード](learn/uploads/) にあります。",
            },
        ),
    ])
}

/// What the types leave to the handler.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the compiler does not check",
                ja: "コンパイラが検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "The type `A` holds, and its values do not. The visitor's browser can send any value of `A`, whatever the page's expressions would compute, and anyone can call the action's URL without loading the page. A handler that guards the page does not guard its actions, so each handler checks that the visitor may act on the arguments it receives, such as the owner of an order whose id it takes.",
                ja: "`A` という型は守られますが、その値は守られません。訪問者のブラウザは、ページの式が何を計算するかにかかわらず `A` の任意の値を送れますし、ページを読み込まずにアクションの URL を呼ぶことは誰にでもできます。ページを守るハンドラはそのアクションを守らないので、ハンドラはそれぞれ、受け取った引数に対して訪問者が操作してよいかを確かめます。たとえば、id を受け取った注文の持ち主かどうかです。",
            },
        ),
        note(
            site,
            L {
                en: "An extractor that answers an action with a redirect compiles, and the page then sees a failure with status 0 and the message `the answer was redirected to` followed by the URL, with no message from the server. Answer an action with a status, as `SignedOut` does.",
                ja: "アクションにリダイレクトで応答するエクストラクタもコンパイルできますが、ページにはステータス 0 の失敗と、`the answer was redirected to` に URL が続くメッセージが届き、サーバーからのメッセージはありません。`SignedOut` のように、アクションにはステータスで応答してください。",
            },
        ),
    ])
}

/// The links to the pages that come next.
fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "Next, [Errors and logging](learn/errors/) covers a handler that fails and what the page shows then.",
            ja: "次の [エラーとログ](learn/errors/) では失敗するハンドラと、そのときページに出るものを説明します。",
        },
    )
}
