//! The page on the clipboard of `Os::clipboard`: `Clipboard::read_text`,
//! `Clipboard::write_text`, and what their errors and `None` mean.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Clipboard).leads()),
        api(site),
        example(site),
        linux(site),
        next(site),
    ])
}

/// `Os::clipboard` and its two methods.
fn api(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Reading and writing text",
                ja: "テキストを読み書きする",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Os {\n    pub fn clipboard(&self) -> Clipboard<'_>;\n}\n\nimpl Clipboard<'_> {\n    pub async fn read_text(&self) -> Result<Option<String>, Error>;\n    pub async fn write_text(&self, text: impl Into<String>) -> Result<(), Error>;\n}",
        ),
        para(
            site,
            L {
                en: "[`Os::clipboard`](api:hypersig_desktop/struct.Os.html#method.clipboard) returns a [`Clipboard`](api:hypersig_desktop/struct.Clipboard.html) that borrows the `Os`. It is `Copy`, and a handler usually calls one method on it right away, as in `state.os.clipboard().write_text(text).await`. Both methods handle text only.",
                ja: "[`Os::clipboard`](api:hypersig_desktop/struct.Os.html#method.clipboard) は、`Os` を借用する [`Clipboard`](api:hypersig_desktop/struct.Clipboard.html) を返します。`Clipboard` は `Copy` で、ハンドラでは `state.os.clipboard().write_text(text).await` のように、受け取ってすぐにメソッドを 1 つ呼ぶのが普通です。どちらのメソッドもテキストだけを扱います。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Call", "Returns"],
                ja: &["呼び出し", "戻り値"],
            },
            L {
                en: &[
                    &["`read_text`", "`Ok(Some(text))` with the clipboard's text"],
                    &["`read_text`", "`Ok(None)` when the clipboard holds no text"],
                    &["`write_text`", "`Ok(())` once `text` is on the clipboard"],
                    &[
                        "either",
                        "`Err` whose `kind()` is `ErrorKind::Clipboard` when the system clipboard cannot be opened, read, or written",
                    ],
                ][..],
                ja: &[
                    &[
                        "`read_text`",
                        "クリップボードのテキストを持つ `Ok(Some(text))`",
                    ],
                    &[
                        "`read_text`",
                        "クリップボードにテキストがないときは `Ok(None)`",
                    ],
                    &["`write_text`", "`text` をクリップボードに置いたら `Ok(())`"],
                    &[
                        "どちらも",
                        "OS のクリップボードを開けない、読めない、書けないときは、`kind()` が `ErrorKind::Clipboard` の `Err`",
                    ],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "The error is an [`Error`](api:hypersig_desktop/struct.Error.html), whose `Display` names the cause, and [`ErrorKind::Clipboard`](api:hypersig_desktop/enum.ErrorKind.html#variant.Clipboard) tells it apart from the other kinds. A clipboard without text is `Ok(None)` rather than an error, so a paste handler answers it with a message of its own.",
                ja: "エラーは [`Error`](api:hypersig_desktop/struct.Error.html) で、`Display` が原因を示します。ほかの種類のエラーとは [`ErrorKind::Clipboard`](api:hypersig_desktop/enum.ErrorKind.html#variant.Clipboard) で見分けます。テキストがないクリップボードはエラーではなく `Ok(None)` になるので、貼り付けのハンドラはそれに専用のメッセージで応じます。",
            },
        ),
    ])
}

/// Copy and paste handlers.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Copy and paste buttons",
                ja: "コピーと貼り付けのボタン",
            },
        ),
        para(
            site,
            L {
                en: "The clipboard is read and written by the handler, in the application's process. The page's text reaches the handler as the action's argument, `COPY.call_with(Editor::text())`, and the clipboard's text reaches the page as the patch the handler returns. The page itself does not read or write the clipboard. The `Os` is reached the same way as for a dialog, from the router's state:",
                ja: "クリップボードを読み書きするのはハンドラで、アプリのプロセスの中で動きます。ページのテキストは `COPY.call_with(Editor::text())` のようにアクションの引数としてハンドラに届き、クリップボードのテキストはハンドラが返す patch としてページに届きます。ページ自身はクリップボードを読み書きしません。`Os` はダイアログのときと同じく、ルーターの状態から取り出します。",
            },
        ),
        show(Snippet::DesktopClipboard),
        para(
            site,
            L {
                en: "examples/notes has the same Copy button. In a test, `FakeOs` keeps a clipboard of its own: `set_clipboard_text` puts text on it as another program would, and `clipboard_text` returns what the handler wrote.",
                ja: "examples/notes にも同じコピーのボタンがあります。テストでは `FakeOs` が専用のクリップボードを持ちます。`set_clipboard_text` はほかのプログラムがするようにテキストを置き、`clipboard_text` はハンドラが書いたテキストを返します。",
            },
        ),
    ])
}

/// How long text written on Linux stays available.
fn linux(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Text written on Linux",
                ja: "Linux で書いたテキスト",
            },
        ),
        para(
            site,
            L {
                en: "On Linux, under X11 and Wayland alike, the text on the clipboard is served by the program that wrote it, so another program reads it from that program when it pastes. `hypersig_desktop` keeps the clipboard open from the first call until the application closes, so text written once stays available for the whole run. Once the application has closed, that text may no longer be available to other programs.",
                ja: "Linux では、X11 でも Wayland でも、クリップボードのテキストはそれを書いたプログラムが提供します。ほかのプログラムが貼り付けるときは、書いたプログラムからテキストを読み出します。`hypersig_desktop` は最初の呼び出しからアプリが終わるまでクリップボードを開いたままにするので、一度書いたテキストはアプリが動いているあいだずっと読み出せます。アプリが終わったあとは、ほかのプログラムからそのテキストを読み出せなくなることがあります。",
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
                    en: "[Testing without a window](desktop/testing/): the fake clipboard of `FakeOs`, read and written by a test.",
                    ja: "[ウィンドウなしのテスト](desktop/testing/): テストから読み書きできる `FakeOs` のクリップボード。",
                },
                L {
                    en: "[Links to other sites](desktop/external-links/): `Os::open_external` and what a window does with a link off the application.",
                    ja: "[ほかのサイトへのリンク](desktop/external-links/): `Os::open_external` と、アプリの外へのリンクをウィンドウがどう扱うか。",
                },
            ],
        ),
    ])
}
