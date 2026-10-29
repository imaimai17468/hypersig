//! The component page on `separator`: a one-pixel line, horizontal as an
//! `<hr>` or vertical as a `role="separator"`.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Separator).leads()),
        demo::show(site, Demo::Separator),
        accessibility(site),
        building(site),
        html(site),
        unchecked(site),
        example(site),
    ])
}

fn accessibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Accessibility",
                ja: "アクセシビリティ",
            },
        ),
        para(
            site,
            L {
                en: "A horizontal separator is an `<hr>`, and a vertical one is a `<div>` with `role=\"separator\"` and `aria-orientation=\"vertical\"`, so a screen reader announces both as separators. Neither takes focus or keys.",
                ja: "横向きの区切り線は `<hr>` で、縦向きの区切り線は `role=\"separator\"` と `aria-orientation=\"vertical\"` を持つ `<div>` です。そのため、スクリーンリーダーはどちらも区切り線として読み上げます。どちらもフォーカスを受けず、キーも受け付けません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Drawing a line",
                ja: "線を引く",
            },
        ),
        para(
            site,
            L {
                en: "[`separator`](api:hypersig_ui/fn.separator.html) takes the direction the line runs in, an [`Orientation`](api:hypersig/html/enum.Orientation.html) from `hypersig::html`:",
                ja: "[`separator`](api:hypersig_ui/fn.separator.html) は線の向きを `hypersig::html` の [`Orientation`](api:hypersig/html/enum.Orientation.html) で受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn separator<S>(orientation: Orientation) -> El<S>;

pub enum Orientation {
    Horizontal,
    Vertical,
}",
        ),
        para(
            site,
            L {
                en: "The line is one pixel thick in `token::border`. A horizontal line spans the width of its parent, and a vertical one stretches to the height of the flex row it sits in. The direction is an enum of two variants, so the only mistake left to the types is the wrong direction, which they cannot see.",
                ja: "線の太さは 1 ピクセルで、色は `token::border` です。横向きの線は親の幅いっぱいに伸び、縦向きの線は置かれた flex の行の高さまで伸びます。向きは 2 つのバリアントを持つ列挙型なので、型に残る間違いは向きの取り違えだけで、それは型からはわかりません。",
            },
        ),
    ])
}

fn html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTML",
                ja: "出力される HTML",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<!-- separator(Orientation::Horizontal) -->
<hr>

<!-- separator(Orientation::Vertical) -->
<div role="separator" aria-orientation="vertical"></div>"#,
        ),
        para(
            site,
            L {
                en: "The vertical `<div>` is empty and has no height of its own.",
                ja: "縦向きの `<div>` は空で、自身の高さを持ちません。",
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
                en: "A vertical separator takes its height from a flex row, such as one styled with `hstack`. Placed in a block, or in a row whose height comes only from the separator, it is zero pixels high and draws nothing, while a screen reader still announces a separator there. Give the row a height, as the example below does with `height(rem(1.25))`.",
                ja: "縦向きの区切り線は `hstack` などで作った flex の行から高さを得ます。ブロックの中や、高さが区切り線だけで決まる行に置くと、高さが 0 ピクセルになって何も描かれませんが、スクリーンリーダーはそこに区切り線があると読み上げます。下の例の `height(rem(1.25))` のように、行に高さを付けてください。",
            },
        ),
        para(
            site,
            L {
                en: "A separator is always announced. For a line that only decorates, draw it with a border from `hypersig::style` instead, which assistive technology does not see.",
                ja: "区切り線は必ず読み上げられます。飾りとしての線には支援技術から見えない `hypersig::style` の枠線を使ってください。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a profile header",
                ja: "例: プロフィールの見出し",
            },
        ),
        para(
            site,
            L {
                en: "A horizontal line under the description, and vertical lines between the names in a row of fixed height:",
                ja: "説明の下に横向きの線を引き、高さを決めた行の中の名前の間に縦向きの線を引きます。",
            },
        ),
        show(Snippet::UiSeparator),
        para(
            site,
            L {
                en: "Next, [Navigation](components/navigation/) lists the sections of an application and marks the one the visitor is in.",
                ja: "次の [ナビゲーション](components/navigation/) ではアプリケーションのセクションを並べ、訪問者がいるセクションに印を付けます。",
            },
        ),
    ])
}
