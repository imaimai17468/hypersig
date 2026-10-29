//! The Style section: typed atomic CSS from `hypersig::style`.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::{L, Text};
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

mod conditions;
mod global;
mod tokens;

/// The article of `page`.
pub fn content(site: Site, page: StylePage) -> Part {
    let head = title(Doc::Style(page).title(site));
    match page {
        // Get started
        StylePage::Overview | StylePage::Applying => overview(site, head),
        // Values
        StylePage::Values
        | StylePage::Lengths
        | StylePage::Colors
        | StylePage::AnglesAndTimes
        | StylePage::UrlsAndStrings
        | StylePage::Keywords => values(site, head),
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

fn overview(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Overview).leads()),
        heading(
            site,
            L {
                en: "A style is a Rust value",
                ja: "スタイルは Rust の値",
            },
        ),
        para(
            site,
            L {
                en: "[`Style`](api:hypersig_style/struct.Style.html) has one method per standard CSS property, and per anchor positioning property. The property list, each property's keywords, the pseudo-classes, the pseudo-elements, and the named colors are generated from MDN's CSS data (`mdn-data` 2.37.2). `El::css` adds a style to an element.",
                ja: "[`Style`](api:hypersig_style/struct.Style.html) には、標準の CSS プロパティとアンカー配置のプロパティごとに 1 つずつメソッドがあります。プロパティの一覧、各プロパティのキーワード、擬似クラス、擬似要素、名前付きの色は、MDN の CSS データ（`mdn-data` 2.37.2）から生成しています。要素にスタイルを付けるには `El::css` を使います。",
            },
        ),
        show(Snippet::StyleBasics),
        para(
            site,
            L {
                en: "A second call to the same property's method under the same conditions replaces the earlier value, as a later declaration does in a CSS rule. `merge` adds the declarations of another style, and the added declarations override the original ones.",
                ja: "同じ条件の下で同じプロパティのメソッドをもう一度呼ぶと、CSS のルールで後に書いた宣言と同じく、後の値が前の値を置き換えます。`merge` は別のスタイルの宣言を追加し、追加した側の宣言が元の宣言を上書きします。",
            },
        ),
        heading(
            site,
            L {
                en: "One class per declaration",
                ja: "宣言ごとに 1 つのクラス",
            },
        ),
        para(
            site,
            L {
                en: "Each declaration becomes one class, whose name comes from the declaration, such as `cursor_pointer` or `hover:color_crimson`. Panda CSS and Tailwind CSS do the same in their atomic CSS. For a value with characters other than letters, digits, `.`, `#`, `%`, and `-`, the class name uses a hash of the value. Two elements with the same declaration use the same class, so the stylesheet grows with the number of distinct declarations rather than the number of elements.",
                ja: "宣言はそれぞれ 1 つのクラスになり、クラス名は `cursor_pointer` や `hover:color_crimson` のように宣言から決まります。Panda CSS や Tailwind CSS の atomic CSS と同じ方式です。英字、数字、`.`、`#`、`%`、`-` 以外の文字を含む値では、値のハッシュをクラス名に使います。同じ宣言を持つ 2 つの要素は同じクラスを使うので、スタイルシートは要素の数ではなく異なる宣言の数に応じて大きくなります。",
            },
        ),
        para(
            site,
            L {
                en: "The page collects the styles its elements use and writes each rule once into `<head>`. A patch that contains new elements also contains their rules, inserted where a breakpoint value still overrides the base value.",
                ja: "ページは要素が使うスタイルを集め、各ルールを 1 回だけ `<head>` に書きます。新しい要素を含むパッチにはその要素のルールも含まれ、ルールはブレークポイントの値が引き続き基本の値を上書きする位置に入ります。",
            },
        ),
        heading(
            site,
            L {
                en: "Styles switched by a signal",
                ja: "シグナルで切り替わるスタイル",
            },
        ),
        para(
            site,
            L {
                en: "`css_when(style, cond)` adds a style while a signal condition holds, and `css_match(value, cases)` picks one style per value of a signal, like the variants of a Panda CSS recipe. Their rules are in a later cascade layer than those of `css`, so while switched on they override the element's own declarations for the same property.",
                ja: "`css_when(style, cond)` はシグナルの条件が成り立つ間だけスタイルを追加し、`css_match(value, cases)` はシグナルの値ごとにスタイルを 1 つ選びます（Panda CSS の recipe の variants に当たります）。これらのルールは `css` のルールより後ろのカスケードレイヤーに入るので、有効な間は同じプロパティについて要素自身の宣言を上書きします。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Cascade layer", "Holds"],
                ja: &["カスケードレイヤー", "入るもの"],
            },
            L {
                en: &[
                    &["`hypersig-tokens`", "The values a `Theme` sets on `:root`"],
                    &[
                        "`hypersig-base`",
                        "The rules of a `Global`, such as the reset",
                    ],
                    &["`hypersig-atoms`", "The classes of `El::css`"],
                    &[
                        "`hypersig-toggled`",
                        "The classes of `El::css_when` and `El::css_match`",
                    ],
                ],
                ja: &[
                    &["`hypersig-tokens`", "`Theme` が `:root` に設定する値"],
                    &["`hypersig-base`", "リセットなど、`Global` のルール"],
                    &["`hypersig-atoms`", "`El::css` のクラス"],
                    &[
                        "`hypersig-toggled`",
                        "`El::css_when` と `El::css_match` のクラス",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The layers come in that order, so an element's classes override the global rules, and a switched-on style overrides both. A stylesheet added with `Page::style` is in no layer and overrides all four layers.",
                ja: "レイヤーはこの順に並ぶので、要素のクラスはグローバルなルールを上書きし、有効になったスタイルはその両方を上書きします。`Page::style` で追加したスタイルシートはどのレイヤーにも入らず、4 つのレイヤーすべてを上書きします。",
            },
        ),
    ])
}

