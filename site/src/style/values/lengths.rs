//! The Style page on `Length` and `Percentage`, the functions that build
//! them, and what the property types accept.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Lengths).leads()),
        types(site),
        units(site),
        accepted(site),
        math(site),
        unchecked(site),
        example(site),
    ])
}

fn types(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Length and Percentage",
                ja: "Length と Percentage",
            },
        ),
        para(
            site,
            L {
                en: "[`Length`](api:hypersig/style/value/struct.Length.html) is a number with a length unit, and [`Percentage`](api:hypersig/style/value/struct.Percentage.html) is a number with `%`. Each unit has a function, such as [`fn px(value: impl Into<f64>) -> Length`](api:hypersig/style/value/fn.px.html), and [`fn pct(value: impl Into<f64>) -> Percentage`](api:hypersig/style/value/fn.pct.html) builds a percentage. Both types implement `Display`, which writes the value as the stylesheet holds it:",
                ja: "[`Length`](api:hypersig/style/value/struct.Length.html) は長さの単位の付いた数値で、[`Percentage`](api:hypersig/style/value/struct.Percentage.html) は `%` の付いた数値です。単位ごとに [`fn px(value: impl Into<f64>) -> Length`](api:hypersig/style/value/fn.px.html) のような関数があり、パーセンテージは [`fn pct(value: impl Into<f64>) -> Percentage`](api:hypersig/style/value/fn.pct.html) で作ります。どちらの型も `Display` を実装しており、スタイルシートに書かれるとおりの値を書き出します。",
            },
        ),
        show(Snippet::StyleLengths),
        para(
            site,
            L {
                en: "A length of zero is written `0`, which CSS accepts without a unit, and [`Length::ZERO`](api:hypersig/style/value/struct.Length.html#associatedconstant.ZERO) is the same value. A percentage of zero keeps its `%`.",
                ja: "長さのゼロは `0` と書かれます。CSS は単位のない `0` を長さとして受け付けます。[`Length::ZERO`](api:hypersig/style/value/struct.Length.html#associatedconstant.ZERO) も同じ値です。パーセンテージのゼロには `%` が残ります。",
            },
        ),
    ])
}

fn units(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The units",
                ja: "単位",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "CSS", "One unit is"],
                ja: &["関数", "CSS", "1 単位の大きさ"],
            },
            L {
                en: &[
                    &["`px(1)`", "`1px`", "One CSS pixel"],
                    &["`rem(1)`", "`1rem`", "The root element's font size"],
                    &["`em(1)`", "`1em`", "The element's font size"],
                    &["`ch(1)`", "`1ch`", "The width of `0` in the element's font"],
                    &["`ex(1)`", "`1ex`", "The x-height of the element's font"],
                    &["`lh(1)`", "`1lh`", "The element's line height"],
                    &[
                        "`vw(1)`, `vh(1)`",
                        "`1vw`, `1vh`",
                        "1% of the viewport's width or height",
                    ],
                    &[
                        "`vmin(1)`, `vmax(1)`",
                        "`1vmin`, `1vmax`",
                        "1% of the viewport's smaller or larger side",
                    ],
                    &[
                        "`svh(1)`, `lvh(1)`, `dvh(1)`",
                        "`1svh`, `1lvh`, `1dvh`",
                        "1% of the viewport's height with the browser's toolbars shown, hidden, or as they are now",
                    ],
                    &[
                        "`cqw(1)`, `cqh(1)`",
                        "`1cqw`, `1cqh`",
                        "1% of the query container's width or height",
                    ],
                    &[
                        "`cm(1)`, `mm(1)`, `pt(1)`",
                        "`1cm`, `1mm`, `1pt`",
                        "A centimeter, a millimeter, or a point",
                    ],
                    &[
                        "`pct(1)`",
                        "`1%`",
                        "1% of what the property measures against",
                    ],
                ],
                ja: &[
                    &["`px(1)`", "`1px`", "CSS の 1 ピクセル"],
                    &["`rem(1)`", "`1rem`", "ルート要素のフォントサイズ"],
                    &["`em(1)`", "`1em`", "その要素のフォントサイズ"],
                    &["`ch(1)`", "`1ch`", "その要素のフォントでの `0` の幅"],
                    &["`ex(1)`", "`1ex`", "その要素のフォントの x ハイト"],
                    &["`lh(1)`", "`1lh`", "その要素の行の高さ"],
                    &[
                        "`vw(1)`、`vh(1)`",
                        "`1vw`、`1vh`",
                        "ビューポートの幅または高さの 1%",
                    ],
                    &[
                        "`vmin(1)`、`vmax(1)`",
                        "`1vmin`、`1vmax`",
                        "ビューポートの短い辺または長い辺の 1%",
                    ],
                    &[
                        "`svh(1)`、`lvh(1)`、`dvh(1)`",
                        "`1svh`、`1lvh`、`1dvh`",
                        "ブラウザのツールバーを表示したとき、隠したとき、現在の状態でのビューポートの高さの 1%",
                    ],
                    &[
                        "`cqw(1)`、`cqh(1)`",
                        "`1cqw`、`1cqh`",
                        "クエリコンテナの幅または高さの 1%",
                    ],
                    &[
                        "`cm(1)`、`mm(1)`、`pt(1)`",
                        "`1cm`、`1mm`、`1pt`",
                        "1 センチメートル、1 ミリメートル、1 ポイント",
                    ],
                    &["`pct(1)`", "`1%`", "プロパティが基準にする大きさの 1%"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`cqw` and `cqh` measure the nearest ancestor that is a query container, which [Container queries](style/containers/) covers. A unit with no function here, such as `vb` or `Q`, goes through `raw`.",
                ja: "`cqw` と `cqh` が測るのは、クエリコンテナである最も近い祖先です。クエリコンテナは[コンテナクエリ](style/containers/)で説明します。`vb` や `Q` のようにここに関数のない単位は `raw` で書きます。",
            },
        ),
    ])
}

