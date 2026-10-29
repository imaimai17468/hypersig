//! The page on `button` and `submit_button`, with `ButtonVariant` and
//! `ButtonSize`.

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
        lead(site, Doc::Components(Component::Button).leads()),
        demo::show(site, Demo::Button),
        accessibility(site),
        functions(site),
        look(site),
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
                en: "Each is a native `<button>`, so it is in the tab order, Enter and Space press it, and a screen reader announces its text followed by \"button\". `El::disabled` sets the `disabled` attribute, which takes the button out of the tab order, makes a screen reader announce it as dimmed or unavailable, and draws it at half opacity.",
                ja: "どれもネイティブの `<button>` なので、Tab キーで移動でき、Enter と Space で押せます。スクリーンリーダーはボタンの文字に続けて「ボタン」と読み上げます。`El::disabled` は `disabled` 属性を設定します。無効になったボタンは Tab キーの移動先から外れ、スクリーンリーダーには使用不可として読み上げられ、半透明で描かれます。",
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
                    &["Tab", "Moves focus to the button."],
                    &["Enter, Space", "Presses the button."],
                ],
                ja: &[
                    &["Tab", "ボタンにフォーカスを移します。"],
                    &["Enter、Space", "ボタンを押します。"],
                ],
            },
        ),
    ])
}

fn functions(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`button` and `submit_button`",
                ja: "`button` と `submit_button`",
            },
        ),
        para(
            site,
            L {
                en: "[`button`](api:hypersig_ui/fn.button.html) and [`submit_button`](api:hypersig_ui/fn.submit_button.html) take a variant and a size and return an empty `El<S>`, which takes its text, handlers, and other attributes like any element:",
                ja: "[`button`](api:hypersig_ui/fn.button.html) と [`submit_button`](api:hypersig_ui/fn.submit_button.html) はバリアントとサイズを受け取り、空の `El<S>` を返します。文字、ハンドラ、ほかの属性は、通常の要素と同じように付けます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn button<S>(variant: ButtonVariant, size: ButtonSize) -> El<S>\n\
             pub fn submit_button<S>(variant: ButtonVariant, size: ButtonSize) -> El<S>",
        ),
        para(
            site,
            L {
                en: "They differ only in `type`. `button` writes `type=\"button\"`, so it never submits the form around it, and `submit_button` writes `type=\"submit\"`, so it runs the form's `on_submit`. `button(ButtonVariant::Default, ButtonSize::Default).text(\"Save\")` and the same `submit_button` render as follows, without the `class` that carries the style:",
                ja: "2 つの違いは `type` だけです。`button` は `type=\"button\"` を書くので、囲んでいるフォームを送信しません。`submit_button` は `type=\"submit\"` を書くので、フォームの `on_submit` を実行します。`button(ButtonVariant::Default, ButtonSize::Default).text(\"Save\")` と同じ引数の `submit_button` は、スタイルを付ける `class` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<button type=\"button\">Save</button>\n<button type=\"submit\">Save</button>",
        ),
    ])
}

