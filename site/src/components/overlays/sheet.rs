//! The Components page on `Dialog::sheet`: a modal dialog along one edge of
//! the viewport.

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
        lead(site, Doc::Components(Component::Sheet).leads()),
        demo::show(site, Demo::Sheet),
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
                en: "A sheet is the same modal `<dialog>` as a [Dialog](components/dialog/), drawn along one edge of the viewport. A screen reader announces it as a dialog named by its title, the rest of the page is inert while it is open, and focus returns to the element that opened it when it closes. Only its place and size differ from a dialog's.",
                ja: "シートは [ダイアログ](components/dialog/) と同じモーダルの `<dialog>` を、ビューポートの端に沿って描いたものです。スクリーンリーダーはタイトルを名前とするダイアログとして読み上げ、開いている間はページのほかの部分が inert になり、閉じるとフォーカスはシートを開いた要素に戻ります。ダイアログと違うのは位置と大きさだけです。",
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
                        "Tab, Shift+Tab",
                        "Moves between the sheet's controls and never reaches the page behind",
                    ],
                    &[
                        "Escape",
                        "Closes the sheet and returns focus to the element that opened it",
                    ],
                ],
                ja: &[
                    &[
                        "Tab、Shift+Tab",
                        "シートのコントロールの間を移り、後ろのページには移りません",
                    ],
                    &[
                        "Escape",
                        "シートを閉じ、シートを開いた要素にフォーカスを戻します",
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
                en: "Building a sheet",
                ja: "シートを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`Dialog::sheet`](api:hypersig_ui/struct.Dialog.html#method.sheet) takes the arguments of `Dialog::dialog` after a [`Side`](api:hypersig_ui/enum.Side.html), the edge the sheet sits along. `trigger`, `close`, `open`, and `dismiss` work on it as on a [Dialog](components/dialog/):",
                ja: "[`Dialog::sheet`](api:hypersig_ui/struct.Dialog.html#method.sheet) は、シートが沿う端を表す [`Side`](api:hypersig_ui/enum.Side.html) を先に受け取り、そのあとに `Dialog::dialog` と同じ引数を受け取ります。`trigger`、`close`、`open`、`dismiss` は [ダイアログ](components/dialog/) と同じように使えます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Dialog<S> {
    pub fn sheet(
        self,
        side: Side,
        title: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<Node<S>>,
        text: &DialogText,
    ) -> El<S>;
}

#[non_exhaustive]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}",
        ),
        table_of(
            site,
            L {
                en: &["`Side`", "Where the sheet sits"],
                ja: &["`Side`", "シートの位置"],
            },
            L {
                en: &[
                    &[
                        "`Side::Top`",
                        "Along the top edge, the full width of the viewport",
                    ],
                    &[
                        "`Side::Right`",
                        "Along the right edge, the full height of the viewport and at most `24rem` wide",
                    ],
                    &[
                        "`Side::Bottom`",
                        "Along the bottom edge, the full width of the viewport",
                    ],
                    &[
                        "`Side::Left`",
                        "Along the left edge, the full height of the viewport and at most `24rem` wide",
                    ],
                ],
                ja: &[
                    &[
                        "`Side::Top`",
                        "上端に沿い、ビューポートの幅いっぱいに広がります",
                    ],
                    &[
                        "`Side::Right`",
                        "右端に沿い、ビューポートの高さいっぱいに広がります。幅は最大 `24rem` です",
                    ],
                    &[
                        "`Side::Bottom`",
                        "下端に沿い、ビューポートの幅いっぱいに広がります",
                    ],
                    &[
                        "`Side::Left`",
                        "左端に沿い、ビューポートの高さいっぱいに広がります。幅は最大 `24rem` です",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Content taller than the sheet scrolls inside it. `Side` is `#[non_exhaustive]`, so a `match` on it outside the crate needs a `_` arm.",
                ja: "シートより高い中身はシートの中でスクロールします。`Side` は `#[non_exhaustive]` なので、クレートの外で `match` するときは `_` の腕が必要です。",
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
                en: "The HTML is a dialog's, and every `Side` renders the same elements and attributes. Only the classes, left out here, place it. Each `…` is an id made from the dialog ref's name, and the corner button is named by the `DialogText` the example passes:",
                ja: "HTML はダイアログと同じで、どの `Side` でも同じ要素と属性が出力されます。位置を決めるのはクラスだけです（ここでは省いています）。`…` はどれもダイアログ参照の名前から作った id で、角のボタンの名前は例で渡した `DialogText` から付きます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<dialog id="…" aria-labelledby="…" aria-describedby="…" closedby="any">
  <div>
    <div>
      <h2 id="…">Cart</h2>
      <p id="…">Two items, ready to check out.</p>
    </div>
    <div>
      <ul><li>Green tea</li><li>Two cups</li></ul>
      <div><button type="button" commandfor="…" command="close">Done</button></div>
    </div>
  </div>
  <button type="button" aria-label="Close the cart" commandfor="…" command="close">
    <svg aria-hidden="true" …></svg>
  </button>
</dialog>"#,
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
                en: "The edge is a `Side`, so a string such as `\"right\"` is E0308, mismatched types, where a string would reach the browser unchecked:",
                ja: "端は `Side` で指定するので、`\"right\"` のような文字列を渡すと E0308（型の不一致）になります。文字列を受け取ると、検査されない値がそのままブラウザに届いてしまいます。",
            },
        ),
        show(Snippet::UiSheetSide),
        para(
            site,
            L {
                en: "Leaving out the `DialogText` is E0061, and a `NodeRef` in place of the `DialogRef` is E0308, as for a [Dialog](components/dialog/).",
                ja: "`DialogText` を省くと E0061 に、`DialogRef` の代わりに `NodeRef` を渡すと E0308 になります。[ダイアログ](components/dialog/) と同じです。",
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
                en: "As with a dialog, one handle's `sheet` placed twice makes `Page::render` panic, and a handle used for both a `sheet` and a `dialog` on one page panics the same way. Nothing checks the side against the screen: a wide table in a `Side::Left` sheet scrolls sideways inside the sheet, which is at most `24rem` wide.",
                ja: "ダイアログと同じく、1 つのハンドルの `sheet` を 2 回置くと `Page::render` が panic します。1 つのハンドルを 1 つのページで `sheet` と `dialog` の両方に使った場合も、同じように panic します。画面に合う端かどうかは検査されません。`Side::Left` のシートの幅は最大 `24rem` なので、幅の広い表を入れるとシートの中で横にスクロールします。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a cart along the right edge",
                ja: "例: 右端に出るカート",
            },
        ),
        show(Snippet::UiSheet),
        para(
            site,
            L {
                en: "A sheet holds what a [Dialog](components/dialog/) would, when it is long or belongs beside the page, such as filters, details, or a cart. For a short choice next to a button, a [Popover](components/popover/) keeps the page usable.",
                ja: "シートには [ダイアログ](components/dialog/) に入れる内容のうち、長いものやページの横に置きたいもの（絞り込み、詳細、カートなど）を入れます。ボタンの横で短く選ばせるなら、ページを操作できるままの [ポップオーバー](components/popover/) を使います。",
            },
        ),
    ])
}
