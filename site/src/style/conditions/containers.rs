//! The page on styles under a container query.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Containers).leads()),
        heading(
            site,
            L {
                en: "A style by the width of an ancestor",
                ja: "祖先の幅に応じたスタイル",
            },
        ),
        para(
            site,
            L {
                en: "A media query measures the viewport, so a card in a narrow sidebar gets the same layout as one in a wide main column. A [`Container`](api:hypersig/style/enum.Container.html) measures the nearest ancestor that is a query container instead. An element becomes a query container with `container_type(ContainerType::InlineSize)`, and `Style::on(Container::Md, ..)` on an element inside it applies from 28rem of that ancestor's width up:",
                ja: "メディアクエリはビューポートを測るので、狭いサイドバーの中のカードも広いメイン列の中のカードも同じレイアウトになります。[`Container`](api:hypersig/style/enum.Container.html) は、ビューポートの代わりに、いちばん近いクエリコンテナの祖先を測ります。要素は `container_type(ContainerType::InlineSize)` でクエリコンテナになります。その内側の要素に付けた `Style::on(Container::Md, ..)` は、その祖先の幅が 28rem 以上のときに適用されます。",
            },
        ),
        show(Snippet::ContainerQueries),
        para(
            site,
            L {
                en: "Writing `.column` and `.card` for the two elements, the browser applies:",
                ja: "2 つの要素をそれぞれ `.column` と `.card` と書くと、ブラウザが適用するのは次の CSS です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ".column { container-type: inline-size }\n\
             .card { display: flex; flex-direction: column; gap: 0.5rem }\n\
             @container (min-width: 28rem) { .card { flex-direction: row } }",
        ),
        heading(
            site,
            L {
                en: "The widths",
                ja: "幅",
            },
        ),
        para(
            site,
            L {
                en: "The widths are the default container sizes of Panda CSS, so its `@/md` is `Container::Md` here:",
                ja: "幅は Panda CSS の既定のコンテナの大きさと同じで、Panda CSS の `@/md` はここでは `Container::Md` です。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Variant", "Container query", "Panda CSS"],
                ja: &["バリアント", "コンテナクエリ", "Panda CSS"],
            },
            L {
                en: &[
                    &["`Xs3`", "`(min-width: 16rem)`", "`@/3xs`"],
                    &["`Xs2`", "`(min-width: 18rem)`", "`@/2xs`"],
                    &["`Xs`", "`(min-width: 20rem)`", "`@/xs`"],
                    &["`Sm`", "`(min-width: 24rem)`", "`@/sm`"],
                    &["`Md`", "`(min-width: 28rem)`", "`@/md`"],
                    &["`Lg`", "`(min-width: 32rem)`", "`@/lg`"],
                    &["`Xl`", "`(min-width: 36rem)`", "`@/xl`"],
                    &["`Xl2`", "`(min-width: 42rem)`", "`@/2xl`"],
                    &["`Xl3`", "`(min-width: 48rem)`", "`@/3xl`"],
                    &["`Xl4`", "`(min-width: 56rem)`", "`@/4xl`"],
                    &["`Xl5`", "`(min-width: 64rem)`", "`@/5xl`"],
                    &["`Xl6`", "`(min-width: 72rem)`", "`@/6xl`"],
                    &["`Xl7`", "`(min-width: 80rem)`", "`@/7xl`"],
                    &["`Xl8`", "`(min-width: 90rem)`", "`@/8xl`"],
                ],
                ja: &[
                    &["`Xs3`", "`(min-width: 16rem)`", "`@/3xs`"],
                    &["`Xs2`", "`(min-width: 18rem)`", "`@/2xs`"],
                    &["`Xs`", "`(min-width: 20rem)`", "`@/xs`"],
                    &["`Sm`", "`(min-width: 24rem)`", "`@/sm`"],
                    &["`Md`", "`(min-width: 28rem)`", "`@/md`"],
                    &["`Lg`", "`(min-width: 32rem)`", "`@/lg`"],
                    &["`Xl`", "`(min-width: 36rem)`", "`@/xl`"],
                    &["`Xl2`", "`(min-width: 42rem)`", "`@/2xl`"],
                    &["`Xl3`", "`(min-width: 48rem)`", "`@/3xl`"],
                    &["`Xl4`", "`(min-width: 56rem)`", "`@/4xl`"],
                    &["`Xl5`", "`(min-width: 64rem)`", "`@/5xl`"],
                    &["`Xl6`", "`(min-width: 72rem)`", "`@/6xl`"],
                    &["`Xl7`", "`(min-width: 80rem)`", "`@/7xl`"],
                    &["`Xl8`", "`(min-width: 90rem)`", "`@/8xl`"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Like the breakpoints of `Media`, each is a min-width query, a larger width overrides a smaller one, and container conditions nest. A container condition nested in a media condition is written inside the `@media` block.",
                ja: "`Media` のブレークポイントと同じく、どれも min-width のクエリで、大きい幅が小さい幅を上書きし、コンテナの条件も入れ子にできます。メディアの条件の中に入れ子にしたコンテナの条件は、`@media` のブロックの内側に書かれます。",
            },
        ),
        heading(
            site,
            L {
                en: "Only the listed widths",
                ja: "使えるのは一覧の幅だけ",
            },
        ),
        para(
            site,
            L {
                en: "`Container` has no variant that takes a length, so a width outside the table is a compile error:",
                ja: "`Container` には長さを受け取るバリアントがないので、表にない幅を書くとコンパイルエラーになります。",
            },
        ),
        show(Snippet::ContainerWidth),
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
                    en: "Nothing checks that an ancestor is a query container. Without one, the condition never matches, and the element keeps the style it has without the condition.",
                    ja: "祖先がクエリコンテナかどうかは検査しません。クエリコンテナの祖先がなければ条件は一度も当てはまらず、要素は条件のないときのスタイルのままです。",
                },
                L {
                    en: "A container condition measures a container above the element, never the element itself. `container_type` and `on(Container::..)` on the same element measure the next container up, or none.",
                    ja: "コンテナの条件が測るのは要素より上にあるコンテナで、要素自身は測りません。同じ要素に `container_type` と `on(Container::..)` を付けると、測るのはさらに上のコンテナになり、なければ何も測りません。",
                },
                L {
                    en: "With `container-type: inline-size`, the container no longer takes its width from its content. A container that would shrink to fit its content, such as a flex item without a width, can collapse to zero width.",
                    ja: "`container-type: inline-size` を付けると、コンテナは内容から幅を決めなくなります。幅のないフレックスアイテムのように内容に合わせて縮むはずだったコンテナは、幅が 0 になることがあります。",
                },
                L {
                    en: "The query always measures the nearest container. A container condition that names a container, as `@container sidebar (..)` does, has no variant.",
                    ja: "クエリが測るのは常にいちばん近いコンテナです。`@container sidebar (..)` のようにコンテナを名前で指定する条件にはバリアントがありません。",
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
                en: "[Group and peer](style/group-and-peer/) applies a style while an ancestor or a sibling matches a pseudo-class.",
                ja: "[グループとピア](style/group-and-peer/)では、祖先や兄弟が擬似クラスに当てはまる間だけスタイルを適用します。",
            },
        ),
    ])
}
