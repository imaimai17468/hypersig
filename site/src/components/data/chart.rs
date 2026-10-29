//! The component page on `Chart` as lines and areas: its series, its
//! scale, its labels, and the table a screen reader reads.

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
        lead(site, Doc::Components(Component::Chart).leads()),
        demo::show(site, Demo::Chart),
        accessibility(site),
        building(site),
        series(site),
        scale(site),
        labels(site),
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
                en: "The chart is a `<figure>` named by its label. The drawing, the axis labels, the legend, and the box that shows a row's values under the pointer are hidden from assistive technology. A screen reader reads a visually hidden `<table>` of the same rows instead: its caption is the chart's label, each column is a series, and each row starts with its category as a row header. The chart takes no keys, and the values under the pointer have no keyboard equivalent beyond that table.",
                ja: "チャートはラベルを名前に持つ `<figure>` です。描画、軸のラベル、凡例、ポインタの下に出る行の値は、支援技術から隠します。代わりにスクリーンリーダーは視覚的に隠した同じ行の `<table>` を読みます。そのキャプションはチャートのラベルで、各列が系列、各行は先頭にカテゴリを行見出しとして持ちます。チャートはキーを受け付けず、ポインタの下に出る値をキーボードで見る手段は、この表のほかにありません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Lines and areas",
                ja: "折れ線と面",
            },
        ),
        para(
            site,
            L {
                en: "[`Chart::line`](api:hypersig_ui/struct.Chart.html#method.line) and [`Chart::area`](api:hypersig_ui/struct.Chart.html#method.area) take the chart's name, the list signal of its rows, and the text field that labels each row along the bottom:",
                ja: "[`Chart::line`](api:hypersig_ui/struct.Chart.html#method.line) と [`Chart::area`](api:hypersig_ui/struct.Chart.html#method.area) は、チャートの名前、行のリストのシグナル、各行に下の軸でラベルを付ける文字列のフィールドを受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> Chart<S, T> {
    pub fn line<R: FromPage>(
        label: impl Into<String>,
        rows: Sig<S, Vec<T>, R>,
        category: Sig<T, String>,
    ) -> Self;
    pub fn area<R: FromPage>(
        label: impl Into<String>,
        rows: Sig<S, Vec<T>, R>,
        category: Sig<T, String>,
    ) -> Self;
    pub fn series<U: Number + Textual + 'static>(
        self,
        name: impl Into<String>,
        field: Sig<T, U>,
        color: Token<Color>,
    ) -> Self;
    pub fn format(self, format: NumberFormat) -> Self;
    pub fn max_labels(self, count: u32) -> Self;
    pub fn render(self) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "Each row is a point along the bottom, in the order of the list, and a line joins the points of each series. An area chart also fills the region between each line and 0. `R` lets the rows come from a field of the page or from a computed list, such as the rows that pass a filter. The same `Chart` draws bars, which [Bar chart](components/bar-chart/) covers. `Chart` converts into an `El<S>` and a `Node<S>`.",
                ja: "各行はリストの順に下の軸に沿って並ぶ点で、系列ごとに点を線で結びます。面グラフはさらに各線と 0 の間を塗ります。`R` があるので、行はページのフィールドからでも、絞り込みを通った行のような計算済みのリストからでも取れます。同じ `Chart` で棒も描けます。これは [棒グラフ](components/bar-chart/) で説明します。`Chart` は `El<S>` と `Node<S>` に変換できます。",
            },
        ),
    ])
}

fn series(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Series",
                ja: "系列",
            },
        ),
        para(
            site,
            L {
                en: "[`series`](api:hypersig_ui/struct.Chart.html#method.series) adds a line drawn from a number field of the row, named for the legend and the table, in a color token. The series appear in the order they are added. The theme's five chart colors are `token::chart_1` to [`token::chart_5`](api:hypersig_ui/token/fn.chart_5.html), which change with dark mode, and any other `Token<Color>` works too.",
                ja: "[`series`](api:hypersig_ui/struct.Chart.html#method.series) は、行の数値のフィールドから描く線を、凡例と表に出す名前と色のトークンを付けて追加します。系列は追加した順に並びます。テーマのチャート用の 5 色は `token::chart_1` から [`token::chart_5`](api:hypersig_ui/token/fn.chart_5.html) までで、ダークモードに合わせて変わります。ほかの `Token<Color>` も使えます。",
            },
        ),
        para(
            site,
            L {
                en: "The field must be a number of the row type, `i32`, `i64`, or `f64`, so a text field is E0277:",
                ja: "フィールドは行の型の数値、つまり `i32`、`i64`、`f64` でなければならないので、文字列のフィールドは E0277 になります。",
            },
        ),
        show(Snippet::UiChartSeries),
    ])
}

fn scale(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The scale",
                ja: "目盛り",
            },
        ),
        para(
            site,
            L {
                en: "The scale of the values runs from a round number at or below the smallest value, or 0, to a round number at or above the largest, with grid lines on a step of 1, 2, 2.5, or 5 times a power of ten. The browser fits it to the values, so a statement or a patch that changes a row moves its point and refits the scale with no request. A negative value reaches below the line at 0.",
                ja: "値の目盛りは最小値か 0 の小さいほう以下の切りのよい数から、最大値以上の切りのよい数までで、目盛りの線の間隔は 10 の累乗の 1、2、2.5、5 倍です。目盛りはブラウザが値に合わせて決めるので、文やパッチで行が変わると、リクエストなしに点が動き、目盛りも合わせ直されます。負の値は 0 の線より下に描かれます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Values", "Scale", "Step"],
                ja: &["値", "目盛り", "間隔"],
            },
            L {
                en: &[
                    &["186, 305, 237", "0 to 400", "100"],
                    &["820", "0 to 1000", "250"],
                    &["7", "0 to 8", "2"],
                    &["120, −45", "−50 to 150", "50"],
                    &["none, or all 0", "0 to 1", "0.25"],
                ],
                ja: &[
                    &["186、305、237", "0〜400", "100"],
                    &["820", "0〜1000", "250"],
                    &["7", "0〜8", "2"],
                    &["120、−45", "−50〜150", "50"],
                    &["なし、またはすべて 0", "0〜1", "0.25"],
                ],
            },
        ),
    ])
}

