//! The page on rendering and changing a list.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Lists).title(site)),
        lead(site, Doc::Learn(Learn::Lists).leads()),
        heading(
            site,
            L {
                en: "Rendering a list",
                ja: "リストを描画する",
            },
        ),
        para(
            site,
            L {
                en: "[`each`](api:hypersig/html/fn.each.html), `fn each<S, T, R>(list: Sig<S, Vec<T>, R>, render: impl FnOnce(Item<S, T>) -> El<S>) -> Node<S>`, takes the list and a function that builds the element for one item. The function runs once on the server to build a template, receiving an [`Item`](api:hypersig/struct.Item.html) that stands for whichever item the browser is rendering. `item.at(field)` reads a field of the item, `item.index()` its position from 0, and `item.value()` the item itself.",
                ja: "[`each`](api:hypersig/html/fn.each.html)（`fn each<S, T, R>(list: Sig<S, Vec<T>, R>, render: impl FnOnce(Item<S, T>) -> El<S>) -> Node<S>`）は、リストと、項目 1 つ分の要素を組み立てる関数を受け取ります。この関数はテンプレートを作るためにサーバーで 1 回だけ実行され、ブラウザがそのとき描画している項目を表す [`Item`](api:hypersig/struct.Item.html) を受け取ります。項目のフィールドは `item.at(field)` で、0 から数えた位置は `item.index()` で、項目そのものは `item.value()` で読みます。",
            },
        ),
        show(Snippet::Each),
        para(
            site,
            L {
                en: "`each` returns a `Node`, which goes into an element with `child`. A list of plain values, such as `Vec<String>`, passes the item itself where an expression is expected, as `bind_text(tag)` does.",
                ja: "`each` は `Node` を返し、`child` で要素に入れます。`Vec<String>` のような値のリストでは `bind_text(tag)` のように、式を受け取る場所に項目そのものを渡せます。",
            },
        ),
        heading(
            site,
            L {
                en: "What the page carries",
                ja: "ページに書き込まれるもの",
            },
        ),
        para(
            site,
            L {
                en: "`render` writes one element per item, built from the template with the item's initial values, so the list shows before the runtime loads, and the runtime keeps those elements rather than building them again:",
                ja: "`render` は、テンプレートと各項目の初期値から作った要素を項目ごとに 1 つずつ書き出します。そのため、リストはランタイムを読み込む前から表示され、ランタイムはそれらの要素を作り直さずにそのまま使います。",
            },
        ),
        show(Snippet::StateListRender),
        heading(
            site,
            L {
                en: "Matching items by key",
                ja: "キーで項目を対応させる",
            },
        ),
        para(
            site,
            L {
                en: "`each` matches items to elements by position. [`each_keyed`](api:hypersig/html/fn.each_keyed.html), `fn each_keyed<S, T, R, K, E: IntoExpr<S, K>>(list: Sig<S, Vec<T>, R>, key: impl FnOnce(Item<S, T>) -> E, render: impl FnOnce(Item<S, T>) -> El<S>) -> Node<S>`, matches them by a key, so an item keeps its element when the list is reordered or an item before it is removed, and focus and the state of its inputs move with it. Use it for a list the visitor or the server reorders, filters, or edits in the middle.",
                ja: "`each` は位置で項目と要素を対応させます。[`each_keyed`](api:hypersig/html/fn.each_keyed.html)（`fn each_keyed<S, T, R, K, E: IntoExpr<S, K>>(list: Sig<S, Vec<T>, R>, key: impl FnOnce(Item<S, T>) -> E, render: impl FnOnce(Item<S, T>) -> El<S>) -> Node<S>`）はキーで対応させるので、リストを並べ替えたり前の項目を削除したりしても項目は自分の要素を保ち、フォーカスや入力欄の状態も項目と一緒に移ります。訪問者やサーバーが並べ替えたり、絞り込んだり、途中を編集したりするリストに使います。",
            },
        ),
        para(
            site,
            L {
                en: "The key is an argument of `each_keyed`, so it can sit only on the root of the template. An element has no method that sets a key, and calling one is a compile error:",
                ja: "キーは `each_keyed` の引数なので、テンプレートの根の要素にしか置けません。要素にはキーを設定するメソッドがなく、呼び出そうとするとコンパイルエラーになります。",
            },
        ),
        show(Snippet::StateKeyOutside),
        para(
            site,
            L {
                en: "The types do not check that keys are unique. Where two items have one key, the runtime writes `hypersig: duplicate key ... in a list; matching it by position` to the console and matches the repeated one by its position.",
                ja: "キーが重複しないことは型では検査しません。2 つの項目が同じキーを持つと、ランタイムはコンソールに `hypersig: duplicate key ... in a list; matching it by position` と書き出し、重複した項目を位置で対応させます。",
            },
        ),
        heading(
            site,
            L {
                en: "Changing a list",
                ja: "リストを変更する",
            },
        ),
        para(
            site,
            L {
                en: "A list signal has statements that change it in the browser without a request. Each takes `self` and returns a `Stmt<S>`, and the closures receive the `Item` being tested or built:",
                ja: "リストのシグナルには、リクエストを送らずにブラウザでリストを変更する文があります。どれも `self` を受け取って `Stmt<S>` を返し、クロージャは調べている項目や作っている項目の `Item` を受け取ります。",
            },
        ),
        show(Snippet::ListStatements),
        table_of(
            site,
            L {
                en: &["Statement", "What it does"],
                ja: &["文", "動作"],
            },
            L {
                en: &[
                    &["`push(value)`", "appends one item"],
                    &["`append(items)`", "appends every item of another list"],
                    &[
                        "`prepend(items)`",
                        "adds items before the first one. With `each_keyed`, the items already on screen stay where they were, and the nearest scrollable ancestor scrolls by the height of the new items. With `each`, nothing scrolls.",
                    ],
                    &[
                        "`extend(count, |i| item)`",
                        "appends `count` items built from their positions",
                    ],
                    &[
                        "`remove_at(index)`",
                        "removes the item at `index`, if there is one",
                    ],
                    &["`clear()`", "empties the list"],
                    &[
                        "`retain(|item| keep)`",
                        "keeps the items for which `keep` is true",
                    ],
                    &["`swap(a, b)`", "exchanges two items, if both exist"],
                    &[
                        "`sort_by(|item| key)`, `sort_by_desc(|item| key)`",
                        "sorts ascending or descending by a `Comparable` key, keeping equal keys in order",
                    ],
                    &["`set_all(field, value)`", "sets one field on every item"],
                    &[
                        "`update_where(|item| when, field, |item| value)`",
                        "sets one field on each item `when` accepts, from a value that can read the item",
                    ],
                    &["`item.remove()`", "removes the item an `Item` stands for"],
                ],
                ja: &[
                    &["`push(value)`", "項目を 1 つ末尾に追加します"],
                    &[
                        "`append(items)`",
                        "別のリストの項目をすべて末尾に追加します",
                    ],
                    &[
                        "`prepend(items)`",
                        "先頭の項目の前に項目を追加します。`each_keyed` では、表示中の項目はその位置に留まり、最も近いスクロール可能な祖先が新しい項目の高さの分だけスクロールします。`each` ではスクロールしません。",
                    ],
                    &[
                        "`extend(count, |i| item)`",
                        "位置から作った項目を `count` 個、末尾に追加します",
                    ],
                    &[
                        "`remove_at(index)`",
                        "`index` の位置に項目があれば削除します",
                    ],
                    &["`clear()`", "リストを空にします"],
                    &["`retain(|item| keep)`", "`keep` が真の項目だけを残します"],
                    &["`swap(a, b)`", "2 つの項目が両方あれば入れ替えます"],
                    &[
                        "`sort_by(|item| key)`、`sort_by_desc(|item| key)`",
                        "`Comparable` なキーで昇順か降順に並べ替えます。キーが等しい項目は順序を保ちます",
                    ],
                    &[
                        "`set_all(field, value)`",
                        "すべての項目の 1 つのフィールドを設定します",
                    ],
                    &[
                        "`update_where(|item| when, field, |item| value)`",
                        "`when` が受け入れる各項目の 1 つのフィールドを、項目を読める値で設定します",
                    ],
                    &["`item.remove()`", "`Item` が表す項目を削除します"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each statement takes items of the list's own type, so pushing a value of another type is a compile error:",
                ja: "どの文もリスト自身の型の項目を受け取るので、別の型の値を追加しようとするとコンパイルエラーになります。",
            },
        ),
        show(Snippet::StatePushWrongType),
        heading(
            site,
            L {
                en: "Reading a list",
                ja: "リストを読む",
            },
        ),
        para(
            site,
            L {
                en: "Expressions over a list read it whole: `len` and `is_empty`, `nth(index)`, which gives an `Option`, and the aggregates `count`, `sum_of`, `max_of`, `min_of`, and `map`, whose closures receive each item in turn. `max_of` and `min_of` give `None` for an empty list, and `sum_of` gives 0:",
                ja: "リストを使った式はリスト全体を読みます。`len` と `is_empty`、`Option` を返す `nth(index)`、そしてクロージャが各項目を順に受け取る集計の `count`、`sum_of`、`max_of`、`min_of`、`map` です。`max_of` と `min_of` は空のリストで `None` を返し、`sum_of` は 0 を返します。",
            },
        ),
        show(Snippet::StateListExpressions),
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
                    en: "An `Item` belongs to the closure it was passed to, and the types do not stop it from being moved out. Reading the item after its closure returns panics, and so does rendering an expression, a statement, or an element built from it anywhere its list does not render.",
                    ja: "`Item` は渡されたクロージャに属しますが、外へ持ち出すことは型では防げません。クロージャが戻ったあとに項目を読むと panic し、項目から作った式、文、要素を、そのリストが描画されない場所で描画しても panic します。",
                },
                L {
                    en: "The closure runs once, so it cannot branch on an item's value in Rust. [The state of a list item](learn/list-items/) shows how to choose per item with expressions.",
                    ja: "クロージャは 1 回しか実行されないので、Rust の側で項目の値によって分岐することはできません。項目ごとに式で選ぶ方法は [リスト項目の状態](learn/list-items/) で示します。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "A to-do list",
                ja: "ToDo リスト",
            },
        ),
        para(
            site,
            L {
                en: "The demo builds a new item with `Expr::lit(&blank).with(...)`, which copies a constant struct and replaces two fields with expressions. The count of open items is an expression over the list.",
                ja: "このデモは `Expr::lit(&blank).with(...)` で新しい項目を作ります。この式は定数の構造体を複製し、2 つのフィールドを式で置き換えます。未完了の項目の数はリストを使った式です。",
            },
        ),
        demo::show(site, Demo::Todos),
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
                en: "[The state of a list item](learn/list-items/) covers the fields of one item and state the browser keeps per item. [List patches](learn/list-patches/) changes a list from the server.",
                ja: "[リスト項目の状態](learn/list-items/) では1 つの項目のフィールドと、ブラウザが項目ごとに持つ状態を説明します。[リストの差分](learn/list-patches/) ではサーバーからリストを変更します。",
            },
        ),
    ])
}
