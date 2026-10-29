//! The page on `MultiToggleGroup`: a row of toggles, each bound to a `bool`
//! signal of its own.

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
        lead(site, Doc::Components(Component::MultiToggleGroup).leads()),
        demo::show(site, Demo::Toggle),
        accessibility(site),
        builder(site),
        unchecked(site),
        example(site),
        next(site),
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
                en: "The group is a `<div>` with `role=\"group\"` and the name you give it, and each toggle is a [toggle](components/toggle/) with `aria-pressed`. A screen reader announces the group's name as focus enters it, then each toggle as pressed or not pressed. The group is one Tab stop, and the arrow keys move between toggles without flipping them, as in the Text style group of the demo above.",
                ja: "グループは `role=\"group\"` と指定した名前を持つ `<div>` で、各トグルは `aria-pressed` を持つ[トグル](components/toggle/)です。スクリーンリーダーは、フォーカスがグループに入るとその名前を読み上げ、続いて各トグルが押されているかどうかを読み上げます。グループの Tab の停止位置は 1 つで、矢印キーはトグルを反転せずにトグル間を移動します。上のデモの「文字のスタイル」のグループがこれです。",
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
                    &["Tab", "Moves focus into the group, to its first toggle."],
                    &[
                        "Left, Right",
                        "Moves to the previous or the next toggle, wrapping at either end, without flipping it.",
                    ],
                    &["Home, End", "Moves to the first or the last toggle."],
                    &["Enter, Space", "Flips the toggle that has focus."],
                ],
                ja: &[
                    &["Tab", "グループの最初のトグルにフォーカスを移します。"],
                    &[
                        "←、→",
                        "前または次のトグルに移り、端では反対の端に戻ります。トグルは反転しません。",
                    ],
                    &["Home、End", "最初または最後のトグルに移ります。"],
                    &["Enter、Space", "フォーカスのあるトグルを反転します。"],
                ],
            },
        ),
    ])
}

fn builder(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`MultiToggleGroup`",
                ja: "`MultiToggleGroup`",
            },
        ),
        para(
            site,
            L {
                en: "[`MultiToggleGroup`](api:hypersig_ui/struct.MultiToggleGroup.html) is a builder like `ToggleGroup`, but its `new` takes only the name, and each `item` or `icon_item` takes the `bool` signal its toggle flips:",
                ja: "[`MultiToggleGroup`](api:hypersig_ui/struct.MultiToggleGroup.html) は `ToggleGroup` と同じくビルダーですが、`new` は名前だけを受け取り、`item` と `icon_item` がそれぞれのトグルが反転する `bool` のシグナルを受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> MultiToggleGroup<S> {\n    \
             pub fn new(label: impl Into<String>) -> Self\n    \
             pub fn item<R: Writable>(self, pressed: Sig<S, bool, R>, text: impl Into<String>) -> Self\n    \
             pub fn icon_item<R: Writable>(self, pressed: Sig<S, bool, R>, label: impl Into<String>, icon: impl Into<Node<S>>) -> Self\n    \
             pub fn render(self) -> El<S>\n}",
        ),
        para(
            site,
            L {
                en: "Only the first toggle stays in the tab order, and the arrow keys reach the rest. With `bold` true and `italic` false, `MultiToggleGroup::new(\"Style\").item(Editor::bold(), \"Bold\").item(Editor::italic(), \"Italic\")` renders as follows, without the `class`:",
                ja: "Tab キーの移動先に残るのは最初のトグルだけで、残りのトグルには矢印キーで移ります。`bold` が true、`italic` が false のとき、`MultiToggleGroup::new(\"Style\").item(Editor::bold(), \"Bold\").item(Editor::italic(), \"Italic\")` は、`class` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div role=\"group\" aria-label=\"Style\">\n  \
             <button type=\"button\" tabindex=\"0\" aria-pressed=\"true\">Bold</button>\n  \
             <button type=\"button\" tabindex=\"-1\" aria-pressed=\"false\">Italic</button>\n\
             </div>",
        ),
        para(
            site,
            L {
                en: "Each toggle is pressed or not, so `item` takes a `Sig<S, bool, R>`, and a signal of another type fails to compile with E0308. A computed `bool` signal fails with E0277, as on [Toggle](components/toggle/):",
                ja: "各トグルは押されているかいないかのどちらかなので、`item` は `Sig<S, bool, R>` を受け取り、ほかの型のシグナルは E0308 でコンパイルできません。算出された `bool` のシグナルは[トグル](components/toggle/)と同じく E0277 になります。",
            },
        ),
        show(Snippet::UiMultiWrongSignal),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
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
                    en: "Tab always enters at the first toggle, wherever focus last was in the group.",
                    ja: "グループ内で最後にどこにフォーカスがあったかにかかわらず、Tab キーは常に最初のトグルに入ります。",
                },
                L {
                    en: "One signal given to two items gives two toggles that flip together, and a screen reader announces both.",
                    ja: "1 つのシグナルを 2 つの項目に渡すと、2 つのトグルが一緒に反転し、スクリーンリーダーは両方を読み上げます。",
                },
                L {
                    en: "An empty name compiles, for the group and for an `icon_item`, and leaves it without a name.",
                    ja: "グループや `icon_item` の名前を空にしてもコンパイルでき、名前のないままになります。",
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
                en: "Example: bold, italic, and underline",
                ja: "例: 太字、斜体、下線",
            },
        ),
        show(Snippet::UiMultiToggleGroup),
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
                en: "[Toggle group](components/toggle-group/) keeps exactly one toggle on. [Checkbox](components/checkbox/) is the control for a set of choices inside a form.",
                ja: "常に 1 つだけをオンにするのは[トグルグループ](components/toggle-group/)です。フォームの中で複数を選ぶには[チェックボックス](components/checkbox/)を使います。",
            },
        ),
    ])
}
