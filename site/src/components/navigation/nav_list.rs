//! The component page on `NavList`: links to the sections of an
//! application, marked against the page's own `Url`.

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
        lead(site, Doc::Components(Component::Navigation).leads()),
        demo::show(site, Demo::Navigation),
        accessibility(site),
        building(site),
        current(site),
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
                en: "The list is a `<nav>` named by its label, so a screen reader lists it among the page's landmarks under that name. It holds a `<ul>` of `<a>` links, which the browser focuses with Tab and follows with Enter. The link to the page the visitor is on carries `aria-current=\"page\"`, which a screen reader announces as the current page, and the link to the section around it carries `aria-current=\"true\"`. Both links are drawn on the accent background, and the attribute gives a screen reader the same mark.",
                ja: "一覧はラベルを名前に持つ `<nav>` なので、スクリーンリーダーはその名前でページのランドマークの一覧に出します。中には `<a>` のリンクを並べた `<ul>` があり、ブラウザは Tab でリンクにフォーカスを移し、Enter でリンク先に移ります。訪問者がいるページへのリンクは `aria-current=\"page\"` を持ち、スクリーンリーダーはそれを現在のページとして読み上げます。そのページを含むセクションへのリンクは `aria-current=\"true\"` を持ちます。どちらのリンクも accent の背景で描かれ、スクリーンリーダーには同じ印が属性で伝わります。",
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
                en: "Building the list",
                ja: "一覧を作る",
            },
        ),
        para(
            site,
            L {
                en: "[`NavList::new`](api:hypersig_ui/struct.NavList.html#method.new) takes the name of the `<nav>` and the URL of the page it is on, and [`item`](api:hypersig_ui/struct.NavList.html#method.item) adds a link to a route:",
                ja: "[`NavList::new`](api:hypersig_ui/struct.NavList.html#method.new) は `<nav>` の名前と、一覧を置くページの URL を受け取ります。[`item`](api:hypersig_ui/struct.NavList.html#method.item) はルートへのリンクを追加します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> NavList<S> {
    pub fn new(label: impl Into<String>, current: Url) -> Self;
    pub fn item<P: Params, Q: Search>(
        self,
        text: impl Into<String>,
        route: Route<P, Q>,
        args: Q::Args<P>,
    ) -> Self
    where
        Q::Args<P>: Serialize;
    pub fn render(self) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "`item` takes the route and its arguments as [`Route::url`](api:hypersig/struct.Route.html#method.url) takes them: `()` for a route without parameters, the value of its one parameter, or a tuple. The links go in the order they are added. An argument of the wrong type for the route is E0308:",
                ja: "`item` は、ルートとその引数を [`Route::url`](api:hypersig/struct.Route.html#method.url) と同じ形で受け取ります。パラメータのないルートには `()`、パラメータが 1 つならその値、複数ならタプルを渡します。リンクは追加した順に並びます。ルートに合わない型の引数は E0308 になります。",
            },
        ),
        show(Snippet::UiNavListArgs),
        para(
            site,
            L {
                en: "`NavList` converts into an `El<S>` and a `Node<S>`, so `El::child` and `Page::child` take it without `render`.",
                ja: "`NavList` は `El<S>` と `Node<S>` に変換できるので、`render` を呼ばずに `El::child` や `Page::child` に渡せます。",
            },
        ),
    ])
}

fn current(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Marking the current page",
                ja: "現在のページに印を付ける",
            },
        ),
        para(
            site,
            L {
                en: "The list decides the mark on the server, when it renders, by comparing the path of each link with the path of `current`. The query string is left out of both. `current` is a [`Url`](api:hypersig/struct.Url.html), which only `Route::url` builds, so the page passes its own route and arguments rather than a string, and a string is E0308:",
                ja: "一覧は描画するときにサーバーで、各リンクのパスと `current` のパスを比べて印を決めます。どちらもクエリ文字列は比べません。`current` は `Route::url` だけが作る [`Url`](api:hypersig/struct.Url.html) なので、ページは文字列ではなく自身のルートと引数を渡します。文字列を渡すと E0308 になります。",
            },
        ),
        show(Snippet::UiNavListCurrent),
        table_of(
            site,
            L {
                en: &["Link's path, on the page `/users/7`", "Mark"],
                ja: &["リンクのパス（ページ `/users/7` の場合）", "印"],
            },
            L {
                en: &[
                    &["`/users/7`, the same path", "`aria-current=\"page\"`"],
                    &[
                        "`/users`, a path that the page's path continues after a `/`",
                        "`aria-current=\"true\"`",
                    ],
                    &["`/`, or any other path", "none"],
                ],
                ja: &[
                    &["`/users/7`（同じパス）", "`aria-current=\"page\"`"],
                    &[
                        "`/users`（ページのパスが `/` を挟んでその続きになっているパス）",
                        "`aria-current=\"true\"`",
                    ],
                    &["`/` やそのほかのパス", "なし"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The link to `/` is never marked as a section, since every path continues it, so a Home link is marked only on the home page. A section stays marked on every page under it, such as Users on each user's page.",
                ja: "どのパスも `/` の続きになるので、`/` へのリンクはセクションとしての印を受けません。Home へのリンクに印が付くのはホームページだけです。セクションはその下のどのページでも印が付いたままになり、たとえば各ユーザーのページで Users に印が付きます。",
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
                en: "On the page `/users/7`, with a third link to that page added, the list renders as:",
                ja: "ページ `/users/7` で、そのページへの 3 つ目のリンクを加えた一覧は、次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<nav aria-label="Main">
  <ul>
    <li><a href="/">Dashboard</a></li>
    <li><a href="/users" aria-current="true">Users</a></li>
    <li><a href="/users/7" aria-current="page">User 7</a></li>
  </ul>
</nav>"#,
        ),
        para(
            site,
            L {
                en: "The marks are fixed in this HTML. The list does not move them when the browser goes to another page; the next page renders its own list with its own `current`.",
                ja: "印はこの HTML の中で決まっています。ブラウザが別のページに移っても、一覧が印を動かすことはなく、次のページは自身の `current` で自分の一覧を描画します。",
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
                en: "The compiler sees a `Url`, not which page it belongs to. A page that passes the URL of another route marks that route's link instead of its own, and one that passes a URL matching no link marks nothing. Both render without an error. Building the list in one function that takes the URL, as the example below does, keeps each page to passing `ROUTE.url(args)` with its own arguments.",
                ja: "コンパイラにわかるのは `Url` であることだけで、それがどのページの URL かはわかりません。別のルートの URL を渡したページでは、自分のリンクではなくそのルートのリンクに印が付き、どのリンクにも合わない URL を渡したページでは、どこにも印が付きません。どちらもエラーなく描画されます。下の例のように、URL を受け取る 1 つの関数で一覧を作れば、各ページは自分の引数で `ROUTE.url(args)` を渡すだけになります。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a sidebar on every page",
                ja: "例: 全ページのサイドバー",
            },
        ),
        para(
            site,
            L {
                en: "Each handler passes its own URL to one `sidebar` function. On `/users/7`, Users carries `aria-current=\"true\"`:",
                ja: "各ハンドラは自分の URL を 1 つの `sidebar` 関数に渡します。`/users/7` では、Users が `aria-current=\"true\"` を持ちます。",
            },
        ),
        show(Snippet::UiNavList),
        para(
            site,
            L {
                en: "Next, [Breadcrumb](components/breadcrumb/) shows the pages above the current one.",
                ja: "次の [パンくずリスト](components/breadcrumb/) では今のページより上のページを並べます。",
            },
        ),
    ])
}
