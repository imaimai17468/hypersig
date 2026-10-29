//! The Style page on `Color`: its constants and constructors, the properties
//! that take it, and the notations it cannot write.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Colors).leads()),
        building(site),
        accepted(site),
        unchecked(site),
        example(site),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a color",
                ja: "色を作る",
            },
        ),
        para(
            site,
            L {
                en: "[`Color`](api:hypersig/style/value/struct.Color.html) has a constant for each CSS named color, generated from MDN's data, and constructors for the other forms:",
                ja: "[`Color`](api:hypersig/style/value/struct.Color.html) には MDN のデータから生成した CSS の名前付きの色ごとの定数と、ほかの形を作るコンストラクタがあります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Item", "Signature", "CSS"],
                ja: &["項目", "シグネチャ", "CSS"],
            },
            L {
                en: &[
                    &[
                        "`Color::CRIMSON` and the other named colors",
                        "`const CRIMSON: Color`",
                        "`crimson`",
                    ],
                    &[
                        "`Color::CURRENT_COLOR`",
                        "`const CURRENT_COLOR: Color`",
                        "`currentColor`, the element's `color`",
                    ],
                    &[
                        "`Color::TRANSPARENT`",
                        "`const TRANSPARENT: Color`",
                        "`transparent`",
                    ],
                    &[
                        "[`Color::hex`](api:hypersig/style/value/struct.Color.html#method.hex)",
                        "`fn hex(rgb: u32) -> Color`",
                        "`#rrggbb`, and panics above `0xffffff`",
                    ],
                    &[
                        "[`Color::hex_alpha`](api:hypersig/style/value/struct.Color.html#method.hex_alpha)",
                        "`fn hex_alpha(rgba: u32) -> Color`",
                        "`#rrggbbaa`",
                    ],
                    &[
                        "[`Color::rgb`](api:hypersig/style/value/struct.Color.html#method.rgb)",
                        "`fn rgb(red: u8, green: u8, blue: u8) -> Color`",
                        "`rgb(r g b)`",
                    ],
                    &[
                        "[`Color::hsl`](api:hypersig/style/value/struct.Color.html#method.hsl)",
                        "`fn hsl(hue: Angle, saturation: Percentage, lightness: Percentage) -> Color`",
                        "`hsl(h s l)`",
                    ],
                    &[
                        "[`Color::oklch`](api:hypersig/style/value/struct.Color.html#method.oklch)",
                        "`fn oklch(lightness: Percentage, chroma: f64, hue: Angle) -> Color`",
                        "`oklch(l c h)`",
                    ],
                    &[
                        "[`Color::oklab`](api:hypersig/style/value/struct.Color.html#method.oklab)",
                        "`fn oklab(lightness: Percentage, a: f64, b: f64) -> Color`",
                        "`oklab(l a b)`",
                    ],
                    &[
                        "[`Color::opacity`](api:hypersig/style/value/struct.Color.html#method.opacity)",
                        "`fn opacity(&self, opacity: u8) -> Color`",
                        "`color-mix(in srgb, <color> <opacity>%, transparent)`",
                    ],
                    &[
                        "[`Color::mix`](api:hypersig/style/value/struct.Color.html#method.mix)",
                        "`fn mix(&self, other: &Color, weight: u8) -> Color`",
                        "`color-mix(in srgb, <color> <weight>%, <other>)`",
                    ],
                ],
                ja: &[
                    &[
                        "`Color::CRIMSON` などの名前付きの色",
                        "`const CRIMSON: Color`",
                        "`crimson`",
                    ],
                    &[
                        "`Color::CURRENT_COLOR`",
                        "`const CURRENT_COLOR: Color`",
                        "`currentColor`（その要素の `color`）",
                    ],
                    &[
                        "`Color::TRANSPARENT`",
                        "`const TRANSPARENT: Color`",
                        "`transparent`",
                    ],
                    &[
                        "[`Color::hex`](api:hypersig/style/value/struct.Color.html#method.hex)",
                        "`fn hex(rgb: u32) -> Color`",
                        "`#rrggbb`。`0xffffff` を超えると panic します",
                    ],
                    &[
                        "[`Color::hex_alpha`](api:hypersig/style/value/struct.Color.html#method.hex_alpha)",
                        "`fn hex_alpha(rgba: u32) -> Color`",
                        "`#rrggbbaa`",
                    ],
                    &[
                        "[`Color::rgb`](api:hypersig/style/value/struct.Color.html#method.rgb)",
                        "`fn rgb(red: u8, green: u8, blue: u8) -> Color`",
                        "`rgb(r g b)`",
                    ],
                    &[
                        "[`Color::hsl`](api:hypersig/style/value/struct.Color.html#method.hsl)",
                        "`fn hsl(hue: Angle, saturation: Percentage, lightness: Percentage) -> Color`",
                        "`hsl(h s l)`",
                    ],
                    &[
                        "[`Color::oklch`](api:hypersig/style/value/struct.Color.html#method.oklch)",
                        "`fn oklch(lightness: Percentage, chroma: f64, hue: Angle) -> Color`",
                        "`oklch(l c h)`",
                    ],
                    &[
                        "[`Color::oklab`](api:hypersig/style/value/struct.Color.html#method.oklab)",
                        "`fn oklab(lightness: Percentage, a: f64, b: f64) -> Color`",
                        "`oklab(l a b)`",
                    ],
                    &[
                        "[`Color::opacity`](api:hypersig/style/value/struct.Color.html#method.opacity)",
                        "`fn opacity(&self, opacity: u8) -> Color`",
                        "`color-mix(in srgb, <色> <opacity>%, transparent)`",
                    ],
                    &[
                        "[`Color::mix`](api:hypersig/style/value/struct.Color.html#method.mix)",
                        "`fn mix(&self, other: &Color, weight: u8) -> Color`",
                        "`color-mix(in srgb, <色> <weight>%, <other>)`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`Color` implements `Display`, which writes the CSS text:",
                ja: "`Color` は `Display` を実装しており、CSS のテキストを書き出します。",
            },
        ),
        show(Snippet::StyleColors),
        para(
            site,
            L {
                en: "`opacity` and `mix` take `&self` and return a new color, so one color can be the base of several. They write CSS's `color-mix()` in sRGB, so the browser computes the result.",
                ja: "`opacity` と `mix` は `&self` を受け取って新しい色を返すので、1 つの色から複数の色を作れます。どちらも sRGB での CSS の `color-mix()` を書くので、結果の色はブラウザが計算します。",
            },
        ),
    ])
}

