//! The page on the file dialogs of `Os::dialog`: the `Dialog` builder, its
//! four async methods, and a handler that answers the page with what the
//! person chose.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Dialogs).leads()),
        builder(site),
        methods(site),
        example(site),
        others(site),
        unchecked(site),
        next(site),
    ])
}

/// `Os::dialog` and the methods that set the dialog up.
fn builder(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Setting up a dialog",
                ja: "ダイアログを設定する",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Os {\n    pub fn dialog(&self) -> Dialog;\n}\n\nimpl Dialog {\n    pub fn title(self, title: impl Into<String>) -> Self;\n    pub fn filter(self, name: impl Into<String>, extensions: &[&str]) -> Self;\n    pub fn directory(self, directory: impl Into<PathBuf>) -> Self;\n    pub fn file_name(self, name: impl Into<String>) -> Self;\n}",
        ),
        para(
            site,
            L {
                en: "[`Os::dialog`](api:hypersig_desktop/struct.Os.html#method.dialog) returns a [`Dialog`](api:hypersig_desktop/struct.Dialog.html), which each method takes by value and returns, so the calls chain. Nothing appears on screen until one of the four async methods below is awaited. `Dialog` is `#[must_use]`, so a chain that ends without one of them is a compiler warning.",
                ja: "[`Os::dialog`](api:hypersig_desktop/struct.Os.html#method.dialog) は [`Dialog`](api:hypersig_desktop/struct.Dialog.html) を返します。各メソッドは `Dialog` を値で受け取って返すので、呼び出しをつなげて書けます。下の 4 つの async メソッドのどれかを await するまで、画面には何も表示されません。`Dialog` には `#[must_use]` が付いているので、そのどれも呼ばずに終わる呼び出しはコンパイラの警告になります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "What it sets"],
                ja: &["メソッド", "設定するもの"],
            },
            L {
                en: &[
                    &["`title`", "The title of the dialog's window."],
                    &[
                        "`filter`",
                        "A filter named `name` that offers the files whose extension is one of `extensions`. Each call adds one; examples/notes writes the extensions without the dot, as in `&[\"txt\", \"md\"]`.",
                    ],
                    &["`directory`", "The directory the dialog opens in."],
                    &[
                        "`file_name`",
                        "The file name filled in for a save, such as the name of the file being saved under another name.",
                    ],
                ][..],
                ja: &[
                    &["`title`", "ダイアログのウィンドウのタイトル。"],
                    &[
                        "`filter`",
                        "拡張子が `extensions` のどれかに当たるファイルを表示する、`name` という名前のフィルター。呼ぶたびに 1 つ増えます。examples/notes では `&[\"txt\", \"md\"]` のように拡張子をドットなしで書いています。",
                    ],
                    &[
                        "`directory`",
                        "ダイアログを開いたときに表示するディレクトリ。",
                    ],
                    &[
                        "`file_name`",
                        "保存するときにあらかじめ入れておくファイル名。別名で保存するときの元のファイル名などを入れます。",
                    ],
                ][..],
            },
        ),
    ])
}

/// The four async methods and what they return.
fn methods(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Showing the dialog",
                ja: "ダイアログを表示する",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Dialog {\n    pub async fn open_file(self) -> Option<PathBuf>;\n    pub async fn open_files(self) -> Option<Vec<PathBuf>>;\n    pub async fn pick_folder(self) -> Option<PathBuf>;\n    pub async fn save_file(self) -> Option<PathBuf>;\n}",
        ),
        table_of(
            site,
            L {
                en: &["Method", "The dialog", "`Some` holds"],
                ja: &["メソッド", "ダイアログ", "`Some` の中身"],
            },
            L {
                en: &[
                    &["`open_file`", "Open one file", "The file chosen"],
                    &[
                        "`open_files`",
                        "Open one or more files",
                        "Every file chosen",
                    ],
                    &["`pick_folder`", "Choose a folder", "The folder chosen"],
                    &["`save_file`", "Choose where to save", "The path to save at"],
                ][..],
                ja: &[
                    &["`open_file`", "1 つのファイルを開く", "選ばれたファイル"],
                    &[
                        "`open_files`",
                        "1 つ以上のファイルを開く",
                        "選ばれたすべてのファイル",
                    ],
                    &["`pick_folder`", "フォルダを選ぶ", "選ばれたフォルダ"],
                    &["`save_file`", "保存先を選ぶ", "保存先のパス"],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "Each method consumes the `Dialog`, shows the system's dialog, and resolves when the person closes it. It returns `None` when they cancelled, and the handlers below answer that case with a `let ... else` or a `match` before they use the path. `save_file` writes nothing. The file at the returned path may not exist yet, and the handler writes it, with `std::fs::write` for example.",
                ja: "どのメソッドも `Dialog` を消費して OS のダイアログを表示し、利用者がダイアログを閉じた時点で完了します。キャンセルされたときは `None` を返します。下の例のハンドラは、パスを使う前に、その場合を `let ... else` や `match` で扱います。`save_file` はファイルを書きません。返されたパスにはまだファイルがないこともあり、書き込むのはハンドラです。たとえば `std::fs::write` を使います。",
            },
        ),
        para(
            site,
            L {
                en: "The dialog is shown from a handler, so the action's HTTP request stays open while it is on screen. The patches the handler returns once the person has chosen are that request's response, and the page applies them as it applies any other action's reply.",
                ja: "ダイアログはハンドラから表示するので、ダイアログが画面にあるあいだ、アクションの HTTP リクエストは応答を待ったままになります。利用者が選んだあとにハンドラが返す patch がそのリクエストのレスポンスで、ページはほかのアクションの応答と同じようにそれを適用します。",
            },
        ),
    ])
}

