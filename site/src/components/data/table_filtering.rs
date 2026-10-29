//! The component page on `filter`, `paginate`, and `Column::visible`:
//! showing part of a `DataTable`'s rows and columns.

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
        lead(site, Doc::Components(Component::TableFiltering).leads()),
        demo::show(site, Demo::TableFiltering),
        accessibility(site),
        filter(site),
        pages(site),
        visible(site),
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
                en: "The filter is a native `<input type=\"search\">` named by its label through `aria-label`, which is also its placeholder. The range of rows shown, such as `1–4 / 7`, sits in an `aria-live=\"polite\"` paragraph, so a screen reader reads the new range after the visitor types or changes page. The page buttons are native buttons named by `TableText`, and the one with no page to go to is disabled.",
                ja: "絞り込みの欄は `aria-label` でラベルを名前に持つネイティブの `<input type=\"search\">` で、同じラベルがプレースホルダーにもなります。`1–4 / 7` のような表示中の行の範囲は `aria-live=\"polite\"` の段落にあるので、訪問者が入力したりページを変えたりすると、スクリーンリーダーが新しい範囲を読み上げます。ページ送りのボタンは `TableText` で名前を付けたネイティブのボタンで、移る先のページがないボタンは無効になります。",
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
                        "Tab",
                        "Moves focus to the filter field, then through the table, then to the page buttons.",
                    ],
                    &["Enter, Space", "On a page button, moves to that page."],
                ],
                ja: &[
                    &[
                        "Tab",
                        "絞り込みの欄、表、ページ送りのボタンの順にフォーカスを移します。",
                    ],
                    &[
                        "Enter、Space",
                        "ページ送りのボタンで押すと、そのページに移ります。",
                    ],
                ],
            },
        ),
    ])
}

fn filter(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Filtering the rows",
                ja: "行を絞り込む",
            },
        ),
        para(
            site,
            L {
                en: "[`filter`](api:hypersig_ui/struct.DataTable.html#method.filter) adds a field above the table and shows only the rows whose text contains what the visitor types:",
                ja: "[`filter`](api:hypersig_ui/struct.DataTable.html#method.filter) は表の上に入力欄を追加し、訪問者が入力した文字列を含む行だけを表示します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> DataTable<S, T> {
    pub fn filter<E: IntoExpr<S, String>>(
        self,
        label: impl Into<String>,
        text: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self;
}",
        ),
        para(
            site,
            L {
                en: "`text` returns the text of a row the filter searches, such as one field or several joined with `concat`. The browser trims the visitor's text, compares both in lower case, and keeps a row whose text contains it, so `KEN` finds `ken99@example.com`. Typing returns the table to its first page, and the field's text is the `query` of the table's `TableState`. The text must be a `String` expression, so a number field is E0277:",
                ja: "`text` は、絞り込みで検索する行の文字列を返します。1 つのフィールドでも、`concat` でいくつかをつないだものでもかまいません。ブラウザは訪問者の文字列の前後の空白を除き、両方を小文字にして比べ、その文字列を含む行を残します。そのため `KEN` で `ken99@example.com` が見つかります。入力すると表は最初のページに戻り、欄の文字列は表の `TableState` の `query` になります。文字列は `String` の式でなければならないので、数値のフィールドは E0277 になります。",
            },
        ),
        show(Snippet::UiFilterNumber),
    ])
}

fn pages(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Pages of rows",
                ja: "行をページに分ける",
            },
        ),
        para(
            site,
            L {
                en: "[`paginate`](api:hypersig_ui/struct.DataTable.html#method.paginate) shows `size` of the rows that pass the filter at a time, with the range shown and buttons for the previous and next page below the table:",
                ja: "[`paginate`](api:hypersig_ui/struct.DataTable.html#method.paginate) は、絞り込みを通った行を一度に `size` 行ずつ表示し、表の下に表示中の範囲と、前後のページに移るボタンを置きます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> DataTable<S, T> {
    pub fn paginate(self, size: i32) -> Self;
}",
        ),
        para(
            site,
            L {
                en: "The page is the `page` of the `TableState`, counted from 0, so a page of the application can open the table at a page by its first value, or move it with a statement. The range reads `first–last / count`, such as `5–7 / 7` on the second page of seven rows four at a time. A page past the last row, which the list reaches when it shrinks through a patch or a statement, returns the table to its first page.",
                ja: "ページは `TableState` の `page` で、0 から数えます。そのため、アプリケーションのページは最初の値で表を任意のページから開けますし、文でページを動かすこともできます。範囲は `最初–最後 / 件数` の形で、7 行を 4 行ずつ表示する表の 2 ページ目なら `5–7 / 7` です。パッチや文でリストが縮んで最後の行より後ろのページになった場合は、表は最初のページに戻ります。",
            },
        ),
        para(
            site,
            L {
                en: "`size` is a plain `i32`, so the types stop no mistake here. A size below 1 panics where `paginate` is called:",
                ja: "`size` はただの `i32` なので、ここで型が防ぐ間違いはありません。1 未満の大きさを渡すと、`paginate` を呼んだところでパニックします。",
            },
        ),
        show(Snippet::UiPaginateZero),
    ])
}

