//! The component page on `Collapsible`: a button and a region it shows and
//! hides, bound to a `bool` signal.

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
        lead(site, Doc::Components(Component::Collapsible).leads()),
        demo::show(site, Demo::Collapsible),
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
                en: "It follows the WAI-ARIA disclosure pattern. The button is a native `<button>` with `aria-expanded`, so a screen reader announces it as expanded or collapsed, and `aria-controls` names the region it shows. A hidden region is out of the page for assistive technology too.",
                ja: "WAI-ARIA の disclosure パターンに従います。ボタンは `aria-expanded` を持つネイティブの `<button>` なので、スクリーンリーダーは展開されているか折りたたまれているかを読み上げます。`aria-controls` はボタンが表示する領域を指します。隠れている領域は支援技術からも見えません。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            L {
                en: &[&["Enter, Space", "Shows or hides the region."]],
                ja: &[&["Enter、Space", "領域を表示するか隠します。"]],
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The button and the region",
                ja: "ボタンと領域",
            },
        ),
        para(
            site,
            L {
                en: "[`Collapsible::new`](api:hypersig_ui/struct.Collapsible.html#method.new) takes the signal that says whether the region is shown, and returns a handle whose two methods build the button and the region:",
                ja: "[`Collapsible::new`](api:hypersig_ui/struct.Collapsible.html#method.new) は、領域を表示するかどうかを持つシグナルを受け取り、ハンドルを返します。ハンドルの 2 つのメソッドがボタンと領域を作ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Collapsible<S> {
    pub fn new(open: Sig<S, bool>) -> Self;
    pub fn trigger(self, variant: ButtonVariant, size: ButtonSize) -> El<S>;
    pub fn content(self) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "`Collapsible` is `Copy`, so one handle builds both parts. [`trigger`](api:hypersig_ui/struct.Collapsible.html#method.trigger) returns a [button](components/button/) of the given look that flips the signal on a click, and [`content`](api:hypersig_ui/struct.Collapsible.html#method.content) returns an empty `<div>`, shown while the signal is true, for the children you add with `El::child`. The two are separate elements, so the button can sit in a heading or a toolbar away from the region.",
                ja: "`Collapsible` は `Copy` なので、1 つのハンドルから両方の部品を作れます。[`trigger`](api:hypersig_ui/struct.Collapsible.html#method.trigger) は指定した見た目の [ボタン](components/button/) を返し、このボタンはクリックでシグナルを反転します。[`content`](api:hypersig_ui/struct.Collapsible.html#method.content) は、シグナルが true の間だけ表示される空の `<div>` を返し、中身は `El::child` で加えます。2 つは別の要素なので、ボタンを領域から離れた見出しやツールバーに置けます。",
            },
        ),
        para(
            site,
            L {
                en: "The state is a signal of the page, so the server sets it with the page's first value or a patch, and any statement can open the region, such as `Releases::older().set(true)` on another button. The signal must be a `bool`. A signal of another type is E0308:",
                ja: "状態はページのシグナルなので、サーバーはページの最初の値やパッチでそれを設定でき、どの文からも領域を開けます。たとえば別のボタンに `Releases::older().set(true)` を付けます。シグナルは `bool` でなければならず、ほかの型のシグナルは E0308 になります。",
            },
        ),
        show(Snippet::UiCollapsibleType),
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
                en: "With the signal false, the button and the region render as below. The value of `aria-controls` is the region's id, which the component derives from the signal's path:",
                ja: "シグナルが false のとき、ボタンと領域は次のように描画されます。`aria-controls` の値は領域の id で、コンポーネントがシグナルのパスから作ります。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<button type="button" aria-controls="…" aria-expanded="false">Show older</button>
…
<div id="…">
  <ul><li>0.8.1</li><li>0.8.0</li></ul>
</div>"#,
        ),
        para(
            site,
            L {
                en: "The region is in the HTML while it is hidden. A click sets the signal to true, which turns `aria-expanded` to `\"true\"` and shows the region, with no request to the server.",
                ja: "領域は隠れている間も HTML に含まれます。クリックするとシグナルが true になり、`aria-expanded` が `\"true\"` に変わって領域が表示されます。サーバーへのリクエストはありません。",
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
                en: "The region's id comes from the signal, so calling `content` twice on one handle, or building two collapsibles on one signal, puts one id on two elements. `Page::render` panics on such a page:",
                ja: "領域の id はシグナルから作られるので、1 つのハンドルで `content` を 2 回呼んだり、1 つのシグナルで 2 つの折りたたみを作ったりすると、2 つの要素に同じ id が付きます。そのようなページでは `Page::render` がパニックします。",
            },
        ),
        show(Snippet::UiCollapsibleTwice),
        para(
            site,
            L {
                en: "Nothing checks that the page has the region at all. A page with only the button still flips the signal, and its `aria-controls` names an element that is not there. The button's name is its text, which `trigger` leaves to you, so a button with no text or label has no name for a screen reader.",
                ja: "ページに領域があるかどうかは検査されません。ボタンだけのページでもシグナルは反転しますが、`aria-controls` は存在しない要素を指します。ボタンの名前はそのテキストで、`trigger` はテキストを付けないので、テキストもラベルもないボタンは、スクリーンリーダーにとって名前のないボタンになります。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: older releases",
                ja: "例: 以前のリリース",
            },
        ),
        para(
            site,
            L {
                en: "The button sits in the page's heading, and the list it shows comes after a paragraph:",
                ja: "ボタンはページの見出しの中にあり、ボタンが表示する一覧は段落のあとに続きます。",
            },
        ),
        show(Snippet::UiCollapsible),
        para(
            site,
            L {
                en: "Next, [Card](components/card/) groups related content in a bordered box.",
                ja: "次の [カード](components/card/) では関連する内容を枠線付きのボックスにまとめます。",
            },
        ),
    ])
}
