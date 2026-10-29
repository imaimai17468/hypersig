//! The page on `toggle` and `icon_toggle`: a button that is pressed or not,
//! bound to a `bool` signal.

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
        lead(site, Doc::Components(Component::Toggle).leads()),
        demo::show(site, Demo::Toggle),
        accessibility(site),
        functions(site),
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
                en: "A toggle is a native `<button>` with `aria-pressed`, which follows the signal. A screen reader announces it as a toggle button and says whether it is pressed, such as \"Pin, toggle button, not pressed\" for the Pin toggle in the demo above, and announces the new state when it flips.",
                ja: "トグルは `aria-pressed` を持つネイティブの `<button>` で、`aria-pressed` はシグナルに従います。スクリーンリーダーはトグルボタンとして読み上げ、押されているかどうかも伝えます。上のデモのピン留めのトグルなら「ピン留め、トグルボタン、オフ」のように読み上げ、状態が変わると新しい状態を読み上げます。",
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
                    &["Tab", "Moves focus to the toggle."],
                    &["Enter, Space", "Flips the toggle."],
                ],
                ja: &[
                    &["Tab", "トグルにフォーカスを移します。"],
                    &["Enter、Space", "トグルを反転します。"],
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
                en: "`toggle` and `icon_toggle`",
                ja: "`toggle` と `icon_toggle`",
            },
        ),
        para(
            site,
            L {
                en: "[`toggle`](api:hypersig_ui/fn.toggle.html) takes the `bool` signal it shows and flips. [`icon_toggle`](api:hypersig_ui/fn.icon_toggle.html) is the same toggle, square, with the name that `aria-label` gives it as its first argument, for a toggle that shows only an icon:",
                ja: "[`toggle`](api:hypersig_ui/fn.toggle.html) は表示して反転する `bool` のシグナルを受け取ります。[`icon_toggle`](api:hypersig_ui/fn.icon_toggle.html) は同じトグルを正方形にしたもので、アイコンだけを表示するトグルのために、`aria-label` に入る名前を最初の引数に取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn toggle<S, R: Writable>(pressed: Sig<S, bool, R>) -> El<S>\n\
             pub fn icon_toggle<S, R: Writable>(label: impl Into<String>, pressed: Sig<S, bool, R>) -> El<S>",
        ),
        para(
            site,
            L {
                en: "A click sets the signal to its opposite, and `aria-pressed` and the toggle's colors follow. With `bold` true and `italic` false, `toggle(Note::bold()).text(\"Bold\")` and `icon_toggle(\"Italic\", Note::italic()).text(\"I\")` render as follows, without the `class`:",
                ja: "クリックするとシグナルが反対の値になり、`aria-pressed` とトグルの色がそれに従います。`bold` が true、`italic` が false のとき、`toggle(Note::bold()).text(\"Bold\")` と `icon_toggle(\"Italic\", Note::italic()).text(\"I\")` は、`class` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<button type=\"button\" aria-pressed=\"true\">Bold</button>\n\
             <button type=\"button\" aria-label=\"Italic\" aria-pressed=\"false\">I</button>",
        ),
        para(
            site,
            L {
                en: "The toggle writes to its signal, so the signal's third type parameter has to be `Writable`. A [computed signal](learn/computed-signals/) is `Computed`, which the page derives from other signals, and a toggle bound to one fails to compile with E0277, `a signal marked Computed cannot be written`:",
                ja: "トグルはシグナルに書き込むので、シグナルの 3 つ目の型パラメーターは `Writable` でなければなりません。[計算シグナル](learn/computed-signals/)はほかのシグナルから導かれる `Computed` なので、それに結び付けたトグルは E0277（`a signal marked Computed cannot be written`）でコンパイルできません。",
            },
        ),
        show(Snippet::UiToggleComputed),
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
                    en: "Keep the text the same in both states. A screen reader announces the state on its own, so a text that changes with it, such as \"Mute\" and \"Unmute\", reads as \"Unmute, pressed\" and leaves the meaning unclear.",
                    ja: "文字は両方の状態で同じにします。状態はスクリーンリーダーが別に読み上げるので、「ミュート」と「ミュート解除」のように状態で変わる文字だと「ミュート解除、オン」と読まれ、意味が分かりにくくなります。",
                },
                L {
                    en: "An empty label for `icon_toggle` compiles and leaves the toggle with no name.",
                    ja: "`icon_toggle` のラベルを空にしてもコンパイルでき、トグルは名前のないままになります。",
                },
                L {
                    en: "A toggle needs no id, so it works on a field of a list item, such as `item.at(Task::done())` inside `each`.",
                    ja: "トグルは id を必要としないので、`each` の中の `item.at(Task::done())` のような、リストの項目のフィールドにも使えます。",
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
                en: "Example: text style",
                ja: "例: 文字のスタイル",
            },
        ),
        show(Snippet::UiToggle),
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
                en: "[Toggle group](components/toggle-group/) picks one value from a row of toggles, and [Multiple-choice toggle group](components/multi-toggle-group/) groups toggles that each flip their own signal. [Switch](components/switch/) is the on and off control for a setting.",
                ja: "トグルの並びから値を 1 つ選ぶのが[トグルグループ](components/toggle-group/)で、それぞれが自分のシグナルを反転するトグルをまとめるのが[複数選択のトグルグループ](components/multi-toggle-group/)です。設定のオンとオフには[スイッチ](components/switch/)を使います。",
            },
        ),
    ])
}
