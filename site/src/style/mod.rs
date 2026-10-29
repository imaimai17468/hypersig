//! The Style section: typed atomic CSS from `hypersig::style`.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

mod conditions;
mod global;
mod start;
mod tokens;
mod values;

pub use start::overview::benchmarks;

/// The article of `page`.
pub fn content(site: Site, page: StylePage) -> Part {
    let head = title(Doc::Style(page).title(site));
    match page {
        // Get started
        StylePage::Overview => start::overview::page(site, head),
        StylePage::Applying => start::applying::page(site, head),
        // Values
        StylePage::Values => values::properties::page(site, head),
        StylePage::Lengths => values::lengths::page(site, head),
        StylePage::Colors => values::colors::page(site, head),
        StylePage::AnglesAndTimes => values::angles_and_times::page(site, head),
        StylePage::UrlsAndStrings => values::urls_and_strings::page(site, head),
        StylePage::Keywords => values::keywords::page(site, head),
        // Conditions
        StylePage::PseudoClasses => conditions::pseudo_classes::page(site, head),
        StylePage::PseudoElements => conditions::pseudo_elements::page(site, head),
        StylePage::Media => conditions::media::page(site, head),
        StylePage::Containers => conditions::containers::page(site, head),
        StylePage::GroupAndPeer => conditions::group_and_peer::page(site, head),
        StylePage::Toggled => conditions::toggled::page(site, head),
        // Tokens and themes
        StylePage::Tokens => tokens::token::page(site, head),
        StylePage::Themes => tokens::themes::page(site, head),
        StylePage::DarkMode => tokens::dark_mode::page(site, head),
        // Global styles
        StylePage::Global => global::rules::page(site, head),
        StylePage::Fonts => global::fonts::page(site, head),
        // Layout
        StylePage::Stacks | StylePage::Grids | StylePage::Patterns => patterns(site, head),
        // Animation
        StylePage::Animation
        | StylePage::Transitions
        | StylePage::PageTransitions
        | StylePage::ReducedMotion => animation(site, head),
        // Opting out of type checks
        StylePage::Unchecked => unchecked(site, head),
    }
}

fn patterns(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Patterns).leads()),
        show(Snippet::Patterns),
        table_of(
            site,
            L {
                en: &["Pattern", "Lays out"],
                ja: &["パターン", "並べ方"],
            },
            L {
                en: &[
                    &[
                        "`stack(gap)`, `vstack(gap)`",
                        "Children in a column, the second centered across it",
                    ],
                    &["`hstack(gap)`", "Children in a row, centered on its line"],
                    &["`wrap(gap)`", "Children in rows that wrap onto new lines"],
                    &[
                        "`grid(columns, gap)`, `grid_item(columns)`",
                        "A grid of equal columns, and a child spanning several",
                    ],
                    &[
                        "`container(max, padding)`",
                        "A block no wider than `max`, centered, with padding on both sides",
                    ],
                    &[
                        "`center()`, `spacer()`",
                        "Children centered on both axes, and an empty flex child that takes the free space",
                    ],
                    &[
                        "`square(size)`, `circle(size)`, `aspect_ratio(ratio)`",
                        "Boxes of a fixed shape",
                    ],
                    &["`divider(width, color)`", "A line across the box"],
                    &[
                        "`visually_hidden()`",
                        "Hidden from sight but read by screen readers",
                    ],
                    &[
                        "`link_overlay()`",
                        "A link that covers its positioned ancestor, so a whole card is clickable",
                    ],
                ],
                ja: &[
                    &[
                        "`stack(gap)`、`vstack(gap)`",
                        "子を縦 1 列に並べ、`vstack` は横方向の中央に揃えます",
                    ],
                    &[
                        "`hstack(gap)`",
                        "子を横 1 行に並べ、行の中で縦の中央に揃えます",
                    ],
                    &[
                        "`wrap(gap)`",
                        "子を横に並べ、収まらない分を次の行に折り返します",
                    ],
                    &[
                        "`grid(columns, gap)`、`grid_item(columns)`",
                        "等しい幅の列のグリッドと、複数の列にまたがる子",
                    ],
                    &[
                        "`container(max, padding)`",
                        "幅が `max` を超えない中央寄せのブロックで、左右に余白があります",
                    ],
                    &[
                        "`center()`、`spacer()`",
                        "子を縦横の中央に置くボックスと、空いた場所を埋める空の flex の子",
                    ],
                    &[
                        "`square(size)`、`circle(size)`、`aspect_ratio(ratio)`",
                        "形の決まったボックス",
                    ],
                    &["`divider(width, color)`", "ボックスを横切る線"],
                    &[
                        "`visually_hidden()`",
                        "画面には見えず、スクリーンリーダーには読まれる要素",
                    ],
                    &[
                        "`link_overlay()`",
                        "位置指定された祖先を覆うリンクで、カード全体をクリックできるようにします",
                    ],
                ],
            },
        ),
        demo::show(site, Demo::Layout),
    ])
}

