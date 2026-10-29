//! The page on `switch`: a native checkbox with `role="switch"`, bound to a
//! `bool` signal.

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
        lead(site, Doc::Components(Component::Switch).leads()),
        demo::show(site, Demo::Checkbox),
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
                en: "It is the native checkbox of [Checkbox](components/checkbox/), inside its `<label>`, with `role=\"switch\"`. A screen reader therefore says \"Airplane mode, switch, off\" rather than \"checkbox, not checked\", and announces \"on\" or \"off\" as it flips. The track and the thumb are drawn and hidden from assistive technology, and follow the input's state.",
                ja: "[チェックボックス](components/checkbox/)と同じく `<label>` の中のネイティブのチェックボックスで、`role=\"switch\"` を持ちます。そのため、スクリーンリーダーは「チェックボックス、チェックなし」ではなく「Airplane mode、スイッチ、オフ」のように読み上げ、切り替えると「オン」や「オフ」を読み上げます。トラックとつまみは描画したもので、支援技術からは隠れていて、入力要素の状態に従います。",
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
                    &["Tab", "Moves focus to the switch."],
                    &["Space", "Turns it on or off."],
                ],
                ja: &[
                    &["Tab", "スイッチにフォーカスを移します。"],
                    &["Space", "オンとオフを切り替えます。"],
                ],
            },
        ),
    ])
}

fn function(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`switch`",
                ja: "`switch`",
            },
        ),
        para(
            site,
            L {
                en: "[`switch`](api:hypersig_ui/fn.switch.html) takes the text and the `bool` signal, as `checkbox` does:",
                ja: "[`switch`](api:hypersig_ui/fn.switch.html) は `checkbox` と同じく、テキストと `bool` のシグナルを受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn switch<S, R: Writable>(text: impl Into<String>, on: Sig<S, bool, R>) -> El<S>",
        ),
        para(
            site,
            L {
                en: "With the signal true, `switch(\"Airplane mode\", Settings::airplane())` renders as follows, without the `class`. The two empty `<span>` elements are the track and the thumb:",
                ja: "シグナルが true のとき、`switch(\"Airplane mode\", Settings::airplane())` は、`class` を除くと次のように描画されます。2 つの空の `<span>` がトラックとつまみです。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<label>\n  <span>\n    <input type=\"checkbox\" role=\"switch\" checked=\"\">\n    <span aria-hidden=\"true\"></span>\n    <span aria-hidden=\"true\"></span>\n  </span>\n  <span>Airplane mode</span>\n</label>",
        ),
        para(
            site,
            L {
                en: "A switch is on or off, so it takes a `Sig<S, bool, R>`. A signal of another type, such as text holding \"on\" or \"off\", fails to compile with E0308, and a computed signal with E0277:",
                ja: "スイッチはオンかオフなので、`Sig<S, bool, R>` を受け取ります。\"on\" や \"off\" を持つ文字列のような、ほかの型のシグナルは E0308 で、計算シグナルは E0277 でコンパイルできません。",
            },
        ),
        show(Snippet::UiSwitchText),
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
                    en: "A switch says that the change takes effect at once. Nothing stops a switch inside a form that waits for a submit button, where a [checkbox](components/checkbox/) says what happens.",
                    ja: "スイッチは、変更がすぐに反映されることを表します。送信ボタンを待つフォームの中にスイッチを置いても止められませんが、そこでは[チェックボックス](components/checkbox/)のほうが動作を正しく伝えます。",
                },
                L {
                    en: "Keep the text the same whether the switch is on or off, since the screen reader announces the state.",
                    ja: "スクリーンリーダーが状態を読み上げるので、テキストはオンでもオフでも同じにします。",
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
                en: "Example: settings",
                ja: "例: 設定",
            },
        ),
        show(Snippet::UiSwitch),
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
                en: "[Fields bound to a form](components/form-fields/) has `switch_in_form`. [Toggle](components/toggle/) is a button that stays pressed, for a tool rather than a setting.",
                ja: "`switch_in_form` は[フォームに結び付けた入力欄](components/form-fields/)で説明しています。設定ではなくツールのオンとオフには、押された状態を保つボタンである[トグル](components/toggle/)を使います。",
            },
        ),
    ])
}
