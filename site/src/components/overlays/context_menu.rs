//! The Components page on `ContextMenu`: a menu of actions that opens at the
//! pointer when the visitor right-clicks an element.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::ContextMenu).leads()),
        demo::show(site, Demo::ContextMenu),
        accessibility(site),
        building(site),
        compile_errors(site),
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
                en: "The menu has `role=\"menu\"`, named by the label it is built with, and each action `role=\"menuitem\"`, as in a [Menu](components/menu/). It is a `popover=\"auto\"`, so the browser shows it above the page and closes it on Escape or on a click outside. When it opens, focus moves to the first action that is not disabled, and when it closes, focus returns to the element that had it.",
                ja: "メニューは作るときに渡したラベルを名前とする `role=\"menu\"` を持ち、各操作は `role=\"menuitem\"` を持ちます。ここは [メニュー](components/menu/) と同じです。メニューは `popover=\"auto\"` なので、ブラウザがページの上に重ねて表示し、Escape か外側のクリックで閉じます。開くとフォーカスは無効でない最初の操作に移り、閉じるとフォーカスは開く前の要素に戻ります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key or pointer", "What it does"],
                ja: &["キーかポインタ", "動作"],
            },
            L {
                en: &[
                    &[
                        "Right click",
                        "Opens the menu at the pointer. Opening it while it is open moves it",
                    ],
                    &[
                        "Shift+F10, the menu key",
                        "Opens the menu at the bottom left corner of the focused element, in a browser that sends the event from a key, such as Chrome",
                    ],
                    &[
                        "Down, Up",
                        "Moves to the next or previous action, skipping disabled ones",
                    ],
                    &["Home, End", "Moves to the first or last action"],
                    &["Enter, Space", "Runs the action and closes the menu"],
                    &["Escape, Tab, Shift+Tab", "Closes the menu"],
                ],
                ja: &[
                    &[
                        "右クリック",
                        "ポインタの位置にメニューを開きます。開いている間にもう一度開くと、メニューが移ります",
                    ],
                    &[
                        "Shift+F10、メニューキー",
                        "キーからイベントを送るブラウザ（Chrome など）では、フォーカスのある要素の左下にメニューを開きます",
                    ],
                    &[
                        "下矢印、上矢印",
                        "次か前の操作に移ります。無効な操作は飛ばします",
                    ],
                    &["Home、End", "最初か最後の操作に移ります"],
                    &["Enter、Space", "操作を実行し、メニューを閉じます"],
                    &["Escape、Tab、Shift+Tab", "メニューを閉じます"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Safari and the web view of macOS send no context menu event from a key, so there a visitor without a pointer cannot open the menu. Give its actions another way in as well, such as a [Menu](components/menu/) button on each row.",
                ja: "Safari と macOS の Web ビューは、キーからはコンテキストメニューのイベントを送りません。そのため、そこではポインタを使わない訪問者はメニューを開けません。各行に [メニュー](components/menu/) のボタンを置くなど、同じ操作を別の方法でも使えるようにしてください。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a context menu",
                ja: "コンテキストメニューを作る",
            },
        ),
        para(
            site,
            L {
                en: "A [`ContextMenu<S>`](api:hypersig_ui/struct.ContextMenu.html) is a `Copy` handle built from a `NodeRef<S>`, declared as a `const`, as [Handles](components/handles/) covers. Its actions are the [`MenuItem<S>`](api:hypersig_ui/struct.MenuItem.html) values a [Menu](components/menu/) takes. An element opens it from [`El::on_context_menu`](api:hypersig/html/struct.El.html#method.on_context_menu), whose statement receives where the visitor asked, as an `Expr<S, Point>`:",
                ja: "[`ContextMenu<S>`](api:hypersig_ui/struct.ContextMenu.html) は `NodeRef<S>` から作る `Copy` のハンドルで、`const` として宣言します。宣言のしかたは [ハンドル](components/handles/) で説明しています。操作には [メニュー](components/menu/) と同じ [`MenuItem<S>`](api:hypersig_ui/struct.MenuItem.html) を使います。メニューを開くのは [`El::on_context_menu`](api:hypersig/html/struct.El.html#method.on_context_menu) で、その文は訪問者がメニューを求めた位置を `Expr<S, Point>` として受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> ContextMenu<S> {
    pub const fn new(menu: NodeRef<S>) -> Self;
    pub fn open_at(self, at: impl IntoExpr<S, Point>) -> Stmt<S>;
    pub fn menu(self, label: impl Into<String>, items: impl IntoIterator<Item = MenuItem<S>>) -> El<S>;
}

impl<S> El<S> {
    pub fn on_context_menu<B: IntoStmt<S>>(self, handle: impl FnOnce(Expr<S, Point>) -> B) -> Self;
}",
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it does"],
                ja: &["項目", "動作"],
            },
            L {
                en: &[
                    &[
                        "[`on_context_menu`](api:hypersig/html/struct.El.html#method.on_context_menu)",
                        "Runs the statement when the visitor asks for a context menu inside the element, and keeps the browser's own menu from showing. Only the innermost element with a handler runs it",
                    ],
                    &[
                        "[`open_at`](api:hypersig_ui/struct.ContextMenu.html#method.open_at)",
                        "A statement that opens the menu with its top left corner at `at`, or left of it or above it where the menu would cross the viewport's edge",
                    ],
                    &[
                        "[`menu`](api:hypersig_ui/struct.ContextMenu.html#method.menu)",
                        "The menu, named `label` for a screen reader, holding `items` in order. Place it once on the page",
                    ],
                ],
                ja: &[
                    &[
                        "[`on_context_menu`](api:hypersig/html/struct.El.html#method.on_context_menu)",
                        "訪問者が要素の中でコンテキストメニューを求めたときに文を実行し、ブラウザ自身のメニューを出さないようにします。実行するのは、ハンドラを持つ要素のうち最も内側のものだけです",
                    ],
                    &[
                        "[`open_at`](api:hypersig_ui/struct.ContextMenu.html#method.open_at)",
                        "メニューの左上を `at` に合わせて開く文。メニューがビューポートの端からはみ出すときは、`at` の左か上に開きます",
                    ],
                    &[
                        "[`menu`](api:hypersig_ui/struct.ContextMenu.html#method.menu)",
                        "`items` を順に並べたメニュー。`label` はスクリーンリーダーが読み上げる名前です。ページに 1 回だけ置きます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "One menu serves every row of a list. The row's statement first sets a signal that says which row it is, then opens the menu, and each action reads that signal. The position is a [`Point`](api:hypersig/html/struct.Point.html), so a `Sig<S, Point>` can also keep it.",
                ja: "1 つのメニューをリストのすべての行で使えます。行の文はどの行かを表すシグナルを先に設定してからメニューを開き、各操作はそのシグナルを読みます。位置は [`Point`](api:hypersig/html/struct.Point.html) なので、`Sig<S, Point>` に入れておくこともできます。",
            },
        ),
    ])
}

