//! The pages on signals, expressions and statements, and lists.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, subheading, table_of, title};

pub fn signals(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Signals).title(site)),
        lead(
            site,
            L {
                en: "The page's state is one Rust struct, and a signal is a typed reference to one of its fields.",
                ja: "ページの状態は Rust の構造体 1 つで表し、シグナルはそのフィールド 1 つを指す型付きの参照です。",
            },
        ),
        heading(
            site,
            L {
                en: "The signal struct",
                ja: "シグナル構造体",
            },
        ),
        para(
            site,
            L {
                en: "Derive `Signals` next to serde's `Serialize` and `Deserialize`. The derive adds one function per field, named after the field, that returns a [`Sig`](api:hypersig/struct.Sig.html). `Sig<Counter, i32>` reads: an `i32` inside the signal struct `Counter`.",
                ja: "serde の `Serialize` と `Deserialize` と並べて `Signals` を derive します。この derive はフィールドごとにそのフィールドと同じ名前で [`Sig`](api:hypersig/struct.Sig.html) を返す関数を追加します。`Sig<Counter, i32>` は「シグナル構造体 `Counter` の中の `i32`」と読みます。",
            },
        ),
        show(Snippet::SignalPaths),
        para(
            site,
            L {
                en: "The browser keeps the struct as one JSON object, and every expression, statement, and patch names its fields through a `Sig`. A misspelled field has no function to call, so it fails to compile:",
                ja: "ブラウザは構造体を 1 つの JSON オブジェクトとして持ち、式も文も差分も `Sig` を通してそのフィールドを指します。打ち間違えたフィールド名には呼び出せる関数がないので、次のようにコンパイルが通りません。",
            },
        ),
        show(Snippet::WrongField),
        para(
            site,
            L {
                en: "The `S` in `Sig<S, T>` also stops a signal of one page from being used on another. An element of a `Counter` page is an `El<Counter>`, and it accepts only expressions over `Counter`.",
                ja: "`Sig<S, T>` の `S` はあるページのシグナルを別のページで使うことも防ぎます。`Counter` のページの要素は `El<Counter>` で、`Counter` のシグナルを使った式しか受け取りません。",
            },
        ),
        heading(
            site,
            L {
                en: "Fields inside fields",
                ja: "フィールドの中のフィールド",
            },
        ),
        para(
            site,
            L {
                en: "A field may itself be a `Signals` struct. `at` reaches into it, and the result is a `Sig` like any other. A path is at most 8 fields deep.",
                ja: "フィールド自体を `Signals` の構造体にもできます。その中へは `at` でたどり、結果はほかと同じ `Sig` になります。パスの深さはフィールド 8 つまでです。",
            },
        ),
        show(Snippet::Nested),
        para(
            site,
            L {
                en: "A `Vec` of values or of structs is a list, which [Lists](learn/lists/) covers. An `Option` field has `is_some`, `is_none`, and `unwrap_or`.",
                ja: "値や構造体の `Vec` はリストで、[リスト](learn/lists/) で説明します。`Option` のフィールドには `is_some`、`is_none`、`unwrap_or` があります。",
            },
        ),
        heading(
            site,
            L {
                en: "The first render",
                ja: "最初の描画",
            },
        ),
        para(
            site,
            L {
                en: "`Page::new` takes the initial values, and `Page::render` evaluates every binding with them. Text, visibility, classes, attributes, form values, and list items are in the HTML before the runtime loads, and the runtime uses those elements as they are rather than building them again. The browser fetches nothing to show the page, because the data is the signal struct's JSON on `<body>`.",
                ja: "`Page::new` が初期値を受け取り、`Page::render` がその値ですべての束縛を評価します。テキスト、表示と非表示、クラス、属性、フォームの値、リストの項目は、ランタイムを読み込む前から HTML に入っています。ランタイムはそれらの要素を作り直さず、そのまま使います。データは `<body>` に置いたシグナル構造体の JSON なので、ブラウザはページを表示するために何も取得しません。",
            },
        ),
        heading(
            site,
            L {
                en: "Computed signals",
                ja: "計算されるシグナル",
            },
        ),
        para(
            site,
            L {
                en: "`Page::computed(signal, value)` keeps a field equal to an expression over other fields. Bindings read it like any field and run again only when its value changes, rather than each evaluating the expression. Computed signals are evaluated in the order they are declared, so one may read an earlier one.",
                ja: "`Page::computed(signal, value)` は、フィールドをほかのフィールドを使った式の値に合わせ続けます。束縛はそれぞれ式を評価する代わりに、ほかのフィールドと同じようにこの値を読み、値が変わったときだけ評価し直します。計算されるシグナルは宣言した順に評価されるので、先に宣言したものを読めます。",
            },
        ),
        demo::show(site, Demo::Cart),
        note(
            site,
            L {
                en: "Nothing stops a statement or the server from setting a computed field. The value it sets lasts until a field the expression reads changes.",
                ja: "文やサーバーが計算されるフィールドに値を設定することは止められません。設定した値は、式が読むフィールドが変わるまで残ります。",
            },
        ),
    ])
}

