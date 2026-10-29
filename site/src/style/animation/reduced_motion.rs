//! The page on `Media::ReducedMotion` and the animations it changes.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Style(StylePage::ReducedMotion).title(site)),
        lead(site, Doc::Style(StylePage::ReducedMotion).leads()),
        heading(
            site,
            L {
                en: "The condition",
                ja: "条件",
            },
        ),
        para(
            site,
            L {
                en: "[`Media::ReducedMotion`](api:hypersig/style/enum.Media.html#variant.ReducedMotion) is the media query `(prefers-reduced-motion: reduce)`. The browser matches it when the visitor turns on the reduce-motion setting of the operating system, and Chrome's DevTools can emulate it in the Rendering panel. `Style::on(Media::ReducedMotion, style)` writes the declarations of `style` inside `@media (prefers-reduced-motion: reduce) { ... }`, and those rules come after the element's base rules, so they override them for that visitor.",
                ja: "[`Media::ReducedMotion`](api:hypersig/style/enum.Media.html#variant.ReducedMotion) はメディアクエリ `(prefers-reduced-motion: reduce)` です。訪問者が OS の動きを減らす設定をオンにすると、ブラウザはこの条件が成り立つものとして扱います。Chrome の DevTools では Rendering パネルでこの設定を再現できます。`Style::on(Media::ReducedMotion, style)` は `style` の宣言を `@media (prefers-reduced-motion: reduce) { ... }` の中に書きます。これらのルールは要素の基本のルールより後に来るので、その訪問者に対しては基本のルールを上書きします。",
            },
        ),
        para(
            site,
            L {
                en: "`Media` has no variant for `no-preference`. Write the motion in the element's base style, and change it under `Media::ReducedMotion`. A variant spelled after the CSS name does not exist:",
                ja: "`Media` には `no-preference` に当たるバリアントがありません。動きは要素の基本のスタイルに書き、`Media::ReducedMotion` の下で変えます。CSS の名前をもとに綴ったバリアントは存在しません。",
            },
        ),
        show(Snippet::ReducedMotionVariant),
        heading(
            site,
            L {
                en: "Ways to reduce motion",
                ja: "動きを減らす方法",
            },
        ),
        table_of(
            site,
            L {
                en: &[
                    "Under `Media::ReducedMotion`",
                    "The CSS",
                    "What the visitor sees",
                ],
                ja: &["`Media::ReducedMotion` の下", "CSS", "訪問者に見えるもの"],
            },
            L {
                en: &[
                    &[
                        "`animation_iteration_count(AnimationIterationCount::Number(0.0))`",
                        "`animation-iteration-count: 0`",
                        "The animation does not run, and the element shows its own style",
                    ],
                    &[
                        "`animation_name(AnimationName::None)`",
                        "`animation-name: none`",
                        "No animation is applied, and the element appears in place",
                    ],
                    &[
                        "`transition_duration(ms(0))`",
                        "`transition-duration: 0ms`",
                        "The new value applies at once",
                    ],
                    &[
                        "`animation_duration(ms(4000))`",
                        "`animation-duration: 4000ms`",
                        "A spinner that still turns, five times slower than at 800ms",
                    ],
                ],
                ja: &[
                    &[
                        "`animation_iteration_count(AnimationIterationCount::Number(0.0))`",
                        "`animation-iteration-count: 0`",
                        "アニメーションは実行されず、要素は自身のスタイルで表示されます",
                    ],
                    &[
                        "`animation_name(AnimationName::None)`",
                        "`animation-name: none`",
                        "アニメーションは適用されず、要素は最初から定位置に表示されます",
                    ],
                    &[
                        "`transition_duration(ms(0))`",
                        "`transition-duration: 0ms`",
                        "新しい値がすぐに適用されます",
                    ],
                    &[
                        "`animation_duration(ms(4000))`",
                        "`animation-duration: 4000ms`",
                        "スピナーは回り続けますが、800ms のときの 5 分の 1 の速さになります",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The first three remove the motion. The last one keeps a spinner turning, because it tells the visitor that the page is still working, and slows it down. The sample below uses the first three:",
                ja: "最初の 3 つは動きをなくします。最後の 1 つは、ページがまだ処理中であることを訪問者に伝えるためにスピナーを回したまま、速さを落とします。下のサンプルは最初の 3 つを使います。",
            },
        ),
        show(Snippet::ReducedMotion),
        para(
            site,
            L {
                en: "The components of `hypersig_ui` do the same: their color transitions take 0ms, the skeleton stops pulsing, and the spinner turns four times slower.",
                ja: "`hypersig_ui` のコンポーネントも同じようにしています。色のトランジションは 0ms になり、スケルトンは点滅を止め、スピナーは 4 分の 1 の速さで回ります。",
            },
        ),
        heading(
            site,
            L {
                en: "View transitions",
                ja: "View Transitions",
            },
        ),
        para(
            site,
            L {
                en: "The browser plays [view transitions](style/page-transitions/) under reduced motion too. Their rules are in a `Stylesheet`, so the condition is written in CSS there: put the animations under `@media (prefers-reduced-motion: no-preference)`, and set `animation: none` on `::view-transition-group(*)`, `::view-transition-old(*)`, and `::view-transition-new(*)` under `@media (prefers-reduced-motion: reduce)`. Without the second rule, a visitor who asks for reduced motion still sees the browser's default cross-fade. The stylesheet on [Page transitions](style/page-transitions/) does both, and so does this site.",
                ja: "ブラウザは動きを減らす設定のときも [View Transitions](style/page-transitions/) を再生します。そのルールは `Stylesheet` に書くので、条件もそこに CSS で書きます。アニメーションは `@media (prefers-reduced-motion: no-preference)` の中に書き、`@media (prefers-reduced-motion: reduce)` の中で `::view-transition-group(*)`、`::view-transition-old(*)`、`::view-transition-new(*)` に `animation: none` を指定します。後者がないと、動きを減らすよう求める訪問者にもブラウザ既定のクロスフェードが表示されます。[ページ遷移](style/page-transitions/)のスタイルシートは両方を書いており、このサイトも同じです。",
            },
        ),
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
                    en: "Whether an animation has a reduced form. Nothing checks that a style with `keyframes` or a transition also has a style under `Media::ReducedMotion`. Without one, the visitor sees the full motion.",
                    ja: "アニメーションに動きを減らした形があるか。`keyframes` やトランジションを持つスタイルに、`Media::ReducedMotion` の下のスタイルもあるかは検査されません。ない場合、訪問者には元の動きがそのまま表示されます。",
                },
                L {
                    en: "The motion in a `Stylesheet` or in `raw`. A `@media` written in the CSS text of a `Stylesheet` is not checked, and a misspelled feature such as `prefers-reduce-motion` never matches.",
                    ja: "`Stylesheet` や `raw` の中の動き。`Stylesheet` の CSS のテキストに書いた `@media` は検査されず、`prefers-reduce-motion` のように綴りを間違えた特性は一度も当てはまりません。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Next, [Unchecked values](style/unchecked/) lists every place where a string reaches the CSS without a check.",
                ja: "次の[検査しない値](style/unchecked/)では、文字列が検査なしで CSS に入るすべての場所を挙げます。",
            },
        ),
    ])
}