/// A complete application whose handler opens a file.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A handler that opens a file",
                ja: "ファイルを開くハンドラ",
            },
        ),
        para(
            site,
            L {
                en: "The handler reaches the system through the [`Os`](api:hypersig_desktop/struct.Os.html) that [`App::run`](api:hypersig_desktop/struct.App.html#method.run) passes to its setup closure. The closure puts it in the router's state, and the handler reads it from there as `state.os`. `Os` is `Clone`, so a state struct that holds it can derive `Clone`, as axum requires of a router's state. Below, the page's button calls `OPEN`, and the handler answers with the file's text or the reason it could not be read:",
                ja: "ハンドラは、[`App::run`](api:hypersig_desktop/struct.App.html#method.run) がセットアップのクロージャに渡す [`Os`](api:hypersig_desktop/struct.Os.html) を通して OS の機能を使います。クロージャがそれをルーターの状態に入れ、ハンドラは `state.os` として読み出します。`Os` は `Clone` を実装しているので、`Os` を持つ状態の構造体にも `Clone` を derive できます。axum はルーターの状態に `Clone` を求めます。下の例では、ページのボタンが `OPEN` を呼び、ハンドラはファイルのテキストか、読めなかった理由を返します。",
            },
        ),
        show(Snippet::DesktopDialogOpen),
        para(
            site,
            L {
                en: "When the person cancels, the handler returns an empty `Patches` and the page stays as it was. The `open` handler of examples/notes answers a cancel by clearing its status line, and opens a chosen file in a window of its own with `Windows::open`.",
                ja: "利用者がキャンセルすると、ハンドラは空の `Patches` を返し、ページはそのまま変わりません。examples/notes の `open` ハンドラは、キャンセルされると状態の行を空にし、ファイルが選ばれると `Windows::open` でそのファイルを別のウィンドウに開きます。",
            },
        ),
    ])
}

/// The other three methods in handlers.
fn others(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Several files, a folder, and a place to save",
                ja: "複数のファイル、フォルダ、保存先",
            },
        ),
        para(
            site,
            L {
                en: "The other three methods are used the same way. `open_files` returns every file chosen, `pick_folder` starts in the directory `directory` names, and `save_file` fills in the name `file_name` gives:",
                ja: "残りの 3 つのメソッドも同じように使います。`open_files` は選ばれたすべてのファイルを返し、`pick_folder` は `directory` で指定したディレクトリから始まり、`save_file` には `file_name` で指定した名前が入った状態で表示されます。",
            },
        ),
        show(Snippet::DesktopDialogKinds),
    ])
}

/// What the dialog does not check.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What is not checked",
                ja: "検査されないもの",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The filters are handed to the system's dialog, and `hypersig_desktop` does not check the extension of the path it returns. A handler that needs a particular kind of file finds out when it reads the file, as the `Err` arm above does.",
                    ja: "フィルターは OS のダイアログに渡されるだけで、`hypersig_desktop` は返されたパスの拡張子を検査しません。特定の種類のファイルが必要なハンドラは、上の例の `Err` の分岐のように、ファイルを読んだ時点でそれを確かめます。",
                },
                L {
                    en: "A path returned by `open_file` can still fail to read, for example when the person lacks permission or the file is removed before the handler reads it, so the read returns a `Result` the handler answers.",
                    ja: "`open_file` が返したパスでも、読み込みに失敗することがあります。たとえば利用者に読む権限がない場合や、ハンドラが読む前にファイルが消された場合です。そのため、読み込みが返す `Result` にもハンドラで応じます。",
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
                    en: "[Testing without a window](desktop/testing/): `FakeOs` answers each dialog with paths the test chose, or cancels it.",
                    ja: "[ウィンドウなしのテスト](desktop/testing/): `FakeOs` は、テストが決めたパスで各ダイアログに答えるか、キャンセルします。",
                },
                L {
                    en: "[Opening windows](desktop/windows/): `Windows::open`, which examples/notes calls after its dialog.",
                    ja: "[ウィンドウを開く](desktop/windows/): examples/notes がダイアログのあとに呼ぶ `Windows::open` について。",
                },
                L {
                    en: "[The clipboard](desktop/clipboard/): the other system service a handler reaches through `Os`.",
                    ja: "[クリップボード](desktop/clipboard/): ハンドラが `Os` を通して使える、もう 1 つの OS の機能。",
                },
            ],
        ),
    ])
}
