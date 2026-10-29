//! The page on the document a handler sends, and what its head tells
//! browsers, search engines, and link previews.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::ThePage).title(site)),
        lead(site, Doc::Learn(Learn::ThePage).leads()),
        document(site),
        serving(site),
        title_section(site),
        search(site),
        other_head(site),
        example(site),
    ])
}

fn document(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The page",
                ja: "ページ",
            },
        ),
        para(
            site,
            L {
                en: "[`Page<S>`](api:hypersig/struct.Page.html) wraps the tree in a full document. `Page::new(title: impl Into<String>, signals: S)` takes the title and the initial values of the signals, `child` appends to `<body>`, and `render(&self) -> String` returns the HTML a handler sends. `Page::lang` sets the `lang` of `<html>` from an [`IntlLocale`](api:hypersig/enum.IntlLocale.html) or a [`LanguageTag`](api:hypersig/struct.LanguageTag.html), which the browser reads for hyphenation, the voice of a screen reader, and the fonts it picks. A page without it has no `lang`.",
                ja: "[`Page<S>`](api:hypersig/struct.Page.html) は木を包んで完全な HTML 文書にします。`Page::new(title: impl Into<String>, signals: S)` はタイトルとシグナルの初期値を受け取り、`child` は `<body>` に要素を追加し、`render(&self) -> String` はハンドラが送る HTML を返します。`Page::lang` は [`IntlLocale`](api:hypersig/enum.IntlLocale.html) か [`LanguageTag`](api:hypersig/struct.LanguageTag.html) から `<html>` の `lang` を設定します。ブラウザはこれをハイフネーション、スクリーンリーダーの声、フォントの選択に使います。設定しないページには `lang` がありません。",
            },
        ),
        show(Snippet::Document),
        para(
            site,
            L {
                en: "The document holds the character set, a viewport for phones, an empty icon so that the browser does not request `/favicon.ico`, the title, and the script that loads the runtime, which reads the initial signal values from the page. Every binding is rendered with those values already applied, so text, visibility, classes, attributes, form values, list items, and conditional elements show before the runtime loads, and the runtime keeps the elements instead of building them again.",
                ja: "文書には文字コード、スマートフォン向けの viewport、ブラウザが `/favicon.ico` を要求しないようにするための空のアイコン、タイトル、ランタイムを読み込むスクリプトが入ります。ランタイムはシグナルの初期値をページから読みます。束縛はすべてその値を当てはめた状態で描画されるので、テキスト、表示と非表示、クラス、属性、フォームの値、リストの項目、条件付きの要素は、ランタイムが読み込まれる前から表示されます。ランタイムは要素を作り直さず、そのまま引き継ぎます。",
            },
        ),
        para(
            site,
            L {
                en: "`Page::render` panics on a mistake the types cannot hold, such as two elements with one id from `Id::new`, a signal struct whose `Serialize` impl fails, or an integer outside -2^53 to 2^53, which the browser would round. In a debug build it also panics when `Page::lang` names the language of an `IntlLocale` and the page formats its plurals and dates only in others. The [rustdoc of `render`](api:hypersig/struct.Page.html#method.render) lists every case. A test that renders each page finds them before a visitor does, as [Testing a page without a browser](learn/testing-pages/) shows.",
                ja: "`Page::render` は、型では防げない間違いでパニックします。たとえば、`Id::new` から作った同じ id を持つ 2 つの要素、`Serialize` の実装が失敗するシグナル構造体、ブラウザが丸めてしまう -2^53 から 2^53 の範囲外の整数です。デバッグビルドでは、`Page::lang` が `IntlLocale` の言語を指しているのに、ページが複数形や日付をほかの言語でしか書式化していない場合にもパニックします。すべての場合は [`render` の rustdoc](api:hypersig/struct.Page.html#method.render) にあります。各ページを描画するテストを書けば、訪問者より先に見つけられます。[ブラウザなしでページをテストする](learn/testing-pages/) で説明します。",
            },
        ),
    ])
}