fn accepted(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Properties that take a color",
                ja: "色を受け取るプロパティ",
            },
        ),
        para(
            site,
            L {
                en: "`color`, `background_color`, and the other properties whose value is only a color take `impl Into<Color>`. A property with keywords besides colors takes its own enum, which converts from `Color`: `accent_color` takes an `AccentColor`, which also has `Auto`, and `border_color` takes a `BorderColor`. A color written as a string has no conversion, so it is E0277:",
                ja: "`color`、`background_color` など、値が色だけのプロパティは `impl Into<Color>` を受け取ります。色のほかにキーワードを持つプロパティは専用の列挙型を受け取り、その型は `Color` から変換できます。`accent_color` は `Auto` も持つ `AccentColor` を、`border_color` は `BorderColor` を受け取ります。文字列で書いた色には変換がないので、E0277 になります。",
            },
        ),
        show(Snippet::StyleColorString),
        para(
            site,
            L {
                en: "A color shared across the page, or one that changes in dark mode, is a [token](style/tokens/). A `Token<Color>`'s `var()` returns a `Color`, which every one of these methods takes.",
                ja: "ページ全体で共有する色や、ダークモードで変わる色は[トークン](style/tokens/)にします。`Token<Color>` の `var()` は `Color` を返すので、これらのメソッドのどれにでも渡せます。",
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
                en: "`Color::hex` panics on a value above `0xffffff`, so an eight-digit `#rrggbbaa` literal stops where it is built, and `Color::hex_alpha` takes it instead. A literal whose red byte is `00`, such as `0x00ff_0080`, is the number `0xff0080`, so `hex` reads it as `#ff0080` and the alpha is lost. `opacity` and `mix` read a percentage above 100 as 100:",
                ja: "`Color::hex` は `0xffffff` を超える値で panic するので、8 桁の `#rrggbbaa` のリテラルは作ったところで止まります。8 桁の色には `Color::hex_alpha` を使います。`0x00ff_0080` のように赤のバイトが `00` のリテラルは数としては `0xff0080` なので、`hex` はそれを `#ff0080` として読み、透明度は失われます。`opacity` と `mix` は 100 を超える割合を 100 として扱います。",
            },
        ),
        show(Snippet::StyleColorLimits),
        para(
            site,
            L {
                en: "For a translucent color, use `hex_alpha` or `opacity`. `Color::rgb` takes `u8`, so a literal channel above 255 is a compile error from the `overflowing_literals` lint. Nothing checks the contrast between a text color and its background.",
                ja: "半透明の色には `hex_alpha` か `opacity` を使います。`Color::rgb` は `u8` を受け取るので、255 を超えるチャンネルをリテラルで書くと、`overflowing_literals` リントによるコンパイルエラーになります。文字の色と背景のコントラストは検査されません。",
            },
        ),
        para(
            site,
            L {
                en: "`hsl`, `oklch`, and `oklab` take their channels as typed values, so a hue is an `Angle` such as `deg(210)` and a lightness a `Percentage`. A channel is not checked against its range, and the browser clamps a value outside it. A color function with no constructor, such as `lab()`, `color()`, or `light-dark()`, goes through `raw`, which `Color` converts from: `color(raw(\"color(display-p3 1 0.5 0)\"))`. [Unchecked values](style/unchecked/) covers `raw`.",
                ja: "`hsl`、`oklch`、`oklab` はチャンネルを型の付いた値で受け取ります。色相は `deg(210)` のような `Angle`、明度は `Percentage` です。チャンネルが範囲に収まっているかは検査されず、範囲外の値はブラウザが範囲内に丸めます。`lab()`、`color()`、`light-dark()` のようにコンストラクタのない色の関数は `raw` で書きます。`Color` は `raw` から変換できるので、`color(raw(\"color(display-p3 1 0.5 0)\"))` と書けます。`raw` は[検査しない値](style/unchecked/)で説明します。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: status badges",
                ja: "例: 状態を示すバッジ",
            },
        ),
        para(
            site,
            L {
                en: "Each badge is built from one hue: the text in the hue, a fill of the hue at 12%, and a border of 80% hue mixed with black. A new status needs only a new hue:",
                ja: "どのバッジも 1 つの色相から作ります。文字はその色、背景はその色を 12% にしたもの、枠線はその色 80% に黒を混ぜたものです。状態を増やすときは色相を 1 つ足すだけです。",
            },
        ),
        show(Snippet::StyleBadge),
        para(
            site,
            L {
                en: "Next, [Angles and times](style/angles-and-times/) covers rotations and durations.",
                ja: "次の[角度と時間](style/angles-and-times/)では回転と時間の長さを説明します。",
            },
        ),
    ])
}
