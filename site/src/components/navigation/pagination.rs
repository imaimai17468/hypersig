//! The component page on `pagination` and `PaginationText`: links to the
//! pages of a long list, built on the server for the page shown.

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
        lead(site, Doc::Components(Component::Pagination).leads()),
        demo::show(site, Demo::Pagination),
        accessibility(site),
        building(site),
        words(site),
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
                en: "The links sit in a `<nav>` named by its label, so a screen reader lists them among the page's landmarks. The link to the current page carries `aria-current=\"page\"`. The links to the page before and after carry `rel=\"prev\"` and `rel=\"next\"` and say Previous and Next in words. On the first page, Previous is text marked `aria-disabled=\"true\"` rather than a link, and so is Next on the last, so Tab skips it. The `…` that stands for the pages left out is hidden from assistive technology.",
                ja: "リンクはラベルを名前に持つ `<nav>` の中にあるので、スクリーンリーダーはそれをページのランドマークの一覧に出します。今のページへのリンクは `aria-current=\"page\"` を持ちます。前後のページへのリンクは `rel=\"prev\"` と `rel=\"next\"` を持ち、「前へ」「次へ」を言葉で示します。最初のページでは「前へ」がリンクではなく `aria-disabled=\"true\"` を持つテキストになり、最後のページでは「次へ」が同じようになるので、Tab では飛ばされます。省いたページの代わりの `…` は支援技術から隠します。",
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
                    &["Tab", "Moves focus to the next link."],
                    &["Enter", "Follows the focused link."],
                ],
                ja: &[
                    &["Tab", "次のリンクにフォーカスを移します。"],
                    &["Enter", "フォーカスのあるリンクの先に移ります。"],
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
                en: "Building the links",
                ja: "リンクを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`pagination`](api:hypersig_ui/fn.pagination.html) takes the current page, the number of pages, a function that returns the link to a page, and the words of its parts:",
                ja: "[`pagination`](api:hypersig_ui/fn.pagination.html) は、今のページ、ページ数、ページへのリンクを返す関数、各部分の言葉を受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn pagination<S>(
    current: u32,
    count: u32,
    link: impl Fn(u32) -> Link<S>,
    text: &PaginationText,
) -> El<S>;",
        ),
        para(
            site,
            L {
                en: "Pages are counted from 1. The links are the page before and after, the first and the last page, and the pages next to the current one, with `…` for the pages left out. Where `…` would stand for one page, that page is shown instead.",
                ja: "ページは 1 から数えます。リンクは、前後のページ、最初と最後のページ、今のページの隣のページで、省いたページは `…` で表します。`…` が 1 ページだけを表すことになる場合は、代わりにそのページを表示します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["`current` of 10 pages", "The page links shown"],
                ja: &["10 ページ中の `current`", "表示されるページのリンク"],
            },
            L {
                en: &[
                    &["1", "1, 2, …, 10"],
                    &["4", "1, 2, 3, 4, 5, …, 10"],
                    &["5", "1, …, 4, 5, 6, …, 10"],
                    &["10", "1, …, 9, 10"],
                ],
                ja: &[
                    &["1", "1、2、…、10"],
                    &["4", "1、2、3、4、5、…、10"],
                    &["5", "1、…、4、5、6、…、10"],
                    &["10", "1、…、9、10"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`link` returns a [`Link`](api:hypersig/struct.Link.html), which a route builds with `link_with`, so each page is a URL of the application. A closure that formats the URL itself returns a `String`, which is E0308:",
                ja: "`link` はルートの `link_with` で作る [`Link`](api:hypersig/struct.Link.html) を返すので、どのページもアプリケーションの URL になります。URL を自分で組み立てるクロージャは `String` を返すので、E0308 になります。",
            },
        ),
        show(Snippet::UiPaginationLink),
        para(
            site,
            L {
                en: "The links are plain `<a>` elements, so following one loads the page at that URL from the server, which renders the rows of that page and the pagination for it.",
                ja: "リンクは普通の `<a>` なので、たどるとサーバーからその URL のページが読み込まれ、サーバーがそのページの行と、そのページ用のページネーションを描画します。",
            },
        ),
    ])
}

