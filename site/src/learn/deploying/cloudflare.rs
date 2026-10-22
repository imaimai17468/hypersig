//! The page on deploying the Workers template on Cloudflare Workers.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

/// The worker of the Cloudflare template, which CI builds for
/// wasm32-unknown-unknown and the end-to-end tests run under `wrangler dev`.
const WORKER: &str =
    include_str!("../../../../crates/hypersig_cli/templates/cloudflare/src/lib.rs");

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DeployCloudflare).title(site)),
        lead(site, Doc::Learn(Learn::DeployCloudflare).leads()),
        workers(site),
        workers_existing(site),
        on_workers(site),
    ])
}

/// The commands that create a project from the Workers template, run it, and
/// publish it.
fn workers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Deploy from the template",
                ja: "テンプレートからデプロイする",
            },
        ),
        para(
            site,
            L {
                en: "These commands need a Cloudflare account, Rust 1.99 or later, and Node.js 22 or later. Node.js runs `wrangler`, Cloudflare's command-line tool, which is published only on npm, and `npx` fetches it on first use, so the project has no `package.json`. `cargo generate` copies the Workers template into `my-worker` and names the package and the worker after it, as it does for the [container](learn/deploying/container/).",
                ja: "このコマンドを実行するには、Cloudflare のアカウント、Rust 1.99 以降、Node.js 22 以降が必要です。Node.js は Cloudflare のコマンドラインツール `wrangler` を動かすためだけに使います。`wrangler` は npm でしか配布されていません。`npx` が最初の実行時にダウンロードするので、プロジェクトに `package.json` はありません。`cargo generate` は [コンテナ](learn/deploying/container/) のときと同じように Workers のテンプレートを `my-worker` にコピーし、パッケージと Worker にその名前を付けます。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo install cargo-generate
cargo generate --git https://github.com/imaimai17468/hypersig templates/cloudflare --name my-worker
cd my-worker
npx wrangler dev
npx wrangler login
npx wrangler deploy",
        ),
        para(
            site,
            L {
                en: "`wrangler dev` builds the worker and runs it at `http://localhost:8787` in workerd, the runtime Workers use. The first build installs `worker-build`, the tool that compiles the crate to wasm, and the wasm target, so it takes a few minutes. `wrangler dev` needs no Cloudflare login. `wrangler login` opens the browser to sign in to Cloudflare, which `wrangler deploy` needs. `wrangler deploy` uploads the worker and prints its address, `https://my-worker.<your subdomain>.workers.dev`, and on an account without a `workers.dev` subdomain it asks to register one first.",
                ja: "`wrangler dev` は Worker をビルドし、Workers と同じ実行環境 workerd で `http://localhost:8787` に配信します。最初のビルドでは、クレートを wasm にコンパイルするツール `worker-build` と wasm の target をインストールするので、数分かかります。`wrangler dev` に Cloudflare へのログインは不要です。`wrangler login` はブラウザを開いて Cloudflare にログインします。`wrangler deploy` にはこのログインが必要です。`wrangler deploy` は Worker をアップロードして `https://my-worker.<サブドメイン>.workers.dev` のアドレスを表示します。`workers.dev` のサブドメインをまだ持たないアカウントでは、先にサブドメインの登録を求められます。",
            },
        ),
        subheading(
            site,
            L {
                en: "What the template holds",
                ja: "テンプレートの中身",
            },
        ),
        table_of(
            site,
            L {
                en: &["File", "What it does"],
                ja: &["ファイル", "役割"],
            },
            L {
                en: &[
                    &[
                        "`src/lib.rs`",
                        "The worker below. The `+1` button counts in the browser, the second button doubles the count on the worker, and a stream sends the worker's time every second.",
                    ],
                    &[
                        "`Cargo.toml`",
                        "A library that `worker-build` compiles, with axum without its default features, which need tokio's networking.",
                    ],
                    &[
                        "`wrangler.toml`",
                        "The worker's name, and the build command, which installs `worker-build` when it is missing and runs it.",
                    ],
                    &[
                        "`rust-toolchain.toml`",
                        "Names the wasm target, which rustup adds to the default toolchain on the first build.",
                    ],
                ],
                ja: &[
                    &[
                        "`src/lib.rs`",
                        "下の Worker です。`+1` のボタンはブラウザで数を増やし、もう 1 つのボタンは Worker で数を 2 倍にし、ストリームは Worker の時刻を 1 秒ごとに送ります。",
                    ],
                    &[
                        "`Cargo.toml`",
                        "`worker-build` がコンパイルするライブラリです。axum は tokio のネットワーク機能を使う既定の機能を外して使います。",
                    ],
                    &[
                        "`wrangler.toml`",
                        "Worker の名前と、ビルドのコマンドです。コマンドは `worker-build` がなければインストールしてから実行します。",
                    ],
                    &[
                        "`rust-toolchain.toml`",
                        "wasm の target を書いておくと、rustup が最初のビルドで既定のツールチェーンに追加します。",
                    ],
                ],
            },
        ),
        code::block(Lang::Rust, Some("src/lib.rs"), code::without_header(WORKER)),
    ])
}