fn labels(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Labels and number format",
                ja: "ラベルと数値の書式",
            },
        ),
        para(
            site,
            L {
                en: "[`format`](api:hypersig_ui/struct.Chart.html#method.format) writes the values, in the table, the box under the pointer, and the labels of the scale, in a [`NumberFormat`](api:hypersig/struct.NumberFormat.html). Until it is set they are written as JavaScript's `String` writes a number.",
                ja: "[`format`](api:hypersig_ui/struct.Chart.html#method.format) は、表、ポインタの下の値、目盛りのラベルに出る値を [`NumberFormat`](api:hypersig/struct.NumberFormat.html) の形で書きます。設定しなければ、JavaScript の `String` が数値を書く形になります。",
            },
        ),
        para(
            site,
            L {
                en: "[`max_labels`](api:hypersig_ui/struct.Chart.html#method.max_labels) labels at most `count` rows along the bottom, 12 until set. With more rows than that, it labels every `n`th row from the first, for the smallest `n` that keeps to `count`, so 30 rows with `max_labels(10)` are labeled every third row. The box under the pointer and the table still name every row. A `count` of 0 panics where `max_labels` is called:",
                ja: "[`max_labels`](api:hypersig_ui/struct.Chart.html#method.max_labels) は、下の軸にラベルを付ける行を最大 `count` 個にします。設定しなければ 12 個です。行がそれより多いときは、`count` 以内に収まる最小の `n` で、最初の行から `n` 行ごとにラベルを付けます。30 行に `max_labels(10)` なら 3 行ごとです。ポインタの下の値と表は、すべての行を示します。`count` が 0 だと、`max_labels` を呼んだところでパニックします。",
            },
        ),
        show(Snippet::UiChartLabels),
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
                en: "A line chart of one series over two months renders as follows, shortened where `…` stands:",
                ja: "2 か月分の 1 系列の折れ線グラフは、次のように描画されます。`…` の部分は省いています。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<figure aria-label="Visitors">
  <div aria-hidden="true">
    <div><span>0</span><span>100</span><span>200</span><span>300</span><span>400</span></div>
    <div>
      <svg viewBox="0 0 100 100" preserveAspectRatio="none" width="100%" height="100%">
        <line x1="0" x2="100" vector-effect="non-scaling-stroke" y1="100.000" y2="100.000"></line>
        …
        <g><line vector-effect="non-scaling-stroke" stroke-linecap="round" x1="25.000" x2="75.000" y1="53.500" y2="23.750"></line></g>
      </svg>
      <div>…</div>
    </div>
    <div></div>
    <div><span>Jan</span><span>Feb</span></div>
  </div>
  <ul aria-hidden="true"><li><span></span><span>Desktop</span></li></ul>
  <table>
    <caption>Visitors</caption>
    <thead><tr><th scope="col"></th><th scope="col">Desktop</th></tr></thead>
    <tbody>
      <tr><th scope="row">Jan</th><td>186</td></tr>
      <tr><th scope="row">Feb</th><td>305</td></tr>
    </tbody>
  </table>
</figure>"#,
        ),
        para(
            site,
            L {
                en: "The `<svg>` has `viewBox=\"0 0 100 100\"` and `preserveAspectRatio=\"none\"`, so it stretches to its box, and `vector-effect=\"non-scaling-stroke\"` keeps its lines at their width. The labels of the scale and the categories are HTML, which does not stretch. An area chart adds a `<polygon>` with a translucent fill under each line.",
                ja: "`<svg>` は `viewBox=\"0 0 100 100\"` と `preserveAspectRatio=\"none\"` を持つので枠に合わせて伸び縮みし、`vector-effect=\"non-scaling-stroke\"` が線の太さを保ちます。目盛りとカテゴリのラベルは HTML なので伸び縮みしません。面グラフは各線の下に、半透明で塗った `<polygon>` を加えます。",
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
                en: "A line joins each row to the next, so a line or area chart of one row draws no line at all, only the grid, while the table still holds the row. Each point and grid line computes the scale from every row, so the work of a change grows with the square of the number of rows. The crate has not measured where that becomes slow; a long series can be summed into fewer rows on the server first. Nothing checks that two series have different colors or names.",
                ja: "線は各行を次の行と結ぶので、1 行だけの折れ線グラフや面グラフは線を描かず、目盛りの線だけになります。表にはその行が入ります。点と目盛りの線はそれぞれ全行から目盛りを計算するので、変更 1 回の手間は行数の 2 乗で増えます。どこから遅くなるかはクレートでは測っていないので、長い系列はサーバーで先に集計して行を減らせます。2 つの系列の色や名前が違うかどうかは検査されません。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: signups over a month",
                ja: "例: 1 か月の登録者数",
            },
        ),
        para(
            site,
            L {
                en: "An area chart of two series over thirty days, with the values written as people and every third day labeled:",
                ja: "30 日分の 2 系列の面グラフで、値を人数として書き、3 日ごとにラベルを付けます。",
            },
        ),
        show(Snippet::UiChart),
        para(
            site,
            L {
                en: "Next, [Bar chart](components/bar-chart/) draws the same rows as bars.",
                ja: "次の [棒グラフ](components/bar-chart/) では同じ行を棒で描きます。",
            },
        ),
    ])
}
