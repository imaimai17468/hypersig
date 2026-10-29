//! The page on the grid patterns: `grid` and `grid_item`.

use crate::Site;
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Style(StylePage::Grids).title(site)),
        lead(site, Doc::Style(StylePage::Grids).leads()),
        heading(
            site,
            L {
                en: "Equal columns",
                ja: "等しい幅の列",
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
                        "[`grid(columns: u16, gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.grid.html)",
                        "`display: grid; grid-template-columns: repeat(<columns>, minmax(0, 1fr)); gap: <gap>`",
                    ],
                    &[
                        "[`grid_item(columns: u16) -> Style`](api:hypersig/style/pattern/fn.grid_item.html)",
                        "`grid-column: span <columns> / span <columns>`",
                    ],
                ],
                ja: &[
                    &[
                        "[`grid(columns: u16, gap: impl Into<Gap>) -> Style`](api:hypersig/style/pattern/fn.grid.html)",
                        "`display: grid; grid-template-columns: repeat(<columns>, minmax(0, 1fr)); gap: <gap>`",
                    ],
                    &[
                        "[`grid_item(columns: u16) -> Style`](api:hypersig/style/pattern/fn.grid_item.html)",
                        "`grid-column: span <columns> / span <columns>`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each column is `minmax(0, 1fr)`: its smallest width is 0, so a long word or a wide table inside a child wraps or overflows inside its column, and the columns keep equal widths. With plain `1fr`, the column would widen to the child's content. The children fill the columns row by row, and the browser adds rows as needed. `grid_item` goes on a child and makes it span several columns of its parent's grid. The gap takes the same values as the gap of a stack.",
                ja: "列はどれも `minmax(0, 1fr)` です。列の最小の幅が 0 なので、子の中の長い単語や幅の広い表は列の中で折り返すかはみ出し、列は等しい幅を保ちます。ただの `1fr` なら、列は子の内容の幅まで広がります。子は列を行ごとに埋めていき、ブラウザは必要なだけ行を足します。`grid_item` は子に付け、親のグリッドの複数の列にまたがらせます。間隔にはスタックの間隔と同じ値を使えます。",
            },
        ),
        heading(
            site,
            L {
                en: "Columns by breakpoint",
                ja: "ブレークポイントごとの列",
            },
        ),
        para(
            site,
            L {
                en: "A grid under [`Media::Md`](style/media/) replaces the column count from 48rem up, and `grid_item` under the same condition spans columns only there:",
                ja: "[`Media::Md`](style/media/) の下に置いたグリッドは、48rem 以上で列の数を置き換えます。同じ条件の下の `grid_item` はその幅でだけ列にまたがります。",
            },
        ),
        show(Snippet::Grids),
        para(
            site,
            L {
                en: "The base rule writes `repeat(1, minmax(0, 1fr))`, and the rule inside `@media (min-width: 48rem)` writes `repeat(4, minmax(0, 1fr))`, which wins at that width because it comes later in the stylesheet.",
                ja: "基本のルールは `repeat(1, minmax(0, 1fr))` を書き、`@media (min-width: 48rem)` の中のルールは `repeat(4, minmax(0, 1fr))` を書きます。後者はスタイルシートで後にあるので、その幅では後者が使われます。",
            },
        ),
        heading(
            site,
            L {
                en: "Columns from a signal",
                ja: "シグナルで決まる列",
            },
        ),
        para(
            site,
            L {
                en: "`grid` takes the column count as a `u16` in Rust, so a count that changes in the browser picks one of several typed grids with [`El::css_match`](style/toggled/). Its cases override the element's own `css` while they apply:",
                ja: "`grid` は列の数を Rust の `u16` で受け取ります。ブラウザで変わる列の数は、型の付いた複数のグリッドから [`El::css_match`](style/toggled/) で 1 つを選んで表します。適用中のケースは要素自身の `css` を上書きします。",
            },
        ),
        show(Snippet::GridSignal),
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
                en: "The column count is a whole number. A fraction or a negative number is not a `u16`:",
                ja: "列の数は整数です。小数や負の数は `u16` ではありません。",
            },
        ),
        show(Snippet::GridColumns),
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
                    en: "A count of 0. `grid(0, gap)` writes `repeat(0, minmax(0, 1fr))` and `grid_item(0)` writes `span 0 / span 0`. The browser drops both declarations, so the grid has one column and the child spans one.",
                    ja: "0 という数。`grid(0, gap)` は `repeat(0, minmax(0, 1fr))` を、`grid_item(0)` は `span 0 / span 0` を書きます。ブラウザはどちらの宣言も捨てるので、グリッドは 1 列になり、子は 1 列だけを占めます。",
                },
                L {
                    en: "A span wider than the grid. `grid_item(3)` in a `grid(2, ..)` makes the browser add a column outside the template for it, so the grid no longer has two equal columns.",
                    ja: "グリッドより広いまたがり。`grid(2, ..)` の中の `grid_item(3)` に合わせて、ブラウザはテンプレートの外に列を足すので、グリッドは等しい幅の 2 列ではなくなります。",
                },
                L {
                    en: "Templates of other shapes. A sidebar layout such as `200px minmax(0, 1fr)` takes `grid_template_columns(raw(\"200px minmax(0, 1fr)\"))`, which [Unchecked values](style/unchecked/) covers.",
                    ja: "ほかの形のテンプレート。`200px minmax(0, 1fr)` のようなサイドバーのレイアウトは `grid_template_columns(raw(\"200px minmax(0, 1fr)\"))` で書きます。`raw` は[検査しない値](style/unchecked/)で説明しています。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Next, [Other patterns](style/patterns/) covers centering, fixed shapes, a link that covers a card, and hidden text.",
                ja: "次の[そのほかのパターン](style/patterns/)では中央寄せ、形の決まったボックス、カードを覆うリンク、画面に見えないテキストを説明します。",
            },
        ),
    ])
}
