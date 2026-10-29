//! The page on styles for a pseudo-element such as `::before`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::PseudoElements).leads()),
        heading(
            site,
            L {
                en: "Declarations for a part of an element",
                ja: "要素の一部に付ける宣言",
            },
        ),
        para(
            site,
            L {
                en: "[`Style::pseudo`](api:hypersig/style/struct.Style.html#method.pseudo), `fn pseudo(self, element: PseudoElement, style: Style) -> Style`, puts the declarations of `style` on a pseudo-element of the element. [`PseudoElement`](api:hypersig/style/enum.PseudoElement.html) is generated from MDN's CSS data and has `Before`, `After`, `Placeholder`, `Marker`, `Selection`, `FileSelectorButton`, `Backdrop`, and the rest of the standard pseudo-elements.",
                ja: "[`Style::pseudo`](api:hypersig/style/struct.Style.html#method.pseudo)（`fn pseudo(self, element: PseudoElement, style: Style) -> Style`）は、`style` の宣言を要素の擬似要素に付けます。[`PseudoElement`](api:hypersig/style/enum.PseudoElement.html) は MDN の CSS データから生成しており、`Before`、`After`、`Placeholder`、`Marker`、`Selection`、`FileSelectorButton`、`Backdrop` など、標準の擬似要素を持っています。",
            },
        ),
        show(Snippet::PseudoElements),
        para(
            site,
            L {
                en: "Writing `.more`, `.field`, and `.step` for the link, the field, and the list item, the browser applies:",
                ja: "リンク、入力欄、リスト項目をそれぞれ `.more`、`.field`、`.step` と書くと、ブラウザが適用するのは次の CSS です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ".more::after { content: \" →\" }\n\
             .more:hover::after { color: red }\n\
             .field::placeholder { color: gray }\n\
             .step::marker { color: #2563eb }",
        ),
        heading(
            site,
            L {
                en: "Pseudo-classes inside a pseudo-element",
                ja: "擬似要素の中の擬似クラス",
            },
        ),
        para(
            site,
            L {
                en: "A pseudo-class always applies to the element, never to the pseudo-element. `pseudo(After, Style::new().on(Hover, ..))` and `on(Hover, Style::new().pseudo(After, ..))` both become `:hover::after`, which matches while the pointer is over the element.",
                ja: "擬似クラスは常に要素に対して適用され、擬似要素には適用されません。`pseudo(After, Style::new().on(Hover, ..))` と `on(Hover, Style::new().pseudo(After, ..))` はどちらも `:hover::after` になり、ポインタが要素の上にある間に当てはまります。",
            },
        ),
        heading(
            site,
            L {
                en: "A pseudo-element is not a condition",
                ja: "擬似要素は条件ではない",
            },
        ),
        para(
            site,
            L {
                en: "`on` takes a `Condition`, and `PseudoElement` does not convert into one, because a pseudo-element is a part of the element and not a state it is in. Passing one to `on` is a compile error:",
                ja: "`on` は `Condition` を受け取りますが、`PseudoElement` は `Condition` に変換できません。擬似要素は要素の一部であって、要素の状態ではないからです。`on` に渡すとコンパイルエラーになります。",
            },
        ),
        show(Snippet::PseudoElementCondition),
        heading(
            site,
            L {
                en: "One pseudo-element deep",
                ja: "擬似要素は 1 段まで",
            },
        ),
        para(
            site,
            L {
                en: "CSS has no pseudo-element of a pseudo-element, so `pseudo` panics when the style it is given already puts a declaration on one. The panic happens where the style is built, so a test that builds the page finds it:",
                ja: "CSS には擬似要素の擬似要素がないので、渡されたスタイルがすでに擬似要素に宣言を付けていると `pseudo` はパニックします。パニックはスタイルを組み立てる場所で起きるので、ページを組み立てるテストで見つかります。",
            },
        ),
        show(Snippet::PseudoElementNested),
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
                    en: "`::before` and `::after` are drawn only when `content` is set. Without it, the other declarations compile and nothing appears.",
                    ja: "`::before` と `::after` は `content` を設定したときだけ描画されます。設定しなければ、ほかの宣言はコンパイルが通っても何も表示されません。",
                },
                L {
                    en: "Nothing checks that the element has the pseudo-element. `::placeholder` on a `<div>` or `::marker` on an element that is not a list item applies to nothing.",
                    ja: "要素にその擬似要素があるかどうかは検査しません。`<div>` の `::placeholder` や、リスト項目でない要素の `::marker` は何にも適用されません。",
                },
                L {
                    en: "Some pseudo-elements accept only a few properties. `::marker` takes `color`, `content`, and the font properties, among others, and the browser ignores the rest.",
                    ja: "一部の擬似要素は限られたプロパティしか受け付けません。`::marker` が受け付けるのは `color`、`content`、フォントのプロパティなどで、それ以外はブラウザが無視します。",
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
                en: "[Media queries](style/media/) applies a style from a viewport width or under a user setting. [URLs and strings](style/urls-and-strings/) covers `quoted`, which builds the value of `content`.",
                ja: "[メディアクエリ](style/media/)ではビューポートの幅やユーザーの設定に応じてスタイルを適用します。`content` の値を作る `quoted` は [URL と文字列](style/urls-and-strings/)で説明しています。",
            },
        ),
    ])
}
