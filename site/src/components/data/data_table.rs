//! The component page on `DataTable` and `Column`: a table of a list
//! signal's rows under columns built from the row type's fields.

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
        lead(site, Doc::Components(Component::DataTable).leads()),
        demo::show(site, Demo::DataTable),
        accessibility(site),
        building(site),
        columns(site),
        state_and_text(site),
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
                en: "The table is a native `<table>` named by its label through `aria-label`, and each header is a `<th scope=\"col\">`, so a screen reader announces the table by name and reads each cell with its column's header. A table with only columns takes no keys of its own. The sort buttons, the checkboxes, and the page buttons that the next pages add are native buttons and inputs, and those pages list their keys.",
                ja: "表は `aria-label` でラベルを名前に持つネイティブの `<table>` で、各見出しは `<th scope=\"col\">` です。そのため、スクリーンリーダーは表を名前で読み上げ、各セルを列の見出しと一緒に読み上げます。列だけの表は、表自体が受け付けるキーを持ちません。次のページで加える並べ替えのボタン、チェックボックス、ページ送りのボタンはネイティブのボタンや入力欄で、そのキーはそれぞれのページに載せています。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a table",
                ja: "表を作る",
            },
        ),
        para(
            site,
            L {
                en: "[`DataTable::new`](api:hypersig_ui/struct.DataTable.html#method.new) takes the table's name, the signal of its state, the list signal of its rows, and the key of each row, and [`column`](api:hypersig_ui/struct.DataTable.html#method.column) adds a column after the others:",
                ja: "[`DataTable::new`](api:hypersig_ui/struct.DataTable.html#method.new) は、表の名前、状態のシグナル、行のリストのシグナル、各行のキーを受け取ります。[`column`](api:hypersig_ui/struct.DataTable.html#method.column) は既存の列の後ろに列を追加します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> DataTable<S, T> {
    pub fn new<K: 'static, E: IntoExpr<S, K>>(
        label: impl Into<String>,
        state: Sig<S, TableState>,
        rows: Sig<S, Vec<T>>,
        key: impl FnOnce(Item<S, T>) -> E + 'static,
    ) -> Self;
    pub fn column(self, column: Column<S, T>) -> Self;
    pub fn text(self, text: TableText) -> Self;
    pub fn max_height(self, height: Length) -> Self;
    pub fn render(self) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "`S` is the page's signals and `T` the row type, so every column of the table reads the same row type. The key works as in [`each_keyed`](learn/lists/): a row keeps its element when the rows move, so a checkbox keeps its focus when sorting moves its row. The label names the table for assistive technology and is not shown. `DataTable` converts into an `El<S>` and a `Node<S>`, so `Page::child` takes it without `render`.",
                ja: "`S` はページのシグナル、`T` は行の型で、表のどの列も同じ行の型を読みます。キーは [`each_keyed`](learn/lists/) と同じ働きをします。行が移動しても行の要素はそのまま残るので、並べ替えで行が動いてもチェックボックスのフォーカスは保たれます。ラベルは支援技術向けの表の名前で、画面には表示されません。`DataTable` は `El<S>` と `Node<S>` に変換できるので、`render` を呼ばずに `Page::child` に渡せます。",
            },
        ),
        para(
            site,
            L {
                en: "[`max_height`](api:hypersig_ui/struct.DataTable.html#method.max_height) keeps the table at most `height` high. Its rows then scroll under a header that stays in place, as in the demo above.",
                ja: "[`max_height`](api:hypersig_ui/struct.DataTable.html#method.max_height) は表の高さを `height` までに抑えます。このとき行はスクロールし、見出しはその上で動かずに残ります。上のデモがこの形です。",
            },
        ),
    ])
}