pub fn expressions(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Expressions).title(site)),
        lead(
            site,
            L {
                en: "An expression computes a value from the signals, and a statement changes them. Both are Rust values that the browser receives as JSON.",
                ja: "式はシグナルから値を計算し、文はシグナルを変更します。どちらも Rust の値で、ブラウザは JSON として受け取ります。",
            },
        ),
        heading(
            site,
            L {
                en: "Expressions",
                ja: "式",
            },
        ),
        para(
            site,
            L {
                en: "An [`Expr<S, T>`](api:hypersig/struct.Expr.html) evaluates to a `T` in the browser. A `Sig` is an expression, and the operators and methods build larger ones. The server evaluates the same expressions for the first HTML.",
                ja: "[`Expr<S, T>`](api:hypersig/struct.Expr.html) は、ブラウザで評価すると `T` になります。`Sig` も式で、演算子やメソッドで大きな式を組み立てます。最初の HTML を作るときは、サーバーが同じ式を評価します。",
            },
        ),
        show(Snippet::Operators),
        table_of(
            site,
            L {
                en: &["On", "Operations"],
                ja: &["対象", "操作"],
            },
            L {
                en: &[
                    &[
                        "`i32` and `f64`",
                        "`+`, `-`, `*`, `%`, `min`, `max`, and `to_fixed`. `f64` adds `/`, `floor`, `ceil`, `round`, `log10`, and `pow`. `to_f64` turns an `i32` into an `f64`, and `format` writes a number in a `NumberFormat`, with grouped digits and affixes.",
                    ],
                    &[
                        "`String`",
                        "`concat`, `trim`, `to_lowercase`, `to_uppercase`, `contains`, `starts_with`, `is_empty`, `char_count`",
                    ],
                    &["`bool`", "`and`, `or`, `!`"],
                    &[
                        "Ordered values",
                        "`gt`, `ge`, `lt`, `le` on `i32`, `f64`, and `String`",
                    ],
                    &[
                        "Any value",
                        "`equals`, `not_equals`, `to_text` for display, `if_else(cond, then, otherwise)`",
                    ],
                    &[
                        "Structs",
                        "`at` for a field, `with` for a copy with one field replaced. `let_in` names any value once for the expression built from it.",
                    ],
                ],
                ja: &[
                    &[
                        "`i32` と `f64`",
                        "`+`、`-`、`*`、`%`、`min`、`max`、`to_fixed`。`f64` では `/`、`floor`、`ceil`、`round`、`log10`、`pow` も使えます。`to_f64` は `i32` を `f64` に変えます。`format` は数を `NumberFormat` に従い、桁を区切って前後に文字を付けたテキストにします。",
                    ],
                    &[
                        "`String`",
                        "`concat`、`trim`、`to_lowercase`、`to_uppercase`、`contains`、`starts_with`、`is_empty`、`char_count`",
                    ],
                    &["`bool`", "`and`、`or`、`!`"],
                    &[
                        "順序のある値",
                        "`i32`、`f64`、`String` の `gt`、`ge`、`lt`、`le`",
                    ],
                    &[
                        "任意の値",
                        "`equals`、`not_equals`、表示用の `to_text`、`if_else(cond, then, otherwise)`",
                    ],
                    &[
                        "構造体",
                        "フィールドを読む `at`、1 つのフィールドを置き換えた複製を作る `with`。`let_in` はどの値にも使え、値に一度だけ名前を付けて、それを使う式を作ります。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`Expr::lit(&value)` turns a Rust value into a constant expression, and most methods accept a plain `&value` where they take an expression. An expression of the wrong type is a compile error, with a message that names the type the place expects:",
                ja: "`Expr::lit(&value)` は Rust の値を定数の式に変えます。式を受け取るメソッドの多くは、`&value` をそのまま受け取れます。型の違う式はコンパイルエラーになり、メッセージにはその場所が求める型が出ます。",
            },
        ),
        show(Snippet::WrongType),
        heading(
            site,
            L {
                en: "Statements",
                ja: "文",
            },
        ),
        para(
            site,
            L {
                en: "A [`Stmt<S>`](api:hypersig/struct.Stmt.html) runs in the browser on an event. `sig.set(value)` assigns, `flag.toggle()` flips a `bool`, and `then` runs one statement after another. `Stmt::when` and `Stmt::if_else` branch on a condition.",
                ja: "[`Stmt<S>`](api:hypersig/struct.Stmt.html) は、イベントが起きたときにブラウザで実行されます。`sig.set(value)` は値を代入し、`flag.toggle()` は `bool` を反転し、`then` は文を順に実行します。`Stmt::when` と `Stmt::if_else` は条件で分岐します。",
            },
        ),
        show(Snippet::Statements),
        heading(
            site,
            L {
                en: "Events",
                ja: "イベント",
            },
        ),
        para(
            site,
            L {
                en: "`on_click`, `on_input`, `on_submit`, `on_change`, and `on_blur` run a statement on their event, and `on(Event::Focus, ...)` covers the rest of [`Event`](api:hypersig/html/enum.Event.html). `on_key` filters `keydown` by a [`Key`](api:hypersig/html/enum.Key.html), and `on_debounced` waits until the event has stopped firing for a given time.",
                ja: "`on_click`、`on_input`、`on_submit`、`on_change`、`on_blur` は、それぞれのイベントで文を実行します。[`Event`](api:hypersig/html/enum.Event.html) のほかのイベントには `on(Event::Focus, ...)` を使います。`on_key` は `keydown` を [`Key`](api:hypersig/html/enum.Key.html) で絞り込み、`on_debounced` はイベントが指定した時間続けて起きなくなるまで待ちます。",
            },
        ),
        para(
            site,
            L {
                en: "`bind` ties a text field, a checkbox, or a number field to a signal in both directions. `bind_radio` and `bind_select` do the same for radio buttons and a `<select>`.",
                ja: "`bind` は、テキスト欄、チェックボックス、数値の欄をシグナルと双方向に結び付けます。ラジオボタンと `<select>` には `bind_radio` と `bind_select` で同じことをします。",
            },
        ),
        show(Snippet::Events),
        para(
            site,
            L {
                en: "The converter below computes Fahrenheit and a description from one `f64`, and each button is one statement.",
                ja: "下の変換器は 1 つの `f64` から華氏の値と説明文を計算します。ボタンはそれぞれ 1 つの文です。",
            },
        ),
        demo::show(site, Demo::Temperature),
        note(
            site,
            L {
                en: "The browser runs only the operations hypersig provides. Anything else, such as formatting a date or calling a JavaScript library, goes through an action to the server, and its result comes back one network round trip later.",
                ja: "ブラウザが実行できるのは hypersig が用意した操作だけです。日付の整形や JavaScript ライブラリの呼び出しのような、それ以外の処理はアクションでサーバーに任せることになり、結果はネットワークの往復 1 回分だけ遅れて返ります。",
            },
        ),
    ])
}

