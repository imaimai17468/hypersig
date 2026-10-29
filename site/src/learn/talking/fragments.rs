//! The page on a part of the page the server renders again and sends as HTML.

use super::needs_a_server;
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::learn::{Program, program};
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Fragments).title(site)),
        lead(site, Doc::Learn(Learn::Fragments).leads()),
        declaring(site),
        sending(site),
        merging(site),
        ids(site),
        unchecked(site),
        example(site),
    ])
}

/// The `Fragment` trait and `fragment`.
fn declaring(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Declaring a fragment",
                ja: "フラグメントを宣言する",
            },
        ),
        para(
            site,
            L {
                en: "A [`Fragment`](api:hypersig/trait.Fragment.html) is a part of the page the server can render again on its own. Its trait has three items: `type Signals: Signals`, the signal struct of its page; `const ID: Id`, the element id of its root; and `fn render(&self) -> El<Self::Signals>`. The fragment owns its element id, so the id in the first page and the id in a later patch cannot differ. [`fragment`](api:hypersig/fn.fragment.html), `fn fragment<F: Fragment>(fragment: &F) -> El<F::Signals>`, renders it in the page with its root's `id` set to `ID`, replacing any `id` `render` gave it.",
                ja: "[`Fragment`](api:hypersig/trait.Fragment.html) は、サーバーが単独で描画し直せるページの一部です。トレイトには 3 つの項目があります。ページのシグナル構造体を表す `type Signals: Signals`、ルート要素の id を表す `const ID: Id`、`fn render(&self) -> El<Self::Signals>` です。フラグメントが要素の id を持つので、最初のページの id とあとの差分の id が食い違うことはありません。[`fragment`](api:hypersig/fn.fragment.html)（`fn fragment<F: Fragment>(fragment: &F) -> El<F::Signals>`）は、ルートの `id` を `ID` にしてページの中に描画します。`render` が付けた `id` があれば置き換えます。",
            },
        ),
        show(Snippet::TalkFragment),
    ])
}

/// `Patches::element`.
fn sending(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Sending it again",
                ja: "もう一度送る",
            },
        ),
        para(
            site,
            L {
                en: "[`Patches::element`](api:hypersig/struct.Patches.html#method.element), `fn element<F: Fragment<Signals = S>>(self, fragment: &F) -> Self`, renders the fragment again and sends its HTML, with the CSS rules its elements use when they use `css`. The text and attributes the fragment's bindings show follow the signals in the browser once the patch arrives, as they do in the first page. A fragment of another page's signals is a compile error, E0271.",
                ja: "[`Patches::element`](api:hypersig/struct.Patches.html#method.element)（`fn element<F: Fragment<Signals = S>>(self, fragment: &F) -> Self`）は、フラグメントを描画し直して HTML を送ります。フラグメントの要素が `css` を使っていれば、その CSS のルールも一緒に送ります。フラグメントの束縛が表示する文字や属性は、最初のページと同じく、差分が届いたあとはブラウザでシグナルに追従します。別のページのシグナルのフラグメントを渡すと、コンパイルエラー E0271 になります。",
            },
        ),
        show(Snippet::TalkWrongFragment),
    ])
}

/// How the browser merges the new HTML into the element.
fn merging(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What changes in the page",
                ja: "ページで変わるもの",
            },
        ),
        para(
            site,
            L {
                en: "The browser compares the HTML it receives with the element that has the fragment's id and rewrites only the parts that differ. An element without an id is matched with the next one of the same tag and `name`, so a message inserted before an input leaves the input and its text in place. Where a patch does replace the element that has focus, focus and the text selection move to the new element with the same id or `name`. The `open` attribute of a `<details>` and a `<dialog>` is left alone, so a section the visitor opened or a dialog a script showed stays as it was. A list or a condition inside the fragment whose template is unchanged keeps the items and the content the browser drew, with their focus and any text being typed in them.",
                ja: "ブラウザは受け取った HTML とフラグメントの id を持つ要素を突き合わせ、違う部分だけを書き換えます。id のない要素は、タグと `name` が同じ次の要素と対応づけるので、入力欄の前にメッセージが加わっても、入力欄も中の文字もそのまま残ります。フォーカスのある要素を差分が置き換えたときは、同じ id か `name` を持つ新しい要素にフォーカスと選択範囲を移します。`<details>` と `<dialog>` の `open` 属性には触れないので、訪問者が開いた項目やスクリプトが表示したダイアログはそのまま残ります。フラグメントの中のリストや条件は、テンプレートが変わっていなければ、ブラウザが描いた項目と内容を、フォーカスや入力中の文字も含めてそのまま残します。",
            },
        ),
    ])
}