fn look(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Variants and sizes",
                ja: "バリアントとサイズ",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "#[non_exhaustive]\npub enum ButtonVariant { Default, Destructive, Outline, Secondary, Ghost, Link }\n\n\
             #[non_exhaustive]\npub enum ButtonSize { Default, Sm, Lg }",
        ),
        table_of(
            site,
            L {
                en: &["Variant", "Look", "Use"],
                ja: &["バリアント", "見た目", "用途"],
            },
            L {
                en: &[
                    &[
                        "`Default`",
                        "Filled with `primary`",
                        "The main action of a view",
                    ],
                    &[
                        "`Destructive`",
                        "Filled with `destructive`",
                        "An action that deletes",
                    ],
                    &[
                        "`Outline`",
                        "A `border` line on `background`",
                        "A second action beside the main one",
                    ],
                    &["`Secondary`", "Filled with `secondary`", "A quieter action"],
                    &[
                        "`Ghost`",
                        "No fill until the pointer is over it",
                        "An action in a toolbar or a list row",
                    ],
                    &[
                        "`Link`",
                        "Text in `primary`, underlined under the pointer",
                        "An action inside a sentence",
                    ],
                ],
                ja: &[
                    &["`Default`", "`primary` で塗る", "画面の主な操作"],
                    &["`Destructive`", "`destructive` で塗る", "削除する操作"],
                    &[
                        "`Outline`",
                        "`background` の上に `border` の線",
                        "主な操作の隣の 2 番目の操作",
                    ],
                    &["`Secondary`", "`secondary` で塗る", "控えめな操作"],
                    &[
                        "`Ghost`",
                        "ポインターが乗るまで塗らない",
                        "ツールバーやリストの行の中の操作",
                    ],
                    &[
                        "`Link`",
                        "`primary` の文字で、ポインターが乗ると下線",
                        "文の中の操作",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`ButtonSize::Sm` is 2rem high, `Default` 2.25rem, and `Lg` 2.5rem, with the inline padding growing from 0.75rem to 1.5rem. The colors are [tokens](components/tokens/), so a theme changes every button at once.",
                ja: "`ButtonSize::Sm` は高さ 2rem、`Default` は 2.25rem、`Lg` は 2.5rem で、左右の余白は 0.75rem から 1.5rem まで大きくなります。色は[トークン](components/tokens/)なので、テーマを変えるとすべてのボタンが一度に変わります。",
            },
        ),
        para(
            site,
            L {
                en: "[`ButtonVariant`](api:hypersig_ui/enum.ButtonVariant.html) and [`ButtonSize`](api:hypersig_ui/enum.ButtonSize.html) are enums, so a variant written as text, as a class name in shadcn/ui, fails to compile with E0308:",
                ja: "[`ButtonVariant`](api:hypersig_ui/enum.ButtonVariant.html) と [`ButtonSize`](api:hypersig_ui/enum.ButtonSize.html) は列挙型なので、shadcn/ui のクラス名のようにバリアントを文字列で書くと、E0308 でコンパイルできません。",
            },
        ),
        show(Snippet::UiVariantString),
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
                    en: "A button with no text has no name, and a screen reader announces just \"button\". A button that shows only an icon is an [icon button](components/icon-button/), which takes its name as an argument.",
                    ja: "文字のないボタンには名前がなく、スクリーンリーダーは「ボタン」としか読み上げません。アイコンだけを表示するボタンには、名前を引数に取る[アイコンボタン](components/icon-button/)を使います。",
                },
                L {
                    en: "A `submit_button` outside a `<form>` does nothing when pressed, and a `button` inside one never submits it. Nothing checks which of the two sits where.",
                    ja: "`<form>` の外の `submit_button` は押しても何も起きず、`<form>` の中の `button` はフォームを送信しません。どちらがどこにあるかは検査しません。",
                },
                L {
                    en: "Both enums are `#[non_exhaustive]`, so a `match` on them outside the crate needs a `_` arm, which a variant added in a later version falls into.",
                    ja: "どちらの列挙型も `#[non_exhaustive]` なので、クレートの外でこれらを `match` するには `_` の腕が必要です。後のバージョンで追加されたバリアントはその腕に入ります。",
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
                en: "Example: a quantity and an order",
                ja: "例: 数量と注文",
            },
        ),
        para(
            site,
            L {
                en: "The two outline buttons change the quantity and are disabled at either end. The ghost button resets it, and only the submit button sends the form:",
                ja: "2 つの outline のボタンは数量を変え、上限と下限で無効になります。ghost のボタンは数量を戻し、フォームを送信するのは submit のボタンだけです。",
            },
        ),
        show(Snippet::UiButton),
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
                en: "[Icon button](components/icon-button/) is a square button with only an icon. [Link styled as a button](components/link-button/) gives a link this look. [Events](learn/events/) covers `on_click` and the other handlers.",
                ja: "アイコンだけの正方形のボタンは[アイコンボタン](components/icon-button/)で、リンクにこの見た目を与える方法は[ボタンの見た目のリンク](components/link-button/)で説明しています。`on_click` などのハンドラは[イベント](learn/events/)で説明しています。",
            },
        ),
    ])
}