fn accepted(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Which properties take which",
                ja: "どのプロパティが何を受け取るか",
            },
        ),
        para(
            site,
            L {
                en: "Whether a property takes a length, a percentage, or both is decided by its value type, and the type's rustdoc lists its `From` implementations. `Width` converts from both. `BorderWidth` converts from a length only, because CSS has no percentage border width, so `pct` there is E0277:",
                ja: "プロパティが長さを取るか、パーセンテージを取るか、その両方かは値の型で決まり、その型の rustdoc に `From` の実装が並んでいます。`Width` はどちらからも変換できます。`BorderWidth` は長さからしか変換できません。CSS に割合で指定する枠線の幅がないからで、そこに `pct` を渡すと E0277 になります。",
            },
        ),
        show(Snippet::StyleBorderPct),
        para(
            site,
            L {
                en: "A bare number is not a length. CSS needs a unit on every length but zero, and `Width` does not convert from an integer, so `.width(240)` is E0277. Write `px(240)`:",
                ja: "単位のない数値は長さではありません。CSS ではゼロ以外の長さには単位が必要で、`Width` は整数から変換できないので、`.width(240)` は E0277 になります。`px(240)` と書きます。",
            },
        ),
        show(Snippet::StyleLengthBare),
        para(
            site,
            L {
                en: "A few types that take a length also take a plain number, with a meaning of its own. `line-height: 2` is twice the element's font size, not 2px, so `LineHeight` takes a multiplier only as an `f64`, and `line_height(1.5)` writes `line-height: 1.5`. An integer such as `line_height(16)` is E0277, and the error lists the conversions `LineHeight` has. Write `px(16)` for a length or `1.0` for a multiplier:",
                ja: "長さを取る型の中には、単位のない数値も別の意味で受け取るものがあります。`line-height: 2` は 2px ではなく要素のフォントサイズの 2 倍なので、`LineHeight` は倍率を `f64` でしか受け取りません。`line_height(1.5)` は `line-height: 1.5` を書きます。`line_height(16)` のように整数を渡すと E0277 になり、エラーには `LineHeight` が変換できる型が並びます。長さなら `px(16)`、倍率なら `1.0` と書きます。",
            },
        ),
        show(Snippet::StyleLineHeightInteger),
    ])
}

