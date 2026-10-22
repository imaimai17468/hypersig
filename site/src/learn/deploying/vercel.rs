//! The page on deploying the Vercel template as a Vercel Function.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

/// The function of `templates/vercel`, which CI lints and the end-to-end tests
/// run through `VercelLayer` outside Vercel.
const FUNCTION: &str = include_str!("../../../../templates/vercel/api/app.rs");

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DeployVercel).title(site)),
        lead(site, Doc::Learn(Learn::DeployVercel).leads()),
        vercel(site),
        vercel_existing(site),
        on_vercel(site),
    ])
}

/// The commands that create a project from the Vercel template, run it, and
/// publish it.
fn vercel(site: Site) -> Part {
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
                en: "These commands need a Vercel account, Rust 1.99 or later, and Node.js 22 or later. Node.js runs `vercel`, Vercel's command-line tool, which `npx` fetches on first use. `cargo generate` copies the Vercel template into `my-app` and names the package after it.",
                ja: "このコマンドを実行するには、Vercel のアカウント、Rust 1.99 以降、Node.js 22 以降が必要です。Node.js は Vercel のコマンドラインツール `vercel` を動かすために使い、`npx` が最初の実行時にダウンロードします。`cargo generate` は Vercel のテンプレートを `my-app` にコピーし、パッケージにその名前を付けます。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo install cargo-generate
cargo generate --git https://github.com/imaimai17468/hypersig templates/vercel --name my-app
cd my-app
cargo run
npx vercel login
npx vercel deploy --prod",
        ),
        para(
            site,
            L {
                en: "`cargo run` starts the function at `http://localhost:3000`. Outside Vercel, `vercel_runtime::run` listens on `127.0.0.1` at the port in `VERCEL_DEV_PORT`, or 3000, and hands each request to the same `VercelLayer` a deployment uses. `vercel login` signs the command-line tool in to Vercel, which keeps its own login apart from the browser's, and `vercel deploy` stops with `No existing credentials found` until it has run.",
                ja: "`cargo run` は関数を `http://localhost:3000` で起動します。Vercel の外では、`vercel_runtime::run` は `127.0.0.1` の `VERCEL_DEV_PORT` のポート（なければ 3000）で待ち受け、受けたリクエストをデプロイ先と同じ `VercelLayer` に渡します。`vercel login` はコマンドラインツールを Vercel にログインさせます。ツールのログインはブラウザのログインとは別なので、これを実行するまで `vercel deploy` は `No existing credentials found` で止まります。",
            },
        ),
        para(
            site,
            L {
                en: "The first `vercel deploy` asks which account and project the directory belongs to, creates the project, and links the directory to it in `.vercel`, which the template's `.gitignore` leaves out. It then uploads the source, and Vercel builds it on its own servers with Rust from rustup, so no binary is uploaded. `--prod` publishes to the project's production address, `https://my-app.vercel.app` where the name is free, and prints it. Without `--prod` the deployment is a preview at an address of its own, which a project with Deployment Protection shows only to members signed in to Vercel. A Git repository connected to the project deploys a preview on each push to a branch, and production on each push to the main branch.",
                ja: "最初の `vercel deploy` は、ディレクトリをどのアカウントのどのプロジェクトにするかを尋ね、プロジェクトを作って `.vercel` でディレクトリと結び付けます。`.vercel` はテンプレートの `.gitignore` で除外してあります。そのあとソースをアップロードし、Vercel が自分のサーバーで rustup の Rust を使ってビルドするので、バイナリはアップロードしません。`--prod` を付けるとプロジェクトの本番のアドレスに公開し、そのアドレスを表示します。名前が空いていれば `https://my-app.vercel.app` です。`--prod` がないとプレビューのデプロイになり、専用のアドレスに置かれます。Deployment Protection を有効にしたプロジェクトでは、プレビューは Vercel にログインしたメンバーにしか表示されません。プロジェクトに Git のリポジトリをつなぐと、ブランチへの push でプレビューが、main ブランチへの push で本番がデプロイされます。",
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
                        "`api/app.rs`",
                        "The function below. The `+1` button counts in the browser, the second button doubles the count in the function, and a stream sends the function's time every second.",
                    ],
                    &[
                        "`Cargo.toml`",
                        "The binary `app`, built from `api/app.rs`, which Vercel turns into the function at `/api/app`, with `vercel_runtime` and its `axum` feature.",
                    ],
                    &[
                        "`vercel.json`",
                        "Sends every path to `/api/app`, so the axum `Router` sees each request with its own path, and gives Vercel an empty `public` directory to serve as static files, which the last section explains.",
                    ],
                    &["`.vercelignore`", "Keeps `target` out of the upload."],
                    &[
                        "`rust-toolchain.toml`",
                        "Names Rust 1.99, which rustup installs on Vercel's build image, whose own Rust is older than hypersig needs.",
                    ],
                ],
                ja: &[
                    &[
                        "`api/app.rs`",
                        "下の関数です。`+1` のボタンはブラウザで数を増やし、もう 1 つのボタンは関数で数を 2 倍にし、ストリームは関数の時刻を 1 秒ごとに送ります。",
                    ],
                    &[
                        "`Cargo.toml`",
                        "`api/app.rs` からビルドするバイナリ `app` を宣言します。Vercel はこれを `/api/app` の関数にします。依存には `axum` の機能を付けた `vercel_runtime` があります。",
                    ],
                    &[
                        "`vercel.json`",
                        "すべてのパスを `/api/app` に送ります。axum の `Router` は各リクエストを元のパスのまま受け取ります。また、Vercel が静的ファイルとして配信するディレクトリを空の `public` にします。理由は最後の節で説明します。",
                    ],
                    &[
                        "`.vercelignore`",
                        "`target` をアップロードの対象から外します。",
                    ],
                    &[
                        "`rust-toolchain.toml`",
                        "Rust 1.99 を指定します。Vercel のビルド環境にある Rust は hypersig が必要とするものより古いので、rustup がこのバージョンをインストールします。",
                    ],
                ],
            },
        ),
        code::block(
            Lang::Rust,
            Some("api/app.rs"),
            code::without_header(FUNCTION),
        ),
    ])
}

