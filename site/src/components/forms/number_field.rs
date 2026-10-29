//! The page on `NumberField` and the `Numeric` trait.

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
        lead(site, Doc::Components(Component::NumberField).leads()),
        demo::show(site, Demo::TextField),
        accessibility(site),
        builder(site),
        numeric(site),
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
                en: "It is an `<input type=\"number\">` with the label, `aria-describedby`, `aria-invalid`, and live error of a [text field](components/text-field/). A screen reader announces it as a spin button with its value, and the browser gives it the keys below. The Guests field in the demo above is one.",
                ja: "[テキストフィールド](components/text-field/)と同じラベル、`aria-describedby`、`aria-invalid`、ライブ領域のエラーを持つ `<input type=\"number\">` です。スクリーンリーダーは値とともにスピンボタンとして読み上げ、ブラウザが次のキー操作を与えます。上のデモの「人数」の入力欄がこれです。",
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
                        "Up, Down",
                        "Raises or lowers the value by one step, within `min` and `max`.",
                    ],
                    &[
                        "Digits",
                        "Type the value. A phone shows a keyboard of digits where `min` is 0 or more.",
                    ],
                ],
                ja: &[
                    &[
                        "↑、↓",
                        "`min` と `max` の範囲で、値を 1 ステップ上げ下げします。",
                    ],
                    &[
                        "数字",
                        "値を入力します。`min` が 0 以上なら、スマートフォンには数字のキーボードが出ます。",
                    ],
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
                en: "`NumberField`",
                ja: "`NumberField`",
            },
        ),
        para(
            site,
            L {
                en: "[`NumberField`](api:hypersig_ui/struct.NumberField.html) is a builder like `TextField`, with a second type parameter `T`, the number type of its signal. `min`, `max`, and `step` take a `T`, so a bound is a number of the field's own type:",
                ja: "[`NumberField`](api:hypersig_ui/struct.NumberField.html) は `TextField` と同じくビルダーで、シグナルの数値型を表す 2 つ目の型パラメーター `T` を持ちます。`min`、`max`、`step` は `T` を受け取るので、範囲は入力欄と同じ型の数値になります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S, T: Numeric> NumberField<S, T> {\n    \
             pub fn new(label: impl Into<String>, value: Sig<S, T>) -> Self\n    \
             pub fn min(self, min: T) -> Self\n    \
             pub fn max(self, max: T) -> Self\n    \
             pub fn step(self, step: T) -> Self\n    \
             pub fn description(self, text: impl Into<String>) -> Self\n    \
             pub fn error(self, message: impl IntoExpr<S, String>) -> Self\n    \
             pub fn required(self) -> Self\n    \
             pub fn disabled(self, when: impl IntoExpr<S, bool>) -> Self\n    \
             pub fn render(self) -> El<S>\n}",
        ),
        para(
            site,
            L {
                en: "`min(1).max(10)` on an `i32` field and `min(0.0)` on an `f64` field render as follows, without the ids and the `class`. A minimum of 0 or more adds `inputmode`, `numeric` for `i32` and `decimal` for `f64`, and an `f64` field without a step gets `step=\"any\"`, since the browser's own step of 1 would refuse a value such as 2.5:",
                ja: "`i32` の入力欄の `min(1).max(10)` と、`f64` の入力欄の `min(0.0)` は、id と `class` を除くと次のように描画されます。最小値が 0 以上なら `inputmode` が付き、`i32` では `numeric`、`f64` では `decimal` になります。ブラウザ既定のステップ 1 では 2.5 のような値が拒否されるので、ステップを指定しない `f64` の入力欄には `step=\"any\"` が付きます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<label for=\"…\">Quantity</label>\n\
             <input id=\"…\" type=\"number\" inputmode=\"numeric\" min=\"1\" max=\"10\" value=\"1\">\n\n\
             <label for=\"…\">Tip</label>\n\
             <input id=\"…\" type=\"number\" inputmode=\"decimal\" min=\"0\" step=\"any\" value=\"0\">",
        ),
    ])
}

