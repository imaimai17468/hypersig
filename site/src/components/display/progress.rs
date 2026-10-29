//! The Components page on `progress`: a bar filled up to a percentage that a
//! signal holds.

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
        lead(site, Doc::Components(Component::Progress).leads()),
        demo::show(site, Demo::Progress),
        accessibility(site),
        building(site),
        html(site),
        compile_errors(site),
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
                en: "The bar is a `role=\"progressbar\"` named by its label through `aria-label`, with `aria-valuemin=\"0\"`, `aria-valuemax=\"100\"`, and `aria-valuenow` kept equal to the value. A screen reader reads the label and the percentage where it reaches the bar. Whether it announces each change of the value differs between screen readers, so say in text when the work finishes. The bar takes no keys.",
                ja: "バーは `aria-label` でラベルを名前にした `role=\"progressbar\"` で、`aria-valuemin=\"0\"` と `aria-valuemax=\"100\"` を持ち、`aria-valuenow` は常に値と同じに保たれます。スクリーンリーダーはバーまで読み進めたときに、ラベルと割合を読み上げます。値が変わるたびに読み上げるかどうかはスクリーンリーダーによって違うので、処理が終わったことはテキストで伝えてください。バーはキー操作を受けません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a progress bar",
                ja: "プログレスバーを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`progress`](api:hypersig_ui/fn.progress.html) takes a label and a percentage from 0 to 100, as an `i32` or any expression of one, such as a signal:",
                ja: "[`progress`](api:hypersig_ui/fn.progress.html) は、ラベルと、0 から 100 までの割合を受け取ります。割合は `i32` か、シグナルのような `i32` の式です。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn progress<S>(label: impl Into<String>, percent: impl IntoExpr<S, i32>) -> El<S>;",
        ),
        para(
            site,
            L {
                en: "With a signal, the browser moves the bar and updates `aria-valuenow` as the signal changes, and a handler moves it with a patch on the signal. The fill moves with a short transition.",
                ja: "シグナルを渡すと、シグナルが変わるたびにブラウザがバーを動かし、`aria-valuenow` を更新します。ハンドラからはそのシグナルへの差分でバーを動かせます。塗りは短いトランジションで動きます。",
            },
        ),
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
                en: "With `percent` at 40, the example at the end of this page renders this bar, with the classes left out. The inner `<div>` is the fill:",
                ja: "`percent` が 40 のとき、このページの最後の例は次のバーを出力します（クラスは省いています）。内側の `<div>` が塗りの部分です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div role="progressbar" aria-label="Uploading report.pdf" aria-valuemin="0" aria-valuemax="100" aria-valuenow="40">
  <div style="width: 40%"></div>
</div>"#,
        ),
    ])
}

fn compile_errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What fails to compile",
                ja: "コンパイルエラーになるもの",
            },
        ),
        para(
            site,
            L {
                en: "The percentage is a whole number, so a fraction held in an `f64` signal is E0277, with a message that names the type it found. Converting it is the caller's choice, such as `(done * 100.0).round() as i32` in Rust before the value reaches the page:",
                ja: "割合は整数なので、`f64` のシグナルに入った分数を渡すと E0277 になります。エラーメッセージは渡した型を示します。どう変換するかは呼び出す側が決めます。たとえば、値がページに届く前に Rust で `(done * 100.0).round() as i32` のように変換します。",
            },
        ),
        show(Snippet::UiProgressFraction),
        para(
            site,
            L {
                en: "The label is a required argument, so a bar without a name does not compile either: leaving it out is E0061.",
                ja: "ラベルは必須の引数なので、名前のないバーもコンパイルできません。省くと E0061 になります。",
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
                en: "The range is not checked by the types. A value below 0 or above 100 is drawn and announced as 0 or 100, so a bug that sends 140 shows a full bar rather than an error. An empty label compiles and leaves the bar with no name.",
                ja: "範囲は型では検査されません。0 未満や 100 を超える値は 0 や 100 として描かれ、読み上げられます。そのため、140 を送ってしまうバグがあっても、エラーにはならず、満杯のバーが表示されます。空のラベルもコンパイルでき、その場合バーには名前がありません。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: an upload",
                ja: "例: アップロード",
            },
        ),
        para(
            site,
            L {
                en: "Two buttons move the bar by ten points. Going past 0 or 100 leaves the bar at its end:",
                ja: "2 つのボタンでバーを 10 ずつ動かします。0 や 100 を越えても、バーは端で止まります。",
            },
        ),
        show(Snippet::UiProgress),
        para(
            site,
            L {
                en: "Work whose length is not known shows a [Spinner](components/spinner/), and content that is still loading can show a [Skeleton](components/skeleton/) in its place.",
                ja: "どれだけかかるかわからない処理には [スピナー](components/spinner/) を、読み込み中の内容の代わりには [スケルトン](components/skeleton/) を表示できます。",
            },
        ),
    ])
}
