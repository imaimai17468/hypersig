//! The component page on `Accordion`: sections built on `<details>`, open
//! independently or one at a time.

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
        lead(site, Doc::Components(Component::Accordion).leads()),
        demo::show(site, Demo::Accordion),
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
                en: "Each section is a native `<details>` whose `<summary>` opens and closes it, so the browser handles the keyboard and a screen reader announces each summary as a button that is expanded or collapsed. The browser's find-in-page opens a closed section that holds a match.",
                ja: "各セクションはネイティブの `<details>` で、その `<summary>` がセクションを開閉します。そのためキーボード操作はブラウザが扱い、スクリーンリーダーは各 summary を、展開か折りたたみの状態を持つボタンとして読み上げます。ブラウザのページ内検索で一致した箇所が閉じたセクションにあれば、そのセクションが開きます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            L {
                en: &[
                    &["Tab", "Moves focus to the next section's summary."],
                    &["Enter, Space", "Opens or closes the focused section."],
                ],
                ja: &[
                    &["Tab", "次のセクションの summary にフォーカスを移します。"],
                    &[
                        "Enter、Space",
                        "フォーカスのあるセクションを開くか閉じます。",
                    ],
                ],
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building an accordion",
                ja: "アコーディオンを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`Accordion`](api:hypersig_ui/struct.Accordion.html) starts empty, and [`item`](api:hypersig_ui/struct.Accordion.html#method.item) adds one section after the others:",
                ja: "[`Accordion`](api:hypersig_ui/struct.Accordion.html) は空の状態から始まり、[`item`](api:hypersig_ui/struct.Accordion.html#method.item) が既存のセクションの後ろにセクションを 1 つ追加します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Accordion<S> {
    pub fn new() -> Self;
    pub fn exclusive(group: Id) -> Self;
    pub fn item(self, title: impl Into<String>, content: impl Into<Node<S>>) -> Self;
    pub fn render(self) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "[`Accordion::new`](api:hypersig_ui/struct.Accordion.html#method.new), which `Default` also returns, opens and closes each section on its own. [`Accordion::exclusive`](api:hypersig_ui/struct.Accordion.html#method.exclusive) keeps at most one open: opening a section closes the one that was open. `Accordion` converts into an `El<S>` and a `Node<S>`, so `El::child` takes it without `render`.",
                ja: "[`Accordion::new`](api:hypersig_ui/struct.Accordion.html#method.new)（`Default` も同じものを返します）は、各セクションを独立に開閉します。[`Accordion::exclusive`](api:hypersig_ui/struct.Accordion.html#method.exclusive) は開いているセクションを多くても 1 つにし、セクションを開くと、それまで開いていたセクションが閉じます。`Accordion` は `El<S>` と `Node<S>` に変換できるので、`render` を呼ばずに `El::child` に渡せます。",
            },
        ),
        para(
            site,
            L {
                en: "The title and the content of a section are any text and any element, so `item` stops no mistake of its own. `exclusive` takes an [`Id`](api:hypersig/html/struct.Id.html), the type of element ids, rather than a string, because the name also becomes the id of the accordion's `<div>`. A string literal is E0308:",
                ja: "セクションのタイトルと中身は任意の文字列と任意の要素なので、`item` 自体が防ぐ間違いはありません。`exclusive` は文字列ではなく要素の id の型である [`Id`](api:hypersig/html/struct.Id.html) を受け取ります。この名前はアコーディオンの `<div>` の id にもなるからです。文字列リテラルを渡すと E0308 になります。",
            },
        ),
        show(Snippet::UiAccordionName),
        para(
            site,
            L {
                en: "Which sections are open lives in the browser's `<details>` elements, not in a signal, so neither the server nor another statement of the page reads or sets it. Where the page needs to know or change it, use a [Collapsible](components/collapsible/), which is bound to a `bool` signal.",
                ja: "どのセクションが開いているかは、シグナルではなくブラウザの `<details>` 要素が持つので、サーバーもページのほかの文もそれを読んだり設定したりできません。ページがそれを知る必要や変える必要があるときは、`bool` のシグナルに束縛する [折りたたみ](components/collapsible/) を使います。",
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
                en: "An exclusive accordion built with `Id::new(\"shipping\")` renders the id on its `<div>` and the same text as the `name` of each `<details>`. An accordion from `new` has neither attribute. The chevron is an `<svg>` hidden from assistive technology:",
                ja: "`Id::new(\"shipping\")` で作った排他的なアコーディオンは、`<div>` にその id を、各 `<details>` に同じ文字列の `name` を出力します。`new` で作ったアコーディオンには、どちらの属性もありません。山形の印は支援技術から隠した `<svg>` です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div id="shipping">
  <details name="shipping">
    <summary>How long does delivery take?<svg aria-hidden="true" …></svg></summary>
    <div><p>Three to five working days.</p></div>
  </details>
  <details name="shipping">
    <summary>Can I change the address?<svg aria-hidden="true" …></svg></summary>
    <div><p>Until the parcel leaves the warehouse.</p></div>
  </details>
</div>"#,
        ),
        para(
            site,
            L {
                en: "Every section starts closed, and the browser closes the other `<details>` of one `name` when one of them opens. Opening and closing a section sends no request and runs no statement.",
                ja: "どのセクションも閉じた状態で始まり、同じ `name` の `<details>` の 1 つが開くと、ブラウザがほかを閉じます。セクションの開閉ではリクエストは送られず、文も実行されません。",
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
                en: "The browser groups every `<details>` of one `name` on the page, wherever it is. The `<div>` carries the name as its id, so two exclusive accordions given one `Id::new` are two elements with one id, and `Page::render` panics on them rather than letting them close each other's sections:",
                ja: "ブラウザはページのどこにあるかに関係なく、同じ `name` の `<details>` をすべてまとめて扱います。`<div>` はこの名前を id に持つので、1 つの `Id::new` を 2 つの排他的なアコーディオンに渡すと、同じ id の要素が 2 つできます。このとき、互いのセクションを閉じ合う代わりに `Page::render` がパニックします。",
            },
        ),
        show(Snippet::UiAccordionTwice),
        para(
            site,
            L {
                en: "The same holds for an id from `Id::with_key`. An id from `Id::from_text`, which may come from stored data, is not checked for a duplicate, so two accordions given one such id render without a panic and close each other's sections. Give each exclusive accordion its own `const` `Id`.",
                ja: "`Id::with_key` で作った id も同じです。保存したデータから作ることのある `Id::from_text` の id は重複が検査されないので、同じ id を 2 つのアコーディオンに渡すと、パニックせずに描画され、互いのセクションを閉じ合います。排他的なアコーディオンにはそれぞれ別の `const` の `Id` を渡してください。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a help page",
                ja: "例: ヘルプページ",
            },
        ),
        para(
            site,
            L {
                en: "The release notes open independently, and the shipping questions open one at a time:",
                ja: "リリースノートはそれぞれ独立に開き、配送に関する質問は一度に 1 つだけ開きます。",
            },
        ),
        show(Snippet::UiAccordion),
        para(
            site,
            L {
                en: "Next, [Collapsible](components/collapsible/) shows and hides one region from a `bool` signal.",
                ja: "次の [折りたたみ](components/collapsible/) では `bool` のシグナルで 1 つの領域を表示したり隠したりします。",
            },
        ),
    ])
}
