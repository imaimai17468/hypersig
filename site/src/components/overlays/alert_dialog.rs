//! The Components page on `Dialog::alert_dialog`: a modal dialog that asks
//! the visitor to confirm or cancel.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::AlertDialog).leads()),
        demo::show(site, Demo::AlertDialog),
        accessibility(site),
        building(site),
        html(site),
        compile_errors(site),
        unchecked(site),
        example(site),
    ])
}

fn accessibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Accessibility",
                ja: "アクセシビリティ",
            },
        ),
        para(
            site,
            L {
                en: "An alert dialog is the same modal `<dialog>` as a [Dialog](components/dialog/), with `role=\"alertdialog\"`. A screen reader announces it as an alert dialog, which tells the visitor that it needs an answer, and reads its title and its description as a dialog's.",
                ja: "アラートダイアログは [ダイアログ](components/dialog/) と同じモーダルの `<dialog>` で、`role=\"alertdialog\"` を持ちます。スクリーンリーダーはアラートダイアログとして読み上げるので、訪問者は答えが必要なことが分かります。タイトルと説明文の読み上げ方はダイアログと同じです。",
            },
        ),
        para(
            site,
            L {
                en: "It has no close button in its corner, and a click on the backdrop leaves it open, so the visitor answers with one of the buttons in its content. Escape still cancels it, as the WAI-ARIA alert dialog pattern asks.",
                ja: "角の閉じるボタンはなく、背景をクリックしても閉じないので、訪問者は中身のボタンのどれかで答えます。WAI-ARIA の alert dialog パターンに従い、Escape では取り消せます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            L {
                en: &[
                    &[
                        "Tab, Shift+Tab",
                        "Moves between the dialog's buttons and never reaches the page behind",
                    ],
                    &[
                        "Escape",
                        "Closes the dialog without running a button's statement, and returns focus to the element that opened it",
                    ],
                ],
                ja: &[
                    &[
                        "Tab、Shift+Tab",
                        "ダイアログのボタンの間を移り、後ろのページには移りません",
                    ],
                    &[
                        "Escape",
                        "ボタンの文を実行せずにダイアログを閉じ、ダイアログを開いた要素にフォーカスを戻します",
                    ],
                ],
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building an alert dialog",
                ja: "アラートダイアログを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`Dialog::alert_dialog`](api:hypersig_ui/struct.Dialog.html#method.alert_dialog) is a method of the same `Dialog<S>` handle, and `trigger`, `close`, `open`, and `dismiss` work on it as on a [Dialog](components/dialog/):",
                ja: "[`Dialog::alert_dialog`](api:hypersig_ui/struct.Dialog.html#method.alert_dialog) は同じ `Dialog<S>` のハンドルのメソッドで、`trigger`、`close`、`open`、`dismiss` は [ダイアログ](components/dialog/) と同じように使えます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Dialog<S> {
    pub fn alert_dialog(
        self,
        title: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<Node<S>>,
    ) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "`title` asks the question, such as \"Delete this project?\", and `description` says what happens. `content` holds the answers, usually a `dialog_footer` with a `close` button that cancels and a `close` button whose `on_click` runs the action. The action's button carries the action's name, such as \"Delete\", where \"OK\" would make the visitor read the question again.",
                ja: "`title` には「このプロジェクトを削除しますか？」のような問いを、`description` には何が起きるかを書きます。`content` には答えのボタンを入れます。ふつうは `dialog_footer` に、取り消す `close` のボタンと、`on_click` で操作を実行する `close` のボタンを並べます。操作のボタンには「削除」のように操作の名前を付けます。「OK」では、訪問者は問いを読み直すことになります。",
            },
        ),
    ])
}

fn html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTML",
                ja: "出力される HTML",
            },
        ),
        para(
            site,
            L {
                en: "The example at the end of this page renders this dialog, with the classes left out. Each `…` is an id made from the dialog ref's name. Unlike a dialog's, it has `role=\"alertdialog\"`, no `closedby`, and no button after the content:",
                ja: "このページの最後の例は次のダイアログを出力します（クラスは省いています）。`…` はどれもダイアログ参照の名前から作った id です。ダイアログと違い、`role=\"alertdialog\"` を持ち、`closedby` も中身のあとのボタンもありません。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<dialog id="…" aria-labelledby="…" aria-describedby="…" role="alertdialog">
  <div>
    <div>
      <h2 id="…">Delete this project?</h2>
      <p id="…">The project and its deployments are removed. This cannot be undone.</p>
    </div>
    <div>
      <button type="button" commandfor="…" command="close">Cancel</button>
      <button type="button" commandfor="…" command="close">Delete</button>
    </div>
  </div>
</dialog>"#,
        ),
    ])
}

fn compile_errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What fails to compile",
                ja: "コンパイルエラーになるもの",
            },
        ),
        para(
            site,
            L {
                en: "An alert dialog has no corner button, so `alert_dialog` takes no `DialogText`. Passing one, as to `Dialog::dialog`, is E0061, wrong number of arguments:",
                ja: "アラートダイアログには角のボタンがないので、`alert_dialog` は `DialogText` を受け取りません。`Dialog::dialog` と同じように渡すと、E0061（引数の数の誤り）になります。",
            },
        ),
        show(Snippet::UiAlertDialogText),
        para(
            site,
            L {
                en: "The handle is the same `Dialog<S>`, so a `NodeRef` in place of the `DialogRef` is E0308 here too, as the [Dialog](components/dialog/) page shows.",
                ja: "ハンドルは同じ `Dialog<S>` なので、`DialogRef` の代わりに `NodeRef` を渡すと、ここでも E0308 になります。例は [ダイアログ](components/dialog/) のページにあります。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What is not checked",
                ja: "検査されないこと",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks that `content` holds a button. An alert dialog without one can be left only with Escape, and a visitor without a keyboard cannot leave it at all. As with a dialog, one handle's `alert_dialog` placed twice makes `Page::render` panic.",
                ja: "`content` にボタンがあるかどうかは検査されません。ボタンのないアラートダイアログは Escape でしか閉じられず、キーボードを使わない訪問者は閉じられません。ダイアログと同じく、1 つのハンドルの `alert_dialog` を 2 回置くと `Page::render` が panic します。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: confirming a delete",
                ja: "例: 削除を確認する",
            },
        ),
        para(
            site,
            L {
                en: "Cancel closes the dialog, and Delete sets a signal before closing it:",
                ja: "Cancel はダイアログを閉じ、Delete はシグナルを設定してから閉じます。",
            },
        ),
        show(Snippet::UiAlertDialog),
        para(
            site,
            L {
                en: "Where the action can be undone, a [Toast](components/toast/) after the action asks less of the visitor than a dialog before it. A dialog the visitor can dismiss is a [Dialog](components/dialog/).",
                ja: "取り消せる操作なら、操作の前にダイアログで確認するより、操作のあとに [トースト](components/toast/) を出すほうが訪問者の手間が少なくなります。訪問者が自由に閉じられるダイアログには [ダイアログ](components/dialog/) を使います。",
            },
        ),
    ])
}
