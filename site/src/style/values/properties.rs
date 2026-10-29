//! The Style page on the property methods of `Style`: the shapes of their
//! signatures, the value enums, numbers, shorthands, and `Property`.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Values).leads()),
        methods(site),
        enums(site),
        numbers(site),
        shorthands(site),
        raw_values(site),
        property(site),
        errors(site),
        example(site),
    ])
}

fn methods(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "One method per property",
                ja: "プロパティごとに 1 つのメソッド",
            },
        ),
        para(
            site,
            L {
                en: "Each method of [`Style`](api:hypersig/style/struct.Style.html) is named after its property in snake case, so `font-size` is `font_size`, and links to the property's page on MDN. Every method takes `self` and a value, and the type of the value decides what the method accepts. The value types are in [`hypersig::style::value`](api:hypersig/style/value/index.html). The signatures come in four shapes:",
                ja: "[`Style`](api:hypersig/style/struct.Style.html) のメソッドはプロパティ名をスネークケースにした名前で、`font-size` なら `font_size` です。各メソッドのドキュメントからは MDN のプロパティのページへリンクしています。メソッドはどれも `self` と値を 1 つ受け取り、値の型によって受け付けるものが決まります。値の型は [`hypersig::style::value`](api:hypersig/style/value/index.html) にあります。シグネチャには 4 つの形があります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Signature", "Call", "CSS"],
                ja: &["シグネチャ", "呼び出し", "CSS"],
            },
            L {
                en: &[
                    &[
                        "`fn display(self, value: impl Into<Display>) -> Self`",
                        "`.display(Display::Flex)`",
                        "`display: flex`",
                    ],
                    &[
                        "`fn width(self, value: impl Into<Width>) -> Self`",
                        "`.width(pct(50))`",
                        "`width: 50%`",
                    ],
                    &[
                        "`fn color(self, value: impl Into<Color>) -> Self`",
                        "`.color(Color::CRIMSON)`",
                        "`color: crimson`",
                    ],
                    &[
                        "`fn order(self, value: impl Into<i32>) -> Self`",
                        "`.order(2)`",
                        "`order: 2`",
                    ],
                ],
                ja: &[
                    &[
                        "`fn display(self, value: impl Into<Display>) -> Self`",
                        "`.display(Display::Flex)`",
                        "`display: flex`",
                    ],
                    &[
                        "`fn width(self, value: impl Into<Width>) -> Self`",
                        "`.width(pct(50))`",
                        "`width: 50%`",
                    ],
                    &[
                        "`fn color(self, value: impl Into<Color>) -> Self`",
                        "`.color(Color::CRIMSON)`",
                        "`color: crimson`",
                    ],
                    &[
                        "`fn order(self, value: impl Into<i32>) -> Self`",
                        "`.order(2)`",
                        "`order: 2`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A property with keywords of its own takes an enum named after it, such as `Display` or `Width`. A property whose value is only a color, a length, or a number takes that type, as `color`, `outline_offset`, and `order` do. The sample below uses one value of each kind:",
                ja: "独自のキーワードを持つプロパティは、`Display` や `Width` のようにプロパティにちなんだ名前の列挙型を受け取ります。値が色、長さ、数値のどれかだけのプロパティは、`color`、`outline_offset`、`order` のようにその型を受け取ります。下のサンプルでは種類ごとに 1 つずつ値を使っています。",
            },
        ),
        show(Snippet::StyleValues),
    ])
}

fn enums(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The value enums",
                ja: "値の列挙型",
            },
        ),
        para(
            site,
            L {
                en: "The variants of a value enum are the property's keywords in upper camel case: `Display::InlineFlex` is `inline-flex`, and `Width::FitContent` is `fit-content`. A variant with a field holds a value of another type, as `Width::Length(Length)` holds a length. The enum implements `From` for each of those types, so `.width(pct(50))` needs no `Width::Percentage(...)` around the percentage.",
                ja: "値の列挙型のバリアントはプロパティのキーワードをアッパーキャメルケースにしたものです。`Display::InlineFlex` は `inline-flex`、`Width::FitContent` は `fit-content` です。フィールドを持つバリアントは、`Width::Length(Length)` が長さを持つように、別の型の値を持ちます。列挙型はそれらの型それぞれについて `From` を実装しているので、`.width(pct(50))` のようにパーセンテージを `Width::Percentage(...)` で包まずに渡せます。",
            },
        ),
        para(
            site,
            L {
                en: "The rustdoc page of each enum lists its variants and its `From` implementations, which together are everything the method accepts. [`Width`](api:hypersig/style/value/enum.Width.html), for example, has `Auto`, `FitContent`, `MaxContent`, `MinContent`, a length, a percentage, and `Raw`.",
                ja: "各列挙型の rustdoc のページには、バリアントと `From` の実装が載っており、それがメソッドの受け付けるもののすべてです。たとえば [`Width`](api:hypersig/style/value/enum.Width.html) には `Auto`、`FitContent`、`MaxContent`、`MinContent`、長さ、パーセンテージ、`Raw` があります。",
            },
        ),
    ])
}

