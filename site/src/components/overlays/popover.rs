//! The Components page on `Popover`: a panel of content that a button opens
//! beside itself.

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
        lead(site, Doc::Components(Component::Popover).leads()),
        demo::show(site, Demo::Popover),
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
                en: "The panel is a `popover=\"auto\"` with `role=\"dialog\"`, named by its button through `aria-labelledby`. The button has `aria-haspopup=\"dialog\"`, so a screen reader announces that it opens a dialog, and reads the panel as a dialog with the button's text as its name.",
                ja: "パネルは `role=\"dialog\"` を持つ `popover=\"auto\"` で、`aria-labelledby` によってボタンのテキストがパネルの名前になります。ボタンは `aria-haspopup=\"dialog\"` を持つので、スクリーンリーダーはボタンがダイアログを開くことを読み上げ、パネルをボタンのテキストを名前とするダイアログとして読み上げます。",
            },
        ),
        para(
            site,
            L {
                en: "The page stays usable while the panel is open, unlike a modal [Dialog](components/dialog/). Focus stays on the button as the panel opens, so the panel goes right after its button, where Tab moves from the button into it. The browser closes the panel on Escape or on a click outside, and returns focus to the button when focus was inside the panel.",
                ja: "モーダルの [ダイアログ](components/dialog/) と違い、パネルが開いている間もページは操作できます。パネルが開いてもフォーカスはボタンに残るので、パネルはボタンの直後に置きます。そうすると、ボタンから Tab でパネルの中に移れます。ブラウザは Escape かパネルの外のクリックでパネルを閉じ、フォーカスがパネルの中にあった場合はボタンに戻します。",
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
                    &["Enter, Space", "On the button, opens or closes the panel"],
                    &["Tab", "From the button, moves into the open panel"],
                    &[
                        "Escape",
                        "Closes the panel, and returns focus to the button when focus was inside it",
                    ],
                ],
                ja: &[
                    &["Enter、Space", "ボタンの上で押すと、パネルを開くか閉じます"],
                    &["Tab", "ボタンから、開いているパネルの中に移ります"],
                    &[
                        "Escape",
                        "パネルを閉じます。フォーカスがパネルの中にあった場合は、ボタンに戻します",
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
                en: "Building a popover",
                ja: "ポップオーバーを作る",
            },
        ),
        para(
            site,
            L {
                en: "A [`Popover<S>`](api:hypersig_ui/struct.Popover.html) is a `Copy` handle built from a `NodeRef<S>`, declared as a `const` so that the button and the panel share it, as [Handles](components/handles/) covers:",
                ja: "[`Popover<S>`](api:hypersig_ui/struct.Popover.html) は `NodeRef<S>` から作る `Copy` のハンドルです。ボタンとパネルで共有するように `const` として宣言します。宣言のしかたは [ハンドル](components/handles/) で説明しています。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Popover<S> {
    pub const fn new(panel: NodeRef<S>) -> Self;
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S>;
    pub fn popover(self) -> El<S>;
}",
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it gives"],
                ja: &["項目", "返すもの"],
            },
            L {
                en: &[
                    &[
                        "[`trigger`](api:hypersig_ui/struct.Popover.html#method.trigger)",
                        "The button that opens and closes the panel. Its text, added with `El::text`, names the panel",
                    ],
                    &[
                        "[`popover`](api:hypersig_ui/struct.Popover.html#method.popover)",
                        "The panel, empty, for the content added with `El::child`",
                    ],
                ],
                ja: &[
                    &[
                        "[`trigger`](api:hypersig_ui/struct.Popover.html#method.trigger)",
                        "パネルを開閉するボタン。`El::text` で加えたテキストがパネルの名前になります",
                    ],
                    &[
                        "[`popover`](api:hypersig_ui/struct.Popover.html#method.popover)",
                        "`El::child` で内容を加える、空のパネル",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The handle has no statement that closes the panel. A button inside it closes it with [`NodeRef::hide_popover`](api:hypersig/struct.NodeRef.html#method.hide_popover) on the node ref the handle is built from, and `show_popover` opens it the same way. [Node refs](learn/node-refs/) covers both.",
                ja: "ハンドルにはパネルを閉じる文がありません。パネルの中のボタンから閉じるには、ハンドルを作ったノード参照の [`NodeRef::hide_popover`](api:hypersig/struct.NodeRef.html#method.hide_popover) を使います。開くときは同じように `show_popover` を使います。どちらも [ノード参照](learn/node-refs/) で説明しています。",
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
                en: "The example at the end of this page renders this HTML, with the classes left out. Each `…` is an id made from the node ref's name, so the button opens the panel with `commandfor` and the panel takes its name from the button's id:",
                ja: "このページの最後の例は次の HTML を出力します（クラスは省いています）。`…` はどれもノード参照の名前から作った id です。ボタンは `commandfor` でパネルを開き、パネルはボタンの id から名前を得ます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<button id="…" type="button" aria-haspopup="dialog" commandfor="…" command="toggle-popover">Size</button>
<div id="…" popover="auto" role="dialog" aria-labelledby="…">
  <div>
    <p>Set the size of the layer.</p>
    <!-- the text field -->
    <button type="button">Done</button>
  </div>
</div>"#,
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
                en: "The panel is a `<div popover>`, so `Popover::new` takes a `NodeRef`. A `DialogRef` names a `<dialog>` and is E0308, mismatched types:",
                ja: "パネルは `<div popover>` なので、`Popover::new` は `NodeRef` を受け取ります。`DialogRef` は `<dialog>` を指す参照なので、渡すと E0308（型の不一致）になります。",
            },
        ),
        show(Snippet::UiPopoverDialogRef),
        para(
            site,
            L {
                en: "`hide_popover` returns a `Stmt<S>` of the node ref's page, so a node ref of another page is E0308 in the panel's `on_click`. `popover` takes no arguments, and its content goes through `El::child` like any element's.",
                ja: "`hide_popover` はノード参照のページの `Stmt<S>` を返すので、ほかのページのノード参照を使うと、パネルの `on_click` で E0308 になります。`popover` は引数を取らず、内容はほかの要素と同じく `El::child` で加えます。",
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
                en: "One handle's `popover` placed twice makes `Page::render` panic with `two elements carry the node ref … where the runtime can reach both`. A trigger with no panel on the page renders, and pressing it does nothing. Nothing checks that the panel follows its button.",
                ja: "1 つのハンドルの `popover` を 2 回置くと、`Page::render` が `two elements carry the node ref … where the runtime can reach both` というメッセージで panic します。ページにパネルがないトリガーは描画されますが、押しても何も起きません。パネルがボタンの直後にあるかどうかは検査されません。",
            },
        ),
        para(
            site,
            L {
                en: "Another `popover=\"auto\"`, such as a [Menu](components/menu/), opening outside the panel closes the panel, as the browser does for any two auto popovers that are not nested.",
                ja: "パネルの外で [メニュー](components/menu/) のような別の `popover=\"auto\"` が開くと、パネルは閉じます。入れ子になっていない 2 つの auto のポップオーバーに対して、ブラウザがそう動作するからです。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: setting a size",
                ja: "例: サイズを設定する",
            },
        ),
        para(
            site,
            L {
                en: "The panel holds a text field bound to a signal and a Done button that closes it:",
                ja: "パネルにはシグナルに束縛したテキスト欄と、パネルを閉じる Done のボタンがあります。",
            },
        ),
        show(Snippet::UiPopover),
        para(
            site,
            L {
                en: "A list of actions is a [Menu](components/menu/), and a description of one button is a [Tooltip](components/tooltip/). Content that needs the visitor's full attention is a [Dialog](components/dialog/).",
                ja: "操作の一覧には [メニュー](components/menu/) を、1 つのボタンの説明には [ツールチップ](components/tooltip/) を使います。訪問者に集中して見てほしい内容には [ダイアログ](components/dialog/) を使います。",
            },
        ),
    ])
}
