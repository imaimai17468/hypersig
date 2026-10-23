//! The page on a handler that calls another service's HTTP API: the client
//! in the router's state, the service's refusals and failures, updates from
//! the service, and the call on Cloudflare Workers.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, title};

pub fn api(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Api).title(site)),
        lead(site, Doc::Learn(Learn::Api).leads()),
        client(site),
        failures(site),
        elsewhere(site),
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
        show(Snippet::Api),
        para(
            site,
            L {
                en: "The sample uses [reqwest](https://docs.rs/reqwest/0.13). A `reqwest::Client` keeps a pool of connections, so build one when the server starts and keep it in the state, in place of one per request. The service's JSON deserializes into a struct with serde, so a reply of another shape fails at `json()` and goes to the handler's error.",
                ja: "サンプルは [reqwest](https://docs.rs/reqwest/0.13) を使います。`reqwest::Client` は接続のプールを持つので、リクエストごとに作らず、サーバーの起動時に 1 つ作って状態に置きます。サービスの JSON は serde で構造体に変換するので、形の違う返事は `json()` で失敗し、ハンドラのエラーになります。",
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
                en: "Tell two kinds of failure apart. A refusal of the visitor's input, such as a 422 with a message, is something the visitor can fix, so answer with that status and the message, and the runtime writes it to the signal given to `Call::error`. Every other failure, such as a service that does not answer, a 5xx, a 401 from a wrong token, or a body that does not parse, is one the visitor can only retry, so log its detail on the server and answer 502 with a message to try again. The [`api`](https://github.com/imaimai17468/hypersig/tree/main/examples/api) example has an `ApiError` with one variant for each.",
                ja: "失敗は 2 種類に分けて扱います。422 とメッセージのように訪問者の入力を拒否したものは訪問者が直せるので、そのステータスとメッセージで応答します。ランタイムはその本文を `Call::error` に渡したシグナルに書きます。サービスが答えない、5xx を返す、トークンの誤りで 401 を返す、本文が変換できない、といったほかの失敗は、訪問者にはやり直すことしかできません。詳細はサーバーのログに出し、502 とやり直しを促すメッセージで応答してください。[`api`](https://github.com/imaimai17468/hypersig/tree/main/examples/api) の例は、それぞれに対応するバリアントを持つ `ApiError` を使います。",
            },
        ),
        note(
            site,
            L {
                en: "The `api` example comes with a stand-in for the service, `cargo run -p example-api --bin notes-backend`, which keeps notes behind a bearer token and refuses a note longer than 80 characters with a 422. Its end-to-end test checks that the refusal's message reaches the page and that the draft stays.",
                ja: "`api` の例にはサービスの代役 `cargo run -p example-api --bin notes-backend` が付いています。代役は bearer トークンの裏でメモを保持し、80 文字を超えるメモは 422 で拒否します。E2E テストは、拒否のメッセージがページに表示され、入力した下書きが残ることを確かめます。",
            },
        ),
    ])
}

/// Changes the service makes, the compiler's reach, and the call on Workers.
fn elsewhere(site: Site) -> Part {
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
                en: "An open tab learns of a change only through a stream of this server. Where the service can call a webhook, serve one as an axum route that sends on a `broadcast` channel in the state, as `examples/todo` does after each action, and the streams send the new list. Where it cannot, a stream can ask the service again after each `tokio::time::sleep`.",
                ja: "開いているタブが変更を知るのは、このサーバーのストリームを通したときだけです。サービスが webhook を呼べるなら、状態の `broadcast` のチャネルに送る axum のルートとして webhook を用意します。`examples/todo` がアクションのたびにするのと同じで、ストリームが新しい一覧を送ります。webhook がなければ、ストリームが `tokio::time::sleep` で待つたびにサービスに問い合わせ直します。",
            },
        ),
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
                en: "The service's paths and the shape of its JSON are outside what hypersig checks, as SQL is: a wrong path or field fails when the call runs. Where the service publishes an OpenAPI description, a client generated from it, such as one from the `progenitor` crate, turns them into Rust types.",
                ja: "サービスのパスと JSON の形は、SQL と同じく hypersig が検査する範囲の外です。パスやフィールドを間違えると、呼び出しを実行したときに失敗します。サービスが OpenAPI の定義を公開していれば、そこから生成したクライアント（たとえば `progenitor` クレートで生成したもの）で、それらを Rust の型にできます。",
            },
        ),
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
