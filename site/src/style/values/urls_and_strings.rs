//! The Style page on `url` and `quoted`: the URLs `url` accepts, how a path
//! follows the page's base, and how a CSS string is escaped.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::UrlsAndStrings).leads()),
        urls(site),
        refused(site),
        base(site),
        strings(site),
        example(site),
    ])
}

fn urls(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "URLs",
                ja: "URL",
            },
        ),
        para(
            site,
            L {
                en: "[`const fn url(href: &'static str) -> Url`](api:hypersig/style/value/fn.url.html) names a file the page loads, and [`Url`](api:hypersig/style/value/struct.Url.html) writes it as `url(\"...\")`. `background_image`, `list_style_image`, `mask_image`, `content`, and the other properties that load an image take it, as does `FontFace::new`, which [Fonts](style/fonts/) covers. `Url::as_str` returns the text as written in the program.",
                ja: "[`const fn url(href: &'static str) -> Url`](api:hypersig/style/value/fn.url.html) はページが読み込むファイルを指し、[`Url`](api:hypersig/style/value/struct.Url.html) はそれを `url(\"...\")` の形で書きます。`background_image`、`list_style_image`、`mask_image`、`content` など画像を読み込むプロパティがこれを受け取り、[フォント](style/fonts/)で説明する `FontFace::new` も受け取ります。`Url::as_str` はプログラムに書いたとおりのテキストを返します。",
            },
        ),
        show(Snippet::StyleUrls),
        para(
            site,
            L {
                en: "`url` accepts a path the application serves, such as `/paper.png`, a URL relative to the page, such as `paper.png`, and an `http` or `https` URL on another host, including one that starts with `//`.",
                ja: "`url` が受け付けるのは、`/paper.png` のようにアプリケーションが配信するパス、`paper.png` のようにページからの相対 URL、ほかのホストの `http` か `https` の URL（`//` で始まるものを含む）です。",
            },
        ),
    ])
}

fn refused(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The URLs url refuses",
                ja: "url が受け付けない URL",
            },
        ),
        para(
            site,
            L {
                en: "`url` panics on an empty URL, a scheme other than `http` or `https`, and a space or a control character. That refuses `javascript:`, and also `data:` and `mailto:`. In a `const` the panic is E0080, so a URL written as a constant is checked by `cargo build`:",
                ja: "`url` は、空の URL、`http` と `https` 以外のスキーム、空白や制御文字を含む URL を受け取るとパニックします。これにより `javascript:` も `data:` も `mailto:` も受け付けません。`const` の中ではこのパニックが E0080 になるので、定数として書いた URL は `cargo build` で検査されます。",
            },
        ),
        show(Snippet::StyleUrlScript),
        para(
            site,
            L {
                en: "Outside a `const`, the panic happens when the line runs. `url` takes only `&'static str`, text written in the program, so a URL built at run time is E0308:",
                ja: "`const` の外ではその行を実行したときにパニックします。`url` が受け取るのはプログラムに書いたテキストである `&'static str` だけなので、実行時に組み立てた URL は E0308 になります。",
            },
        ),
        show(Snippet::StyleUrlString),
        para(
            site,
            L {
                en: "The compiler's hint, `url(&avatar)`, leads to E0597 instead, because `url` needs text that lives for the whole program.",
                ja: "コンパイラが示す `url(&avatar)` に直すと、今度は E0597 になります。`url` が必要とするのは、プログラムの終わりまで有効なテキストだからです。",
            },
        ),
        para(
            site,
            L {
                en: "Each distinct URL is a declaration of its own, with its own rule in the stylesheet, so a URL per item would add one rule per item. An image that differs per item, such as an avatar, belongs in an `<img>` whose `El::src` takes a [`SafeUrl`](learn/unchecked-values/).",
                ja: "異なる URL はそれぞれ別の宣言になり、スタイルシートにそれぞれのルールができるので、項目ごとに URL を変えると、項目の数だけルールが増えます。アバターのように項目ごとに違う画像は `<img>` に置き、その `El::src` に [`SafeUrl`](learn/unchecked-values/) を渡します。",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks that the file exists or what the host serves. A path no service answers loads nothing, the element shows no image, and the browser's console shows the 404.",
                ja: "ファイルがあるかどうかや、ホストが何を返すかは検査されません。どのサービスも応答しないパスでは何も読み込まれず、要素には画像が表示されず、ブラウザのコンソールに 404 が表示されます。",
            },
        ),
    ])
}

