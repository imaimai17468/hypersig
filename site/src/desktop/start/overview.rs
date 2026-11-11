//! The Desktop overview: adding the crate, the `main` that opens the window, and a map of the section.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Overview).leads()),
        crate_setup(site),
        first_window(site),
        no_tokio_main(site),
        where_it_runs(site),
        features(site),
        map(site),
    ])
}

fn crate_setup(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Add the crate",
                ja: "クレートを追加する",
            },
        ),
        para(
            site,
            L {
                en: "Add `hypersig_desktop` next to the crates a hypersig application already uses: `hypersig` and `axum` for the router, and `serde` for the signal structs.",
                ja: "hypersig のアプリがすでに使っているクレートに `hypersig_desktop` を加えます。ルーターには `hypersig` と `axum` を、シグナルの構造体には `serde` を使います。",
            },
        ),
        code::block(Lang::Toml, Some("Cargo.toml"), CARGO_TOML),
        para(
            site,
            L {
                en: "[`App::run`](api:hypersig_desktop/struct.App.html#method.run) starts the Tokio runtime itself, so `main` needs no `tokio` dependency. Add `tokio` when the application's own code calls it, as `tokio::spawn` does, which needs the `rt` feature.",
                ja: "[`App::run`](api:hypersig_desktop/struct.App.html#method.run) は Tokio のランタイムを自分で起動するので、`main` のために `tokio` に依存する必要はありません。アプリのコードが `tokio::spawn` などで Tokio を直接呼ぶときに `tokio` を追加します。`tokio::spawn` には `rt` の feature が必要です。",
            },
        ),
        para(
            site,
            L {
                en: "On Linux, the crate links GTK and WebKitGTK at build time, and reads the desktop's color scheme over D-Bus. On Debian and Ubuntu, the repository's CI installs `libwebkit2gtk-4.1-dev` and `libdbus-1-dev` for these.",
                ja: "Linux では、ビルド時に GTK と WebKitGTK をリンクし、デスクトップの配色を D-Bus で読みます。Debian や Ubuntu では、リポジトリの CI がそのために `libwebkit2gtk-4.1-dev` と `libdbus-1-dev` を入れています。",
            },
        ),
    ])
}

/// The dependencies the overview's sample needs, with `tokio` for the
/// handlers that spawn tasks.
const CARGO_TOML: &str = r#"[dependencies]
axum = "0.8"
hypersig = "0.15"
hypersig_desktop = "0.15"
serde = { version = "1", features = ["derive"] }
# Only for code that calls Tokio itself, such as `tokio::spawn`.
tokio = { version = "1", features = ["rt"] }"#;

fn first_window(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Open the first window",
                ja: "最初のウィンドウを開く",
            },
        ),
        para(
            site,
            L {
                en: "The routes, handlers, and pages are the ones the application serves on the web. Only `main` changes: it names the application with [`App::new`](api:hypersig_desktop/struct.App.html#method.new), and calls `App::run` with the first page and a closure that builds the router.",
                ja: "ルート、ハンドラ、ページは、Web で配信するときと同じものを使います。書き換えるのは `main` だけです。[`App::new`](api:hypersig_desktop/struct.App.html#method.new) でアプリに名前を付け、最初のページと、ルーターを作るクロージャを渡して `App::run` を呼びます。",
            },
        ),
        show(Snippet::DesktopMain),
        para(
            site,
            L {
                en: "`cargo run` opens an 800 by 600 window that shows `GET /`, titled `Home` after the page's `<title>`. The first page is a [`Url`](api:hypersig/struct.Url.html), which only `Route::url` produces, so `run` cannot be given a path that no route declares. The closure receives an [`Os`](api:hypersig_desktop/struct.Os.html), through which the handlers open windows and reach the system, and returns a future that produces the `Router`. [The application](desktop/app/) covers each argument and what `run` returns.",
                ja: "`cargo run` を実行すると、`GET /` を表示する 800×600 のウィンドウが開きます。ウィンドウのタイトルはページの `<title>` と同じ `Home` です。最初のページは [`Url`](api:hypersig/struct.Url.html) で渡します。`Url` は `Route::url` からしか作れないので、どのルートも宣言していないパスは `run` に渡せません。クロージャは [`Os`](api:hypersig_desktop/struct.Os.html) を受け取り、`Router` を返す future を返します。ハンドラは `Os` を使ってウィンドウを開いたり、OS の機能を使ったりします。それぞれの引数と `run` の戻り値は [アプリの設定](desktop/app/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "One package can build both: [`examples/counter`](https://github.com/imaimai17468/hypersig/tree/main/examples/counter) serves the page on port 3000 with `cargo run -p example-counter`, and opens it in a window with `--features desktop`, which makes `hypersig_desktop` an optional dependency and picks one of two `main` functions:",
                ja: "1 つのパッケージで Web 版とデスクトップ版の両方をビルドすることもできます。[`examples/counter`](https://github.com/imaimai17468/hypersig/tree/main/examples/counter) は `cargo run -p example-counter` でポート 3000 にページを配信し、`--features desktop` を付けるとウィンドウで開きます。この feature は `hypersig_desktop` を任意の依存として有効にし、2 つの `main` のどちらを使うかを決めます。",
            },
        ),
        code::block(
            Lang::Rust,
            Some("examples/counter/src/main.rs"),
            COUNTER_MAIN,
        ),
    ])
}