/// The changes that turn a server project the reader already has into a
/// Vercel Function.
fn vercel_existing(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Turn a server project into a Vercel Function",
                ja: "サーバーのプロジェクトを Vercel の関数に移行する",
            },
        ),
        para(
            site,
            L {
                en: "A server project becomes a Vercel Function when its `main` hands the `Router` to `vercel_runtime::run` in place of `axum::serve`. The function runs on tokio as the server did, so the pages, actions, and streams stay as they are, `tokio::time::sleep` included.",
                ja: "サーバーのプロジェクトを Vercel の関数に移行するには、`main` で `Router` を `axum::serve` ではなく `vercel_runtime::run` に渡します。関数はサーバーと同じく tokio の上で動くので、ページ、アクション、ストリームのコードは `tokio::time::sleep` も含めてそのまま使えます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["File", "What changes"],
                ja: &["ファイル", "変えること"],
            },
            L {
                en: &[
                    &[
                        "`Cargo.toml`",
                        "Add `vercel_runtime` with the feature `axum`, and `tower`. Add a `[[bin]]` named `app` whose `path` is `api/app.rs`.",
                    ],
                    &[
                        "`src/main.rs`",
                        "Move it to `api/app.rs`, and replace the listener and `axum::serve` with the template's last line, which wraps the `Router` in `VercelLayer` and runs it.",
                    ],
                    &[
                        "`vercel.json`, `.vercelignore`, `rust-toolchain.toml`",
                        "Copy them from `templates/vercel`.",
                    ],
                ],
                ja: &[
                    &[
                        "`Cargo.toml`",
                        "`axum` の機能を付けた `vercel_runtime` と `tower` を追加します。`path` が `api/app.rs` で名前が `app` の `[[bin]]` も追加します。",
                    ],
                    &[
                        "`src/main.rs`",
                        "`api/app.rs` に移し、リスナーと `axum::serve` をテンプレートの最後の行に置き換えます。この行は `Router` を `VercelLayer` で包んで実行します。",
                    ],
                    &[
                        "`vercel.json`、`.vercelignore`、`rust-toolchain.toml`",
                        "`templates/vercel` からコピーします。",
                    ],
                ],
            },
        ),
    ])
}

