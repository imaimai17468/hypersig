//! The page on deploying an application from a template, in a container and
//! on Cloudflare Workers.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

/// The worker of `templates/cloudflare`, which CI builds for
/// wasm32-unknown-unknown and the end-to-end tests run under `wrangler dev`.
const WORKER: &str = include_str!("../../../templates/cloudflare/src/lib.rs");

pub fn deploying(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Deploying).title(site)),
        lead(site, Doc::Learn(Learn::Deploying).leads()),
        container(site),
        workers(site),
        on_workers(site),
    ])
}

/// The commands that create a project from the server template and run its
/// container image.
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
                en: "These commands need Rust 1.99 or later and Docker. `cargo generate` copies the server template into `my-app` and names the package after it, and the last two lines build the image and run it.",
                ja: "このコマンドには Rust 1.99 以降と Docker が要ります。`cargo generate` はサーバーのテンプレートを `my-app` にコピーし、パッケージにその名前を付けます。最後の 2 行はイメージをビルドして動かします。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo install cargo-generate
cargo generate --git https://github.com/imaimai17468/hypersig templates/server --name my-app
cd my-app
docker build -t my-app .
docker run --rm -p 3000:3000 my-app",
        ),
        para(
            site,
            L {
                en: "Open `http://localhost:3000`. The project is the [quick start](learn/) counter, listening on every interface at the port in `PORT`, with a `Dockerfile` that builds the binary in one stage and copies it into a small image in the next. The browser runtime is compiled into the binary, so the image holds the binary alone. A host that builds from a Dockerfile, such as Fly.io, Cloud Run, or Render, takes the project as it is, and a host that runs an image takes the one built here, pushed to its registry.",
                ja: "`http://localhost:3000` を開いてください。プロジェクトの中身は [クイックスタート](learn/) のカウンターで、すべてのインターフェースの `PORT` のポートで待ち受けます。`Dockerfile` は 1 つ目のステージでバイナリをビルドし、2 つ目のステージで小さなイメージにコピーします。ブラウザのランタイムはバイナリに埋め込まれているので、イメージに入るのはバイナリだけです。Fly.io、Cloud Run、Render のように Dockerfile からビルドするホストにはこのままのプロジェクトを渡し、イメージを動かすホストにはここでビルドしたイメージをそのホストのレジストリに push して渡します。",
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

/// The commands that create a project from the Workers template, run it, and
/// publish it.
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
                en: "These commands need a Cloudflare account, Rust 1.99 or later, and Node.js 22 or later. Node.js runs `wrangler`, Cloudflare's command-line tool, which is published only on npm, and `npx` fetches it on first use, so the project has no `package.json`. `cargo generate` copies the Workers template into `my-worker` and names the package and the worker after it, as it does for the container.",
                ja: "このコマンドには、Cloudflare のアカウント、Rust 1.99 以降、Node.js 22 以降が要ります。Node.js は Cloudflare のコマンドラインツール `wrangler` を動かすためだけに使います。`wrangler` は npm でしか配られていません。`npx` が最初の実行で取ってくるので、プロジェクトに `package.json` はありません。`cargo generate` はコンテナのときと同じように Workers のテンプレートを `my-worker` にコピーし、パッケージと Worker にその名前を付けます。",
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
                en: "`wrangler dev` builds the worker and runs it at `http://localhost:8787` in workerd, the runtime Workers use. The first build installs `worker-build`, the tool that compiles the crate to wasm, and the wasm target, so it takes a few minutes. `wrangler login` opens the browser to sign in to Cloudflare. `wrangler deploy` uploads the worker and prints its address, `https://my-worker.<your subdomain>.workers.dev`, and on an account without a `workers.dev` subdomain it asks to register one first.",
                ja: "`wrangler dev` は Worker をビルドし、Workers と同じ実行環境 workerd で `http://localhost:8787` に配信します。最初のビルドでは、クレートを wasm にコンパイルするツール `worker-build` と wasm の target を入れるので、数分かかります。`wrangler login` はブラウザを開いて Cloudflare にログインします。`wrangler deploy` は Worker をアップロードして `https://my-worker.<サブドメイン>.workers.dev` のアドレスを表示します。`workers.dev` のサブドメインをまだ持たないアカウントでは、先にサブドメインの登録を求められます。",
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
                        "Worker の名前と、ビルドのコマンドです。コマンドは `worker-build` がなければ入れてから実行します。",
                    ],
                    &[
                        "`rust-toolchain.toml`",
                        "wasm の target を書いておくと、rustup が最初のビルドで既定のツールチェーンに足します。",
                    ],
                ],
            },
        ),
        code::block(Lang::Rust, Some("src/lib.rs"), code::without_header(WORKER)),
    ])
}

/// What runs differently on Workers than on a server.
fn on_workers(site: Site) -> Part {
    parts([
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
