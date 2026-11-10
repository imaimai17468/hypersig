//! The page on running an application in a desktop window with
//! `hypersig_desktop`, testing it without one, and packaging it.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

/// The `main` of the counter example's desktop build.
const MAIN: &str = r#"const APP: hypersig_desktop::App = hypersig_desktop::App::new("dev.example.counter");

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.size(560, 480).run(&INDEX.url(()), |_os| async { app() })
}"#;

/// The handler of examples/notes that opens a file.
const OPEN: &str = r#"async fn open(state: AppState, (): ()) -> Patches<NotesPage> {
    let status = NotesPage::status();
    let chosen = state.os.dialog().filter("Text", &["txt", "md"]).open_file().await;
    let Some(path) = chosen else {
        return status.patch(String::new()).into();
    };
    let note = match state.keep(path) {
        Ok(note) => note,
        Err(error) => return status.patch(format!("Not read: {error}")).into(),
    };
    match state.os.windows().open(&NOTE.url(note.id)).await {
        Ok(_) => status.patch(format!("Opened {}.", note.name)).into(),
        Err(error) => status.patch(format!("Not shown: {error}")).into(),
    }
}"#;

/// A test of examples/notes, which opens no window.
const TEST: &str = r#"#[tokio::test]
async fn opening_a_file_shows_it_in_a_window_of_its_own() {
    let dir = folder("open");
    let fake = FakeOs::new();
    fake.answer_dialog([dir.join("today.md")]);
    let state = AppState::new(fake.os());

    let page = index(state.clone(), ()).await;
    let mut browser = page.simulate();
    let effects = browser.click(OPEN_BUTTON);
    assert_eq!(effects.calls(&OPEN), [()]);
    browser.apply(open(state.clone(), ()).await);

    assert_eq!(browser.value_of(NotesPage::status()), "Opened today.md.");
    assert_eq!(fake.open_windows(), [NOTE.url(1)]);
}"#;

/// The packaging table of examples/notes/Cargo.toml.
const PACKAGER: &str = r#"[package.metadata.packager]
product-name = "hypersig notes"
identifier = "dev.hypersig.example-notes"
version = "0.1.0"
before-packaging-command = "cargo build --release -p example-notes"
binaries = [{ path = "example-notes", main = true }]"#;

pub(crate) fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DeployDesktop).title(site)),
        lead(site, Doc::Learn(Learn::DeployDesktop).leads()),
        window(site),
        system(site),
        testing(site),
        packaging(site),
        limits(site),
    ])
}

fn window(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Open the application in a window",
                ja: "アプリをウィンドウで開く",
            },
        ),
        para(
            site,
            L {
                en: "`hypersig_desktop::App::run` serves the `Router` in the same process, on `127.0.0.1`, and shows its pages in the system's web view: WKWebView on macOS, WebView2 on Windows, and WebKitGTK on Linux. Only `main` changes. The window and its event loop take the main thread, so `main` is not `#[tokio::main]`; `run` starts the Tokio runtime itself and runs the future the closure returns on it.",
                ja: "`hypersig_desktop::App::run` は `Router` を同じプロセスの中で `127.0.0.1` に配信し、そのページを OS の Web ビューで表示します。macOS では WKWebView、Windows では WebView2、Linux では WebKitGTK を使います。書き換えるのは `main` だけです。ウィンドウとそのイベントループがメインスレッドを使うので、`main` に `#[tokio::main]` は付けません。`run` が Tokio のランタイムを自分で起動し、クロージャが返す future をその上で動かします。",
            },
        ),
        code::block(Lang::Rust, None, MAIN),
        para(
            site,
            L {
                en: "`App::new` takes the application's identifier, a reverse domain name that a `const` checks at compile time. It names the directory `App::data_dir` returns, where the application keeps its files, the port its server listens on, and the lock that stops a second copy from starting. The first page is a `Url` from `Route::url`, so a window cannot open a page the router does not serve.",
                ja: "`App::new` にはアプリの識別子を渡します。識別子は逆ドメイン形式で、`const` で使えばコンパイル時に検査されます。識別子からは `App::data_dir` が返すディレクトリが決まり、アプリのファイル、サーバーが待ち受けるポート、2 つめの起動を止めるロックがそこに置かれます。最初のページは `Route::url` が返す `Url` で指定するので、ルーターが配信しないページはウィンドウで開けません。",
            },
        ),
    ])
}

fn system(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Reach the windows and the system from a handler",
                ja: "ハンドラからウィンドウと OS の機能を使う",
            },
        ),
        para(
            site,
            L {
                en: "The closure receives an `Os`, which the setup puts in the router's state. A handler opens a window with `os.windows().open`, shows a file dialog with `os.dialog()`, reads and writes the clipboard's text with `os.clipboard()`, and opens a URL in the person's browser with `os.open_external`. The page can call only the actions the router declares, so there is no separate list of allowed commands to maintain. Every window shows a page of the same server, so two windows whose pages open the same broadcast-fed stream stay in step, as two browser tabs do.",
                ja: "クロージャは `Os` を受け取り、セットアップがそれをルーターの状態に入れます。ハンドラは `os.windows().open` でウィンドウを開き、`os.dialog()` でファイルのダイアログを出し、`os.clipboard()` でクリップボードのテキストを読み書きし、`os.open_external` で URL を利用者のブラウザで開きます。ブラウザから呼べるのはルーターが宣言したアクションだけなので、ページが呼べるコマンドの一覧を管理する必要はありません。どのウィンドウも同じサーバーのページを表示するので、同じ broadcast から流れるストリームを開いた 2 つのウィンドウは、ブラウザの 2 つのタブと同じように同じ状態を表示します。",
            },
        ),
        code::block(Lang::Rust, None, OPEN),
    ])
}