fn columns(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Columns",
                ja: "列",
            },
        ),
        para(
            site,
            L {
                en: "A [`Column`](api:hypersig_ui/struct.Column.html) shows a field of the row, or any element built from the row:",
                ja: "[`Column`](api:hypersig_ui/struct.Column.html) は行のフィールドか、行から作った任意の要素を表示します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Constructor", "What each cell shows"],
                ja: &["コンストラクタ", "各セルが表示するもの"],
            },
            L {
                en: &[
                    &[
                        "[`Column::text(header, field: Sig<T, String>)`](api:hypersig_ui/struct.Column.html#method.text)",
                        "The text field, aligned to the start.",
                    ],
                    &[
                        "[`Column::number(header, field: Sig<T, U>)`](api:hypersig_ui/struct.Column.html#method.number), `U: Number + Textual`",
                        "The number field as JavaScript's `String` writes it, aligned to the end in digits of one width.",
                    ],
                    &[
                        "[`Column::formatted(header, field: Sig<T, U>, format: NumberFormat)`](api:hypersig_ui/struct.Column.html#method.formatted), `U: Number`",
                        "The number field written in `format`, aligned to the end.",
                    ],
                    &[
                        "[`Column::custom(header, render: impl Fn(Item<S, T>) -> El<S>)`](api:hypersig_ui/struct.Column.html#method.custom)",
                        "The element `render` returns for the row, aligned to the start.",
                    ],
                ],
                ja: &[
                    &[
                        "[`Column::text(header, field: Sig<T, String>)`](api:hypersig_ui/struct.Column.html#method.text)",
                        "文字列のフィールド。行頭に揃えます。",
                    ],
                    &[
                        "[`Column::number(header, field: Sig<T, U>)`](api:hypersig_ui/struct.Column.html#method.number)、`U: Number + Textual`",
                        "数値のフィールドを JavaScript の `String` が書く形で表示し、等幅の数字で行末に揃えます。",
                    ],
                    &[
                        "[`Column::formatted(header, field: Sig<T, U>, format: NumberFormat)`](api:hypersig_ui/struct.Column.html#method.formatted)、`U: Number`",
                        "数値のフィールドを `format` の形で書き、行末に揃えます。",
                    ],
                    &[
                        "[`Column::custom(header, render: impl Fn(Item<S, T>) -> El<S>)`](api:hypersig_ui/struct.Column.html#method.custom)",
                        "`render` がその行に返す要素。行頭に揃えます。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`header` is `impl Into<String>` in each. A [`NumberFormat`](api:hypersig/struct.NumberFormat.html) sets the decimals, the separators, and the affixes, and the browser and the first HTML write the same text with it, whatever the visitor's locale. A column is built from a field of the row type, so a field of the wrong type is E0308:",
                ja: "どのコンストラクタでも `header` は `impl Into<String>` です。[`NumberFormat`](api:hypersig/struct.NumberFormat.html) は小数の桁数、区切り文字、前後に付ける文字を決め、ブラウザと最初の HTML は訪問者のロケールに関係なく同じ文字列を書きます。列は行の型のフィールドから作るので、型の違うフィールドを渡すと E0308 になります。",
            },
        ),
        show(Snippet::UiColumnType),
        para(
            site,
            L {
                en: "A field of another struct is E0308 as well, because the column's row type no longer matches the table's:",
                ja: "別の構造体のフィールドも、列の行の型が表の行の型と合わなくなるので、同じく E0308 になります。",
            },
        ),
        show(Snippet::UiColumnOtherRow),
    ])
}

