//! The page on the `const` handles of `Dialog`, `Menu`, `Popover`,
//! `Tooltip`, and `Combobox`: building one, why it is `const`, and what a
//! name used twice does.

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
        lead(site, Doc::Components(Component::Handles).leads()),
        demo::show(site, Demo::Menu),
        building(site),
        why_const(site),
        twice(site),
        example(site),
        next(site),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a handle",
                ja: "ハンドルを作る",
            },
        ),
        para(
            site,
            L {
                en: "Five components are split into a button and the element it opens, or a field and its list, which the page places apart. Each is a handle built by a `const fn new` from a reference that [`node_ref!`](api:hypersig/macro.node_ref.html) or [`dialog_ref!`](api:hypersig/macro.dialog_ref.html) declares:",
                ja: "5 つのコンポーネントは、ボタンとそれが開く要素、または入力欄とその一覧に分かれていて、ページはそれらを別々の場所に置きます。どれも、[`node_ref!`](api:hypersig/macro.node_ref.html) か [`dialog_ref!`](api:hypersig/macro.dialog_ref.html) で宣言した参照から `const fn new` で作るハンドルです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Handle", "Constructor", "Built from"],
                ja: &["ハンドル", "コンストラクタ", "元になる参照"],
            },
            L {
                en: &[
                    &[
                        "[`Dialog<S>`](api:hypersig_ui/struct.Dialog.html)",
                        "`const fn new(dialog: DialogRef<S>) -> Self`",
                        "one `dialog_ref!`",
                    ],
                    &[
                        "[`Menu<S>`](api:hypersig_ui/struct.Menu.html)",
                        "`const fn new(menu: NodeRef<S>) -> Self`",
                        "one `node_ref!`",
                    ],
                    &[
                        "[`Popover<S>`](api:hypersig_ui/struct.Popover.html)",
                        "`const fn new(panel: NodeRef<S>) -> Self`",
                        "one `node_ref!`",
                    ],
                    &[
                        "[`Tooltip<S>`](api:hypersig_ui/struct.Tooltip.html)",
                        "`const fn new(tip: NodeRef<S>) -> Self`",
                        "one `node_ref!`",
                    ],
                    &[
                        "[`Combobox<S>`](api:hypersig_ui/struct.Combobox.html)",
                        "`const fn new(field: NodeRef<S>, list: NodeRef<S>) -> Self`",
                        "two `node_ref!`, the text field's and the list's",
                    ],
                ],
                ja: &[
                    &[
                        "[`Dialog<S>`](api:hypersig_ui/struct.Dialog.html)",
                        "`const fn new(dialog: DialogRef<S>) -> Self`",
                        "`dialog_ref!` 1 つ",
                    ],
                    &[
                        "[`Menu<S>`](api:hypersig_ui/struct.Menu.html)",
                        "`const fn new(menu: NodeRef<S>) -> Self`",
                        "`node_ref!` 1 つ",
                    ],
                    &[
                        "[`Popover<S>`](api:hypersig_ui/struct.Popover.html)",
                        "`const fn new(panel: NodeRef<S>) -> Self`",
                        "`node_ref!` 1 つ",
                    ],
                    &[
                        "[`Tooltip<S>`](api:hypersig_ui/struct.Tooltip.html)",
                        "`const fn new(tip: NodeRef<S>) -> Self`",
                        "`node_ref!` 1 つ",
                    ],
                    &[
                        "[`Combobox<S>`](api:hypersig_ui/struct.Combobox.html)",
                        "`const fn new(field: NodeRef<S>, list: NodeRef<S>) -> Self`",
                        "`node_ref!` 2 つ（入力欄と一覧）",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A handle is `Copy`, and each of its methods takes it by value and builds one part: `trigger` the button, and `dialog`, `menu`, `popover`, `tooltip`, or `render` the element. The type parameter `S` is the page's signal struct, so a handle of one page cannot build parts on another, and each part is an `El<S>` the page places like any element. A menu's two parts render as:",
                ja: "ハンドルは `Copy` で、各メソッドはハンドルを値で受け取って部品を 1 つ作ります。ボタンを作るのは `trigger` で、要素を作るのは `dialog`、`menu`、`popover`、`tooltip`、`render` です。型パラメーター `S` はページのシグナル構造体なので、あるページのハンドルで別のページの部品は作れません。どの部品も `El<S>` なので、ページはほかの要素と同じように配置します。メニューの 2 つの部品は次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<button type=\"button\" aria-haspopup=\"menu\" commandfor=\"shop::DOC_ACTIONS\" command=\"toggle-popover\">Actions</button>\n\
             <div id=\"shop::DOC_ACTIONS\" popover=\"auto\" role=\"menu\">\n  <button type=\"button\" role=\"menuitem\">Archive</button>\n</div>",
        ),
        para(
            site,
            L {
                en: "The menu's `id` and the button's `commandfor` are both the node ref's name, which is the constant's path, `shop::DOC_ACTIONS` here, so the button opens this menu and no other. The `aria-labelledby` that names the menu after its button is left out above.",
                ja: "メニューの `id` とボタンの `commandfor` はどちらも node ref の名前です。名前は定数のパスで、ここでは `shop::DOC_ACTIONS` です。そのため、ボタンはほかのメニューではなく、このメニューを開きます。上の例ではメニューの名前をボタンから取る `aria-labelledby` を省いています。",
            },
        ),
    ])
}

