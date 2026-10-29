//! The Style page on `Angle` and `Time`, the functions that build them, and
//! the properties that take them.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::AnglesAndTimes).leads()),
        types(site),
        accepted(site),
        unchecked(site),
        example(site),
    ])
}

fn types(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Angle and Time",
                ja: "Angle と Time",
            },
        ),
        para(
            site,
            L {
                en: "[`Angle`](api:hypersig/style/value/struct.Angle.html) and [`Time`](api:hypersig/style/value/struct.Time.html) are numbers with a unit, built by four functions that each take `impl Into<f64>`:",
                ja: "[`Angle`](api:hypersig/style/value/struct.Angle.html) と [`Time`](api:hypersig/style/value/struct.Time.html) は単位の付いた数値で、どれも `impl Into<f64>` を受け取る 4 つの関数で作ります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "Returns", "CSS"],
                ja: &["関数", "戻り値", "CSS"],
            },
            L {
                en: &[
                    &[
                        "[`deg(90)`](api:hypersig/style/value/fn.deg.html)",
                        "`Angle`",
                        "`90deg`",
                    ],
                    &[
                        "[`turn(0.25)`](api:hypersig/style/value/fn.turn.html)",
                        "`Angle`",
                        "`0.25turn`",
                    ],
                    &[
                        "[`ms(150)`](api:hypersig/style/value/fn.ms.html)",
                        "`Time`",
                        "`150ms`",
                    ],
                    &[
                        "[`secs(1.5)`](api:hypersig/style/value/fn.secs.html)",
                        "`Time`",
                        "`1.5s`",
                    ],
                ],
                ja: &[
                    &[
                        "[`deg(90)`](api:hypersig/style/value/fn.deg.html)",
                        "`Angle`",
                        "`90deg`",
                    ],
                    &[
                        "[`turn(0.25)`](api:hypersig/style/value/fn.turn.html)",
                        "`Angle`",
                        "`0.25turn`",
                    ],
                    &[
                        "[`ms(150)`](api:hypersig/style/value/fn.ms.html)",
                        "`Time`",
                        "`150ms`",
                    ],
                    &[
                        "[`secs(1.5)`](api:hypersig/style/value/fn.secs.html)",
                        "`Time`",
                        "`1.5s`",
                    ],
                ],
            },
        ),
        show(Snippet::StyleAnglesTimes),
        para(
            site,
            L {
                en: "A zero angle or duration keeps its unit, because CSS reads `transition-duration: 0` as invalid, while a [zero length](style/lengths/) is written `0`. Angles in `rad` or `grad` have no function, and go through `raw`. Angles and durations take the [math functions](style/lengths/#math-functions) that lengths do, so `ms(150) * 2.0` writes `calc(150ms * 2)`.",
                ja: "角度と時間のゼロには単位が残ります。CSS は `transition-duration: 0` を無効な値として読むからです。[長さのゼロ](style/lengths/)は `0` と書かれます。`rad` や `grad` の角度を作る関数はないので、`raw` で書きます。角度と時間にも長さと同じ[数学関数](style/lengths/#math-functions)を使えるので、`ms(150) * 2.0` は `calc(150ms * 2)` を書きます。",
            },
        ),
    ])
}

fn accepted(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Properties that take them",
                ja: "受け取るプロパティ",
            },
        ),
        para(
            site,
            L {
                en: "`rotate`, `offset_rotate`, and `image_orientation` take an angle. `transition_duration`, `transition_delay`, `animation_duration`, `animation_delay`, and the shorthands `transition` and `animation` take a time. A bare number is neither, so `.transition_duration(150)` is E0277:",
                ja: "角度を受け取るのは `rotate`、`offset_rotate`、`image_orientation` です。時間を受け取るのは `transition_duration`、`transition_delay`、`animation_duration`、`animation_delay` と、ショートハンドの `transition` と `animation` です。単位のない数値はどちらでもないので、`.transition_duration(150)` は E0277 になります。",
            },
        ),
        show(Snippet::StyleTimeBare),
        para(
            site,
            L {
                en: "The two types do not convert into each other, so a time where an angle belongs is E0277 too:",
                ja: "2 つの型は互いに変換できないので、角度の場所に時間を渡しても E0277 になります。",
            },
        ),
        show(Snippet::StyleAngleTimeSwap),
        para(
            site,
            L {
                en: "A `transform` with a rotation, such as `rotate(45deg) scale(1.1)`, has several parts and goes through `raw`. [Transitions](style/transitions/) and [Keyframes](style/animation/) cover the properties that use times.",
                ja: "`rotate(45deg) scale(1.1)` のように回転を含む `transform` は複数の部分からなるので、`raw` で書きます。時間を使うプロパティは[トランジション](style/transitions/)と[キーフレーム](style/animation/)で説明します。",
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
                en: "The sign is not checked. A negative delay is valid CSS and starts the transition partway through, while a negative duration is invalid, and the browser drops the declaration. Like the other unit functions, these four panic on NaN or infinity.",
                ja: "符号は検査されません。負の遅延は正しい CSS で、トランジションを途中から始めます。一方、負の長さの時間は無効で、ブラウザはその宣言を捨てます。ほかの単位の関数と同じく、この 4 つも NaN や無限大を受け取るとパニックします。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a turning arrow",
                ja: "例: 回る矢印",
            },
        ),
        para(
            site,
            L {
                en: "The arrow turns 90 degrees over 200 milliseconds while the pointer is over the button. `Style::group` marks the button, and the arrow's rotation applies under `Condition::Group(PseudoClass::Hover)`, which [Group and peer](style/group-and-peer/) covers:",
                ja: "ポインタがボタンの上にある間、矢印が 200 ミリ秒かけて 90 度回ります。`Style::group` でボタンに印を付け、矢印の回転は `Condition::Group(PseudoClass::Hover)` の下で適用します。この条件は[グループとピア](style/group-and-peer/)で説明します。",
            },
        ),
        show(Snippet::StyleChevron),
        para(
            site,
            L {
                en: "Next, [URLs and strings](style/urls-and-strings/) covers `url` and `quoted`.",
                ja: "次の[URL と文字列](style/urls-and-strings/)では `url` と `quoted` を説明します。",
            },
        ),
    ])
}
