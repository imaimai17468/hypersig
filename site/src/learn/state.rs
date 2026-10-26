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
        lead(site, Doc::Learn(Learn::Signals).leads()),
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
                en: "A component of `hypersig_ui` that keeps state of its own, such as `Combobox` or `DataTable`, takes it as a `Sig` of its state struct, so that state is a field of the page's struct too. Gather those fields into one struct that derives `Default`, and the page's struct gains one field, set with `Default::default()`. Adding a component then adds a field to that struct and leaves the page's initializer as it is. `ComboboxState` and `TableState`, the state structs of `hypersig_ui`, implement `Default`, which gives an empty, closed combobox and an unfiltered, unsorted table on its first page.",
                ja: "`Combobox` や `DataTable` のように自分の状態を持つ `hypersig_ui` の部品は、その状態を状態用の構造体の `Sig` で受け取るので、その状態もページの構造体のフィールドになります。こうしたフィールドを `Default` を実装する 1 つの構造体にまとめると、ページの構造体に増えるフィールドは 1 つで、初期化は `Default::default()` で済みます。部品を足すときはその構造体にフィールドを足すだけで、ページの初期化は変わりません。`hypersig_ui` の状態用の構造体である `ComboboxState` と `TableState` は `Default` を実装しています。その値は、空で閉じたコンボボックスと、絞り込みも並べ替えもない 1 ページ目の表です。",
            },
        ),
        show(Snippet::ComponentState),
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
                en: "`Page::computed(signal, value)` keeps a field equal to an expression over other fields. Bindings read it like any field and run again only when its value changes, so no binding evaluates the expression itself. Computed signals are evaluated in the order they are declared, so one may read an earlier one. The field is marked `#[signals(computed)]` in the struct, and `Page::computed` accepts only a field with that mark.",
                ja: "`Page::computed(signal, value)` は、フィールドをほかのフィールドを使った式の値に合わせ続けます。束縛はそれぞれ式を評価する代わりに、ほかのフィールドと同じようにこの値を読み、値が変わったときだけ評価し直します。計算されるシグナルは宣言した順に評価されるので、先に宣言したものを読めます。構造体ではそのフィールドに `#[signals(computed)]` を付けます。`Page::computed` が受け取るのは、この印の付いたフィールドだけです。",
            },
        ),
        demo::show(site, Demo::Cart),
        note(
            site,
            L {
                en: "The accessor of a field marked `#[signals(computed)]` returns a `Sig<S, T, Computed>`, which has no `set`, `toggle`, list statement, or `patch`, and which `bind` refuses. A statement that writes it or a patch that sets it is a compile error, because the browser would overwrite that value the next time a field the expression reads changes. A field of a nested struct with the mark is reached with `at_computed` instead of `at`.",
                ja: "`#[signals(computed)]` を付けたフィールドのアクセサーは `Sig<S, T, Computed>` を返します。この型には `set`、`toggle`、リストの文、`patch` がなく、`bind` にも渡せません。書き込む文や設定する差分を書くとコンパイルエラーになります。書き込んだ値は、式が読むフィールドが次に変わったときにブラウザが上書きするからです。入れ子の構造体にある印の付いたフィールドには、`at` の代わりに `at_computed` でたどり着きます。",
            },
        ),
    ])
}

