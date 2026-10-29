//! The page on `Slider`: a labeled native range input bound to an `i32`
//! signal, drawn as a track and a thumb.

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
        lead(site, Doc::Components(Component::Slider).leads()),
        demo::show(site, Demo::Slider),
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
                en: "It is a native `<input type=\"range\">` named by its `<label>`, so the browser gives it `role=\"slider\"`, the keys below, and dragging with a pointer or a finger, and a screen reader announces \"Volume, slider, 40\". The input is transparent and lies over a track and a thumb drawn from the value, so the slider looks the same in every browser. The value shown beside the label is hidden from assistive technology, which reads it from the input, or from `aria-valuetext` where `value_text` sets one.",
                ja: "`<label>` で名前の付いたネイティブの `<input type=\"range\">` なので、ブラウザが `role=\"slider\"`、次のキー操作、ポインターや指でのドラッグを与え、スクリーンリーダーは「Volume、スライダー、40」のように読み上げます。入力要素は透明で、値から描いたトラックとつまみの上に重なっているので、どのブラウザでも同じ見た目になります。ラベルの横に表示する値は支援技術からは隠れていて、支援技術は値を入力要素から、`value_text` を指定した場合は `aria-valuetext` から読みます。",
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
                    &["Arrow keys", "Move the value by one step."],
                    &[
                        "Page Up, Page Down",
                        "Move the value by a larger step, which the browser picks.",
                    ],
                    &["Home, End", "Move to the lowest or the highest value."],
                ],
                ja: &[
                    &["矢印キー", "値を 1 ステップ動かします。"],
                    &[
                        "Page Up、Page Down",
                        "ブラウザが決める大きなステップで値を動かします。",
                    ],
                    &["Home、End", "最小値または最大値に移ります。"],
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
                en: "`Slider`",
                ja: "`Slider`",
            },
        ),
        para(
            site,
            L {
                en: "[`Slider`](api:hypersig_ui/struct.Slider.html) is a builder. `new` makes a slider from 0 to 100 in steps of 1, which `range` and `step` change:",
                ja: "[`Slider`](api:hypersig_ui/struct.Slider.html) はビルダーです。`new` は 0 から 100 まで 1 刻みのスライダーを作り、`range` と `step` でそれを変えます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Slider<S> {\n    \
             pub fn new(label: impl Into<String>, value: Sig<S, i32>) -> Self\n    \
             pub fn range(self, min: i32, max: i32) -> Self\n    \
             pub fn step(self, step: i32) -> Self\n    \
             pub fn value_text(self, text: impl IntoExpr<S, String>) -> Self\n    \
             pub fn disabled(self, when: impl IntoExpr<S, bool>) -> Self\n    \
             pub fn render(self) -> El<S>\n}",
        ),
        para(
            site,
            L {
                en: "`value_text` takes an expression for a value with a unit, such as \"40 %\": it is shown beside the label and written to `aria-valuetext`, so a screen reader reads the unit too. With the signal at 40, the slider of the example below renders as follows, without the `class`, the ids, and the inline styles that place the fill and the thumb:",
                ja: "`value_text` は \"40 %\" のような単位付きの値を表す式を受け取ります。その値はラベルの横に表示され、`aria-valuetext` にも書かれるので、スクリーンリーダーも単位まで読み上げます。シグナルが 40 のとき、下の例のスライダーは、`class`、id、塗りとつまみの位置を決めるインラインのスタイルを除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div>\n  <div>\n    <label for=\"…\">Volume</label>\n    <span aria-hidden=\"true\">40 %</span>\n  </div>\n  <div>\n    \
             <input id=\"…\" type=\"range\" min=\"0\" max=\"100\" step=\"5\" aria-valuetext=\"40 %\" value=\"40\">\n    \
             <div><div></div></div>\n    <div></div>\n  </div>\n</div>",
        ),
        para(
            site,
            L {
                en: "The browser's range input and the binding read whole numbers here, so `new` takes a `Sig<S, i32>`, and an `f64` signal fails to compile with E0308:",
                ja: "ここではブラウザの範囲入力と束縛が整数を読むので、`new` は `Sig<S, i32>` を受け取り、`f64` のシグナルは E0308 でコンパイルできません。",
            },
        ),
        show(Snippet::UiSliderFloat),
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
                    en: "`range` panics when `min` is not below `max`, and `step` when it is not above zero. The panic comes where the slider is built.",
                    ja: "`range` は `min` が `max` より小さくないとき、`step` は 0 より大きくないときに panic します。panic はスライダーを作ったところで起きます。",
                },
                L {
                    en: "A signal holding a value outside the range draws the thumb at the nearer end, and the browser puts the input there too, while the signal keeps its value until the visitor moves the slider.",
                    ja: "範囲外の値を持つシグナルでは、つまみは近いほうの端に描かれ、ブラウザも入力要素をその端に置きます。シグナルの値は、訪問者がスライダーを動かすまでそのままです。",
                },
                L {
                    en: "The drawn thumb is 1rem wide, and the browser's transparent thumb, which the pointer drags, has the width the browser gives it, so the drawn thumb can sit a few pixels off the pointer near either end.",
                    ja: "描いたつまみの幅は 1rem ですが、ポインターで実際にドラッグする透明なつまみの幅はブラウザが決めます。そのため、両端の近くでは、描いたつまみがポインターから数ピクセルずれることがあります。",
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
                en: "Example: a volume in percent",
                ja: "例: パーセントで表す音量",
            },
        ),
        show(Snippet::UiSlider),
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
                en: "[Number field](components/number-field/) takes an exact number, an `f64` too. [Progress](components/progress/) shows a value the visitor does not change.",
                ja: "正確な数値や `f64` を入力するには[数値フィールド](components/number-field/)を使います。訪問者が変えない値を表示するのは[プログレス](components/progress/)です。",
            },
        ),
    ])
}
