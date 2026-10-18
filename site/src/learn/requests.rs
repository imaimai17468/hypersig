//! The page on what a handler reads from the request and writes beside its
//! answer: extractors, headers such as cookies, errors, and the requests an
//! action refuses.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, subheading, title};

pub fn requests(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Requests).title(site)),
        lead(
            site,
            L {
                en: "A handler reads the request through an axum extractor, sets cookies by returning headers beside its answer, and fails with `?`.",
                ja: "ハンドラは axum の extractor でリクエストを読み、答えと組にしたヘッダーで Cookie を設定し、`?` で失敗を返します。",
            },
        ),
        heading(
            site,
            L {
                en: "Extractors and headers",
                ja: "extractor とヘッダー",
            },
        ),
        para(
            site,
            L {
                en: "A page or action handler takes the state and its arguments, and may take one [`FromRequestParts`](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html) extractor between them: `HeaderMap`, a tuple of extractors, or the application's own type, such as the signed-in member read from a session cookie. When the extractor fails, its rejection is the response and the handler does not run. A pair of headers and a page or patches, such as `([(SET_COOKIE, value)], patches)`, sends the headers with the answer.",
                ja: "ページとアクションのハンドラは状態と引数を受け取り、そのあいだに [`FromRequestParts`](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html) の extractor を 1 つ受け取れます。`HeaderMap`、extractor のタプル、セッションの Cookie から読んだサインイン中のメンバーのようなアプリケーションの型が使えます。extractor が失敗すると、その rejection が応答になり、ハンドラは動きません。`([(SET_COOKIE, value)], patches)` のようにヘッダーとページや差分を組にすると、ヘッダーが答えと一緒に送られます。",
            },
        ),
        show(Snippet::Extractors),
        para(
            site,
            L {
                en: "An extractor that guards actions answers a signed-out request with a status such as 401. The browser's `fetch` follows a redirect, so an action answered with one receives the sign-in page's HTML and the page shows nothing. The `Accept` header tells the two apart: the runtime asks for `text/event-stream`.",
                ja: "アクションを守る extractor は、サインインしていないリクエストに 401 などの状態で答えます。ブラウザの `fetch` はリダイレクトをたどるので、リダイレクトで答えたアクションはサインインのページの HTML を受け取り、ページには何も表示されません。2 つは `Accept` ヘッダーで見分けられます。ランタイムは `text/event-stream` を求めます。",
            },
        ),
        heading(
            site,
            L {
                en: "Errors",
                ja: "エラー",
            },
        ),
        para(
            site,
            L {
                en: "A handler may return `Result<_, E>` where `E` implements `IntoResponse`, so `?` works. An action whose response has a status outside 2xx fails on the page: the runtime writes the body to the signal given to `Call::error`, so the error's response text is what the visitor reads. An empty body writes the status and its reason, such as `502 Bad Gateway`.",
                ja: "ハンドラは、`E` が `IntoResponse` を実装する `Result<_, E>` を返せるので、`?` が使えます。応答の状態が 2xx 以外のアクションは、ページでは失敗になります。ランタイムは、`Call::error` に渡したシグナルに応答の本文を書くので、エラーの応答の文言がそのまま訪問者に見えます。本文が空のときは、`502 Bad Gateway` のように状態とその理由を書きます。",
            },
        ),
        show(Snippet::HandlerErrors),
        heading(
            site,
            L {
                en: "Requests an action refuses",
                ja: "アクションが断るリクエスト",
            },
        ),
        para(
            site,
            L {
                en: "Before its handler runs, an action answers a request whose `Sec-Fetch-Site` is not `same-origin` with a 403, and a POST whose body is not `application/json` with a 415. A form on another site cannot send JSON, and a script there that tries needs a preflight request this server does not answer, so neither can call an action with the visitor's cookie.",
                ja: "アクションは、ハンドラが動く前に、`Sec-Fetch-Site` が `same-origin` でないリクエストに 403 を、本文が `application/json` でない POST に 415 を返します。別のサイトのフォームは JSON を送れず、そこでスクリプトが送ろうとするとプリフライトのリクエストが要り、このサーバーはそれに答えません。どちらも訪問者の Cookie を付けてアクションを呼び出せません。",
            },
        ),
        note(
            site,
            L {
                en: "A browser that sends no `Sec-Fetch-Site` still reaches a GET action from a link on another site, so a GET action should change nothing. The `auth` example in the repository signs in with a cookie session end to end.",
                ja: "`Sec-Fetch-Site` を送らないブラウザからは、別のサイトのリンクで GET のアクションに届くので、GET のアクションでは何も変更しないでください。リポジトリの `auth` の例は、Cookie のセッションでのサインインを最初から最後まで示します。",
            },
        ),
        uploads(site),
    ])
}