fn numbers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Numbers",
                ja: "数値",
            },
        ),
        para(
            site,
            L {
                en: "A property that takes an integer, such as `order` or `z-index`, takes an `i32`. One that takes any number, such as `flex-grow`, takes `impl Into<f64>`, so an integer literal works too. A value enum with a `Number` variant, such as `FontWeight` or `Opacity`, converts from `f64` and `i32`. `LineHeight` converts from `f64` only, because an integer there reads as a length in pixels, as [Lengths and units](style/lengths/) shows:",
                ja: "`order` や `z-index` のように整数を取るプロパティは `i32` を受け取ります。`flex-grow` のように任意の数値を取るプロパティは `impl Into<f64>` を受け取るので、整数のリテラルも渡せます。`FontWeight` や `Opacity` のように `Number` バリアントを持つ値の列挙型は、`f64` と `i32` から変換できます。`LineHeight` に渡す整数はピクセル単位の長さに見えるので、`LineHeight` は `f64` からしか変換できません。詳しくは[長さと単位](style/lengths/)で説明しています。",
            },
        ),
        show(Snippet::StyleNumbers),
        para(
            site,
            L {
                en: "CSS has no NaN or infinity. A method given one panics at the call, in debug and release builds alike, so a ratio computed from empty data needs a check before it reaches the style:",
                ja: "CSS には NaN も無限大もありません。これらを受け取ったメソッドは、デバッグビルドでもリリースビルドでも、その呼び出しでパニックします。空のデータから計算した比率などは、スタイルに渡す前に確かめる必要があります。",
            },
        ),
        show(Snippet::StyleNumberNan),
        para(
            site,
            L {
                en: "Ranges are not checked. `opacity(5)` and `font_weight(-1)` compile, and the browser handles a value out of range as CSS defines for that property. A negative width, for example, is dropped, and the element renders as if `width` were not set.",
                ja: "値の範囲は検査されません。`opacity(5)` も `font_weight(-1)` もコンパイルが通り、範囲外の値をブラウザがどう扱うかは、プロパティごとに CSS が定めています。たとえば負の幅は捨てられ、要素は `width` を設定しなかったときと同じように描画されます。",
            },
        ),
    ])
}

fn shorthands(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Shorthands",
                ja: "ショートハンド",
            },
        ),
        para(
            site,
            L {
                en: "A shorthand such as `margin`, `padding`, or `border` has a method like any other property, and its typed forms are the single values: `margin(Margin::Auto)` writes `margin: auto`. A value with several parts goes through `raw`, or is written as typed longhands:",
                ja: "`margin`、`padding`、`border` のようなショートハンドにもほかのプロパティと同じくメソッドがあり、型が付くのは値が 1 つの形です。`margin(Margin::Auto)` は `margin: auto` を書きます。複数の部分からなる値は `raw` で書くか、型の付いたロングハンドに分けて書きます。",
            },
        ),
        show(Snippet::StyleShorthands),
        para(
            site,
            L {
                en: "When a style sets a shorthand and one of its longhands, the longhand wins whatever the order of the calls, as [Which declaration wins](style/applying/#which-declaration-wins) shows.",
                ja: "1 つのスタイルでショートハンドとそのロングハンドの両方を設定すると、呼び出しの順序にかかわらずロングハンドが勝ちます。[どの宣言が勝つか](style/applying/#which-declaration-wins)で示しています。",
            },
        ),
    ])
}

