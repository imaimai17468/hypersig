//! The page on testing the handlers and pages with `FakeOs`: the `testing`
//! feature, the answers a test gives the dialogs, what it reads back, and a
//! test that clicks through a page with `Page::simulate`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of};

/// The dev-dependency that turns the `testing` feature on.
const DEV_DEPENDENCY: &str = r#"[dependencies]
hypersig_desktop = "0.15"

[dev-dependencies]
hypersig_desktop = { version = "0.15", features = ["testing"] }"#;

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Testing).leads()),
        feature(site),
        dialogs(site),
        reads(site),
        example(site),
        unchecked(site),
        next(site),
    ])
}

/// The `testing` feature and `FakeOs::os`.
fn feature(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A fake system for the handlers",
                ja: "ハンドラに渡す偽の OS",
            },
        ),
        para(
            site,
            L {
                en: "`hypersig_desktop::testing` exists only with the `testing` feature. Turn it on under `[dev-dependencies]`, as examples/notes does, so that `cargo test` builds it and the application's own build does not:",
                ja: "`hypersig_desktop::testing` は `testing` feature を有効にしたときだけ使えます。examples/notes と同じように `[dev-dependencies]` で有効にすると、`cargo test` のビルドにだけ入り、アプリ自体のビルドには入りません。",
            },
        ),
        code::block(Lang::Toml, Some("Cargo.toml"), DEV_DEPENDENCY),
        code::block(
            Lang::Rust,
            None,
            "impl FakeOs {\n    pub fn new() -> Self;\n    pub fn os(&self) -> Os;\n}",
        ),
        para(
            site,
            L {
                en: "[`FakeOs::new`](api:hypersig_desktop/testing/struct.FakeOs.html#method.new) makes a fake system with no answers, no clipboard text, and no windows. `os()` returns an `Os` to put in the router's state where `App::run` would give the real one, so the handlers are the same functions in the application and in the test. Every `Os` from one `FakeOs`, and every clone of either, shares the same answers and records. Nothing on screen changes: no dialog, window, or browser opens, and the system clipboard is left alone.",
                ja: "[`FakeOs::new`](api:hypersig_desktop/testing/struct.FakeOs.html#method.new) は、答えもクリップボードのテキストもウィンドウもない偽の OS を作ります。`os()` が返す `Os` を、`App::run` が本物を渡すのと同じ場所、つまりルーターの状態に入れます。そのため、アプリとテストで同じハンドラの関数を使えます。1 つの `FakeOs` から得たどの `Os` も、そのどちらのクローンも、同じ答えと記録を共有します。画面には何も起きません。ダイアログもウィンドウもブラウザも開かず、OS のクリップボードにも触れません。",
            },
        ),
    ])
}

