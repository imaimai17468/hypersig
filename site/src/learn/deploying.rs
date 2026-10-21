//! The page on running an application as a server process and on Cloudflare
//! Workers.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, note, para, parts, title};

/// The worker of `examples/cloudflare`, which CI builds for
/// wasm32-unknown-unknown.
const WORKER: &str = include_str!("../../../examples/cloudflare/src/lib.rs");

/// The wrangler configuration of `examples/cloudflare`.
const WRANGLER: &str = include_str!("../../../examples/cloudflare/wrangler.toml");

/// The worker's source after its module header and the `#![cfg]` line that
/// leaves the crate empty on other targets, which a reader's own worker does
/// not need.
fn worker_source() -> &'static str {
    let source = code::without_header(WORKER);
    let (attribute, rest) = source
        .split_once("\n\n")
        .expect("the worker's first item follows a blank line");
    assert!(
        attribute.starts_with("#![cfg("),
        "the worker opens with its `#![cfg]` line"
    );
    rest
}

pub fn deploying(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Deploying).title(site)),
        lead(site, Doc::Learn(Learn::Deploying).leads()),
        heading(
            site,
            L {
                en: "Run it as a server process",
                ja: "サーバーのプロセスとして動かす",
            },
        ),
        para(
            site,
            L {
                en: "`cargo build --release` writes one binary. The browser runtime is compiled into it and `hypersig_runtime()` serves it, so no static file goes up beside the binary.",
                ja: "`cargo build --release` はバイナリを 1 つ書き出します。ブラウザのランタイムはバイナリに埋め込まれていて `hypersig_runtime()` が配信するので、バイナリのほかに置く静的ファイルはありません。",
            },
        ),
        para(
            site,
            L {
                en: "A stream holds its connection open for as long as it sends patches, so the host has to let a request run that long. A container host or a virtual machine does, and a platform that stops each function after a time limit closes the stream at that limit. State kept in memory, such as a counter in the router's state, belongs to one process, so an application running as several instances keeps it in a database instead.",
                ja: "ストリームはパッチを送るあいだ接続を開いたままにするので、ホストはその長さのリクエストを許す必要があります。コンテナのホストや仮想マシンは許しますが、関数ごとに実行時間の上限があるプラットフォームでは、上限に達したところでストリームが切れます。ルーターの状態に置いたカウンターのようにメモリに持つ状態は 1 つのプロセスのものなので、インスタンスを複数動かすアプリケーションはその状態をデータベースに置いてください。",
            },
        ),
        heading(
            site,
            L {
                en: "Run it on Cloudflare Workers",
                ja: "Cloudflare Workers で動かす",
            },
        ),
        para(
            site,
            L {
                en: "Workers run Rust compiled to `wasm32-unknown-unknown`. The `worker` crate's `axum` feature hands each request to the axum `Router`, so pages, actions, and streams are written as for a server. The crate is a library that `worker-build` compiles, and axum goes without its default features, which need tokio's networking.",
                ja: "Workers は `wasm32-unknown-unknown` 向けにコンパイルした Rust を動かします。`worker` クレートの `axum` 機能が受けたリクエストを axum の `Router` に渡すので、ページ、アクション、ストリームはサーバーと同じように書けます。クレートは `worker-build` がコンパイルするライブラリにし、axum は tokio のネットワーク機能を使う既定の機能を外して使います。",
            },
        ),
        note(
            site,
            L {
                en: "hypersig builds for `wasm32-unknown-unknown` from the release after 0.1.1. Until then, depend on the repository with `hypersig = { git = \"https://github.com/imaimai17468/hypersig\" }`.",
                ja: "hypersig が `wasm32-unknown-unknown` 向けにビルドできるのは 0.1.1 より後のリリースからです。それまでは `hypersig = { git = \"https://github.com/imaimai17468/hypersig\" }` でリポジトリに依存してください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[lib]
crate-type = ["cdylib"]

[dependencies]
axum = { version = "0.8", default-features = false }
futures = "0.3"
hypersig = "0.1"
serde = { version = "1", features = ["derive"] }
tower-service = "0.3"
worker = { version = "0.8", features = ["http", "axum"] }"#,
        ),
        para(
            site,
            L {
                en: "The worker below is the [`cloudflare`](https://github.com/imaimai17468/hypersig/tree/main/examples/cloudflare) example in the repository, which the end-to-end tests run under `wrangler dev`. The `+1` button counts in the browser, the second button doubles the count on the worker, and a stream sends the worker's time every second.",
                ja: "下の Worker は、リポジトリにある例 [`cloudflare`](https://github.com/imaimai17468/hypersig/tree/main/examples/cloudflare) です。E2E テストはこの例を `wrangler dev` で動かします。`+1` のボタンはブラウザで数を増やし、もう 1 つのボタンは Worker で数を 2 倍にし、ストリームは Worker の時刻を 1 秒ごとに送ります。",
            },
        ),
        code::block(Lang::Rust, Some("src/lib.rs"), worker_source()),
        code::block(Lang::Toml, Some("wrangler.toml"), WRANGLER),
        para(
            site,
            L {
                en: "Install the target and `worker-build` once. `wrangler dev` builds the worker and serves it at `http://localhost:8787`, and `wrangler deploy` publishes it to your Cloudflare account.",
                ja: "target と `worker-build` は最初に 1 回だけ入れます。`wrangler dev` は Worker をビルドして `http://localhost:8787` で配信し、`wrangler deploy` は Cloudflare のアカウントに公開します。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "rustup target add wasm32-unknown-unknown
cargo install --locked worker-build@0.8.7
npx wrangler dev
npx wrangler deploy",
        ),
        heading(
            site,
            L {
                en: "What changes on Workers",
                ja: "Workers で変わること",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Requests may run in different isolates, so a value kept in memory may be gone at the next request. Keep the state requests share in KV, D1, or a Durable Object.",
                    ja: "リクエストごとに別の isolate で動くことがあるので、メモリに置いた値は次のリクエストで消えていることがあります。リクエストをまたいで共有する状態は KV、D1、Durable Object に置いてください。",
                },
                L {
                    en: "A stream sends no keep-alive comments, because axum times them with tokio, which has no timer on this target. A stream that waits between patches waits with `worker::Delay`, wrapped in `SendFuture` because a JavaScript timer is not `Send`.",
                    ja: "axum は keep-alive のコメントを tokio のタイマーで送り、この target には tokio のタイマーがないので、ストリームはこのコメントを送りません。パッチの合間に待つストリームは `worker::Delay` で待ちます。JavaScript のタイマーは `Send` ではないので、`SendFuture` で包みます。",
                },
                L {
                    en: "`hypersig_runtime()` sends the runtime uncompressed. The Workers runtime compresses a body that names a `Content-Encoding` again, so the file compressed with Brotli at build time would reach the browser compressed twice.",
                    ja: "`hypersig_runtime()` はランタイムを圧縮せずに送ります。Workers の実行環境は `Content-Encoding` を付けた本文をもう一度圧縮するので、ビルド時に Brotli で圧縮したファイルを送ると、ブラウザには 2 回圧縮されたファイルが届きます。",
                },
                L {
                    en: "`Timestamp::now` reads the clock with JavaScript's `Date.now()`, because `SystemTime::now` panics on this target.",
                    ja: "`Timestamp::now` は JavaScript の `Date.now()` で時刻を読みます。この target では `SystemTime::now` がパニックするためです。",
                },
            ],
        ),
    ])
}