pub fn lists(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Lists).title(site)),
        lead(
            site,
            L {
                en: "A `Vec` field is a list. `each` renders an element per item in the browser, and list statements change it without a request.",
                ja: "`Vec` のフィールドはリストです。`each` はブラウザで項目ごとに要素を描画し、リストの文はリクエストを送らずにリストを変更します。",
            },
        ),
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
                en: "[`each`](api:hypersig/html/fn.each.html) takes the list and a function that builds the element for one item. The function runs once on the server to build a template, receiving an [`Item`](api:hypersig/struct.Item.html) that stands for whichever item the browser is rendering. `item.at(field)` reads a field of the item, `item.index()` its position, and `item.value()` the item itself.",
                ja: "[`each`](api:hypersig/html/fn.each.html) は、リストと、項目 1 つ分の要素を組み立てる関数を受け取ります。この関数はテンプレートを作るためにサーバーで 1 回だけ実行され、ブラウザがそのとき描画している項目を表す [`Item`](api:hypersig/struct.Item.html) を受け取ります。項目のフィールドは `item.at(field)` で、位置は `item.index()` で、項目そのものは `item.value()` で読みます。",
            },
        ),
        show(Snippet::Each),
        para(
            site,
            L {
                en: "`each` matches items to elements by position. [`each_keyed`](api:hypersig/html/fn.each_keyed.html) matches them by a key, so an item keeps its element when the list is reordered or an item before it is removed, and focus and the state of its inputs move with it.",
                ja: "`each` は位置で項目と要素を対応させます。[`each_keyed`](api:hypersig/html/fn.each_keyed.html) はキーで対応させるので、リストを並べ替えたり前の項目を削除したりしても項目は自分の要素を保ち、フォーカスや入力欄の状態も項目と一緒に移ります。",
            },
        ),
        heading(
            site,
            L {
                en: "Changing a list",
                ja: "リストを変更する",
            },
        ),
        show(Snippet::ListStatements),
        table_of(
            site,
            L {
                en: &["Kind", "Methods on a list signal"],
                ja: &["種類", "リストのシグナルのメソッド"],
            },
            L {
                en: &[
                    &[
                        "Statements",
                        "`push`, `append`, `extend`, `remove_at`, `clear`, `retain`, `swap`, `sort_by`, `set_all`, `update_where`",
                    ],
                    &[
                        "Expressions",
                        "`len`, `is_empty`, `nth`, `map`, `count`, `sum_of`, `max_of`, `min_of`",
                    ],
                    &[
                        "On one item",
                        "`item.remove()`, and `set` on `item.at(field)`",
                    ],
                ],
                ja: &[
                    &[
                        "文",
                        "`push`、`append`、`extend`、`remove_at`、`clear`、`retain`、`swap`、`sort_by`、`set_all`、`update_where`",
                    ],
                    &[
                        "式",
                        "`len`、`is_empty`、`nth`、`map`、`count`、`sum_of`、`max_of`、`min_of`",
                    ],
                    &["1 つの項目に", "`item.remove()`、`item.at(field)` の `set`"],
                ],
            },
        ),
        subheading(
            site,
            L {
                en: "Item signals stay in the browser",
                ja: "項目のシグナルはブラウザの中だけにある",
            },
        ),
        para(
            site,
            L {
                en: "A field of an item is a `Sig<S, T, Local>`, whose path is resolved per item in the browser. It has no `patch` method, so the server cannot address one item's field by position. The server replaces the whole list with a patch of the list field, or re-renders a part of the page as a fragment, which [Talking to the server](learn/server/) covers.",
                ja: "項目のフィールドは `Sig<S, T, Local>` で、そのパスはブラウザで項目ごとに解決されます。`patch` メソッドがないので、サーバーは位置を指定して 1 つの項目のフィールドを書き換えられません。サーバーはリストのフィールドの差分でリスト全体を置き換えるか、ページの一部をフラグメントとして描画し直します。これは [サーバーとのやり取り](learn/server/) で説明します。",
            },
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
    ])
}
