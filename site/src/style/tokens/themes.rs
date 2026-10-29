//! The page on themes, which set tokens for the whole page.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Themes).leads()),
        heading(
            site,
            L {
                en: "Values for the whole page",
                ja: "ページ全体の値",
            },
        ),
        para(
            site,
            L {
                en: "A [`Theme`](api:hypersig/style/struct.Theme.html) holds values for tokens, and [`Page::theme`](api:hypersig/struct.Page.html#method.theme), `fn theme(self, theme: Theme) -> Self`, writes them on `:root`, so every element of the page reads them in place of the tokens' defaults. A theme is built with three methods:",
                ja: "[`Theme`](api:hypersig/style/struct.Theme.html) はトークンの値を持ち、[`Page::theme`](api:hypersig/struct.Page.html#method.theme)（`fn theme(self, theme: Theme) -> Self`）はその値を `:root` に書きます。そのため、ページのすべての要素はトークンの既定値の代わりにテーマの値を読みます。テーマは 3 つのメソッドで組み立てます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "What it does"],
                ja: &["メソッド", "すること"],
            },
            L {
                en: &[
                    &[
                        "[`set`](api:hypersig/style/struct.Theme.html#method.set)`<T: TokenValue>(self, token: &Token<T>, value: impl Into<T>) -> Theme`",
                        "Sets `token` to `value`. A later value for the same token under the same conditions replaces an earlier one.",
                    ],
                    &[
                        "[`on`](api:hypersig/style/struct.Theme.html#method.on)`(self, media: Media, theme: Theme) -> Theme`",
                        "Adds the values of `theme`, applied only under `media`.",
                    ],
                    &[
                        "[`merge`](api:hypersig/style/struct.Theme.html#method.merge)`(self, theme: Theme) -> Theme`",
                        "Adds the values of `theme`, which win over this theme's own.",
                    ],
                ],
                ja: &[
                    &[
                        "[`set`](api:hypersig/style/struct.Theme.html#method.set)`<T: TokenValue>(self, token: &Token<T>, value: impl Into<T>) -> Theme`",
                        "`token` に `value` を設定します。同じ条件の下で同じトークンにあとから値を設定すると、前の値を置き換えます。",
                    ],
                    &[
                        "[`on`](api:hypersig/style/struct.Theme.html#method.on)`(self, media: Media, theme: Theme) -> Theme`",
                        "`theme` の値を、`media` の下でだけ適用するものとして追加します。",
                    ],
                    &[
                        "[`merge`](api:hypersig/style/struct.Theme.html#method.merge)`(self, theme: Theme) -> Theme`",
                        "`theme` の値を追加します。追加した値はこのテーマ自身の値より優先されます。",
                    ],
                ],
            },
        ),
        show(Snippet::Themes),
        para(
            site,
            L {
                en: "`Page::theme` merges each theme it is given into the earlier ones, so the second call above replaces `--gap` and keeps the rest. The page gets this CSS:",
                ja: "`Page::theme` は、渡されたテーマをそれまでのテーマに `merge` します。そのため、上の 2 回目の呼び出しは `--gap` を置き換え、残りはそのまま保ちます。ページに書かれる CSS は次のとおりです。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ":root { --ink: #111827; --gap: 2rem }\n\
             @media (prefers-color-scheme: dark) { :root { --ink: #f9fafb; --paper: #111827 } }\n\
             @media print { :root { --ink: black } }",
        ),
        heading(
            site,
            L {
                en: "Which value applies",
                ja: "どの値が適用されるか",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The values without a condition come first, then one block per set of media conditions, in the order `Media` lists its variants. Where two blocks match and set the same token, the later block wins, so a printed page above gets black ink in dark mode too.",
                    ja: "条件のない値が最初に書かれ、そのあとにメディアの条件の組み合わせごとに 1 つのブロックが、`Media` がバリアントを並べる順に続きます。2 つのブロックが当てはまり、同じトークンを設定する場合はあとのブロックが優先されます。そのため上の例では、ダークモードでも印刷したページのインクは黒になります。",
                },
                L {
                    en: "A value set on an element with `Style::token` or `El::bind_token` wins over the theme for that element and those inside it, because the element is closer than `:root`.",
                    ja: "`Style::token` や `El::bind_token` で要素に設定した値は、その要素とその内側の要素についてテーマより優先されます。その要素のほうが `:root` より近いからです。",
                },
                L {
                    en: "`on` nests: `on(Media::Dark, Theme::new().on(Media::Print, ..))` applies while both hold, written `@media print and (prefers-color-scheme: dark)`.",
                    ja: "`on` は入れ子にできます。`on(Media::Dark, Theme::new().on(Media::Print, ..))` は両方が成り立つときに適用され、`@media print and (prefers-color-scheme: dark)` と書かれます。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Only media conditions",
                ja: "使えるのはメディアの条件だけ",
            },
        ),
        para(
            site,
            L {
                en: "A theme sets values for the whole document on `:root`, which has no container above it, so `Theme::on` takes a `Media` and nothing else. A pseudo-class is a compile error:",
                ja: "テーマは文書全体の値を `:root` に設定し、`:root` の上にはコンテナがありません。そのため、`Theme::on` が受け取るのは `Media` だけです。擬似クラスを渡すとコンパイルエラーになります。",
            },
        ),
        show(Snippet::ThemeCondition),
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
                    en: "Nothing checks that a theme sets every token a page reads. A token it leaves out shows its default.",
                    ja: "ページが読むトークンをテーマがすべて設定しているかどうかは検査しません。テーマが設定しなかったトークンには既定値が使われます。",
                },
                L {
                    en: "A plain stylesheet added with `Page::style` sits outside the cascade layers of the theme, so a `:root` rule in it that sets the same custom property wins over the theme.",
                    ja: "`Page::style` で追加した普通のスタイルシートはテーマのカスケードレイヤーの外にあるので、その中で同じカスタムプロパティを設定する `:root` のルールはテーマより優先されます。",
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
                en: "[Dark mode](style/dark-mode/) uses a theme to follow the system's color scheme, and a style on `<body>` to let the visitor pick one. [Theming](components/theming/) covers the theme of `hypersig_ui`.",
                ja: "[ダークモード](style/dark-mode/)ではテーマでシステムの配色に従い、`<body>` に付けるスタイルで訪問者に配色を選ばせます。`hypersig_ui` のテーマは Components の[テーマ](components/theming/)のページで説明しています。",
            },
        ),
    ])
}