fn base(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Paths under a base",
                ja: "ベースの下のパス",
            },
        ),
        para(
            site,
            L {
                en: "A path that starts with a single `/` is written under the [`Base`](learn/base-path/) the page is served under, in the first render and in every patch. Under `Base::new(\"/app\")`, `url(\"/paper.png\")` loads `/app/paper.png`. A relative URL follows the page's own path, and a URL on another host is written as given:",
                ja: "1 つの `/` で始まるパスはページを配信する [`Base`](learn/base-path/) の下に書かれます。最初の描画でも、どのパッチでも同じです。`Base::new(\"/app\")` の下では、`url(\"/paper.png\")` は `/app/paper.png` を読み込みます。相対 URL はページ自身のパスをたどり、ほかのホストの URL はそのまま書かれます。",
            },
        ),
        show(Snippet::StyleUrlBase),
    ])
}

fn strings(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Strings",
                ja: "文字列",
            },
        ),
        para(
            site,
            L {
                en: "[`fn quoted(text: impl Into<String>) -> Quoted`](api:hypersig/style/value/fn.quoted.html) writes text as a double-quoted CSS string. `content`, `font_family`, `list_style_type`, `text_overflow`, and the other properties that take a string take a [`Quoted`](api:hypersig/style/value/struct.Quoted.html). It escapes every character that could end the string or the stylesheet:",
                ja: "[`fn quoted(text: impl Into<String>) -> Quoted`](api:hypersig/style/value/fn.quoted.html) はテキストを二重引用符で囲んだ CSS の文字列として書きます。`content`、`font_family`、`list_style_type`、`text_overflow` など文字列を取るプロパティは [`Quoted`](api:hypersig/style/value/struct.Quoted.html) を受け取ります。文字列やスタイルシートを終わらせうる文字は、すべてエスケープされます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Character", "Written", "Why"],
                ja: &["文字", "書き方", "理由"],
            },
            L {
                en: &[
                    &["`\"`", "`\\\"`", "It would end the string"],
                    &["A backslash", "Two backslashes", "It would start an escape"],
                    &[
                        "`<`",
                        "`\\3c `",
                        "It could close the `<style>` element the rules are written into",
                    ],
                    &[
                        "A control character, such as a newline",
                        "A hex escape, such as `\\a `",
                        "A newline would end the string, and the text after it would be read as rules",
                    ],
                ],
                ja: &[
                    &["`\"`", "`\\\"`", "文字列を終わらせてしまうため"],
                    &[
                        "バックスラッシュ",
                        "バックスラッシュ 2 つ",
                        "エスケープを始めてしまうため",
                    ],
                    &[
                        "`<`",
                        "`\\3c `",
                        "ルールを書く `<style>` 要素を閉じうるため",
                    ],
                    &[
                        "改行などの制御文字",
                        "`\\a ` のような 16 進のエスケープ",
                        "改行は文字列を終わらせ、その後のテキストがルールとして読まれてしまうため",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Unlike `url`, `quoted` takes a `String` built at run time, and any text is safe in it. Each distinct text is still a rule of its own.",
                ja: "`url` と違い、`quoted` は実行時に組み立てた `String` も受け取り、どんなテキストでも安全に入れられます。ただし、異なるテキストはそれぞれ別のルールになります。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: links to other sites",
                ja: "例: ほかのサイトへのリンク",
            },
        ),
        para(
            site,
            L {
                en: "The list marks each item with the string `\"→ \"` in place of a bullet, and each link ends with an icon loaded by `content: url(...)` on its `::after` pseudo-element, which [Pseudo-elements](style/pseudo-elements/) covers:",
                ja: "このリストは、項目の印に黒丸ではなく文字列 `\"→ \"` を使います。各リンクの末尾には、`::after` 擬似要素の `content: url(...)` で読み込むアイコンが付きます。擬似要素の書き方は[擬似要素](style/pseudo-elements/)で説明します。",
            },
        ),
        show(Snippet::StyleLinksList),
        para(
            site,
            L {
                en: "Next, [CSS-wide keywords](style/keywords/) covers `inherit` and the other keywords every property accepts.",
                ja: "次の[CSS 全体のキーワード](style/keywords/)では `inherit` など、どのプロパティでも使えるキーワードを説明します。",
            },
        ),
    ])
}
