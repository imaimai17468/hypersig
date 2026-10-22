//! The page on deploying the server template in a container.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, title};

use crate::learn::snippet::{Snippet, show};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DeployContainer).title(site)),
        lead(site, Doc::Learn(Learn::DeployContainer).leads()),
        container(site),
        on_a_host(site),
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
                en: "Open `http://localhost:3000`. The project is the [quick start](learn/) counter, listening on every interface at the port in `PORT`, with a `Dockerfile` that builds the binary in one stage and copies it into a small image in the next. The browser runtime is compiled into the binary, so the image holds the binary alone. Stop it with Ctrl+C.",
                ja: "`http://localhost:3000` を開いてください。プロジェクトの中身は [クイックスタート](learn/) のカウンターで、すべてのインターフェースの `PORT` のポートで待ち受けます。`Dockerfile` は 1 つ目のステージでバイナリをビルドし、2 つ目のステージで小さなイメージにコピーします。ブラウザのランタイムはバイナリに埋め込まれているので、イメージに入るのはバイナリだけです。止めるときは Ctrl+C を押してください。",
            },
        ),
        para(
            site,
            L {
                en: "`docker build` prints `warning: no Cargo.lock file published` when the project has no `Cargo.lock` yet, and `warning: be sure to add /out/bin to your PATH`. Neither stops the build.",
                ja: "`docker build` は、プロジェクトにまだ `Cargo.lock` がないと `warning: no Cargo.lock file published` を表示し、ほかに `warning: be sure to add /out/bin to your PATH` も表示します。どちらもビルドを止めません。",
            },
        ),
    ])
}

/// How the image reaches a host: Cloud Run with the command that was run,
/// and the other hosts with what they need from the image.
fn on_a_host(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Put it on a host",
                ja: "ホストに置く",
            },
        ),
        para(
            site,
            L {
                en: "On Cloud Run, these commands build the image from the `Dockerfile` with Cloud Build and deploy it. They need the `gcloud` command-line tool and a Google Cloud project with billing enabled, and the first deploy asks to turn on the APIs it uses. Choose the region in `--region`. `--timeout 3600` raises the request timeout from its default of 300 seconds to the most Cloud Run allows, which the next section explains.",
                ja: "Cloud Run では、次のコマンドが Cloud Build で `Dockerfile` からイメージをビルドし、デプロイします。`gcloud` のコマンドラインツールと、請求先を設定した Google Cloud のプロジェクトが必要です。最初のデプロイでは、使う API を有効にするかを尋ねられます。リージョンは `--region` で選んでください。`--timeout 3600` はリクエストのタイムアウトを既定の 300 秒から Cloud Run の上限まで延ばします。理由は次の節で説明します。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "gcloud auth login
gcloud config set project <your project id>
gcloud run deploy my-app --source . --region asia-northeast1 --allow-unauthenticated --timeout 3600",
        ),
        para(
            site,
            L {
                en: "The last command prints the service's address, `https://my-app-<number>.<region>.run.app`. Cloud Run gives the container the port in `PORT`, which the server listens on.",
                ja: "最後のコマンドはサービスのアドレス（`https://my-app-<番号>.<リージョン>.run.app`）を表示します。Cloud Run はコンテナに `PORT` でポートを渡し、サーバーはそのポートで待ち受けます。",
            },
        ),
        para(
            site,
            L {
                en: "Fly.io and Render build from the `Dockerfile` too; follow their guides for a Dockerfile app. The server listens on the port in `PORT`, and on 3000 when `PORT` is unset, so tell the host that port or set `PORT` to the one it connects to. A host that runs an image you push to its registry takes the one `docker build` makes. An image built on a Mac with Apple Silicon is for arm64, so build with `docker build --platform linux/amd64 -t my-app .` for a host that runs amd64.",
                ja: "Fly.io と Render も `Dockerfile` からビルドします。それぞれの Dockerfile のアプリの手順に従ってください。サーバーは `PORT` のポート（`PORT` がなければ 3000）で待ち受けるので、ホストにそのポートを伝えるか、ホストが接続するポートを `PORT` に設定してください。レジストリに push したイメージを動かすホストには、`docker build` で作ったイメージを渡します。Apple Silicon の Mac でビルドしたイメージは arm64 向けなので、amd64 で動かすホストには `docker build --platform linux/amd64 -t my-app .` でビルドしてください。",
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
                en: "A `main` that binds `127.0.0.1`, as the [quick start](learn/) does, cannot be reached from outside the container. Replace its `TcpListener::bind(\"127.0.0.1:3000\")` line and the `axum::serve` line below it with the lines below, which listen on every interface at the port in `PORT`, and add the `shutdown` function after `main`. The server runs as the container's first process, which does not stop on Ctrl+C or on the SIGTERM a host sends unless it handles them, so without `shutdown`, `docker stop` waits 10 seconds and then kills it. Add `\"signal\"` to tokio's features in `Cargo.toml` for it.",
                ja: "[クイックスタート](learn/) のように `127.0.0.1` で待ち受ける `main` には、コンテナの外から接続できません。`TcpListener::bind(\"127.0.0.1:3000\")` の行と、その下の `axum::serve` の行を次の行に置き換え、すべてのインターフェースの `PORT` のポートで待ち受けるようにします。`main` のあとには `shutdown` 関数を追加してください。サーバーはコンテナの最初のプロセスとして動き、自分で扱わない限り Ctrl+C やホストが送る SIGTERM では止まりません。そのため `shutdown` がないと、`docker stop` は 10 秒待ってから強制終了します。`Cargo.toml` の tokio の features には `\"signal\"` を追加してください。",
            },
        ),
        show(Snippet::Listen),
        para(
            site,
            L {
                en: "Then build and run the image as with the template, naming it after the project:",
                ja: "そのあと、テンプレートのときと同じようにイメージをビルドして起動します。イメージにはプロジェクトの名前を付けてください。",
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
                    en: "A stream holds its request open for as long as it sends patches. A host that ends a request after a time limit closes the stream at that limit, and a call with [`reconnect()`](learn/server/) opens it again after 1 second. On Cloud Run, with its default timeout of 300 seconds, a stream that sent the time every second closed after 303 seconds and opened again. Raise the limit with `--timeout` so the page reconnects less often.",
                    ja: "ストリームはパッチを送るあいだリクエストを開いたままにします。リクエストを一定の時間で終えるホストでは、その時間でストリームが切れます。[`reconnect()`](learn/server/) を付けた呼び出しは、切れたストリームを 1 秒後に開き直します。既定のタイムアウトが 300 秒の Cloud Run で確かめると、1 秒ごとに時刻を送るストリームは 303 秒で切れ、開き直りました。ページがつなぎ直す回数を減らすには、`--timeout` で上限を延ばしてください。",
                },
                L {
                    en: "State kept in memory, such as a counter in the router's state, belongs to one process. When the host runs more than one instance, keep that state in a database.",
                    ja: "ルーターの状態に置いたカウンターのようにメモリに持つ状態は、1 つのプロセスのものです。ホストがインスタンスを複数動かすときは、その状態をデータベースに置いてください。",
                },
            ],
        ),
    ])
}
