//! The page on node refs and dialog refs, the names that statements calling
//! an element's DOM methods act on.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::NodeRefs).title(site)),
        lead(site, Doc::Learn(Learn::NodeRefs).leads()),
        heading(
            site,
            L {
                en: "Declaring a node ref",
                ja: "ノード参照を宣言する",
            },
        ),
        para(
            site,
            L {
                en: "A [`NodeRef<S>`](api:hypersig/struct.NodeRef.html) names one element of a page whose signals are `S`, so that a statement can call a DOM method on it, such as `focus()`. Declare it at module level with [`node_ref!`](api:hypersig/macro.node_ref.html), as `node_ref!(const FIELD: NodeRef<Search>)`, and attach it with [`El::node_ref`](api:hypersig/html/struct.El.html#method.node_ref), `fn node_ref(self, node_ref: NodeRef<S>) -> Self`.",
                ja: "[`NodeRef<S>`](api:hypersig/struct.NodeRef.html) は、シグナルが `S` のページにある要素を 1 つ指す名前です。文はこの名前を使って、`focus()` などの DOM のメソッドを要素に対して呼び出します。モジュールの直下で [`node_ref!`](api:hypersig/macro.node_ref.html) を使って `node_ref!(const FIELD: NodeRef<Search>)` のように宣言し、[`El::node_ref`](api:hypersig/html/struct.El.html#method.node_ref)（`fn node_ref(self, node_ref: NodeRef<S>) -> Self`）で要素に付けます。",
            },
        ),
        show(Snippet::NodeRefDeclare),
        para(
            site,
            L {
                en: "The name is the constant's path: the module path that `module_path!()` gives, then the identifier, such as `shop::search::FIELD`. Two constants have two paths, so their names differ.",
                ja: "名前は定数のパスで、`module_path!()` が返すモジュールのパスに `shop::search::FIELD` のように識別子を続けたものです。定数が 2 つあればパスも 2 つあるので、名前が重なることはありません。",
            },
        ),
        para(
            site,
            L {
                en: "`FIELD.focus()` is a statement like any other. A click on the button runs it, and the browser moves focus to the field.",
                ja: "`FIELD.focus()` はほかと同じ文です。ボタンをクリックするとこの文が実行され、ブラウザは入力欄にフォーカスを移します。",
            },
        ),
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
                en: "`node_ref!` inside a function body fails to compile with E0425, `cannot find value FIELD in module self`. Two functions of one module could otherwise each declare a constant with one identifier and give two elements one name.",
                ja: "関数の本体の中で `node_ref!` を書くと、E0425（`cannot find value FIELD in module self`）でコンパイルエラーになります。これを許すと、同じモジュールの 2 つの関数がそれぞれ同じ識別子の定数を宣言でき、2 つの要素が同じ名前になるからです。",
            },
        ),
        show(Snippet::NodeRefInFn),
        para(
            site,
            L {
                en: "The type parameter ties the reference to one page. A `NodeRef<Search>` on an element of a page whose signals are `Cart` is E0308, mismatched types:",
                ja: "型引数によって、参照は 1 つのページに結び付きます。`NodeRef<Search>` を、シグナルが `Cart` のページの要素に付けると、E0308（型の不一致）になります。",
            },
        ),
        show(Snippet::NodeRefOtherPage),
        heading(
            site,
            L {
                en: "The statements",
                ja: "文",
            },
        ),
        para(
            site,
            L {
                en: "Each method takes the reference by value, since `NodeRef` is `Copy`, and returns a `Stmt<S>`. A statement goes wherever one is taken, such as `on_click`, `on_key`, or a timer, and `then` runs another after it.",
                ja: "どのメソッドも参照を値で受け取り（`NodeRef` は `Copy` です）、`Stmt<S>` を返します。文は `on_click`、`on_key`、タイマーなど、文を受け取るところならどこにでも渡せます。`then` を使うと、続けて別の文を実行できます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "What the browser does"],
                ja: &["メソッド", "ブラウザが行うこと"],
            },
            L {
                en: &[
                    &[
                        "`fn focus(self) -> Stmt<S>`",
                        "Focuses the element after the next render",
                    ],
                    &[
                        "`fn blur(self) -> Stmt<S>`",
                        "Removes focus from the element",
                    ],
                    &[
                        "`fn select(self) -> Stmt<S>`",
                        "Selects the text of an `<input>`",
                    ],
                    &[
                        "`fn scroll_into_view(self) -> Stmt<S>`",
                        "Scrolls the element into view",
                    ],
                    &[
                        "`fn show_popover(self) -> Stmt<S>`",
                        "Shows the element if it is a closed popover",
                    ],
                    &[
                        "`fn hide_popover(self) -> Stmt<S>`",
                        "Hides the element if it is an open popover. Where focus was inside it, the browser returns focus to the element that had it before the popover opened",
                    ],
                ],
                ja: &[
                    &[
                        "`fn focus(self) -> Stmt<S>`",
                        "次の描画のあとで要素にフォーカスします",
                    ],
                    &["`fn blur(self) -> Stmt<S>`", "要素からフォーカスを外します"],
                    &[
                        "`fn select(self) -> Stmt<S>`",
                        "`<input>` のテキストを選択します",
                    ],
                    &[
                        "`fn scroll_into_view(self) -> Stmt<S>`",
                        "要素が見える位置までスクロールします",
                    ],
                    &[
                        "`fn show_popover(self) -> Stmt<S>`",
                        "要素が閉じたポップオーバーなら表示します",
                    ],
                    &[
                        "`fn hide_popover(self) -> Stmt<S>`",
                        "要素が開いたポップオーバーなら隠します。フォーカスがポップオーバーの中にあった場合、ブラウザはポップオーバーを開く前にフォーカスを持っていた要素にフォーカスを戻します",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`NodeRef::id`, `fn id(self) -> Id`, gives an element id spelled as the reference's name. A component that names its element by a `NodeRef` uses it for the `id` that an `aria-*` attribute points at.",
                ja: "`NodeRef::id`（`fn id(self) -> Id`）は、参照の名前と同じつづりの要素の id を返します。`NodeRef` で要素に名前を付けるコンポーネントは、`aria-*` 属性が指す `id` にこれを使います。",
            },
        ),
        heading(
            site,
            L {
                en: "Inside a list",
                ja: "リストの中で使う",
            },
        ),
        para(
            site,
            L {
                en: "Each item of a list may carry the same constant. Inside a list item the browser looks for the element in the same item first, so one `NodeRef` serves every item. Below, each Rename button selects the title in its own card:",
                ja: "リストの項目ごとに同じ定数を付けてかまいません。リストの項目の中では、ブラウザはまず同じ項目の中で要素を探すので、1 つの `NodeRef` をすべての項目で使えます。下の例では、それぞれの Rename ボタンが自分のカードのタイトルを選択します。",
            },
        ),
        show(Snippet::NodeRefList),
        heading(
            site,
            L {
                en: "One name on one element",
                ja: "1 つの名前は 1 つの要素に",
            },
        ),
        para(
            site,
            L {
                en: "The compiler does not see which elements carry a name. Outside a list template, one constant on two elements that the browser can both reach makes [`Page::render`](api:hypersig/struct.Page.html#method.render) and `El::render` panic with `two elements carry the node ref ... where the runtime can reach both`. A test that renders the page finds it. A fixed id from `El::id` on two elements panics the same way.",
                ja: "どの要素に名前が付いているかは、コンパイラには分かりません。リストのテンプレートの外で、ブラウザがどちらにも届く 2 つの要素に 1 つの定数を付けると、[`Page::render`](api:hypersig/struct.Page.html#method.render) と `El::render` が `two elements carry the node ref ... where the runtime can reach both` というメッセージで panic します。ページを描画するテストを書けば見つかります。`El::id` で同じ固定の id を 2 つの要素に付けた場合も、同じように panic します。",
            },
        ),
        show(Snippet::NodeRefTwice),
        para(
            site,
            L {
                en: "One element takes one node ref. A second call of `El::node_ref`, or `El::node_ref` together with `El::foreign`, panics when the element is built. A [`Fragment`](api:hypersig/trait.Fragment.html) sent again in a patch is rendered on its own, so the server cannot compare it with the rest of the page. For that case, the browser writes a warning to the console when the patch adds a name or an id that another element it can reach already carries. The name of a `FileInput` is looked up apart from these names, so it never collides with one.",
                ja: "1 つの要素に付けられるノード参照は 1 つです。`El::node_ref` を 2 回呼ぶか、`El::node_ref` と `El::foreign` を両方呼ぶと、要素を組み立てるときに panic します。パッチで送り直す [`Fragment`](api:hypersig/trait.Fragment.html) は単独で描画されるので、サーバーはページのほかの部分と突き合わせられません。この場合は、ブラウザが届く範囲のほかの要素がすでに持つ名前や id をパッチが追加したときに、ブラウザがコンソールに警告を出します。`FileInput` の名前はこれらの名前とは別に探すので、重なることはありません。",
            },
        ),
        heading(
            site,
            L {
                en: "Dialogs",
                ja: "ダイアログ",
            },
        ),
        para(
            site,
            L {
                en: "A [`DialogRef<S>`](api:hypersig/struct.DialogRef.html) names a `<dialog>`. Declare it with [`dialog_ref!`](api:hypersig/macro.dialog_ref.html), which takes the same form as `node_ref!`. `fn dialog(self) -> El<S>` builds the empty `<dialog>` element the constant names, and it is the only way to attach one, so its statements always act on a dialog:",
                ja: "[`DialogRef<S>`](api:hypersig/struct.DialogRef.html) は `<dialog>` を指す名前です。`node_ref!` と同じ形の [`dialog_ref!`](api:hypersig/macro.dialog_ref.html) で宣言します。`fn dialog(self) -> El<S>` は定数が指す空の `<dialog>` 要素を作ります。参照を付ける方法はこれしかないので、`DialogRef` の文は必ずダイアログに対して実行されます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "What the browser does"],
                ja: &["メソッド", "ブラウザが行うこと"],
            },
            L {
                en: &[
                    &[
                        "`fn show_modal(self) -> Stmt<S>`",
                        "Opens the dialog with `showModal()`, unless it is open. The browser makes the rest of the page inert, moves focus into the dialog, and closes it on Escape",
                    ],
                    &[
                        "`fn close(self) -> Stmt<S>`",
                        "Closes the dialog, if it is open. The browser returns focus to the element that had it before the dialog opened",
                    ],
                ],
                ja: &[
                    &[
                        "`fn show_modal(self) -> Stmt<S>`",
                        "ダイアログが開いていなければ `showModal()` で開きます。ブラウザはページのほかの部分を操作できなくし、フォーカスをダイアログの中に移し、Escape でダイアログを閉じます",
                    ],
                    &[
                        "`fn close(self) -> Stmt<S>`",
                        "ダイアログが開いていれば閉じます。ブラウザは、ダイアログを開く前にフォーカスを持っていた要素にフォーカスを戻します",
                    ],
                ],
            },
        ),
        show(Snippet::DialogRefs),
        para(
            site,
            L {
                en: "A `DialogRef` and a `NodeRef` share one set of names, and rendering panics where the dialog of one constant sits twice where the browser can reach both.",
                ja: "`DialogRef` と `NodeRef` の名前は同じ集合に属します。1 つの定数のダイアログが、ブラウザがどちらにも届く 2 か所にあると、描画が panic します。",
            },
        ),
        heading(
            site,
            L {
                en: "Example: a stopwatch",
                ja: "例：ストップウォッチ",
            },
        ),
        para(
            site,
            L {
                en: "The stopwatch below counts tenths of a second with `on_interval`. The reset button stops it, sets the count to zero, and moves focus to the start button through the node ref `START`.",
                ja: "下のストップウォッチは `on_interval` で 0.1 秒ずつ数えます。リセットボタンは計測を止めて数を 0 に戻し、ノード参照 `START` を通してスタートボタンにフォーカスを移します。",
            },
        ),
        demo::show(site, Demo::Stopwatch),
        para(
            site,
            L {
                en: "Next, [Timers and reactions](learn/timers/) covers `on_interval` and the other statements that run without an event, and [Keyboard patterns](learn/keyboard/) moves focus with the arrow keys.",
                ja: "次の [タイマーと変化への反応](learn/timers/) では`on_interval` をはじめ、イベントなしで実行される文を説明します。[キーボード操作](learn/keyboard/) では矢印キーでフォーカスを動かす方法を説明します。",
            },
        ),
    ])
}