fn animation(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Animation).leads()),
        heading(
            site,
            L {
                en: "Keyframes",
                ja: "キーフレーム",
            },
        ),
        para(
            site,
            L {
                en: "[`Keyframes::new(name)`](api:hypersig_style/struct.Keyframes.html) creates an animation, and `at(percent, style)` adds a step. `Style::keyframes` sets an element's `animation-name` and includes the `@keyframes` rule in the style, so the page or patch that contains the element also contains the rule. A keyframe writes only the unconditioned declarations of its style, because a keyframe takes no selector or media query.",
                ja: "[`Keyframes::new(name)`](api:hypersig_style/struct.Keyframes.html) でアニメーションを作り、`at(percent, style)` で段階を 1 つ追加します。`Style::keyframes` は要素の `animation-name` を設定し、`@keyframes` のルールもスタイルに含めます。そのため、要素を含むページやパッチには、そのルールも含まれます。キーフレームにはセレクタもメディアクエリも書けないので、キーフレームに書かれるのはスタイルのうち条件の付かない宣言だけです。",
            },
        ),
        show(Snippet::Animation),
        demo::show(site, Demo::Spinner),
        heading(
            site,
            L {
                en: "Reduced motion",
                ja: "動きを減らす設定",
            },
        ),
        para(
            site,
            L {
                en: "`Media::ReducedMotion` applies a style for a visitor who asks for reduced motion. In that case the components of `hypersig_ui` change colors without a transition.",
                ja: "`Media::ReducedMotion` は、動きを減らすよう求める訪問者にだけスタイルを適用します。`hypersig_ui` のコンポーネントはその場合に色をトランジションなしで切り替えます。",
            },
        ),
        note(
            site,
            L {
                en: "Two `Keyframes` with the same name are treated as one rule by the browser, and the one later in the stylesheet is used. Duplicate names are not checked.",
                ja: "同じ名前の 2 つの `Keyframes` は、ブラウザにとっては 1 つのルールとして扱われ、スタイルシートで後にある方が使われます。名前が重複していないかは検査されません。",
            },
        ),
        heading(
            site,
            L {
                en: "Page transitions",
                ja: "ページ遷移",
            },
        ),
        para(
            site,
            L {
                en: "A link from one page to another loads the new page, so the browser's cross-document view transitions animate the change. A page opts in with the CSS rule `@view-transition { navigation: auto; }`, and both the page left and the page reached need it. The browser takes a picture of the old page, shows the new one, and fades between the two unless `::view-transition-old(root)` and `::view-transition-new(root)` set other animations. `Style` has no at-rule or pseudo-element for these, so they go in a [`Stylesheet`](api:hypersig/html/struct.Stylesheet.html), which `Page::style` adds to `<head>`.",
                ja: "あるページから別のページへのリンクは新しいページを読み込むので、その切り替わりはブラウザのクロスドキュメントの View Transitions でアニメーションできます。ページは CSS の `@view-transition { navigation: auto; }` で有効にし、移る前のページと移った先のページの両方にこのルールが必要です。ブラウザは前のページの画像を撮って新しいページを表示し、`::view-transition-old(root)` と `::view-transition-new(root)` で別のアニメーションを指定しない限り、2 つをクロスフェードでつなぎます。`Style` にはこのアットルールも擬似要素もないので、[`Stylesheet`](api:hypersig/html/struct.Stylesheet.html) に書き、`Page::style` で `<head>` に入れます。",
            },
        ),
        para(
            site,
            L {
                en: "An element with a `view-transition-name` moves on its own, from where it was on the old page to where it is on the new one. `Style::view_transition_name` sets the name, and an element that keeps its place on both pages, such as a header, then stays still while the rest of the page changes.",
                ja: "`view-transition-name` を持つ要素は、前のページでの位置から新しいページでの位置へ、ほかとは別に動きます。名前は `Style::view_transition_name` で付けます。ヘッダーのように両方のページで同じ位置にある要素に名前を付けると、その要素はページのほかの部分が切り替わるあいだ動きません。",
            },
        ),
        show(Snippet::ViewTransitions),
        bullets(
            site,
            &[
                L {
                    en: "The name goes through `raw(...)`, and nothing checks it. Two elements of one page with the same name make the browser skip the transition.",
                    ja: "名前は `raw(...)` で渡し、検査はされません。1 つのページで 2 つの要素が同じ名前を持つと、ブラウザは遷移を飛ばします。",
                },
                L {
                    en: "A patch from the server changes the page in place and loads no page, so it plays no view transition. A statement plays one inside `Stmt::view_transition`, which the next section covers.",
                    ja: "サーバーからのパッチはページをその場で書き換え、ページを読み込まないので、View Transitions は起きません。文の場合は `Stmt::view_transition` の中で実行すると起きます。次の節で説明します。",
                },
                L {
                    en: "Chrome and Edge play cross-document view transitions from version 126, and Safari from 18.2. A browser without them changes the page as usual.",
                    ja: "クロスドキュメントの View Transitions は、Chrome と Edge では 126 から、Safari では 18.2 から動きます。対応していないブラウザでは、ページは通常どおり切り替わります。",
                },
                L {
                    en: "Put the animations under `@media (prefers-reduced-motion: no-preference)`, and set `animation: none` on the view transition pseudo-elements under `@media (prefers-reduced-motion: reduce)`. Without the second rule, a visitor who asks for reduced motion still sees the browser's default cross-fade. This site does the same.",
                    ja: "アニメーションは `@media (prefers-reduced-motion: no-preference)` の中に書き、`@media (prefers-reduced-motion: reduce)` の中で View Transitions の擬似要素に `animation: none` を指定します。後者がないと、動きを減らすよう求める訪問者にもブラウザ既定のクロスフェードが表示されます。このサイトも同じように書いています。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Transitions within a page",
                ja: "ページ内の遷移",
            },
        ),
        para(
            site,
            L {
                en: "A statement changes the page without loading another, so the browser animates it only when the runtime starts the transition. [`Stmt::view_transition`](api:hypersig/struct.Stmt.html#method.view_transition) wraps a statement: the browser takes a picture of the page, the runtime runs the statement and renders what it wrote, and the two pictures cross-fade unless the page's CSS sets other animations. The theme button in this site's header changes its colors this way.",
                ja: "文は別のページを読み込まずにページを書き換えるので、ブラウザがアニメーションするのは、ランタイムが遷移を始めたときだけです。[`Stmt::view_transition`](api:hypersig/struct.Stmt.html#method.view_transition) は文を包みます。ブラウザがページの画像を撮り、ランタイムが文を実行して書き込んだ内容を描画すると、ページの CSS で別のアニメーションを指定しない限り、2 つの画像がクロスフェードでつながります。このサイトのヘッダーにある配色のボタンも、この方法で色を切り替えています。",
            },
        ),
        show(Snippet::ViewTransitionStmt),
        bullets(
            site,
            &[
                L {
                    en: "To animate the move between pages and the change within a page differently, give the navigation a type with `@view-transition { navigation: auto; types: page; }` and put its animations under `html:active-view-transition-type(page)`. A transition from `Stmt::view_transition` has no type, so `html:not(:active-view-transition-type(page))` selects it. This site writes its two animations that way.",
                    ja: "ページ間の移動とページ内の変化に別のアニメーションを付けるには、`@view-transition { navigation: auto; types: page; }` で移動に種類を付け、そのアニメーションを `html:active-view-transition-type(page)` の下に書きます。`Stmt::view_transition` の遷移には種類がないので、`html:not(:active-view-transition-type(page))` で選べます。このサイトも 2 つのアニメーションをそのように書いています。",
                },
                L {
                    en: "Chrome and Edge run `document.startViewTransition` from version 111, Safari from 18, and Firefox from 144, according to MDN's compatibility data. A browser without it runs the statement with no animation. `:active-view-transition-type()` needs Chrome 125, Safari 18.2, or Firefox 147.",
                    ja: "MDN の互換性データによると、`document.startViewTransition` は Chrome と Edge では 111 から、Safari では 18 から、Firefox では 144 から動きます。対応していないブラウザでは、文はアニメーションなしで実行されます。`:active-view-transition-type()` には Chrome 125、Safari 18.2、Firefox 147 以降が要ります。",
                },
            ],
        ),
    ])
}