/// The two `main` functions of examples/counter, as that file has them.
const COUNTER_MAIN: &str = r#"#[cfg(not(feature = "desktop"))]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    hypersig::serve(app(), 3000).await
}

/// With `--features desktop`, the counter opens in a window of its own.
#[cfg(feature = "desktop")]
fn main() -> Result<(), hypersig_desktop::Error> {
    const APP: hypersig_desktop::App = hypersig_desktop::App::new("dev.hypersig.example-counter");
    APP.size(560, 480)
        .run(&INDEX.url(()), |_os| async { app() })
}"#;

fn no_tokio_main(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Why `main` is not `#[tokio::main]`",
                ja: "`main` に `#[tokio::main]` を付けない理由",
            },
        ),
        para(
            site,
            L {
                en: "`App::run` keeps the thread that calls it for the windows and their event loop, and on macOS that thread has to be the main thread; `run` panics on any other. It starts a multi-threaded Tokio runtime of its own and runs the server and the closure's future there. So `main` is a plain `fn` that returns `Result<(), hypersig_desktop::Error>`.",
                ja: "`App::run` は、呼び出したスレッドをウィンドウとそのイベントループのために使い続けます。macOS ではそのスレッドがメインスレッドでなければならず、ほかのスレッドから呼ぶと `run` はパニックします。`run` はマルチスレッドの Tokio ランタイムを自分で起動し、サーバーとクロージャの future をその上で動かします。そのため `main` は `Result<(), hypersig_desktop::Error>` を返す普通の `fn` にします。",
            },
        ),
        note(
            site,
            L {
                en: "A `main` with `#[tokio::main]` that calls `App::run` compiles, and `run` returns an error of the kind `ErrorKind::InRuntime` as it starts, without opening a window.",
                ja: "`#[tokio::main]` を付けた `main` から `App::run` を呼ぶコードはコンパイルできますが、起動するとすぐに `run` が `ErrorKind::InRuntime` のエラーを返し、ウィンドウは開きません。",
            },
        ),
    ])
}

