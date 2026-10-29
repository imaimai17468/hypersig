//! The page on `Keyframes` and `Style::keyframes`.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Style(StylePage::Animation).title(site)),
        lead(site, Doc::Style(StylePage::Animation).leads()),
        heading(
            site,
            L {
                en: "A keyframes value",
                ja: "キーフレームの値",
            },
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it does"],
                ja: &["項目", "すること"],
            },
            L {
                en: &[
                    &[
                        "[`Keyframes::new(name: &'static str) -> Keyframes`](api:hypersig/style/struct.Keyframes.html#method.new)",
                        "Keyframes with no steps, written as `@keyframes <name>`",
                    ],
                    &[
                        "[`Keyframes::at(self, percent: u8, style: Style) -> Keyframes`](api:hypersig/style/struct.Keyframes.html#method.at)",
                        "Adds the declarations of `style` as the step `<percent>%`",
                    ],
                    &[
                        "[`Keyframes::name(&self) -> &'static str`](api:hypersig/style/struct.Keyframes.html#method.name)",
                        "The name",
                    ],
                    &[
                        "[`Style::keyframes(self, keyframes: &Keyframes) -> Style`](api:hypersig/style/struct.Style.html#method.keyframes)",
                        "Sets `animation-name: <name>`, and adds the `@keyframes` rule to the page that uses the style",
                    ],
                ],
                ja: &[
                    &[
                        "[`Keyframes::new(name: &'static str) -> Keyframes`](api:hypersig/style/struct.Keyframes.html#method.new)",
                        "段階のないキーフレームを作ります。`@keyframes <name>` として書かれます",
                    ],
                    &[
                        "[`Keyframes::at(self, percent: u8, style: Style) -> Keyframes`](api:hypersig/style/struct.Keyframes.html#method.at)",
                        "`style` の宣言を `<percent>%` の段階として追加します",
                    ],
                    &[
                        "[`Keyframes::name(&self) -> &'static str`](api:hypersig/style/struct.Keyframes.html#method.name)",
                        "名前を返します",
                    ],
                    &[
                        "[`Style::keyframes(self, keyframes: &Keyframes) -> Style`](api:hypersig/style/struct.Style.html#method.keyframes)",
                        "`animation-name: <name>` を設定し、そのスタイルを使うページに `@keyframes` のルールを加えます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A spinner turns once every 800ms. Each step is a `Style`, so a step's properties and values are checked like those of any other style, and `rotate(deg(360))` writes `rotate: 360deg`:",
                ja: "下のスピナーは 800ms ごとに 1 回転します。段階はそれぞれ `Style` なので、段階のプロパティと値もほかのスタイルと同じように検査されます。`rotate(deg(360))` は `rotate: 360deg` を書きます。",
            },
        ),
        show(Snippet::Keyframes),
        para(
            site,
            L {
                en: "The `@keyframes` rule is written only where the style is used. The page writes it into `<head>` when an element of the page uses the style through `El::css`, `El::css_when`, `El::css_match`, or `Page::css_when`, or when a [`Global`](style/global/) rule uses it. A patch whose new elements use the style brings the rule with them. A page that never shows the spinner sends no `@keyframes spin`.",
                ja: "`@keyframes` のルールはそのスタイルを使う場所にだけ書かれます。ページの要素が `El::css`、`El::css_when`、`El::css_match`、`Page::css_when` でそのスタイルを使うときと、[`Global`](style/global/) のルールがそのスタイルを使うときに、ページはそのルールを `<head>` に書きます。新しい要素がそのスタイルを使うパッチも、ルールを一緒に届けます。スピナーを一度も表示しないページは `@keyframes spin` を送りません。",
            },
        ),
        demo::show(site, Demo::Spinner),
        heading(
            site,
            L {
                en: "Timing",
                ja: "タイミング",
            },
        ),
        para(
            site,
            L {
                en: "`Style::keyframes` sets only the name. The other `animation-*` longhands are property methods with typed values, each of the shape `fn animation_duration(self, value: impl Into<AnimationDuration>) -> Style`:",
                ja: "`Style::keyframes` が設定するのは名前だけです。ほかの `animation-*` の個別プロパティは型の付いた値を受け取るプロパティのメソッドで設定します。どれも `fn animation_duration(self, value: impl Into<AnimationDuration>) -> Style` と同じ形です。",
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
                        "`animation_duration`",
                        "A `Time`, `AnimationDuration::Auto`",
                        "`ms(800)`: `animation-duration: 800ms`",
                    ],
                    &[
                        "`animation_delay`",
                        "A `Time`",
                        "`ms(150)`: `animation-delay: 150ms`",
                    ],
                    &[
                        "`animation_timing_function`",
                        "`Ease`, `EaseIn`, `EaseOut`, `EaseInOut`, `Linear`, `StepStart`, `StepEnd`",
                        "`AnimationTimingFunction::Linear`: `animation-timing-function: linear`",
                    ],
                    &[
                        "`animation_iteration_count`",
                        "`Infinite`, a number",
                        "`AnimationIterationCount::Infinite`: `animation-iteration-count: infinite`",
                    ],
                    &[
                        "`animation_direction`",
                        "`Normal`, `Reverse`, `Alternate`, `AlternateReverse`",
                        "`AnimationDirection::Alternate`: `animation-direction: alternate`",
                    ],
                    &[
                        "`animation_fill_mode`",
                        "`None`, `Forwards`, `Backwards`, `Both`",
                        "`AnimationFillMode::Backwards`: `animation-fill-mode: backwards`",
                    ],
                    &[
                        "`animation_play_state`",
                        "`Running`, `Paused`",
                        "`AnimationPlayState::Paused`: `animation-play-state: paused`",
                    ],
                ],
                ja: &[
                    &[
                        "`animation_duration`",
                        "`Time`、`AnimationDuration::Auto`",
                        "`ms(800)`：`animation-duration: 800ms`",
                    ],
                    &[
                        "`animation_delay`",
                        "`Time`",
                        "`ms(150)`：`animation-delay: 150ms`",
                    ],
                    &[
                        "`animation_timing_function`",
                        "`Ease`、`EaseIn`、`EaseOut`、`EaseInOut`、`Linear`、`StepStart`、`StepEnd`",
                        "`AnimationTimingFunction::Linear`：`animation-timing-function: linear`",
                    ],
                    &[
                        "`animation_iteration_count`",
                        "`Infinite`、数",
                        "`AnimationIterationCount::Infinite`：`animation-iteration-count: infinite`",
                    ],
                    &[
                        "`animation_direction`",
                        "`Normal`、`Reverse`、`Alternate`、`AlternateReverse`",
                        "`AnimationDirection::Alternate`：`animation-direction: alternate`",
                    ],
                    &[
                        "`animation_fill_mode`",
                        "`None`、`Forwards`、`Backwards`、`Both`",
                        "`AnimationFillMode::Backwards`：`animation-fill-mode: backwards`",
                    ],
                    &[
                        "`animation_play_state`",
                        "`Running`、`Paused`",
                        "`AnimationPlayState::Paused`：`animation-play-state: paused`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A timing function other than a keyword, such as `cubic-bezier(0.16, 1, 0.3, 1)`, goes through `raw`. A step sets only the properties it names, and a property with no step at 0% or 100% starts or ends at the element's own value. Below, the toast grows past its size at 60% and settles at 100%, and `animation-fill-mode: backwards` shows the 0% step during the 150ms delay:",
                ja: "`cubic-bezier(0.16, 1, 0.3, 1)` のようなキーワード以外のタイミング関数は `raw` で書きます。段階が設定するのはその段階に書いたプロパティだけで、0% や 100% の段階がないプロパティは、要素自身の値から始まるか、要素自身の値で終わります。下のトーストは 60% で本来の大きさを超え、100% で本来の大きさに戻ります。`animation-fill-mode: backwards` により、150ms の遅延の間は 0% の段階が表示されます。",
            },
        ),
        show(Snippet::KeyframesSteps),
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
                en: "`animation_name` takes an [`AnimationName`](api:hypersig/style/value/enum.AnimationName.html), which converts from no string, so a name written by hand is E0277. The name reaches `animation-name` through `Style::keyframes`, which also adds the rule:",
                ja: "`animation_name` は [`AnimationName`](api:hypersig/style/value/enum.AnimationName.html) を受け取り、`AnimationName` は文字列から変換できません。そのため、手で書いた名前は E0277 になります。名前は `Style::keyframes` を通して `animation-name` に入り、そのときルールもページに加わります。",
            },
        ),
        show(Snippet::KeyframesName),
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
                    en: "The name's characters and the step's percent, at run time. `Keyframes::new` panics where it is called on an empty name, a name that starts with a digit, or a character other than an ASCII letter, a digit, `-`, and `_`. `at` panics on a percent over 100. Neither is a `const fn`, so these are panics when the line runs, not compile errors. The name `none` passes the check, and the browser plays no animation for it.",
                    ja: "名前の文字と段階のパーセント（実行時）。`Keyframes::new` は、空の名前、数字で始まる名前、ASCII の英字、数字、`-`、`_` 以外の文字を含む名前を受け取ると、呼んだところで panic します。`at` は 100 を超えるパーセントで panic します。どちらも `const fn` ではないので、これはコンパイルエラーではなく、その行を実行したときの panic です。`none` という名前は検査を通りますが、ブラウザはそのアニメーションを再生しません。",
                },
                L {
                    en: "Conditions inside a step. A keyframe takes no selector or media query, so `at` writes only the declarations of `style` that have no condition, and drops a declaration under `on` or `pseudo` without a warning.",
                    ja: "段階の中の条件。キーフレームにはセレクタもメディアクエリも書けないので、`at` は `style` のうち条件の付かない宣言だけを書き、`on` や `pseudo` の下の宣言は警告なしに捨てます。",
                },
                L {
                    en: "Unique names across patches, at run time. Two `Keyframes` with one name and different steps panic where a style combines them and where `Page::render` or a patch writes both. `Page::render` checks the page's `Global` rules and its elements together. One `Keyframes` that many elements use is written once. A patch is checked only against itself, so a patch whose keyframes reuse a name of the page with other steps replaces the steps for every element that uses the name.",
                    ja: "パッチをまたいだ名前の重複（実行時）。同じ名前で段階の違う 2 つの `Keyframes` は、スタイルが両方を組み合わせたところと、`Page::render` やパッチが両方を書くところで panic します。`Page::render` はページの `Global` のルールと要素をまとめて検査します。多くの要素が使う 1 つの `Keyframes` は 1 回だけ書かれます。パッチはそのパッチの中でしか検査されないので、ページと同じ名前で段階の違うキーフレームを持つパッチが届くと、その名前を使うすべての要素の段階がパッチのものに置き換わります。",
                },
                L {
                    en: "A name in `raw`. `animation_name(raw(\"spin\"))` and `animation(raw(\"spin 1s linear infinite\"))` write the name without the rule, which [Unchecked values](style/unchecked/) covers.",
                    ja: "`raw` の中の名前。`animation_name(raw(\"spin\"))` や `animation(raw(\"spin 1s linear infinite\"))` はルールなしで名前だけを書きます。これは[検査しない値](style/unchecked/)で説明しています。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Next, [Transitions](style/transitions/) animates a property from its old value to its new one when it changes.",
                ja: "次の[トランジション](style/transitions/)では、プロパティの値が変わったときに、前の値から新しい値へアニメーションさせます。",
            },
        ),
    ])
}