fn numeric(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`Numeric`",
                ja: "`Numeric`",
            },
        ),
        para(
            site,
            L {
                en: "[`Numeric`](api:hypersig_ui/trait.Numeric.html) is the bound on `T`. It is sealed and implemented for `i32` and `f64`, the two types that the browser's number input and the runtime's binding read:",
                ja: "[`Numeric`](api:hypersig_ui/trait.Numeric.html) は `T` の境界です。封印されたトレイトで、ブラウザの数値入力とランタイムの束縛が読む 2 つの型、`i32` と `f64` に実装されています。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub trait Numeric: Bindable + Copy + PartialOrd + Display + Sealed {}\n\n\
             impl Numeric for i32 {}\nimpl Numeric for f64 {}",
        ),
        para(
            site,
            L {
                en: "A signal of another type fails to compile with E0277 and the message `a number field cannot be bound to a String signal`, with the note `bind an i32 or f64 signal; text goes in a TextField`:",
                ja: "ほかの型のシグナルは E0277 でコンパイルできず、`a number field cannot be bound to a String signal` というメッセージと、`bind an i32 or f64 signal; text goes in a TextField` という注記が出ます。",
            },
        ),
        show(Snippet::UiNumberFieldText),
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
        para(
            site,
            L {
                en: "The types cannot see the values. `min` panics when it is above a maximum set before, `max` when it is below a minimum set before, either one on an `f64` NaN, and `step` when it is not above zero. The panic comes where the field is built, so a test that renders the page finds it:",
                ja: "値そのものは型では分かりません。`min` は先に設定した最大値より大きいとき、`max` は先に設定した最小値より小さいとき、どちらも `f64` の NaN のとき、`step` は 0 より大きくないときに panic します。panic は入力欄を作ったところで起きるので、ページを描画するテストで見つけられます。",
            },
        ),
        show(Snippet::UiNumberBounds),
        bullets(
            site,
            &[
                L {
                    en: "The arrow keys stop at `min` and `max`, but a typed value outside them still goes to the signal. A plain `<form>` refuses to submit it, and a form from `Sig::form` does not, since it has `novalidate`. Check the range again in an error message or a form rule, and on the server.",
                    ja: "矢印キーは `min` と `max` で止まりますが、範囲外の値を入力すると、その値はシグナルに入ります。通常の `<form>` はそれを送信しませんが、`Sig::form` のフォームは `novalidate` を持つので送信します。範囲はエラーメッセージかフォームのルールと、サーバーでも検査します。",
                },
                L {
                    en: "A minimum below 0 adds no `inputmode`, so a phone shows its full number keyboard with the minus sign.",
                    ja: "最小値が 0 未満なら `inputmode` は付かないので、スマートフォンにはマイナス記号を含む通常の数値キーボードが出ます。",
                },
                L {
                    en: "A field the visitor empties leaves the signal at its last number, and a decimal such as 2.5 typed into an `i32` field sets the signal to its integer part, 2.",
                    ja: "訪問者が入力欄を空にしても、シグナルは最後の数値のままです。`i32` の入力欄に 2.5 のような小数を入力すると、シグナルには整数部の 2 が入ります。",
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
                en: "Example: a quantity and a tip",
                ja: "例: 数量とチップ",
            },
        ),
        show(Snippet::UiNumberField),
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
                en: "[Slider](components/slider/) sets an `i32` by dragging. [Fields bound to a form](components/form-fields/) has `NumberField::in_form`, which takes its message from a form's rules.",
                ja: "ドラッグで `i32` を設定するのは[スライダー](components/slider/)です。フォームのルールからメッセージを取る `NumberField::in_form` は、[フォームに結び付けた入力欄](components/form-fields/)で説明しています。",
            },
        ),
    ])
}
