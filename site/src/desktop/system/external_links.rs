//! The page on links to other sites: what a window does with a URL outside
//! the application, and `Os::open_external`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::ExternalLinks).leads()),
        links(site),
        open_external(site),
        types(site),
        example(site),
        next(site),
    ])
}

/// What a window does with a URL outside the application.
fn links(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Links in a page",
                ja: "ページの中のリンク",
            },
        ),
        para(
            site,
            L {
                en: "A window shows only the pages of the application's own server, `http://127.0.0.1:<port>/`. When a page asks the window to go to any other URL, the window stays on the page it shows and hands the URL to the system, which opens it in the program it opens such URLs with. The same holds for a link with `target=\"_blank\"` and for `window.open`: a URL of the application opens in a new window of the application, and any other URL goes to the system.",
                ja: "ウィンドウが表示するのは、アプリ自身のサーバー `http://127.0.0.1:<port>/` のページだけです。ページがそれ以外の URL への移動をウィンドウに求めると、ウィンドウは今のページに留まり、その URL を OS に渡します。OS はその種類の URL を開くプログラムで開きます。`target=\"_blank\"` のリンクと `window.open` も同じです。アプリの URL はアプリの新しいウィンドウで開き、それ以外の URL は OS に渡ります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["The URL", "What happens"],
                ja: &["URL", "結果"],
            },
            L {
                en: &[
                    &[
                        "A page of the application",
                        "The window shows it. With `target=\"_blank\"` or `window.open`, a new window shows it.",
                    ],
                    &[
                        "`http:` or `https:` on another host or port",
                        "The person's default browser opens it.",
                    ],
                    &["`mailto:`", "The person's mail program opens it."],
                    &[
                        "Any other scheme",
                        "Nothing opens, and `hypersig_desktop` logs a warning with `tracing`.",
                    ],
                ][..],
                ja: &[
                    &[
                        "アプリのページ",
                        "そのウィンドウに表示します。`target=\"_blank\"` か `window.open` なら新しいウィンドウに表示します。",
                    ],
                    &[
                        "ほかのホストかポートの `http:` か `https:`",
                        "利用者の既定のブラウザで開きます。",
                    ],
                    &["`mailto:`", "利用者のメールのプログラムで開きます。"],
                    &[
                        "それ以外のスキーム",
                        "何も開かず、`hypersig_desktop` が `tracing` で警告を記録します。",
                    ],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "A link built with `El::href` takes a `SafeUrl` for an address the router does not serve, and a `SafeUrl` has the scheme `http:`, `https:`, `mailto:`, or none, so every link to another site built with `El::href` falls in the second or third row. [Values the types do not check](learn/unchecked-values/) covers `SafeUrl`.",
                ja: "`El::href` で作るリンクは、ルーターが配信しないアドレスには `SafeUrl` を受け取ります。`SafeUrl` のスキームは `http:`、`https:`、`mailto:` のどれかか、スキームなしです。そのため、`El::href` で作るほかのサイトへのリンクは、表の 2 行目か 3 行目に当たります。`SafeUrl` については [型で検査しない値](learn/unchecked-values/) で説明しています。",
            },
        ),
    ])
}

/// `Os::open_external`, its signature, and its errors.
fn open_external(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Opening a URL from a handler",
                ja: "ハンドラから URL を開く",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Os {\n    pub fn open_external(&self, url: &SafeUrl) -> Result<(), Error>;\n}",
        ),
        para(
            site,
            L {
                en: "[`Os::open_external`](api:hypersig_desktop/struct.Os.html#method.open_external) hands `url` to the system as a link does, for a URL the handler decides on, such as the page of a newer version or a form for reporting a problem. It is not async.",
                ja: "[`Os::open_external`](api:hypersig_desktop/struct.Os.html#method.open_external) は、リンクと同じように `url` を OS に渡します。新しいバージョンのページや問題を報告するフォームなど、開く URL をハンドラが決める場合に使います。async 関数ではありません。",
            },
        ),
        table_of(
            site,
            L {
                en: &["`url`", "Returns"],
                ja: &["`url`", "戻り値"],
            },
            L {
                en: &[
                    &[
                        "`http:`, `https:`, or `mailto:`, in any letter case",
                        "`Ok(())`, once the system has been asked to open it",
                    ],
                    &[
                        "No scheme, as in `/notes`, `#top`, or `//example.com`",
                        "`Err` whose `kind()` is `ErrorKind::NotExternal`",
                    ],
                    &[
                        "A URL the system fails to open",
                        "`Err` whose `kind()` is `ErrorKind::Io`",
                    ],
                ][..],
                ja: &[
                    &[
                        "`http:`、`https:`、`mailto:`（大文字小文字は問わない）",
                        "OS に開くよう求めたら `Ok(())`",
                    ],
                    &[
                        "`/notes`、`#top`、`//example.com` のようにスキームがないもの",
                        "`kind()` が `ErrorKind::NotExternal` の `Err`",
                    ],
                    &[
                        "OS が開けなかった URL",
                        "`kind()` が `ErrorKind::Io` の `Err`",
                    ],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "[`ErrorKind::NotExternal`](api:hypersig_desktop/enum.ErrorKind.html#variant.NotExternal) exists because the system would open a URL without a scheme as a path to a file. The check runs before anything is opened, so the URL never reaches the system:",
                ja: "[`ErrorKind::NotExternal`](api:hypersig_desktop/enum.ErrorKind.html#variant.NotExternal) があるのは、スキームのない URL を OS がファイルのパスとして開いてしまうからです。この検査は何かを開く前に行われるので、その URL が OS に渡ることはありません。",
            },
        ),
        show(Snippet::DesktopExternalRelative),
    ])
}

