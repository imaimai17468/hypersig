//! The page on why to use `hypersig_desktop`: the applications it suits,
//! what it gives them, and how its start-up and size compare with Tauri and
//! Electron.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::lang::{L, Text};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Why).leads()),
        suits(site),
        gives(site),
        benchmark(site),
        next(site),
    ])
}

fn suits(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The applications it suits",
                ja: "向いているアプリ",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A hypersig application served on the web that is also to ship as a desktop application. [`examples/counter`](https://github.com/imaimai17468/hypersig/tree/main/examples/counter) builds both from one package.",
                    ja: "Web で配信している hypersig のアプリを、デスクトップのアプリとしても配りたいとき。[`examples/counter`](https://github.com/imaimai17468/hypersig/tree/main/examples/counter) は 1 つのパッケージから両方をビルドします。",
                },
                L {
                    en: "A tool that works on the person's own files. [`examples/notes`](https://github.com/imaimai17468/hypersig/tree/main/examples/notes) opens Markdown files with the system's dialog, edits each in a window of its own, and saves it.",
                    ja: "利用者の手元のファイルを扱うツール。[`examples/notes`](https://github.com/imaimai17468/hypersig/tree/main/examples/notes) は OS のダイアログで Markdown のファイルを開き、ファイルごとにウィンドウを開いて編集し、保存します。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "It does not suit an application whose interface is written with a JavaScript framework such as React, since a window shows the pages the router renders. Nor does it suit one that needs the system's own controls, since a window holds only a web view, or one that runs on phones, since the crate builds for macOS, Windows, and Linux.",
                ja: "React のような JavaScript のフレームワークで画面を書くアプリには向きません。ウィンドウが表示するのはルーターが描画したページだからです。OS 自身の部品（ネイティブのコントロール）が必要なアプリにも向きません。ウィンドウの中身は Web ビューだけだからです。スマートフォンで動かすアプリにも向きません。このクレートがビルドできるのは macOS、Windows、Linux だけだからです。",
            },
        ),
    ])
}

fn gives(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What it gives an application",
                ja: "アプリにとっての利点",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "One codebase. The window shows the routes, pages, actions, and streams the application serves on the web, and only `main` changes.",
                    ja: "コードベースが 1 つで済みます。ウィンドウが表示するのは Web で配信しているルート、ページ、アクション、ストリームで、書き換えるのは `main` だけです。",
                },
                L {
                    en: "The same compile-time checks. Signal names, action URLs, and links are produced from Rust types in the window as on the web, so a mistake in one is still a compile error.",
                    ja: "コンパイル時の検査はウィンドウの中でも同じです。シグナル名、アクションの URL、リンクは Web と同じく Rust の型から作られるので、間違えればコンパイルエラーになります。",
                },
                L {
                    en: "No second list of what a page may call. A page calls only the actions the router declares, and a handler reaches files, the clipboard, and windows through `Os`.",
                    ja: "ページが呼べるものの一覧を別に書く必要がありません。ページが呼べるのはルーターが宣言したアクションだけで、ハンドラは `Os` を通してファイル、クリップボード、ウィンドウを扱います。",
                },
                L {
                    en: "Tests without a window. With the `testing` feature, `FakeOs` stands in for the system, so `cargo test` clicks through the pages and checks what the handlers did.",
                    ja: "ウィンドウを開かずにテストできます。`testing` feature を有効にすると `FakeOs` が OS の代わりになり、`cargo test` の中でページをクリックし、ハンドラがしたことを確かめられます。",
                },
                L {
                    en: "No browser in the package. The window is the system's web view: WKWebView on macOS, WebView2 on Windows, and WebKitGTK on Linux.",
                    ja: "パッケージにブラウザを含みません。ウィンドウは OS の Web ビューで、macOS では WKWebView、Windows では WebView2、Linux では WebKitGTK です。",
                },
            ],
        ),
    ])
}

fn benchmark(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Start-up and size",
                ja: "起動時間とサイズ",
            },
        ),
        para(site, CONDITIONS),
        results(site),
        para(site, READING),
        para(
            site,
            L {
                en: "Not measured yet:",
                ja: "まだ測っていないもの:",
            },
        ),
        not_measured(site),
    ])
}