fn compile_errors(site: Site) -> Part {
    parts([
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
                en: "`open_at` takes an `Expr<S, Point>` or something that converts to one, so a signal of another type is E0308:",
                ja: "`open_at` は `Expr<S, Point>` か、それに変換できる値を受け取るので、ほかの型のシグナルを渡すと E0308 になります。",
            },
        ),
        show(Snippet::UiContextMenuPoint),
        para(
            site,
            L {
                en: "The actions take `impl IntoStmt<S>`, so a statement of another page is E0308, and `ContextMenu::new` takes a `NodeRef`, so a `DialogRef` is E0308.",
                ja: "操作は `impl IntoStmt<S>` を受け取るので、ほかのページの文を渡すと E0308 になります。`ContextMenu::new` は `NodeRef` を受け取るので、`DialogRef` を渡すと E0308 になります。",
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
                en: "One handle's `menu` placed twice on a page makes `Page::render` panic with `two elements carry the node ref … where the runtime can reach both`. An `open_at` with no menu on the page writes a warning to the browser's console, and the browser shows no menu at all, because `on_context_menu` keeps its own from showing.",
                ja: "1 つのハンドルの `menu` をページに 2 回置くと、`Page::render` が `two elements carry the node ref … where the runtime can reach both` というメッセージで panic します。ページにメニューがないまま `open_at` を実行すると、ブラウザのコンソールに警告が出ます。`on_context_menu` がブラウザ自身のメニューを止めるので、メニューは何も表示されません。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: message actions",
                ja: "例: メッセージの操作",
            },
        ),
        para(
            site,
            L {
                en: "Each message sets `picked` to its id and opens the one menu, whose actions set a status:",
                ja: "各メッセージは `picked` に自分の id を設定してから、1 つだけのメニューを開きます。メニューの操作は状態のテキストを設定します。",
            },
        ),
        show(Snippet::UiContextMenu),
    ])
}
