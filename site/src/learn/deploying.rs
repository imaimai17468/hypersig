//! The page on deploying an application, in a container and on Cloudflare
//! Workers.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, title};

/// The worker of `examples/cloudflare`, which CI builds for
/// wasm32-unknown-unknown and the end-to-end tests run under `wrangler dev`.
const WORKER: &str = include_str!("../../../examples/cloudflare/src/lib.rs");

/// The worker's source after its module header and the `#![cfg]` line that
/// leaves the example empty on other targets, which a worker crate of its own
/// does not need.
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
        container(site),
        workers(site),
        publish(site),
    ])
}

/// The steps that build a container image.
fn container(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Deploy in a container",
                ja: "コンテナでデプロイする",
            },
        ),
        para(
            site,
            L {
                en: "These steps put the [quick start](learn/) application, named `my-app` by `cargo new`, into a container image that a container host such as Fly.io, Cloud Run, or Render runs. The browser runtime is compiled into the binary, so the image holds the binary alone.",
                ja: "この手順では、`cargo new` で `my-app` という名前で作った [クイックスタート](learn/) のアプリケーションを、コンテナのイメージにします。このイメージは Fly.io、Cloud Run、Render のようなコンテナのホストで動きます。ブラウザのランタイムはバイナリに埋め込まれているので、イメージに入れるのはバイナリだけです。",
            },
        ),
        subheading(
            site,
            L {
                en: "1. Listen on the port the host names",
                ja: "1. ホストが指定するポートで待ち受ける",
            },
        ),
        para(
            site,
            L {
                en: "The quick start listens on `127.0.0.1:3000`, which nothing outside the container reaches. Replace the two lines in `main` that bind and serve with these, which read the port from `PORT` and listen on every interface:",
                ja: "クイックスタートは `127.0.0.1:3000` で待ち受けるので、コンテナの外からは届きません。`main` の中で待ち受けて配信する 2 行を次のように書き換え、`PORT` からポートを読んで、すべてのインターフェースで待ち受けるようにします。",
            },
        ),
        show(Snippet::ServePort),
        subheading(
            site,
            L {
                en: "2. Write the Dockerfile",
                ja: "2. Dockerfile を書く",
            },
        ),
        para(
            site,
            L {
                en: "Put these two files at the root of the project. The first stage builds the binary and the second copies it into a small image. Replace `my-app` with the `name` in `Cargo.toml`. `.dockerignore` keeps the local `target` directory out of the build.",
                ja: "プロジェクトのルートに次の 2 つのファイルを置きます。1 つ目のステージでバイナリをビルドし、2 つ目のステージで小さなイメージにコピーします。`my-app` は `Cargo.toml` の `name` に置き換えてください。`.dockerignore` は手元の `target` ディレクトリをビルドに含めないためのものです。",
            },
        ),
        code::block(
            Lang::Shell,
            Some("Dockerfile"),
            r#"FROM rust:1.99-slim-trixie AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:trixie-slim
COPY --from=build /app/target/release/my-app /usr/local/bin/my-app
ENV PORT=3000
EXPOSE 3000
CMD ["my-app"]"#,
        ),
        code::block(Lang::Shell, Some(".dockerignore"), "target"),
        subheading(
            site,
            L {
                en: "3. Build and run the image",
                ja: "3. イメージをビルドして動かす",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "docker build -t my-app .
docker run --rm -p 3000:3000 my-app",
        ),
        para(
            site,
            L {
                en: "Open `http://localhost:3000`. A host that builds from a Dockerfile takes the project as it is now, and a host that runs an image takes the one built here, pushed to its registry.",
                ja: "`http://localhost:3000` を開いてください。Dockerfile からビルドするホストにはこのままのプロジェクトを渡し、イメージを動かすホストにはここでビルドしたイメージをそのホストのレジストリに push して渡します。",
            },
        ),
        subheading(
            site,
            L {
                en: "What the host has to allow",
                ja: "ホストに必要なこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A stream holds its request open for as long as it sends patches. A host that ends a request after a time limit, as Cloud Run does, closes the stream at that limit, so raise the limit, and a call with `reconnect()` opens the stream again.",
                    ja: "ストリームはパッチを送るあいだリクエストを開いたままにします。Cloud Run のようにリクエストを一定の時間で終えるホストでは、その時間でストリームが切れるので、上限を延ばしてください。`reconnect()` を付けた呼び出しはストリームをつなぎ直します。",
                },
                L {
                    en: "State kept in memory, such as a counter in the router's state, belongs to one process. When the host runs more than one instance, keep that state in a database.",
                    ja: "ルーターの状態に置いたカウンターのようにメモリに持つ状態は、1 つのプロセスのものです。ホストがインスタンスを複数動かすときは、その状態をデータベースに置いてください。",
                },
            ],
        ),
    ])
}