fn testing(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Test without a window",
                ja: "ウィンドウを開かずにテストする",
            },
        ),
        para(
            site,
            L {
                en: "With the `testing` feature, `hypersig_desktop::testing::FakeOs` stands in for the system: its dialogs answer what the test queued, and the test reads back the windows, the clipboard, and the URLs the handlers opened. `Page::simulate` clicks through the page, `Effects::calls` gives the arguments of the action a click sent, and `Simulation::apply` applies the batch the handler returns. The whole round trip runs under `cargo test`, with no web view and no WebDriver.",
                ja: "`testing` feature を有効にすると、`hypersig_desktop::testing::FakeOs` を OS の代わりに使えます。ダイアログはテストがあらかじめ決めた答えを返し、テストはハンドラが開いたウィンドウ、クリップボード、URL をあとから読めます。`Page::simulate` でページをクリックし、`Effects::calls` でクリックが送ったアクションの引数を取り出し、`Simulation::apply` でハンドラが返した patch を適用します。この往復はすべて `cargo test` の中で動き、Web ビューも WebDriver も使いません。",
            },
        ),
        code::block(Lang::Rust, None, TEST),
    ])
}

fn packaging(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Package and update the application",
                ja: "パッケージを作り、更新を配る",
            },
        ),
        para(
            site,
            L {
                en: "`cargo packager -p <package> --release` builds the packages from a `[package.metadata.packager]` table: an .app and a .dmg on macOS, an NSIS installer on Windows, and a .deb, an AppImage, and a pacman package on Linux. The identifier should be the one `App::new` names, since macOS keys the web view's data by it. With `CARGO_PACKAGER_SIGN_PRIVATE_KEY` set, it also signs each package for updates.",
                ja: "`cargo packager -p <パッケージ名> --release` は、`[package.metadata.packager]` の設定からパッケージを作ります。macOS では .app と .dmg、Windows では NSIS のインストーラー、Linux では .deb、AppImage、pacman のパッケージができます。macOS は Web ビューのデータを識別子ごとに分けるので、ここの識別子は `App::new` に渡したものと同じにします。`CARGO_PACKAGER_SIGN_PRIVATE_KEY` を設定すると、更新のための署名も各パッケージに付けます。",
            },
        ),
        code::block(Lang::Toml, None, PACKAGER),
        para(
            site,
            L {
                en: "With the `updater` feature, `App::updates` names the endpoint that announces new versions and the public key that signs them. `os.updates()` asks that endpoint for a newer version, `Update::download` fetches it and refuses a package whose signature the key does not verify, and `Downloaded::install` puts it over the running application.",
                ja: "`updater` feature を有効にすると、`App::updates` で新しいバージョンを知らせるエンドポイントと、署名を確かめる公開鍵を指定できます。`os.updates()` がそのエンドポイントに新しいバージョンを問い合わせ、`Update::download` がパッケージを取得して、公開鍵で署名を確かめられないものを拒否します。`Downloaded::install` はそのパッケージを動いているアプリの上にインストールします。",
            },
        ),
    ])
}

fn limits(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the window allows",
                ja: "ウィンドウの制約",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The server answers only the application's windows: it refuses a request whose `Host` is not `127.0.0.1:<port>`, and one without the session cookie the first window receives when it opens.",
                    ja: "サーバーはアプリのウィンドウにしか応答しません。`Host` が `127.0.0.1:<port>` でない要求と、最初のウィンドウが開くときに受け取るセッションの cookie を持たない要求は拒否します。",
                },
                L {
                    en: "The windows reach the server over HTTP/1.1, whose six connections per host they share. A stream holds its connection open, so keep at most five streams open across all windows.",
                    ja: "ウィンドウは HTTP/1.1 でサーバーにつなぐので、ホストあたり 6 本の接続をすべてのウィンドウで共有します。ストリームは接続を開いたままにするので、ウィンドウ全体で開いておくストリームは 5 本までにします。",
                },
                L {
                    en: "WKWebView does not send a `Secure` cookie to `http://127.0.0.1`, so a session layer sets its cookie without `Secure` in the desktop build.",
                    ja: "WKWebView は `http://127.0.0.1` に `Secure` の cookie を送りません。セッションを使うアプリは、デスクトップ版では cookie から `Secure` を外します。",
                },
                L {
                    en: "Windows needs the WebView2 Runtime, which Windows 11 includes, and Linux needs WebKitGTK 4.1.",
                    ja: "Windows では WebView2 Runtime が必要です（Windows 11 には含まれています）。Linux では WebKitGTK 4.1 が必要です。",
                },
            ],
        ),
    ])
}
