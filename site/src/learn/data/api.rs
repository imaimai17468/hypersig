//! The page on a handler that calls another service's HTTP API: the client
//! in the router's state, the service's refusals and failures, updates from
//! the service, and the call on Cloudflare Workers.

use super::next;
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Api).title(site)),
        lead(site, Doc::Learn(Learn::Api).leads()),
        client(site),
        failures(site),
        changes(site),
        compiler(site),
        workers(site),
        next(
            site,
            &[
                L {
                    en: "[Busy and failed calls](learn/call-status/) shows the signals a call sets while it waits and after it fails.",
                    ja: "[呼び出し中と失敗](learn/call-status/) では呼び出しが応答を待つあいだと失敗したあとに設定するシグナルを説明します。",
                },
                L {
                    en: "[Streams](learn/streams/) covers the stream that sends an open tab the changes a webhook reports.",
                    ja: "[ストリーム](learn/streams/) では webhook が知らせた変更を開いているタブに送るストリームを説明します。",
                },
                L {
                    en: "[Signing in](learn/sessions/) keeps who the visitor is in a session, for a service that answers per member.",
                    ja: "[サインインとセッション](learn/sessions/) ではメンバーごとに答えるサービスのために、訪問者が誰かをセッションに持ちます。",
                },
            ],
        ),
    ])
}

/// The call from the handler, with the client in the router's state.
fn client(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The client in the router's state",
                ja: "ルーターの状態に置くクライアント",
            },
        ),
        para(
            site,
            L {
                en: "The data may belong to another service: a backend another team runs, a SaaS, or a database that answers over HTTP. The browser still calls only this server's actions, and the action's handler calls the service and answers with a `Page` or `Patches` built from the reply. The service's address and its token are read from the environment on the server, and no page holds them.",
                ja: "データがほかのサービスにあることもあります。ほかのチームが運用するバックエンド、SaaS、HTTP で答えるデータベースなどです。その場合もブラウザが呼ぶのはこのサーバーのアクションだけで、アクションのハンドラがサービスを呼び、その返事から作った `Page` か `Patches` で応答します。サービスのアドレスとトークンはサーバーで環境変数から読むので、ページには含まれません。",
            },
        ),
        para(
            site,
            L {
                en: "The samples use [reqwest](https://docs.rs/reqwest/0.13) with its `json` feature, which adds `RequestBuilder::json` and `Response::json`:",
                ja: "サンプルは [reqwest](https://docs.rs/reqwest/0.13) を、`RequestBuilder::json` と `Response::json` を加える `json` の feature 付きで使います。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
reqwest = { version = "0.13", default-features = false, features = ["json", "rustls"] }"#,
        ),
        show(Snippet::Api),
        para(
            site,
            L {
                en: "A `reqwest::Client` keeps a pool of connections, so build one when the server starts and keep it in the state, in place of one per request. The service's JSON deserializes into a struct with serde, so a reply of another shape fails at `json()` and goes to the handler's error. `error_for_status` turns a 4xx or 5xx from the service into an error the same way.",
                ja: "`reqwest::Client` は接続のプールを持つので、リクエストごとに作らず、サーバーの起動時に 1 つ作って状態に置きます。サービスの JSON は serde で構造体に変換するので、形の違う返事は `json()` で失敗し、ハンドラのエラーになります。`error_for_status` も同じように、サービスが返した 4xx や 5xx をエラーにします。",
            },
        ),
    ])
}