/// The section on uploads, apart from `requests` because clippy rejects an
/// array of parts larger than 16 KiB on the stack.
fn uploads(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Uploads",
                ja: "ファイルのアップロード",
            },
        ),
        para(
            site,
            L {
                en: "An [`Upload<S, A>`](api:hypersig/struct.Upload.html) is an action that also receives the files picked in a [`FileInput<S>`](api:hypersig/struct.FileInput.html). Put `FileInput::input` on the page and pass the same constant to `call` or `call_with`. `RouterExt::upload` serves it, and its handler receives an `Uploaded<A>` with the arguments and each file's name, media type, and bytes. It answers as an action's handler does, so it can return a `Result` and take an extractor.",
                ja: "[`Upload<S, A>`](api:hypersig/struct.Upload.html) は、[`FileInput<S>`](api:hypersig/struct.FileInput.html) で選んだファイルも受け取るアクションです。ページに `FileInput::input` を置き、同じ定数を `call` か `call_with` に渡します。`RouterExt::upload` で配信し、そのハンドラは引数と、ファイルごとの名前、メディアタイプ、中身を持つ `Uploaded<A>` を受け取ります。応答はアクションのハンドラと同じなので、`Result` を返したり extractor を受け取ったりできます。",
            },
        ),
        show(Snippet::Uploads),
        para(
            site,
            L {
                en: "A form on another site can send a `multipart/form-data` body without a preflight, so the runtime adds the `hypersig-upload` header, which no form can set, and the router refuses an upload without it with a 403. An ordinary action still refuses a multipart body with a 415. A body longer than `Upload::DEFAULT_MAX_BYTES` (8 MiB), or the limit `max_bytes` sets, gets a 413, before it is read when its `Content-Length` says so.",
                ja: "`multipart/form-data` の本文は、別のサイトのフォームからプリフライトなしで送れます。そこでランタイムはフォームでは付けられない `hypersig-upload` ヘッダーを付け、ルーターはそれがないアップロードを 403 で断ります。普通のアクションは、これまでどおりマルチパートの本文を 415 で断ります。`Upload::DEFAULT_MAX_BYTES`（8 MiB）か、`max_bytes` で決めた上限を超える本文には 413 を返します。`Content-Length` が上限を超えていれば、本文を読む前に返します。",
            },
        ),
        note(
            site,
            L {
                en: "The file name and the media type come from the visitor's browser, so check the bytes before trusting them. The `auth` example accepts a picture only when its first bytes are those of a PNG or a JPEG.",
                ja: "ファイル名とメディアタイプは訪問者のブラウザが決めるので、信用する前に中身を検査してください。`auth` の例は、先頭のバイトが PNG か JPEG のものである画像だけを受け付けます。",
            },
        ),
        subheading(
            site,
            L {
                en: "Large files",
                ja: "大きなファイル",
            },
        ),
        para(
            site,
            L {
                en: "`RouterExt::upload` reads every file into memory before its handler runs, so keep its limit near the largest file you accept. For larger files, serve the same `Upload` with `RouterExt::upload_stream`. Its handler receives an [`UploadStream<A>`](api:hypersig/struct.UploadStream.html): the arguments, already read because the runtime sends them first, and `next_file`, which yields each file for the handler to read with `chunk` as its bytes arrive. Once the body passes the limit, `next_file` or `chunk` returns a `RequestError` with status 413, which the handler can return with `?`. A body whose first part is not the arguments gets a 400.",
                ja: "`RouterExt::upload` は、ハンドラが動く前にすべてのファイルをメモリに読み込みます。そのため、上限は受け付ける最大のファイルに近い値にしてください。それより大きなファイルには、同じ `Upload` を `RouterExt::upload_stream` で配信します。ハンドラは [`UploadStream<A>`](api:hypersig/struct.UploadStream.html) を受け取ります。引数はランタイムが先頭で送るので読み終えてあり、`next_file` がファイルを 1 つずつ返します。ハンドラはファイルの中身を、届いた分ずつ `chunk` で読みます。本文が上限を超えると、`next_file` か `chunk` が状態 413 の `RequestError` を返すので、ハンドラは `?` でそのまま返せます。先頭の部分が引数でない本文には 400 を返します。",
            },
        ),
    ])
}
