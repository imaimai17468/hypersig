//! The page on a handler that fails: `Result` and `?`, what the page shows
//! for a failed call, panics, refusals in a log, and logging with `tracing`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, table_of, title};

/// The article.
pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Errors).title(site)),
        lead(site, Doc::Learn(Learn::Errors).leads()),
        returning(site),
        on_the_page(site),
        panics(site),
        refusals(site),
        logging(site),
        unchecked(site),
        next(site),
    ])
}

/// `Result<_, E>` with `E: IntoResponse`, and the compile error without it.
fn returning(site: Site) -> Part {
    parts([
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
                en: "A handler may return `Result<_, E>` where `E` implements axum's [`IntoResponse`](https://docs.rs/axum/0.8/axum/response/trait.IntoResponse.html), so `?` works. For an action, [`IntoActionResponse`](api:hypersig/trait.IntoActionResponse.html) is implemented for such a `Result` of patches, and for a page, [`PageResponse`](api:hypersig/struct.PageResponse.html) answers with the error in the same way. The error's response is the answer: below, `NotesFull` answers `409 Conflict` with a sentence the visitor reads.",
                ja: "ハンドラは、`E` が axum の [`IntoResponse`](https://docs.rs/axum/0.8/axum/response/trait.IntoResponse.html) を実装する `Result<_, E>` を返せるので、`?` が使えます。アクションでは差分のそうした `Result` に [`IntoActionResponse`](api:hypersig/trait.IntoActionResponse.html) が実装されていて、ページでは [`PageResponse`](api:hypersig/struct.PageResponse.html) が同じようにエラーで応答します。応答はエラーの応答そのものです。下の `NotesFull` は、訪問者が読む 1 文を付けて `409 Conflict` で応答します。",
            },
        ),
        show(Snippet::HandlerErrors),
        para(
            site,
            L {
                en: "An error type without `IntoResponse` gives the router no answer to send, so `routes!` does not compile:",
                ja: "`IntoResponse` のないエラーの型では、ルーターが送る応答がないので、`routes!` がコンパイルできません。",
            },
        ),
        show(Snippet::ErrorNotResponse),
    ])
}

/// What `Call::error` and `Call::status` hold for each kind of failure.
fn on_the_page(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the page shows",
                ja: "ページに出るもの",
            },
        ),
        para(
            site,
            L {
                en: "An action whose response has a status outside 2xx fails on the page. The runtime writes a message to the signal given to [`Call::error`](api:hypersig/struct.Call.html#method.error) and the status to the signal given to [`Call::status`](api:hypersig/struct.Call.html#method.status). Both are emptied, to `\"\"` and `0`, when the call is sent. The message depends on the answer:",
                ja: "応答のステータスが 2xx 以外のアクションは、ページでは失敗になります。ランタイムは [`Call::error`](api:hypersig/struct.Call.html#method.error) に渡したシグナルにメッセージを、[`Call::status`](api:hypersig/struct.Call.html#method.status) に渡したシグナルにステータスを書きます。どちらも呼び出しを送るときに `\"\"` と `0` に戻ります。メッセージは応答によって次のように決まります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Answer", "`error` signal", "`status` signal"],
                ja: &["応答", "`error` のシグナル", "`status` のシグナル"],
            },
            L {
                en: &[
                    &[
                        "Outside 2xx, with a `text/plain` body, which axum sends for a `String` or a `&str`",
                        "The body, such as `You can keep 3 notes.`",
                        "The status, such as `409`",
                    ],
                    &[
                        "Outside 2xx, with an empty body or another type, such as a proxy's HTML error page",
                        "The status and its reason, such as `502 Bad Gateway`",
                        "The status",
                    ],
                    &[
                        "2xx, but redirected, not an answer of the router, or closed before the router finished it",
                        "A message of the runtime's own",
                        "`0`",
                    ],
                    &[
                        "No answer, such as a dropped connection",
                        "The browser's message",
                        "`0`",
                    ],
                ],
                ja: &[
                    &[
                        "2xx 以外で、本文が `text/plain`（axum は `String` と `&str` をこの形式で送ります）",
                        "本文。たとえば `You can keep 3 notes.`",
                        "ステータス。たとえば `409`",
                    ],
                    &[
                        "2xx 以外で、本文が空か、プロキシの HTML のエラーページのようにほかの形式",
                        "ステータスとその理由。たとえば `502 Bad Gateway`",
                        "ステータス",
                    ],
                    &[
                        "2xx だが、リダイレクトされたか、ルーターの応答でないか、ルーターが応答を終える前に閉じた",
                        "ランタイム自身のメッセージ",
                        "`0`",
                    ],
                    &[
                        "応答がない。たとえば接続が切れた",
                        "ブラウザのメッセージ",
                        "`0`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each failure is also written to the browser's console. The call of the sample above that fails logs:",
                ja: "失敗はどれもブラウザのコンソールにも書かれます。上のサンプルの呼び出しが失敗すると、次の行が出ます。",
            },
        ),
        code::block(
            Lang::JavaScript,
            None,
            "hypersig: POST /notes failed with 409: You can keep 3 notes.",
        ),
        para(
            site,
            L {
                en: "[Busy and failed calls](learn/call-status/) shows these signals on the page and runs a statement on failure.",
                ja: "これらのシグナルをページに出す方法と、失敗したときに文を実行する方法は [呼び出し中と失敗](learn/call-status/) で説明します。",
            },
        ),
    ])
}

