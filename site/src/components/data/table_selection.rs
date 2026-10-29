//! The component page on `selectable` and `expandable`: rows a visitor
//! selects into a list signal, and rows that open their details.

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
        lead(site, Doc::Components(Component::TableSelection).leads()),
        demo::show(site, Demo::TableSelection),
        accessibility(site),
        selecting(site),
        expanding(site),
        html(site),
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
                en: "Each row's checkbox is a native `<input type=\"checkbox\">` named by `TableText::select_row`. The header checkbox follows the WAI-ARIA checkbox pattern: a `<button>` with `role=\"checkbox\"` whose `aria-checked` is `true` while every row is selected, `mixed` while some are, and `false` while none is. It is a button because a native checkbox's mixed state is a property that no attribute sets, and the first HTML could not show it.",
                ja: "各行のチェックボックスは `TableText::select_row` で名前を付けたネイティブの `<input type=\"checkbox\">` です。見出しのチェックボックスは WAI-ARIA の checkbox パターンに従い、`role=\"checkbox\"` を持つ `<button>` です。その `aria-checked` は、すべての行が選ばれている間は `true`、一部が選ばれている間は `mixed`、どれも選ばれていなければ `false` です。ボタンにしているのは、ネイティブのチェックボックスの中間状態はプロパティで、属性からは設定できず、最初の HTML で表せないためです。",
            },
        ),
        para(
            site,
            L {
                en: "The button at the start of an expandable row is named by `TableText::expand_row` and carries `aria-expanded`, as the WAI-ARIA disclosure pattern asks, so a screen reader announces whether the row's details are shown.",
                ja: "展開できる行の先頭のボタンは `TableText::expand_row` で名前を付け、WAI-ARIA の disclosure パターンに従って `aria-expanded` を持ちます。そのため、スクリーンリーダーは行の詳細が表示されているかどうかを読み上げます。",
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
                        "Space",
                        "Toggles the focused checkbox. On the header checkbox, selects every row, or clears them once all are selected.",
                    ],
                    &[
                        "Enter, Space",
                        "On the button at the start of a row, shows or hides the row's details.",
                    ],
                ],
                ja: &[
                    &[
                        "Space",
                        "フォーカスのあるチェックボックスを切り替えます。見出しのチェックボックスでは、すべての行を選び、すべて選ばれていれば選択を外します。",
                    ],
                    &[
                        "Enter、Space",
                        "行の先頭のボタンで押すと、その行の詳細を表示するか隠します。",
                    ],
                ],
            },
        ),
    ])
}

fn selecting(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Selecting rows",
                ja: "行を選ぶ",
            },
        ),
        para(
            site,
            L {
                en: "[`selectable`](api:hypersig_ui/struct.DataTable.html#method.selectable) adds a first column of checkboxes, which keep the key of each selected row in a list signal:",
                ja: "[`selectable`](api:hypersig_ui/struct.DataTable.html#method.selectable) はチェックボックスの列を先頭に追加し、チェックボックスは選ばれた各行のキーをリストのシグナルに持ちます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> DataTable<S, T> {
    pub fn selectable<K: PartialEq + 'static, E: IntoExpr<S, K>>(
        self,
        selected: Sig<S, Vec<K>>,
        key: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self;
}",
        ),
        para(
            site,
            L {
                en: "The keys live in a signal of the page, apart from the rows, so the row type needs no field for the selection, and a patch of the rows leaves the selection as it was. Any statement reads them, such as an action call that takes `Ledger::chosen()` as its argument, and the handler receives the keys as a `Vec`. `key` must return the type the list holds, so a key of another type is E0277:",
                ja: "キーは行とは別にページのシグナルに入るので、行の型に選択用のフィールドは要らず、行をパッチで送っても選択はそのまま残ります。どの文からでもキーを読めます。たとえば `Ledger::chosen()` を引数にしたアクションの呼び出しでは、ハンドラがキーを `Vec` として受け取ります。`key` はリストが持つ型を返さなければならないので、ほかの型のキーは E0277 になります。",
            },
        ),
        show(Snippet::UiSelectionKey),
        para(
            site,
            L {
                en: "The header checkbox selects every row of the list, including rows the filter or the page hides, and a press while all are selected clears them. A selected row is drawn on the muted background.",
                ja: "見出しのチェックボックスは絞り込みやページで隠れている行も含めて、リストのすべての行を選びます。すべて選ばれているときに押すと、選択を外します。選ばれた行は muted の背景で描かれます。",
            },
        ),
    ])
}

