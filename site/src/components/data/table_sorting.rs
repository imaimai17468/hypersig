//! The component page on sorting a `DataTable`: `sortable_column`,
//! `SortOrder`, and the sort that `TableState` keeps.

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
        lead(site, Doc::Components(Component::TableSorting).leads()),
        demo::show(site, Demo::TableSorting),
        accessibility(site),
        sortable(site),
        html(site),
        state(site),
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
                en: "A sortable column's header holds a native `<button>` with the header's text, and the `<th>` carries `aria-sort`, which is `ascending`, `descending`, or `none`. A screen reader reads the header as a button and announces the direction the column is sorted in. The arrows beside the text are drawn for sighted visitors and hidden from assistive technology.",
                ja: "並べ替えられる列の見出しは見出しの文字列を持つネイティブの `<button>` を含み、`<th>` は `ascending`、`descending`、`none` のどれかの `aria-sort` を持ちます。スクリーンリーダーは見出しをボタンとして読み上げ、その列がどちら向きに並んでいるかを伝えます。文字列の横の矢印は目で見る訪問者のためのもので、支援技術からは隠します。",
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
                    &["Tab", "Moves focus to the next sortable header."],
                    &[
                        "Enter, Space",
                        "On a header, sorts the rows by that column: ascending, or descending where the column is sorted ascending.",
                    ],
                ],
                ja: &[
                    &["Tab", "次の並べ替えられる見出しにフォーカスを移します。"],
                    &[
                        "Enter、Space",
                        "見出しで押すと、その列で行を並べ替えます。昇順で並んでいる列なら降順に、それ以外は昇順にします。",
                    ],
                ],
            },
        ),
    ])
}

fn sortable(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A sortable column",
                ja: "並べ替えられる列",
            },
        ),
        para(
            site,
            L {
                en: "[`sortable_column`](api:hypersig_ui/struct.DataTable.html#method.sortable_column) adds a column, as `column` does, with the key its header sorts the rows by:",
                ja: "[`sortable_column`](api:hypersig_ui/struct.DataTable.html#method.sortable_column) は `column` と同じように列を追加し、見出しが行を並べ替えるときのキーを一緒に受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> DataTable<S, T> {
    pub fn sortable_column<K: Comparable, E: IntoExpr<S, K>>(
        self,
        column: Column<S, T>,
        key: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self;
}",
        ),
        para(
            site,
            L {
                en: "The key is usually the field the column shows, but it can be any expression of the row, such as a lower-cased name. The first press on a header sorts the rows ascending, a press on the column sorted ascending sorts it descending, and a press on any other state sorts ascending again. The browser sorts without a request, and the sort is stable: rows with equal keys keep their order, so sorting by one column and then by another orders the rows by the second and, among equal values, by the first.",
                ja: "キーはふつう列が表示するフィールドですが、小文字にした名前のように、行から作る任意の式にできます。見出しを最初に押すと行は昇順に並び、昇順で並んでいる列を押すと降順に、それ以外の状態で押すと再び昇順になります。ブラウザはリクエストなしに並べ替えます。並べ替えは安定で、キーが等しい行は元の順序を保つので、ある列で並べたあと別の列で並べると、行は後の列の順に、同じ値の中では前の列の順に並びます。",
            },
        ),
        para(
            site,
            L {
                en: "`K` must be [`Comparable`](api:hypersig/trait.Comparable.html), which `i32`, `i64`, `f64`, and `String` implement, because the browser orders only numbers and strings. A key of another type, such as a `bool`, is E0277:",
                ja: "ブラウザが順序を決められるのは数値と文字列だけなので、`K` は [`Comparable`](api:hypersig/trait.Comparable.html) でなければなりません。これを実装するのは `i32`、`i64`、`f64`、`String` です。`bool` のようなほかの型のキーは E0277 になります。",
            },
        ),
        show(Snippet::UiSortKey),
        para(
            site,
            L {
                en: "Sorting reorders the list signal itself, not only the table. Anything else on the page that reads the list, such as a chart of the same rows, follows the new order.",
                ja: "並べ替えは表だけでなく、リストのシグナルそのものの順序を変えます。同じ行を描くチャートのように、ページの中でそのリストを読むものも新しい順序に従います。",
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
                en: "The header row of the example below, before any sort, renders as follows. The `<svg>` elements are the three arrows, of which the browser shows the one for the current direction:",
                ja: "下の例の見出しの行は、並べ替える前には次のように描画されます。`<svg>` は 3 つの矢印で、ブラウザは今の向きに合う 1 つだけを表示します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<tr>
  <th scope="col">Id</th>
  <th scope="col" aria-sort="none">
    <button type="button"><span>Email</span><svg …></svg><svg …></svg><svg …></svg></button>
  </th>
  <th scope="col" aria-sort="none">
    <button type="button"><span>Amount</span><svg …></svg><svg …></svg><svg …></svg></button>
  </th>
</tr>"#,
        ),
        para(
            site,
            L {
                en: "A press on Email turns its `aria-sort` to `ascending`, leaves Amount at `none`, and moves the `<tr>` elements of the body into the new order.",
                ja: "Email を押すと、その `aria-sort` が `ascending` になり、Amount は `none` のままで、本体の `<tr>` が新しい順序に移動します。",
            },
        ),
    ])
}

