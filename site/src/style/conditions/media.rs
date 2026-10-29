//! The page on styles under a media query.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Media).leads()),
        heading(
            site,
            L {
                en: "Breakpoints",
                ja: "ブレークポイント",
            },
        ),
        para(
            site,
            L {
                en: "A [`Media`](api:hypersig/style/enum.Media.html) converts into a `Condition`, so `Style::on` takes it as it takes a pseudo-class. The breakpoints are min-width queries: a style for one applies from that width up, and a larger breakpoint overrides a smaller one. Write the narrowest layout without a condition, then add what changes at each width:",
                ja: "[`Media`](api:hypersig/style/enum.Media.html) は `Condition` に変換できるので、`Style::on` は擬似クラスと同じように受け取ります。ブレークポイントは min-width のメディアクエリです。あるブレークポイントのスタイルはその幅以上で適用され、大きいブレークポイントが小さいブレークポイントを上書きします。いちばん狭いときのレイアウトを条件なしで書き、幅ごとに変わる部分を追加します。",
            },
        ),
        show(Snippet::MediaQueries),
        para(
            site,
            L {
                en: "Writing `.column` and `.sidebar` for the two elements, the browser applies:",
                ja: "2 つの要素をそれぞれ `.column` と `.sidebar` と書くと、ブラウザが適用するのは次の CSS です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ".column { width: 100% }\n\
             @media (min-width: 48rem) { .column { width: 50% } }\n\
             @media (min-width: 64rem) { .column { width: 33% } }\n\
             @media (min-width: 48rem) and (prefers-color-scheme: dark) { .column { color: white } }\n\
             @media print { .sidebar { display: none } }",
        ),
        para(
            site,
            L {
                en: "The widths are the default breakpoints of Tailwind CSS and Panda CSS, so a layout ported from either keeps its widths:",
                ja: "幅は Tailwind CSS と Panda CSS の既定のブレークポイントと同じなので、どちらから移したレイアウトも同じ幅のまま使えます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Variant", "Media query", "Pixels at a 16px root font size"],
                ja: &[
                    "バリアント",
                    "メディアクエリ",
                    "ルートのフォントサイズが 16px のときのピクセル",
                ],
            },
            L {
                en: &[
                    &["`Sm`", "`(min-width: 40rem)`", "640"],
                    &["`Md`", "`(min-width: 48rem)`", "768"],
                    &["`Lg`", "`(min-width: 64rem)`", "1024"],
                    &["`Xl`", "`(min-width: 80rem)`", "1280"],
                    &["`Xxl`", "`(min-width: 96rem)`", "1536"],
                ],
                ja: &[
                    &["`Sm`", "`(min-width: 40rem)`", "640"],
                    &["`Md`", "`(min-width: 48rem)`", "768"],
                    &["`Lg`", "`(min-width: 64rem)`", "1024"],
                    &["`Xl`", "`(min-width: 80rem)`", "1280"],
                    &["`Xxl`", "`(min-width: 96rem)`", "1536"],
                ],
            },
        ),
        heading(
            site,
            L {
                en: "User settings and devices",
                ja: "ユーザーの設定とデバイス",
            },
        ),
        para(
            site,
            L {
                en: "The other variants test a setting of the user or a property of the device:",
                ja: "ほかのバリアントはユーザーの設定やデバイスの性質を調べます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Variant", "Media query", "Matches when"],
                ja: &["バリアント", "メディアクエリ", "当てはまるとき"],
            },
            L {
                en: &[
                    &[
                        "`Dark`",
                        "`(prefers-color-scheme: dark)`",
                        "The system prefers a dark color scheme",
                    ],
                    &[
                        "`Light`",
                        "`(prefers-color-scheme: light)`",
                        "The system prefers a light color scheme",
                    ],
                    &[
                        "`ReducedMotion`",
                        "`(prefers-reduced-motion: reduce)`",
                        "The user asks for reduced motion",
                    ],
                    &[
                        "`MoreContrast`",
                        "`(prefers-contrast: more)`",
                        "The user asks for more contrast",
                    ],
                    &[
                        "`ForcedColors`",
                        "`(forced-colors: active)`",
                        "The browser forces a limited palette, as Windows high contrast does",
                    ],
                    &["`Print`", "`print`", "The page is printed"],
                    &[
                        "`Landscape`",
                        "`(orientation: landscape)`",
                        "The viewport is wider than it is tall",
                    ],
                    &[
                        "`Portrait`",
                        "`(orientation: portrait)`",
                        "The viewport is taller than it is wide",
                    ],
                    &[
                        "`CanHover`",
                        "`(hover: hover)`",
                        "The primary pointer can hover, as a mouse can and a finger cannot",
                    ],
                    &[
                        "`CoarsePointer`",
                        "`(pointer: coarse)`",
                        "The primary pointer is coarse, such as a finger",
                    ],
                ],
                ja: &[
                    &[
                        "`Dark`",
                        "`(prefers-color-scheme: dark)`",
                        "システムがダークの配色を選んでいる",
                    ],
                    &[
                        "`Light`",
                        "`(prefers-color-scheme: light)`",
                        "システムがライトの配色を選んでいる",
                    ],
                    &[
                        "`ReducedMotion`",
                        "`(prefers-reduced-motion: reduce)`",
                        "ユーザーが動きを減らすよう設定している",
                    ],
                    &[
                        "`MoreContrast`",
                        "`(prefers-contrast: more)`",
                        "ユーザーがコントラストを上げるよう設定している",
                    ],
                    &[
                        "`ForcedColors`",
                        "`(forced-colors: active)`",
                        "Windows のハイコントラストのように、ブラウザが限られた色だけを使わせている",
                    ],
                    &["`Print`", "`print`", "ページを印刷している"],
                    &[
                        "`Landscape`",
                        "`(orientation: landscape)`",
                        "ビューポートの幅が高さより大きい",
                    ],
                    &[
                        "`Portrait`",
                        "`(orientation: portrait)`",
                        "ビューポートの高さが幅より大きい",
                    ],
                    &[
                        "`CanHover`",
                        "`(hover: hover)`",
                        "マウスのように、主なポインタでホバーできる（指ではできない）",
                    ],
                    &[
                        "`CoarsePointer`",
                        "`(pointer: coarse)`",
                        "主なポインタが指のように大まかである",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Media conditions nest as pseudo-classes do, and nested ones are joined with `and` in one query, as `Md` and `Dark` are above. `print` is written first in the query, because the browser reads a media type after a feature as a query that never matches. [Dark mode](style/dark-mode/) and [Reduced motion](style/reduced-motion/) cover `Dark` and `ReducedMotion` in detail.",
                ja: "メディアの条件も擬似クラスと同じように入れ子にでき、入れ子にした条件は上の `Md` と `Dark` のように `and` でつないだ 1 つのクエリになります。`print` はクエリの先頭に書かれます。メディア特性のあとにメディアタイプを書くと、ブラウザはそのクエリを決して当てはまらないものとして読むからです。`Dark` と `ReducedMotion` は[ダークモード](style/dark-mode/)と[動きを減らす設定](style/reduced-motion/)で詳しく説明しています。",
            },
        ),
        heading(
            site,
            L {
                en: "A media query and a pseudo-class",
                ja: "メディアクエリと擬似クラス",
            },
        ),
        para(
            site,
            L {
                en: "A rule under a media query comes after every rule without one, and the later rule wins where both match. If a link sets `color` under `Hover` and again under `Md`, the `Md` value applies from 48rem up even while the link is hovered. To keep the hover color at that width, set it inside the `Md` style too:",
                ja: "メディアクエリの下のルールはメディアクエリのないすべてのルールのあとに来ます。両方が当てはまる場合は、あとのルールが優先されます。リンクが `Hover` の下と `Md` の下の両方で `color` を設定すると、48rem 以上ではホバー中でも `Md` の値が適用されます。その幅でもホバーの色を保つには、`Md` のスタイルの中でもホバーの色を設定します。",
            },
        ),
        show(Snippet::MediaOrder),
        heading(
            site,
            L {
                en: "A media query that does not exist",
                ja: "存在しないメディアクエリ",
            },
        ),
        para(
            site,
            L {
                en: "`Media` holds only the variants above, so a breakpoint name from another framework, or a query that does not exist, is a compile error:",
                ja: "`Media` には上に挙げたバリアントしかないので、ほかのフレームワークのブレークポイント名や存在しないクエリを書くとコンパイルエラーになります。",
            },
        ),
        show(Snippet::MediaTypo),
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型では検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "There is no max-width breakpoint and no width of your own. A style for narrow screens only is the style without a condition, overridden from the breakpoint up.",
                    ja: "max-width のブレークポイントや、自分で決めた幅はありません。狭い画面だけのスタイルは条件なしで書き、ブレークポイント以上で上書きします。",
                },
                L {
                    en: "Nothing checks that two conditions can hold at once. `on(Landscape, Style::new().on(Portrait, ..))` compiles, and its declarations never apply.",
                    ja: "2 つの条件が同時に成り立ちうるかどうかは検査しません。`on(Landscape, Style::new().on(Portrait, ..))` はコンパイルが通り、その宣言は一度も適用されません。",
                },
                L {
                    en: "The breakpoints are in `rem`, so they move with the root font size. A visitor who sets a larger default font size gets the narrower layout at a wider window.",
                    ja: "ブレークポイントの単位は `rem` なので、ルートのフォントサイズに合わせて動きます。既定のフォントサイズを大きくした訪問者には、より広いウィンドウでも狭いときのレイアウトが表示されます。",
                },
            ],
        ),
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
                en: "[Container queries](style/containers/) applies a style by the width of an ancestor instead of the viewport.",
                ja: "[コンテナクエリ](style/containers/)ではビューポートではなく祖先の幅に応じてスタイルを適用します。",
            },
        ),
    ])
}