/// The steps that publish a worker on Cloudflare Workers.
fn workers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Deploy on Cloudflare Workers",
                ja: "Cloudflare Workers にデプロイする",
            },
        ),
        para(
            site,
            L {
                en: "Workers run Rust compiled to `wasm32-unknown-unknown`. The `worker` crate's `axum` feature hands each request to the axum `Router`, so pages, actions, and streams are written as for a server. These steps need a Cloudflare account, Rust 1.99 or later, and Node.js 22 or later, which `wrangler`, Cloudflare's command-line tool, runs on.",
                ja: "Workers は `wasm32-unknown-unknown` 向けにコンパイルした Rust を動かします。`worker` クレートの `axum` 機能が受けたリクエストを axum の `Router` に渡すので、ページ、アクション、ストリームはサーバーと同じように書けます。この手順には、Cloudflare のアカウント、Rust 1.99 以降、Node.js 22 以降が要ります。Node.js は Cloudflare のコマンドラインツール `wrangler` が使います。",
            },
        ),
        subheading(
            site,
            L {
                en: "1. Install the target and worker-build",
                ja: "1. target と worker-build を入れる",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "rustup target add wasm32-unknown-unknown
cargo install --locked worker-build@0.8.7",
        ),
        subheading(
            site,
            L {
                en: "2. Create the project",
                ja: "2. プロジェクトを作る",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo new --lib my-worker
cd my-worker",
        ),
        para(
            site,
            L {
                en: "Replace `Cargo.toml` with the file below. A worker is a library that `worker-build` compiles to wasm, and axum goes without its default features, which need tokio's networking.",
                ja: "`Cargo.toml` を下のファイルに置き換えます。Worker は `worker-build` が wasm にコンパイルするライブラリで、axum は tokio のネットワーク機能を使う既定の機能を外して使います。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[package]
name = "my-worker"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
axum = { version = "0.8", default-features = false }
futures = "0.3"
hypersig = "0.1.2"
serde = { version = "1", features = ["derive"] }
tower-service = "0.3"
worker = { version = "0.8", features = ["http", "axum"] }"#,
        ),
        subheading(
            site,
            L {
                en: "3. Write the worker",
                ja: "3. Worker を書く",
            },
        ),
        para(
            site,
            L {
                en: "Replace `src/lib.rs` with the worker below, the [`cloudflare`](https://github.com/imaimai17468/hypersig/tree/main/examples/cloudflare) example in the repository. The `+1` button counts in the browser, the second button doubles the count on the worker, and a stream sends the worker's time every second.",
                ja: "`src/lib.rs` を下の Worker に置き換えます。これはリポジトリにある例 [`cloudflare`](https://github.com/imaimai17468/hypersig/tree/main/examples/cloudflare) です。`+1` のボタンはブラウザで数を増やし、もう 1 つのボタンは Worker で数を 2 倍にし、ストリームは Worker の時刻を 1 秒ごとに送ります。",
            },
        ),
        code::block(Lang::Rust, Some("src/lib.rs"), worker_source()),
        subheading(
            site,
            L {
                en: "4. Configure wrangler",
                ja: "4. wrangler を設定する",
            },
        ),
        para(
            site,
            L {
                en: "Put `wrangler.toml` at the root of the project. `name` becomes the worker's name in your account and the first part of its address, and `wrangler` runs the `build` command before it serves or publishes.",
                ja: "プロジェクトのルートに `wrangler.toml` を置きます。`name` はアカウントの中での Worker の名前になり、アドレスの先頭にも使われます。`wrangler` は配信や公開の前に `build` のコマンドを実行します。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("wrangler.toml"),
            r#"name = "my-worker"
main = "build/worker/shim.mjs"
compatibility_date = "2026-10-01"

[build]
command = "worker-build --release""#,
        ),
    ])
}

/// The steps that run the worker locally and publish it, and what changes
/// on Workers.
fn publish(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "5. Run it locally",
                ja: "5. 手元で動かす",
            },
        ),
        code::block(Lang::Shell, None, "npx wrangler dev"),
        para(
            site,
            L {
                en: "`wrangler dev` builds the worker and runs it in workerd, the runtime Workers use, at `http://localhost:8787`. The first build takes about a minute.",
                ja: "`wrangler dev` は Worker をビルドし、Workers と同じ実行環境 workerd で `http://localhost:8787` に配信します。最初のビルドには 1 分ほどかかります。",
            },
        ),
        subheading(
            site,
            L {
                en: "6. Publish it",
                ja: "6. 公開する",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "npx wrangler login
npx wrangler deploy",
        ),
        para(
            site,
            L {
                en: "`wrangler login` opens the browser to sign in to Cloudflare. `wrangler deploy` builds the worker, uploads it, and prints its address, `https://my-worker.<your subdomain>.workers.dev`. On an account without a `workers.dev` subdomain, it asks to register one first.",
                ja: "`wrangler login` はブラウザを開いて Cloudflare にログインします。`wrangler deploy` は Worker をビルドしてアップロードし、`https://my-worker.<サブドメイン>.workers.dev` のアドレスを表示します。`workers.dev` のサブドメインをまだ持たないアカウントでは、先にサブドメインの登録を求められます。",
            },
        ),
        subheading(
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