fn state(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The sort in the state",
                ja: "状態の中の並べ替え",
            },
        ),
        para(
            site,
            L {
                en: "The header writes which column is sorted, and whether descending, into the table's [`TableState`](api:hypersig_ui/struct.TableState.html). Those two fields are private. [`SortOrder`](api:hypersig_ui/enum.SortOrder.html) names the direction, and two methods write and read the sort by a key with `#[derive(Segment)]`:",
                ja: "見出しはどの列で並べたか、降順かどうかを、表の [`TableState`](api:hypersig_ui/struct.TableState.html) に書き込みます。この 2 つのフィールドは非公開です。並べ替えの向きは [`SortOrder`](api:hypersig_ui/enum.SortOrder.html) で表し、2 つのメソッドが `#[derive(Segment)]` を付けたキーで並べ替えを書き込み、読み出します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub enum SortOrder {
    Ascending,
    Descending,
}

impl TableState {
    pub fn with_sorting<K: Segment>(self, key: &K, order: SortOrder) -> Self;
    pub fn sorting<K: Segment>(&self) -> Option<(K, SortOrder)>;
}",
        ),
        show(Snippet::UiSortingState),
        para(
            site,
            L {
                en: "These keys are the ones a [`ServerTable`](components/server-table/)'s headers write, where the server sorts the rows and reads the sort back with `sorting`. A `DataTable`'s headers do not name their columns by such a key, so `with_sorting` does not mark one of its headers, and `sorting` reads `None` from its state for an enum of keys. A page that sends a `DataTable` its rows already sorted shows every header at `aria-sort=\"none\"` until the visitor sorts.",
                ja: "これらのキーは [`ServerTable`](components/server-table/) の見出しが書くもので、そこではサーバーが行を並べ替え、`sorting` で並べ替えを読み戻します。`DataTable` の見出しはこのようなキーで列を指さないので、`with_sorting` で `DataTable` の見出しに印は付かず、キーの列挙型で `sorting` を呼ぶと、その状態からは `None` が返ります。並べ替え済みの行を `DataTable` に渡すページでは、訪問者が並べ替えるまで、どの見出しも `aria-sort=\"none\"` のままです。",
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
                en: "The order holds until the list changes. A patch from the server replaces the list in the server's order, and the header still marks the last sort, so a page whose handler patches the rows sorts them the same way on the server or accepts the mismatch. Nothing checks that the key matches what the column shows: a column of names sorted by an id sorts by the id.",
                ja: "順序はリストが変わるまで保たれます。サーバーからのパッチはリストをサーバーの順序で置き換えますが、見出しの印は最後の並べ替えのままです。そのため、ハンドラが行をパッチで送るページは、サーバーでも同じように並べるか、この食い違いを受け入れます。キーが列の表示内容と合っているかは検査されません。名前の列を id で並べるようにすると、id の順に並びます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: sortable payments",
                ja: "例: 並べ替えられる支払い",
            },
        ),
        para(
            site,
            L {
                en: "Email and Amount sort the rows, and Id does not:",
                ja: "Email と Amount は行を並べ替え、Id は並べ替えません。",
            },
        ),
        show(Snippet::UiTableSorting),
        para(
            site,
            L {
                en: "Next, [Filtering and pages](components/table-filtering/) shows part of the rows at a time.",
                ja: "次の [絞り込みとページ分け](components/table-filtering/) では行の一部だけを表示します。",
            },
        ),
    ])
}