fn where_it_runs(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What runs where",
                ja: "どこで何が動くか",
            },
        ),
        para(
            site,
            L {
                en: "The server and the windows are one process. The `Router` the closure returns is served over HTTP/1.1 on `127.0.0.1`, and each window is the system's web view, which requests the pages and sends the actions and streams to that server, as a browser tab does. The server answers only the application's own windows; [Who the server answers](desktop/security/) covers how.",
                ja: "サーバーとウィンドウは 1 つのプロセスで動きます。クロージャが返した `Router` は `127.0.0.1` 上で HTTP/1.1 で配信されます。それぞれのウィンドウは OS の Web ビューで、ブラウザのタブと同じように、このサーバーにページを要求し、アクションとストリームの要求を送ります。サーバーはアプリ自身のウィンドウにしか応答しません。その仕組みは [サーバーが応答する相手](desktop/security/) で説明します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["System", "Web view", "What the system needs"],
                ja: &["OS", "Web ビュー", "OS に必要なもの"],
            },
            L {
                en: &[
                    &[
                        "macOS",
                        "WKWebView",
                        "Nothing more; WKWebView is part of macOS",
                    ],
                    &[
                        "Windows",
                        "WebView2",
                        "The WebView2 Runtime, which Windows 11 includes",
                    ],
                    &["Linux", "WebKitGTK", "GTK 3 and WebKitGTK 4.1"],
                ],
                ja: &[
                    &[
                        "macOS",
                        "WKWebView",
                        "追加は不要（WKWebView は macOS に含まれる）",
                    ],
                    &[
                        "Windows",
                        "WebView2",
                        "WebView2 Runtime（Windows 11 には含まれる）",
                    ],
                    &["Linux", "WebKitGTK", "GTK 3 と WebKitGTK 4.1"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "[Limits](desktop/limits/) covers what the windows share, such as the connections to the server.",
                ja: "サーバーへの接続など、ウィンドウが共有するものは [制約](desktop/limits/) で説明します。",
            },
        ),
    ])
}

fn features(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Features",
                ja: "feature",
            },
        ),
        para(
            site,
            L {
                en: "No feature is on by default. Each one adds items or behavior for one purpose:",
                ja: "既定で有効な feature はありません。それぞれの feature は 1 つの用途のための型や動作を追加します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Feature", "Adds", "Page"],
                ja: &["feature", "追加されるもの", "ページ"],
            },
            L {
                en: &[
                    &[
                        "`testing`",
                        "[`testing::FakeOs`](api:hypersig_desktop/testing/struct.FakeOs.html), an `Os` whose dialogs, clipboard, and windows a test answers and reads",
                        "[Testing without a window](desktop/testing/)",
                    ],
                    &[
                        "`updater`",
                        "`App::updates` and `Os::updates`, which find, download, verify, and install a new version",
                        "[Updates](desktop/updates/)",
                    ],
                    &[
                        "`smoke`",
                        "Smoke runs, started by the `HYPERSIG_DESKTOP_SMOKE` and `HYPERSIG_DESKTOP_SMOKE_SCRIPT` environment variables; a build without it ignores both",
                        "[Smoke runs in CI](desktop/smoke-runs/)",
                    ],
                ],
                ja: &[
                    &[
                        "`testing`",
                        "[`testing::FakeOs`](api:hypersig_desktop/testing/struct.FakeOs.html)。ダイアログ、クリップボード、ウィンドウをテストから操作し、読み出せる `Os`",
                        "[ウィンドウなしのテスト](desktop/testing/)",
                    ],
                    &[
                        "`updater`",
                        "新しいバージョンを探し、ダウンロードし、署名を確かめてインストールする `App::updates` と `Os::updates`",
                        "[更新の配布](desktop/updates/)",
                    ],
                    &[
                        "`smoke`",
                        "環境変数 `HYPERSIG_DESKTOP_SMOKE` と `HYPERSIG_DESKTOP_SMOKE_SCRIPT` で始まるスモークテスト。この feature がないビルドは両方を無視する",
                        "[CI でのスモークテスト](desktop/smoke-runs/)",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`testing` is for tests only, so `examples/notes` turns it on in `[dev-dependencies]`, and the binary it ships is built without it:",
                ja: "`testing` はテストのためだけに使うので、`examples/notes` は `[dev-dependencies]` でこの feature を有効にしています。配布するバイナリはこの feature なしでビルドされます。",
            },
        ),
        code::block(Lang::Toml, Some("Cargo.toml"), TESTING_TOML),
    ])
}

/// How examples/notes turns on `testing` for its tests alone.
const TESTING_TOML: &str = r#"[dependencies]
hypersig_desktop = "0.15"

[dev-dependencies]
hypersig_desktop = { version = "0.15", features = ["testing"] }"#;

