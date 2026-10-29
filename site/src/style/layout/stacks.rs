//! The page on the flexbox patterns: `stack`, `vstack`, `hstack`, `wrap`,
//! `spacer`, and `divider`.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Style(StylePage::Stacks).title(site)),
        lead(site, Doc::Style(StylePage::Stacks).leads()),
        heading(
            site,
            L {
                en: "Four stacks",
                ja: "4 つのスタック",
            },
        ),
        para(
            site,
            L {
                en: "Each function of [`hypersig::style::pattern`](api:hypersig/style/pattern/index.html) returns a plain [`Style`](api:hypersig/style/struct.Style.html), and `El::css` adds it to an element like any other style. The four stacks make the element a flex container and put `gap` between its children:",
                ja: "[`hypersig::style::pattern`](api:hypersig/style/pattern/index.html) の関数はどれも普通の [`Style`](api:hypersig/style/struct.Style.html) を返し、ほかのスタイルと同じように `El::css` で要素に付けます。4 つのスタックは要素を flex コンテナにし、子の間に `gap` の間隔を空けます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "The CSS it writes"],
                ja: &["関数", "書かれる CSS"],
            },
            L {
                en: &[
                    &[
                        "[`stack(gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.stack.html)",
                        "`display: flex; flex-direction: column; gap: <gap>`",
                    ],
                    &[
                        "[`vstack(gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.vstack.html)",
                        "The declarations of `stack`, and `align-items: center`",
                    ],
                    &[
                        "[`hstack(gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.hstack.html)",
                        "`display: flex; flex-direction: row; align-items: center; gap: <gap>`",
                    ],
                    &[
                        "[`wrap(gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.wrap.html)",
                        "`display: flex; flex-wrap: wrap; gap: <gap>`",
                    ],
                ],
                ja: &[
                    &[
                        "[`stack(gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.stack.html)",
                        "`display: flex; flex-direction: column; gap: <gap>`",
                    ],
                    &[
                        "[`vstack(gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.vstack.html)",
                        "`stack` の宣言と `align-items: center`",
                    ],
                    &[
                        "[`hstack(gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.hstack.html)",
                        "`display: flex; flex-direction: row; align-items: center; gap: <gap>`",
                    ],
                    &[
                        "[`wrap(gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.wrap.html)",
                        "`display: flex; flex-wrap: wrap; gap: <gap>`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "So `stack` puts the children in a column at their full width, and `vstack` centers each child across the column at its own width. `hstack` puts them in one row, centered on its height. `wrap` leaves `flex-direction` at its initial value `row` and moves the children that do not fit onto a new line.",
                ja: "`stack` は子を縦 1 列に並べ、子は列の幅いっぱいに広がります。`vstack` は子をそれぞれの幅のまま列の中央に揃えます。`hstack` は子を横 1 行に並べ、行の高さの中央に揃えます。`wrap` は `flex-direction` を初期値の `row` のままにし、収まらない子を次の行へ折り返します。",
            },
        ),
        para(
            site,
            L {
                en: "`gap` takes a [`Gap`](api:hypersig/style/value/enum.Gap.html), which converts from a length such as `rem(1)`, a percentage such as `pct(5)`, and `raw`. `Gap::Normal` writes `gap: normal`, which is no space between the children of a flex container.",
                ja: "`gap` は [`Gap`](api:hypersig/style/value/enum.Gap.html) を受け取ります。`Gap` には `rem(1)` のような長さ、`pct(5)` のようなパーセンテージ、`raw` から変換できます。`Gap::Normal` は `gap: normal` を書き、flex コンテナではこれは子の間の間隔が 0 であることを意味します。",
            },
        ),
        heading(
            site,
            L {
                en: "Changing a pattern",
                ja: "パターンを変える",
            },
        ),
        para(
            site,
            L {
                en: "Because a pattern is a `Style`, a property method called after it replaces the pattern's value for that property, and [`merge`](api:hypersig/style/struct.Style.html#method.merge) does the same with a whole style. `hstack(rem(0.25)).align_items(AlignItems::Baseline)` is a row whose texts of two sizes share a baseline. A pattern can also be the style under a condition: below, the buttons are a column on a phone and a row from the `md` breakpoint (48rem) up, where the rules under the media query override the base ones.",
                ja: "パターンは `Style` なので、パターンの後で呼んだプロパティのメソッドはそのプロパティについてパターンの値を置き換えます。[`merge`](api:hypersig/style/struct.Style.html#method.merge) はスタイル全体について同じことをします。`hstack(rem(0.25)).align_items(AlignItems::Baseline)` は、大きさの違う 2 つのテキストのベースラインを揃える行になります。パターンは条件の下のスタイルにも使えます。下の例のボタンはスマートフォンでは縦 1 列、`md` のブレークポイント（48rem）以上では横 1 行になります。メディアクエリの下のルールが基本のルールを上書きするからです。",
            },
        ),
        show(Snippet::StackResponsive),
        heading(
            site,
            L {
                en: "Spacer and divider",
                ja: "スペーサーと区切り線",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "The CSS it writes"],
                ja: &["関数", "書かれる CSS"],
            },
            L {
                en: &[
                    &[
                        "[`spacer() -> Style`](api:hypersig/style/pattern/fn.spacer.html)",
                        "`flex-grow: 1; flex-shrink: 1; flex-basis: 0`",
                    ],
                    &[
                        "[`divider(width: Length, color: Color) -> Style`](api:hypersig/style/pattern/fn.divider.html)",
                        "`border-top-width: <width>; border-top-style: solid; border-top-color: <color>`",
                    ],
                ],
                ja: &[
                    &[
                        "[`spacer() -> Style`](api:hypersig/style/pattern/fn.spacer.html)",
                        "`flex-grow: 1; flex-shrink: 1; flex-basis: 0`",
                    ],
                    &[
                        "[`divider(width: Length, color: Color) -> Style`](api:hypersig/style/pattern/fn.divider.html)",
                        "`border-top-width: <width>; border-top-style: solid; border-top-color: <color>`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "An empty element with `spacer()` inside a stack takes the free space, so the siblings after it move to the far end of the row or the column. `divider` draws a line along the top edge of its element, as wide as the element.",
                ja: "スタックの中で `spacer()` を付けた空の要素は空いた場所を占めるので、その後の兄弟は行や列の反対の端へ移ります。`divider` は要素の上の辺に沿って、要素と同じ幅の線を引きます。",
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
                en: "`Gap` converts from values of CSS types only. A string has no conversion into it, and neither has a bare number, which carries no unit, so `stack(\"1rem\")` and `stack(16)` are E0277:",
                ja: "`Gap` は CSS の型の値からだけ変換できます。文字列からも、単位を持たないただの数からも変換できないので、`stack(\"1rem\")` も `stack(16)` も E0277 になります。",
            },
        ),
        show(Snippet::StackGap),
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
                    en: "The range of a gap. `stack(rem(-1))` compiles, the browser drops the negative `gap`, and the children touch.",
                    ja: "間隔の範囲。`stack(rem(-1))` はコンパイルが通り、ブラウザは負の `gap` を捨て、子の間に間隔がなくなります。",
                },
                L {
                    en: "Where `spacer` and `divider` sit. Outside a flex container the `flex-*` declarations of `spacer` do nothing. An empty element in an `hstack` has no width, so a `divider` on it draws no visible line; a vertical line between the items of a row takes `border_left_width` and the other `border-left-*` properties instead.",
                    ja: "`spacer` と `divider` を置く場所。flex コンテナの外では `spacer` の `flex-*` の宣言は何もしません。`hstack` の中の空の要素には幅がないので、そこに付けた `divider` の線は見えません。行の項目の間に縦の線を引くには、代わりに `border_left_width` などの `border-left-*` のプロパティを使います。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Example",
                ja: "例",
            },
        ),
        para(
            site,
            L {
                en: "A pricing card uses each function on this page:",
                ja: "下の料金プランのカードはこのページの関数をすべて使います。",
            },
        ),
        show(Snippet::Stacks),
        para(
            site,
            L {
                en: "The demo below is built the same way, with a grid of tiles under the row:",
                ja: "下のデモも同じ作りで、行の下にタイルのグリッドがあります。",
            },
        ),
        demo::show(site, Demo::Layout),
        para(
            site,
            L {
                en: "Next, [Grids](style/grids/) lays children out in columns of equal width, and [Other patterns](style/patterns/) covers centering, fixed shapes, and hidden text.",
                ja: "次の[グリッド](style/grids/)では子を等しい幅の列に並べます。[そのほかのパターン](style/patterns/)では中央寄せ、形の決まったボックス、画面に見えないテキストを説明します。",
            },
        ),
    ])
}