/// What compiles and what does not.
fn types(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the type rules out",
                ja: "型で防げること",
            },
        ),
        para(
            site,
            L {
                en: "`url` is a `&SafeUrl`, so a `&str` does not compile:",
                ja: "`url` は `&SafeUrl` なので、`&str` を渡すとコンパイルできません。",
            },
        ),
        show(Snippet::DesktopExternalStr),
        para(
            site,
            L {
                en: "A URL written in the program is a `const` made with `SafeUrl::new`, and a scheme that is not `http`, `https`, or `mailto`, such as `file:` or `javascript:`, is E0080 there:",
                ja: "プログラムに書く URL は `SafeUrl::new` で作る `const` にします。`file:` や `javascript:` のように `http`、`https`、`mailto` 以外のスキームを書くと、そこで E0080 になります。",
            },
        ),
        show(Snippet::DesktopExternalScheme),
        para(
            site,
            L {
                en: "A URL that arrives at run time, from a file or from what the person typed, goes through `SafeUrl::parse`, which returns `Err` for the same schemes. The host is not checked: a `SafeUrl` may name any site, and `open_external` opens whichever one it names.",
                ja: "ファイルから読んだ URL や利用者が入力した URL など、実行時に受け取る URL は `SafeUrl::parse` を通します。同じスキームに対して `Err` を返します。ホストは検査されません。`SafeUrl` はどのサイトでも指すことができ、`open_external` はそのサイトを開きます。",
            },
        ),
    ])
}

/// A page with a link and a button that opens a URL.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A link and a button",
                ja: "リンクとボタン",
            },
        ),
        para(
            site,
            L {
                en: "The page below has a link to the repository, which the browser opens when it is clicked, and a button whose handler opens the form for a new issue. The `Os` comes from the router's state, as on [File dialogs](desktop/dialogs/):",
                ja: "下のページにはクリックするとブラウザで開くリポジトリへのリンクと、新しい issue のフォームを開くハンドラを呼ぶボタンがあります。`Os` は [ファイルのダイアログ](desktop/dialogs/) と同じく、ルーターの状態から取り出します。",
            },
        ),
        show(Snippet::DesktopOpenExternal),
        para(
            site,
            L {
                en: "In a test, `FakeOs::opened_externally` returns each `SafeUrl` the handlers passed to `open_external`, and opens nothing. A link the person clicks is handled by the window, not by a handler, so `opened_externally` does not list it.",
                ja: "テストでは、`FakeOs::opened_externally` がハンドラが `open_external` に渡した `SafeUrl` を順に返し、実際には何も開きません。利用者がクリックしたリンクはハンドラではなくウィンドウが扱うので、`opened_externally` には含まれません。",
            },
        ),
    ])
}

fn next(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "[Opening windows](desktop/windows/): links and `window.open` to the application's own pages.",
                    ja: "[ウィンドウを開く](desktop/windows/): アプリ自身のページへのリンクと `window.open`。",
                },
                L {
                    en: "[Testing without a window](desktop/testing/): `FakeOs` and what it records.",
                    ja: "[ウィンドウなしのテスト](desktop/testing/): `FakeOs` と、それが記録するもの。",
                },
            ],
        ),
    ])
}