fn serving(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Serving the page and the runtime",
                ja: "ページとランタイムを配信する",
            },
        ),
        para(
            site,
            L {
                en: "A page handler returns the `Page`, and the function `routes!` writes registers it on the router. The document loads the runtime from [`RUNTIME_PATH`](api:hypersig/constant.RUNTIME_PATH.html), and [`RouterExt::hypersig_runtime`](api:hypersig/trait.RouterExt.html#tymethod.hypersig_runtime) serves the file at that path.",
                ja: "ページのハンドラは `Page` を返し、`routes!` が生成する関数がそれをルーターに登録します。文書は [`RUNTIME_PATH`](api:hypersig/constant.RUNTIME_PATH.html) からランタイムを読み込み、[`RouterExt::hypersig_runtime`](api:hypersig/trait.RouterExt.html#tymethod.hypersig_runtime) がそのパスでファイルを配信します。",
            },
        ),
        show(Snippet::Serve),
        para(
            site,
            L {
                en: "Nothing checks that the router calls `hypersig_runtime`. Without it, the request for the runtime gets a 404 that the browser's console shows, and the page stays as its first HTML drew it: nothing reacts to a click or a key. A page served under a path prefix loads the runtime from under that prefix, as [Serving under a path prefix](learn/base-path/) shows. In a debug build, an open tab reloads on its own when the [development server](learn/dev-server/) restarts the server.",
                ja: "ルーターが `hypersig_runtime` を呼んでいるかどうかは検査しません。呼んでいないと、ランタイムのリクエストは 404 になってブラウザのコンソールに表示され、ページは最初の HTML のまま変わりません。クリックにもキーにも反応しません。パスの接頭辞の下で配信するページは、その接頭辞の下からランタイムを読み込みます。これは [パスの接頭辞の下で配信する](learn/base-path/) で説明します。デバッグビルドでは、[開発サーバー](learn/dev-server/) がサーバーを起動し直すと、開いているタブが自動で読み込み直されます。",
            },
        ),
    ])
}

fn title_section(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A title that follows the signals",
                ja: "シグナルに追従するタイトル",
            },
        ),
        para(
            site,
            L {
                en: "`Page::bind_title(expr)` makes the tab's title follow a `String` expression, such as an unread count. The server writes its first value into `<title>`, and the runtime writes each later value into `document.title`. A search engine reads the HTML, so it shows the first value, and the card of `OpenGraph` keeps the title given to `Page::new`.",
                ja: "`Page::bind_title(expr)` は未読の件数のように変わる `String` の式にタブのタイトルを合わせます。最初の値はサーバーが `<title>` に書き、その後の値はランタイムが `document.title` に書きます。検索エンジンは HTML を読むので最初の値を表示し、`OpenGraph` のカードには `Page::new` に渡したタイトルが保持されます。",
            },
        ),
        show(Snippet::Title),
        para(
            site,
            L {
                en: "The expression must be a `String`. A number, such as the count itself, is a compile error until `to_text` turns it into text:",
                ja: "式は `String` でなければなりません。件数そのもののような数値は、`to_text` でテキストにするまでコンパイルエラーになります。",
            },
        ),
        show(Snippet::TitleType),
    ])
}

fn search(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Search results and link previews",
                ja: "検索結果とリンクのプレビュー",
            },
        ),
        para(
            site,
            L {
                en: "`Page::description` writes the summary a search result shows, and `Page::icon` the icon of the tab from an [`Asset`](api:hypersig/struct.Asset.html), a URL written in the program. A canonical URL and the image of a link preview must be absolute, so `Page::canonical` and `OpenGraph::image` join a path to an [`Origin`](api:hypersig/struct.Origin.html). [`OpenGraph`](api:hypersig/struct.OpenGraph.html) takes the card's title, text, and URL from the page, so each is written once.",
                ja: "`Page::description` は検索結果に出る要約を書き、`Page::icon` はタブのアイコンを [`Asset`](api:hypersig/struct.Asset.html)（プログラムに書いた URL）から書きます。canonical の URL とリンクのプレビューの画像は絶対 URL でなければならないので、`Page::canonical` と `OpenGraph::image` はパスを [`Origin`](api:hypersig/struct.Origin.html) につなぎます。[`OpenGraph`](api:hypersig/struct.OpenGraph.html) はカードのタイトル、本文、URL をページから取るので、どれも 1 回書くだけで済みます。",
            },
        ),
        show(Snippet::Head),
        para(
            site,
            L {
                en: "The `Origin` constant fails to compile when it holds more than the scheme, the host, and the port. An application that a proxy serves under a path such as `/blog` adds that path with `Origin::under`, which rejects a malformed prefix the same way:",
                ja: "`Origin` の定数はスキーム、ホスト、ポート以外を含むとコンパイルエラーになります。プロキシが `/blog` のようなパスの下に配信するアプリは、`Origin::under` でそのパスを追加します。`under` も、形の正しくない接頭辞を同じようにコンパイルエラーにします。",
            },
        ),
        show(Snippet::OriginPath),
        para(
            site,
            L {
                en: "`OpenGraph::image` panics when the path does not start with `/`. Nothing checks that the router serves the image or the icon, and the browser's console shows the 404 of a missing icon. `OpenGraph::kind` sets `og:type`, which is `website` without it, and `OpenGraph::locale` sets `og:locale` from an [`OpenGraphLocale`](api:hypersig/struct.OpenGraphLocale.html) such as `ja_JP`.",
                ja: "`OpenGraph::image` は、パスが `/` で始まらないとパニックします。ルーターが画像やアイコンを配信しているかどうかは検査しません。アイコンが見つからなければ、その 404 がブラウザのコンソールに表示されます。`OpenGraph::kind` は `og:type` を設定し、設定しなければ `website` になります。`OpenGraph::locale` は `ja_JP` のような [`OpenGraphLocale`](api:hypersig/struct.OpenGraphLocale.html) から `og:locale` を設定します。",
            },
        ),
        para(
            site,
            L {
                en: "`Page::json_ld` adds structured data that search engines read, written so that no string inside it can close the block. `noindex` and `nofollow` ask search engines to leave the page out of their results and not to follow its links. `Page::alternate` names the same page in another language, which [Alternate pages](i18n/alternates/) covers.",
                ja: "`Page::json_ld` は検索エンジンが読む構造化データを追加します。中の文字列がブロックを閉じることはありません。`noindex` と `nofollow` は検索結果にページを載せないことと、ページのリンクをたどらないことを検索エンジンに求めます。`Page::alternate` は別の言語の同じページを示します。これは [別の言語のページを示す](i18n/alternates/) で説明します。",
            },
        ),
        show(Snippet::Search),
    ])
}

