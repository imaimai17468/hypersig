//! The component page on `ServerTable`: a table whose rows an action sends
//! one page at a time, and the request and answer of that action.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::ServerTable).leads()),
        note(
            site,
            L {
                en: "This page has no live demo, because the site is static files with no server to answer the action. The code and the HTTP below are what an application runs.",
                ja: "このサイトは静的なファイルで、アクションに応答するサーバーがないため、このページには動くデモがありません。下のコードと HTTP はアプリケーションで実際に動くものです。",
            },
        ),
        accessibility(site),
        building(site),
        request(site),
        answer(site),
        html(site),
        in_the_url(site),
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
                en: "A `ServerTable` renders the same `<table>`, filter field, sortable headers with `aria-sort`, checkboxes, and page buttons as a [Data table](components/data-table/), and takes the same keys. The range of rows shown sits in an `aria-live=\"polite\"` paragraph, so a screen reader reads the new range once the server's answer arrives.",
                ja: "`ServerTable` は [データテーブル](components/data-table/) と同じ `<table>`、絞り込みの欄、`aria-sort` を持つ並べ替えの見出し、チェックボックス、ページ送りのボタンを描画し、同じキーを受け付けます。表示中の行の範囲は `aria-live=\"polite\"` の段落にあるので、サーバーの応答が届くと、スクリーンリーダーが新しい範囲を読み上げます。",
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
                        "Enter, Space",
                        "On a sortable header, asks the server for the rows sorted by that column. On a page button, asks for that page.",
                    ],
                    &[
                        "Space",
                        "Toggles the focused checkbox. On the header checkbox, selects or clears the rows of the page shown.",
                    ],
                ],
                ja: &[
                    &[
                        "Enter、Space",
                        "並べ替えの見出しで押すと、その列で並べた行をサーバーに求めます。ページ送りのボタンで押すと、そのページを求めます。",
                    ],
                    &[
                        "Space",
                        "フォーカスのあるチェックボックスを切り替えます。見出しのチェックボックスでは、表示中のページの行を選ぶか選択を外します。",
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
                en: "Building the table",
                ja: "表を作る",
            },
        ),
        para(
            site,
            L {
                en: "[`ServerTable::new`](api:hypersig_ui/struct.ServerTable.html#method.new) takes what `DataTable::new` takes and, last, `load`: the statement that fetches rows, which is a call to an action that receives the table's `TableState`:",
                ja: "[`ServerTable::new`](api:hypersig_ui/struct.ServerTable.html#method.new) は `DataTable::new` と同じものに加えて、最後に `load` を受け取ります。`load` は行を取りに行く文で、表の `TableState` を受け取るアクションの呼び出しです。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> ServerTable<S, T> {
    pub fn new<K: 'static, E: IntoExpr<S, K>>(
        label: impl Into<String>,
        state: Sig<S, TableState>,
        rows: Sig<S, Vec<T>>,
        key: impl FnOnce(Item<S, T>) -> E + 'static,
        load: impl IntoStmt<S>,
    ) -> Self;
    pub fn column(self, column: Column<S, T>) -> Self;
    pub fn sortable_column<K: Segment>(self, column: Column<S, T>, key: &K) -> Self;
    pub fn selectable<K: PartialEq + 'static, E: IntoExpr<S, K>>(
        self,
        selected: Sig<S, Vec<K>>,
        key: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self;
    pub fn expandable<K: PartialEq + 'static, E: IntoExpr<S, K>>(
        self,
        expanded: Sig<S, Vec<K>>,
        key: impl Fn(Item<S, T>) -> E + 'static,
        details: impl Fn(Item<S, T>) -> El<S> + 'static,
    ) -> Self;
    pub fn filter(self, label: impl Into<String>) -> Self;
    pub fn paginate(self, size: i32, total: Sig<S, i32>) -> Self;
    pub fn text(self, text: TableText) -> Self;
    pub fn max_height(self, height: Length) -> Self;
    pub fn render(self) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "The list signal holds only the rows shown, and the browser counts no rows, so its work per change grows with the rows of one page, however many the server holds. The controls write the `TableState` and then run `load`:",
                ja: "リストのシグナルには表示中の行だけが入り、ブラウザは行を数えないので、変更 1 回あたりのブラウザの手間は、サーバーが何行持っていても 1 ページ分の行数でしか増えません。操作部品は `TableState` を書き換えてから `load` を実行します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "What the control does"],
                ja: &["メソッド", "操作部品の動き"],
            },
            L {
                en: &[
                    &[
                        "[`sortable_column`](api:hypersig_ui/struct.ServerTable.html#method.sortable_column)",
                        "The header writes `key` and the direction into the state, returns to the first page, and runs `load`.",
                    ],
                    &[
                        "[`filter`](api:hypersig_ui/struct.ServerTable.html#method.filter)",
                        "Once typing pauses, the field returns to the first page and runs `load`. Its text is `query`, which the server filters by.",
                    ],
                    &[
                        "[`paginate`](api:hypersig_ui/struct.ServerTable.html#method.paginate)",
                        "The buttons move `page` by one and run `load`. The range reads `first–last / total`, with `total` from a signal the server writes.",
                    ],
                    &[
                        "[`selectable`](api:hypersig_ui/struct.ServerTable.html#method.selectable)",
                        "The keys stay in their signal when the page changes, so a row selected on one page stays selected on the next. The header checkbox selects or clears the rows of the page shown.",
                    ],
                ],
                ja: &[
                    &[
                        "[`sortable_column`](api:hypersig_ui/struct.ServerTable.html#method.sortable_column)",
                        "見出しは `key` と向きを状態に書き込み、最初のページに戻ってから `load` を実行します。",
                    ],
                    &[
                        "[`filter`](api:hypersig_ui/struct.ServerTable.html#method.filter)",
                        "入力が止まると、欄は最初のページに戻ってから `load` を実行します。欄の文字列は `query` で、サーバーはこれで絞り込みます。",
                    ],
                    &[
                        "[`paginate`](api:hypersig_ui/struct.ServerTable.html#method.paginate)",
                        "ボタンは `page` を 1 つ動かして `load` を実行します。範囲は `最初–最後 / total` の形で、`total` はサーバーが書くシグナルから取ります。",
                    ],
                    &[
                        "[`selectable`](api:hypersig_ui/struct.ServerTable.html#method.selectable)",
                        "ページが変わってもキーはシグナルに残るので、あるページで選んだ行は次のページに移っても選ばれたままです。見出しのチェックボックスは、表示中のページの行を選ぶか選択を外します。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A sortable column takes its key as a value of a type with `#[derive(Segment)]`, usually an enum of the columns, which the server reads back from the state. The headers and the handler's `match` then name the columns with one type. A key that is not a `Segment`, such as a string, is E0277:",
                ja: "並べ替えられる列は `#[derive(Segment)]` を付けた型の値をキーに取ります。ふつうは列の列挙型で、サーバーはそれを状態から読み戻します。こうすると、見出しとハンドラの `match` が 1 つの型で列を指します。文字列のように `Segment` でないキーは E0277 になります。",
            },
        ),
        show(Snippet::UiServerSortKey),
    ])
}

