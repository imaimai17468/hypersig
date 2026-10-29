//! The page on loading fonts with `FontFace`.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Fonts).leads()),
        heading(
            site,
            L {
                en: "Loading a font file",
                ja: "フォントのファイルを読み込む",
            },
        ),
        para(
            site,
            L {
                en: "A [`FontFace`](api:hypersig/style/struct.FontFace.html) is one font file of a family, written as an `@font-face` rule. `Global::font` adds it to a `Global`, and [`FontFace::family`](api:hypersig/style/struct.FontFace.html#method.family), `fn family(&self) -> FontFamily`, returns the value `font-family` takes, so the name in the rule and the name in the style are the same string:",
                ja: "[`FontFace`](api:hypersig/style/struct.FontFace.html) はファミリーの 1 つのフォントファイルで、`@font-face` のルールとして書かれます。`Global::font` で `Global` に追加し、[`FontFace::family`](api:hypersig/style/struct.FontFace.html#method.family)（`fn family(&self) -> FontFamily`）が `font-family` に渡す値を返します。そのため、ルールの中の名前とスタイルの中の名前は同じ文字列になります。",
            },
        ),
        show(Snippet::Fonts),
        para(
            site,
            L {
                en: "Besides the reset, the page gets:",
                ja: "リセットのほかに、ページには次の CSS が書かれます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "body { font-family: \"Inter\"; line-height: 1.5 }\n\
             code { font-family: \"JetBrains Mono\", monospace }\n\
             kbd { font-family: ui-monospace }\n\
             @font-face { font-family: \"Inter\"; src: url(\"/fonts/inter.woff2\"); font-display: swap; font-weight: 100 900 }\n\
             @font-face { font-family: \"Inter\"; src: url(\"/fonts/inter-italic.woff2\"); font-display: swap; font-weight: 100 900; font-style: italic }",
        ),
        heading(
            site,
            L {
                en: "The methods",
                ja: "メソッド",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "What it writes"],
                ja: &["メソッド", "書くもの"],
            },
            L {
                en: &[
                    &[
                        "[`new`](api:hypersig/style/struct.FontFace.html#method.new)`(family: &'static str, src: Url) -> FontFace`",
                        "`font-family` as a quoted string, `src: url(..)`, and `font-display: swap`. The browser recognizes the format from the file, and WOFF2 is the usual choice.",
                    ],
                    &[
                        "[`weights`](api:hypersig/style/struct.FontFace.html#method.weights)`(self, min: u16, max: u16) -> FontFace`",
                        "`font-weight: <min> <max>`, the range a variable font covers. One weight is `weights(400, 400)`.",
                    ],
                    &[
                        "[`italic`](api:hypersig/style/struct.FontFace.html#method.italic)`(self) -> FontFace`",
                        "`font-style: italic`, which marks the file as the italic face of the family.",
                    ],
                ],
                ja: &[
                    &[
                        "[`new`](api:hypersig/style/struct.FontFace.html#method.new)`(family: &'static str, src: Url) -> FontFace`",
                        "引用符で囲んだ文字列の `font-family`、`src: url(..)`、`font-display: swap` を書きます。形式はブラウザがファイルから判断し、ふつうは WOFF2 を使います。",
                    ],
                    &[
                        "[`weights`](api:hypersig/style/struct.FontFace.html#method.weights)`(self, min: u16, max: u16) -> FontFace`",
                        "可変フォントが対応する範囲として `font-weight: <min> <max>` を書きます。1 つの太さだけなら `weights(400, 400)` です。",
                    ],
                    &[
                        "[`italic`](api:hypersig/style/struct.FontFace.html#method.italic)`(self) -> FontFace`",
                        "`font-style: italic` を書き、ファイルをそのファミリーのイタリック体として扱わせます。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Two faces with the same family name and different `font-style` are one family to the browser. `<em>` above is drawn from the italic file, and the rest from the upright one.",
                ja: "ファミリー名が同じで `font-style` の違う 2 つのフェイスは、ブラウザにとっては 1 つのファミリーです。上の `<em>` はイタリック体のファイルで描かれ、ほかは立体のファイルで描かれます。",
            },
        ),
        heading(
            site,
            L {
                en: "The URL of the file",
                ja: "ファイルの URL",
            },
        ),
        para(
            site,
            L {
                en: "`src` is a [`Url`](api:hypersig/style/value/struct.Url.html) made by [`url`](api:hypersig/style/value/fn.url.html), `const fn url(href: &'static str) -> Url`, so a plain string is a compile error:",
                ja: "`src` は [`url`](api:hypersig/style/value/fn.url.html)（`const fn url(href: &'static str) -> Url`）で作る [`Url`](api:hypersig/style/value/struct.Url.html) なので、ただの文字列を渡すとコンパイルエラーになります。",
            },
        ),
        show(Snippet::FontSrc),
        para(
            site,
            L {
                en: "`url` panics on an empty URL, a scheme other than `http` or `https`, or a space or a control character. In a `const`, the panic is a compile error:",
                ja: "`url` は、URL が空のとき、`http` と `https` 以外のスキームを持つとき、空白や制御文字を含むときにパニックします。`const` の中では、このパニックはコンパイルエラーになります。",
            },
        ),
        show(Snippet::FontUrl),
        para(
            site,
            L {
                en: "A path that starts with a single `/` names a file the application serves, and the page writes it under the base its routes are declared under, so under `Base::new(\"/app\")` the file `/fonts/inter.woff2` loads from `/app/fonts/inter.woff2`. [URLs and strings](style/urls-and-strings/) covers `url` in full.",
                ja: "1 つの `/` で始まるパスはアプリケーションが配信するファイルを指し、ページはそれをルートを宣言したベースの下に書きます。そのため `Base::new(\"/app\")` の下では、ファイル `/fonts/inter.woff2` は `/app/fonts/inter.woff2` から読み込まれます。`url` の詳細は [URL と文字列](style/urls-and-strings/)で説明しています。",
            },
        ),
        heading(
            site,
            L {
                en: "Fallback fonts",
                ja: "代わりのフォント",
            },
        ),
        para(
            site,
            L {
                en: "A `FontFamily` holds one family: a generic keyword such as `FontFamily::SystemUi`, or the quoted name `family()` returns. A list such as `\"JetBrains Mono\", monospace` goes through `raw`, and nothing checks it. [Unchecked values](style/unchecked/) covers `raw`.",
                ja: "`FontFamily` が持つのは 1 つのファミリーで、`FontFamily::SystemUi` のような総称のキーワードか、`family()` が返す引用符付きの名前です。`\"JetBrains Mono\", monospace` のような並びは `raw` で書き、検査はしません。`raw` は[検査しない値](style/unchecked/)で説明しています。",
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
                    en: "Nothing checks that the file exists. While the file loads, and for good if it does not, `font-display: swap` shows the text in the next font of the list, or in the browser's default font.",
                    ja: "ファイルが存在するかどうかは検査しません。ファイルを読み込んでいる間と、読み込めなかった場合はその後もずっと、`font-display: swap` によってテキストは並びの次のフォントか、ブラウザの既定のフォントで表示されます。",
                },
                L {
                    en: "The weight range, at run time. `weights` panics where it is called on a range in reverse, such as `weights(900, 100)`, or on a weight outside the 1 to 1000 that CSS allows. It is not a `const fn`, so this is a panic when the line runs, not a compile error.",
                    ja: "太さの範囲（実行時）。`weights` は、`weights(900, 100)` のような逆向きの範囲や、CSS が認める 1 から 1000 の外にある太さを受け取ると、呼んだところで panic します。`const fn` ではないので、これはコンパイルエラーではなく、その行を実行したときの panic です。",
                },
                L {
                    en: "The browser downloads a font only when text on the page uses its family. A `FontFace` that no rule or style names costs nothing, and nothing reports it.",
                    ja: "ブラウザがフォントをダウンロードするのは、ページのテキストがそのファミリーを使うときだけです。どのルールやスタイルからも使われない `FontFace` は何も読み込まず、そのことを知らせるものもありません。",
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
                en: "[Stacks](style/stacks/) starts the Layout pages, with the functions that return the styles of common layouts.",
                ja: "[スタック](style/stacks/)からはレイアウトのページで、よく使うレイアウトのスタイルを返す関数を説明します。",
            },
        ),
    ])
}
