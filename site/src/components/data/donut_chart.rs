//! The component page on `DonutChart`: each row an arc as long as its
//! share of the total.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::DonutChart).leads()),
        demo::show(site, Demo::DonutChart),
        accessibility(site),
        building(site),
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
                en: "The chart is a `<figure>` named by its label. The ring, the total in its middle, and the legend are hidden from assistive technology, and a screen reader reads a visually hidden `<table>` instead: captioned with the label, one row per slice with its category as the row header and its value in the cell. The chart takes no keys.",
                ja: "チャートはラベルを名前に持つ `<figure>` です。輪、その中央の合計、凡例は支援技術から隠し、代わりにスクリーンリーダーは視覚的に隠した `<table>` を読みます。表はラベルをキャプションに持ち、弧ごとに 1 行で、カテゴリを行見出しに、値をセルに入れます。チャートはキーを受け付けません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a donut chart",
                ja: "ドーナツチャートを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`DonutChart::new`](api:hypersig_ui/struct.DonutChart.html#method.new) takes the chart's name, the list signal of its rows, the text field that labels each slice, and the number field that sizes it:",
                ja: "[`DonutChart::new`](api:hypersig_ui/struct.DonutChart.html#method.new) は、チャートの名前、行のリストのシグナル、各弧にラベルを付ける文字列のフィールド、弧の大きさを決める数値のフィールドを受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S: 'static, T: 'static> DonutChart<S, T> {
    pub fn new<U: Number + Textual + 'static, R: FromPage>(
        label: impl Into<String>,
        rows: Sig<S, Vec<T>, R>,
        category: Sig<T, String>,
        value: Sig<T, U>,
    ) -> Self;
    pub fn format(self, format: NumberFormat) -> Self;
    pub fn render(self) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "Each row is an arc as long as its value over the sum of the values. The arcs run clockwise from the top in the order of the rows, in the colors `token::chart_1` to `token::chart_5`, which repeat from the first after the fifth row. The total is written in the middle of the ring, and the legend names each row with its value. [`format`](api:hypersig_ui/struct.DonutChart.html#method.format) writes the values and the total in a [`NumberFormat`](api:hypersig/struct.NumberFormat.html). A statement or a patch that changes a value resizes every arc and the total with no request. `DonutChart` converts into an `El<S>` and a `Node<S>`.",
                ja: "各行は、値が合計に占める割合の長さの弧です。弧は行の順に一番上から時計回りに並び、色は `token::chart_1` から `token::chart_5` までで、6 行目からは最初の色に戻ります。輪の中央に合計を書き、凡例は各行をその値と一緒に示します。[`format`](api:hypersig_ui/struct.DonutChart.html#method.format) は値と合計を [`NumberFormat`](api:hypersig/struct.NumberFormat.html) の形で書きます。文やパッチで値が変わると、リクエストなしにすべての弧と合計が更新されます。`DonutChart` は `El<S>` と `Node<S>` に変換できます。",
            },
        ),
        para(
            site,
            L {
                en: "`value` must be a number field of the row type, so a text field is E0277:",
                ja: "`value` は行の型の数値のフィールドでなければならないので、文字列のフィールドは E0277 になります。",
            },
        ),
        show(Snippet::UiDonutValue),
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
                en: "A chart of 275 Chrome visitors and 125 Safari visitors renders as:",
                ja: "Chrome の訪問者 275 人と Safari の訪問者 125 人のチャートは、次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<figure aria-label="Browsers">
  <div aria-hidden="true">
    <svg viewBox="0 0 100 100" width="100%" height="100%">
      <circle cx="50" cy="50" r="40" fill="none" stroke-width="16"></circle>
      <circle cx="50" cy="50" r="40" fill="none" stroke-width="16" pathLength="100" stroke-dasharray="68.750 31.250" stroke-dashoffset="0.000"></circle>
      <circle cx="50" cy="50" r="40" fill="none" stroke-width="16" pathLength="100" stroke-dasharray="31.250 68.750" stroke-dashoffset="-68.750"></circle>
    </svg>
    <div><span>400</span></div>
  </div>
  <ul aria-hidden="true">
    <li><span></span><span>Chrome</span><span>275</span></li>
    <li><span></span><span>Safari</span><span>125</span></li>
  </ul>
  <table>
    <caption>Browsers</caption>
    <thead><tr><th scope="col"></th><th scope="col">Browsers</th></tr></thead>
    <tbody>
      <tr><th scope="row">Chrome</th><td>275</td></tr>
      <tr><th scope="row">Safari</th><td>125</td></tr>
    </tbody>
  </table>
</figure>"#,
        ),
        para(
            site,
            L {
                en: "The first `<circle>` is the ring behind the arcs, in `token::muted`. Each arc is a circle with `pathLength=\"100\"`, whose `stroke-dasharray` is the row's share of the total in hundredths and the rest, and whose `stroke-dashoffset` starts it where the rows before it end: 275 of 400 is 68.75 hundredths.",
                ja: "最初の `<circle>` は弧の後ろにある `token::muted` の色の輪です。各弧は `pathLength=\"100\"` を持つ円で、`stroke-dasharray` は合計に占める行の割合の百分率と残りの長さ、`stroke-dashoffset` は前の行が終わる位置から弧を始めるための値です。400 のうち 275 は 68.75 です。",
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
                en: "A negative value counts as 0, since a share cannot be negative: its row draws no arc and adds nothing to the total, while the legend and the table still show the negative value. With every value at 0, the ring draws no arcs and the total reads 0. After the fifth row the colors repeat, so the sixth slice has the first slice's color and only the legend's order tells them apart. Each arc sums the rows before it, so the work of a change grows with the square of the number of rows.",
                ja: "割合は負になれないので、負の値は 0 として数えます。その行は弧を描かず合計にも加わりませんが、凡例と表には負の値がそのまま出ます。すべての値が 0 のときは、輪に弧が描かれず、合計は 0 になります。6 行目からは色が繰り返されるので、6 つ目の弧は 1 つ目と同じ色になり、区別できるのは凡例の順番だけです。各弧はそれより前の行を足すので、変更 1 回の手間は行数の 2 乗で増えます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: revenue by plan",
                ja: "例: プランごとの売上",
            },
        ),
        para(
            site,
            L {
                en: "Three plans, with the values and the total written in dollars:",
                ja: "3 つのプランで、値と合計をドルで書きます。",
            },
        ),
        show(Snippet::UiDonutChart),
        para(
            site,
            L {
                en: "The [Overlays](components/dialog/) pages come next, starting with Dialog.",
                ja: "次は [ダイアログ](components/dialog/) から始まる、重ねて表示するコンポーネントのページです。",
            },
        ),
    ])
}