fn map(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The pages of this section",
                ja: "このセクションのページ",
            },
        ),
        table_of(
            site,
            L {
                en: &["Page", "Covers"],
                ja: &["ページ", "内容"],
            },
            L {
                en: &[
                    &[
                        "[The application](desktop/app/)",
                        "`App::new`, the window's size, `App::data_dir`, `App::run`, and its errors",
                    ],
                    &[
                        "[Opening windows](desktop/windows/)",
                        "`Windows::open`, `Window`, and windows a page opens",
                    ],
                    &[
                        "[File dialogs](desktop/dialogs/)",
                        "`Os::dialog` for opening and saving files",
                    ],
                    &[
                        "[The clipboard](desktop/clipboard/)",
                        "`Os::clipboard` and the clipboard's text",
                    ],
                    &[
                        "[Links to other sites](desktop/external-links/)",
                        "Links that leave the application, and `Os::open_external`",
                    ],
                    &[
                        "[Testing without a window](desktop/testing/)",
                        "`testing::FakeOs` with `Page::simulate`",
                    ],
                    &[
                        "[Smoke runs in CI](desktop/smoke-runs/)",
                        "The `smoke` feature and its environment variables",
                    ],
                    &[
                        "[Packaging](desktop/packaging/)",
                        "`cargo packager` and the packages for each system",
                    ],
                    &[
                        "[Updates](desktop/updates/)",
                        "The `updater` feature, `App::updates`, and `Os::updates`",
                    ],
                    &[
                        "[Who the server answers](desktop/security/)",
                        "What the server refuses, and cookies inside the window",
                    ],
                    &[
                        "[Signals kept between runs](desktop/persisted-signals/)",
                        "The port the server listens on, and `Page::persist`",
                    ],
                    &[
                        "[Limits](desktop/limits/)",
                        "What the windows share, and what each system needs",
                    ],
                ],
                ja: &[
                    &[
                        "[アプリの設定](desktop/app/)",
                        "`App::new`、ウィンドウの大きさ、`App::data_dir`、`App::run` とそのエラー",
                    ],
                    &[
                        "[ウィンドウを開く](desktop/windows/)",
                        "`Windows::open`、`Window`、ページが開くウィンドウ",
                    ],
                    &[
                        "[ファイルのダイアログ](desktop/dialogs/)",
                        "ファイルを開く・保存するための `Os::dialog`",
                    ],
                    &[
                        "[クリップボード](desktop/clipboard/)",
                        "`Os::clipboard` とクリップボードのテキスト",
                    ],
                    &[
                        "[ほかのサイトへのリンク](desktop/external-links/)",
                        "アプリの外へ出るリンクと `Os::open_external`",
                    ],
                    &[
                        "[ウィンドウなしのテスト](desktop/testing/)",
                        "`testing::FakeOs` と `Page::simulate`",
                    ],
                    &[
                        "[CI でのスモークテスト](desktop/smoke-runs/)",
                        "`smoke` の feature とその環境変数",
                    ],
                    &[
                        "[パッケージの作成](desktop/packaging/)",
                        "`cargo packager` と OS ごとのパッケージ",
                    ],
                    &[
                        "[更新の配布](desktop/updates/)",
                        "`updater` の feature、`App::updates`、`Os::updates`",
                    ],
                    &[
                        "[サーバーが応答する相手](desktop/security/)",
                        "サーバーが拒否する要求と、ウィンドウの中の cookie",
                    ],
                    &[
                        "[起動をまたいで残るシグナル](desktop/persisted-signals/)",
                        "サーバーが待ち受けるポートと `Page::persist`",
                    ],
                    &[
                        "[制約](desktop/limits/)",
                        "ウィンドウが共有するものと、OS ごとに必要なもの",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Next, [The application](desktop/app/) covers what `App` holds and what `App::run` does from the call to its return.",
                ja: "次の [アプリの設定](desktop/app/) では、`App` が持つ設定と、`App::run` が呼ばれてから戻るまでに行うことを説明します。",
            },
        ),
    ])
}