/// The latest results of `bench/desktop`, which the Desktop landing page
/// shows too.
pub(crate) fn results(site: Site) -> Part {
    let header = &["", "hypersig_desktop", "Tauri", "Electron"];
    table_of(
        site,
        L {
            en: header,
            ja: header,
        },
        // The numbers read the same in both languages, and the first
        // cell of each row is picked with `site.t`.
        Text::Chosen::<[&[&str; 4]]>(&[
            &[
                site.t(L {
                    en: "First page shown, median",
                    ja: "最初のページの表示（中央値）",
                }),
                "482 ms",
                "455 ms",
                "489 ms",
            ],
            &[
                site.t(L {
                    en: "First page shown, p95",
                    ja: "最初のページの表示（p95）",
                }),
                "534 ms",
                "639 ms",
                "679 ms",
            ],
            &[
                site.t(L {
                    en: "Size",
                    ja: "サイズ",
                }),
                "4.7 MiB",
                "9.8 MiB",
                "269.9 MiB",
            ],
        ]),
    )
}

fn not_measured(site: Site) -> Part {
    bullets(
        site,
        &[
            L {
                en: "Memory. On macOS, the system starts WKWebView's processes rather than the application, so they cannot be counted from its process tree.",
                ja: "メモリ。macOS では WKWebView のプロセスをアプリではなく OS が起動するので、アプリのプロセスツリーから数えられません。",
            },
            L {
                en: "A cold start, after the system's caches are emptied.",
                ja: "OS のキャッシュを空にしたあとのコールドスタート。",
            },
            L {
                en: "Windows and Linux.",
                ja: "Windows と Linux。",
            },
            L {
                en: "Tauri with a front end built by a JavaScript framework.",
                ja: "JavaScript のフレームワークでビルドしたフロントエンドを持つ Tauri。",
            },
        ],
    )
}

const CONDITIONS: L<&str> = L {
    en: "The same todo list, built three ways: [`examples/todo`](https://github.com/imaimai17468/hypersig/tree/main/examples/todo) with `hypersig_desktop`, and the list written in plain JavaScript in Tauri 2.12.5 and in Electron 38.2.0. Each build is started 5 times to warm up, since macOS checks a newly built executable on its first starts, then 20 times, and the time runs from spawning the process to the window showing its first page. Size counts the executable, and for Electron the Electron distribution with the application's files. Measured on 2026-11-10 on an Apple M4 Max with macOS 26. The apps and the harness are in [`bench/desktop`](https://github.com/imaimai17468/hypersig/tree/main/bench/desktop).",
    ja: "同じ ToDo リストを 3 通りにビルドしました。[`examples/todo`](https://github.com/imaimai17468/hypersig/tree/main/examples/todo) を `hypersig_desktop` で動かしたものと、同じリストを素の JavaScript で書いて Tauri 2.12.5 と Electron 38.2.0 で動かしたものです。macOS はビルドしたばかりの実行ファイルを最初の数回の起動で検査するので、各ビルドを 5 回起動して慣らしたあと、20 回起動しました。時間はプロセスを起動してからウィンドウが最初のページを表示するまでです。サイズは実行ファイルの大きさで、Electron は Electron の配布物とアプリのファイルを合わせた大きさです。2026-11-10 に、macOS 26 の Apple M4 Max で測りました。測ったアプリと計測の仕組みは [`bench/desktop`](https://github.com/imaimai17468/hypersig/tree/main/bench/desktop) にあります。",
};

const READING: L<&str> = L {
    en: "The three show their first page within 35 ms of each other at the median, and Tauri's is the soonest. hypersig_desktop has the lowest p95. Its build is half the size of Tauri's and under a fiftieth of Electron's, whose distribution holds a browser.",
    ja: "中央値では 3 つとも 35 ms 以内の差で最初のページを表示し、最も早いのは Tauri です。p95 が最も小さいのは hypersig_desktop です。hypersig_desktop のビルドの大きさは Tauri の半分で、ブラウザを配布物に含む Electron の 50 分の 1 未満です。",
};

fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "Next, [Overview](desktop/overview/) adds the crate to an application and opens its first window.",
            ja: "次の [概要](desktop/overview/) ではアプリにこのクレートを追加し、最初のウィンドウを開きます。",
        },
    )
}