/// The changes that turn a server project the reader already has into a
/// worker.
fn workers_existing(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Turn a server project into a worker",
                ja: "サーバーのプロジェクトを Worker に移行する",
            },
        ),
        para(
            site,
            L {
                en: "A server project becomes a worker when its package turns from a binary into a library that `worker-build` compiles, and its `main` into the `fetch` function above. The pages, actions, and streams stay as they are.",
                ja: "サーバーのプロジェクトを Worker に移行するには、パッケージをバイナリから `worker-build` がコンパイルするライブラリに変え、`main` を上の `fetch` 関数に置き換えます。ページ、アクション、ストリームのコードはそのまま使えます。",
            },
        ),
        para(
            site,
            L {
                en: "In `Cargo.toml`, add a `[lib]` section that makes the package the library `worker-build` compiles, give axum `default-features = false`, add `tower-service` and `worker`, and remove tokio. hypersig, serde, and the other dependencies stay as they are:",
                ja: "`Cargo.toml` には、パッケージを `worker-build` がコンパイルするライブラリにする `[lib]` の節を追加します。axum は `default-features = false` にし、`tower-service` と `worker` を追加し、tokio を外してください。hypersig、serde、ほかの依存はそのままです。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[lib]
crate-type = ["cdylib"]

[dependencies]
axum = { version = "0.8", default-features = false }
tower-service = "0.3"
worker = { version = "0.8", features = ["http", "axum"] }"#,
        ),
        para(
            site,
            L {
                en: "Rename `src/main.rs` to `src/lib.rs`. Replace `#[tokio::main] async fn main` with a `fetch` function that builds the same `Router` as `main` did and hands it the request, and add the two `use` lines it needs. For the [quick start](learn/) counter, it is:",
                ja: "`src/main.rs` のファイル名を `src/lib.rs` に変えます。`#[tokio::main] async fn main` を、`main` と同じ `Router` を組み立ててリクエストを渡す `fetch` 関数に置き換え、必要な 2 行の `use` を追加します。[クイックスタート](learn/) のカウンターなら次のとおりです。",
            },
        ),
        code::block(
            Lang::Rust,
            Some("src/lib.rs"),
            "use tower_service::Service as _;
use worker::{Context, Env, HttpRequest, event};

#[event(fetch)]
async fn fetch(
    request: HttpRequest,
    _env: Env,
    _context: Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    let mut app = pages(Router::new()).hypersig_runtime().action(RESET, reset);
    Ok(app.call(request).await?)
}",
        ),
        para(
            site,
            L {
                en: "Download the template's `wrangler.toml` and `rust-toolchain.toml` next to `Cargo.toml`, and replace `{{project-name}}` in the `name` line of `wrangler.toml` with the worker's name. The build writes `build` and `.wrangler`, which stay out of Git:",
                ja: "テンプレートの `wrangler.toml` と `rust-toolchain.toml` を `Cargo.toml` の隣にダウンロードし、`wrangler.toml` の `name` の行にある `{{project-name}}` を Worker の名前に置き換えます。ビルドが書き出す `build` と `.wrangler` は Git の対象から外してください。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "curl -O https://raw.githubusercontent.com/imaimai17468/hypersig/main/templates/cloudflare/wrangler.toml
curl -O https://raw.githubusercontent.com/imaimai17468/hypersig/main/templates/cloudflare/rust-toolchain.toml
printf '/build\\n/.wrangler\\n' >> .gitignore",
        ),
        para(
            site,
            L {
                en: "Code that uses tokio changes too. A stream that waits with `tokio::time::sleep` waits with `worker::Delay`, as the clock above does, and state in the router's state does not outlast the isolate, as the next section says. Then `npx wrangler dev` runs the project.",
                ja: "tokio を使うコードも書き換えが必要です。`tokio::time::sleep` で待つストリームは、上の時計と同じように `worker::Delay` で待ちます。ルーターの状態に置いた値は、次の節のとおり isolate より長くは残りません。そのあとプロジェクトで `npx wrangler dev` を実行してください。",
            },
        ),
    ])
}

/// What runs differently on Workers than on a server.
fn on_workers(site: Site) -> Part {
    parts([
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