fn why_const(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Why a handle is `const`",
                ja: "ハンドルが `const` である理由",
            },
        ),
        para(
            site,
            L {
                en: "`node_ref!` and `dialog_ref!` declare a constant at module level and fail to compile inside a function body, so every name is a path that only one constant has. The handle's `new` is a `const fn`, so the handle is a module-level constant too, and the function that builds the button and the function that builds the menu read the same value wherever they are. A `const` also lets the compiler check what `new` asserts: `Combobox::new` given one node ref for both its field and its list fails to compile with E0080:",
                ja: "`node_ref!` と `dialog_ref!` はモジュールのレベルで定数を宣言し、関数の本体の中ではコンパイルできません。そのため、どの名前も、ただ 1 つの定数だけが持つパスになります。ハンドルの `new` は `const fn` なので、ハンドルもモジュールのレベルの定数になり、ボタンを作る関数とメニューを作る関数は、どこにあっても同じ値を読みます。`const` にすると、`new` の中の検査もコンパイラが行います。`Combobox::new` に入力欄と一覧で同じ node ref を渡すと、E0080 でコンパイルできません。",
            },
        ),
        show(Snippet::UiComboboxSameRef),
        para(
            site,
            L {
                en: "The error reads `evaluation panicked: a combobox's field and list are two node refs`. Built with `let` inside a function instead, the same call compiles and panics when it runs.",
                ja: "エラーは `evaluation panicked: a combobox's field and list are two node refs` です。関数の中で `let` を使って作ると、同じ呼び出しでもコンパイルは通り、実行時に panic します。",
            },
        ),
    ])
}

fn twice(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A name used twice",
                ja: "同じ名前を 2 回使う",
            },
        ),
        para(
            site,
            L {
                en: "The compiler does not see how many times a page places a part. The element a handle opens carries its node ref, so one handle's menu, popover, tooltip, combobox, or dialog placed twice where the browser can reach both makes [`Page::render`](api:hypersig/struct.Page.html#method.render) panic:",
                ja: "ページが部品を何回置いたかは、コンパイラには分かりません。ハンドルが開く要素は node ref を持つので、1 つのハンドルのメニュー、ポップオーバー、ツールチップ、コンボボックス、ダイアログを、ブラウザが両方に届く場所へ 2 回置くと、[`Page::render`](api:hypersig/struct.Page.html#method.render) が panic します。",
            },
        ),
        show(Snippet::UiHandleTwice),
        bullets(
            site,
            &[
                L {
                    en: "A test that renders the page finds it before a visitor does.",
                    ja: "ページを描画するテストを書けば、訪問者より先に見つけられます。",
                },
                L {
                    en: "A dialog or a tooltip may have several buttons: two `trigger` buttons of one `Dialog` open the same dialog. A menu or a popover is named after its button, so its button carries an id too, and a second `trigger` of one `Menu` or `Popover` makes `Page::render` panic the same way.",
                    ja: "ダイアログとツールチップには複数のボタンを置けます。1 つの `Dialog` の `trigger` を 2 つ置けば、どちらも同じダイアログを開きます。メニューとポップオーバーは名前をボタンから取るので、ボタンも id を持ちます。そのため、1 つの `Menu` か `Popover` の `trigger` を 2 つ置くと、同じように `Page::render` が panic します。",
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
                en: "Example: all five on one page",
                ja: "例: 5 つを 1 ページに置く",
            },
        ),
        show(Snippet::UiHandles),
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
                en: "[Node refs](learn/node-refs/) covers `node_ref!` and `dialog_ref!`. [Dialog](components/dialog/), [Menu](components/menu/), [Popover](components/popover/), [Tooltip](components/tooltip/), and [Combobox](components/combobox/) each cover one handle.",
                ja: "`node_ref!` と `dialog_ref!` は [ノード参照](learn/node-refs/) で説明しています。各ハンドルについては[ダイアログ](components/dialog/)、[メニュー](components/menu/)、[ポップオーバー](components/popover/)、[ツールチップ](components/tooltip/)、[コンボボックス](components/combobox/)のページで説明しています。",
            },
        ),
    ])
}
