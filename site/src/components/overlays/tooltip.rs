//! The Components page on `Tooltip`: a short description shown while a
//! button is hovered or focused.

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
        lead(site, Doc::Components(Component::Tooltip).leads()),
        demo::show(site, Demo::Tooltip),
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
                en: "It follows the WAI-ARIA tooltip pattern. The tooltip has `role=\"tooltip\"`, and the button has `aria-describedby` pointing at it, so a screen reader reads the tooltip's text after the button's name, whether or not the tooltip is shown. The tooltip describes the button and does not name it: an icon button gets its name from its own label.",
                ja: "WAI-ARIA の tooltip パターンに従います。ツールチップは `role=\"tooltip\"` を持ち、ボタンの `aria-describedby` がツールチップを指すので、スクリーンリーダーはツールチップが表示されているかどうかに関係なく、ボタンの名前のあとにツールチップのテキストを読み上げます。ツールチップはボタンを説明するもので、ボタンの名前にはなりません。アイコンボタンの名前はボタン自身のラベルから付きます。",
            },
        ),
        para(
            site,
            L {
                en: "The tooltip opens while the pointer is on the button or on the tooltip, or while the button has focus, and closes when both are left or a key is pressed on the button. It is a `popover=\"hint\"`, which browser-compat-data 8.1.6 lists for Chrome 151 and Firefox 153, with an older partial version in Chrome 133 to 150 and Firefox 149 to 152, and for Safari only in its preview. A browser without it treats the value as `manual`, and there Escape closes the tooltip only while focus is on its button.",
                ja: "ツールチップは、ポインタがボタンかツールチップの上にある間と、ボタンにフォーカスがある間に開き、両方から離れたときか、ボタンの上でキーが押されたときに閉じます。ツールチップは `popover=\"hint\"` です。browser-compat-data 8.1.6 では Chrome 151 と Firefox 153 が対応しており、Chrome 133〜150 と Firefox 149〜152 には部分的な古い実装があり、Safari はプレビュー版だけが対応しています。対応していないブラウザはこの値を `manual` として扱うので、Escape でツールチップが閉じるのは、フォーカスがそのボタンにある間だけです。",
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
                        "Tab",
                        "Focusing the button opens the tooltip, and leaving it closes the tooltip",
                    ],
                    &[
                        "Any key",
                        "With focus on the button, closes the tooltip. Escape also reaches a dialog around the button and closes it",
                    ],
                ],
                ja: &[
                    &[
                        "Tab",
                        "ボタンにフォーカスが移るとツールチップが開き、フォーカスが離れると閉じます",
                    ],
                    &[
                        "任意のキー",
                        "ボタンにフォーカスがあるときに押すと、ツールチップが閉じます。Escape はボタンを囲むダイアログにも届き、ダイアログも閉じます",
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
                en: "Building a tooltip",
                ja: "ツールチップを作る",
            },
        ),
        para(
            site,
            L {
                en: "A [`Tooltip<S>`](api:hypersig_ui/struct.Tooltip.html) is a `Copy` handle built from a `NodeRef<S>`, declared as a `const` so that the button and the tooltip share it, as [Handles](components/handles/) covers. The handle builds the button itself, because the button carries the attributes and handlers the tooltip needs:",
                ja: "[`Tooltip<S>`](api:hypersig_ui/struct.Tooltip.html) は `NodeRef<S>` から作る `Copy` のハンドルです。ボタンとツールチップで共有するように `const` として宣言します。宣言のしかたは [ハンドル](components/handles/) で説明しています。ツールチップに必要な属性とハンドラはボタンに付くので、ボタンはハンドルが作ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Tooltip<S> {
    pub const fn new(tip: NodeRef<S>) -> Self;
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S>;
    pub fn icon_trigger(self, variant: ButtonVariant, label: impl Into<String>) -> El<S>;
    pub fn tooltip(self, text: impl Into<String>) -> El<S>;
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
                        "[`trigger`](api:hypersig_ui/struct.Tooltip.html#method.trigger)",
                        "A button with text, described by the tooltip. Its text is added with `El::text`",
                    ],
                    &[
                        "[`icon_trigger`](api:hypersig_ui/struct.Tooltip.html#method.icon_trigger)",
                        "An icon button named `label` through `aria-label`, described by the tooltip. Its icon is added with `El::child` or `El::text`",
                    ],
                    &[
                        "[`tooltip`](api:hypersig_ui/struct.Tooltip.html#method.tooltip)",
                        "The tooltip holding `text`. Place it right after its button",
                    ],
                ],
                ja: &[
                    &[
                        "[`trigger`](api:hypersig_ui/struct.Tooltip.html#method.trigger)",
                        "ツールチップが説明する、テキストのボタン。テキストは `El::text` で加えます",
                    ],
                    &[
                        "[`icon_trigger`](api:hypersig_ui/struct.Tooltip.html#method.icon_trigger)",
                        "`aria-label` で `label` を名前に持ち、ツールチップが説明するアイコンボタン。アイコンは `El::child` か `El::text` で加えます",
                    ],
                    &[
                        "[`tooltip`](api:hypersig_ui/struct.Tooltip.html#method.tooltip)",
                        "`text` を入れたツールチップ。ボタンの直後に置きます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The returned button is an `El<S>`, so it takes `on_click` and the other handlers of any button. Its own focus, blur, pointer, and key handlers stay in place alongside them.",
                ja: "返されるボタンは `El<S>` なので、ほかのボタンと同じく `on_click` などのハンドラを受け取ります。ツールチップのためのフォーカス、ブラー、ポインタ、キーのハンドラは、それらと並んで残ります。",
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
                en: "The example at the end of this page renders this HTML for the Bold button and its tooltip, with the classes left out. `…` is the tooltip's id, made from the node ref's name:",
                ja: "このページの最後の例はBold のボタンとそのツールチップについて次の HTML を出力します（クラスは省いています）。`…` はノード参照の名前から作ったツールチップの id です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<button type="button" aria-label="Bold" aria-describedby="…">B</button>
<div id="…" popover="hint" role="tooltip">Ctrl+B</div>"#,
        ),
        para(
            site,
            L {
                en: "A screen reader reads the button as \"Bold, button, Ctrl+B\". The text \"B\" inside the button is not read, because `aria-label` names the button.",
                ja: "スクリーンリーダーはこのボタンを「Bold、ボタン、Ctrl+B」のように読み上げます。`aria-label` がボタンの名前になるので、ボタンの中のテキスト「B」は読まれません。",
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
                en: "An icon button has no text for a screen reader to read, so `icon_trigger` takes its name as an argument. Leaving it out is E0061, wrong number of arguments:",
                ja: "アイコンボタンにはスクリーンリーダーが読むテキストがないので、`icon_trigger` は名前を引数に取ります。省くと E0061（引数の数の誤り）になります。",
            },
        ),
        show(Snippet::UiTooltipLabel),
        para(
            site,
            L {
                en: "A tooltip is read as plain text through `aria-describedby`, and the WAI-ARIA pattern does not let it hold controls or structure. `tooltip` takes `impl Into<String>`, so an element is E0277:",
                ja: "ツールチップは `aria-describedby` を通して文字列として読まれ、WAI-ARIA のパターンではコントロールや構造を持てません。`tooltip` は `impl Into<String>` を受け取るので、要素を渡すと E0277 になります。",
            },
        ),
        show(Snippet::UiTooltipRich),
        para(
            site,
            L {
                en: "`Tooltip` has no method that attaches a tooltip to a button built elsewhere, so a button without the handlers the tooltip needs cannot be described by it. A `DialogRef` in place of the `NodeRef` is E0308.",
                ja: "`Tooltip` には、ほかで作ったボタンにツールチップを付けるメソッドがありません。そのため、ツールチップに必要なハンドラのないボタンを説明することはできません。`NodeRef` の代わりに `DialogRef` を渡すと E0308 になります。",
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
                en: "One handle's `tooltip` placed twice makes `Page::render` panic with `two elements carry the node ref … where the runtime can reach both`. A button from `trigger` with no tooltip on the page still renders, and its `aria-describedby` points at nothing, so a screen reader reads only its name. Two buttons from one handle both point at the one tooltip.",
                ja: "1 つのハンドルの `tooltip` を 2 回置くと、`Page::render` が `two elements carry the node ref … where the runtime can reach both` というメッセージで panic します。ページにツールチップがない `trigger` のボタンも描画されますが、`aria-describedby` が指す要素がないので、スクリーンリーダーはボタンの名前だけを読みます。1 つのハンドルから作った 2 つのボタンは、どちらも同じツールチップを指します。",
            },
        ),
        para(
            site,
            L {
                en: "The length of the text is not checked. A tooltip wraps at `20rem` and closes when the pointer leaves, so text the visitor needs to act on belongs in the page or in a [Popover](components/popover/).",
                ja: "テキストの長さは検査されません。ツールチップは `20rem` で折り返し、ポインタが離れると閉じるので、訪問者が操作に必要とするテキストは、ページの中か [ポップオーバー](components/popover/) に書きます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a toolbar",
                ja: "例: ツールバー",
            },
        ),
        para(
            site,
            L {
                en: "The Bold icon button shows its shortcut, and the Share button says what it does:",
                ja: "Bold のアイコンボタンはショートカットを表示し、Share のボタンは何をするかを表示します。",
            },
        ),
        show(Snippet::UiTooltip),
        para(
            site,
            L {
                en: "A key in running text is drawn with [Kbd](components/kbd/). An icon button with no tooltip is on the [Icon button](components/icon-button/) page.",
                ja: "文中のキーは [Kbd](components/kbd/) で描きます。ツールチップのないアイコンボタンは [アイコンボタン](components/icon-button/) のページで説明しています。",
            },
        ),
    ])
}