/// `answer_dialog`, `cancel_dialog`, the panics of a dialog answered wrongly,
/// and `dialogs`.
fn dialogs(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Answering the dialogs",
                ja: "ダイアログに答える",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl FakeOs {\n    pub fn answer_dialog(&self, paths: impl IntoIterator<Item = PathBuf>);\n    pub fn cancel_dialog(&self);\n    pub fn dialogs(&self) -> Vec<ShownDialog>;\n}",
        ),
        para(
            site,
            L {
                en: "A test gives the answers before the handler shows its dialogs. `answer_dialog` answers the next dialog as if the person chose `paths`, and `cancel_dialog` as if they cancelled it. The dialogs take the answers in the order they were given, whichever of the four methods shows them:",
                ja: "テストは、ハンドラがダイアログを表示する前に答えを渡しておきます。`answer_dialog` は利用者が `paths` を選んだものとして次のダイアログに答え、`cancel_dialog` は利用者がキャンセルしたものとして答えます。どのダイアログも、4 つのメソッドのどれで表示されたかにかかわらず、渡された順に答えを受け取ります。",
            },
        ),
        table_of(
            site,
            L {
                en: &[
                    "Method",
                    "After `answer_dialog(paths)`",
                    "After `cancel_dialog()`",
                ],
                ja: &[
                    "メソッド",
                    "`answer_dialog(paths)` のあと",
                    "`cancel_dialog()` のあと",
                ],
            },
            L {
                en: &[
                    &["`open_file`", "The one path of `paths`", "`None`"],
                    &[
                        "`open_files`",
                        "All of `paths`, which may be none",
                        "`None`",
                    ],
                    &["`pick_folder`", "The one path of `paths`", "`None`"],
                    &["`save_file`", "The one path of `paths`", "`None`"],
                ][..],
                ja: &[
                    &["`open_file`", "`paths` の 1 つのパス", "`None`"],
                    &["`open_files`", "`paths` のすべて（0 個でもよい）", "`None`"],
                    &["`pick_folder`", "`paths` の 1 つのパス", "`None`"],
                    &["`save_file`", "`paths` の 1 つのパス", "`None`"],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "Two mistakes in the answers panic, so the test fails where the mistake is. A dialog shown with no answer left panics with a message that names it by its number, as in `dialog 1 was shown, and the test gave FakeOs no answer for it with answer_dialog or cancel_dialog`. A dialog other than `open_files` answered with no path or with several panics too, since the person chooses exactly one there:",
                ja: "次の 2 つの場合はパニックします。テストの誤りを見逃さず、テストを失敗させるためです。答えが残っていないのにダイアログを表示すると、`dialog 1 was shown, and the test gave FakeOs no answer for it with answer_dialog or cancel_dialog` のように、何番目のダイアログかを示すメッセージでパニックします。`open_files` 以外のダイアログに、パスを 1 つも渡さないか 2 つ以上渡した場合もパニックします。利用者がそこで選ぶパスはちょうど 1 つだからです。",
            },
        ),
        show(Snippet::DesktopFakeOsUnanswered),
        para(
            site,
            L {
                en: "`dialogs` returns a [`ShownDialog`](api:hypersig_desktop/testing/struct.ShownDialog.html) for each dialog the handlers showed and the test answered, in order. Its `kind` is the method that showed it, as a [`DialogKind`](api:hypersig_desktop/testing/enum.DialogKind.html), and its `title`, `filters`, `directory`, and `file_name` are what the handler gave, so a test can check that a handler asked for `.md` files or proposed a file name. An empty `Vec` means the handler showed none:",
                ja: "`dialogs` は、ハンドラが表示し、テストが答えた各ダイアログの [`ShownDialog`](api:hypersig_desktop/testing/struct.ShownDialog.html) を、表示した順に返します。その `kind` はダイアログを表示したメソッドを [`DialogKind`](api:hypersig_desktop/testing/enum.DialogKind.html) で表し、`title`、`filters`、`directory`、`file_name` はハンドラが指定した値です。そのため、ハンドラが `.md` のファイルを求めたことや、ファイル名を提案したことをテストで確かめられます。空の `Vec` なら、ハンドラはダイアログを表示していません。",
            },
        ),
        show(Snippet::DesktopFakeOsDialogs),
    ])
}

/// What a test reads back: the clipboard, the windows, and the URLs.
fn reads(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Reading what the handlers did",
                ja: "ハンドラがしたことを読む",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl FakeOs {\n    pub fn clipboard_text(&self) -> Option<String>;\n    pub fn set_clipboard_text(&self, text: impl Into<String>);\n    pub fn open_windows(&self) -> Vec<Url>;\n    pub fn focused_window(&self) -> Option<Url>;\n    pub fn opened_externally(&self) -> Vec<SafeUrl>;\n    pub fn updater(&self) -> Updater;\n}",
        ),
        table_of(
            site,
            L {
                en: &["Method", "What it gives or does"],
                ja: &["メソッド", "返すもの、すること"],
            },
            L {
                en: &[
                    &[
                        "`clipboard_text`",
                        "The text on the fake clipboard: the last `write_text`, or `None` before any",
                    ],
                    &[
                        "`set_clipboard_text`",
                        "Puts `text` on the fake clipboard, as another program would, for `read_text` to return",
                    ],
                    &[
                        "`open_windows`",
                        "The `Url` of each window `Windows::open` opened and `Window::close` has not closed, in the order they opened",
                    ],
                    &[
                        "`focused_window`",
                        "The `Url` of the window `Windows::open` opened or `Window::focus` focused last, or `None` once it is closed, as a system window comes to the front when it is shown",
                    ],
                    &[
                        "`opened_externally`",
                        "Each `SafeUrl` passed to `Os::open_external`, in order. A URL without a scheme still fails with `ErrorKind::NotExternal` and is not listed",
                    ],
                ][..],
                ja: &[
                    &[
                        "`clipboard_text`",
                        "偽のクリップボードのテキスト。最後に `write_text` したもので、一度もなければ `None`",
                    ],
                    &[
                        "`set_clipboard_text`",
                        "ほかのプログラムがするように `text` を偽のクリップボードに置く。`read_text` はこれを返す",
                    ],
                    &[
                        "`open_windows`",
                        "`Windows::open` で開き、`Window::close` で閉じていない各ウィンドウの `Url`。開いた順",
                    ],
                    &[
                        "`focused_window`",
                        "`Windows::open` で最後に開いたか、`Window::focus` で最後にフォーカスしたウィンドウの `Url`。そのウィンドウを閉じたあとは `None`。OS のウィンドウは表示されると前面に出るため、開いたウィンドウも含む",
                    ],
                    &[
                        "`opened_externally`",
                        "`Os::open_external` に渡された各 `SafeUrl`。渡された順。スキームのない URL はここでも `ErrorKind::NotExternal` で失敗し、含まれない",
                    ],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "With the `updater` feature as well, `FakeOs::updater` returns an `Updater` for the handlers' state whose `check` returns `Ok(None)`: it finds no newer version and makes no request. The test below goes through each of these:",
                ja: "`updater` feature も有効にしている場合、`FakeOs::updater` はハンドラの状態に入れる `Updater` を返します。その `check` は `Ok(None)` を返し、新しいバージョンは見つからず、リクエストも送りません。下のテストはこれらを 1 つずつ試します。",
            },
        ),
        show(Snippet::DesktopFakeOsReads),
    ])
}