fn expanding(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Expanding rows",
                ja: "行を展開する",
            },
        ),
        para(
            site,
            L {
                en: "[`expandable`](api:hypersig_ui/struct.DataTable.html#method.expandable) adds a first column of buttons that show a row's details in a row of their own under it, across every column:",
                ja: "[`expandable`](api:hypersig_ui/struct.DataTable.html#method.expandable) はボタンの列を先頭に追加し、ボタンは行の下に、すべての列にまたがる専用の行で、その行の詳細を表示します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> DataTable<S, T> {
    pub fn expandable<K: PartialEq + 'static, E: IntoExpr<S, K>>(
        self,
        expanded: Sig<S, Vec<K>>,
        key: impl Fn(Item<S, T>) -> E + 'static,
        details: impl Fn(Item<S, T>) -> El<S> + 'static,
    ) -> Self;
}",
        ),
        para(
            site,
            L {
                en: "`expanded` keeps the keys of the open rows as `selected` does, and its key is checked against the list's type the same way. `details` builds the element from the row. Each row and its details are a `<tbody>` of their own, so the two move together when the rows are sorted. A table with both has the expand buttons first, then the checkboxes.",
                ja: "`expanded` は、`selected` と同じように開いている行のキーを持ち、キーの型も同じようにリストの型と照らして検査されます。`details` は行から要素を作ります。各行とその詳細はそれぞれ専用の `<tbody>` に入るので、行を並べ替えると 2 つが一緒に動きます。両方を使う表では、展開のボタンが先、チェックボックスが次に並びます。",
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
                en: "The table of the example below, with the second payment selected, renders its header and first row as:",
                ja: "下の例の表で 2 つ目の支払いを選んだ状態では、見出しと最初の行は次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<thead>
  <tr>
    <td></td>
    <th scope="col">
      <button type="button" role="checkbox" aria-label="Select all rows" aria-checked="mixed">
        <span aria-hidden="true"><svg …></svg><svg …></svg></span>
      </button>
    </th>
    <th scope="col">Email</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td><button type="button" aria-label="Show row details" aria-expanded="false"><svg …></svg></button></td>
    <td><span><input aria-label="Select row" type="checkbox"><span aria-hidden="true"><svg …></svg></span></span></td>
    <td><span>ken99@example.com</span></td>
  </tr>
  <tr><td colspan="3"><p>Paid by card.</p></td></tr>
</tbody>"#,
        ),
        para(
            site,
            L {
                en: "The second row's checkbox carries `checked`. The details row is in the HTML and hidden until its button is pressed.",
                ja: "2 つ目の行のチェックボックスは `checked` を持ちます。詳細の行は HTML に含まれ、ボタンを押すまで隠れています。",
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
                en: "A selected key stays in the list after its row leaves the table, until the header clears the selection or the page removes the key, so a handler that receives the keys checks them against its own data. The keys are compared as JSON, as `each_keyed` compares them. Selecting with the header includes rows the visitor cannot see on the current page or through the filter.",
                ja: "選ばれたキーは、その行が表から消えたあとも、見出しで選択を外すかページがキーを取り除くまでリストに残ります。そのため、キーを受け取るハンドラは自分のデータと照らして確かめます。キーは `each_keyed` と同じく JSON として比べます。見出しで選ぶと、今のページや絞り込みのせいで訪問者に見えていない行も選ばれます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: refunding selected payments",
                ja: "例: 選んだ支払いの返金",
            },
        ),
        para(
            site,
            L {
                en: "The button sends the selected ids to an action, which clears the selection in its answer:",
                ja: "ボタンは選んだ id をアクションに送り、アクションは応答で選択を空にします。",
            },
        ),
        show(Snippet::UiTableSelection),
        para(
            site,
            L {
                en: "Next, [Server table](components/server-table/) lets the server filter, sort, and page the rows.",
                ja: "次の [サーバーテーブル](components/server-table/) では行の絞り込み、並べ替え、ページ分けをサーバーに任せます。",
            },
        ),
    ])
}
