//! The Components page on `Menu`: a button that opens a menu of actions in a
//! popover.

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
        lead(site, Doc::Components(Component::Menu).leads()),
        demo::show(site, Demo::Menu),
        accessibility(site),
        building(site),
        html(site),
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
                en: "It follows the WAI-ARIA menu button pattern. The button has `aria-haspopup=\"menu\"`, the menu `role=\"menu\"` named by the button, and each action `role=\"menuitem\"`, so a screen reader announces the button as opening a menu and reads each action as a menu item with its position. A disabled action keeps its place and is announced as unavailable.",
                ja: "WAI-ARIA の menu button パターンに従います。ボタンは `aria-haspopup=\"menu\"` を、メニューはボタンを名前とする `role=\"menu\"` を、各操作は `role=\"menuitem\"` を持ちます。そのため、スクリーンリーダーはボタンをメニューを開くボタンとして読み上げ、各操作をメニューの項目として位置と一緒に読み上げます。無効な操作はその位置に残り、使えないことが読み上げられます。",
            },
        ),
        para(
            site,
            L {
                en: "The menu is a `popover=\"auto\"`, so the browser shows it above the page and closes it on Escape or on a click outside. It is placed under the button with CSS anchor positioning and moves above the button where there is no room below. When it opens, focus moves to the first action that is not disabled.",
                ja: "メニューは `popover=\"auto\"` なので、ブラウザがページの上に重ねて表示し、Escape か外側のクリックで閉じます。メニューは CSS の anchor positioning でボタンの下に置かれ、下に場所がなければボタンの上に移ります。メニューが開くと、フォーカスは無効でない最初の操作に移ります。",
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
                        "On the button, opens the menu and moves focus to the first action. On an action, runs it and closes the menu",
                    ],
                    &[
                        "Down, Up",
                        "Moves to the next or previous action, skipping disabled ones",
                    ],
                    &["Home, End", "Moves to the first or last action"],
                    &["Escape", "Closes the menu and returns focus to the button"],
                    &[
                        "Tab, Shift+Tab",
                        "Closes the menu and returns focus to the button",
                    ],
                ],
                ja: &[
                    &[
                        "Enter、Space",
                        "ボタンの上では、メニューを開いて最初の操作にフォーカスを移します。操作の上では、その操作を実行してメニューを閉じます",
                    ],
                    &[
                        "下矢印、上矢印",
                        "次か前の操作に移ります。無効な操作は飛ばします",
                    ],
                    &["Home、End", "最初か最後の操作に移ります"],
                    &["Escape", "メニューを閉じ、ボタンにフォーカスを戻します"],
                    &[
                        "Tab、Shift+Tab",
                        "メニューを閉じ、ボタンにフォーカスを戻します",
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
                en: "Building a menu",
                ja: "メニューを作る",
            },
        ),
        para(
            site,
            L {
                en: "A [`Menu<S>`](api:hypersig_ui/struct.Menu.html) is a `Copy` handle built from a `NodeRef<S>`, declared as a `const` so that the button and the menu share it, as [Handles](components/handles/) covers. Each action is a [`MenuItem<S>`](api:hypersig_ui/struct.MenuItem.html), which holds a label and the statement it runs:",
                ja: "[`Menu<S>`](api:hypersig_ui/struct.Menu.html) は `NodeRef<S>` から作る `Copy` のハンドルです。ボタンとメニューで共有するように `const` として宣言します。宣言のしかたは [ハンドル](components/handles/) で説明しています。各操作は [`MenuItem<S>`](api:hypersig_ui/struct.MenuItem.html) で、ラベルと実行する文を持ちます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Menu<S> {
    pub const fn new(menu: NodeRef<S>) -> Self;
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S>;
    pub fn menu(self, items: impl IntoIterator<Item = MenuItem<S>>) -> El<S>;
}

impl<S> MenuItem<S> {
    pub fn new(label: impl Into<String>, run: impl IntoStmt<S>) -> Self;
    pub fn destructive(self) -> Self;
    pub fn disabled(self, when: impl IntoExpr<S, bool>) -> Self;
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
                        "[`trigger`](api:hypersig_ui/struct.Menu.html#method.trigger)",
                        "The button that opens and closes the menu. Its text is added with `El::text`",
                    ],
                    &[
                        "[`menu`](api:hypersig_ui/struct.Menu.html#method.menu)",
                        "The menu, holding `items` in order. Place it right after the button",
                    ],
                    &[
                        "[`MenuItem::new`](api:hypersig_ui/struct.MenuItem.html#method.new)",
                        "An action labeled `label`. Choosing it runs `run`, then closes the menu",
                    ],
                    &[
                        "[`destructive`](api:hypersig_ui/struct.MenuItem.html#method.destructive)",
                        "Draws the action in `token::destructive`, for an action that deletes",
                    ],
                    &[
                        "[`disabled`](api:hypersig_ui/struct.MenuItem.html#method.disabled)",
                        "Disables the action while `when` is true. The browser updates it as the signals change",
                    ],
                ],
                ja: &[
                    &[
                        "[`trigger`](api:hypersig_ui/struct.Menu.html#method.trigger)",
                        "メニューを開閉するボタン。テキストは `El::text` で加えます",
                    ],
                    &[
                        "[`menu`](api:hypersig_ui/struct.Menu.html#method.menu)",
                        "`items` を順に並べたメニュー。ボタンの直後に置きます",
                    ],
                    &[
                        "[`MenuItem::new`](api:hypersig_ui/struct.MenuItem.html#method.new)",
                        "`label` をラベルに持つ操作。選ぶと `run` を実行し、メニューを閉じます",
                    ],
                    &[
                        "[`destructive`](api:hypersig_ui/struct.MenuItem.html#method.destructive)",
                        "操作を `token::destructive` の色で描きます。削除する操作に使います",
                    ],
                    &[
                        "[`disabled`](api:hypersig_ui/struct.MenuItem.html#method.disabled)",
                        "`when` が true の間、操作を無効にします。シグナルが変わると、ブラウザが状態を更新します",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`run` is any statement, so an action can set a signal, call an action on the server, or open a [Dialog](components/dialog/) with `Dialog::open`.",
                ja: "`run` には任意の文を渡せます。シグナルを設定することも、サーバーのアクションを呼ぶことも、`Dialog::open` で [ダイアログ](components/dialog/) を開くこともできます。",
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
                en: "The example at the end of this page renders this HTML for the button and the menu, with the classes left out. Each `…` is an id made from the node ref's name. The Archive action is disabled because `archived` starts as `true`:",
                ja: "このページの最後の例はボタンとメニューについて次の HTML を出力します（クラスは省いています）。`…` はどれもノード参照の名前から作った id です。`archived` が `true` で始まるので、Archive の操作は無効になっています。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<button id="…" type="button" aria-haspopup="menu" commandfor="…" command="toggle-popover">Actions</button>
<div id="…" popover="auto" role="menu" aria-labelledby="…">
  <button type="button" role="menuitem" tabindex="-1" autofocus="">Duplicate</button>
  <button type="button" role="menuitem" tabindex="-1" autofocus="" disabled="">Archive</button>
  <button type="button" role="menuitem" tabindex="-1" autofocus="">Delete</button>
</div>"#,
        ),
        para(
            site,
            L {
                en: "Every action has `tabindex=\"-1\"`, so Tab does not stop on each one, and the arrow keys move between them instead. Every action also has `autofocus`, and as the popover opens the browser focuses the first of them that is not disabled.",
                ja: "どの操作も `tabindex=\"-1\"` を持つので、Tab は操作ごとには止まらず、操作の間は矢印キーで移ります。どの操作も `autofocus` も持っており、ポップオーバーが開くとき、ブラウザはそのうち無効でない最初の操作にフォーカスします。",
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
                en: "`disabled` takes an `Expr<S, bool>` or something that converts to one, so a signal of another type is E0277, with a message that names the type it found:",
                ja: "`disabled` は `Expr<S, bool>` か、それに変換できる値を受け取るので、ほかの型のシグナルを渡すと E0277 になります。エラーメッセージは渡した型を示します。",
            },
        ),
        show(Snippet::UiMenuDisabled),
        para(
            site,
            L {
                en: "`run` takes `impl IntoStmt<S>`, so a statement of another page is E0308. `Menu::new` takes a `NodeRef`, so a `DialogRef` is E0308. `destructive` takes nothing, and its types stop no mistake.",
                ja: "`run` は `impl IntoStmt<S>` を受け取るので、ほかのページの文を渡すと E0308 になります。`Menu::new` は `NodeRef` を受け取るので、`DialogRef` を渡すと E0308 になります。`destructive` は引数を取らず、型が防ぐ誤りはありません。",
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
                en: "One handle's `menu` placed twice on a page makes `Page::render` panic with `two elements carry the node ref … where the runtime can reach both`. A trigger with no menu on the page renders, and pressing it does nothing. A menu with no items opens as an empty box, and focus stays on the button.",
                ja: "1 つのハンドルの `menu` をページに 2 回置くと、`Page::render` が `two elements carry the node ref … where the runtime can reach both` というメッセージで panic します。ページにメニューがないトリガーは描画されますが、押しても何も起きません。項目のないメニューは空の枠として開き、フォーカスはボタンに残ります。",
            },
        ),
        para(
            site,
            L {
                en: "The menu sits in the top layer, and CSS anchor positioning places it next to the button. In a browser without anchor positioning, the menu still opens and answers to the keys above, but it is not placed next to the button.",
                ja: "メニューは top layer に置かれ、CSS の anchor positioning でボタンの隣に配置されます。anchor positioning に対応していないブラウザでも、メニューは開き、上のキーで操作できますが、ボタンの隣には配置されません。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: project actions",
                ja: "例: プロジェクトの操作",
            },
        ),
        para(
            site,
            L {
                en: "Each action sets a status, Archive is disabled once the project is archived, and Delete is drawn as destructive:",
                ja: "各操作は状態のテキストを設定します。Archive はプロジェクトをアーカイブすると無効になり、Delete は destructive の色で描かれます。",
            },
        ),
        show(Snippet::UiMenu),
        para(
            site,
            L {
                en: "An action that deletes is often followed by an [Alert dialog](components/alert-dialog/), which the menu item opens with `Dialog::open`. A panel of controls rather than a list of actions is a [Popover](components/popover/).",
                ja: "削除する操作のあとには [アラートダイアログ](components/alert-dialog/) を出すことがよくあります。メニューの項目から `Dialog::open` で開けます。コントロールを並べたパネルには [ポップオーバー](components/popover/) を使います。",
            },
        ),
    ])
}
