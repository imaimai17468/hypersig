//! The page on `icon_button`, whose name is a required argument.

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
        lead(site, Doc::Components(Component::IconButton).leads()),
        demo::show(site, Demo::Button),
        accessibility(site),
        function(site),
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
                en: "It is a native `<button type=\"button\">`, so it takes focus and Enter and Space press it, as [Button](components/button/) does. An icon is not text a screen reader can read, so the button's name comes from `aria-label`, which the label argument fills: the screen reader announces \"Louder, button\" for the button in the example below. The `+` button in the demo above is one.",
                ja: "ネイティブの `<button type=\"button\">` なので、[ボタン](components/button/)と同じようにフォーカスを受け取り、Enter と Space で押せます。アイコンはスクリーンリーダーが読めるテキストではないので、ボタンの名前はラベルの引数を入れた `aria-label` から取ります。下の例のボタンは「Louder、ボタン」と読み上げられます。上のデモの `+` のボタンもアイコンボタンです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            L {
                en: &[&["Enter, Space", "Presses the button."]],
                ja: &[&["Enter、Space", "ボタンを押します。"]],
            },
        ),
    ])
}

fn function(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`icon_button`",
                ja: "`icon_button`",
            },
        ),
        para(
            site,
            L {
                en: "[`icon_button`](api:hypersig_ui/fn.icon_button.html) takes a variant and the name, and returns a square button 2.25rem on each side with no padding. It has no size argument:",
                ja: "[`icon_button`](api:hypersig_ui/fn.icon_button.html) はバリアントと名前を受け取り、一辺 2.25rem で余白のない正方形のボタンを返します。サイズの引数はありません。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn icon_button<S>(variant: ButtonVariant, label: impl Into<String>) -> El<S>",
        ),
        para(
            site,
            L {
                en: "`icon_button(ButtonVariant::Ghost, \"More actions\").text(\"…\")` renders as follows, without the `class` that carries the style. Add an `<svg>` with `El::child` in place of the text for a drawn icon:",
                ja: "`icon_button(ButtonVariant::Ghost, \"More actions\").text(\"…\")` は、スタイルを付ける `class` を除くと次のように描画されます。描いたアイコンを使うには、テキストの代わりに `El::child` で `<svg>` を加えます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<button type=\"button\" aria-label=\"More actions\">…</button>",
        ),
        para(
            site,
            L {
                en: "The name is a parameter, so an icon button without one fails to compile with E0061, `this function takes 2 arguments but 1 argument was supplied`:",
                ja: "名前は引数なので、名前のないアイコンボタンは E0061（`this function takes 2 arguments but 1 argument was supplied`）でコンパイルできません。",
            },
        ),
        show(Snippet::UiIconWithoutLabel),
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
                    en: "An empty label compiles and leaves the button without a name. Name the action, such as \"Delete\", rather than the icon, such as \"Trash can\".",
                    ja: "空のラベルもコンパイルでき、ボタンは名前のないままになります。ラベルには「ゴミ箱」のようなアイコンの名前ではなく、「削除」のような操作の名前を付けます。",
                },
                L {
                    en: "`aria-label` replaces the button's text for assistive technology, so any text inside the button, such as `…`, is not read. Speech control software matches what the visitor says against the label.",
                    ja: "`aria-label` は支援技術に対してボタンの中身の文字を置き換えるので、`…` のようなボタン内の文字は読み上げられません。音声操作のソフトウェアは訪問者の発話をラベルと照らし合わせます。",
                },
                L {
                    en: "An `<svg>` you add stays visible to assistive technology unless it has `aria-hidden=\"true\"`. Add that attribute, so the icon is not announced as an image after the name.",
                    ja: "追加した `<svg>` は、`aria-hidden=\"true\"` がなければ支援技術からも見えたままです。名前の後に画像として読み上げられないように、この属性を付けます。",
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
                en: "Example: a volume control",
                ja: "例: 音量の調整",
            },
        ),
        show(Snippet::UiIconButton),
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
                en: "[Toggle](components/toggle/) has `icon_toggle`, an icon button that stays pressed. [Tooltip](components/tooltip/) adds a description to an icon button with `Tooltip::icon_trigger`.",
                ja: "押された状態を保つアイコンボタンは、[トグル](components/toggle/)の `icon_toggle` です。アイコンボタンに説明を加えるには、[ツールチップ](components/tooltip/)の `Tooltip::icon_trigger` を使います。",
            },
        ),
    ])
}
