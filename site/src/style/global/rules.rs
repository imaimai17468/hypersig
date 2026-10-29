//! The page on rules that select elements by tag, and the reset.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Style(StylePage::Global).leads()),
        heading(
            site,
            L {
                en: "A style for every element with a tag",
                ja: "あるタグのすべての要素に付けるスタイル",
            },
        ),
        para(
            site,
            L {
                en: "`El::css` styles one element. Text that comes from Markdown or a CMS has elements no Rust code builds, and every page wants the same `<body>`. A [`Global`](api:hypersig/style/struct.Global.html) holds rules for elements by selector, and [`Page::global`](api:hypersig/struct.Page.html#method.global), `fn global(self, global: Global) -> Self`, adds them to the page:",
                ja: "`El::css` が付けるのは 1 つの要素のスタイルです。Markdown や CMS から来るテキストには Rust のコードが組み立てない要素があり、`<body>` はどのページでも同じにしたいものです。[`Global`](api:hypersig/style/struct.Global.html) はセレクタで要素を選ぶルールを持ち、[`Page::global`](api:hypersig/struct.Page.html#method.global)（`fn global(self, global: Global) -> Self`）はそれをページに追加します。",
            },
        ),
        show(Snippet::GlobalRules),
        para(
            site,
            L {
                en: "Each rule is one `Style`, and conditions in it apply to the selector as they do to a class. Besides the reset, the page gets:",
                ja: "ルールはそれぞれ 1 つの `Style` で、その中の条件はクラスの場合と同じようにセレクタに対して適用されます。リセットのほかに、ページには次の CSS が書かれます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "@media (prefers-color-scheme: dark) { :root { color: white } }\n\
             body { font-family: system-ui; line-height: 1.6 }\n\
             a { color: #2563eb }\n\
             a:hover { text-decoration-line: underline }\n\
             code { font-family: monospace }",
        ),
        heading(
            site,
            L {
                en: "Building a Global",
                ja: "Global を組み立てる",
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
                        "[`new`](api:hypersig/style/struct.Global.html#method.new)`() -> Global`",
                        "No rules and no fonts.",
                    ],
                    &[
                        "[`preflight`](api:hypersig/style/struct.Global.html#method.preflight)`() -> Global`",
                        "The reset below.",
                    ],
                    &[
                        "[`rule`](api:hypersig/style/struct.Global.html#method.rule)`(self, selector: impl Into<Selector>, style: Style) -> Global`",
                        "Adds `style` for every element `selector` matches. A later rule for the same selector adds to the earlier one, and wins where both set a property under the same conditions.",
                    ],
                    &[
                        "[`font`](api:hypersig/style/struct.Global.html#method.font)`(self, font: FontFace) -> Global`",
                        "Loads a font, which [Fonts](style/fonts/) covers.",
                    ],
                    &[
                        "[`merge`](api:hypersig/style/struct.Global.html#method.merge)`(self, global: Global) -> Global`",
                        "Adds the rules and fonts of `global`, whose rules win over this one's.",
                    ],
                ],
                ja: &[
                    &[
                        "[`new`](api:hypersig/style/struct.Global.html#method.new)`() -> Global`",
                        "ルールもフォントもない `Global` を作ります。",
                    ],
                    &[
                        "[`preflight`](api:hypersig/style/struct.Global.html#method.preflight)`() -> Global`",
                        "下のリセットの入った `Global` を作ります。",
                    ],
                    &[
                        "[`rule`](api:hypersig/style/struct.Global.html#method.rule)`(self, selector: impl Into<Selector>, style: Style) -> Global`",
                        "`selector` に当てはまるすべての要素に `style` を追加します。同じセレクタのルールをあとから追加すると前のルールに加わり、同じ条件の下で同じプロパティを設定する場合はあとのルールが優先されます。",
                    ],
                    &[
                        "[`font`](api:hypersig/style/struct.Global.html#method.font)`(self, font: FontFace) -> Global`",
                        "フォントを読み込みます。[フォント](style/fonts/)で説明しています。",
                    ],
                    &[
                        "[`merge`](api:hypersig/style/struct.Global.html#method.merge)`(self, global: Global) -> Global`",
                        "`global` のルールとフォントを追加します。追加したルールはこの `Global` 自身のルールより優先されます。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`Page::global` merges each `Global` it is given into the earlier ones. `hypersig_ui::global()` is a `Global` built this way, so a page can call `.global(hypersig_ui::global())` and then `.global(..)` again with its own rules.",
                ja: "`Page::global` は、渡された `Global` をそれまでのものに `merge` します。`hypersig_ui::global()` もこのように組み立てた `Global` なので、ページは `.global(hypersig_ui::global())` を呼んだあと、自分のルールでもう一度 `.global(..)` を呼べます。",
            },
        ),
        heading(
            site,
            L {
                en: "Selectors",
                ja: "セレクタ",
            },
        ),
        para(
            site,
            L {
                en: "A rule takes a [`Selector`](api:hypersig/style/enum.Selector.html), and an [`Element`](api:hypersig/style/enum.Element.html) converts into one:",
                ja: "ルールは [`Selector`](api:hypersig/style/enum.Selector.html) を受け取り、[`Element`](api:hypersig/style/enum.Element.html) は `Selector` に変換できます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Selector", "CSS", "Matches"],
                ja: &["セレクタ", "CSS", "当てはまるもの"],
            },
            L {
                en: &[
                    &[
                        "`Selector::Root`",
                        "`:root`",
                        "The document's root, `<html>`",
                    ],
                    &[
                        "`Selector::Everything`",
                        "`*, ::before, ::after`",
                        "Every element and its `::before` and `::after`",
                    ],
                    &[
                        "`Element::Body`, `Element::A`, ...",
                        "`body`, `a`, ...",
                        "Every element with the tag",
                    ],
                ],
                ja: &[
                    &[
                        "`Selector::Root`",
                        "`:root`",
                        "文書のルート、つまり `<html>`",
                    ],
                    &[
                        "`Selector::Everything`",
                        "`*, ::before, ::after`",
                        "すべての要素と、その `::before` と `::after`",
                    ],
                    &[
                        "`Element::Body`、`Element::A` など",
                        "`body`、`a` など",
                        "そのタグを持つすべての要素",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`Element` has a variant for each of 82 HTML tags, from `A` to `Video`. A selector written as a string, such as a class, is a compile error, so a rule cannot reach an element by a class name that might not exist:",
                ja: "`Element` には `A` から `Video` まで、82 の HTML のタグにそれぞれ 1 つのバリアントがあります。クラスのような文字列で書いたセレクタはコンパイルエラーになるので、存在しないかもしれないクラス名で要素を選ぶルールは書けません。",
            },
        ),
        show(Snippet::SelectorString),
        heading(
            site,
            L {
                en: "An element's own style wins",
                ja: "要素自身のスタイルが優先される",
            },
        ),
        para(
            site,
            L {
                en: "The rules of a `Global` sit in [`BASE_LAYER`](api:hypersig/style/constant.BASE_LAYER.html), a cascade layer before the classes of `El::css`. An element's own style therefore overrides them whatever the selector, as the green link above shows. A plain stylesheet from `Page::style` sits outside the cascade layers and overrides both.",
                ja: "`Global` のルールは [`BASE_LAYER`](api:hypersig/style/constant.BASE_LAYER.html) に入ります。これは `El::css` のクラスより前にあるカスケードレイヤーです。そのため、セレクタが何であっても要素自身のスタイルがこれらのルールを上書きします。上の緑のリンクがその例です。`Page::style` で追加した普通のスタイルシートはカスケードレイヤーの外にあり、その両方を上書きします。",
            },
        ),
        heading(
            site,
            L {
                en: "The reset",
                ja: "リセット",
            },
        ),
        para(
            site,
            L {
                en: "[`Global::preflight`](api:hypersig/style/struct.Global.html#method.preflight) starts with a small reset. It sizes every element with `border-box`, removes default margins, makes form controls use the page's font and color, and shrinks images, SVGs, and videos to fit their box:",
                ja: "[`Global::preflight`](api:hypersig/style/struct.Global.html#method.preflight) には最初から小さなリセットが入っています。すべての要素の大きさを `border-box` で測り、既定のマージンを消し、フォームのコントロールにページのフォントと色を使わせ、画像、SVG、動画を枠に収まるまで縮めます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "*, ::before, ::after { box-sizing: border-box; margin: 0 }\n\
             body { line-height: 1.5 }\n\
             img, svg, video { display: block; max-width: 100% }\n\
             button, input, select, textarea { font: inherit; color: currentColor }",
        ),
        para(
            site,
            L {
                en: "A rule added after `preflight()` for the same selector adds to the reset's rule, so the `body` rule above replaces `line-height: 1.5` with `1.6` and keeps the rest. `Global::new()` starts without the reset.",
                ja: "`preflight()` のあとに同じセレクタのルールを追加すると、リセットのルールに加わります。そのため、上の `body` のルールは `line-height: 1.5` を `1.6` に置き換え、残りはそのまま保ちます。`Global::new()` はリセットなしで始まります。",
            },
        ),
        heading(
            site,
            L {
                en: "Conditions a rule cannot use",
                ja: "ルールで使えない条件",
            },
        ),
        para(
            site,
            L {
                en: "A rule selects elements by tag, so it has no marked ancestor or sibling for a group or peer condition, and `Selector::Everything` already names `::before` and `::after`. The types do not stop either, and `Page::render` panics when it writes the rule:",
                ja: "ルールはタグで要素を選ぶので、group や peer の条件に使う印の付いた祖先や兄弟がありません。また、`Selector::Everything` はすでに `::before` と `::after` を含んでいます。どちらも型では防げず、`Page::render` がそのルールを書くときにパニックします。",
            },
        ),
        show(Snippet::RuleGroup),
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
                    en: "A group or peer condition in a rule, or a pseudo-element in a rule for `Selector::Everything`, panics at `Page::render`, as above.",
                    ja: "上のとおり、ルールの中の group や peer の条件と、`Selector::Everything` のルールの中の擬似要素は、`Page::render` でパニックします。",
                },
                L {
                    en: "A rule applies to every element with the tag, including the elements of components such as those of `hypersig_ui`. A rule for `button` changes their buttons too, wherever their own style does not set the property.",
                    ja: "ルールは `hypersig_ui` のようなコンポーネントの要素も含めて、そのタグを持つすべての要素に適用されます。`button` のルールは、コンポーネント自身のスタイルがそのプロパティを設定していないかぎり、コンポーネントのボタンも変えます。",
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
                en: "[Fonts](style/fonts/) loads a font file with `@font-face` and uses it in a rule for `body`.",
                ja: "[フォント](style/fonts/)ではフォントのファイルを `@font-face` で読み込み、`body` のルールで使います。",
            },
        ),
    ])
}