fn other_head(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What else a page sets",
                ja: "ページが設定するほかのもの",
            },
        ),
        para(
            site,
            L {
                en: "The other methods of `Page` belong to the topics below.",
                ja: "`Page` のほかのメソッドは次のページで扱います。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Methods", "What they set", "Page"],
                ja: &["メソッド", "設定するもの", "ページ"],
            },
            L {
                en: &[
                    &[
                        "`theme`, `global`, `style`, `stylesheet`, `css_when`, `css_match`",
                        "Token values, rules by tag, stylesheets, and styles on `<body>`",
                        "[Styling](learn/styling/)",
                    ],
                    &[
                        "`computed`",
                        "A signal kept equal to an expression",
                        "[Computed signals](learn/computed-signals/)",
                    ],
                    &[
                        "`persist`",
                        "A signal kept in `localStorage`",
                        "[Signals kept across page loads](learn/persisted-signals/)",
                    ],
                    &[
                        "`on_key`",
                        "A shortcut anywhere on the page",
                        "[Events](learn/events/)",
                    ],
                    &[
                        "`script`",
                        "A `<script type=\"module\">` the page's elements need",
                        "[JavaScript libraries](learn/javascript/)",
                    ],
                    &[
                        "`csp_nonce`, `framing`",
                        "A Content-Security-Policy nonce, and which pages may frame this one",
                        "[Security](learn/security/)",
                    ],
                    &[
                        "`runtime_src`, `render_under`",
                        "Where the runtime loads from, and the path prefix of a static file",
                        "[Static pages](learn/static-pages/)",
                    ],
                ],
                ja: &[
                    &[
                        "`theme`、`global`、`style`、`stylesheet`、`css_when`、`css_match`",
                        "トークンの値、タグごとのルール、スタイルシート、`<body>` のスタイル",
                        "[スタイル](learn/styling/)",
                    ],
                    &[
                        "`computed`",
                        "式の値に合わせ続けるシグナル",
                        "[計算シグナル](learn/computed-signals/)",
                    ],
                    &[
                        "`persist`",
                        "`localStorage` に保つシグナル",
                        "[ページをまたいで保つシグナル](learn/persisted-signals/)",
                    ],
                    &[
                        "`on_key`",
                        "ページのどこでも働くショートカット",
                        "[イベント](learn/events/)",
                    ],
                    &[
                        "`script`",
                        "ページの要素が必要とする `<script type=\"module\">`",
                        "[JavaScript のライブラリ](learn/javascript/)",
                    ],
                    &[
                        "`csp_nonce`、`framing`",
                        "Content-Security-Policy の nonce と、このページをフレームに表示できるページ",
                        "[セキュリティ](learn/security/)",
                    ],
                    &[
                        "`runtime_src`、`render_under`",
                        "ランタイムの読み込み元と、静的ファイルのパスの接頭辞",
                        "[静的ページ](learn/static-pages/)",
                    ],
                ],
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A blog post",
                ja: "ブログの記事",
            },
        ),
        para(
            site,
            L {
                en: "The handler below answers each post with its language, summary, canonical URL, icon, and a card whose image is named after the post. A post the store does not hold answers `None`, which the router sends as a 404.",
                ja: "次のハンドラは記事ごとに言語、要約、canonical の URL、アイコン、記事にちなんだ名前の画像を持つカードを付けて応答します。保存されていない記事には `None` を返し、ルーターはそれを 404 として送ります。",
            },
        ),
        show(Snippet::PostPage),
        para(
            site,
            L {
                en: "Next, [Values that follow the signals](learn/bindings/) makes the page's text and attributes change in the browser. [Routes](learn/routes/) covers the route `POST` and its parameter.",
                ja: "次の [シグナルに追従する値](learn/bindings/) では、ページのテキストと属性をブラウザで変わるようにします。ルート `POST` とそのパラメータは [ルート](learn/routes/) で説明します。",
            },
        ),
    ])
}
