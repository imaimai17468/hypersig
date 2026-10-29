//! The page on `checkbox`: a native checkbox inside its label, bound to a
//! `bool` signal.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Checkbox).leads()),
        demo::show(site, Demo::Checkbox),
        accessibility(site),
        function(site),
        unchecked(site),
        example(site),
        next(site),
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
                en: "It is a native `<input type=\"checkbox\">` inside the `<label>` that holds its text, so the text names it, a click on the text toggles it, and a screen reader announces \"I accept the terms, checkbox, not checked\". The input is transparent and lies over the box that is drawn, so the browser keeps its role, its keys, and its place in a form, and the drawn box follows its checked, focused, and disabled states.",
                ja: "テキストを入れた `<label>` の中のネイティブの `<input type=\"checkbox\">` です。そのため、テキストがチェックボックスの名前になり、テキストをクリックするとチェックが切り替わり、スクリーンリーダーは「I accept the terms、チェックボックス、チェックなし」のように読み上げます。入力要素は透明で、描画したボックスの上に重なっているので、ロール、キー操作、フォームの中での扱いはブラウザのものがそのまま残り、描画したボックスはチェック、フォーカス、無効の状態に従います。",
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
                    &["Tab", "Moves focus to the checkbox."],
                    &["Space", "Checks or unchecks it."],
                ],
                ja: &[
                    &["Tab", "チェックボックスにフォーカスを移します。"],
                    &["Space", "チェックを付けたり外したりします。"],
                ],
            },
        ),
    ])
}

fn function(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`checkbox`",
                ja: "`checkbox`",
            },
        ),
        para(
            site,
            L {
                en: "[`checkbox`](api:hypersig_ui/fn.checkbox.html) takes the text and the `bool` signal it shows and sets:",
                ja: "[`checkbox`](api:hypersig_ui/fn.checkbox.html) は、テキストと、表示して設定する `bool` のシグナルを受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn checkbox<S, R: Writable>(text: impl Into<String>, checked: Sig<S, bool, R>) -> El<S>",
        ),
        para(
            site,
            L {
                en: "With the signal true, `checkbox(\"I accept\", Terms::accepted())` renders as follows, without the `class` and the check mark's `<svg>`:",
                ja: "シグナルが true のとき、`checkbox(\"I accept\", Terms::accepted())` は、`class` とチェックマークの `<svg>` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<label>\n  <span>\n    <input type=\"checkbox\" checked=\"\">\n    <span aria-hidden=\"true\"><svg …></svg></span>\n  </span>\n  <span>I accept</span>\n</label>",
        ),
        para(
            site,
            L {
                en: "The label holds the input, so the checkbox needs no id. That lets it bind a field of a list item, such as `task.at(Task::done())` inside `each`, where an id would repeat on every item.",
                ja: "ラベルが入力要素を包んでいるので、チェックボックスに id は不要です。そのため、`each` の中の `task.at(Task::done())` のように、id を付けると項目ごとに重複してしまうリストの項目のフィールドにも結び付けられます。",
            },
        ),
        para(
            site,
            L {
                en: "A click writes to the signal, so its third type parameter has to be `Writable`. A computed signal fails to compile with E0277, `a signal marked Computed cannot be written`:",
                ja: "クリックするとシグナルに書き込むので、シグナルの 3 つ目の型パラメーターは `Writable` でなければなりません。計算シグナルは E0277（`a signal marked Computed cannot be written`）でコンパイルできません。",
            },
        ),
        show(Snippet::UiCheckboxComputed),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型では検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "An empty text compiles and leaves the checkbox with no name.",
                    ja: "テキストを空にしてもコンパイルでき、チェックボックスは名前のないままになります。",
                },
                L {
                    en: "`checkbox` has no error message. A checkbox that a form requires, such as accepting the terms, uses `checkbox_in_form`, on [Fields bound to a form](components/form-fields/).",
                    ja: "`checkbox` にはエラーメッセージがありません。規約への同意のように、フォームが必須とするチェックボックスには、[フォームに結び付けた入力欄](components/form-fields/)の `checkbox_in_form` を使います。",
                },
                L {
                    en: "The checkbox has two states. A checkbox that shows a third, mixed state, as a header that selects all rows does, is not covered here; the [data table](components/data-table/) draws its own.",
                    ja: "チェックボックスの状態は 2 つです。すべての行を選ぶヘッダーのような「一部選択」の 3 つ目の状態は扱いません。[データテーブル](components/data-table/)は独自にそれを描きます。",
                },
            ],
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: terms and a task list",
                ja: "例: 規約とタスクの一覧",
            },
        ),
        show(Snippet::UiCheckbox),
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
        para(
            site,
            L {
                en: "[Switch](components/switch/) is the same input for a setting that takes effect at once. [Fields bound to a form](components/form-fields/) has `checkbox_in_form`. [Lists](learn/lists/) covers `each`.",
                ja: "すぐに反映される設定には同じ入力要素を使った[スイッチ](components/switch/)を使います。`checkbox_in_form` は[フォームに結び付けた入力欄](components/form-fields/)で説明しています。`each` は[リスト](learn/lists/)で説明しています。",
            },
        ),
    ])
}
