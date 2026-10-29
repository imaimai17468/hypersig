//! The page on view transitions: between documents with `@view-transition`,
//! and within a page with `Stmt::view_transition`.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Style(StylePage::PageTransitions).title(site)),
        lead(site, Doc::Style(StylePage::PageTransitions).leads()),
        heading(
            site,
            L {
                en: "From one page to the next",
                ja: "ページから次のページへ",
            },
        ),
        para(
            site,
            L {
                en: "A link from one page to another loads the new page, so the browser's cross-document view transitions animate the change. A page opts in with the CSS rule `@view-transition { navigation: auto; }`, and both the page left and the page reached need it. The browser captures a snapshot of the old page, shows the new one, and cross-fades between the two unless `::view-transition-old(root)` and `::view-transition-new(root)` set other animations. `Style` has no at-rule or pseudo-element for these, so they go in a [`Stylesheet`](api:hypersig/html/struct.Stylesheet.html), which `Page::style` adds to `<head>`:",
                ja: "あるページから別のページへのリンクは新しいページを読み込むので、その切り替わりはブラウザのクロスドキュメントの View Transitions でアニメーションできます。ページは CSS の `@view-transition { navigation: auto; }` で有効にし、移る前のページと移った先のページの両方にこのルールが必要です。ブラウザは前のページのスナップショットを取って新しいページを表示し、`::view-transition-old(root)` と `::view-transition-new(root)` で別のアニメーションを指定しない限り、2 つをクロスフェードでつなぎます。`Style` にはこのアットルールも擬似要素もないので、[`Stylesheet`](api:hypersig/html/struct.Stylesheet.html) に書き、`Page::style` で `<head>` に入れます。",
            },
        ),
        show(Snippet::PageTransitions),
        para(
            site,
            L {
                en: "Chrome and Edge play cross-document view transitions from version 126, and Safari from 18.2. A browser without them changes the page as usual.",
                ja: "クロスドキュメントの View Transitions は、Chrome と Edge では 126 から、Safari では 18.2 から動きます。対応していないブラウザでは、ページは通常どおり切り替わります。",
            },
        ),
        heading(
            site,
            L {
                en: "Elements that move on their own",
                ja: "ほかとは別に動く要素",
            },
        ),
        para(
            site,
            L {
                en: "An element with a `view-transition-name` moves on its own, from where it was on the old page to where it is on the new one. [`Style::view_transition_name`](api:hypersig/style/struct.Style.html#method.view_transition_name) takes a [`ViewTransitionName`](api:hypersig/style/value/enum.ViewTransitionName.html): `None`, `MatchElement`, or a name through `raw`. In the sample above, `view_transition_name(raw(\"site-header\"))` writes `view-transition-name: site-header`, and the header, which keeps its place on both pages, stays still while the rest of the page changes.",
                ja: "`view-transition-name` を持つ要素は前のページでの位置から新しいページでの位置へ、ほかとは別に動きます。[`Style::view_transition_name`](api:hypersig/style/struct.Style.html#method.view_transition_name) は [`ViewTransitionName`](api:hypersig/style/value/enum.ViewTransitionName.html) を受け取ります。`None`、`MatchElement`、または `raw` で渡す名前です。上のサンプルの `view_transition_name(raw(\"site-header\"))` は `view-transition-name: site-header` を書きます。ヘッダーは両方のページで同じ位置にあるので、ページのほかの部分が切り替わるあいだ動きません。",
            },
        ),
        para(
            site,
            L {
                en: "`ViewTransitionName` has no conversion from a string, so a name written by hand is E0277:",
                ja: "`ViewTransitionName` は文字列から変換できないので、手で書いた名前は E0277 になります。",
            },
        ),
        show(Snippet::ViewTransitionName),
        heading(
            site,
            L {
                en: "Within a page",
                ja: "ページの中",
            },
        ),
        para(
            site,
            L {
                en: "A statement changes the page without loading another, and a patch from the server changes it in place, so neither plays a view transition on its own. [`Stmt::view_transition(body: impl IntoStmt<S>) -> Stmt<S>`](api:hypersig/struct.Stmt.html#method.view_transition) runs `body` inside `document.startViewTransition`: the browser captures a snapshot of the page, `body` runs and its writes render, and the two snapshots cross-fade unless the page's CSS sets other animations. The theme button in this site's header changes its colors this way.",
                ja: "文は別のページを読み込まずにページを書き換え、サーバーからのパッチもページをその場で書き換えるので、どちらもそれだけでは View Transitions を起こしません。[`Stmt::view_transition(body: impl IntoStmt<S>) -> Stmt<S>`](api:hypersig/struct.Stmt.html#method.view_transition) は `body` を `document.startViewTransition` の中で実行します。ブラウザがページのスナップショットを取り、`body` が実行されてその書き込みが描画されると、ページの CSS で別のアニメーションを指定しない限り、2 つのスナップショットがクロスフェードでつながります。このサイトのヘッダーにある配色のボタンも、この方法で色を切り替えています。",
            },
        ),
        show(Snippet::PageTransitionStmt),
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
                L {
                    en: "An error thrown while `body` runs is written to the console with the prefix `hypersig:`, and the transition still finishes.",
                    ja: "`body` の実行中に投げられたエラーは、`hypersig:` を先頭に付けてコンソールに書かれ、遷移はそのまま終わります。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型が検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The CSS of the `Stylesheet`. `Stylesheet::new` refuses only `</style`, and the browser drops a rule it cannot parse. A page that lacks `@view-transition` changes without an animation.",
                    ja: "`Stylesheet` の CSS。`Stylesheet::new` が受け付けないのは `</style` だけで、ブラウザは解析できないルールを捨てます。`@view-transition` のないページはアニメーションなしで切り替わります。",
                },
                L {
                    en: "Unique names. The name in `raw` is not checked, and two elements of one page with the same `view-transition-name` make the browser skip the transition.",
                    ja: "名前の重複。`raw` の中の名前は検査されません。1 つのページで 2 つの要素が同じ `view-transition-name` を持つと、ブラウザは遷移を飛ばします。",
                },
                L {
                    en: "Reduced motion. The browser plays view transitions for a visitor who asks for reduced motion too, and [Reduced motion](style/reduced-motion/) shows the rules that turn them off.",
                    ja: "動きを減らす設定。ブラウザは動きを減らすよう求める訪問者にも View Transitions を再生します。それを止めるルールは[動きを減らす設定](style/reduced-motion/)で示します。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Next, [Reduced motion](style/reduced-motion/) changes these animations for a visitor who asks for less motion.",
                ja: "次の[動きを減らす設定](style/reduced-motion/)では、動きを減らすよう求める訪問者に合わせてこれらのアニメーションを変えます。",
            },
        ),
    ])
}
