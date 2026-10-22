//! The page on deploying the server template in a container.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

use crate::learn::snippet::{Snippet, show};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DeployContainer).title(site)),
        lead(site, Doc::Learn(Learn::DeployContainer).leads()),
        container(site),
        container_existing(site),
        host(site),
    ])
}

/// The commands that create a project from the server template and run its
/// container image.
fn container(site: Site) -> Part {
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
                en: "These commands need Rust 1.99 or later and Docker. `cargo generate` copies the server template into `my-app` and names the package after it, and the last two lines build the image and run it.",
                ja: "このコマンドを実行するには、Rust 1.99 以降と Docker が必要です。`cargo generate` はサーバーのテンプレートを `my-app` にコピーし、パッケージにその名前を付けます。最後の 2 行でイメージをビルドし、起動します。",
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
    ])
}

/// The two changes that let a project the reader already has build with the
/// template's `Dockerfile`.
fn container_existing(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Add the Dockerfile to a project you have",
                ja: "既存のプロジェクトに Dockerfile を追加する",
            },
        ),
        para(
            site,
            L {
                en: "The template's `Dockerfile` names no package, so it builds any project whose package has one binary. Download it to the directory that holds `Cargo.toml`, with a `.dockerignore` that keeps `target` out of the build.",
                ja: "テンプレートの `Dockerfile` はパッケージの名前を書いていないので、バイナリが 1 つのパッケージならどのプロジェクトでもそのままビルドできます。`Cargo.toml` のあるディレクトリにダウンロードし、`target` をビルドの対象から外す `.dockerignore` も置いてください。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "curl -O https://raw.githubusercontent.com/imaimai17468/hypersig/main/templates/server/Dockerfile
echo target >> .dockerignore",
        ),
        para(
            site,
            L {
                en: "A `main` that binds `127.0.0.1`, as the examples in this repository do, cannot be reached from outside the container. Listen on every interface at the port in `PORT` instead.",
                ja: "このリポジトリの例のように `127.0.0.1` で待ち受ける `main` には、コンテナの外から接続できません。すべてのインターフェースの `PORT` のポートで待ち受けるように変えてください。",
            },
        ),
        show(Snippet::Listen),
        para(
            site,
            L {
                en: "The `Dockerfile` builds with `cargo install --path .`, which fails at the root of a workspace, because the root holds no package. There, change `.` to the package's directory. The image's Rust is 1.99, so raise the `FROM` line where the project's `rust-version` is newer.",
                ja: "`Dockerfile` は `cargo install --path .` でビルドします。ワークスペースのルートにはパッケージがないので、そこでは失敗します。その場合は `.` をパッケージのディレクトリに変えてください。イメージの Rust は 1.99 なので、プロジェクトの `rust-version` がそれより新しいときは `FROM` の行のバージョンも上げてください。",
            },
        ),
    ])
}

/// What a container host has to allow for streams and shared state.
fn host(site: Site) -> Part {
    parts([
        heading(
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
