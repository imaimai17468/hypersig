//! The page on styles under a pseudo-class such as `:hover`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::PseudoClasses).leads()),
        heading(
            site,
            L {
                en: "A style under a pseudo-class",
                ja: "擬似クラスの下のスタイル",
            },
        ),
        para(
            site,
            L {
                en: "[`Style::on`](api:hypersig/style/struct.Style.html#method.on), `fn on(self, condition: impl Into<Condition>, style: Style) -> Style`, adds the declarations of `style` and applies them only while `condition` holds. A [`PseudoClass`](api:hypersig/style/enum.PseudoClass.html) converts into a [`Condition`](api:hypersig/style/enum.Condition.html), so it can be passed as it is. The button below changes its background while hovered and pressed, shows an outline while focused from the keyboard, and dims while disabled:",
                ja: "[`Style::on`](api:hypersig/style/struct.Style.html#method.on)（`fn on(self, condition: impl Into<Condition>, style: Style) -> Style`）は `style` の宣言を追加し、`condition` が成り立つ間だけ適用します。[`PseudoClass`](api:hypersig/style/enum.PseudoClass.html) は [`Condition`](api:hypersig/style/enum.Condition.html) に変換できるので、そのまま渡せます。下のボタンは、ホバー中と押している間に背景が変わり、キーボードでフォーカスしたときに輪郭線を表示し、無効の間は薄くなります。",
            },
        ),
        show(Snippet::PseudoClasses),
        para(
            site,
            L {
                en: "Writing `.save` for the button, the browser applies these declarations as this CSS would:",
                ja: "ボタンを `.save` と書くと、ブラウザはこれらの宣言を次の CSS と同じように適用します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ".save { padding: 0.5rem; background-color: #2563eb; color: white; cursor: pointer }\n\
             .save:hover { background-color: #1d4ed8 }\n\
             .save:focus-visible { outline-width: 2px; outline-style: solid }\n\
             .save:active { background-color: #1e3a8a }\n\
             .save:disabled { opacity: 0.5; cursor: not-allowed }",
        ),
        heading(
            site,
            L {
                en: "Which pseudo-class wins",
                ja: "どの擬似クラスが優先されるか",
            },
        ),
        para(
            site,
            L {
                en: "A pressed button is hovered too, so `:hover` and `:active` both match it. The rules for the user-action pseudo-classes are written in a fixed order, whatever the order of the `on` calls, and the later rule wins where two set the same property:",
                ja: "押しているボタンはホバーもされているので、`:hover` と `:active` の両方に当てはまります。ユーザー操作の擬似クラスのルールは、`on` を呼んだ順に関係なく決まった順に書かれ、2 つのルールが同じプロパティを設定する場合はあとのルールが優先されます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Order", "Pseudo-classes"],
                ja: &["順番", "擬似クラス"],
            },
            L {
                en: &[
                    &["First", "Every pseudo-class not listed below"],
                    &[
                        "Then",
                        "`Link`, `Visited`, `FocusWithin`, `Hover`, `Focus`, `FocusVisible`, `Active`",
                    ],
                    &["Last", "`Disabled`"],
                ],
                ja: &[
                    &["最初", "下に挙げていないすべての擬似クラス"],
                    &[
                        "次",
                        "`Link`、`Visited`、`FocusWithin`、`Hover`、`Focus`、`FocusVisible`、`Active`",
                    ],
                    &["最後", "`Disabled`"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "So `:active` wins over `:hover` while the button is pressed, and `:disabled` wins over all of them. The order follows the link, visited, hover, focus, active order that CSS authors use for links.",
                ja: "そのため、ボタンを押している間は `:active` が `:hover` より優先され、`:disabled` はそのすべてより優先されます。この順番は CSS でリンクに使われる link、visited、hover、focus、active の順に沿っています。",
            },
        ),
        heading(
            site,
            L {
                en: "Two conditions at once",
                ja: "2 つの条件を同時に使う",
            },
        ),
        para(
            site,
            L {
                en: "The style passed to `on` can call `on` itself, and its declarations then apply only while both conditions hold. `on(Visited, Style::new().on(Hover, ..))` and `on(Hover, Style::new().on(Visited, ..))` produce the same rule, `:visited:hover`:",
                ja: "`on` に渡すスタイルの中でも `on` を呼べます。その場合、内側の宣言は両方の条件が成り立つ間だけ適用されます。`on(Visited, Style::new().on(Hover, ..))` と `on(Hover, Style::new().on(Visited, ..))` は同じルール `:visited:hover` になります。",
            },
        ),
        show(Snippet::NestedConditions),
        code::block(
            Lang::Html,
            None,
            ".link { color: #2563eb }\n\
             .link:hover { text-decoration-line: underline }\n\
             .link:visited { color: #6d28d9 }\n\
             .link:visited:hover { color: #4c1d95 }",
        ),
        heading(
            site,
            L {
                en: "A misspelled pseudo-class",
                ja: "擬似クラスの綴りの間違い",
            },
        ),
        para(
            site,
            L {
                en: "`PseudoClass` has one variant per standard pseudo-class without arguments that can match an element's class, generated from MDN's CSS data, such as `Hover`, `Checked`, `PlaceholderShown`, `UserInvalid`, and `PopoverOpen`. A pseudo-class that does not exist is a compile error:",
                ja: "`PseudoClass` には、引数を取らない標準の擬似クラスのうち、要素のクラスに当てはまりうるものに 1 つずつバリアントがあり、MDN の CSS データから生成しています。`Hover`、`Checked`、`PlaceholderShown`、`UserInvalid`、`PopoverOpen` などです。存在しない擬似クラスを書くとコンパイルエラーになります。",
            },
        ),
        show(Snippet::PseudoClassTypo),
        para(
            site,
            L {
                en: "Six standard pseudo-classes without arguments have no variant, because a class on an element never matches them. `:root` and `:scope` match only `<html>`, and a [`Global`](style/global/) rule for `Selector::Root` styles that element instead. `:host` matches only from inside a shadow tree, and `:first`, `:left`, and `:right` select a page box in `@page`.",
                ja: "引数を取らない標準の擬似クラスのうち 6 つは、要素のクラスに当てはまることがないので、バリアントがありません。`:root` と `:scope` が当てはまるのは `<html>` だけで、`<html>` には `Selector::Root` を対象にした [`Global`](style/global/) のルールでスタイルを付けます。`:host` はシャドウツリーの内側からしか当てはまらず、`:first`、`:left`、`:right` は `@page` のページボックスを選びます。",
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
                    en: "Nothing checks that the pseudo-class can match the element. `:checked` on a `<div>` or `:placeholder-shown` on a `<button>` compiles, and the declarations never apply.",
                    ja: "擬似クラスがその要素に当てはまりうるかどうかは検査しません。`<div>` に `:checked` を付けても、`<button>` に `:placeholder-shown` を付けてもコンパイルは通り、宣言は一度も適用されません。",
                },
                L {
                    en: "A browser that does not know a pseudo-class, such as an older one with `:open`, drops the rule that uses it. Each rule holds one declaration, so only that declaration is lost, and the element's other declarations still apply.",
                    ja: "`:open` を知らない古いブラウザのように、擬似クラスを知らないブラウザは、それを使うルールを捨てます。ルールには宣言が 1 つしかないので、失われるのはその宣言だけで、要素のほかの宣言は適用されます。",
                },
                L {
                    en: "The pseudo-classes that take an argument, such as `:nth-child()`, `:not()`, and `:has()`, have no variant. A rule that needs one goes in a plain stylesheet, which [Unchecked values](style/unchecked/) covers.",
                    ja: "`:nth-child()`、`:not()`、`:has()` のように引数を取る擬似クラスにはバリアントがありません。それらが必要なルールは普通のスタイルシートに書きます。その方法は[検査しない値](style/unchecked/)で説明しています。",
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
                en: "[Pseudo-elements](style/pseudo-elements/) styles a part of an element, such as `::before` or `::placeholder`. [Group and peer](style/group-and-peer/) applies a style while a pseudo-class matches another element.",
                ja: "[擬似要素](style/pseudo-elements/)では `::before` や `::placeholder` のような要素の一部にスタイルを付けます。[グループとピア](style/group-and-peer/)では別の要素が擬似クラスに当てはまる間だけスタイルを適用します。",
            },
        ),
    ])
}