fn math(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Math functions",
                ja: "数学関数",
            },
        ),
        para(
            site,
            L {
                en: "[`clamp`](api:hypersig/style/value/fn.clamp.html), [`min`](api:hypersig/style/value/fn.min.html), and [`max`](api:hypersig/style/value/fn.max.html) write the CSS functions of the same names. `+` and `-` between two values, and `*` and `/` by an `f64`, write a `calc()`. Each takes values of one type and returns that type, so the result goes wherever its arguments would. The types are those that implement [`Quantity`](api:hypersig/style/value/trait.Quantity.html): `Length`, `Percentage`, `Time`, and `Angle`.",
                ja: "[`clamp`](api:hypersig/style/value/fn.clamp.html)、[`min`](api:hypersig/style/value/fn.min.html)、[`max`](api:hypersig/style/value/fn.max.html) は同じ名前の CSS の関数を書きます。2 つの値の `+` と `-`、`f64` との `*` と `/` は `calc()` を書きます。どれも 1 つの型の値を受け取ってその型を返すので、結果は引数を渡せる場所ならどこにでも渡せます。使える型は [`Quantity`](api:hypersig/style/value/trait.Quantity.html) を実装する `Length`、`Percentage`、`Time`、`Angle` です。",
            },
        ),
        show(Snippet::StyleLengthMath),
        para(
            site,
            L {
                en: "A length and a percentage are different types, so `pct(100) - rem(2)` is E0308. A property such as `scroll_margin_top` takes only a length, and the browser would drop a percentage there. Where a property takes both, write the mixed `calc()` with `raw`:",
                ja: "長さとパーセンテージは別の型なので、`pct(100) - rem(2)` は E0308 になります。`scroll_margin_top` のように長さしか受け取らないプロパティでは、パーセンテージを含む値をブラウザが捨ててしまうからです。両方を受け取るプロパティで混ぜた `calc()` を書くときは `raw` を使います。",
            },
        ),
        show(Snippet::StyleLengthMathMixed),
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
                en: "The sign is not checked. A negative length is right for `margin_top` or `letter_spacing`, and wrong for `width`, where the browser drops the declaration and the element renders as if `width` were not set.",
                ja: "符号は検査されません。負の長さは `margin_top` や `letter_spacing` には正しい値ですが、`width` には誤りです。ブラウザはその宣言を捨て、要素は `width` を設定しなかったときと同じように描画されます。",
            },
        ),
        para(
            site,
            L {
                en: "Every unit function panics on NaN or infinity, as [Numbers](style/values/#numbers) shows. An `f32` converts into the `f64` nearest to it, which is written with all its digits:",
                ja: "単位の関数はどれも、NaN や無限大を受け取るとパニックします。これは[数値](style/values/#numbers)で示しています。`f32` は最も近い `f64` に変換され、その値がすべての桁で書かれます。",
            },
        ),
        show(Snippet::StyleLengthF32),
        para(
            site,
            L {
                en: "A literal without a suffix, such as `px(0.1)`, is an `f64` and is written as `0.1px`. A `calc()` that mixes a length and a percentage goes through `raw`, which [Unchecked values](style/unchecked/) covers, and a length shared across the page is a [token](style/tokens/), whose `var()` is a `Length`.",
                ja: "`px(0.1)` のように接尾辞のないリテラルは `f64` なので、`0.1px` と書かれます。長さとパーセンテージを混ぜた `calc()` は `raw` で書きます。`raw` は[検査しない値](style/unchecked/)で説明します。ページ全体で共有する長さは[トークン](style/tokens/)にすると、その `var()` が `Length` になります。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a reading column",
                ja: "例: 読みやすい段組み",
            },
        ),
        para(
            site,
            L {
                en: "The column is as wide as the screen up to 65 characters of its font, at least as tall as the viewport, and centered with `margin-inline: auto`. The paragraph's top margin is one line of its own text:",
                ja: "この段は画面の幅いっぱいに広がりますが、そのフォントで 65 文字分を超えません。高さは少なくともビューポートと同じで、`margin-inline: auto` で中央に置かれます。段落の上の余白はその段落の文字 1 行分です。",
            },
        ),
        show(Snippet::StyleArticle),
        para(
            site,
            L {
                en: "Next, [Colors](style/colors/) covers the `Color` type.",
                ja: "次の[色](style/colors/)では `Color` 型を説明します。",
            },
        ),
    ])
}