/// The ids and names the server cannot compare with the rest of the page.
fn ids(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Ids the server cannot compare",
                ja: "サーバーが比べられない id",
            },
        ),
        para(
            site,
            L {
                en: "The server renders a fragment sent again on its own, so it cannot compare its ids with the rest of the page. The browser compares them once the patch is applied and writes a warning to the console when the patch adds an id that another element already carries. The same holds for the name of a `NodeRef`, which [Node refs](learn/node-refs/) covers.",
                ja: "サーバーは送り直すフラグメントを単独で描画するので、その id をページの残りの部分と比べられません。そこでブラウザが差分を適用したあとに比べ、ほかの要素がすでに持つ id を差分が追加したときは、コンソールに警告を書きます。[ノード参照](learn/node-refs/) で説明する `NodeRef` の名前も同じです。",
            },
        ),
    ])
}

/// What the types leave unchecked, and the cost of sending HTML.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型で検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "A fragment of a [component](learn/components/) that is a `Part` is rendered with its paths under the field of the part being answered. A batch built outside the handler or stream answering that part, such as in a task that broadcasts one batch to every page, cannot be sent to it: an action answers `500 Internal Server Error` with the reason, and a stream ends. `to_event_stream` panics in that case.",
                ja: "`Part` である [コンポーネント](learn/components/) のフラグメントは、応答している部品のフィールドの下にパスを置いて描画されます。すべてのページに 1 つのまとまりを配るタスクの中など、その部品に応答するハンドラやストリームの外で作ったまとまりは、その部品には送れません。アクションは理由を書いた `500 Internal Server Error` を返し、ストリームは終わります。このとき `to_event_stream` はパニックします。",
            },
        ),
        para(
            site,
            L {
                en: "The server sends the fragment's whole HTML each time, so a list that changes one item at a time costs less as a list signal drawn with `each_keyed`, which a [list patch](learn/list-patches/) changes one item at a time, as `examples/todo` does.",
                ja: "サーバーはフラグメントの HTML を毎回まるごと送ります。項目が 1 件ずつ変わるリストは、`each_keyed` で描くリストのシグナルにして [リストの差分](learn/list-patches/) で 1 件ずつ変えるほうが送る量が少なくなります。`examples/todo` はこの形です。",
            },
        ),
    ])
}

/// The complete program, and the pages that follow.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A complete example",
                ja: "完全な例",
            },
        ),
        para(
            site,
            L {
                en: "This search sends the query 200 ms after the visitor stops typing, and the handler answers with the result list rendered again as a fragment. The search field sits outside the fragment, so its text and focus stay as they were.",
                ja: "この検索は訪問者が入力を止めて 200 ms 後にクエリを送り、ハンドラは結果のリストをフラグメントとして描画し直して返します。検索欄はフラグメントの外にあるので、文字もフォーカスもそのまま残ります。",
            },
        ),
        program(Program::Fragments),
        needs_a_server(site),
        para(
            site,
            L {
                en: "Next, [Streams](learn/streams/) sends patches and fragments as changes happen on the server.",
                ja: "次はサーバーで変化が起きた時点で差分やフラグメントを送る [ストリーム](learn/streams/) です。",
            },
        ),
    ])
}
