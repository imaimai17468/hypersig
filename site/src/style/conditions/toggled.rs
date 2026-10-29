//! The page on styles that a signal switches on and off.

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
        lead(site, Doc::Style(StylePage::Toggled).leads()),
        heading(
            site,
            L {
                en: "A style while an expression is true",
                ja: "式が真の間だけのスタイル",
            },
        ),
        para(
            site,
            L {
                en: "[`El::css_when`](api:hypersig/html/struct.El.html#method.css_when), `fn css_when(self, style: Style, when: impl IntoExpr<S, bool>) -> Self`, adds the declarations of `style` to the element while `when` is true in the browser. `when` is a `bool` signal or any expression over the signals that yields a `bool`, and it changes without a request to the server. The first HTML already has the style when `when` is true for the initial signals.",
                ja: "[`El::css_when`](api:hypersig/html/struct.El.html#method.css_when)（`fn css_when(self, style: Style, when: impl IntoExpr<S, bool>) -> Self`）は、ブラウザで `when` が真の間、`style` の宣言を要素に追加します。`when` は `bool` のシグナルか、シグナルから `bool` を作る任意の式で、サーバーへのリクエストなしに切り替わります。初期のシグナルで `when` が真なら、最初の HTML からそのスタイルが付いています。",
            },
        ),
        show(Snippet::CssWhen),
        para(
            site,
            L {
                en: "The style may use conditions of its own, such as the `Hover` above. Writing `.item` for the list item, while `done` is true the browser applies:",
                ja: "スタイルは上の `Hover` のように自分の条件を使えます。リスト項目を `.item` と書くと、`done` が真の間にブラウザが適用するのは次の CSS です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ".item { padding: 0.5rem; color: black }\n\
             .item { color: gray; text-decoration-line: line-through } /* wins over color: black */\n\
             .item:hover { opacity: 0.8 }",
        ),
        heading(
            site,
            L {
                en: "Which declaration wins",
                ja: "どの宣言が優先されるか",
            },
        ),
        para(
            site,
            L {
                en: "The rules of `css_when` come after every rule of `El::css` in the cascade, so while `when` is true they override the element's own declarations for the same property, including the ones under a condition such as `:hover`. When `when` turns false, the element's own declarations apply again.",
                ja: "`css_when` のルールはカスケードの中で `El::css` のどのルールよりもあとに来ます。そのため `when` が真の間は、`:hover` のような条件の下にあるものも含めて、同じプロパティについて要素自身の宣言を上書きします。`when` が偽になると、要素自身の宣言がまた適用されます。",
            },
        ),
        para(
            site,
            L {
                en: "Two styles switched on at once that set the same property are not checked against each other. The one whose rule comes later in the stylesheet wins, which depends on the conditions and the property, not on the order of the calls. Give two styles that can hold at once different properties, or use `css_match` below.",
                ja: "同時に有効になった 2 つのスタイルが同じプロパティを設定しても、互いに検査はしません。優先されるのはスタイルシートであとに来るルールで、それは条件とプロパティで決まり、呼び出しの順には左右されません。同時に有効になりうる 2 つのスタイルには別々のプロパティを設定するか、下の `css_match` を使ってください。",
            },
        ),
        heading(
            site,
            L {
                en: "One style per value",
                ja: "値ごとに 1 つのスタイル",
            },
        ),
        para(
            site,
            L {
                en: "[`El::css_match`](api:hypersig/html/struct.El.html#method.css_match), `fn css_match<T: PartialEq + Serialize>(self, value: impl IntoExpr<S, T>, cases: impl IntoIterator<Item = (T, Style)>) -> Self`, adds the style of the case whose value equals `value`. At most one case applies at a time, so two cases never compete for a property. This is the variant half of a recipe: the base goes in `El::css`, and each case adds what one value changes.",
                ja: "[`El::css_match`](api:hypersig/html/struct.El.html#method.css_match)（`fn css_match<T: PartialEq + Serialize>(self, value: impl IntoExpr<S, T>, cases: impl IntoIterator<Item = (T, Style)>) -> Self`）は、値が `value` と等しいケースのスタイルを追加します。一度に適用されるケースは 1 つまでなので、2 つのケースが同じプロパティを取り合うことはありません。レシピでいえばバリアントの部分にあたります。基本のスタイルは `El::css` に書き、ケースごとに、その値で変わる部分だけを追加します。",
            },
        ),
        show(Snippet::CssMatch),
        para(
            site,
            L {
                en: "The badge below takes its color from `css_match` over an enum:",
                ja: "下のバッジは、列挙型に対する `css_match` で色が決まります。",
            },
        ),
        demo::show(site, Demo::Status),
        heading(
            site,
            L {
                en: "When the server knows the value",
                ja: "値がサーバーで決まる場合",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks that the cases of `css_match` cover every value of `T`. A value with no case adds nothing. Where the value is known on the server and does not change in the browser, call a function that returns a `Style` from a `match` instead, which the compiler checks for every variant:",
                ja: "`css_match` のケースが `T` のすべての値を網羅しているかどうかは検査しません。どのケースにも当たらない値は何も追加しません。値がサーバーで決まり、ブラウザで変わらない場合は、代わりに `match` で `Style` を返す関数を呼びます。そうすれば、すべてのバリアントをコンパイラが検査します。",
            },
        ),
        show(Snippet::ServerMatch),
        heading(
            site,
            L {
                en: "The condition is an expression of type bool",
                ja: "条件は bool の式",
            },
        ),
        para(
            site,
            L {
                en: "`when` must be an expression of type `bool` on the page's signals. A signal of another type is a compile error, so compare it to a value first, as with `Todo::title().is_empty()`:",
                ja: "`when` はページのシグナルから作る `bool` の式でなければなりません。ほかの型のシグナルを渡すとコンパイルエラーになるので、`Todo::title().is_empty()` のように、先に値と比べてください。",
            },
        ),
        show(Snippet::CssWhenType),
        heading(
            site,
            L {
                en: "On the whole page",
                ja: "ページ全体に付ける",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::css_when`](api:hypersig/struct.Page.html#method.css_when) and [`Page::css_match`](api:hypersig/struct.Page.html#method.css_match) do the same on `<body>`. Tokens set this way reach every element of the page and the background of `<body>` itself, which a style on an element inside it cannot reach. [Dark mode](style/dark-mode/) uses them for a color scheme picker.",
                ja: "[`Page::css_when`](api:hypersig/struct.Page.html#method.css_when) と [`Page::css_match`](api:hypersig/struct.Page.html#method.css_match) は、同じことを `<body>` に対して行います。こうして設定したトークンは、ページのすべての要素と `<body>` 自身の背景に届きます。`<body>` の内側の要素に付けたスタイルでは、`<body>` の背景には届きません。[ダークモード](style/dark-mode/)ではこれを配色の切り替えに使います。",
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
                    en: "Two styles of `css_when` that hold at once and set the same property are not checked against each other, as above.",
                    ja: "上で説明したとおり、同時に有効になった `css_when` の 2 つのスタイルが同じプロパティを設定しても、互いに検査はしません。",
                },
                L {
                    en: "The cases of `css_match` are not checked for covering every value of `T`, and a value with no case adds nothing.",
                    ja: "`css_match` のケースが `T` のすべての値を網羅しているかどうかは検査せず、どのケースにも当たらない値は何も追加しません。",
                },
                L {
                    en: "For a value that no fixed set holds, such as a width in percent, `El::bind_style` and `El::bind_token` set a property from a string, which is not checked. [Tokens](style/tokens/) and [Unchecked values](style/unchecked/) cover them.",
                    ja: "パーセントで表す幅のように決まった候補のない値には、`El::bind_style` と `El::bind_token` が文字列からプロパティを設定します。この文字列は検査しません。これらは[トークン](style/tokens/)と[検査しない値](style/unchecked/)で説明しています。",
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
                en: "[Tokens](style/tokens/) names a value that several styles read, so one change of a signal can restyle many elements.",
                ja: "[トークン](style/tokens/)では、いくつものスタイルが読む値に名前を付けます。シグナルを 1 回変えるだけで、たくさんの要素のスタイルを変えられます。",
            },
        ),
    ])
}