fn request(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the handler receives",
                ja: "ハンドラが受け取るもの",
            },
        ),
        para(
            site,
            L {
                en: "The action is an `Action<S, TableState>`, and the table calls it with `LOAD.call_with(Users::table())`, so the browser sends the table's state as the arguments. After the visitor typed `ada`, sorted Email descending, and moved to the second page, the browser's network tab shows:",
                ja: "アクションは `Action<S, TableState>` で、表は `LOAD.call_with(Users::table())` でそれを呼ぶので、ブラウザは表の状態を引数として送ります。訪問者が `ada` と入力し、Email を降順で並べ、2 ページ目に移ったあとは、ブラウザのネットワークタブに次のように表示されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "POST /users/rows\naccept: text/event-stream\nhypersig-call: 1\ncontent-type: application/json\n\n{\"query\":\"ada\",\"page\":1,\"sort\":\"Email\",\"descending\":true}",
        ),
        para(
            site,
            L {
                en: "The handler receives it deserialized as a `TableState`: `query` and `page` are public fields, and [`TableState::sorting`](api:hypersig_ui/struct.TableState.html#method.sorting) reads the sort back as the key's type and a [`SortOrder`](api:hypersig_ui/enum.SortOrder.html). [`Action::request`](api:hypersig/struct.Action.html#method.request) builds the same request for a test:",
                ja: "ハンドラはこれを `TableState` にデシリアライズして受け取ります。`query` と `page` は公開フィールドで、[`TableState::sorting`](api:hypersig_ui/struct.TableState.html#method.sorting) が並べ替えをキーの型と [`SortOrder`](api:hypersig_ui/enum.SortOrder.html) として読み戻します。[`Action::request`](api:hypersig/struct.Action.html#method.request) は、テストのために同じリクエストを作ります。",
            },
        ),
        show(Snippet::UiServerRequest),
    ])
}