/// What the visitor reads when the service refuses or fails.
fn failures(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "When the service refuses or fails",
                ja: "サービスが拒否したとき、失敗したとき",
            },
        ),
        para(
            site,
            L {
                en: "Tell two kinds of failure apart. A refusal of the visitor's input, such as a 422 with a message, is something the visitor can fix, so answer with that status and the message. Every other failure, such as a service that does not answer, a 5xx, a 401 from a wrong token, or a body that does not parse, is one the visitor can only retry, so log its detail on the server and answer 502 with a message to try again. The [`api`](https://github.com/imaimai17468/hypersig/tree/main/examples/api) example has an `ApiError` with one variant for each, and the sample below takes it from there:",
                ja: "失敗は 2 種類に分けて扱います。422 とメッセージのように訪問者の入力を拒否したものは訪問者が直せるので、そのステータスとメッセージで応答します。サービスが答えない、5xx を返す、トークンの誤りで 401 を返す、本文が変換できない、といったほかの失敗は、訪問者にはやり直すことしかできません。詳細はサーバーのログに出し、502 とやり直しを促すメッセージで応答してください。[`api`](https://github.com/imaimai17468/hypersig/tree/main/examples/api) の例はそれぞれに対応するバリアントを持つ `ApiError` を使います。下のサンプルはそこから取ったものです。",
            },
        ),
        show(Snippet::ApiRefusal),
        para(
            site,
            L {
                en: "Submitting the form calls `ADD` with the draft, and the handler passes it to the service. When the service refuses the note, the handler answers the call with the service's message as plain text and the status 422:",
                ja: "フォームを送信すると下書きを引数に `ADD` が呼ばれ、ハンドラはそれをサービスに渡します。サービスがメモを拒否すると、ハンドラはサービスのメッセージをプレーンテキストにし、ステータス 422 で呼び出しに応答します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "HTTP/1.1 422 Unprocessable Entity
content-type: text/plain; charset=utf-8

A note holds 1 to 80 characters.",
        ),
        subheading(
            site,
            L {
                en: "The signal that receives the message",
                ja: "メッセージを受け取るシグナル",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::error`](api:hypersig/struct.Call.html#method.error) is `fn error(self, message: Sig<S, String>) -> Self`. It sets `message` to `\"\"` when the call is sent, and to a text when the call fails. The text depends on what came back:",
                ja: "[`Call::error`](api:hypersig/struct.Call.html#method.error) は `fn error(self, message: Sig<S, String>) -> Self` です。呼び出しを送るときに `message` を `\"\"` にし、呼び出しが失敗するとそこに文を書きます。文は返ってきたものによって決まります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["What came back", "The text in `message`"],
                ja: &["返ってきたもの", "`message` に入る文"],
            },
            L {
                en: &[
                    &[
                        "A status outside 2xx with a `text/plain` body, as axum sends for a `String`",
                        "The body",
                    ],
                    &[
                        "A status outside 2xx with an empty body or one of another type, such as a proxy's HTML error page",
                        "The status and its reason, such as `502 Bad Gateway`",
                    ],
                    &["No answer", "The browser's message"],
                ],
                ja: &[
                    &[
                        "2xx 以外のステータスと `text/plain` の本文（axum が `String` に対して送るもの）",
                        "本文",
                    ],
                    &[
                        "2xx 以外のステータスと、空の本文かほかの種類の本文（プロキシの HTML のエラーページなど）",
                        "ステータスとその理由（`502 Bad Gateway` など）",
                    ],
                    &["応答がない", "ブラウザのメッセージ"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The status also goes to the console. The signal holds a `String`, so a signal of another type fails to compile with E0308:",
                ja: "ステータスはコンソールにも出ます。シグナルは `String` を持つ型なので、ほかの型のシグナルを渡すと E0308 でコンパイルに失敗します。",
            },
        ),
        show(Snippet::WrongErrorSignal),
        para(
            site,
            L {
                en: "Which status the handler picks is not checked. A handler that answered the refusal with a 502 would show the same sentence, and a page that tells the two apart through [`Call::status`](api:hypersig/struct.Call.html#method.status) would read the wrong one.",
                ja: "ハンドラがどのステータスを選ぶかは検査されません。拒否に 502 で応答しても同じ文が表示されますが、[`Call::status`](api:hypersig/struct.Call.html#method.status) で 2 つを見分けるページは、違うほうとして読みます。",
            },
        ),
        note(
            site,
            L {
                en: "The `api` example comes with a stand-in for the service, `cargo run -p example-api --bin notes-backend`, which keeps notes behind a bearer token and refuses a note longer than 80 characters with a 422. Its end-to-end test checks that the refusal's message reaches the page and that the draft stays.",
                ja: "`api` の例にはサービスの代役 `cargo run -p example-api --bin notes-backend` が付いています。代役は bearer トークンの裏でメモを保持し、80 文字を超えるメモは 422 で拒否します。E2E テストは、拒否のメッセージがページに表示され、入力した下書きが保持されることを確かめます。",
            },
        ),
    ])
}

/// Changes the service makes, through a webhook or by asking again.
fn changes(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Changes the service makes",
                ja: "サービス側で起きた変更",
            },
        ),
        para(
            site,
            L {
                en: "An open tab learns of a change only through a stream of this server. Where the service can call a webhook, serve one as an axum route that sends on a `broadcast` channel in the state, as `examples/todo` does after each action, and the streams send the item that changed. Where it cannot, a stream can ask the service again after each `tokio::time::sleep`. The channel reaches only the streams of the process the webhook reached, so with more than one process, carry the change to each of them as [Updates with more than one process](learn/database/#updates-with-more-than-one-process) does.",
                ja: "開いているタブが変更を知るのは、このサーバーのストリームを通したときだけです。サービスが webhook を呼べるなら、状態の `broadcast` のチャネルに送る axum のルートとして webhook を用意します。`examples/todo` がアクションのたびにするのと同じで、ストリームが変わった項目を送ります。webhook がなければ、ストリームが `tokio::time::sleep` で待つたびにサービスに問い合わせ直します。このチャネルが届くのは、webhook を受けたプロセスのストリームだけです。プロセスが複数あるときは、[プロセスが複数あるときの更新](learn/database/#updates-with-more-than-one-process) と同じように、変更をそれぞれのプロセスに届けてください。",
            },
        ),
    ])
}

/// What the compiler checks of a call to the service.
fn compiler(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査するもの",
            },
        ),
        para(
            site,
            L {
                en: "The service's paths and the shape of its JSON are outside what hypersig checks, as a plain SQL string is: a wrong path or field fails when the call runs. Where the service publishes an OpenAPI description, a client generated from it, such as one from the `progenitor` crate, turns them into Rust types.",
                ja: "サービスのパスと JSON の形は、単なる文字列で書いた SQL と同じく、hypersig が検査する範囲の外です。パスやフィールドを間違えると、呼び出しを実行したときに失敗します。サービスが OpenAPI の定義を公開していれば、そこから生成したクライアント（たとえば `progenitor` クレートで生成したもの）で、それらを Rust の型にできます。",
            },
        ),
    ])
}

/// The call on Cloudflare Workers.
fn workers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "On Cloudflare Workers",
                ja: "Cloudflare Workers では",
            },
        ),
        para(
            site,
            L {
                en: "A worker calls a service with the `worker` crate's `Fetch`, or calls another worker through a service binding that `env.service` returns, in place of reqwest. Its future is not `Send`, so wrap it in `SendFuture`, as the [Cloudflare template](learn/deploying/cloudflare/) does with its D1 calls.",
                ja: "Worker は reqwest の代わりに、`worker` クレートの `Fetch` でサービスを呼ぶか、`env.service` が返すサービスバインディングでほかの Worker を呼びます。その future は `Send` ではないので、[Cloudflare のテンプレート](learn/deploying/cloudflare/) が D1 の呼び出しにするのと同じように `SendFuture` で包みます。",
            },
        ),
    ])
}
