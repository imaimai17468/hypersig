//! The page on every form that reaches the CSS without a check: `raw`,
//! stylesheets, classes, and values bound to signals.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Style(StylePage::Unchecked).title(site)),
        lead(site, Doc::Style(StylePage::Unchecked).leads()),
        heading(
            site,
            L {
                en: "Values in `raw`",
                ja: "`raw` の値",
            },
        ),
        para(
            site,
            L {
                en: "[`raw(css: impl Into<String>) -> Raw`](api:hypersig/style/value/fn.raw.html) writes a value as given. It is for the forms a property's generated type leaves out: a value of several parts such as `margin: 0 auto`, a shadow, a `calc()` that mixes a length and a percentage, a `cubic-bezier()`, or a list of grid tracks. Every generated value type, `Color`, and `Length` convert from a [`Raw`](api:hypersig/style/value/struct.Raw.html), so `raw` goes wherever a typed value would:",
                ja: "[`raw(css: impl Into<String>) -> Raw`](api:hypersig/style/value/fn.raw.html) は値を渡されたとおりに書きます。`margin: 0 auto` のように複数の部分からなる値、影、長さとパーセンテージを混ぜた `calc()`、`cubic-bezier()`、グリッドのトラックの一覧など、プロパティの生成された型に含まれない書き方に使います。生成された値の型と `Color`、`Length` はどれも [`Raw`](api:hypersig/style/value/struct.Raw.html) から変換できるので、`raw` は型の付いた値を渡す場所ならどこにでも渡せます。",
            },
        ),
        show(Snippet::Raw),
        para(
            site,
            L {
                en: "The browser checks the value. If it rejects it, it drops the declaration, and the element renders as if the property were not set. The methods that take a bare number, such as `flex_grow` and `order`, take no `raw`.",
                ja: "値を検査するのはブラウザです。ブラウザが受け付けない値なら、その宣言は捨てられ、要素はそのプロパティを設定していないかのように描画されます。`flex_grow` や `order` のように数値をそのまま受け取るメソッドは `raw` を受け取りません。",
            },
        ),
        para(
            site,
            L {
                en: "`raw` checks only that the text stays inside its declaration. It panics on `;`, `{`, `}`, `<`, a backslash, `/*`, a control character, and a quote or a parenthesis left open. The first three would end the declaration or the rule, `<` could close the `<style>` element, and the rest would carry the value on into the rules after it. `raw` is not a `const fn`, so the panic happens when the line runs:",
                ja: "`raw` が検査するのは、テキストが値の中で終わることだけです。`;`、`{`、`}`、`<`、バックスラッシュ、`/*`、制御文字を含む値と、引用符や括弧を閉じていない値を受け取ると panic します。最初の 3 つは宣言やルールを終わらせ、`<` は `<style>` 要素を閉じることがあり、残りは値をその後のルールまで続けてしまうからです。`raw` は `const fn` ではないので、panic はその行を実行したときに起きます。",
            },
        ),
        show(Snippet::RawPanic),
        heading(
            site,
            L {
                en: "An existing stylesheet",
                ja: "既存のスタイルシート",
            },
        ),
        table_of(
            site,
            L {
                en: &["Item", "What it does"],
                ja: &["項目", "すること"],
            },
            L {
                en: &[
                    &[
                        "[`Stylesheet::new(css: &'static str) -> Stylesheet`](api:hypersig/html/struct.Stylesheet.html#method.new)",
                        "Wraps CSS text, usually from `include_str!`. Panics on `</style` in any case, and in a `const` that is a compile error",
                    ],
                    &[
                        "[`Stylesheet::class(self, name: &'static str) -> Class`](api:hypersig/html/struct.Stylesheet.html#method.class)",
                        "The class `name`, after checking that a selector of the CSS names `.name`. Panics otherwise, and in a `const` that is a compile error",
                    ],
                    &[
                        "[`Page::style(self, sheet: Stylesheet) -> Page<S>`](api:hypersig/struct.Page.html#method.style)",
                        "Writes the CSS into a `<style>` element of `<head>`",
                    ],
                    &[
                        "[`El::class(self, class: Class) -> El<S>`](api:hypersig/html/struct.El.html#method.class), [`El::class_when`](api:hypersig/html/struct.El.html#method.class_when)",
                        "Put the class on an element, always or while a condition holds",
                    ],
                ],
                ja: &[
                    &[
                        "[`Stylesheet::new(css: &'static str) -> Stylesheet`](api:hypersig/html/struct.Stylesheet.html#method.new)",
                        "CSS のテキストを包みます。テキストは多くの場合 `include_str!` で読みます。大文字と小文字を問わず `</style` を含むと panic し、`const` の中ではコンパイルエラーになります",
                    ],
                    &[
                        "[`Stylesheet::class(self, name: &'static str) -> Class`](api:hypersig/html/struct.Stylesheet.html#method.class)",
                        "CSS のセレクタに `.name` があることを確かめてから、クラス `name` を返します。ない場合は panic し、`const` の中ではコンパイルエラーになります",
                    ],
                    &[
                        "[`Page::style(self, sheet: Stylesheet) -> Page<S>`](api:hypersig/struct.Page.html#method.style)",
                        "CSS を `<head>` の `<style>` 要素に書きます",
                    ],
                    &[
                        "[`El::class(self, class: Class) -> El<S>`](api:hypersig/html/struct.El.html#method.class)、[`El::class_when`](api:hypersig/html/struct.El.html#method.class_when)",
                        "クラスを要素に付けます。`class_when` は条件が成り立つ間だけ付けます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The rules of a `Stylesheet` are in no cascade layer, so they override the classes of `El::css` wherever both set a property. Next to the stylesheet, the sample keeps a style of `raw` values:",
                ja: "`Stylesheet` のルールはどのカスケードレイヤーにも入らないので、両方が同じプロパティを設定する場所では `El::css` のクラスより優先されます。下のサンプルではスタイルシートと並べて `raw` の値のスタイルも使います。",
            },
        ),
        show(Snippet::Unchecked),
        para(
            site,
            L {
                en: "A misspelled class in a `const` is E0080:",
                ja: "`const` の中でクラス名を書き間違えると E0080 になります。",
            },
        ),
        show(Snippet::StylesheetClass),
        para(
            site,
            L {
                en: "The search for `.name` skips comments and quoted strings and reads a backslash escape such as `.w-1\\/2` as the character it escapes. It does not read hex escapes, so a class written only as `.\\31 0` is not found, and a `.name` inside an unquoted `url(...)` counts as found. The rest of the CSS text is not checked, and the browser drops a rule it cannot parse.",
                ja: "`.name` を探すときは、コメントと引用符で囲んだ文字列を飛ばし、`.w-1\\/2` のようなバックスラッシュのエスケープはエスケープした文字として読みます。16 進のエスケープは読まないので、`.\\31 0` としか書かれていないクラスは見つかりません。また、引用符のない `url(...)` の中の `.name` は見つかったものとして扱います。CSS のテキストのそのほかの部分は検査されず、ブラウザは解析できないルールを捨てます。",
            },
        ),
        heading(
            site,
            L {
                en: "A stylesheet the page links",
                ja: "ページがリンクするスタイルシート",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::stylesheet(self, href: Asset) -> Page<S>`](api:hypersig/struct.Page.html#method.stylesheet) adds `<link rel=\"stylesheet\">` for a file the browser loads. The program never reads the file, so its classes come from [`Class::new(name: &'static str) -> Class`](api:hypersig/html/struct.Class.html#method.new), which checks only that the name is not empty and holds no whitespace:",
                ja: "[`Page::stylesheet(self, href: Asset) -> Page<S>`](api:hypersig/struct.Page.html#method.stylesheet) は、ブラウザが読み込むファイルの `<link rel=\"stylesheet\">` を加えます。プログラムはそのファイルを読まないので、クラスは [`Class::new(name: &'static str) -> Class`](api:hypersig/html/struct.Class.html#method.new) で作ります。`Class::new` が検査するのは、名前が空でなく空白を含まないことだけです。",
            },
        ),
        show(Snippet::LinkedStylesheet),
        para(
            site,
            L {
                en: "A class the file does not define puts the class on the element and applies nothing. A path that the router does not serve loads nothing, and the browser's console shows the 404.",
                ja: "ファイルに定義のないクラスは、要素に付くだけで何も適用しません。ルーターが配信していないパスでは何も読み込まれず、ブラウザのコンソールに 404 が表示されます。",
            },
        ),
        heading(
            site,
            L {
                en: "Values from signals",
                ja: "シグナルから作る値",
            },
        ),
        para(
            site,
            L {
                en: "[`El::bind_style(self, property: Property, value: impl IntoExpr<S, String>) -> El<S>`](api:hypersig/html/struct.El.html#method.bind_style) keeps one property of the element's `style` attribute equal to a string expression over the signals, and [`El::bind_token`](api:hypersig/html/struct.El.html#method.bind_token) does the same for a [token](style/tokens/). The property is a typed [`Property`](api:hypersig/style/enum.Property.html), and the value is a string the browser parses:",
                ja: "[`El::bind_style(self, property: Property, value: impl IntoExpr<S, String>) -> El<S>`](api:hypersig/html/struct.El.html#method.bind_style) は、要素の `style` 属性の 1 つのプロパティを、シグナルから作る文字列の式の値と常に同じにします。[`El::bind_token`](api:hypersig/html/struct.El.html#method.bind_token) は[トークン](style/tokens/)について同じことをします。プロパティは型の付いた [`Property`](api:hypersig/style/enum.Property.html) で、値はブラウザが解析する文字列です。",
            },
        ),
        show(Snippet::BindStyle),
        para(
            site,
            L {
                en: "Nothing checks that the value is valid for the property. A value the browser rejects leaves the property as the element's classes and stylesheets set it. Where the value comes from a fixed set, [`El::css_match`](style/toggled/) picks one typed style per value instead.",
                ja: "値がそのプロパティに有効かどうかは検査されません。ブラウザが受け付けない値なら、プロパティは要素のクラスとスタイルシートが設定したままになります。値が決まった選択肢から来るなら、代わりに [`El::css_match`](style/toggled/) で値ごとに型の付いたスタイルを 1 つ選べます。",
            },
        ),
        heading(
            site,
            L {
                en: "The `style` attribute",
                ja: "`style` 属性",
            },
        ),
        para(
            site,
            L {
                en: "No other method writes the `style` attribute. [`attr!`](api:hypersig/macro.attr.html) refuses the name `style`, because `El::css` and `El::bind_style` write the element's CSS, so the attribute is E0080:",
                ja: "ほかに `style` 属性を書くメソッドはありません。`El::css` と `El::bind_style` が要素の CSS を書くので、[`attr!`](api:hypersig/macro.attr.html) は `style` という名前を受け付けず、この属性は E0080 になります。",
            },
        ),
        show(Snippet::StyleAttr),
        heading(
            site,
            L {
                en: "Names nothing checks for uniqueness",
                ja: "重複を検査しない名前",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "[`Keyframes::new`](style/animation/): two keyframes with one name make one `@keyframes` rule in the page.",
                    ja: "[`Keyframes::new`](style/animation/)：同じ名前の 2 つのキーフレームからは、ページに `@keyframes` のルールが 1 つしかできません。",
                },
                L {
                    en: "[`Token::new`](style/tokens/): two tokens with one name are one custom property to the browser.",
                    ja: "[`Token::new`](style/tokens/)：同じ名前の 2 つのトークンはブラウザにとっては 1 つのカスタムプロパティです。",
                },
                L {
                    en: "A `view-transition-name` in `raw`: two elements of one page with the same name make the browser skip the [view transition](style/page-transitions/).",
                    ja: "`raw` で書いた `view-transition-name`：1 つのページで 2 つの要素が同じ名前を持つと、ブラウザは [View Transitions](style/page-transitions/) を飛ばします。",
                },
            ],
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
                en: &[
                    "Name to search for",
                    "Checked",
                    "Not checked",
                    "When it is wrong",
                ],
                ja: &[
                    "検索する名前",
                    "検査されること",
                    "検査されないこと",
                    "間違っていたとき",
                ],
            },
            L {
                en: &[
                    &[
                        "`raw`",
                        "That the text stays inside its declaration, when the line runs",
                        "The value's syntax and range",
                        "The browser drops the declaration",
                    ],
                    &[
                        "`Stylesheet::new`",
                        "No `</style`, at compile time in a `const`",
                        "The rest of the CSS",
                        "The browser drops the rules it cannot parse",
                    ],
                    &[
                        "`Stylesheet::class`",
                        "A selector names the class, at compile time in a `const`",
                        "Hex escapes, and `.name` in an unquoted `url()`",
                        "Fails to compile, or panics outside a `const`",
                    ],
                    &[
                        "`Class::new`, `Page::stylesheet`",
                        "The name is not empty and has no whitespace",
                        "That the linked file defines the class, and that it is served",
                        "The class applies nothing, and a missing file is a 404 in the console",
                    ],
                    &[
                        "`bind_style`, `bind_token`",
                        "The property",
                        "The value's syntax",
                        "The property keeps the value the classes and stylesheets set",
                    ],
                    &[
                        "`Keyframes::new`, `Token::new`",
                        "The characters of a keyframes name, when the line runs",
                        "That each name is used once",
                        "One rule or one custom property for both",
                    ],
                ],
                ja: &[
                    &[
                        "`raw`",
                        "テキストが宣言の中で終わること（その行の実行時）",
                        "値の構文と範囲",
                        "ブラウザが宣言を捨てます",
                    ],
                    &[
                        "`Stylesheet::new`",
                        "`</style` がないこと（`const` の中ではコンパイル時）",
                        "そのほかの CSS",
                        "ブラウザは解析できないルールを捨てます",
                    ],
                    &[
                        "`Stylesheet::class`",
                        "セレクタがそのクラスを指すこと（`const` の中ではコンパイル時）",
                        "16 進のエスケープと、引用符のない `url()` の中の `.name`",
                        "コンパイルエラーになります。`const` の外では panic します",
                    ],
                    &[
                        "`Class::new`、`Page::stylesheet`",
                        "名前が空でなく、空白を含まないこと",
                        "リンクしたファイルにそのクラスがあるか、ファイルが配信されているか",
                        "クラスは何も適用せず、ファイルがなければコンソールに 404 が表示されます",
                    ],
                    &[
                        "`bind_style`、`bind_token`",
                        "プロパティ",
                        "値の構文",
                        "プロパティはクラスとスタイルシートが設定した値のままです",
                    ],
                    &[
                        "`Keyframes::new`、`Token::new`",
                        "キーフレームの名前の文字（その行の実行時）",
                        "名前が一度しか使われていないか",
                        "両方で 1 つのルールか 1 つのカスタムプロパティになります",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "[Values the types do not check](learn/unchecked-values/) covers the same question outside CSS: URLs, attribute values, and the files a page loads.",
                ja: "CSS の外の同じ問題、つまり URL、属性の値、ページが読み込むファイルについては[型で検査しない値](learn/unchecked-values/)で説明しています。",
            },
        ),
    ])
}