/// A test that clicks through a page and calls its handler with `FakeOs`.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Clicking through a page",
                ja: "ページをクリックして試す",
            },
        ),
        para(
            site,
            L {
                en: "The handlers take the fake `Os` from the state, and the framework's page testing does the rest. [`Page::simulate`](api:hypersig/struct.Page.html#method.simulate) starts the page from its initial values, a click returns [`Effects`](api:hypersig/testing/struct.Effects.html) whose [`calls`](api:hypersig/testing/struct.Effects.html#method.calls) lists the arguments of each action it sent, and [`Simulation::apply`](api:hypersig/testing/struct.Simulation.html#method.apply) applies the `Patches` the handler returned, as the page would apply its reply. The test below drives the viewer of [File dialogs](desktop/dialogs/): the page with its button marked by a `NodeRef`, and the `open` handler unchanged.",
                ja: "ハンドラは状態から偽の `Os` を受け取り、残りはフレームワークのページのテストで試せます。[`Page::simulate`](api:hypersig/struct.Page.html#method.simulate) はページを初期値から始め、クリックは [`Effects`](api:hypersig/testing/struct.Effects.html) を返します。その [`calls`](api:hypersig/testing/struct.Effects.html#method.calls) は、クリックが送った各アクションの引数を並べます。[`Simulation::apply`](api:hypersig/testing/struct.Simulation.html#method.apply) は、ページが応答を適用するのと同じように、ハンドラが返した `Patches` を適用します。下のテストは [ファイルのダイアログ](desktop/dialogs/) のビューアを動かします。ページのボタンには `NodeRef` を付け、`open` ハンドラはそのまま使います。",
            },
        ),
        show(Snippet::DesktopFakeOs),
        para(
            site,
            L {
                en: "In a test file, the body sits in a `#[tokio::test] async fn`, since the handlers are async. examples/notes tests its handlers this way: opening a file in a window of its own, a cancelled dialog, a save that reaches the list window over a stream, Save as, and Copy.",
                ja: "ハンドラは async なので、テストのファイルではこの本体を `#[tokio::test] async fn` の中に書きます。examples/notes もこの方法でハンドラをテストしています。ファイルを別のウィンドウで開く場合、ダイアログをキャンセルした場合、保存がストリームで一覧のウィンドウに届く場合、別名で保存する場合、コピーする場合です。",
            },
        ),
    ])
}

/// What a `FakeOs` does not record.
fn unchecked(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "What the fake does not check",
                ja: "偽の OS が確かめないもの",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "One queue of answers serves the four dialog methods, so an answer is not tied to a kind of dialog. A test checks the kind afterwards with `dialogs`.",
                    ja: "4 つのダイアログのメソッドが 1 つの答えの列を共有するので、答えはダイアログの種類に結び付きません。種類はあとから `dialogs` で確かめます。",
                },
                L {
                    en: "Which window is in front once the focused one closes. The system brings another forward, and `focused_window` returns `None`.",
                    ja: "フォーカスしていたウィンドウを閉じたあと、どのウィンドウが前面に出るか。OS は別のウィンドウを前面に出しますが、`focused_window` は `None` を返します。",
                },
                L {
                    en: "The window, the web view, and the HTTP between them are not involved. [Smoke runs in CI](desktop/smoke-runs/) start the real window for that.",
                    ja: "ウィンドウ、Web ビュー、そのあいだの HTTP は関わりません。それらを確かめるには、本物のウィンドウを開く [CI でのスモークテスト](desktop/smoke-runs/) を使います。",
                },
            ],
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
                    en: "[Testing handlers](learn/testing/): calling a handler and the router from a test.",
                    ja: "[ハンドラのテスト](learn/testing/): テストからハンドラとルーターを呼ぶ方法。",
                },
                L {
                    en: "[Testing a page without a browser](learn/testing-pages/): every event and read of `Page::simulate`.",
                    ja: "[ブラウザなしでページをテストする](learn/testing-pages/): `Page::simulate` のすべてのイベントと読み出し。",
                },
                L {
                    en: "[Smoke runs in CI](desktop/smoke-runs/): the real window, started by CI.",
                    ja: "[CI でのスモークテスト](desktop/smoke-runs/): CI が開く本物のウィンドウ。",
                },
            ],
        ),
    ])
}
