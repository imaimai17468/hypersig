//! The component page on `Chart` as bars: `bar`, `stacked_bar`, and
//! `horizontal_bar`.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::BarChart).leads()),
        demo::show(site, Demo::BarChart),
        accessibility(site),
        kinds(site),
        colors(site),
        html(site),
        unchecked(site),
        example(site),
    ])
}

fn accessibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Accessibility",
                ja: "アクセシビリティ",
            },
        ),
        para(
            site,
            L {
                en: "A bar chart is the same `<figure>` as a [line chart](components/chart/): the bars, the labels, the legend, and the box under the pointer are hidden from assistive technology, and a screen reader reads a visually hidden `<table>` of the rows, captioned with the chart's label, with a column per series. The chart takes no keys.",
                ja: "棒グラフは [折れ線グラフ](components/chart/) と同じ `<figure>` です。棒、ラベル、凡例、ポインタの下の値は支援技術から隠し、スクリーンリーダーはチャートのラベルをキャプションに持ち系列ごとに列のある、視覚的に隠した行の `<table>` を読みます。チャートはキーを受け付けません。",
            },
        ),
    ])
}

fn kinds(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Three kinds of bars",
                ja: "3 種類の棒",
            },
        ),
        para(
            site,
            L {
                en: "Three constructors of [`Chart`](api:hypersig_ui/struct.Chart.html) draw bars. They take the same arguments as `Chart::line`, and the chart takes `series`, `format`, and `max_labels` as on the [Chart](components/chart/) page:",
                ja: "[`Chart`](api:hypersig_ui/struct.Chart.html) の 3 つのコンストラクタが棒を描きます。引数は `Chart::line` と同じで、`series`、`format`、`max_labels` も [チャート](components/chart/) のページと同じように使えます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> Chart<S, T> {
    pub fn bar<R: FromPage>(
        label: impl Into<String>,
        rows: Sig<S, Vec<T>, R>,
        category: Sig<T, String>,
    ) -> Self;
    pub fn stacked_bar<R: FromPage>(/* the same */) -> Self;
    pub fn horizontal_bar<R: FromPage>(/* the same */) -> Self;
}",
        ),
        table_of(
            site,
            L {
                en: &["Constructor", "What each row draws"],
                ja: &["コンストラクタ", "各行が描くもの"],
            },
            L {
                en: &[
                    &[
                        "[`Chart::bar`](api:hypersig_ui/struct.Chart.html#method.bar)",
                        "A group of bars side by side, one per series, from 0 up, or down for a negative value.",
                    ],
                    &[
                        "[`Chart::stacked_bar`](api:hypersig_ui/struct.Chart.html#method.stacked_bar)",
                        "One bar in which each series stacks on the ones added before it. Positive values stack up from 0 and negative ones down from it, so the scale reaches the largest sum of a row's positive values and the smallest sum of its negative ones.",
                    ],
                    &[
                        "[`Chart::horizontal_bar`](api:hypersig_ui/struct.Chart.html#method.horizontal_bar)",
                        "A band down the side, labeled on the left, with a bar per series running from 0 along the scale at the bottom. It suits categories whose names are too long to sit under a vertical bar.",
                    ],
                ],
                ja: &[
                    &[
                        "[`Chart::bar`](api:hypersig_ui/struct.Chart.html#method.bar)",
                        "系列ごとに 1 本の棒を横に並べたまとまり。0 から上へ、負の値なら下へ伸びます。",
                    ],
                    &[
                        "[`Chart::stacked_bar`](api:hypersig_ui/struct.Chart.html#method.stacked_bar)",
                        "1 本の棒で、各系列を先に追加した系列の上に積みます。正の値は 0 から上へ、負の値は 0 から下へ積むので、目盛りは行の正の値の和の最大と、負の値の和の最小まで届きます。",
                    ],
                    &[
                        "[`Chart::horizontal_bar`](api:hypersig_ui/struct.Chart.html#method.horizontal_bar)",
                        "左にラベルを付けた横の帯で、系列ごとの棒が下の目盛りに沿って 0 から伸びます。縦の棒の下に収まらない長さのカテゴリ名に向いています。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The scale is fitted to the values as for a line chart, on a step of 1, 2, 2.5, or 5 times a power of ten, and a statement or a patch that changes a value moves its bar and refits the scale with no request.",
                ja: "目盛りは折れ線グラフと同じく値に合わせて決まり、間隔は 10 の累乗の 1、2、2.5、5 倍です。文やパッチで値が変わると、リクエストなしに棒が動き、目盛りも合わせ直されます。",
            },
        ),
    ])
}