pub fn expressions(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Expressions).title(site)),
        lead(site, Doc::Learn(Learn::Expressions).leads()),
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
        para(
            site,
            L {
                en: "A `Sig` is `Copy`, so `price` and `quantity` appear in two expressions each as they are. An `Expr` is not, and an operator or a method takes it by value, so an expression used in two places is cloned first, as `subtotal.clone()` is.",
                ja: "`Sig` は `Copy` なので、`price` と `quantity` はそのまま 2 つの式に使えます。`Expr` は `Copy` ではなく、演算子やメソッドに値として渡すと使えなくなります。そのため、2 か所で使う式は `subtotal.clone()` のように先に複製します。",
            },
        ),
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
                        "`+`、`-`、`*`、`%`、`min`、`max`、`to_fixed`。`f64` では `/`、`floor`、`ceil`、`round`、`log10`、`pow` も使えます。`to_f64` は `i32` を `f64` に変えます。`format` は `NumberFormat` に従って数をテキストにし、桁を区切って前後に文字を付けます。",
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
                en: "`on_click`, `on_double_click`, `on_input`, `on_submit`, `on_change`, and `on_blur` run a statement on their event, and `on(Event::Focus, ...)` covers the rest of [`Event`](api:hypersig/html/enum.Event.html), such as `KeyDown`, `MouseEnter`, and `MouseLeave`. `on_load` runs a statement once when the element enters the document, as a stream that sends changes made elsewhere needs. `on_key` filters `keydown` by a [`Key`](api:hypersig/html/enum.Key.html) or a [`Shortcut`](api:hypersig/html/struct.Shortcut.html) such as `Key::Char('k').primary()`, and leaves the characters a text field inside the element types to the field. [`Page::on_key`](api:hypersig/struct.Page.html#method.on_key) does the same for the whole page, even while nothing has focus. `on_visible` runs each time the element scrolls into view, which loads the next page of a list. `on_debounced` waits until the event has stopped firing for a given time.",
                ja: "`on_click`、`on_double_click`、`on_input`、`on_submit`、`on_change`、`on_blur` は、それぞれのイベントで文を実行します。`KeyDown`、`MouseEnter`、`MouseLeave` など、[`Event`](api:hypersig/html/enum.Event.html) のほかのイベントには `on(Event::Focus, ...)` を使います。`on_load` は、要素が文書に入ったときに文を 1 回実行します。ほかの場所で起きた変更を送るストリームは、ここから開きます。`on_key` は `keydown` を [`Key`](api:hypersig/html/enum.Key.html) か、`Key::Char('k').primary()` のような [`Shortcut`](api:hypersig/html/struct.Shortcut.html) で絞り込みます。要素の中のテキスト欄に入力する文字は、その欄に任せます。[`Page::on_key`](api:hypersig/struct.Page.html#method.on_key) は同じことをページ全体で行い、どこにもフォーカスがないときにも動きます。`on_visible` は要素がスクロールして見えるたびに実行し、リストの続きを読み込むのに使えます。`on_debounced` は、イベントが起きない状態が指定した時間続くまで待ちます。",
            },
        ),
        para(
            site,
            L {
                en: "`bind` ties a text field, a checkbox, or a number field to a signal in both directions. `bind_radio` and `bind_select` do the same for radio buttons and a `<select>`. While the visitor is typing in a bound field, a change to its signal does not rewrite the field. Where an action's answer sends back text the field held since the typing began, such as the text an autosave sent before the visitor typed on, the field's text goes back into the signal. Any other value stays in the signal, where a call made in the meantime sends it, and shows in the field when the typing ends: when the visitor presses Enter, submits the form, or moves focus away. Typing on before that puts the field's text in its place. A change after the typing ends shows at once, so a draft the server empties after a submit empties while the field still has focus.",
                ja: "`bind` はテキスト欄、チェックボックス、数値の欄をシグナルと双方向に結び付けます。ラジオボタンと `<select>` には `bind_radio` と `bind_select` で同じことをします。訪問者が入力している途中の欄は、シグナルが変わっても書き換えません。自動保存が送ったあとに訪問者が入力を続けた場合のように、入力を始めてから欄が持っていた文字をアクションの応答が送り返したときは、欄の文字をシグナルに戻します。それ以外の値はシグナルに入り、その間に呼んだアクションにはその値が送られます。欄には、訪問者が Enter キーを押すか、フォームを送信するか、フォーカスを移して入力を終えたときに表示されます。それより前に入力を続けると、欄の文字がシグナルに入ります。入力を終えたあとの変更はすぐに表示されるので、送信のあとにサーバーが下書きを空にすると、フォーカスが欄にあるままでも欄は空になります。",
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
                    &["1 つの項目", "`item.remove()`、`item.at(field)` の `set`"],
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
                en: "`Sig` has a third type parameter that says where its path starts. It defaults to `Root`, the page's signal struct, so `Sig<S, T>` is a field reached from the page. A field of an item is a `Sig<S, T, Local>`, whose path starts at the item and is resolved per item in the browser. It has no `patch` method, so the server cannot address one item's field by position. The server instead replaces the whole list with a patch of the list field, changes the items it names by key with a list patch such as `patch_append` or `patch_remove`, or re-renders a part of the page as a fragment. [Calls in depth](learn/calls/#list-patches) covers list patches, and [Talking to the server](learn/server/) covers fragments.",
                ja: "`Sig` には、パスの起点を表す 3 つ目の型パラメータがあります。省略すると `Root`（ページのシグナル構造体）になるので、`Sig<S, T>` はページから辿ったフィールドです。項目のフィールドは `Sig<S, T, Local>` で、パスは項目から始まり、ブラウザで項目ごとに解決されます。`patch` メソッドがないので、サーバーは位置を指定して 1 つの項目のフィールドを書き換えられません。サーバーは、リストのフィールドの差分でリスト全体を置き換えるか、`patch_append` や `patch_remove` のようなリストの差分でキーを指定した項目だけを変えるか、ページの一部をフラグメントとして描画し直します。リストの差分は [呼び出しの詳細](learn/calls/#list-patches) で、フラグメントは [サーバーとのやり取り](learn/server/) で説明します。",
            },
        ),
        subheading(
            site,
            L {
                en: "Browser-only state of an item",
                ja: "ブラウザだけで持つ項目の状態",
            },
        ),
        para(
            site,
            L {
                en: "A patch from the server writes each item it carries whole. `patch_upsert` puts the server's item in place of the one with the same key, and `patch` replaces the whole list. A field held on the item only for the browser, such as whether a card is open for editing, takes the value the server sent, so a card someone is editing closes when another user's change to it arrives. Keep such state at the page's root as the key of the item it applies to, and compare it with each item's key:",
                ja: "サーバーからの差分は、運ぶ項目を丸ごと書き込みます。`patch_upsert` は同じキーの項目をサーバーの項目で置き換え、`patch` はリスト全体を置き換えます。カードを編集中かどうかのように、ブラウザのためだけに項目に持たせたフィールドはサーバーが送った値になるので、編集中のカードはほかのユーザーの変更が届いた時点で閉じます。こうした状態は、対象の項目のキーとしてページの構造体の直下に持ち、各項目のキーと比べてください。",
            },
        ),
        show(Snippet::EditState),
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
