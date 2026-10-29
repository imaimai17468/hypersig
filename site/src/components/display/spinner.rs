//! The Components page on `spinner`: a turning ring that marks work whose
//! length is not known.

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
        lead(site, Doc::Components(Component::Spinner).leads()),
        demo::show(site, Demo::Loading),
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
                en: "The spinner is a `role=\"status\"` named by its label through `aria-label`, and the ring inside it is hidden from assistive technology. A screen reader reads the label where it reaches the spinner. Whether it also reads the label as the spinner appears differs between screen readers, so say in text when the work finishes, such as in a [Toast](components/toast/).",
                ja: "スピナーは `aria-label` でラベルを名前にした `role=\"status\"` で、中の輪は支援技術から隠されています。スクリーンリーダーはスピナーまで読み進めたときにラベルを読み上げます。スピナーが現れたときにも読み上げるかどうかはスクリーンリーダーによって違うので、処理が終わったことは [トースト](components/toast/) などのテキストで伝えてください。",
            },
        ),
        para(
            site,
            L {
                en: "The ring turns once a second in the current text color, and four times slower for a visitor who asks for reduced motion. The spinner takes no keys.",
                ja: "輪は現在の文字色で 1 秒に 1 回転し、動きを減らす設定の訪問者には 4 倍遅く回ります。スピナーはキー操作を受けません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a spinner",
                ja: "スピナーを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`spinner`](api:hypersig_ui/fn.spinner.html) takes the label a screen reader reads, such as \"Saving\", and returns a `<span>` that sits in a line of text. Show it while the work runs with `El::show` on a signal, or with `El::show` on a call's busy state:",
                ja: "[`spinner`](api:hypersig_ui/fn.spinner.html) は「保存中」のような、スクリーンリーダーが読むラベルを受け取り、文の行の中に置ける `<span>` を返します。処理の間だけ表示するには、シグナルか呼び出し中の状態に対して `El::show` を使います。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn spinner<S>(label: impl Into<String>) -> El<S>;",
        ),
        para(
            site,
            L {
                en: "The ring is `1rem` wide and takes the color of the text around it, so a spinner inside a button follows the button's text color. [Busy and failed calls](learn/call-status/) covers a call's busy state.",
                ja: "輪の幅は `1rem` で、周りのテキストの色になるので、ボタンの中のスピナーはボタンの文字色に従います。呼び出し中の状態は [呼び出し中と失敗](learn/call-status/) で説明しています。",
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
                en: "While `busy` is true, the example at the end of this page renders this spinner, with the classes left out. The paragraph that says the sync is done is in the HTML too, hidden while `busy` is true, and is left out here:",
                ja: "`busy` が true の間、このページの最後の例は次のスピナーを出力します（クラスは省いています）。同期が終わったことを伝える段落も HTML にありますが、`busy` が true の間は隠れているので、ここでは省いています。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<span role="status" aria-label="Syncing">
  <svg aria-hidden="true" …></svg>
</span>"#,
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
                en: "A spinner has no text of its own, so its label is a required argument, and a spinner without one is E0061, wrong number of arguments. A screen reader would otherwise find a status with nothing to read:",
                ja: "スピナーには自身のテキストがないので、ラベルは必須の引数で、ラベルのないスピナーは E0061（引数の数の誤り）になります。ラベルがなければ、スクリーンリーダーは読むもののない status を見つけることになります。",
            },
        ),
        show(Snippet::UiSpinnerLabel),
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
                en: "Nothing checks that the spinner goes away. A spinner shown with no condition turns forever, and its status says the work is running after it has finished. An empty label compiles and leaves the status with no name.",
                ja: "スピナーが消えるかどうかは検査されません。条件なしで表示したスピナーは回り続け、処理が終わったあとも、その status は処理中だと伝え続けます。空のラベルもコンパイルでき、その場合 status には名前がありません。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: syncing",
                ja: "例: 同期",
            },
        ),
        para(
            site,
            L {
                en: "The spinner shows while `busy` is true, and a line of text takes its place when the work is done:",
                ja: "`busy` が true の間はスピナーを表示し、処理が終わるとテキストの行に入れ替えます。",
            },
        ),
        show(Snippet::UiSpinner),
        para(
            site,
            L {
                en: "Work whose share done is known shows a [Progress](components/progress/) bar, and content still loading can show a [Skeleton](components/skeleton/). The [Keyframes](style/animation/) page of the Style section builds a spinner of its own from keyframes.",
                ja: "進み具合がわかる処理には [プログレス](components/progress/) のバーを、読み込み中の内容の代わりには [スケルトン](components/skeleton/) を表示できます。スタイルのセクションの [キーフレーム](style/animation/) のページでは、キーフレームからスピナーを作っています。",
            },
        ),
    ])
}