/// What runs differently on Vercel than on a server.
fn on_vercel(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What changes on Vercel",
                ja: "Vercel で変わること",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Vercel serves the files of its output directory before a rewrite applies, and a project that names none has the project's root as that directory. There, `Cargo.toml`, `Cargo.lock`, and every file outside `api/`, the Rust source under `src/` included, are public at their paths, and `.env` was the one file tried that was not. The template's `vercel.json` names `public` as the output directory, which the build command creates holding only an empty `.keep`, so every other path reaches the function. Keep those two fields in a `vercel.json` of your own.",
                    ja: "Vercel は rewrite より先に出力ディレクトリのファイルを配信し、出力ディレクトリを指定しないプロジェクトではプロジェクトのルートがそのディレクトリになります。その場合、`Cargo.toml`、`Cargo.lock`、`src/` の Rust のソースを含めて `api/` の外のファイルはすべて、そのパスで公開されます。試したファイルのうち公開されなかったのは `.env` だけでした。テンプレートの `vercel.json` は出力ディレクトリを `public` にし、ビルドのコマンドが空の `.keep` だけを入れてこのディレクトリを作るので、それ以外のパスはすべて関数に届きます。自分で `vercel.json` を書くときも、この 2 つのフィールドを残してください。",
                },
                L {
                    en: "Vercel ends a request at the function's maximum duration, which counts the time a stream stays open. The limit is 300 seconds by default and on the Hobby plan, and the Pro plan raises it to 800 seconds with `maxDuration`. A call with `reconnect()` opens the stream again after the limit closes it.",
                    ja: "Vercel は関数の最大実行時間でリクエストを終えます。この時間にはストリームを開いている時間も含まれます。上限は既定で 300 秒で、Hobby プランでは 300 秒より延ばせません。Pro プランでは `maxDuration` で 800 秒まで延ばせます。`reconnect()` を付けた呼び出しは、上限で切れたストリームをつなぎ直します。",
                },
                L {
                    en: "Vercel may run more than one instance of the function and stops an idle one, so a value kept in memory may be gone at the next request. Keep the state requests share in a database.",
                    ja: "Vercel は関数のインスタンスを複数動かすことがあり、使われていないインスタンスは止めます。メモリに置いた値は次のリクエストで消えていることがあるので、リクエストをまたいで共有する状態はデータベースに置いてください。",
                },
                L {
                    en: "`VercelLayer` reads the whole request body before the `Router` sees it, so a file upload read in chunks arrives in memory at once. Vercel refuses a request body over 4.5 MB.",
                    ja: "`VercelLayer` はリクエストの本文をすべて読んでから `Router` に渡します。ファイルのアップロードを少しずつ読むコードでも、本文は一度にメモリに載ります。Vercel は 4.5 MB を超える本文を受け付けません。",
                },
                L {
                    en: "`hypersig_runtime()` sends the runtime compressed with Brotli, with `Content-Encoding: br`. Vercel compresses it again on the way, so the bytes the browser receives differ from the ones the function sent, but they are compressed once and decode to the same runtime.",
                    ja: "`hypersig_runtime()` はランタイムを Brotli で圧縮し、`Content-Encoding: br` を付けて送ります。Vercel は途中でこれを圧縮し直すので、ブラウザが受け取るバイト列は関数が送ったものと変わります。ただし圧縮は 1 回だけで、展開すると同じランタイムになります。",
                },
            ],
        ),
    ])
}