fn raw_values(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Values written with raw",
                ja: "raw で書く値",
            },
        ),
        para(
            site,
            L {
                en: "Every value enum ends in a `Raw(Raw)` variant, so it also accepts [`raw(css)`](api:hypersig/style/value/fn.raw.html), which the browser parses and nothing checks. Each method that takes an enum says so in its rustdoc: \"The single-value forms are typed. Any other value goes through `Raw`\". `Color` and `Length` convert from `Raw` too, so the methods that take one directly, such as `color` and `scroll_margin_top`, accept `raw` as well. A `calc()` of numbers, or one that mixes a length and a percentage, has no typed form and goes through `raw`:",
                ja: "値の列挙型はどれも最後に `Raw(Raw)` バリアントを持つので、[`raw(css)`](api:hypersig/style/value/fn.raw.html) も受け付けます。その値はブラウザが解析し、何も検査しません。列挙型を受け取るメソッドの rustdoc には「The single-value forms are typed. Any other value goes through `Raw`」と書かれています。`Color` と `Length` も `Raw` から変換できるので、`color` や `scroll_margin_top` のようにそれらを直接受け取るメソッドにも `raw` を渡せます。数値どうしの `calc()` や、長さとパーセンテージを混ぜた `calc()` には型の付いた書き方がないので、`raw` で書きます。",
            },
        ),
        show(Snippet::StyleRawValues),
        para(
            site,
            L {
                en: "A `calc()`, `min()`, `max()`, or `clamp()` over values of one type is typed, as [Lengths](style/lengths/#math-functions) shows. The methods that take a bare number, such as `flex_grow`, `order`, and `widows`, take no `raw`.",
                ja: "1 つの型の値だけを使う `calc()`、`min()`、`max()`、`clamp()` には型が付きます。これは[長さ](style/lengths/#math-functions)で示しています。`flex_grow`、`order`、`widows` のように数値をそのまま受け取るメソッドは `raw` を受け取りません。",
            },
        ),
    ])
}

fn property(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The Property enum",
                ja: "Property 列挙型",
            },
        ),
        para(
            site,
            L {
                en: "[`Property`](api:hypersig/style/enum.Property.html) has one variant per property method, such as `Property::FontSize` for `font_size`, and `Property::name` returns the CSS name, `\"font-size\"`. It is the argument where a property is passed as a value: [`Style::keyword`](style/keywords/) sets a property to `inherit` or another CSS-wide keyword, and `El::bind_style` sets one from a signal.",
                ja: "[`Property`](api:hypersig/style/enum.Property.html) にはプロパティのメソッドごとに 1 つのバリアントがあり、`font_size` には `Property::FontSize` が対応します。`Property::name` は CSS での名前 `\"font-size\"` を返します。`Property` は、プロパティを値として渡す場所の引数になります。[`Style::keyword`](style/keywords/) はプロパティを `inherit` などの CSS 全体のキーワードに設定し、`El::bind_style` はシグナルからプロパティを設定します。",
            },
        ),
    ])
}

fn errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What does not compile",
                ja: "コンパイルできないもの",
            },
        ),
        para(
            site,
            L {
                en: "A misspelled property has no method, which is E0599:",
                ja: "綴りを間違えたプロパティにはメソッドがないので、E0599 になります。",
            },
        ),
        show(Snippet::StyleMethodTypo),
        para(
            site,
            L {
                en: "A misspelled keyword has no variant, which is E0599 too:",
                ja: "綴りを間違えたキーワードにはバリアントがないので、これも E0599 になります。",
            },
        ),
        show(Snippet::StyleKeyword),
        para(
            site,
            L {
                en: "A value of the wrong type has no `From` conversion into the property's type, which is E0277. The help under the error lists the `From` implementations of that type, which are the values the method accepts:",
                ja: "型の違う値には、プロパティの型への `From` の変換がないので、E0277 になります。エラーの下の help には、その型の `From` の実装、つまりメソッドが受け付ける値が並びます。",
            },
        ),
        show(Snippet::WrongValue),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a notice",
                ja: "例: お知らせ",
            },
        ),
        para(
            site,
            L {
                en: "The notice uses keywords for its layout, lengths for its spacing and its left bar, colors for the bar and the fill, and a number for its line height:",
                ja: "このお知らせはレイアウトにキーワード、余白と左の帯に長さ、帯と背景に色、行の高さに数値を使っています。",
            },
        ),
        show(Snippet::StyleNotice),
        para(
            site,
            L {
                en: "Next, [Lengths and units](style/lengths/) covers `px`, `rem`, `pct`, and the other units.",
                ja: "次の[長さと単位](style/lengths/)では `px`、`rem`、`pct` などの単位を説明します。",
            },
        ),
    ])
}
