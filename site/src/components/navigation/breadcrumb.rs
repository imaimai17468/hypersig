//! The component page on `Breadcrumb`: a trail of links that only
//! `Breadcrumb::current` can finish.

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
        lead(site, Doc::Components(Component::Breadcrumb).leads()),
        demo::show(site, Demo::Breadcrumb),
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
                en: "The trail is a `<nav>` named by its label, holding an ordered list, so a screen reader lists it among the page's landmarks and reads how many items the trail has. The current page ends the list as text with `aria-current=\"page\"`, which a screen reader announces as the current page. The chevrons between the items are hidden from assistive technology. The links take Tab and Enter as any link does.",
                ja: "パンくずリストはラベルを名前に持つ `<nav>` で、中に順序付きリストを持ちます。そのため、スクリーンリーダーはページのランドマークの一覧にそれを出し、項目がいくつあるかを読み上げます。今のページは `aria-current=\"page\"` を持つテキストとしてリストの最後に置かれ、スクリーンリーダーはそれを現在のページとして読み上げます。項目の間の山形の印は支援技術から隠します。リンクはほかのリンクと同じく Tab と Enter で操作します。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building a trail",
                ja: "パンくずリストを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`Breadcrumb::new`](api:hypersig_ui/struct.Breadcrumb.html#method.new) starts an empty trail, [`link`](api:hypersig_ui/struct.Breadcrumb.html#method.link) adds a page above the current one, and [`current`](api:hypersig_ui/struct.Breadcrumb.html#method.current) ends the trail at the page the visitor is on:",
                ja: "[`Breadcrumb::new`](api:hypersig_ui/struct.Breadcrumb.html#method.new) は空のパンくずリストを作り、[`link`](api:hypersig_ui/struct.Breadcrumb.html#method.link) は今のページより上のページを追加し、[`current`](api:hypersig_ui/struct.Breadcrumb.html#method.current) は訪問者がいるページでリストを終えます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Breadcrumb<S> {
    pub fn new(label: impl Into<String>) -> Self;
    pub fn link(self, text: impl Into<String>, link: Link<S>) -> Self;
    pub fn current(self, text: impl Into<String>) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "`link` takes a [`Link`](api:hypersig/struct.Link.html), which a route builds with `link` or `link_with`, so each href is a route of the application. `current` is the only way to get the element out of a `Breadcrumb`, and it returns an `El<S>`, which has no `link`. A trail therefore always ends at the page it is on, and nothing can follow it:",
                ja: "`link` はルートの `link` か `link_with` で作る [`Link`](api:hypersig/struct.Link.html) を受け取るので、どの href もアプリケーションのルートになります。`Breadcrumb` から要素を取り出す方法は `current` だけで、`current` が返す `El<S>` には `link` がありません。そのため、パンくずリストは必ず今のページで終わり、そのあとには何も続けられません。",
            },
        ),
        show(Snippet::UiBreadcrumbAfter),
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
                en: "A trail of two links ending at the issue page renders as:",
                ja: "2 つのリンクのあとに課題のページで終わるパンくずリストは、次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<nav aria-label="Breadcrumb">
  <ol>
    <li><a href="/">Home</a></li>
    <li role="presentation" aria-hidden="true"><svg aria-hidden="true" …></svg></li>
    <li><a href="/projects/3">Project 3</a></li>
    <li role="presentation" aria-hidden="true"><svg aria-hidden="true" …></svg></li>
    <li><span aria-current="page">Issue 41</span></li>
  </ol>
</nav>"#,
        ),
        para(
            site,
            L {
                en: "The chevrons are list items with `role=\"presentation\"`, so a screen reader counts only the three pages.",
                ja: "山形の印は `role=\"presentation\"` を持つリストの項目なので、スクリーンリーダーが数えるのは 3 つのページだけです。",
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
                en: "The links appear in the order they are added, and nothing checks that each page is above the next or that the route of a link holds the current page. A trail added out of order renders out of order. The label and the texts are plain strings.",
                ja: "リンクは追加した順に並び、各ページが次のページより上にあるか、リンクのルートが今のページを含むかは検査されません。順番を間違えて追加したパンくずリストは、その順番のまま描画されます。ラベルと各テキストはただの文字列です。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: an issue page",
                ja: "例: 課題のページ",
            },
        ),
        para(
            site,
            L {
                en: "The handler builds the trail from the same arguments that name the page:",
                ja: "ハンドラはページを指すのと同じ引数からパンくずリストを作ります。",
            },
        ),
        show(Snippet::UiBreadcrumb),
        para(
            site,
            L {
                en: "Next, [Pagination](components/pagination/) links the pages of a long list.",
                ja: "次の [ページネーション](components/pagination/) では長い一覧のページをリンクで結びます。",
            },
        ),
    ])
}