fn values(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Values).leads()),
        heading(
            site,
            L {
                en: "Typed values",
                ja: "型の付いた値",
            },
        ),
        para(
            site,
            L {
                en: "Where a property accepts one keyword or one value of a type in [`value`](api:hypersig_style/value/index.html), that value is typed. `display` takes a `Display`, whose variants are its keywords. `width` takes a `Width`, which converts from a length or a percentage.",
                ja: "プロパティがキーワード 1 つか、[`value`](api:hypersig_style/value/index.html) にある型の値 1 つを受け取る場合、その値には型が付きます。`display` は `Display` を受け取り、そのバリアントが `display` のキーワードです。`width` は `Width` を受け取り、`Width` は長さかパーセンテージから変換できます。",
            },
        ),
        show(Snippet::StyleValues),
        table_of(
            site,
            L {
                en: &["Value", "Written"],
                ja: &["値", "書き方"],
            },
            L {
                en: &[
                    &["Length", "`px(1)`, `rem(1.5)`, and the other units"],
                    &["Percentage", "`pct(50)`"],
                    &[
                        "Time and angle",
                        "`ms(150)`, `secs(1)`, `deg(90)`, `turn(0.5)`",
                    ],
                    &[
                        "Color",
                        "`Color::CRIMSON` and the other named colors, `Color::hex`, `Color::rgb`, `opacity`, `mix`, `Color::CURRENT_COLOR`, `Color::TRANSPARENT`",
                    ],
                    &[
                        "Text and URLs",
                        "`quoted(\"→\")` escapes the text, and `url(\"/a.png\")` writes `url(\"...\")` for a path or an `http` or `https` URL, so `url(\"javascript:...\")` in a `const` fails to compile",
                    ],
                    &[
                        "Every property",
                        "`keyword(Property::Color, CssWideKeyword::Inherit)` for `inherit`, `initial`, `unset`, `revert`, and `revert-layer`",
                    ],
                ],
                ja: &[
                    &["長さ", "`px(1)`、`rem(1.5)` などの単位"],
                    &["パーセンテージ", "`pct(50)`"],
                    &["時間と角度", "`ms(150)`、`secs(1)`、`deg(90)`、`turn(0.5)`"],
                    &[
                        "色",
                        "`Color::CRIMSON` などの名前付きの色、`Color::hex`、`Color::rgb`、`opacity`、`mix`、`Color::CURRENT_COLOR`、`Color::TRANSPARENT`",
                    ],
                    &[
                        "テキストと URL",
                        "`quoted(\"→\")` はテキストをエスケープします。`url(\"/a.png\")` はパスか `http`、`https` の URL から `url(\"...\")` を書くので、`const` の中の `url(\"javascript:...\")` はコンパイルエラーになります",
                    ],
                    &[
                        "どのプロパティでも",
                        "`inherit`、`initial`、`unset`、`revert`、`revert-layer` には `keyword(Property::Color, CssWideKeyword::Inherit)`",
                    ],
                ],
            },
        ),
        heading(
            site,
            L {
                en: "What does not compile",
                ja: "コンパイルできないもの",
            },
        ),
        para(
            site,
            L {
                en: "A misspelled property has no method, a misspelled keyword has no variant, and a value of the wrong type has no conversion:",
                ja: "綴りを間違えたプロパティにはメソッドがなく、綴りを間違えたキーワードにはバリアントがなく、型の違う値には変換がありません。",
            },
        ),
        show(Snippet::WrongValue),
        note(
            site,
            L {
                en: "Ranges are not checked, so a negative width compiles and the browser ignores the declaration. A value with several parts, such as `margin: 0 auto`, is written with `raw`, which [Unchecked values](style/unchecked/) covers.",
                ja: "値の範囲は検査しないので、負の幅もコンパイルが通り、ブラウザはその宣言を無視します。`margin: 0 auto` のように複数の部分からなる値は `raw` で書きます。`raw` は[検査しない値](style/unchecked/)で説明しています。",
            },
        ),
    ])
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

