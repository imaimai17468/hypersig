//! The page on what the windows share and what each system needs.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

/// The packages CI installs on Ubuntu to build `hypersig_desktop`.
const LINUX_BUILD: &str =
    "sudo apt-get install -y --no-install-recommends libwebkit2gtk-4.1-dev libdbus-1-dev";

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Desktop(DesktopPage::Limits).leads()),
        heading(
            site,
            L {
                en: "At most five open streams",
                ja: "開いておけるストリームは 5 本まで",
            },
        ),
        para(
            site,
            L {
                en: "The windows reach the server over HTTP/1.1, and a web view keeps at most six connections to one host for all its windows together. A stream, such as a [`StreamAction`](api:hypersig/struct.StreamAction.html) a page opens, holds its connection for as long as the page is open. With six windows that each hold a stream, a seventh request waits until one of them closes. In WKWebView, the todo example's sixth window left the first window's actions unanswered.",
                ja: "ウィンドウは HTTP/1.1 でサーバーに接続し、Web ビューは 1 つのホストへの接続を、すべてのウィンドウを合わせて 6 本までしか持ちません。ページが開く [`StreamAction`](api:hypersig/struct.StreamAction.html) のようなストリームは、ページが開いている間ずっと接続を 1 本使います。6 つのウィンドウがそれぞれストリームを持つと、7 つめのリクエストはどれかが閉じるまで待ちます。WKWebView では、todo の例で 6 つめのウィンドウを開くと、最初のウィンドウのアクションに応答が返らなくなりました。",
            },
        ),
        para(
            site,
            L {
                en: "Keep at most five streams open across all the windows, so one connection stays free for page loads and action calls. Nothing counts the streams: a sixth one compiles and opens, and the requests after it wait.",
                ja: "すべてのウィンドウを合わせて、開いておくストリームは 5 本までにします。そうすれば、ページの読み込みとアクションの呼び出しに接続が 1 本残ります。ストリームの数は数えていません。6 本めもコンパイルを通って開き、そのあとのリクエストが待たされます。",
            },
        ),
        heading(
            site,
            L {
                en: "What each system needs",
                ja: "OS ごとに必要なもの",
            },
        ),
        table_of(
            site,
            L {
                en: &["System", "Web view", "Needed to run"][..],
                ja: &["OS", "Web ビュー", "実行に必要なもの"][..],
            },
            L {
                en: &[
                    &["macOS", "WKWebView", "nothing beyond the system"][..],
                    &[
                        "Windows",
                        "WebView2",
                        "the WebView2 Runtime, which Windows 11 includes",
                    ],
                    &["Linux", "WebKitGTK", "WebKitGTK 4.1"],
                ][..],
                ja: &[
                    &["macOS", "WKWebView", "OS のほかには不要"][..],
                    &[
                        "Windows",
                        "WebView2",
                        "WebView2 Runtime（Windows 11 には含まれています）",
                    ],
                    &["Linux", "WebKitGTK", "WebKitGTK 4.1"],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "Building on Linux needs the development files of WebKitGTK 4.1 and D-Bus, since hypersig_desktop links GTK and WebKitGTK there and the window reads the desktop's color scheme over D-Bus. On Ubuntu, CI installs these packages before it builds:",
                ja: "Linux でビルドするには、WebKitGTK 4.1 と D-Bus の開発用ファイルが必要です。Linux の hypersig_desktop は GTK と WebKitGTK をリンクし、ウィンドウはデスクトップの配色を D-Bus で読み取るからです。CI は Ubuntu で、ビルドの前に次のパッケージをインストールしています。",
            },
        ),
        code::block(Lang::Shell, None, LINUX_BUILD),
        para(
            site,
            L {
                en: "CI builds and runs the examples on macOS, Windows, and Ubuntu, the latest runner image of each.",
                ja: "CI は macOS、Windows、Ubuntu のそれぞれ最新のランナーイメージで、例をビルドして実行しています。",
            },
        ),
        heading(
            site,
            L {
                en: "Other limits",
                ja: "そのほかの制約",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`App::run` takes the thread that calls it for the windows and their event loop, and on macOS it panics unless that is the main thread. Call it from `main`, which is not `#[tokio::main]`; `App::run` starts the Tokio runtime itself.",
                    ja: "`App::run` は、呼び出したスレッドをウィンドウとそのイベントループに使います。macOS では、それがメインスレッドでなければパニックします。`App::run` は `#[tokio::main]` を付けない `main` から呼びます。Tokio のランタイムは `App::run` が自分で起動します。",
                },
                L {
                    en: "Closing the last window closes the application, on macOS as well. The server is then given five seconds to finish the requests it is answering, and `App::run` returns once it has stopped.",
                    ja: "最後のウィンドウを閉じるとアプリが終了します。macOS でも同じです。そのあとサーバーには応答中のリクエストを終えるために 5 秒が与えられ、サーバーが止まると `App::run` が戻ります。",
                },
                L {
                    en: "A window is shown once its page has loaded. A page that has not loaded in 10 seconds is shown anyway, and an error is logged.",
                    ja: "ウィンドウはページの読み込みが終わってから表示されます。10 秒たっても読み込みが終わらないページは、そのまま表示され、エラーが記録されます。",
                },
                L {
                    en: "The web inspector is on in a debug build and off in a release build.",
                    ja: "Web インスペクタはデバッグビルドでは使え、リリースビルドでは使えません。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        para(
            site,
            L {
                en: "[Streams](learn/streams/) covers the `StreamAction` that holds a connection. [Smoke runs in CI](desktop/smoke-runs/) opens the real window on each system in CI.",
                ja: "接続を使い続ける `StreamAction` については [ストリーム](learn/streams/) で説明しています。CI の各 OS で本物のウィンドウを開く方法は [CI でのスモークテスト](desktop/smoke-runs/) にあります。",
            },
        ),
    ])
}