/// A handler that panics.
fn panics(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A handler that panics",
                ja: "パニックするハンドラ",
            },
        ),
        para(
            site,
            L {
                en: "A handler of an action, a stream, or an upload that panics before it returns answers `500 Internal Server Error`, which the page shows as any failed call, and the default panic hook still prints the message to stderr. A stream's stream is read after the response has begun, so a panic while it is read closes the connection before the answer is complete, and the page sees status 0.",
                ja: "アクション、ストリーム、アップロードのハンドラが値を返す前にパニックすると、`500 Internal Server Error` で応答します。ページはこれをほかの失敗した呼び出しと同じように表示し、既定のパニックフックはメッセージを標準エラー出力に書きます。ストリームのハンドラが返したストリームは応答が始まってから読まれるので、読んでいるあいだのパニックは応答が終わる前に接続を閉じ、ページにはステータス 0 が届きます。",
            },
        ),
    ])
}

/// `RequestError` as a value to log.
fn refusals(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Refusals in a log",
                ja: "拒否をログに書く",
            },
        ),
        para(
            site,
            L {
                en: "The router answers a request it refuses before the handler runs, listed in [Extractors and headers](learn/requests/#requests-an-action-refuses), with a [`RequestError`](api:hypersig/struct.RequestError.html). A streaming upload's handler receives one when the body goes wrong while it reads, and returns it with `?`. Its methods return more than the visitor sees, for a log:",
                ja: "ハンドラが動く前にルーターが拒否するリクエストには、[`RequestError`](api:hypersig/struct.RequestError.html) で応答します。拒否するリクエストの一覧は [エクストラクタとヘッダー](learn/requests/#requests-an-action-refuses) にあります。ストリーミングのアップロードのハンドラは、読んでいるあいだに本文がおかしくなるとこれを受け取り、`?` で返します。そのメソッドからは、訪問者に見せる本文より多くの情報をログのために取り出せます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it gives"],
                ja: &["項目", "内容"],
            },
            L {
                en: &[
                    &["`status()`", "The status of the response, such as `413`"],
                    &[
                        "`Display`",
                        "One fixed sentence per refusal, which is also the response's body",
                    ],
                    &[
                        "`source()`",
                        "What the parser said about a malformed body or arguments, which names the argument type's fields and quotes the visitor's input",
                    ],
                    &[
                        "`backtrace()`",
                        "The backtrace captured where the error was created, when `RUST_BACKTRACE` asks for one",
                    ],
                ],
                ja: &[
                    &["`status()`", "応答のステータス。たとえば `413`"],
                    &[
                        "`Display`",
                        "拒否ごとに決まった 1 文。応答の本文にもなります",
                    ],
                    &[
                        "`source()`",
                        "形の崩れた本文や引数についてのパーサーのメッセージ。引数の型のフィールド名と訪問者の入力を含みます",
                    ],
                    &[
                        "`backtrace()`",
                        "エラーを作った場所のバックトレース。`RUST_BACKTRACE` で求めたときに記録されます",
                    ],
                ],
            },
        ),
    ])
}

/// Logging with `tracing`, and a line per request from `TraceLayer`.
fn logging(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Logging",
                ja: "ログ",
            },
        ),
        para(
            site,
            L {
                en: "The samples on this site write the server's side of an error with `eprintln!` to stay short. An application logs with [`tracing`](https://docs.rs/tracing/0.1): a handler writes an event with `tracing::warn!` or `tracing::error!`, and `tracing-subscriber` writes the events to stderr. [`TraceLayer`](https://docs.rs/tower-http/0.7/tower_http/trace/struct.TraceLayer.html) from `tower-http`, put on the router with `layer`, adds a line for each request with the answer's status and how long it took. Add the three crates to `Cargo.toml`:",
                ja: "このサイトのサンプルは短く保つためにエラーのサーバー側の内容を `eprintln!` で書いています。アプリケーションでは [`tracing`](https://docs.rs/tracing/0.1) でログを書きます。ハンドラは `tracing::warn!` や `tracing::error!` でイベントを書き、`tracing-subscriber` がイベントを標準エラー出力に書きます。`tower-http` の [`TraceLayer`](https://docs.rs/tower-http/0.7/tower_http/trace/struct.TraceLayer.html) を `layer` でルーターに付けると、リクエストごとに、応答のステータスとかかった時間を書いた行が加わります。`Cargo.toml` に 3 つのクレートを追加してください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
tower-http = { version = "0.7", features = ["trace"] }
tracing = "0.1"
tracing-subscriber = "0.3""#,
        ),
        show(Snippet::Logging),
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
                en: "The compiler checks that an error has a response, and not what the response says. The `text/plain` body of a failed action is what the visitor reads, so write a sentence the visitor can act on there, and log the error's own message, which can hold a query or a file path, on the server.",
                ja: "コンパイラが検査するのは、エラーに応答があることで、応答の中身ではありません。失敗したアクションの `text/plain` の本文はそのまま訪問者に見えるので、本文には訪問者が次にどうすればよいかを書いてください。クエリやファイルのパスを含みうるエラー自身のメッセージは、サーバーのログに書きます。",
            },
        ),
        note(
            site,
            L {
                en: "A page handler that fails answers with what the error's `IntoResponse` writes, such as a line of plain text, and not with a page of the site. [Error pages](learn/error-pages/) shows a page for a 404 instead.",
                ja: "失敗したページのハンドラは、エラーの `IntoResponse` が書いたもの、たとえば 1 行のテキストで応答し、サイトのページでは応答しません。404 にページを出す方法は [エラーページ](learn/error-pages/) で説明します。",
            },
        ),
    ])
}

/// The links to the pages that come next.
fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "Next, [Uploads](learn/uploads/) sends files with a call.",
            ja: "次の [アップロード](learn/uploads/) では呼び出しでファイルを送ります。",
        },
    )
}
