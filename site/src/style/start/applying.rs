//! The Style page on building a `Style`, adding it to an element, and which
//! of two declarations for one property the browser applies.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Applying).leads()),
        building(site),
        adding(site),
        merging(site),
        moved(site),
        winning(site),
        example(site),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a style",
                ja: "スタイルを組み立てる",
            },
        ),
        para(
            site,
            L {
                en: "[`Style::new() -> Style`](api:hypersig/style/struct.Style.html#method.new) is an empty style. Each property method takes the style by value and returns it with one more declaration, as `fn color(self, value: impl Into<Color>) -> Self` does, so the calls chain. The methods are `#[must_use]`, so a call whose result goes unused is a warning.",
                ja: "[`Style::new() -> Style`](api:hypersig/style/struct.Style.html#method.new) は空のスタイルです。プロパティのメソッドは `fn color(self, value: impl Into<Color>) -> Self` のように、スタイルを値で受け取り、宣言を 1 つ加えて返すので、呼び出しをつなげて書けます。メソッドには `#[must_use]` が付いているので、結果を使わない呼び出しは警告になります。",
            },
        ),
        para(
            site,
            L {
                en: "A `Style` is an ordinary Rust value. It implements `Clone`, `Default`, `PartialEq`, and `Debug`, so a function can return one, a test can compare two, and a component can take one as an argument. [`Style::is_empty`](api:hypersig/style/struct.Style.html#method.is_empty) tells whether it has any declaration.",
                ja: "`Style` は普通の Rust の値です。`Clone`、`Default`、`PartialEq`、`Debug` を実装しているので、関数から返すことも、テストで 2 つを比べることも、コンポーネントの引数に取ることもできます。宣言があるかどうかは [`Style::is_empty`](api:hypersig/style/struct.Style.html#method.is_empty) で調べられます。",
            },
        ),
    ])
}

fn adding(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Adding a style to an element",
                ja: "要素にスタイルを付ける",
            },
        ),
        para(
            site,
            L {
                en: "[`El::css(self, style: Style) -> Self`](api:hypersig/html/struct.El.html#method.css) gives the element one class per declaration. The page writes the rule of each class once into `<head>`, however many elements use it. The three rows below share one rule for each of their four declarations:",
                ja: "[`El::css(self, style: Style) -> Self`](api:hypersig/html/struct.El.html#method.css) は、宣言 1 つごとに 1 つのクラスを要素に付けます。ページは各クラスのルールを、使う要素がいくつあっても `<head>` に 1 回だけ書きます。下の 3 つの行は 4 つの宣言のそれぞれについて 1 つのルールを共有します。",
            },
        ),
        show(Snippet::StyleOneRule),
        para(
            site,
            L {
                en: "Each `<li>` gets these declarations:",
                ja: "それぞれの `<li>` には次の宣言が適用されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "padding-block: 0.5rem;\nborder-bottom-width: 1px;\nborder-bottom-style: solid;\nborder-bottom-color: gainsboro;",
        ),
        para(
            site,
            L {
                en: "A patch that adds elements to the page carries the rules of their classes, so an element the server sends later is styled when it arrives.",
                ja: "ページに要素を追加するパッチには、その要素のクラスのルールも含まれます。そのため、サーバーが後から送った要素も、届いた時点でスタイルが付いています。",
            },
        ),
    ])
}

fn merging(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Combining two styles",
                ja: "2 つのスタイルを組み合わせる",
            },
        ),
        para(
            site,
            L {
                en: "[`Style::merge(self, style: Style) -> Self`](api:hypersig/style/struct.Style.html#method.merge) adds the declarations of another style. Where both set the same property under the same conditions, the added one replaces the original, as a later declaration in a CSS rule does. A second call to the same property method replaces the first in the same way, and so does a second `El::css` call on one element.",
                ja: "[`Style::merge(self, style: Style) -> Self`](api:hypersig/style/struct.Style.html#method.merge) は別のスタイルの宣言を加えます。同じ条件の下で両方が同じプロパティを設定している場合、CSS のルールで後に書いた宣言と同じように、加えた側が元の宣言を置き換えます。同じプロパティのメソッドを 2 回呼んだときも、1 つの要素で `El::css` を 2 回呼んだときも、同じように置き換わります。",
            },
        ),
        show(Snippet::StyleMerge),
        para(
            site,
            L {
                en: "`danger` holds five declarations, and only the background differs from `button()`:",
                ja: "`danger` には 5 つの宣言があり、`button()` と違うのは背景だけです。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "padding-inline: 1rem;\nborder-radius: 6px;\nbackground-color: #dc2626;\ncolor: white;\ncursor: pointer;",
        ),
        para(
            site,
            L {
                en: "A declaration under a condition, such as one inside `on(PseudoClass::Hover, ...)`, does not replace the declaration without it. Both stay, and the conditioned one applies while its condition holds. [Pseudo-classes](style/pseudo-classes/) covers conditions.",
                ja: "`on(PseudoClass::Hover, ...)` の中のように条件の付いた宣言は、条件のない宣言を置き換えません。両方が残り、条件の付いた方はその条件が成り立つ間だけ適用されます。条件は[擬似クラス](style/pseudo-classes/)で説明します。",
            },
        ),
    ])
}