fn answer(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the handler returns",
                ja: "ハンドラが返すもの",
            },
        ),
        para(
            site,
            L {
                en: "The handler answers with a patch of the list signal holding the rows of the page the state names, and, for a paged table, a patch of the total, the number of rows that pass the filter:",
                ja: "ハンドラは、状態が指すページの行を入れたリストのシグナルのパッチと、ページに分ける表では総数のパッチを返します。総数は絞り込みを通る行の数です。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "async fn load((): (), state: TableState) -> Patches<Users> {
    let (rows, total) = query(&everyone(), &state);
    Users::users().patch(rows).and(Users::total().patch(total)).into()
}",
        ),
        para(
            site,
            L {
                en: "The browser replaces the rows and the range when the answer arrives. Use the same query to render the page's first rows, as the example below does, so the first HTML and every later answer agree. [Patches](learn/patches/) covers what an answer can hold.",
                ja: "応答が届くと、ブラウザは行と範囲を置き換えます。下の例のように、ページの最初の行も同じ問い合わせで描画すると、最初の HTML とその後の応答が一致します。応答に入れられるものは [パッチ](learn/patches/) で説明します。",
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
                en: "The table of the example below, first rendered sorted by Id, renders the header of that column marked and the range from `total`:",
                ja: "下の例の表は最初に Id で並べた状態で描画されるので、その列の見出しに印が付き、範囲は `total` から取ります。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div>
  <input type="search" aria-label="Filter emails" placeholder="Filter emails" value="">
  <div>
    <table aria-label="Users">
      <thead>
        <tr>
          <th scope="col" aria-sort="ascending"><button type="button"><span>Id</span>…</button></th>
          <th scope="col" aria-sort="none"><button type="button"><span>Email</span>…</button></th>
        </tr>
      </thead>
      <tbody>
        <tr><td><span>1</span></td><td><span>user1@example.com</span></td></tr>
        …
      </tbody>
    </table>
  </div>
  <div>
    <p aria-live="polite">1–10 / 42</p>
    <button type="button" aria-label="Previous page" disabled>…</button>
    <button type="button" aria-label="Next page">…</button>
  </div>
</div>"#,
        ),
    ])
}

fn in_the_url(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Keeping the state in the URL",
                ja: "状態を URL に残す",
            },
        ),
        para(
            site,
            L {
                en: "To let a reload or a shared link show the same rows, give the page a route whose search parameters are the application's own struct with `#[derive(Search)]` and `Option` fields, so the address without a query string still loads. The page handler builds the first `TableState` from it with `with_sorting`, and the action answers with `Patches::replace_url` of the URL its state names. `TableState` itself is not a search struct, because a URL that lacks one of its fields would get a 400. [Search parameters](learn/search-params/) covers the route.",
                ja: "再読み込みや共有したリンクでも同じ行を表示するには、`Option` のフィールドを持ち `#[derive(Search)]` を付けたアプリケーション自身の構造体を、ページのルートのクエリパラメータにします。こうするとクエリ文字列のないアドレスも読み込めます。ページのハンドラはそこから `with_sorting` で最初の `TableState` を作り、アクションは状態が指す URL を `Patches::replace_url` で応答に含めます。`TableState` そのものをクエリパラメータにしないのは、フィールドが 1 つでも欠けた URL に 400 が返るためです。ルートは [クエリパラメータ](learn/search-params/) で説明します。",
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
                en: "What the server does with the state is the application's code, which the compiler does not check against the table. The handler answers with at most `size` rows of the page the state names, and a total that counts every row passing the filter. A handler that returns more rows shows them all, and a wrong total shows a wrong range and enables or disables the next-page button wrongly. A page past the last row shows no rows until the visitor goes back.",
                ja: "サーバーが状態をどう扱うかはアプリケーションのコードで、コンパイラはそれを表と照らして検査しません。ハンドラは、状態が指すページの行を最大 `size` 行と、絞り込みを通るすべての行を数えた総数を返します。それより多くの行を返すとすべて表示され、総数が違うと範囲の表示が誤り、次のページへのボタンの有効と無効も誤ります。最後の行より後ろのページでは、訪問者が戻るまで行が表示されません。",
            },
        ),
        para(
            site,
            L {
                en: "`page` and `query` come from the browser, so a handler treats them as input: a negative or very large page must not overflow its arithmetic, as `saturating_mul` keeps it from doing in the example.",
                ja: "`page` と `query` はブラウザから届くので、ハンドラは入力として扱います。例の `saturating_mul` のように、負のページや非常に大きなページで計算があふれないようにします。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: 42 users, ten at a time",
                ja: "例: 42 人のユーザーを 10 人ずつ",
            },
        ),
        para(
            site,
            L {
                en: "One `query` function serves both the first render and the action:",
                ja: "1 つの `query` 関数が最初の描画とアクションの両方に使われます。",
            },
        ),
        show(Snippet::UiServerTable),
        para(
            site,
            L {
                en: "Next, [Chart](components/chart/) draws the rows of a list signal as lines and areas.",
                ja: "次の [チャート](components/chart/) ではリストのシグナルの行を折れ線と面で描きます。",
            },
        ),
    ])
}
