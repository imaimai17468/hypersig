//! The page on CSS transitions: the `transition-*` property methods and what
//! starts a transition on a hypersig page.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Style(StylePage::Transitions).title(site)),
        lead(site, Doc::Style(StylePage::Transitions).leads()),
        heading(
            site,
            L {
                en: "The transition properties",
                ja: "トランジションのプロパティ",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "Values", "Example and its CSS"],
                ja: &["メソッド", "値", "例と CSS"],
            },
            L {
                en: &[
                    &[
                        "[`transition_property`](api:hypersig/style/struct.Style.html#method.transition_property)",
                        "`TransitionProperty::All`, `TransitionProperty::None`, `raw`",
                        "`raw(\"opacity, translate\")`: `transition-property: opacity, translate`",
                    ],
                    &[
                        "[`transition_duration`](api:hypersig/style/struct.Style.html#method.transition_duration)",
                        "A `Time`",
                        "`ms(200)`: `transition-duration: 200ms`",
                    ],
                    &[
                        "[`transition_timing_function`](api:hypersig/style/struct.Style.html#method.transition_timing_function)",
                        "`Ease`, `EaseIn`, `EaseOut`, `EaseInOut`, `Linear`, `StepStart`, `StepEnd`",
                        "`TransitionTimingFunction::EaseOut`: `transition-timing-function: ease-out`",
                    ],
                    &[
                        "[`transition_delay`](api:hypersig/style/struct.Style.html#method.transition_delay)",
                        "A `Time`",
                        "`ms(50)`: `transition-delay: 50ms`",
                    ],
                    &[
                        "[`transition_behavior`](api:hypersig/style/struct.Style.html#method.transition_behavior)",
                        "`Normal`, `AllowDiscrete`",
                        "`TransitionBehavior::AllowDiscrete`: `transition-behavior: allow-discrete`",
                    ],
                ],
                ja: &[
                    &[
                        "[`transition_property`](api:hypersig/style/struct.Style.html#method.transition_property)",
                        "`TransitionProperty::All`、`TransitionProperty::None`、`raw`",
                        "`raw(\"opacity, translate\")`：`transition-property: opacity, translate`",
                    ],
                    &[
                        "[`transition_duration`](api:hypersig/style/struct.Style.html#method.transition_duration)",
                        "`Time`",
                        "`ms(200)`：`transition-duration: 200ms`",
                    ],
                    &[
                        "[`transition_timing_function`](api:hypersig/style/struct.Style.html#method.transition_timing_function)",
                        "`Ease`、`EaseIn`、`EaseOut`、`EaseInOut`、`Linear`、`StepStart`、`StepEnd`",
                        "`TransitionTimingFunction::EaseOut`：`transition-timing-function: ease-out`",
                    ],
                    &[
                        "[`transition_delay`](api:hypersig/style/struct.Style.html#method.transition_delay)",
                        "`Time`",
                        "`ms(50)`：`transition-delay: 50ms`",
                    ],
                    &[
                        "[`transition_behavior`](api:hypersig/style/struct.Style.html#method.transition_behavior)",
                        "`Normal`、`AllowDiscrete`",
                        "`TransitionBehavior::AllowDiscrete`：`transition-behavior: allow-discrete`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each method has the shape `fn transition_duration(self, value: impl Into<TransitionDuration>) -> Style`, with the value type named after the property, and each also takes `raw`. The list of properties has no type of its own: `TransitionProperty` has only `all` and `none`, so a list of named properties is `raw`, and so is a timing function such as `cubic-bezier(0.4, 0, 0.2, 1)`.",
                ja: "どのメソッドも `fn transition_duration(self, value: impl Into<TransitionDuration>) -> Style` と同じ形で、値の型はプロパティにちなんだ名前です。どのメソッドも `raw` も受け取ります。プロパティの一覧には専用の型がありません。`TransitionProperty` にあるのは `all` と `none` だけなので、プロパティを名前で並べた一覧は `raw` で書きます。`cubic-bezier(0.4, 0, 0.2, 1)` のようなタイミング関数も `raw` で書きます。",
            },
        ),
        heading(
            site,
            L {
                en: "What starts a transition",
                ja: "トランジションが始まるとき",
            },
        ),
        para(
            site,
            L {
                en: "The browser runs a transition when the value of a listed property changes on an element that stays in the document. On a hypersig page that is a condition of `Style::on` that starts or stops matching, such as `PseudoClass::Hover`, or a class of `El::css_when` or `El::css_match` that is added or removed as a signal changes. The declarations of the transition go on the element's own style, so they apply in both directions:",
                ja: "ブラウザは、文書に残っている要素で、一覧にあるプロパティの値が変わったときにトランジションを実行します。hypersig のページでは、`PseudoClass::Hover` のような `Style::on` の条件が当てはまり始めたときや当てはまらなくなったとき、またはシグナルが変わって `El::css_when` や `El::css_match` のクラスが付いたり外れたりしたときです。トランジションの宣言は要素自身のスタイルに書くので、どちらの向きの変化にも適用されます。",
            },
        ),
        show(Snippet::Transitions),
        para(
            site,
            L {
                en: "The panel uses `visibility` with the other two: `visibility: hidden` keeps the closed panel from taking clicks and focus, and in a transition the browser keeps it visible until the fade-out ends.",
                ja: "パネルではほかの 2 つと一緒に `visibility` も使います。`visibility: hidden` により、閉じたパネルはクリックもフォーカスも受け取りません。トランジションの間は、フェードアウトが終わるまでブラウザがパネルを表示したままにします。",
            },
        ),
        heading(
            site,
            L {
                en: "The shorthand and the longhands",
                ja: "一括指定と個別のプロパティ",
            },
        ),
        para(
            site,
            L {
                en: "[`transition`](api:hypersig/style/struct.Style.html#method.transition) is the shorthand, and takes a `Time` or `raw`, as in `transition(raw(\"opacity 150ms ease\"))`. Its rule is written before the rules of the longhands, so on one element `transition_duration(ms(300))` overrides the duration in the shorthand whichever method is called first.",
                ja: "[`transition`](api:hypersig/style/struct.Style.html#method.transition) は一括指定のプロパティで、`transition(raw(\"opacity 150ms ease\"))` のように `Time` か `raw` を受け取ります。そのルールは個別のプロパティのルールより前に書かれます。そのため 1 つの要素では、どちらのメソッドを先に呼んでも`transition_duration(ms(300))` が一括指定の中の時間を上書きします。",
            },
        ),
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
                en: "A duration is a `Time` built with `ms` or `secs`, and a bare number has no unit:",
                ja: "時間は `ms` か `secs` で作る `Time` で、ただの数には単位がありません。",
            },
        ),
        show(Snippet::TransitionDuration),
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型が検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The names in a `raw` list. A misspelled name such as `raw(\"opactiy\")` is valid CSS that matches no property, so the change happens at once.",
                    ja: "`raw` の一覧の中の名前。`raw(\"opactiy\")` のように綴りを間違えた名前は、どのプロパティにも当てはまらない正しい CSS なので、値はすぐに切り替わります。",
                },
                L {
                    en: "`TransitionProperty::All`. It also animates every other property that changes on the element, including ones that a later change of the page sets for other reasons.",
                    ja: "`TransitionProperty::All`。要素で変わるほかのプロパティも、ページの後の変更が別の理由で設定するものまで含めて、すべてアニメーションします。",
                },
                L {
                    en: "An element that enters the page. An element added by a patch, or shown by `El::show`, has no earlier value to start from, so it appears at its final value. The CSS rule `@starting-style` gives it a starting value, and `Style` has no method for it, so it goes in a [`Stylesheet`](style/unchecked/).",
                    ja: "ページに入ってくる要素。パッチで追加された要素や `El::show` で表示された要素には始まりとなる前の値がないので、最後の値で現れます。始まりの値は CSS の `@starting-style` で指定でき、`Style` にはそのためのメソッドがないので、[`Stylesheet`](style/unchecked/) に書きます。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Next, [Page transitions](style/page-transitions/) animates the change from one page to the next with the browser's view transitions.",
                ja: "次の[ページ遷移](style/page-transitions/)ではブラウザの View Transitions で、あるページから次のページへの切り替わりをアニメーションさせます。",
            },
        ),
    ])
}