fn moved(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A style is moved into the element",
                ja: "スタイルは要素にムーブされる",
            },
        ),
        para(
            site,
            L {
                en: "`El::css` takes the `Style` by value, so one variable can be passed only once. A second use is E0382:",
                ja: "`El::css` は `Style` を値で受け取るので、1 つの変数を渡せるのは 1 回だけです。2 回目に使うと E0382 になります。",
            },
        ),
        show(Snippet::StyleMoved),
        para(
            site,
            L {
                en: "Write the style in a function that returns a new `Style` each time, as `row()` does above, or pass `pill.clone()` to all but the last use.",
                ja: "上の `row()` のように、呼ぶたびに新しい `Style` を返す関数にスタイルを書くか、最後以外の箇所に `pill.clone()` を渡します。",
            },
        ),
    ])
}

fn winning(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Which declaration wins",
                ja: "どの宣言が勝つか",
            },
        ),
        para(
            site,
            L {
                en: "Within one style, a later call replaces an earlier one only for the same property. A shorthand and one of its longhands are different properties, so both stay, and the browser applies the longhand over the shorthand whatever the order of the calls:",
                ja: "1 つのスタイルの中で、後の呼び出しが前の呼び出しを置き換えるのは同じプロパティの場合だけです。ショートハンドとそのロングハンドは別のプロパティなので両方が残り、呼び出しの順序にかかわらず、ブラウザはロングハンドをショートハンドより優先して適用します。",
            },
        ),
        show(Snippet::StyleShorthand),
        para(
            site,
            L {
                en: "The stylesheet holds `padding: 1rem` before `padding-top: 0`, so the box has 1rem of padding on three sides and none on top. Between styles from different sources, the browser applies them in this order, each overriding the ones above it:",
                ja: "スタイルシートでは `padding: 1rem` が `padding-top: 0` より前にあるので、ボックスの余白は 3 辺が 1rem で、上はなしになります。出どころの違うスタイルの間では、ブラウザは次の順に適用し、後のものが前のものを上書きします。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Source", "Page"],
                ja: &["出どころ", "ページ"],
            },
            L {
                en: &[
                    &[
                        "The rules of a `Global`, such as the reset",
                        "[Rules by tag](style/global/)",
                    ],
                    &["The classes of `El::css`", "This page"],
                    &[
                        "The classes of `El::css_when` and `El::css_match`, while switched on",
                        "[Styles switched by a signal](style/toggled/)",
                    ],
                    &[
                        "A stylesheet added with `Page::style`",
                        "[Unchecked values](style/unchecked/)",
                    ],
                ],
                ja: &[
                    &[
                        "リセットなど、`Global` のルール",
                        "[タグ単位のルール](style/global/)",
                    ],
                    &["`El::css` のクラス", "このページ"],
                    &[
                        "有効になっている間の `El::css_when` と `El::css_match` のクラス",
                        "[シグナルで切り替えるスタイル](style/toggled/)",
                    ],
                    &[
                        "`Page::style` で追加したスタイルシート",
                        "[検査しない値](style/unchecked/)",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The order does not depend on the selectors' specificity. A rule of `Page::style` that selects by tag overrides a class of `El::css`:",
                ja: "この順序はセレクタの詳細度によりません。`Page::style` のルールはタグで選ぶものでも `El::css` のクラスを上書きします。",
            },
        ),
        show(Snippet::StylePageStyle),
        para(
            site,
            L {
                en: "Nothing compares these sources with each other. A `Page::style` rule that sets a property silently overrides every `El::css` declaration of that property on the elements it selects.",
                ja: "これらの出どころ同士は照合されません。あるプロパティを設定する `Page::style` のルールは、そのルールが選ぶ要素にある `El::css` の同じプロパティの宣言を、警告なしにすべて上書きします。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: buttons in two tones",
                ja: "例: 2 つの色調のボタン",
            },
        ),
        para(
            site,
            L {
                en: "`button_style` returns the shared base merged with what the tone changes. The `match` must cover every `Tone`, so a new tone fails to compile until it has a style. The second `css` call adds `margin-left: auto` to one button only, which pushes it to the end of the row.",
                ja: "`button_style` は共通のスタイルに、色調ごとに変わる部分を `merge` して返します。`match` はすべての `Tone` を扱う必要があるので、色調を増やすと、そのスタイルを書くまでコンパイルが通りません。2 回目の `css` の呼び出しは 1 つのボタンだけに `margin-left: auto` を加え、そのボタンを行の端に寄せます。",
            },
        ),
        show(Snippet::StyleButtons),
        para(
            site,
            L {
                en: "Next, [Properties](style/values/) covers the property methods and the value types they take.",
                ja: "次の[プロパティ](style/values/)ではプロパティのメソッドと、メソッドが受け取る値の型を説明します。",
            },
        ),
    ])
}
