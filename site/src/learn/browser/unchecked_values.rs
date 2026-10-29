//! The page on the values that reach the browser with a checked type but
//! unchecked content: URLs outside the router, attribute values, CSS values,
//! and the paths of files a page loads.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::UncheckedValues).title(site)),
        lead(site, Doc::Learn(Learn::UncheckedValues).leads()),
        para(
            site,
            L {
                en: "Each value on this page has a type, so it cannot be passed where another kind of value belongs. What the type does not check is the text inside: which host a URL names, whether an attribute value means anything to the browser, or whether the router serves a file. Each section names the check that runs, what is left, and what the browser does with a wrong value. [What the compiler does not check](learn/why/#what-the-compiler-does-not-check) lists the mistakes that make rendering panic.",
                ja: "このページで扱う値にはどれも型があるので、別の種類の値を渡すところには渡せません。型が検査しないのは中身のテキストです。URL がどのホストを指すか、属性の値がブラウザにとって意味を持つか、ルーターがそのファイルを配信しているかは検査されません。各節では行われる検査、残るもの、間違った値をブラウザがどう扱うかを説明します。描画が panic するミスは [コンパイラが検査しないこと](learn/why/#what-the-compiler-does-not-check) にまとめてあります。",
            },
        ),
        heading(
            site,
            L {
                en: "Links the router does not serve",
                ja: "ルーターが配信しないページへのリンク",
            },
        ),
        para(
            site,
            L {
                en: "`El::href` and `El::src` take `url: impl IntoHref<S>`: a link built from a [`Route`](learn/routes/), or a [`SafeUrl`](api:hypersig/struct.SafeUrl.html) for an address the router does not serve, such as another site or a `#fragment` on the same page. `El::poster` takes the same, and `El::srcset` takes pairs of a `SafeUrl` and a width or a pixel density. A `SafeUrl` holds only a URL whose scheme cannot run a script: `http:`, `https:`, `mailto:`, or none at all, as in `/logo.png`, `#top`, or `?page=2`.",
                ja: "`El::href` と `El::src` は `url: impl IntoHref<S>` を受け取ります。[`Route`](learn/routes/) から作ったリンクか、ルーターが配信しないアドレスのための [`SafeUrl`](api:hypersig/struct.SafeUrl.html) です。ほかのサイトや、同じページの `#fragment` へのリンクが後者にあたります。`El::poster` も同じものを受け取り、`El::srcset` は `SafeUrl` と幅またはピクセル密度の組を受け取ります。`SafeUrl` に入るのは、スクリプトを実行できないスキーム（`http:`、`https:`、`mailto:`）の URL と、`/logo.png`、`#top`、`?page=2` のようにスキームのない URL だけです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "Signature", "On a refused URL"],
                ja: &["関数", "シグネチャ", "受け付けない URL のとき"],
            },
            L {
                en: &[
                    &[
                        "[`SafeUrl::new`](api:hypersig/struct.SafeUrl.html#method.new)",
                        "`const fn new(url: &'static str) -> SafeUrl`",
                        "Panics, and in a `const` fails to compile",
                    ],
                    &[
                        "[`SafeUrl::parse`](api:hypersig/struct.SafeUrl.html#method.parse)",
                        "`fn parse(url: impl Into<String>) -> Result<SafeUrl, UnsafeUrl>`",
                        "Returns an [`UnsafeUrl`](api:hypersig/struct.UnsafeUrl.html), which names the rule the URL broke",
                    ],
                    &[
                        "`Deserialize for SafeUrl`",
                        "A field of a page's signals",
                        "Fails as `SafeUrl::parse` does, so a page's signals cannot hold a `javascript:` URL",
                    ],
                ],
                ja: &[
                    &[
                        "[`SafeUrl::new`](api:hypersig/struct.SafeUrl.html#method.new)",
                        "`const fn new(url: &'static str) -> SafeUrl`",
                        "panic します。`const` の中ではコンパイルエラーになります",
                    ],
                    &[
                        "[`SafeUrl::parse`](api:hypersig/struct.SafeUrl.html#method.parse)",
                        "`fn parse(url: impl Into<String>) -> Result<SafeUrl, UnsafeUrl>`",
                        "URL が破った規則を示す [`UnsafeUrl`](api:hypersig/struct.UnsafeUrl.html) を返します",
                    ],
                    &[
                        "`SafeUrl` の `Deserialize`",
                        "ページのシグナルのフィールド",
                        "`SafeUrl::parse` と同じように失敗するので、ページのシグナルに `javascript:` の URL は入りません",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`SafeUrl::new` takes a URL written in the program. `SafeUrl::parse` reads a URL from outside the program, such as one a visitor typed:",
                ja: "`SafeUrl::new` はプログラムに書いた URL を受け取ります。`SafeUrl::parse` は、訪問者が入力したものなど、プログラムの外から来た URL を読みます。",
            },
        ),
        show(Snippet::SafeUrl),
        para(
            site,
            L {
                en: "A plain string is not a URL `href` takes, so it is E0277. The note under the error names what it takes: a `Link` or a `Url` from a route, a `SafeUrl`, or a signal or expression of `SafeUrl`, and `SafeUrl::parse` for a `String`.",
                ja: "素の文字列は `href` が受け取る URL ではないので、E0277 になります。エラーの下の注記に、受け取れるものが示されます。ルートから作った `Link` か `Url`、`SafeUrl`、`SafeUrl` のシグナルか式で、`String` は `SafeUrl::parse` を通します。",
            },
        ),
        show(Snippet::HrefString),
        para(
            site,
            L {
                en: "A `javascript:` URL in a `const` is E0080, with the message `its scheme is not http, https, or mailto`:",
                ja: "`const` の中の `javascript:` の URL は E0080 になり、`its scheme is not http, https, or mailto` というメッセージが出ます。",
            },
        ),
        show(Snippet::SafeUrlScript),
        para(
            site,
            L {
                en: "The host is not checked, so `https://evil.example/a.js` and `//evil.example` are `SafeUrl`s too. A visitor's URL suits a link the visitor follows, and not the `src` of an `<iframe>`, which would show whatever page the host serves inside this one. So `El::src` panics on an `<iframe>`, an `<embed>`, or a `<frame>` when the URL reads a signal, and a fixed `SafeUrl` there comes from the program.",
                ja: "ホストは検査しないので、`https://evil.example/a.js` や `//evil.example` も `SafeUrl` です。訪問者の URL は訪問者自身がたどるリンクには使えますが、`<iframe>` の `src` には使えません。そのホストが配信するページを、このページの中にそのまま表示するからです。そのため `El::src` は、`<iframe>`、`<embed>`、`<frame>` に対して URL がシグナルを読む場合に panic します。これらに渡す固定の `SafeUrl` は、プログラムに書いたものになります。",
            },
        ),
        para(
            site,
            L {
                en: "In the browser, a `SafeUrl` can also come from code outside hypersig, such as the `detail` of a [custom event](learn/javascript/), which nothing checks on its way in. When a signal writes `href`, `src`, or `poster`, the browser checks the same rule and removes the attribute where the URL breaks it.",
                ja: "ブラウザでは、`SafeUrl` が [カスタムイベント](learn/javascript/) の `detail` のように hypersig の外のコードから来ることもあり、入ってくるときには検査されません。シグナルが `href`、`src`、`poster` を書くときにブラウザが同じ規則で検査し、規則に反する URL なら属性を取り除きます。",
            },
        ),
        heading(
            site,
            L {
                en: "Attribute names and values",
                ja: "属性の名前と値",
            },
        ),
        para(
            site,
            L {
                en: "[`El::attr`](api:hypersig/html/struct.El.html#method.attr), `fn attr(self, name: Attr, value: impl Into<String>) -> Self`, sets a plain attribute. The name is an [`Attr`](api:hypersig/html/struct.Attr.html): a constant of [`html::attr`](api:hypersig/html/attr/index.html) for each attribute of HTML, SVG, and ARIA, or [`attr!`](api:hypersig/macro.attr.html) for any other name, such as `data-track` or an attribute of a custom element. `attr!` checks the name at compile time. It refuses the attributes hypersig writes itself (`id`, `class`, `role`, and those starting with `data-hs-`), the ones whose value names another element, such as `for`, and the ones whose value can run a script or load a URL: `href`, `src`, `poster`, `srcset`, every `on*` handler, `srcdoc`, `action`, `formaction`, `style`, and a few more. Each refused name has a method of its own, so `attr!(\"href\")` is E0080, with a message naming `El::href`:",
                ja: "[`El::attr`](api:hypersig/html/struct.El.html#method.attr)（`fn attr(self, name: Attr, value: impl Into<String>) -> Self`）は普通の属性を設定します。名前は [`Attr`](api:hypersig/html/struct.Attr.html) です。HTML、SVG、ARIA の属性には [`html::attr`](api:hypersig/html/attr/index.html) の定数があり、`data-track` やカスタム要素の属性のようなそれ以外の名前は [`attr!`](api:hypersig/macro.attr.html) で作ります。`attr!` は名前をコンパイル時に検査します。hypersig が自分で書く属性（`id`、`class`、`role`、`data-hs-` で始まる属性）、`for` のように値がほかの要素を指す属性、値がスクリプトを実行したり URL を読み込んだりできる属性（`href`、`src`、`poster`、`srcset`、すべての `on*` ハンドラ、`srcdoc`、`action`、`formaction`、`style` など）は受け付けません。受け付けない名前にはそれぞれ専用のメソッドがあるので、`attr!(\"href\")` は E0080 になり、メッセージで `El::href` が示されます。",
            },
        ),
        show(Snippet::AttrHref),
        para(
            site,
            L {
                en: "The value is any text, and nothing checks that it means anything for the attribute. The code below compiles and renders `type=\"emial\"`. The browser treats a `type` it does not recognize as `text`, so the field accepts any text and checks no address.",
                ja: "値は任意のテキストで、その属性にとって意味のある値かどうかは検査されません。下のコードはコンパイルが通り、`type=\"emial\"` を描画します。ブラウザは知らない `type` を `text` として扱うので、入力欄は任意のテキストを受け付け、メールアドレスの形かどうかも確かめません。",
            },
        ),
        show(Snippet::AttrValue),
        para(
            site,
            L {
                en: "`bind_attr` and `attr_when` take an `Attr` too, with a value that follows the signals. One attribute set twice on one element panics when the element is built, because the browser reads the first of two and ignores the second.",
                ja: "`bind_attr` と `attr_when` も `Attr` を受け取り、値はシグナルに合わせて変わります。1 つの要素に同じ属性を 2 回設定すると、要素を組み立てるときに panic します。ブラウザは 2 つのうち最初の属性を読み、2 つ目を無視するからです。",
            },
        ),
        heading(
            site,
            L {
                en: "CSS values from signals",
                ja: "シグナルから作る CSS の値",
            },
        ),
        para(
            site,
            L {
                en: "[`El::bind_style`](api:hypersig/html/struct.El.html#method.bind_style), `fn bind_style(self, property: Property, value: impl IntoExpr<S, String>) -> Self`, takes the property as a typed [`Property`](api:hypersig/style/enum.Property.html) and the value as a string the browser parses. Nothing checks that the value is valid for the property. A value the browser rejects leaves the property as the element's classes and stylesheets set it. `bind_token` takes a value the same way. Where the value comes from a fixed set, `El::css_match` keeps it typed, as [Styling](learn/styling/) shows.",
                ja: "[`El::bind_style`](api:hypersig/html/struct.El.html#method.bind_style)（`fn bind_style(self, property: Property, value: impl IntoExpr<S, String>) -> Self`）は、プロパティを型付きの [`Property`](api:hypersig/style/enum.Property.html) で、値をブラウザが解析する文字列で受け取ります。値がそのプロパティに有効かどうかは検査されません。ブラウザが受け付けない値なら、プロパティは要素のクラスとスタイルシートが設定したままになります。`bind_token` も同じように値を受け取ります。値が決まった選択肢から来るなら、[スタイル](learn/styling/) で示すように `El::css_match` で型付きのまま書けます。",
            },
        ),
        heading(
            site,
            L {
                en: "Files a page loads",
                ja: "ページが読み込むファイル",
            },
        ),
        para(
            site,
            L {
                en: "[`Asset::new`](api:hypersig/struct.Asset.html#method.new), `const fn new(href: &'static str) -> Asset`, names a script, a stylesheet, an icon, or the runtime. It panics, and in a `const` fails to compile, on an empty URL, a scheme other than `http` or `https`, or a space or a control character. It takes only text written in the program, because a script's URL decides what code the page runs. Nothing checks that the router serves the file: a path that no service answers loads nothing, and the browser's console shows the 404. [JavaScript libraries](learn/javascript/) shows how to serve one.",
                ja: "[`Asset::new`](api:hypersig/struct.Asset.html#method.new)（`const fn new(href: &'static str) -> Asset`）は、スクリプト、スタイルシート、アイコン、ランタイムのファイルを指します。空の URL、`http` と `https` 以外のスキーム、空白や制御文字を含む URL では panic し、`const` の中ではコンパイルエラーになります。スクリプトの URL はページが実行するコードを決めるので、受け取るのはプログラムに書いたテキストだけです。ルーターがそのファイルを配信しているかどうかは検査されません。どのサービスも応答しないパスでは何も読み込まれず、ブラウザのコンソールに 404 が表示されます。配信の方法は [JavaScript のライブラリ](learn/javascript/) で示します。",
            },
        ),
        heading(
            site,
            L {
                en: "Summary",
                ja: "まとめ",
            },
        ),
        table_of(
            site,
            L {
                en: &["Value", "Checked", "Not checked", "When it is wrong"],
                ja: &[
                    "値",
                    "検査されること",
                    "検査されないこと",
                    "間違っていたとき",
                ],
            },
            L {
                en: &[
                    &[
                        "`SafeUrl`",
                        "The scheme",
                        "The host, and whether the address exists",
                        "The link leads wherever the host sends it",
                    ],
                    &[
                        "`El::attr` value",
                        "The attribute's name",
                        "The value's meaning",
                        "The browser ignores the value or falls back to the attribute's default",
                    ],
                    &[
                        "`bind_style` and `bind_token` values",
                        "The property",
                        "The value's syntax",
                        "The browser keeps the property as the classes and stylesheets set it",
                    ],
                    &[
                        "`Asset` path",
                        "The scheme and the characters",
                        "Whether the router serves the file",
                        "Nothing loads, and the console shows the 404",
                    ],
                    &[
                        "`CustomEvent` name and `detail`",
                        "How the page uses the `detail`",
                        "Whether the script dispatches the event, and the shape of its `detail`",
                        "Nothing runs, or a missing field reads as `null`",
                    ],
                ],
                ja: &[
                    &[
                        "`SafeUrl`",
                        "スキーム",
                        "ホストと、そのアドレスがあるかどうか",
                        "リンク先はそのホストが返すものになります",
                    ],
                    &[
                        "`El::attr` の値",
                        "属性の名前",
                        "値の意味",
                        "ブラウザは値を無視するか、属性の既定の値として扱います",
                    ],
                    &[
                        "`bind_style` と `bind_token` の値",
                        "プロパティ",
                        "値の構文",
                        "プロパティはクラスとスタイルシートが設定したままになります",
                    ],
                    &[
                        "`Asset` のパス",
                        "スキームと文字",
                        "ルーターがそのファイルを配信しているか",
                        "何も読み込まれず、コンソールに 404 が表示されます",
                    ],
                    &[
                        "`CustomEvent` の名前と `detail`",
                        "ページが `detail` をどう使うか",
                        "スクリプトがそのイベントを投げるか、`detail` の形",
                        "何も実行されないか、ないフィールドが `null` になります",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Next, [Static pages](learn/static-pages/) serves a page without a server.",
                ja: "次の [静的ページ](learn/static-pages/) ではサーバーなしでページを配信する方法を説明します。",
            },
        ),
    ])
}