fn colors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Colors",
                ja: "色",
            },
        ),
        para(
            site,
            L {
                en: "Each series is drawn in the [`Token<Color>`](api:hypersig/style/struct.Token.html) it takes, usually one of `token::chart_1` to `token::chart_5`, so the bars follow the theme and dark mode. A color written as a string has no conversion to a token and is E0308:",
                ja: "各系列は受け取った [`Token<Color>`](api:hypersig/style/struct.Token.html) で描かれます。ふつうは `token::chart_1` から `token::chart_5` のどれかなので、棒はテーマとダークモードに従います。文字列で書いた色はトークンに変換できないので、E0308 になります。",
            },
        ),
        show(Snippet::UiBarColor),
    ])
}

fn html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTML",
                ja: "出力される HTML",
            },
        ),
        para(
            site,
            L {
                en: "A bar chart of two series over two months draws one `<rect>` per row and series inside the same `<svg>` as a line chart. Here the scale runs from 0 to 400, and each value is placed in hundredths of the drawing:",
                ja: "2 か月分の 2 系列の棒グラフは、折れ線グラフと同じ `<svg>` の中に、行と系列ごとに 1 つの `<rect>` を描きます。この例の目盛りは 0 から 400 までで、各値は描画の百分率で置かれます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<svg viewBox="0 0 100 100" preserveAspectRatio="none" width="100%" height="100%">
  <line x1="0" x2="100" vector-effect="non-scaling-stroke" y1="100.000" y2="100.000"></line>
  …
  <rect x="5.000" width="20.000" y="53.500" height="46.500"></rect>
  <rect x="55.000" width="20.000" y="23.750" height="76.250"></rect>
  <rect x="25.000" width="20.000" y="80.000" height="20.000"></rect>
  <rect x="75.000" width="20.000" y="50.000" height="50.000"></rect>
</svg>"#,
        ),
        para(
            site,
            L {
                en: "The first two `<rect>` elements are Desktop in January and February, and the last two are Mobile. The browser updates `y` and `height` when a value or the scale changes.",
                ja: "最初の 2 つの `<rect>` は 1 月と 2 月の Desktop で、残りの 2 つが Mobile です。値や目盛りが変わると、ブラウザが `y` と `height` を更新します。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What is not checked",
                ja: "検査されないこと",
            },
        ),
        para(
            site,
            L {
                en: "The bars of a row share its band, so many series in a bar chart draw thin bars, and nothing limits their number. A stacked bar adds values of different series, which makes sense only where they are parts of one whole; the compiler does not know. Each bar computes the scale from every row, so the work of a change grows with the square of the number of rows, as for a line chart.",
                ja: "1 行の棒はその行の帯を分け合うので、棒グラフに系列が多いと棒は細くなり、系列の数に上限はありません。積み上げ棒グラフは別々の系列の値を足すので、それらが 1 つの全体の部分であるときにしか意味がありませんが、コンパイラにはそれがわかりません。棒はそれぞれ全行から目盛りを計算するので、折れ線グラフと同じく、変更 1 回の手間は行数の 2 乗で増えます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: income and costs",
                ja: "例: 収入と費用",
            },
        ),
        para(
            site,
            L {
                en: "A stacked bar chart where the costs are negative, so each bar reaches above and below 0:",
                ja: "費用を負の値にした積み上げ棒グラフで、各棒は 0 の上と下に伸びます。",
            },
        ),
        show(Snippet::UiBarChart),
        para(
            site,
            L {
                en: "Next, [Donut chart](components/donut-chart/) draws each row's share of a total.",
                ja: "次の [ドーナツチャート](components/donut-chart/) では各行が合計に占める割合を描きます。",
            },
        ),
    ])
}
