//! The page on the remaining layout patterns: `center`, `container`,
//! `square`, `circle`, `aspect_ratio`, `link_overlay`, and `visually_hidden`.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Style(StylePage::Patterns).title(site)),
        lead(site, Doc::Style(StylePage::Patterns).leads()),
        heading(
            site,
            L {
                en: "Centering",
                ja: "中央に置く",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "The CSS it writes"],
                ja: &["関数", "書かれる CSS"],
            },
            L {
                en: &[
                    &[
                        "[`center() -> Style`](api:hypersig/style/pattern/fn.center.html)",
                        "`display: flex; align-items: center; justify-content: center`",
                    ],
                    &[
                        "[`container(max: impl Into<MaxWidth>, padding: impl Into<PaddingInline>) -> Style`](api:hypersig/style/pattern/fn.container.html)",
                        "`max-width: <max>; margin-inline: auto; padding-inline: <padding>`",
                    ],
                ],
                ja: &[
                    &[
                        "[`center() -> Style`](api:hypersig/style/pattern/fn.center.html)",
                        "`display: flex; align-items: center; justify-content: center`",
                    ],
                    &[
                        "[`container(max: impl Into<MaxWidth>, padding: impl Into<PaddingInline>) -> Style`](api:hypersig/style/pattern/fn.container.html)",
                        "`max-width: <max>; margin-inline: auto; padding-inline: <padding>`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`center` puts its children in the middle of the element on both axes, which shows when the element is larger than they are, as with `center().min_height(rem(12))`. `container` keeps a block of the page at most `max` wide and centers it in the space around it, with `padding` on its left and right, so text does not touch the edges of a narrow screen. `max` and `padding` take a length or a percentage.",
                ja: "`center` は子を縦横とも要素の中央に置きます。効果が見えるのは、`center().min_height(rem(12))` のように要素が子より大きいときです。`container` はページのブロックの幅を `max` 以下に抑えて周りの空間の中央に置き、左右に `padding` の余白を付けます。そのため、幅の狭い画面でもテキストが端に触れません。`max` と `padding` には長さかパーセンテージを渡します。",
            },
        ),
        heading(
            site,
            L {
                en: "Fixed shapes",
                ja: "形の決まったボックス",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "The CSS it writes"],
                ja: &["関数", "書かれる CSS"],
            },
            L {
                en: &[
                    &[
                        "[`square(size: Length) -> Style`](api:hypersig/style/pattern/fn.square.html)",
                        "The declarations of `center`, and `width: <size>; height: <size>; flex-shrink: 0`",
                    ],
                    &[
                        "[`circle(size: Length) -> Style`](api:hypersig/style/pattern/fn.circle.html)",
                        "The declarations of `square`, and `border-radius: 50%`",
                    ],
                    &[
                        "[`aspect_ratio(ratio: f64) -> Style`](api:hypersig/style/pattern/fn.aspect_ratio.html)",
                        "`position: relative; aspect-ratio: <ratio>; overflow: hidden`",
                    ],
                ],
                ja: &[
                    &[
                        "[`square(size: Length) -> Style`](api:hypersig/style/pattern/fn.square.html)",
                        "`center` の宣言と `width: <size>; height: <size>; flex-shrink: 0`",
                    ],
                    &[
                        "[`circle(size: Length) -> Style`](api:hypersig/style/pattern/fn.circle.html)",
                        "`square` の宣言と `border-radius: 50%`",
                    ],
                    &[
                        "[`aspect_ratio(ratio: f64) -> Style`](api:hypersig/style/pattern/fn.aspect_ratio.html)",
                        "`position: relative; aspect-ratio: <ratio>; overflow: hidden`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`square` and `circle` center their children, such as the initials of an avatar, and `flex-shrink: 0` keeps their size inside a crowded `hstack`. `aspect_ratio` takes the width over the height, so `aspect_ratio(16.0 / 9.0)` writes `aspect-ratio: 1.7777777777777777`. Its height follows its width, and content taller than that is clipped.",
                ja: "`square` と `circle` はアバターのイニシャルのような子を中央に置きます。`flex-shrink: 0` があるので、詰まった `hstack` の中でも大きさが変わりません。`aspect_ratio` は幅を高さで割った値を受け取るので、`aspect_ratio(16.0 / 9.0)` は `aspect-ratio: 1.7777777777777777` を書きます。高さは幅に合わせて決まり、それより高い内容ははみ出した部分が切り取られます。",
            },
        ),
        para(
            site,
            L {
                en: "The size of `square` and `circle` is a [`Length`](api:hypersig/style/value/struct.Length.html) because the same value sets the width and the height, and a percentage resolves against the parent's width for one and its height for the other. So a percentage is E0308:",
                ja: "`square` と `circle` の大きさが [`Length`](api:hypersig/style/value/struct.Length.html) なのは、同じ値で幅と高さを設定するからです。パーセンテージは、幅では親の幅に対する割合に、高さでは親の高さに対する割合になります。そのため、パーセンテージを渡すと E0308 になります。",
            },
        ),
        show(Snippet::CircleSize),
        heading(
            site,
            L {
                en: "A link that covers a card",
                ja: "カードを覆うリンク",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "The CSS it writes"],
                ja: &["関数", "書かれる CSS"],
            },
            L {
                en: &[&[
                    "[`link_overlay() -> Style`](api:hypersig/style/pattern/fn.link_overlay.html)",
                    "On the element's `::before`: `content: \"\"; position: absolute; inset: 0`",
                ]],
                ja: &[&[
                    "[`link_overlay() -> Style`](api:hypersig/style/pattern/fn.link_overlay.html)",
                    "要素の `::before` に `content: \"\"; position: absolute; inset: 0`",
                ]],
            },
        ),
        para(
            site,
            L {
                en: "Put it on the card's `<a>`. Its `::before` fills the nearest ancestor whose `position` is not `static`, so a click anywhere on the card follows the link, while the link's text stays the card's title for a screen reader. `aspect_ratio` sets `position: relative`, and on a card without it, add `.position(Position::Relative)` yourself.",
                ja: "カードの `<a>` に付けます。その `::before` は `position` が `static` でない最も近い祖先を覆うので、カードのどこをクリックしてもリンク先へ移り、スクリーンリーダーにとってはリンクのテキストがカードの見出しのままです。`aspect_ratio` は `position: relative` を設定します。これを使わないカードには自分で `.position(Position::Relative)` を付けます。",
            },
        ),
        heading(
            site,
            L {
                en: "Hidden text",
                ja: "画面に見えないテキスト",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "The CSS it writes"],
                ja: &["関数", "書かれる CSS"],
            },
            L {
                en: &[&[
                    "[`visually_hidden() -> Style`](api:hypersig/style/pattern/fn.visually_hidden.html)",
                    "`position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; border-width: 0`",
                ]],
                ja: &[&[
                    "[`visually_hidden() -> Style`](api:hypersig/style/pattern/fn.visually_hidden.html)",
                    "`position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; border-width: 0`",
                ]],
            },
        ),
        para(
            site,
            L {
                en: "These are the declarations of the `sr-only` utility of Tailwind CSS and the `visuallyHidden` pattern of Panda CSS. The element takes no space and shows nothing, and a screen reader still reads its text, such as a count that an icon shows to sighted visitors. `display: none` or the `hidden` attribute would hide the text from the screen reader too.",
                ja: "これは Tailwind CSS の `sr-only` と Panda CSS の `visuallyHidden` と同じ宣言です。要素は場所を取らず何も表示しませんが、スクリーンリーダーはそのテキストを読みます。目の見える訪問者にはアイコンで示す件数などに使います。`display: none` や `hidden` 属性では、スクリーンリーダーからもテキストが隠れます。",
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
                    en: "The ratio. `aspect_ratio(0.0)` and a negative ratio compile. The browser treats a ratio of 0 as `auto` and drops a negative one, so in both cases the box is as tall as its content. A NaN or an infinite ratio panics where `aspect_ratio` is called, because CSS has no way to write it.",
                    ja: "比の値。`aspect_ratio(0.0)` や負の比もコンパイルが通ります。ブラウザは 0 の比を `auto` として扱い、負の比は捨てるので、どちらの場合もボックスの高さは内容の高さになります。NaN や無限大の比は CSS で書けないので、`aspect_ratio` を呼んだところで panic します。",
                },
                L {
                    en: "The positioned ancestor of `link_overlay`. Without one, the `::before` covers an area the size of the viewport at the top of the page, and a click there follows the link.",
                    ja: "`link_overlay` の位置指定された祖先。それがないと `::before` はページの先頭でビューポートと同じ大きさの範囲を覆い、そこをクリックするとリンク先へ移ります。",
                },
                L {
                    en: "Other controls inside a linked card. A second link or a button in the card sits under the `::before`, so a click on it follows the card's link. `.position(Position::Relative).z_index(1)` puts it above the `::before`.",
                    ja: "リンクにしたカードの中のほかの操作要素。カードの中の 2 つ目のリンクやボタンは `::before` の下に入るので、それをクリックするとカードのリンク先へ移ります。`.position(Position::Relative).z_index(1)` を付けると `::before` の上に出ます。",
                },
                L {
                    en: "Focus on hidden text. A link or a button with `visually_hidden()` can still take focus, and a sighted keyboard user then sees no focused element. A skip link that appears on focus needs a style under `PseudoClass::Focus` that sets each of these properties back.",
                    ja: "見えないテキストのフォーカス。`visually_hidden()` を付けたリンクやボタンもフォーカスを受け取れるので、キーボードで操作する目の見える訪問者には、フォーカスのある要素が見えなくなります。フォーカスされると現れるスキップリンクには、`PseudoClass::Focus` の下でこれらのプロパティをそれぞれ元に戻すスタイルが要ります。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Example",
                ja: "例",
            },
        ),
        para(
            site,
            L {
                en: "A blog's list page puts each pattern on this page to use:",
                ja: "下のブログの一覧ページはこのページのパターンをすべて使います。",
            },
        ),
        show(Snippet::Patterns),
        para(
            site,
            L {
                en: "Next, [Keyframes](style/animation/) animates an element with an `@keyframes` rule.",
                ja: "次の[キーフレーム](style/animation/)では `@keyframes` のルールで要素をアニメーションさせます。",
            },
        ),
    ])
}