fn visible(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Columns shown on a condition",
                ja: "条件付きで表示する列",
            },
        ),
        para(
            site,
            L {
                en: "[`Column::visible`](api:hypersig_ui/struct.Column.html#method.visible) shows a column, its header and its cells, only while an expression is true:",
                ja: "[`Column::visible`](api:hypersig_ui/struct.Column.html#method.visible) は、式が真の間だけ列の見出しとセルを表示します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> Column<S, T> {
    pub fn visible(self, when: impl IntoExpr<S, bool>) -> Self;
}",
        ),
        para(
            site,
            L {
                en: "The expression reads the page's signals, such as a `bool` that a switch is bound to, as in the demo. A signal of another type is E0277:",
                ja: "式はページのシグナルを読みます。デモのように、スイッチに束縛した `bool` などです。ほかの型のシグナルは E0277 になります。",
            },
        ),
        show(Snippet::UiVisibleType),
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
                en: "The table of the example below, with its Id column hidden, renders the filter above the table and the range and buttons below it:",
                ja: "下の例の表はId の列を隠した状態で、表の上に絞り込みの欄、表の下に範囲とボタンを描画します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div>
  <input type="search" aria-label="Filter emails" placeholder="Filter emails" value="">
  <div>
    <table aria-label="Payments">…</table>
  </div>
  <div>
    <p aria-live="polite">1–4 / 7</p>
    <button type="button" aria-label="Previous page" disabled><svg …></svg></button>
    <button type="button" aria-label="Next page"><svg …></svg></button>
  </div>
</div>"#,
        ),
        para(
            site,
            L {
                en: "Every row is in the HTML, including those the filter or the page hides, and the browser shows only the rows that pass. The Id header and cells are in it too, hidden while the switch is off.",
                ja: "絞り込みやページで隠れる行も含めて、すべての行が HTML に含まれ、ブラウザは条件を通った行だけを表示します。Id の見出しとセルも含まれ、スイッチがオフの間は隠れています。",
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
                en: "The browser counts the rows on each change of the list, the filter, or the page, and with a filter and pages both on, the work grows with the square of the number of rows. The crate has not measured where that becomes slow, and a list of thousands of rows belongs in a [Server table](components/server-table/), which receives one page at a time. The filter matches text only as written: it ignores case but not accents, and finds `ken` in `broken`.",
                ja: "ブラウザはリスト、絞り込み、ページが変わるたびに行を数え、絞り込みとページ分けを両方使うと、その手間は行数の 2 乗で増えます。どこから遅くなるかはクレートでは測っていません。数千行のリストは1 ページずつ受け取る [サーバーテーブル](components/server-table/) で扱います。絞り込みは書かれたとおりの文字列だけを比べます。大文字と小文字は区別しませんがアクセント記号は区別し、`broken` の中の `ken` も見つけます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: payments by email",
                ja: "例: メールアドレスで探す支払い",
            },
        ),
        para(
            site,
            L {
                en: "A filter on the email, four rows a page, and an Id column that a switch shows:",
                ja: "メールアドレスで絞り込み、1 ページに 4 行を表示し、スイッチで Id の列を表示します。",
            },
        ),
        show(Snippet::UiTableFiltering),
        para(
            site,
            L {
                en: "Next, [Selecting and expanding rows](components/table-selection/) adds checkboxes and row details.",
                ja: "次の [行の選択と展開](components/table-selection/) ではチェックボックスと行の詳細を加えます。",
            },
        ),
    ])
}
