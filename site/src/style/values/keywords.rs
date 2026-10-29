//! The Style page on the CSS-wide keywords: `CssWideKeyword` and
//! `Style::keyword`.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Keywords).leads()),
        keywords(site),
        setting(site),
        errors(site),
        reset(site),
        example(site),
    ])
}

fn keywords(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The keywords",
                ja: "キーワード",
            },
        ),
        para(
            site,
            L {
                en: "CSS defines five keywords that every property accepts. [`CssWideKeyword`](api:hypersig/style/value/enum.CssWideKeyword.html) has one variant for each:",
                ja: "CSS には、どのプロパティでも使える 5 つのキーワードがあります。[`CssWideKeyword`](api:hypersig/style/value/enum.CssWideKeyword.html) にはそれぞれに対応するバリアントがあります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Variant", "CSS", "The property takes"],
                ja: &["バリアント", "CSS", "プロパティが取る値"],
            },
            L {
                en: &[
                    &["`Inherit`", "`inherit`", "The parent's computed value"],
                    &["`Initial`", "`initial`", "The property's initial value"],
                    &[
                        "`Unset`",
                        "`unset`",
                        "`inherit` for an inherited property, such as `color`, and `initial` for any other",
                    ],
                    &[
                        "`Revert`",
                        "`revert`",
                        "The value from the browser's or the user's stylesheet",
                    ],
                    &[
                        "`RevertLayer`",
                        "`revert-layer`",
                        "The value from the previous cascade layer",
                    ],
                ],
                ja: &[
                    &["`Inherit`", "`inherit`", "親の計算済みの値"],
                    &["`Initial`", "`initial`", "そのプロパティの初期値"],
                    &[
                        "`Unset`",
                        "`unset`",
                        "`color` のように継承されるプロパティでは `inherit`、それ以外では `initial`",
                    ],
                    &[
                        "`Revert`",
                        "`revert`",
                        "ブラウザかユーザーのスタイルシートの値",
                    ],
                    &[
                        "`RevertLayer`",
                        "`revert-layer`",
                        "1 つ前のカスケードレイヤーの値",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "For a declaration of `El::css`, the previous layers hold the rules of a [`Global`](style/global/), so `revert-layer` gives the element the value a `Global` rule sets for it, or the browser's default where no rule does.",
                ja: "`El::css` の宣言の場合、それより前のレイヤーには [`Global`](style/global/) のルールがあります。そのため `revert-layer` は、`Global` のルールがその要素に設定する値を、そうしたルールがなければブラウザの既定の値を要素に与えます。",
            },
        ),
    ])
}

fn setting(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Setting a property to a keyword",
                ja: "プロパティをキーワードに設定する",
            },
        ),
        para(
            site,
            L {
                en: "[`Style::keyword(self, property: Property, value: CssWideKeyword) -> Self`](api:hypersig/style/struct.Style.html#method.keyword) writes `property: keyword`. The property is a variant of [`Property`](style/values/#the-property-enum), so any property can take any of the five. It replaces an earlier value for the same property, as a second call to the property's method does:",
                ja: "[`Style::keyword(self, property: Property, value: CssWideKeyword) -> Self`](api:hypersig/style/struct.Style.html#method.keyword) は `property: keyword` を書きます。プロパティは [`Property`](style/values/#the-property-enum) のバリアントで指定するので、どのプロパティにも 5 つのどれでも設定できます。プロパティのメソッドを 2 回呼んだときと同じく、同じプロパティの前の値を置き換えます。",
            },
        ),
        show(Snippet::StyleKeywords),
        para(
            site,
            L {
                en: "The element gets `color: inherit` and `margin: revert`. To set a keyword under a condition, call `keyword` on the style passed to `on`.",
                ja: "要素には `color: inherit` と `margin: revert` が適用されます。条件の下でキーワードを設定するには、`on` に渡すスタイルで `keyword` を呼びます。",
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
                en: "The property methods do not take a `CssWideKeyword`, because their value types have no conversion from it, so `.color(CssWideKeyword::Inherit)` is E0277. Use `keyword` with `Property::Color`:",
                ja: "プロパティのメソッドは `CssWideKeyword` を受け取りません。値の型にそこからの変換がないからで、`.color(CssWideKeyword::Inherit)` は E0277 になります。`Property::Color` を指定して `keyword` を使います。",
            },
        ),
        show(Snippet::StyleKeywordMethod),
        para(
            site,
            L {
                en: "A misspelled `Property` variant, such as `Property::Colour`, has no variant and is E0599.",
                ja: "`Property::Colour` のように綴りを間違えた `Property` のバリアントは存在しないので、E0599 になります。",
            },
        ),
    ])
}

fn reset(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Resetting with `all`",
                ja: "`all` によるリセット",
            },
        ),
        para(
            site,
            L {
                en: "`all` is the shorthand of every property except `direction` and `unicode-bidi`, and `keyword(Property::All, CssWideKeyword::Unset)` resets them. The stylesheet writes `all` before the properties it resets, as it writes `padding` before `padding-top`, so in one style a declaration such as `color` or `accent-color` wins over `all: unset` whatever the order of the calls. The example below relies on that.",
                ja: "`all` は `direction` と `unicode-bidi` を除くすべてのプロパティのショートハンドで、`keyword(Property::All, CssWideKeyword::Unset)` はそれらを戻します。スタイルシートは `padding` を `padding-top` より前に書くのと同じように、`all` を戻す対象のプロパティより前に書きます。そのため 1 つのスタイルの中では、呼び出しの順序にかかわらず、`color` や `accent-color` などの宣言が `all: unset` に勝ちます。下の例はこの順序を前提にしています。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a button that reads as a link",
                ja: "例: リンクのように見えるボタン",
            },
        ),
        para(
            site,
            L {
                en: "`all: unset` removes the browser's button look, and `inherit` gives the button the color and the font of the sentence around it. An underline and a pointer cursor make it read as a link:",
                ja: "`all: unset` でブラウザのボタンの見た目を消し、`inherit` で周りの文と同じ色とフォントにします。下線とポインタのカーソルを付けると、リンクのように見えます。",
            },
        ),
        show(Snippet::StyleTextButton),
        para(
            site,
            L {
                en: "Next, the [Pseudo-classes](style/pseudo-classes/) page applies a style only under a condition, such as while the pointer is over the element.",
                ja: "次の[擬似クラス](style/pseudo-classes/)ではポインタが要素の上にある間など、条件の下でだけスタイルを適用する方法を説明します。",
            },
        ),
    ])
}
