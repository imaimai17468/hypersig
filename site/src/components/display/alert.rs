//! The Components page on `alert`: a bordered box for a message, which a
//! screen reader reads as a live region.

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
        lead(site, Doc::Components(Component::Alert).leads()),
        demo::show(site, Demo::Alert),
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
                en: "Both variants are live regions, and the variant picks the role. A destructive alert has `role=\"alert\"`, which a screen reader reads at once when the alert is put on the page, interrupting what it was reading. A default alert has `role=\"status\"`, which is read when its text changes, after what the screen reader is reading.",
                ja: "どちらのバリアントもライブリージョンで、バリアントによってロールが決まります。destructive のアラートは `role=\"alert\"` を持ち、ページに置かれるとスクリーンリーダーは読み上げ中の内容を中断してすぐに読み上げます。default のアラートは `role=\"status\"` を持ち、テキストが変わったときに、読み上げ中の内容のあとで読み上げられます。",
            },
        ),
        para(
            site,
            L {
                en: "Some screen readers skip a status that arrives with its text already in it, so keep a status box on the page and change its text with `bind_text`. Put an alert on the page with `when` or a patch, as the demo does, so that it is announced as it appears. The alert takes no keys.",
                ja: "最初からテキストが入った状態で現れた status を読み飛ばすスクリーンリーダーもあるので、status のボックスはページに置いたままにして、テキストを `bind_text` で変えてください。alert はデモのように `when` や差分でページに置くと、現れたときに読み上げられます。アラートはキー操作を受けません。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building an alert",
                ja: "アラートを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`alert`](api:hypersig_ui/fn.alert.html) returns the box, and [`alert_title`](api:hypersig_ui/fn.alert_title.html) and [`alert_description`](api:hypersig_ui/fn.alert_description.html) return the two lines that go inside it. Each is an `El<S>`, so its text comes from `El::text` or `El::bind_text`:",
                ja: "[`alert`](api:hypersig_ui/fn.alert.html) はボックスを返し、[`alert_title`](api:hypersig_ui/fn.alert_title.html) と [`alert_description`](api:hypersig_ui/fn.alert_description.html) はその中に入れる 2 つの行を返します。どれも `El<S>` なので、テキストは `El::text` か `El::bind_text` で入れます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn alert<S>(variant: AlertVariant) -> El<S>;
pub fn alert_title<S>() -> El<S>;
pub fn alert_description<S>() -> El<S>;

#[non_exhaustive]
pub enum AlertVariant {
    Default,
    Destructive,
}",
        ),
        table_of(
            site,
            L {
                en: &[
                    "[`AlertVariant`](api:hypersig_ui/enum.AlertVariant.html)",
                    "Role",
                    "Look",
                ],
                ja: &[
                    "[`AlertVariant`](api:hypersig_ui/enum.AlertVariant.html)",
                    "ロール",
                    "見た目",
                ],
            },
            L {
                en: &[
                    &[
                        "`AlertVariant::Default`",
                        "`role=\"status\"`, read when the screen reader is idle",
                        "The card colors",
                    ],
                    &[
                        "`AlertVariant::Destructive`",
                        "`role=\"alert\"`, read at once",
                        "The text in `token::destructive`",
                    ],
                ],
                ja: &[
                    &[
                        "`AlertVariant::Default`",
                        "`role=\"status\"`。スクリーンリーダーの読み上げが空いたときに読まれます",
                        "カードの色",
                    ],
                    &[
                        "`AlertVariant::Destructive`",
                        "`role=\"alert\"`。すぐに読まれます",
                        "テキストを `token::destructive` の色で描きます",
                    ],
                ],
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
                en: "The example at the end of this page renders these two boxes, with the classes left out. The title and the description are `<div>`s, so they add no heading to the page's outline:",
                ja: "このページの最後の例は次の 2 つのボックスを出力します（クラスは省いています）。タイトルと説明は `<div>` なので、ページの見出しの構造には加わりません。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div role="status">
  <div>Deployments are paused</div>
  <div>They resume at 18:00 UTC.</div>
</div>
<div role="alert">
  <div>Build failed</div>
  <div>The last commit does not compile.</div>
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
                en: "The variant decides how urgently a screen reader speaks, so `alert` takes it as an argument and has no default. An alert without one is E0061, wrong number of arguments:",
                ja: "バリアントはスクリーンリーダーがどれだけ急いで読み上げるかを決めるので、`alert` はバリアントを引数に取り、既定値はありません。バリアントのないアラートは E0061（引数の数の誤り）になります。",
            },
        ),
        show(Snippet::UiAlertVariant),
        para(
            site,
            L {
                en: "`alert_title` and `alert_description` take no arguments and hold only children, and their types stop no mistake.",
                ja: "`alert_title` と `alert_description` は引数を取らず、子要素を持つだけなので、型が防ぐ誤りはありません。",
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
                en: "Nothing checks that the box holds a title or a description, or that they come in that order. A destructive alert that is already in the first HTML is read as part of the page, and most screen readers do not announce it. A default alert whose text never changes gives a screen reader nothing to announce.",
                ja: "ボックスにタイトルや説明があるか、その順に並んでいるかは検査されません。最初の HTML にある destructive のアラートはページの一部として読まれ、多くのスクリーンリーダーはお知らせとしては読み上げません。テキストが変わらない default のアラートには、スクリーンリーダーが読み上げる変化がありません。",
            },
        ),
        para(
            site,
            L {
                en: "Many destructive alerts on one page interrupt the visitor again and again. Use `AlertVariant::Destructive` for an error that needs action now, and `AlertVariant::Default` for the rest.",
                ja: "1 つのページに destructive のアラートが多いと、訪問者は何度も読み上げを中断されます。`AlertVariant::Destructive` はすぐに対応が必要なエラーに使い、それ以外には `AlertVariant::Default` を使います。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a status and an error",
                ja: "例: 状態とエラー",
            },
        ),
        para(
            site,
            L {
                en: "The status stays on the page and binds its description to a signal. The error is put on the page by `when` while `failed` is true:",
                ja: "状態のボックスはページに置いたままにし、説明をシグナルに束縛します。エラーのボックスは、`failed` が true の間だけ `when` でページに置きます。",
            },
        ),
        show(Snippet::UiAlert),
        para(
            site,
            L {
                en: "A notice that leaves once read is a [Toast](components/toast/), and a short label such as a status next to a name is a [Badge](components/badge/). A message that needs an answer before the visitor goes on is an [Alert dialog](components/alert-dialog/).",
                ja: "読んだら消えるお知らせには [トースト](components/toast/) を、名前の横に置く状態のような短いラベルには [バッジ](components/badge/) を使います。訪問者が先に進む前に答える必要があるメッセージには [アラートダイアログ](components/alert-dialog/) を使います。",
            },
        ),
    ])
}