fn state_and_text(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The state and the words",
                ja: "状態と言葉",
            },
        ),
        para(
            site,
            L {
                en: "Every table keeps its filter text, its page, and its sort in a [`TableState`](api:hypersig_ui/struct.TableState.html), a field of the page's signals that `new` takes even when the table uses none of them. Give each table a field of its own:",
                ja: "どの表も絞り込みの文字列、ページ、並べ替えを [`TableState`](api:hypersig_ui/struct.TableState.html) に持ちます。これはページのシグナルのフィールドで、表がどれも使わない場合でも `new` に渡します。表ごとに別のフィールドを用意してください。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "#[derive(Signals, Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct TableState {
    /// What the filter field holds.
    pub query: String,
    /// The page shown, counted from 0.
    pub page: i32,
    // The sort is private; `with_sorting` and `sorting` write and read it.
}",
        ),
        para(
            site,
            L {
                en: "`TableState::default()` is the first state: no filter, the first page, unsorted. [Sorting a table](components/table-sorting/) covers the sort, and [Filtering and pages](components/table-filtering/) the filter and the page.",
                ja: "`TableState::default()` が最初の状態で、絞り込みなし、最初のページ、並べ替えなしです。並べ替えは [テーブルの並べ替え](components/table-sorting/) で、絞り込みとページは [絞り込みとページ分け](components/table-filtering/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "[`TableText`](api:hypersig_ui/struct.TableText.html) holds the words the table shows and names its controls with, and [`text`](api:hypersig_ui/struct.DataTable.html#method.text) replaces them. They are English until set:",
                ja: "[`TableText`](api:hypersig_ui/struct.TableText.html) は、表が表示する言葉と操作部品に付ける名前を持ち、[`text`](api:hypersig_ui/struct.DataTable.html#method.text) で置き換えます。設定しなければ英語です。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Field", "Default", "Where it appears"],
                ja: &["フィールド", "既定値", "現れる場所"],
            },
            L {
                en: &[
                    &[
                        "`empty`",
                        "No results.",
                        "The row shown when no row is shown.",
                    ],
                    &[
                        "`select_row`",
                        "Select row",
                        "The name of a row's checkbox.",
                    ],
                    &[
                        "`select_all`",
                        "Select all rows",
                        "The name of the header checkbox.",
                    ],
                    &[
                        "`previous_page`",
                        "Previous page",
                        "The name of the button that goes back a page.",
                    ],
                    &[
                        "`next_page`",
                        "Next page",
                        "The name of the button that goes forward a page.",
                    ],
                    &[
                        "`expand_row`",
                        "Show row details",
                        "The name of the button that shows a row's details.",
                    ],
                ],
                ja: &[
                    &["`empty`", "No results.", "表示する行がないときの行"],
                    &["`select_row`", "Select row", "各行のチェックボックスの名前"],
                    &[
                        "`select_all`",
                        "Select all rows",
                        "見出しのチェックボックスの名前",
                    ],
                    &[
                        "`previous_page`",
                        "Previous page",
                        "前のページに戻るボタンの名前",
                    ],
                    &["`next_page`", "Next page", "次のページに進むボタンの名前"],
                    &[
                        "`expand_row`",
                        "Show row details",
                        "行の詳細を表示するボタンの名前",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each field is a `String`, so a page in another language writes the ones its table uses and takes the rest from `..TableText::default()`.",
                ja: "どのフィールドも `String` なので、ほかの言語のページは表が使うフィールドだけを書き、残りは `..TableText::default()` で埋めます。",
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
                en: "The table of the example below, with two invoices, renders as:",
                ja: "下の例の表に 2 件の請求書を入れると、次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div>
  <div>
    <table aria-label="Invoices">
      <thead>
        <tr>
          <th scope="col">Number</th>
          <th scope="col">Email</th>
          <th scope="col">Status</th>
          <th scope="col">Amount</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><span>1001</span></td>
          <td><span>ken99@example.com</span></td>
          <td><span>Paid</span></td>
          <td><span>$316.00</span></td>
        </tr>
        <tr>
          <td><span>1002</span></td>
          <td><span>abe45@example.com</span></td>
          <td><span>Pending</span></td>
          <td><span>$1,242.50</span></td>
        </tr>
        <tr><td colspan="4">No results.</td></tr>
      </tbody>
    </table>
  </div>
</div>"#,
        ),
        para(
            site,
            L {
                en: "The browser shows the last row, which spans every column, only while the list is empty. The cells follow the list signal: a statement or a patch that adds, removes, or changes a row updates its `<tr>` with no request for the table.",
                ja: "すべての列にまたがる最後の行は、リストが空の間だけ表示されます。セルはリストのシグナルに従うので、文やパッチで行を追加、削除、変更すると、表のためのリクエストなしにその `<tr>` が更新されます。",
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
                en: "The key's type is free, and the types do not check that the keys of two rows differ. Where two rows share one, the browser writes a warning to the console and matches the repeated row by its position, as on [Lists](learn/lists/). What a `custom` column renders is any element, so its alignment and its text for a screen reader are yours. The label and the headers are plain strings.",
                ja: "キーの型は自由で、2 つの行のキーが違うことは型では検査しません。キーが重複すると、[リスト](learn/lists/) で説明したとおり、ブラウザはコンソールに警告を書き、重複した行を位置で対応させます。`custom` の列が描画するのは任意の要素なので、その揃え方とスクリーンリーダー向けのテキストは自分で決めます。ラベルと見出しはただの文字列です。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: invoices",
                ja: "例: 請求書",
            },
        ),
        para(
            site,
            L {
                en: "A number, a text, a custom, and a formatted column, in a table at most 20rem high:",
                ja: "数値、文字列、任意の要素、書式付きの数値の列を、高さが最大 20rem の表に並べます。",
            },
        ),
        show(Snippet::UiDataTable),
        para(
            site,
            L {
                en: "Next, [Sorting a table](components/table-sorting/) adds headers that sort the rows.",
                ja: "次の [テーブルの並べ替え](components/table-sorting/) では行を並べ替える見出しを加えます。",
            },
        ),
    ])
}