/// The styling benchmark of `bench/style`, with the numbers of its
/// `results/latest.md`, for the landing page of the Style docs.
pub fn benchmarks(site: Site) -> [Part; 4] {
    [
        para(
            site,
            L {
                en: "These figures come from the 2000-item TodoMVC page of [hypersig's framework benchmark](https://github.com/imaimai17468/hypersig/tree/main/bench/compare), styled six ways and rendered by hypersig with the same markup, so only the class names and the CSS differ. They were measured in Chrome 155 on an Apple M4 Max with the CPU slowed 4x. Browser values are medians of 5 runs, and the server render is the median of 50.",
                ja: "[hypersig のフレームワークのベンチマーク](https://github.com/imaimai17468/hypersig/tree/main/bench/compare) にある 2000 件の TodoMVC のページを、6 通りの方法でスタイルを付け、同じマークアップのまま hypersig で描画したものです。違うのはクラス名と CSS だけです。Apple M4 Max の Chrome 155 で、CPU を 4 倍に減速して測りました。ブラウザの値は 5 回、サーバーでの描画時間は 50 回の中央値です。",
            },
        ),
        table_of(
            site,
            L {
                en: &[
                    "",
                    "hypersig",
                    "Tailwind CSS",
                    "Panda CSS",
                    "StyleX",
                    "vanilla-extract",
                    "Hand-written CSS",
                ],
                ja: &[
                    "",
                    "hypersig",
                    "Tailwind CSS",
                    "Panda CSS",
                    "StyleX",
                    "vanilla-extract",
                    "手書きの CSS",
                ],
            },
            Text::Chosen::<[&[&str; 7]]>(&[
                &[
                    site.t(L {
                        en: "CSS (gzip)",
                        ja: "CSS（gzip）",
                    }),
                    "1.0 KB",
                    "1.6 KB",
                    "1.0 KB",
                    "1.0 KB",
                    "0.6 KB",
                    "0.8 KB",
                ],
                &[
                    site.t(L {
                        en: "HTML without CSS (gzip)",
                        ja: "CSS を除いた HTML（gzip）",
                    }),
                    "30.2 KB",
                    "29.1 KB",
                    "26.9 KB",
                    "26.8 KB",
                    "27.6 KB",
                    "27.3 KB",
                ],
                &[
                    site.t(L {
                        en: "Sent in all (gzip)",
                        ja: "送る合計（gzip）",
                    }),
                    "31.5 KB",
                    "30.7 KB",
                    "27.8 KB",
                    "27.8 KB",
                    "28.2 KB",
                    "28.1 KB",
                ],
                &[
                    site.t(L {
                        en: "Build of the CSS",
                        ja: "CSS のビルド",
                    }),
                    site.t(L {
                        en: "None",
                        ja: "なし",
                    }),
                    "268 ms",
                    "574 ms",
                    "182 ms",
                    "74 ms",
                    "3.1 ms",
                ],
                &[
                    site.t(L {
                        en: "Server render",
                        ja: "サーバーでの描画時間",
                    }),
                    "1.7 ms",
                    "1.5 ms",
                    "1.4 ms",
                    "1.4 ms",
                    "0.8 ms",
                    "0.8 ms",
                ],
                &[
                    "First Contentful Paint",
                    "136 ms",
                    "140 ms",
                    "136 ms",
                    "144 ms",
                    "140 ms",
                    "140 ms",
                ],
                &[
                    site.t(L {
                        en: "Style and layout on load",
                        ja: "読み込み時のスタイル計算とレイアウト",
                    }),
                    "76 ms",
                    "74 ms",
                    "73 ms",
                    "81 ms",
                    "67 ms",
                    "68 ms",
                ],
                &[
                    site.t(L {
                        en: "Marking 2000 items done",
                        ja: "2000 件を完了にする時間",
                    }),
                    "27 ms",
                    "37 ms",
                    "26 ms",
                    "27 ms",
                    "24 ms",
                    "24 ms",
                ],
            ]),
        ),
        table_of(
            site,
            L {
                en: &["Where hypersig falls behind", "Why"],
                ja: &["hypersig が不利な点", "理由"],
            },
            L {
                en: &[
                    &[
                        "HTML size",
                        "Each class name is made from its declaration alone: the conditions, the property's full name, and the value, as in `font-size_24px`. A value with a space or another character outside letters, digits, `.`, `#`, `%`, and `-` becomes a 16-digit hash. A server patch renders new elements without knowing the page's classes, so one declaration has to get the same name every time it is rendered",
                    ],
                    &[
                        "Server render",
                        "`Page::render` writes those names into every item. The HTML before gzip is 1.21 MB for hypersig and 0.88 MB for Panda CSS, and the render times grow in the same order",
                    ],
                ],
                ja: &[
                    &[
                        "HTML の大きさ",
                        "クラス名を宣言だけから作るので長くなります。名前は条件、プロパティの正式な名前、値をつないだ `font-size_24px` のような形で、空白など英数字と `.`、`#`、`%`、`-` 以外の文字を含む値は 16 桁のハッシュになります。サーバーのパッチはページにあるクラスを知らずに要素を描画するので、同じ宣言はいつ描画しても同じ名前になる必要があります",
                    ],
                    &[
                        "サーバーでの描画時間",
                        "`Page::render` がその長い名前を項目ごとに書き出します。gzip 前の HTML は hypersig が 1.21 MB、Panda CSS が 0.88 MB で、描画時間も同じ順に長くなります",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The pages and the harness are in [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style).",
                ja: "ページと計測の仕組みは [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style) にあります。",
            },
        ),
    ]
}