fn unchecked(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Unchecked).leads()),
        show(Snippet::Unchecked),
        bullets(
            site,
            &[
                L {
                    en: "`raw(css)` writes a value as given, for the forms a property's generated type leaves out, such as `margin: 0 auto`, a shadow, or a `calc()`. If the browser rejects the value, it ignores the declaration. `raw` panics on `;`, `{`, `}`, `<`, a backslash, `/*`, a control character, and a quote or a parenthesis left open. The first three would end the declaration or the rule, `<` could close the `<style>` element, and the rest would carry the value on into the rules after it.",
                    ja: "`raw(css)` は値を渡されたとおりに書きます。`margin: 0 auto`、影、`calc()` のように、プロパティの生成された型に含まれない書き方に使います。ブラウザが受け付けない値なら、ブラウザはその宣言を無視します。`raw` は `;`、`{`、`}`、`<`、バックスラッシュ、`/*`、制御文字を含む値と、引用符や括弧を閉じていない値を受け取るとパニックします。最初の 3 つは宣言やルールを終わらせ、`<` は `<style>` 要素を閉じることがあり、残りは値をその後のルールまで続けてしまうからです。",
                },
                L {
                    en: "`Stylesheet::new(css)` makes a `Stylesheet` from CSS you already have, and `Page::style` adds it to the page. `Stylesheet::class(\"name\")` checks that the CSS has a selector for the class and panics if it does not. In a `const`, as in the sample, the panic is a compile error, and outside one it happens when the line runs. `Class::new` refers to a class by name without that check.",
                    ja: "`Stylesheet::new(css)` は手元にある CSS から `Stylesheet` を作り、`Page::style` でそれをページに追加します。`Stylesheet::class(\"name\")` は、その CSS にクラスのセレクタがあるかを確かめ、なければパニックします。サンプルのように `const` の中ではこのパニックがコンパイルエラーになり、`const` の外ではその行を実行したときに起きます。`Class::new` はその確認をせずにクラス名を指定します。",
                },
                L {
                    en: "`El::bind_style` and `El::bind_token` set a property from a string expression over the signals. The expression is not checked.",
                    ja: "`El::bind_style` と `El::bind_token` はシグナルから作る文字列の式でプロパティを設定します。この式は検査されません。",
                },
            ],
        ),
    ])
}