fn words(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The words",
                ja: "言葉",
            },
        ),
        para(
            site,
            L {
                en: "[`PaginationText`](api:hypersig_ui/struct.PaginationText.html) holds the name of the `<nav>` and the words of the two links. `PaginationText::default()` is English, so a page in another language passes its own:",
                ja: "[`PaginationText`](api:hypersig_ui/struct.PaginationText.html) は、`<nav>` の名前と前後のリンクの言葉を持ちます。`PaginationText::default()` は英語なので、ほかの言語のページは自分の言葉を渡します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub struct PaginationText {
    /// The name of the `<nav>`, \"Pagination\".
    pub label: String,
    /// The link to the page before, \"Previous\".
    pub previous: String,
    /// The link to the page after, \"Next\".
    pub next: String,
}",
        ),
        para(
            site,
            L {
                en: "The fields are plain text and every field must be given, so the only check is that none is forgotten. [Text in other languages](components/component-text/) covers passing the words of every component together.",
                ja: "フィールドはただの文字列で、すべてのフィールドを書く必要があるので、検査されるのは書き忘れがないことだけです。すべてのコンポーネントの言葉をまとめて渡す方法は [ほかの言語の文言](components/component-text/) で説明します。",
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
                en: "Page 5 of 10, with links from `PRODUCTS.link_with(&page)`, renders as:",
                ja: "`PRODUCTS.link_with(&page)` のリンクで 10 ページ中の 5 ページ目を表示すると、次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<nav aria-label="Pagination">
  <ul>
    <li><a href="/products/4" rel="prev"><svg aria-hidden="true" …></svg><span>Previous</span></a></li>
    <li><a href="/products/1">1</a></li>
    <li aria-hidden="true">…</li>
    <li><a href="/products/4">4</a></li>
    <li><a href="/products/5" aria-current="page">5</a></li>
    <li><a href="/products/6">6</a></li>
    <li aria-hidden="true">…</li>
    <li><a href="/products/10">10</a></li>
    <li><a href="/products/6" rel="next"><span>Next</span><svg aria-hidden="true" …></svg></a></li>
  </ul>
</nav>"#,
        ),
        para(
            site,
            L {
                en: "On page 1, the first item is `<span aria-disabled=\"true\">` holding the same arrow and `Previous`, with no `href`.",
                ja: "1 ページ目では、最初の項目が同じ矢印と `Previous` を持つ `<span aria-disabled=\"true\">` になり、`href` はありません。",
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
                en: "`current` and `count` are plain numbers. A `current` of 0, or above `count`, panics when `pagination` is called, so a handler whose page number comes from the URL checks it first and answers 404 for a page past the end, as the example below does:",
                ja: "`current` と `count` はただの数です。`current` が 0 か `count` より大きいと、`pagination` を呼んだ時点でパニックします。そのため、ページ番号を URL から受け取るハンドラは先にそれを確かめ、下の例のように、最後より後ろのページには 404 を返します。",
            },
        ),
        show(Snippet::UiPaginationPast),
        para(
            site,
            L {
                en: "A `count` of 0, as for a search that found nothing, shows one page, and page 1 of it is valid. Nothing checks that `link` returns the URL of page `n` for `n`, so a closure that ignores its argument gives every number the same href.",
                ja: "検索結果が 0 件のときのように `count` が 0 の場合は 1 ページとして表示し、その 1 ページ目は正しい値として扱われます。`link` が `n` に対してページ `n` の URL を返すかどうかは検査されないので、引数を使わないクロージャでは、どの番号も同じ href になります。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a product list",
                ja: "例: 商品の一覧",
            },
        ),
        para(
            site,
            L {
                en: "The handler counts the pages, answers 404 for a page outside them, and renders the products of the page with the links under them:",
                ja: "ハンドラはページ数を数え、範囲外のページには 404 を返し、そのページの商品とその下のリンクを描画します。",
            },
        ),
        show(Snippet::UiPagination),
        para(
            site,
            L {
                en: "A table that pages its rows in the browser or on the server has its own page buttons, which [Filtering and pages](components/table-filtering/) and [Server table](components/server-table/) cover. Next, [Data table](components/data-table/) lists rows under typed columns.",
                ja: "行をブラウザやサーバーでページに分ける表は、専用のページ送りのボタンを持ちます。これは [絞り込みとページ分け](components/table-filtering/) と [サーバーテーブル](components/server-table/) で説明します。次の [データテーブル](components/data-table/) では型の付いた列の下に行を並べます。",
            },
        ),
    ])
}
