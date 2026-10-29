//! The page on styles under the state of a marked ancestor or sibling.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::GroupAndPeer).leads()),
        heading(
            site,
            L {
                en: "The state of another element",
                ja: "別の要素の状態",
            },
        ),
        para(
            site,
            L {
                en: "A pseudo-class passed to `on` tests the element itself. Two variants of [`Condition`](api:hypersig/style/enum.Condition.html) test another element, which a method of `Style` marks first:",
                ja: "`on` に渡した擬似クラスは要素自身を調べます。[`Condition`](api:hypersig/style/enum.Condition.html) の 2 つのバリアントは別の要素を調べます。その要素には先に `Style` のメソッドで印を付けておきます。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "[`Style::group`](api:hypersig/style/struct.Style.html#method.group), `fn group(self) -> Style`, marks an ancestor, and `Condition::Group(PseudoClass)` applies a style while a marked ancestor matches the pseudo-class.",
                    ja: "[`Style::group`](api:hypersig/style/struct.Style.html#method.group)（`fn group(self) -> Style`）は祖先に印を付け、`Condition::Group(PseudoClass)` は印を付けた祖先が擬似クラスに当てはまる間だけスタイルを適用します。",
                },
                L {
                    en: "[`Style::peer`](api:hypersig/style/struct.Style.html#method.peer), `fn peer(self) -> Style`, marks a sibling, and `Condition::Peer(PseudoClass)` applies a style while a marked sibling before the element matches the pseudo-class.",
                    ja: "[`Style::peer`](api:hypersig/style/struct.Style.html#method.peer)（`fn peer(self) -> Style`）は兄弟に印を付け、`Condition::Peer(PseudoClass)` は要素より前にある印を付けた兄弟が擬似クラスに当てはまる間だけスタイルを適用します。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Both work like the `group` and `peer` of Tailwind CSS and Panda CSS. Below, the arrow shows while the pointer is anywhere on the card, and the label turns green while the checkbox before it is checked:",
                ja: "どちらも Tailwind CSS や Panda CSS の `group` と `peer` と同じように動きます。下の例では、ポインタがカードのどこかにある間は矢印が表示され、前にあるチェックボックスがチェックされている間はラベルが緑になります。",
            },
        ),
        show(Snippet::GroupPeer),
        para(
            site,
            L {
                en: "Writing `.card` for the marked link, `.arrow` for the span inside it, `.box` for the marked checkbox, and `.agree` for the span after it, the browser applies:",
                ja: "印を付けたリンクを `.card`、その中の span を `.arrow`、印を付けたチェックボックスを `.box`、そのあとの span を `.agree` と書くと、ブラウザが適用するのは次の CSS です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ".arrow { opacity: 0 }\n\
             .card:hover .arrow { opacity: 1 }\n\
             .box:checked ~ .agree { color: green }",
        ),
        para(
            site,
            L {
                en: "The demo below is built the same way, with the components' tokens:",
                ja: "下のデモも同じ作りで、コンポーネントのトークンを使っています。",
            },
        ),
        demo::show(site, Demo::Hover),
        heading(
            site,
            L {
                en: "Only a pseudo-class",
                ja: "使えるのは擬似クラスだけ",
            },
        ),
        para(
            site,
            L {
                en: "`Group` and `Peer` hold a `PseudoClass`. A media query is the same for every element on the page, so a group or peer condition on one is a compile error:",
                ja: "`Group` と `Peer` が持つのは `PseudoClass` です。メディアクエリはページのどの要素にとっても同じなので、メディアクエリを group や peer の条件にするとコンパイルエラーになります。",
            },
        ),
        show(Snippet::GroupCondition),
        para(
            site,
            L {
                en: "Group and peer conditions combine with the others. `on(Media::Md, Style::new().on(Condition::Group(PseudoClass::Hover), ..))` applies from 48rem up while a marked ancestor is hovered.",
                ja: "group と peer の条件はほかの条件と組み合わせられます。`on(Media::Md, Style::new().on(Condition::Group(PseudoClass::Hover), ..))` は、48rem 以上で、印を付けた祖先がホバーされている間に適用されます。",
            },
        ),
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
                    en: "Nothing checks that a marked element is there. Without a marked ancestor, or without a marked sibling before the element, the condition never matches.",
                    ja: "印を付けた要素があるかどうかは検査しません。印を付けた祖先がない場合や、要素より前に印を付けた兄弟がない場合、条件は一度も当てはまりません。",
                },
                L {
                    en: "A peer has to come before the element and share its parent. A marked sibling after the element, or a marked element elsewhere on the page, does not count.",
                    ja: "ピアは要素より前にあり、同じ親を持つ必要があります。要素よりあとにある印を付けた兄弟や、ページのほかの場所にある印を付けた要素は数えません。",
                },
                L {
                    en: "Any marked ancestor counts. In a card marked with `group` inside another, hovering the outer card also applies the group styles of the elements inside the inner one.",
                    ja: "印を付けた祖先ならどれでも数えます。`group` の印を付けたカードの中に別のカードがあると、外側のカードをホバーしただけで、内側のカードの中の要素にも group のスタイルが適用されます。",
                },
                L {
                    en: "A rule of a `Global` selects elements by tag and has no marked element, so `Page::render` panics when one uses a group or peer condition. [Rules by tag](style/global/) shows it.",
                    ja: "`Global` のルールはタグで要素を選び、印を付けた要素を持たないので、group や peer の条件を使うと `Page::render` がパニックします。[タグ単位のルール](style/global/)で例を示しています。",
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
                en: "[Styles switched by a signal](style/toggled/) applies a style while a signal holds a value, in place of a pseudo-class.",
                ja: "[シグナルで切り替えるスタイル](style/toggled/)では、擬似クラスの代わりに、シグナルの値に応じてスタイルを適用します。",
            },
        ),
    ])
}
