//! The Reference section, one page per group of items.
//!
//! Each row links to the item in the rustdoc output that the site serves
//! under `api/`, and its summary is the first sentence of the item's doc
//! comment, shortened where it runs past one line of the table. The binary checks every link against that output when it is
//! given `--rustdoc`, so a renamed or removed item fails the build.

use crate::Site;
use crate::doc::{Doc, Reference};
use crate::lang::{L, Text};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

/// One item: its name, its rustdoc page and anchor, and its summary.
type Row = (&'static str, &'static str, L<&'static str>);

/// A section of a reference page: a heading, an optional paragraph, and a
/// table of items.
struct Section {
    heading: L<&'static str>,
    text: L<&'static str>,
    rows: &'static [Row],
}

/// The article of `page`.
pub fn content(site: Site, page: Reference) -> Part {
    let doc = Doc::Reference(page);
    if page == Reference::Overview {
        return overview(site);
    }
    let (summary, sections) = page_items(page);
    let mut children = vec![title(doc.title(site)), lead(site, summary)];
    for section in sections {
        children.push(heading(site, section.heading));
        if !section.text.en.is_empty() {
            children.push(para(site, section.text));
        }
        let cells: Vec<[String; 2]> = section
            .rows
            .iter()
            .map(|(name, target, summary)| {
                [
                    format!("[`{name}`](api:hypersig/{target})"),
                    site.t(*summary).to_owned(),
                ]
            })
            .collect();
        let rows: Vec<[&str; 2]> = cells
            .iter()
            .map(|[name, summary]| [name.as_str(), summary.as_str()])
            .collect();
        let rows: Vec<&[&str]> = rows.iter().map(<[&str; 2]>::as_slice).collect();
        children.push(table_of(
            site,
            L {
                en: &["Item", "Summary"],
                ja: &["項目", "概要"],
            },
            Text::Chosen(rows.as_slice()),
        ));
    }
    parts(children)
}

fn overview(site: Site) -> Part {
    let rows: Vec<[String; 2]> = crate::doc::REFERENCE
        .iter()
        .flat_map(|group| group.docs.iter())
        .filter_map(|&doc| match doc {
            Doc::Reference(page) if page != Reference::Overview => Some([
                format!("[{}]({})", doc.title(site), doc.path()),
                site.t(page_items(page).0).to_owned(),
            ]),
            _ => None,
        })
        .collect();
    let rows: Vec<[&str; 2]> = rows
        .iter()
        .map(|[page, summary]| [page.as_str(), summary.as_str()])
        .collect();
    let rows: Vec<&[&str]> = rows.iter().map(<[&str; 2]>::as_slice).collect();
    parts([
        title(Doc::Reference(Reference::Overview).title(site)),
        lead(site, Doc::Reference(Reference::Overview).leads()),
        para(
            site,
            L {
                en: "The [complete API documentation](api:hypersig/index.html) is the rustdoc output of the same commit as this site, with every method, its signature, and its examples. The pages here list the items used most often, each with the first sentence of its doc comment, shortened where it runs long.",
                ja: "[API ドキュメントの全体](api:hypersig/index.html)は、このサイトと同じコミットから rustdoc で生成したもので、すべてのメソッドのシグネチャと例を載せています。ここに並べたのはよく使う項目です。各項目にはドキュメントコメントの最初の 1 文を訳して添え、長い文は短くしています。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Page", "What it covers"],
                ja: &["ページ", "扱う内容"],
            },
            Text::Chosen(rows.as_slice()),
        ),
        para(
            site,
            L {
                en: "Styles are in `hypersig::style`, a module that re-exports the `hypersig_style` crate. The [Styling](learn/styling/) page under Learn introduces it, the [Style](style/) documentation covers it in full, and the [`style` module](api:hypersig/style/index.html) lists its items.",
                ja: "スタイルは `hypersig::style` にあります。これは `hypersig_style` クレートを再エクスポートしたモジュールです。「学ぶ」の [スタイル](learn/styling/) のページで紹介し、[Style のドキュメント](style/) で詳しく説明しています。項目の一覧は [`style` モジュール](api:hypersig/style/index.html) にあります。",
            },
        ),
    ])
}

/// The lead of the page of `page`, a group of items rather than the
/// overview.
pub(crate) fn page_lead(page: Reference) -> L<&'static str> {
    page_items(page).0
}

/// The summary and the sections of `page`.
#[expect(clippy::too_many_lines, reason = "one table of data per page")]
fn page_items(page: Reference) -> (L<&'static str>, &'static [Section]) {
    match page {
        Reference::Overview => (L { en: "", ja: "" }, &[]),
        Reference::Page => (
            L {
                en: "`Page` is the full HTML document for a page, with its title, its initial signals, and the runtime.",
                ja: "`Page` はページ全体の HTML 文書です。タイトル、シグナルの初期値、ランタイムを持ちます。",
            },
            &[
                Section {
                    heading: L {
                        en: "Page",
                        ja: "Page",
                    },
                    text: L {
                        en: "`Page::render` evaluates every binding with the initial signals, so the first HTML already contains the values the runtime would compute.",
                        ja: "`Page::render` はすべての束縛をシグナルの初期値で評価します。そのため、最初の HTML には、ランタイムが計算するはずの値がすでに入っています。",
                    },
                    rows: &[
                        (
                            "Page::new",
                            "struct.Page.html#method.new",
                            L {
                                en: "Starts a page with its title and initial signal values.",
                                ja: "タイトルとシグナルの初期値からページを作り始めます。",
                            },
                        ),
                        (
                            "child",
                            "struct.Page.html#method.child",
                            L {
                                en: "Appends a node to `<body>`.",
                                ja: "`<body>` にノードを追加します。",
                            },
                        ),
                        (
                            "on_key",
                            "struct.Page.html#method.on_key",
                            L {
                                en: "Runs `stmt` when `shortcut` is pressed anywhere on the page.",
                                ja: "ページのどこかで `shortcut` が押されると `stmt` を実行します。",
                            },
                        ),
                        (
                            "computed",
                            "struct.Page.html#method.computed",
                            L {
                                en: "Keeps `signal` equal to `value`, recomputed when a signal `value` reads changes.",
                                ja: "`signal` を `value` と等しく保ちます。`value` が読むシグナルが変わると計算し直します。",
                            },
                        ),
                        (
                            "bind_title",
                            "struct.Page.html#method.bind_title",
                            L {
                                en: "Keeps the document's title, the text of the browser tab, equal to `title`.",
                                ja: "文書のタイトル（ブラウザのタブに出る文字列）を `title` と等しく保ちます。",
                            },
                        ),
                        (
                            "persist",
                            "struct.Page.html#method.persist",
                            L {
                                en: "Keeps `signal` in the browser's `localStorage`, so the next page load starts from it.",
                                ja: "`signal` をブラウザの `localStorage` に保存し、次にページを読み込んだときはその値から始めます。",
                            },
                        ),
                        (
                            "css_when",
                            "struct.Page.html#method.css_when",
                            L {
                                en: "Adds `style` to `<body>` while `when` is true.",
                                ja: "`when` が真のあいだ、`<body>` に `style` を追加します。",
                            },
                        ),
                        (
                            "css_match",
                            "struct.Page.html#method.css_match",
                            L {
                                en: "Adds to `<body>` the style of the case whose value `value` equals.",
                                ja: "`value` と等しい値のケースのスタイルを `<body>` に追加します。",
                            },
                        ),
                        (
                            "theme",
                            "struct.Page.html#method.theme",
                            L {
                                en: "Sets token values for the whole page.",
                                ja: "ページ全体のトークンの値を設定します。",
                            },
                        ),
                        (
                            "global",
                            "struct.Page.html#method.global",
                            L {
                                en: "Adds rules for elements by their tag, and fonts, to the page.",
                                ja: "タグで要素を選ぶ規則とフォントをページに追加します。",
                            },
                        ),
                        (
                            "style",
                            "struct.Page.html#method.style",
                            L {
                                en: "Adds `sheet` to `<head>` as a `<style>` block.",
                                ja: "`sheet` を `<style>` ブロックとして `<head>` に追加します。",
                            },
                        ),
                        (
                            "stylesheet",
                            "struct.Page.html#method.stylesheet",
                            L {
                                en: "Adds a `<link rel=\"stylesheet\">` for the file at `href` to `<head>`.",
                                ja: "`href` にあるファイルを読み込む `<link rel=\"stylesheet\">` を `<head>` に追加します。",
                            },
                        ),
                        (
                            "script",
                            "struct.Page.html#method.script",
                            L {
                                en: "Adds a `<script type=\"module\">` for the file at `src` to `<head>`.",
                                ja: "`src` にあるファイルを読み込む `<script type=\"module\">` を `<head>` に追加します。",
                            },
                        ),
                        (
                            "Asset",
                            "struct.Asset.html",
                            L {
                                en: "The URL of a file a page loads.",
                                ja: "ページが読み込むファイルの URL です。",
                            },
                        ),
                        (
                            "lang",
                            "struct.Page.html#method.lang",
                            L {
                                en: "Sets the language of the page, the `lang` attribute of `<html>`, to `tag`.",
                                ja: "ページの言語（`<html>` の `lang` 属性）を `tag` に設定します。",
                            },
                        ),
                        (
                            "runtime_src",
                            "struct.Page.html#method.runtime_src",
                            L {
                                en: "Loads the runtime from `src` instead of `RUNTIME_PATH`.",
                                ja: "`RUNTIME_PATH` の代わりに `src` からランタイムを読み込みます。",
                            },
                        ),
                        (
                            "csp_nonce",
                            "struct.Page.html#method.csp_nonce",
                            L {
                                en: "Puts `nonce` on the page's `<style>` and `<script>` elements.",
                                ja: "ページの `<style>` と `<script>` に `nonce` を付けます。",
                            },
                        ),
                        (
                            "render",
                            "struct.Page.html#method.render",
                            L {
                                en: "Renders the document.",
                                ja: "文書を描画します。",
                            },
                        ),
                        (
                            "render_under",
                            "struct.Page.html#method.render_under",
                            L {
                                en: "Renders the document for a site served under `base`.",
                                ja: "`base` の下で配信するサイトに向けて文書を描画します。",
                            },
                        ),
                        (
                            "value_of",
                            "struct.Page.html#method.value_of",
                            L {
                                en: "The value `signal` starts at in the browser.",
                                ja: "ブラウザで `signal` が最初に持つ値です。",
                            },
                        ),
                        (
                            "simulate",
                            "struct.Page.html#method.simulate",
                            L {
                                en: "Simulates this page in a browser, for a test that clicks and types.",
                                ja: "このページをブラウザで動かしたときを再現します。クリックや入力をするテストで使います。",
                            },
                        ),
                        (
                            "RUNTIME_PATH",
                            "constant.RUNTIME_PATH.html",
                            L {
                                en: "The path `Page` loads the runtime from, which names a hash of `RUNTIME_JS`.",
                                ja: "`Page` がランタイムを読み込むパスです。パスには `RUNTIME_JS` のハッシュが入ります。",
                            },
                        ),
                        (
                            "RUNTIME_JS",
                            "constant.RUNTIME_JS.html",
                            L {
                                en: "The browser runtime, built from `runtime/` in the repository.",
                                ja: "リポジトリの `runtime/` からビルドした、ブラウザのランタイムです。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Clicking and typing in a test",
                        ja: "テストでのクリックと入力",
                    },
                    text: L {
                        en: "`Page::simulate` runs the statements of an event on the server's evaluator and records the calls, links, and element statements it leaves for the server and the browser.",
                        ja: "`Page::simulate` はイベントの文をサーバーの評価器で実行し、サーバーとブラウザに残す呼び出し、リンク、要素の文を記録します。",
                    },
                    rows: &[
                        (
                            "Simulation::value_of",
                            "testing/struct.Simulation.html#method.value_of",
                            L {
                                en: "The value `signal` holds now.",
                                ja: "`signal` がいま持つ値です。",
                            },
                        ),
                        (
                            "Simulation::element",
                            "testing/struct.Simulation.html#method.element",
                            L {
                                en: "The element `target` names, as the page shows it now.",
                                ja: "`target` が指す要素を、いまページに表示されているとおりに返します。",
                            },
                        ),
                        (
                            "Simulation::count",
                            "testing/struct.Simulation.html#method.count",
                            L {
                                en: "The number of elements the page shows now that carry `target`.",
                                ja: "`target` を持つ要素のうち、いまページに表示されている数です。",
                            },
                        ),
                        (
                            "Simulation::click",
                            "testing/struct.Simulation.html#method.click",
                            L {
                                en: "Clicks the element `target` names and returns what the server and browser must do.",
                                ja: "`target` が指す要素をクリックし、サーバーとブラウザがすべき処理を返します。",
                            },
                        ),
                        (
                            "Simulation::input",
                            "testing/struct.Simulation.html#method.input",
                            L {
                                en: "Types `text` into the control `target` names, as one `input` event.",
                                ja: "`target` が指す入力欄に、1 回の `input` イベントとして `text` を入力します。",
                            },
                        ),
                        (
                            "Simulation::change",
                            "testing/struct.Simulation.html#method.change",
                            L {
                                en: "Sets the control `target` names to `text` and commits it.",
                                ja: "`target` が指す入力欄を `text` にして確定します。",
                            },
                        ),
                        (
                            "Simulation::submit",
                            "testing/struct.Simulation.html#method.submit",
                            L {
                                en: "Submits the `<form>` `target` names, which runs the closest `on_submit` from it up.",
                                ja: "`target` が指す `<form>` を送信し、そこから外側へ向かっていちばん近い `on_submit` を実行します。",
                            },
                        ),
                        (
                            "Simulation::pause",
                            "testing/struct.Simulation.html#method.pause",
                            L {
                                en: "Runs the debounced statements, as the browser does once the visitor stops typing.",
                                ja: "訪問者が入力を止めたときのブラウザと同じように、`on_debounced` の文を実行します。",
                            },
                        ),
                        (
                            "Effects::calls",
                            "testing/struct.Effects.html#method.calls",
                            L {
                                en: "The arguments of each call to `endpoint` the event sent, in order.",
                                ja: "イベントが送った `endpoint` の呼び出しの引数を、順に返します。",
                            },
                        ),
                        (
                            "Effects::navigates_to",
                            "testing/struct.Effects.html#method.navigates_to",
                            L {
                                en: "The URL the first `Link::navigate` the event ran loads.",
                                ja: "イベントが最初に実行した `Link::navigate` が読み込む URL です。",
                            },
                        ),
                        (
                            "Effects::runs_in_browser",
                            "testing/struct.Effects.html#method.runs_in_browser",
                            L {
                                en: "Whether the event asked the browser to run `stmt`, such as `FIELD.focus()`.",
                                ja: "`FIELD.focus()` のような `stmt` の実行を、イベントがブラウザに求めたかどうかです。",
                            },
                        ),
                        (
                            "testing::nth",
                            "testing/fn.nth.html",
                            L {
                                en: "The `index`th element, from 0, that carries `target`, in document order.",
                                ja: "`target` を持つ要素のうち、文書の順で 0 から数えて `index` 番目のものです。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Search results and link previews",
                        ja: "検索結果とリンクのプレビュー",
                    },
                    text: L {
                        en: "A canonical URL and the image of a card are absolute, so they join a path to an `Origin`. Navigation loads the next document, so each page writes its own tags.",
                        ja: "canonical の URL とカードの画像は絶対 URL なので、パスを `Origin` につないで作ります。ページ遷移では次の文書を読み込むので、タグはページごとに書きます。",
                    },
                    rows: &[
                        (
                            "description",
                            "struct.Page.html#method.description",
                            L {
                                en: "Sets the summary of the page that search results and link previews show.",
                                ja: "検索結果やリンクのプレビューに出す、ページの要約を設定します。",
                            },
                        ),
                        (
                            "canonical",
                            "struct.Page.html#method.canonical",
                            L {
                                en: "Names `url` on `origin` as the page's one address.",
                                ja: "`origin` 上の `url` をページのただ 1 つのアドレスとして示します。",
                            },
                        ),
                        (
                            "alternate",
                            "struct.Page.html#method.alternate",
                            L {
                                en: "Names `url` on `origin` as the same page in the language `hreflang`.",
                                ja: "`origin` 上の `url` を`hreflang` の言語で書いた同じページとして示します。",
                            },
                        ),
                        (
                            "alternate_default",
                            "struct.Page.html#method.alternate_default",
                            L {
                                en: "Names `url` on `origin` as the page's `x-default` alternate.",
                                ja: "`origin` 上の `url` をページの `x-default` の alternate として示します。",
                            },
                        ),
                        (
                            "noindex",
                            "struct.Page.html#method.noindex",
                            L {
                                en: "Asks search engines to leave the page out of their results.",
                                ja: "ページを検索結果に出さないよう、検索エンジンに求めます。",
                            },
                        ),
                        (
                            "nofollow",
                            "struct.Page.html#method.nofollow",
                            L {
                                en: "Asks search engines not to follow the links on the page.",
                                ja: "ページのリンクをたどらないよう、検索エンジンに求めます。",
                            },
                        ),
                        (
                            "framing",
                            "struct.Page.html#method.framing",
                            L {
                                en: "Sets which pages may show this page in a frame.",
                                ja: "このページを枠に表示してよいページを設定します。",
                            },
                        ),
                        (
                            "Framing",
                            "enum.Framing.html",
                            L {
                                en: "Which pages may show a page in a frame, which `Page::framing` sets.",
                                ja: "ページを枠に表示してよいページです。`Page::framing` で設定します。",
                            },
                        ),
                        (
                            "json_ld",
                            "struct.Page.html#method.json_ld",
                            L {
                                en: "Adds `data` to `<head>` as JSON-LD, the structured data search engines read.",
                                ja: "`data` を検索エンジンが読む構造化データの JSON-LD として `<head>` に追加します。",
                            },
                        ),
                        (
                            "icon",
                            "struct.Page.html#method.icon",
                            L {
                                en: "Uses the file at `href` as the page's icon in tabs and bookmarks.",
                                ja: "`href` のファイルをタブやブックマークに出すページのアイコンにします。",
                            },
                        ),
                        (
                            "open_graph",
                            "struct.Page.html#method.open_graph",
                            L {
                                en: "Adds the card a link to the page shows in chat apps.",
                                ja: "チャットアプリでページへのリンクに出すカードを追加します。",
                            },
                        ),
                        (
                            "Origin",
                            "struct.Origin.html",
                            L {
                                en: "The scheme, host, and optional path prefix a site is served from.",
                                ja: "サイトを配信するスキーム、ホスト、省略できるパスの接頭辞です。",
                            },
                        ),
                        (
                            "Origin::under",
                            "struct.Origin.html#method.under",
                            L {
                                en: "The same origin with the site served under `prefix`, such as `/blog`.",
                                ja: "サイトを `/blog` のような `prefix` の下に配信する、同じオリジンです。",
                            },
                        ),
                        (
                            "Origin::parse",
                            "struct.Origin.html#method.parse",
                            L {
                                en: "Reads an origin and an optional prefix from configuration, such as `https://example.com/blog`.",
                                ja: "`https://example.com/blog` のような設定の値から、オリジンと省略できる接頭辞を読みます。",
                            },
                        ),
                        (
                            "OriginError",
                            "struct.OriginError.html",
                            L {
                                en: "Text that `Origin::parse` refused, with the rule it broke.",
                                ja: "`Origin::parse` が受け付けなかった文字列と、違反していた規則です。",
                            },
                        ),
                        (
                            "OpenGraph",
                            "struct.OpenGraph.html",
                            L {
                                en: "The card a chat app shows for a link to the page.",
                                ja: "チャットアプリがページへのリンクに出すカードです。",
                            },
                        ),
                        (
                            "OpenGraphType",
                            "enum.OpenGraphType.html",
                            L {
                                en: "What a page shows, the `og:type` of its `OpenGraph` card.",
                                ja: "ページが何を見せるかを表す、`OpenGraph` のカードの `og:type` です。",
                            },
                        ),
                        (
                            "LanguageTag",
                            "struct.LanguageTag.html",
                            L {
                                en: "A BCP 47 language tag, such as `ja` or `en-US`, for `lang` and `hreflang`.",
                                ja: "`lang` と `hreflang` に書く、`ja` や `en-US` のような BCP 47 の言語タグです。",
                            },
                        ),
                        (
                            "LanguageTag::parse",
                            "struct.LanguageTag.html#method.parse",
                            L {
                                en: "Reads a tag built at run time.",
                                ja: "実行時に作ったタグを読みます。",
                            },
                        ),
                        (
                            "LanguageTagError",
                            "struct.LanguageTagError.html",
                            L {
                                en: "Text that `LanguageTag::parse` refused, with the rule it broke.",
                                ja: "`LanguageTag::parse` が受け付けなかった文字列と、違反していた規則です。",
                            },
                        ),
                        (
                            "OpenGraphLocale",
                            "struct.OpenGraphLocale.html",
                            L {
                                en: "A language and territory in the `ja_JP` form that `og:locale` takes.",
                                ja: "`og:locale` が受け取る `ja_JP` の形の、言語と地域です。",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::El => (
            L {
                en: "`El<S>` is an HTML element on a page whose signals are `S`. Its methods return the element, so a tree is one expression.",
                ja: "`El<S>` は、シグナルが `S` のページに置く HTML 要素です。メソッドが要素を返すので、要素の木を 1 つの式で書けます。",
            },
            &[
                Section {
                    heading: L {
                        en: "Content and attributes",
                        ja: "内容と属性",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "child",
                            "html/struct.El.html#method.child",
                            L {
                                en: "Appends a child element, text, or template.",
                                ja: "子要素、テキスト、テンプレートのいずれかを追加します。",
                            },
                        ),
                        (
                            "children",
                            "html/struct.El.html#method.children",
                            L {
                                en: "Appends several children. An `Option` counts as zero or one child.",
                                ja: "複数の子を追加します。`Option` は 0 個か 1 個の子として数えます。",
                            },
                        ),
                        (
                            "text",
                            "html/struct.El.html#method.text",
                            L {
                                en: "Appends static text.",
                                ja: "固定のテキストを追加します。",
                            },
                        ),
                        (
                            "attr",
                            "html/struct.El.html#method.attr",
                            L {
                                en: "Sets a plain attribute.",
                                ja: "ふつうの属性を設定します。",
                            },
                        ),
                        (
                            "role",
                            "html/struct.El.html#method.role",
                            L {
                                en: "Sets the WAI-ARIA role a screen reader announces the element by.",
                                ja: "スクリーンリーダーが要素を読み上げるときの WAI-ARIA のロールを設定します。",
                            },
                        ),
                        (
                            "href",
                            "html/struct.El.html#method.href",
                            L {
                                en: "Sets the element's `href`.",
                                ja: "要素の `href` を設定します。",
                            },
                        ),
                        (
                            "src",
                            "html/struct.El.html#method.src",
                            L {
                                en: "Sets the element's `src`, the address of the image, frame, or script it loads.",
                                ja: "要素が読み込む画像、フレーム、スクリプトのアドレスである `src` を設定します。",
                            },
                        ),
                        (
                            "poster",
                            "html/struct.El.html#method.poster",
                            L {
                                en: "Sets the image a `<video>` shows before it plays, as `El::src` sets `src`.",
                                ja: "`<video>` が再生前に表示する画像を、`El::src` と同じように設定します。",
                            },
                        ),
                        (
                            "srcset",
                            "html/struct.El.html#method.srcset",
                            L {
                                en: "Sets the images the browser picks from for an `<img>` or a `<source>`.",
                                ja: "`<img>` や `<source>` でブラウザが選ぶ画像の候補を設定します。",
                            },
                        ),
                        (
                            "Candidate",
                            "html/enum.Candidate.html",
                            L {
                                en: "What an image of `El::srcset` is drawn for.",
                                ja: "`El::srcset` の画像が想定する幅か画素密度です。",
                            },
                        ),
                        (
                            "id",
                            "html/struct.El.html#method.id",
                            L {
                                en: "Sets the element's `id`.",
                                ja: "要素の `id` を設定します。",
                            },
                        ),
                        (
                            "label_for",
                            "html/struct.El.html#method.label_for",
                            L {
                                en: "Points a `<label>` or an `<output>` at the control it names, through `for`.",
                                ja: "`<label>` や `<output>` を、`for` で名前の対象のコントロールに向けます。",
                            },
                        ),
                        (
                            "aria_describedby",
                            "html/struct.El.html#method.aria_describedby",
                            L {
                                en: "Names the elements whose text describes this one, through `aria-describedby`.",
                                ja: "`aria-describedby` で、この要素を説明するテキストを持つ要素を指します。",
                            },
                        ),
                        (
                            "class",
                            "html/struct.El.html#method.class",
                            L {
                                en: "Adds a static class.",
                                ja: "固定のクラスを追加します。",
                            },
                        ),
                        (
                            "css",
                            "html/struct.El.html#method.css",
                            L {
                                en: "Adds the declarations of `style` as atomic classes.",
                                ja: "`style` の宣言をアトミックなクラスとして追加します。",
                            },
                        ),
                        (
                            "node_ref",
                            "html/struct.El.html#method.node_ref",
                            L {
                                en: "Attaches `node_ref`, so its statements act on this element.",
                                ja: "`node_ref` を付け、`node_ref` の文がこの要素を操作するようにします。",
                            },
                        ),
                        (
                            "foreign",
                            "html/struct.El.html#method.foreign",
                            L {
                                en: "Leaves this element's attributes and children to a script, and names it `node_ref`.",
                                ja: "この要素の属性と子要素をスクリプトに任せ、`node_ref` の名前を付けます。",
                            },
                        ),
                        (
                            "render",
                            "html/struct.El.html#method.render",
                            L {
                                en: "Renders the element and its children to an HTML string.",
                                ja: "要素とその子を HTML の文字列に描画します。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Bindings",
                        ja: "束縛",
                    },
                    text: L {
                        en: "Each binding takes an expression over the signals, and the runtime updates the element with the expression's value whenever a signal changes.",
                        ja: "束縛はどれもシグナルを読む式を受け取ります。シグナルが変わるたびに、ランタイムがその式の値で要素を更新します。",
                    },
                    rows: &[
                        (
                            "bind_text",
                            "html/struct.El.html#method.bind_text",
                            L {
                                en: "Replaces the element's text with `text`, kept up to date as signals change.",
                                ja: "要素のテキストを `text` に置き換え、シグナルが変わるたびに更新します。",
                            },
                        ),
                        (
                            "show",
                            "html/struct.El.html#method.show",
                            L {
                                en: "Shows the element only while `when` is true.",
                                ja: "`when` が真のあいだだけ要素を表示します。",
                            },
                        ),
                        (
                            "bind_attr",
                            "html/struct.El.html#method.bind_attr",
                            L {
                                en: "Keeps attribute `name` equal to `value` as signals change.",
                                ja: "シグナルが変わっても、属性 `name` を `value` と等しく保ちます。",
                            },
                        ),
                        (
                            "attr_when",
                            "html/struct.El.html#method.attr_when",
                            L {
                                en: "Sets the boolean attribute `name`, such as `disabled`, while `when` is true.",
                                ja: "`when` が真のあいだ、`disabled` のような真偽値の属性 `name` を設定します。",
                            },
                        ),
                        (
                            "disabled",
                            "html/struct.El.html#method.disabled",
                            L {
                                en: "Disables the control while `when` is true, through `disabled`.",
                                ja: "`when` が真のあいだ、`disabled` でコントロールを無効にします。",
                            },
                        ),
                        (
                            "class_when",
                            "html/struct.El.html#method.class_when",
                            L {
                                en: "Adds `class` while `when` is true.",
                                ja: "`when` が真のあいだ `class` を追加します。",
                            },
                        ),
                        (
                            "css_when",
                            "html/struct.El.html#method.css_when",
                            L {
                                en: "Adds the declarations of `style` while `when` is true.",
                                ja: "`when` が真のあいだ `style` の宣言を追加します。",
                            },
                        ),
                        (
                            "css_match",
                            "html/struct.El.html#method.css_match",
                            L {
                                en: "Adds the style of the case whose value `value` equals.",
                                ja: "値が `value` と等しい場合のスタイルを追加します。",
                            },
                        ),
                        (
                            "bind_style",
                            "html/struct.El.html#method.bind_style",
                            L {
                                en: "Keeps the CSS property `property` equal to `value` as signals change.",
                                ja: "シグナルが変わっても、CSS プロパティ `property` を `value` と等しく保ちます。",
                            },
                        ),
                        (
                            "bind_token",
                            "html/struct.El.html#method.bind_token",
                            L {
                                en: "Keeps `token` equal to `value` on this element and those inside it.",
                                ja: "この要素とその内側の要素で、`token` を `value` と等しく保ちます。",
                            },
                        ),
                        (
                            "bind",
                            "html/struct.El.html#method.bind",
                            L {
                                en: "Binds a form control to a signal in both directions.",
                                ja: "フォームコントロールをシグナルに双方向で束縛します。",
                            },
                        ),
                        (
                            "bind_checked",
                            "html/struct.El.html#method.bind_checked",
                            L {
                                en: "Keeps a checkbox's `checked` state equal to `when`, in one direction.",
                                ja: "チェックボックスの `checked` の状態を一方向で `when` と等しく保ちます。",
                            },
                        ),
                        (
                            "bind_radio",
                            "html/struct.El.html#method.bind_radio",
                            L {
                                en: "Makes this radio button set `signal` to `value` when checked.",
                                ja: "このラジオボタンが選ばれたときに `signal` を `value` にします。",
                            },
                        ),
                        (
                            "bind_select",
                            "html/struct.El.html#method.bind_select",
                            L {
                                en: "Fills this `<select>` with `options` and binds it to `signal`.",
                                ja: "この `<select>` に `options` を入れ、`signal` に束縛します。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Events",
                        ja: "イベント",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "on",
                            "html/struct.El.html#method.on",
                            L {
                                en: "Runs `stmt` on `event`. When `on` is called twice for the same event, the second statement runs after the first.",
                                ja: "`event` が起きると `stmt` を実行します。同じイベントで 2 回呼ぶと、2 回目の文は 1 回目の文の後に実行されます。",
                            },
                        ),
                        (
                            "on_click",
                            "html/struct.El.html#method.on_click",
                            L {
                                en: "Runs `stmt` on click.",
                                ja: "クリックで `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_double_click",
                            "html/struct.El.html#method.on_double_click",
                            L {
                                en: "Runs `stmt` on double click.",
                                ja: "ダブルクリックで `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_input",
                            "html/struct.El.html#method.on_input",
                            L {
                                en: "Runs `stmt` on every edit of a text field.",
                                ja: "テキスト欄が編集されるたびに `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_change",
                            "html/struct.El.html#method.on_change",
                            L {
                                en: "Runs `stmt` on `change`.",
                                ja: "`change` イベントで `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_submit",
                            "html/struct.El.html#method.on_submit",
                            L {
                                en: "Runs `stmt` when a form is submitted, after `preventDefault`.",
                                ja: "フォームが送信されたとき、`preventDefault` の後に `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_blur",
                            "html/struct.El.html#method.on_blur",
                            L {
                                en: "Runs `stmt` when focus leaves the element.",
                                ja: "フォーカスが要素から外れたときに `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_visible",
                            "html/struct.El.html#method.on_visible",
                            L {
                                en: "Runs `stmt` each time the element comes into the viewport.",
                                ja: "要素が表示領域に入るたびに `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_key",
                            "html/struct.El.html#method.on_key",
                            L {
                                en: "Runs `stmt` when `shortcut` is pressed while focus is in this element.",
                                ja: "フォーカスがこの要素の中にあるあいだに `shortcut` が押されると `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_debounced",
                            "html/struct.El.html#method.on_debounced",
                            L {
                                en: "Runs `stmt` once `event` has stopped firing for `quiet`.",
                                ja: "`event` が発生しないまま `quiet` だけ経ったら `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_custom",
                            "html/struct.El.html#method.on_custom",
                            L {
                                en: "Runs the statement `handle` builds whenever `event` reaches this element.",
                                ja: "`event` がこの要素に届くたびに、`handle` が組み立てた文を実行します。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Lifecycle and focus",
                        ja: "ライフサイクルとフォーカス",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "on_load",
                            "html/struct.El.html#method.on_load",
                            L {
                                en: "Runs `stmt` once when the element is added to the document.",
                                ja: "要素が文書に追加されたときに `stmt` を 1 回実行します。",
                            },
                        ),
                        (
                            "on_interval",
                            "html/struct.El.html#method.on_interval",
                            L {
                                en: "Runs `stmt` every `period` while the element is in the document.",
                                ja: "要素が文書にあるあいだ、`period` ごとに `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_timeout",
                            "html/struct.El.html#method.on_timeout",
                            L {
                                en: "Runs `stmt` once, `delay` after the element is added to the document.",
                                ja: "要素が文書に追加されてから `delay` 後に、`stmt` を 1 回実行します。",
                            },
                        ),
                        (
                            "effect",
                            "html/struct.El.html#method.effect",
                            L {
                                en: "Runs `stmt` when the element is added to the document.",
                                ja: "要素が文書に追加されたときに `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_signal_change",
                            "html/struct.El.html#method.on_signal_change",
                            L {
                                en: "Runs `stmt` whenever `signal`, or any field inside it, changes.",
                                ja: "`signal` か、その中のどれかのフィールドが変わるたびに `stmt` を実行します。",
                            },
                        ),
                        (
                            "roving_focus",
                            "html/struct.El.html#method.roving_focus",
                            L {
                                en: "Moves focus between this element's items on the arrow keys, Home, and End.",
                                ja: "矢印キー、Home、End で、この要素の中の項目のあいだでフォーカスを動かします。",
                            },
                        ),
                        (
                            "active_descendant",
                            "html/struct.El.html#method.active_descendant",
                            L {
                                en: "Tracks the active option of `listbox` in `active` on the up and down arrows.",
                                ja: "上下の矢印キーに応じて、`listbox` のアクティブな選択肢を `active` に記録します。",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Html => (
            L {
                en: "The `hypersig::html` module builds elements, lists, and conditional parts.",
                ja: "`hypersig::html` モジュールは要素、リスト、条件付きで現れる部分を組み立てます。",
            },
            &[
                Section {
                    heading: L {
                        en: "Functions",
                        ja: "関数",
                    },
                    text: L {
                        en: "Besides these, the module has one function per common tag, such as `div`, `p`, `button`, `input`, and `ul`.",
                        ja: "このほか、`div`、`p`、`button`、`input`、`ul` のようなよく使うタグには、タグごとに関数が 1 つずつあります。",
                    },
                    rows: &[
                        (
                            "el",
                            "html/fn.el.html",
                            L {
                                en: "Creates an element with a tag that has no named constructor.",
                                ja: "専用の関数がないタグの要素を作ります。",
                            },
                        ),
                        (
                            "each",
                            "html/fn.each.html",
                            L {
                                en: "Renders `render` once per item of `list` in the browser.",
                                ja: "ブラウザで `list` の項目ごとに `render` を 1 回描画します。",
                            },
                        ),
                        (
                            "each_keyed",
                            "html/fn.each_keyed.html",
                            L {
                                en: "`each`, with each item's element identified by `key`.",
                                ja: "`each` と同じですが、各項目の要素を `key` で見分けます。",
                            },
                        ),
                        (
                            "when",
                            "html/fn.when.html",
                            L {
                                en: "Puts `el` in the document only while `cond` is true.",
                                ja: "`cond` が真のあいだだけ `el` を文書に置きます。",
                            },
                        ),
                        (
                            "part",
                            "html/fn.part.html",
                            L {
                                en: "Embeds the `Part` whose signals are the field `at` of this page.",
                                ja: "このページのフィールド `at` をシグナルとする `Part` を埋め込みます。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Types",
                        ja: "型",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "El",
                            "html/struct.El.html",
                            L {
                                en: "An HTML element belonging to a page whose signals are `S`.",
                                ja: "シグナルが `S` のページに属する HTML 要素",
                            },
                        ),
                        (
                            "Node",
                            "html/struct.Node.html",
                            L {
                                en: "A node in the tree.",
                                ja: "木のノード",
                            },
                        ),
                        (
                            "IntoNode",
                            "html/trait.IntoNode.html",
                            L {
                                en: "A value `El::child` takes as a child on the page whose signals are `S`.",
                                ja: "シグナルが `S` のページで `El::child` が子として受け取る値",
                            },
                        ),
                        (
                            "Attr",
                            "html/struct.Attr.html",
                            L {
                                en: "The name of an attribute that `El::attr`, `El::bind_attr`, and `El::attr_when` write.",
                                ja: "`El::attr`、`El::bind_attr`、`El::attr_when` が書く属性の名前",
                            },
                        ),
                        (
                            "Role",
                            "html/enum.Role.html",
                            L {
                                en: "A WAI-ARIA role, which `El::role` writes into `role`.",
                                ja: "`El::role` が `role` に書く WAI-ARIA のロール",
                            },
                        ),
                        (
                            "Id",
                            "html/struct.Id.html",
                            L {
                                en: "The id of an element.",
                                ja: "要素の id",
                            },
                        ),
                        (
                            "Tag",
                            "html/struct.Tag.html",
                            L {
                                en: "The name of an element that `el` creates.",
                                ja: "`el` が作る要素の名前",
                            },
                        ),
                        (
                            "tag!",
                            "macro.tag.html",
                            L {
                                en: "Builds a `Tag` for `html::el`, checked at compile time.",
                                ja: "`html::el` に渡す `Tag` を作り、コンパイル時に検査します。",
                            },
                        ),
                        (
                            "attr!",
                            "macro.attr.html",
                            L {
                                en: "Builds an `Attr` for a name with no constant in `html::attr`.",
                                ja: "`html::attr` に定数のない名前の `Attr` を作ります。",
                            },
                        ),
                        (
                            "Event",
                            "html/enum.Event.html",
                            L {
                                en: "A DOM event a statement can run on.",
                                ja: "文を実行するきっかけにできる DOM イベント",
                            },
                        ),
                        (
                            "CustomEvent",
                            "html/struct.CustomEvent.html",
                            L {
                                en: "An event that a script, such as a custom element, dispatches on an element.",
                                ja: "カスタム要素などのスクリプトが要素に送出するイベント",
                            },
                        ),
                        (
                            "Key",
                            "html/enum.Key.html",
                            L {
                                en: "A key that `El::on_key` filters `keydown` by.",
                                ja: "`El::on_key` が `keydown` を絞り込むのに使うキー",
                            },
                        ),
                        (
                            "Shortcut",
                            "html/struct.Shortcut.html",
                            L {
                                en: "A `Key` and the modifiers held with it, which `El::on_key` and `Page::on_key` match.",
                                ja: "`Key` と、一緒に押す修飾キー。`El::on_key` と `Page::on_key` が照合します。",
                            },
                        ),
                        (
                            "Orientation",
                            "html/enum.Orientation.html",
                            L {
                                en: "A horizontal or vertical direction, which decides the arrow keys `El::roving_focus` uses.",
                                ja: "`El::roving_focus` が使う矢印キーを決める、横か縦の向き",
                            },
                        ),
                        (
                            "Stylesheet",
                            "html/struct.Stylesheet.html",
                            L {
                                en: "CSS text whose class selectors `Stylesheet::class` checks at compile time.",
                                ja: "`Stylesheet::class` がコンパイル時にクラスセレクタを検査する、CSS のテキスト",
                            },
                        ),
                        (
                            "Class",
                            "html/struct.Class.html",
                            L {
                                en: "A class name that `El::class` and `El::class_when` put on an element.",
                                ja: "`El::class` と `El::class_when` が要素に付けるクラス名",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Signals => (
            L {
                en: "`Signals` is the trait the page's state struct implements, and `Sig<S, T>` is a typed reference to one of its fields.",
                ja: "`Signals` はページの状態を持つ構造体が実装するトレイトで、`Sig<S, T>` はそのフィールドの 1 つを指す型付きの参照です。",
            },
            &[
                Section {
                    heading: L {
                        en: "Types",
                        ja: "型",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "Signals",
                            "trait.Signals.html",
                            L {
                                en: "A struct whose fields are signals.",
                                ja: "フィールドがシグナルである構造体",
                            },
                        ),
                        (
                            "Sig",
                            "struct.Sig.html",
                            L {
                                en: "A typed reference to a signal of type `T` inside the signal struct `S`.",
                                ja: "シグナル構造体 `S` の中にある、型が `T` のシグナルを指す型付きの参照",
                            },
                        ),
                        (
                            "Item",
                            "struct.Item.html",
                            L {
                                en: "The current item of a list.",
                                ja: "いま扱っているリストの項目",
                            },
                        ),
                        (
                            "Root",
                            "enum.Root.html",
                            L {
                                en: "Marks a `Sig` whose path starts at the page's signal struct.",
                                ja: "`Sig` のパスがページのシグナル構造体から始まることを表す型",
                            },
                        ),
                        (
                            "Local",
                            "enum.Local.html",
                            L {
                                en: "Marks a `Sig` whose path starts at the current list item.",
                                ja: "`Sig` のパスがリストのいまの項目から始まることを表す型",
                            },
                        ),
                        (
                            "Computed",
                            "enum.Computed.html",
                            L {
                                en: "Marks a `Sig` whose value `Page::computed` keeps up to date.",
                                ja: "`Sig` の値を `Page::computed` が式の値に合わせ続けることを表す型",
                            },
                        ),
                        (
                            "Read",
                            "enum.Read.html",
                            L {
                                en: "Marks a read-only view of a page-rooted `Sig`, which `Sig::read` returns.",
                                ja: "`Sig::read` が返す、ページから始まる `Sig` を読み取り専用で指す参照を表す型",
                            },
                        ),
                        (
                            "Writable",
                            "trait.Writable.html",
                            L {
                                en: "The markers of a `Sig` the browser may write: `Root` and `Local`.",
                                ja: "ブラウザが書き込める `Sig` の印（`Root` と `Local`）",
                            },
                        ),
                        (
                            "FromPage",
                            "trait.FromPage.html",
                            L {
                                en: "The markers of a `Sig` whose path starts at the page's signal struct.",
                                ja: "パスがページのシグナル構造体から始まる `Sig` の印",
                            },
                        ),
                        (
                            "IsComputed",
                            "trait.IsComputed.html",
                            L {
                                en: "The marker of a `Sig` that `Page::computed` accepts: `Computed` alone.",
                                ja: "`Page::computed` が受け取る `Sig` の印（`Computed` だけ）",
                            },
                        ),
                        (
                            "Part",
                            "trait.Part.html",
                            L {
                                en: "A piece of page written once against its own signals.",
                                ja: "自分のシグナルに対して一度だけ書くページの部品",
                            },
                        ),
                        (
                            "Part::render",
                            "trait.Part.html#tymethod.render",
                            L {
                                en: "Renders the part, with its paths written under the field it is embedded at.",
                                ja: "埋め込まれたフィールドの下にパスを書いて、部品を描画します。",
                            },
                        ),
                        (
                            "MAX_DEPTH",
                            "constant.MAX_DEPTH.html",
                            L {
                                en: "The deepest chain of fields a `Sig` can name.",
                                ja: "`Sig` が指せるフィールドの連なりの最大の深さ",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Sig",
                        ja: "Sig",
                    },
                    text: L {
                        en: "A `Sig` is also an expression, so the methods of `Expr` apply to it.",
                        ja: "`Sig` は式でもあるので、`Expr` のメソッドも使えます。",
                    },
                    rows: &[
                        (
                            "at",
                            "struct.Sig.html#method.at",
                            L {
                                en: "The field `field` of the struct this signal holds.",
                                ja: "このシグナルが持つ構造体のフィールド `field`",
                            },
                        ),
                        (
                            "at_computed",
                            "struct.Sig.html#method.at_computed",
                            L {
                                en: "The computed field `field` of the struct this signal holds.",
                                ja: "このシグナルが持つ構造体の、計算されるフィールド `field`",
                            },
                        ),
                        (
                            "set",
                            "struct.Sig.html#method.set",
                            L {
                                en: "A statement that assigns `value` to this signal in the browser.",
                                ja: "ブラウザでこのシグナルに `value` を代入する文",
                            },
                        ),
                        (
                            "toggle",
                            "struct.Sig.html#method.toggle",
                            L {
                                en: "A statement that flips this flag.",
                                ja: "このフラグを反転する文",
                            },
                        ),
                        (
                            "patch",
                            "struct.Sig.html#method.patch",
                            L {
                                en: "A patch that sets this signal to `value` when the server sends it.",
                                ja: "サーバーが送ったときに、このシグナルを `value` にする差分",
                            },
                        ),
                        (
                            "path",
                            "struct.Sig.html#method.path",
                            L {
                                en: "The dotted field path this signal is stored under.",
                                ja: "このシグナルの格納先を表す、ドットで区切ったフィールドのパス",
                            },
                        ),
                        (
                            "read",
                            "struct.Sig.html#method.read",
                            L {
                                en: "A read-only view of this signal, with the same path.",
                                ja: "同じパスを持つ、このシグナルを読み取り専用で指す参照",
                            },
                        ),
                        (
                            "let_in_stmt",
                            "struct.Sig.html#method.let_in_stmt",
                            L {
                                en: "A statement that runs `body` with this signal's value when the statement runs.",
                                ja: "文を実行した時点のこのシグナルの値で、`body` を実行する文",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Lists",
                        ja: "リスト",
                    },
                    text: L {
                        en: "These exist on a `Sig` whose value is a `Vec`.",
                        ja: "以下は値が `Vec` の `Sig` にあるメソッドです。",
                    },
                    rows: &[
                        (
                            "push",
                            "struct.Sig.html#method.push",
                            L {
                                en: "A statement that appends `value` to this list.",
                                ja: "このリストに `value` を追加する文",
                            },
                        ),
                        (
                            "append",
                            "struct.Sig.html#method.append",
                            L {
                                en: "A statement that appends every item of `items` to this list.",
                                ja: "このリストに `items` のすべての項目を追加する文",
                            },
                        ),
                        (
                            "prepend",
                            "struct.Sig.html#method.prepend",
                            L {
                                en: "A statement that adds every item of `items` before the items of this list.",
                                ja: "このリストの項目の前に `items` のすべての項目を加える文",
                            },
                        ),
                        (
                            "patch_append",
                            "struct.Sig.html#method.patch_append",
                            L {
                                en: "A patch that adds `items` after the items the browser holds.",
                                ja: "ブラウザが持つ項目の後ろに `items` を加える差分",
                            },
                        ),
                        (
                            "patch_prepend",
                            "struct.Sig.html#method.patch_prepend",
                            L {
                                en: "A patch that adds `items` before the items the browser holds.",
                                ja: "ブラウザが持つ項目の前に `items` を加える差分",
                            },
                        ),
                        (
                            "patch_upsert",
                            "struct.Sig.html#method.patch_upsert",
                            L {
                                en: "A patch that replaces items by key and appends those with new keys.",
                                ja: "キーで項目を置き換え、新しいキーの項目を末尾に加える差分",
                            },
                        ),
                        (
                            "patch_remove",
                            "struct.Sig.html#method.patch_remove",
                            L {
                                en: "A patch that removes each item whose key is one of `keys`.",
                                ja: "キーが `keys` のどれかに一致する項目を取り除く差分",
                            },
                        ),
                        (
                            "extend",
                            "struct.Sig.html#method.extend",
                            L {
                                en: "A statement that appends `count` items built from their positions.",
                                ja: "位置から作った `count` 個の項目を追加する文",
                            },
                        ),
                        (
                            "remove_at",
                            "struct.Sig.html#method.remove_at",
                            L {
                                en: "A statement that removes the item at `index`, if there is one.",
                                ja: "`index` の位置に項目があれば、それを取り除く文",
                            },
                        ),
                        (
                            "clear",
                            "struct.Sig.html#method.clear",
                            L {
                                en: "A statement that empties this list.",
                                ja: "このリストを空にする文",
                            },
                        ),
                        (
                            "retain",
                            "struct.Sig.html#method.retain",
                            L {
                                en: "A statement that keeps only the items for which `keep` is true.",
                                ja: "`keep` が真になる項目だけを残す文",
                            },
                        ),
                        (
                            "set_all",
                            "struct.Sig.html#method.set_all",
                            L {
                                en: "A statement that sets `field` to `value` on every item.",
                                ja: "すべての項目で `field` を `value` にする文",
                            },
                        ),
                        (
                            "update_where",
                            "struct.Sig.html#method.update_where",
                            L {
                                en: "A statement that sets `field` to `value` on each item `when` accepts.",
                                ja: "`when` が真になる項目で `field` を `value` にする文",
                            },
                        ),
                        (
                            "swap",
                            "struct.Sig.html#method.swap",
                            L {
                                en: "A statement that exchanges the items at `a` and `b`, if both exist.",
                                ja: "`a` と `b` の位置に項目が両方あれば、それらを入れ替える文",
                            },
                        ),
                        (
                            "sort_by",
                            "struct.Sig.html#method.sort_by",
                            L {
                                en: "A statement that sorts the items by `key`, ascending.",
                                ja: "項目を `key` の昇順に並べる文",
                            },
                        ),
                        (
                            "sort_by_desc",
                            "struct.Sig.html#method.sort_by_desc",
                            L {
                                en: "A statement that sorts the items by `key`, descending.",
                                ja: "項目を `key` の降順に並べる文",
                            },
                        ),
                        (
                            "count",
                            "struct.Sig.html#method.count",
                            L {
                                en: "The number of items for which `matches` is true.",
                                ja: "`matches` が真になる項目の数",
                            },
                        ),
                        (
                            "map",
                            "struct.Sig.html#method.map",
                            L {
                                en: "The list of what `value` gives for each item, in order.",
                                ja: "各項目について `value` が返す値を、順に並べたリスト",
                            },
                        ),
                        (
                            "sum_of",
                            "struct.Sig.html#method.sum_of",
                            L {
                                en: "The sum of `value` over the items, 0 for an empty list.",
                                ja: "各項目の `value` の合計。空のリストでは 0",
                            },
                        ),
                        (
                            "max_of",
                            "struct.Sig.html#method.max_of",
                            L {
                                en: "The largest `value` over the items, or `None` for an empty list.",
                                ja: "各項目の `value` のうち最大のもの。空のリストでは `None`",
                            },
                        ),
                        (
                            "min_of",
                            "struct.Sig.html#method.min_of",
                            L {
                                en: "The smallest `value` over the items, or `None` for an empty list.",
                                ja: "各項目の `value` のうち最小のもの。空のリストでは `None`",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Item",
                        ja: "Item",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "at",
                            "struct.Item.html#method.at",
                            L {
                                en: "The field `field` of the item.",
                                ja: "項目のフィールド `field`",
                            },
                        ),
                        (
                            "value",
                            "struct.Item.html#method.value",
                            L {
                                en: "The item itself, as a signal that can be read and written.",
                                ja: "読み書きできるシグナルとしての、項目そのもの",
                            },
                        ),
                        (
                            "index",
                            "struct.Item.html#method.index",
                            L {
                                en: "The item's position in its list, starting at 0.",
                                ja: "0 から数えた、リストの中での項目の位置",
                            },
                        ),
                        (
                            "remove",
                            "struct.Item.html#method.remove",
                            L {
                                en: "A statement that removes this item from its list.",
                                ja: "この項目をリストから取り除く文",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Expr => (
            L {
                en: "`Expr<S, T>` is an expression over the signals of `S` that evaluates to a `T` in the browser.",
                ja: "`Expr<S, T>` は `S` のシグナルを読む式で、ブラウザで `T` の値に評価されます。",
            },
            &[
                Section {
                    heading: L {
                        en: "Building expressions",
                        ja: "式を組み立てる",
                    },
                    text: L {
                        en: "`+`, `-`, `*`, and `%` work on `i32`, `i64`, and `f64`, `/` on `f64`, and `!` on `bool`.",
                        ja: "`+`、`-`、`*`、`%` は `i32`、`i64`、`f64` に、`/` は `f64` に、`!` は `bool` に使えます。",
                    },
                    rows: &[
                        (
                            "Expr::lit",
                            "struct.Expr.html#method.lit",
                            L {
                                en: "A constant value, serialized to JSON once at render time.",
                                ja: "描画のときに 1 回だけ JSON にシリアライズする定数の値",
                            },
                        ),
                        (
                            "Expr::text",
                            "struct.Expr.html#method.text",
                            L {
                                en: "A constant string, for the start of a `concat` chain.",
                                ja: "`concat` の連なりの先頭に置く、定数の文字列",
                            },
                        ),
                        (
                            "if_else",
                            "fn.if_else.html",
                            L {
                                en: "The expression `then` while `cond` is true, and `otherwise` while it is false.",
                                ja: "`cond` が真のあいだは `then`、偽のあいだは `otherwise` の値になる式",
                            },
                        ),
                        (
                            "Expr::now",
                            "struct.Expr.html#method.now",
                            L {
                                en: "The browser's current time, in milliseconds since the Unix epoch.",
                                ja: "Unix エポックからのミリ秒で表した、ブラウザの現在時刻",
                            },
                        ),
                        (
                            "Expr::random_below",
                            "struct.Expr.html#method.random_below",
                            L {
                                en: "A random integer from 0 up to but not including `bound`.",
                                ja: "0 以上 `bound` 未満のランダムな整数",
                            },
                        ),
                        (
                            "at",
                            "struct.Expr.html#method.at",
                            L {
                                en: "The field `field` of the struct this expression evaluates to.",
                                ja: "この式を評価してできる構造体のフィールド `field`",
                            },
                        ),
                        (
                            "with",
                            "struct.Expr.html#method.with",
                            L {
                                en: "A copy of this struct with `field` replaced by `value`.",
                                ja: "この構造体の `field` を `value` に置き換えた複製",
                            },
                        ),
                        (
                            "let_in",
                            "struct.Expr.html#method.let_in",
                            L {
                                en: "The expression `body` builds from this value.",
                                ja: "この値から `body` が作る式",
                            },
                        ),
                        (
                            "let_in_stmt",
                            "struct.Expr.html#method.let_in_stmt",
                            L {
                                en: "A statement that runs `body` with this value, computed once when the statement runs.",
                                ja: "文を実行した時点で 1 回だけ計算したこの値で、`body` を実行する文",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Comparisons and logic",
                        ja: "比較と論理演算",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "equals",
                            "struct.Expr.html#method.equals",
                            L {
                                en: "Whether this value equals `rhs`.",
                                ja: "この値が `rhs` と等しいかどうか",
                            },
                        ),
                        (
                            "not_equals",
                            "struct.Expr.html#method.not_equals",
                            L {
                                en: "Whether this value differs from `rhs`, comparing JSON forms.",
                                ja: "JSON の形で比べたときに、この値が `rhs` と異なるかどうか",
                            },
                        ),
                        (
                            "gt",
                            "struct.Expr.html#method.gt",
                            L {
                                en: "Whether this value is strictly greater than `rhs`.",
                                ja: "この値が `rhs` より大きいかどうか",
                            },
                        ),
                        (
                            "ge",
                            "struct.Expr.html#method.ge",
                            L {
                                en: "Whether this value is greater than or equal to `rhs`.",
                                ja: "この値が `rhs` 以上かどうか",
                            },
                        ),
                        (
                            "lt",
                            "struct.Expr.html#method.lt",
                            L {
                                en: "Whether this value is strictly less than `rhs`.",
                                ja: "この値が `rhs` より小さいかどうか",
                            },
                        ),
                        (
                            "le",
                            "struct.Expr.html#method.le",
                            L {
                                en: "Whether this value is less than or equal to `rhs`.",
                                ja: "この値が `rhs` 以下かどうか",
                            },
                        ),
                        (
                            "and",
                            "struct.Expr.html#method.and",
                            L {
                                en: "Whether both this value and `rhs` are true.",
                                ja: "この値と `rhs` が両方とも真かどうか",
                            },
                        ),
                        (
                            "or",
                            "struct.Expr.html#method.or",
                            L {
                                en: "Whether this value or `rhs` is true.",
                                ja: "この値と `rhs` のどちらかが真かどうか",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Numbers and text",
                        ja: "数値とテキスト",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "min",
                            "struct.Expr.html#method.min",
                            L {
                                en: "The smaller of this value and `rhs`.",
                                ja: "この値と `rhs` のうち小さいほう",
                            },
                        ),
                        (
                            "max",
                            "struct.Expr.html#method.max",
                            L {
                                en: "The larger of this value and `rhs`.",
                                ja: "この値と `rhs` のうち大きいほう",
                            },
                        ),
                        (
                            "to_f64",
                            "struct.Expr.html#method.to_f64",
                            L {
                                en: "This number as an `f64`, which holds every `i32` exactly.",
                                ja: "この数を、どの `i32` も正確に表せる `f64` にした値",
                            },
                        ),
                        (
                            "to_fixed",
                            "struct.Expr.html#method.to_fixed",
                            L {
                                en: "This number as text with `digits` digits after the point.",
                                ja: "この数を小数点以下 `digits` 桁のテキストにしたもの",
                            },
                        ),
                        (
                            "floor",
                            "struct.Expr.html#method.floor",
                            L {
                                en: "The largest whole number at or below this one.",
                                ja: "この数以下で最大の整数",
                            },
                        ),
                        (
                            "ceil",
                            "struct.Expr.html#method.ceil",
                            L {
                                en: "The smallest whole number at or above this one.",
                                ja: "この数以上で最小の整数",
                            },
                        ),
                        (
                            "round",
                            "struct.Expr.html#method.round",
                            L {
                                en: "The nearest whole number, with a halfway value rounded up.",
                                ja: "最も近い整数。ちょうど半分の値は大きいほうに丸めます。",
                            },
                        ),
                        (
                            "log10",
                            "struct.Expr.html#method.log10",
                            L {
                                en: "The base 10 logarithm of this number.",
                                ja: "この数の常用対数",
                            },
                        ),
                        (
                            "pow",
                            "struct.Expr.html#method.pow",
                            L {
                                en: "This number raised to the power `exponent`.",
                                ja: "この数の `exponent` 乗",
                            },
                        ),
                        (
                            "format",
                            "struct.Expr.html#method.format",
                            L {
                                en: "This number as text in `format`, with its digits grouped and its affixes.",
                                ja: "この数を `format` に従って、桁を区切り、前後に文字を付けたテキストにしたもの",
                            },
                        ),
                        (
                            "NumberFormat",
                            "struct.NumberFormat.html",
                            L {
                                en: "How `format` writes a number: decimals, separators, and affixes.",
                                ja: "`format` が数を書く形。小数の桁数、区切り文字、前後に付ける文字",
                            },
                        ),
                        (
                            "plural",
                            "struct.Expr.html#method.plural",
                            L {
                                en: "The form that `form` gives for the plural category of this number in `locale`.",
                                ja: "この数の `locale` での複数形のカテゴリに対して、`form` が返す形",
                            },
                        ),
                        (
                            "PluralCategory",
                            "enum.PluralCategory.html",
                            L {
                                en: "A plural category of the Unicode CLDR, which `Intl.PluralRules` picks for a number.",
                                ja: "`Intl.PluralRules` が数に対して選ぶ、Unicode CLDR の複数形のカテゴリ",
                            },
                        ),
                        (
                            "IntlLocale",
                            "enum.IntlLocale.html",
                            L {
                                en: "A locale that the browser's `Intl` writes text for, and the first HTML matches.",
                                ja: "ブラウザの `Intl` がテキストを書くロケールで、最初の HTML も同じテキストになります。",
                            },
                        ),
                        (
                            "Timestamp",
                            "struct.Timestamp.html",
                            L {
                                en: "A point in time, in milliseconds since the Unix epoch.",
                                ja: "Unix エポックからのミリ秒で表した時刻",
                            },
                        ),
                        (
                            "CalendarDate",
                            "struct.CalendarDate.html",
                            L {
                                en: "A day of the calendar, with no time of day and no time zone.",
                                ja: "時刻もタイムゾーンも持たない、暦の上の一日",
                            },
                        ),
                        (
                            "format_date",
                            "struct.Expr.html#method.format_date",
                            L {
                                en: "This time written in `format` with the browser's `Intl.DateTimeFormat`.",
                                ja: "この時刻をブラウザの `Intl.DateTimeFormat` で `format` に従って書いたテキスト",
                            },
                        ),
                        (
                            "DateFormat",
                            "struct.DateFormat.html",
                            L {
                                en: "How `format_date` writes a `Timestamp`: a locale, styles, and a time zone.",
                                ja: "`format_date` が `Timestamp` を書く形。ロケール、スタイル、タイムゾーン",
                            },
                        ),
                        (
                            "DateStyle",
                            "enum.DateStyle.html",
                            L {
                                en: "How much of the date `DateFormat` writes, as `Intl.DateTimeFormat`'s `dateStyle`.",
                                ja: "`DateFormat` が日付をどこまで書くか。`Intl.DateTimeFormat` の `dateStyle` に当たります。",
                            },
                        ),
                        (
                            "TimeStyle",
                            "enum.TimeStyle.html",
                            L {
                                en: "How much of the time of day `DateFormat` writes, as `Intl.DateTimeFormat`'s `timeStyle`.",
                                ja: "`DateFormat` が時刻をどこまで書くか。`Intl.DateTimeFormat` の `timeStyle` に当たります。",
                            },
                        ),
                        (
                            "TimeZone",
                            "enum.TimeZone.html",
                            L {
                                en: "The time zone `DateFormat` writes a time in.",
                                ja: "`DateFormat` が時刻を書くときのタイムゾーン",
                            },
                        ),
                        (
                            "relative_to",
                            "struct.Expr.html#method.relative_to",
                            L {
                                en: "This time relative to `now` in `locale`, such as `3 minutes ago`.",
                                ja: "この時刻を `now` からの相対時間として、`locale` で `3 minutes ago` のように書いたテキスト",
                            },
                        ),
                        (
                            "to_text",
                            "struct.Expr.html#method.to_text",
                            L {
                                en: "This value as display text, as JavaScript's `String` writes it.",
                                ja: "この値を JavaScript の `String` と同じ書き方で表示用のテキストにしたもの",
                            },
                        ),
                        (
                            "label",
                            "struct.Expr.html#method.label",
                            L {
                                en: "The text `label` gives for this value's variant, with every variant covered.",
                                ja: "この値のバリアントに対して `label` が返すテキスト。どのバリアントにもテキストが必要です。",
                            },
                        ),
                        (
                            "Variants",
                            "trait.Variants.html",
                            L {
                                en: "An enum with `#[derive(Segment)]`, whose variants the derive lists.",
                                ja: "`#[derive(Segment)]` を付けた enum で、derive がバリアントを並べます。",
                            },
                        ),
                        (
                            "concat",
                            "struct.Expr.html#method.concat",
                            L {
                                en: "This string with `rhs` appended.",
                                ja: "この文字列の後ろに `rhs` をつなげた文字列",
                            },
                        ),
                        (
                            "trim",
                            "struct.Expr.html#method.trim",
                            L {
                                en: "This string without leading and trailing whitespace.",
                                ja: "この文字列から前後の空白を除いたもの",
                            },
                        ),
                        (
                            "to_lowercase",
                            "struct.Expr.html#method.to_lowercase",
                            L {
                                en: "This string in lowercase.",
                                ja: "この文字列を小文字にしたもの",
                            },
                        ),
                        (
                            "to_uppercase",
                            "struct.Expr.html#method.to_uppercase",
                            L {
                                en: "This string in uppercase.",
                                ja: "この文字列を大文字にしたもの",
                            },
                        ),
                        (
                            "contains",
                            "struct.Expr.html#method.contains",
                            L {
                                en: "Whether `needle` occurs in this string.",
                                ja: "この文字列に `needle` が含まれるかどうか",
                            },
                        ),
                        (
                            "starts_with",
                            "struct.Expr.html#method.starts_with",
                            L {
                                en: "Whether this string starts with `prefix`.",
                                ja: "この文字列が `prefix` で始まるかどうか",
                            },
                        ),
                        (
                            "char_count",
                            "struct.Expr.html#method.char_count",
                            L {
                                en: "The number of Unicode scalar values, as Rust's `chars().count()`.",
                                ja: "Rust の `chars().count()` と同じく数えた、Unicode スカラー値の数",
                            },
                        ),
                        (
                            "all_chars",
                            "struct.Expr.html#method.all_chars",
                            L {
                                en: "Whether every character of this string is in `class`, true for an empty string.",
                                ja: "この文字列のどの文字も `class` に入るかどうか。空の文字列では true です。",
                            },
                        ),
                        (
                            "any_char",
                            "struct.Expr.html#method.any_char",
                            L {
                                en: "Whether some character of this string is in `class`, false for an empty string.",
                                ja: "この文字列のどれかの文字が `class` に入るかどうか。空の文字列では false です。",
                            },
                        ),
                        (
                            "CharClass",
                            "enum.CharClass.html",
                            L {
                                en: "A set of ASCII characters that `all_chars` and `any_char` test for.",
                                ja: "`all_chars` と `any_char` が調べる ASCII 文字の集まり",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Lists and options",
                        ja: "リストと Option",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "len",
                            "struct.Expr.html#method.len",
                            L {
                                en: "The number of items.",
                                ja: "項目の数",
                            },
                        ),
                        (
                            "nth",
                            "struct.Expr.html#method.nth",
                            L {
                                en: "The item at `index`, or `None` past either end.",
                                ja: "`index` の位置の項目で、両端より外なら `None`",
                            },
                        ),
                        (
                            "is_some",
                            "struct.Expr.html#method.is_some",
                            L {
                                en: "Whether this option holds a value.",
                                ja: "この Option が値を持つかどうか",
                            },
                        ),
                        (
                            "is_none",
                            "struct.Expr.html#method.is_none",
                            L {
                                en: "Whether this option is `None`.",
                                ja: "この Option が `None` かどうか",
                            },
                        ),
                        (
                            "unwrap_or",
                            "struct.Expr.html#method.unwrap_or",
                            L {
                                en: "The held value, or `default` for `None`.",
                                ja: "持っている値で、`None` なら `default`",
                            },
                        ),
                        (
                            "some",
                            "struct.Expr.html#method.some",
                            L {
                                en: "This value wrapped in `Some`.",
                                ja: "この値を `Some` で包んだもの",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Stmt => (
            L {
                en: "`Stmt<S>` is a statement over the signals of `S` that the browser runs on an event.",
                ja: "`Stmt<S>` は `S` のシグナルを扱う文で、イベントが起きるとブラウザが実行します。",
            },
            &[Section {
                heading: L {
                    en: "Stmt",
                    ja: "Stmt",
                },
                text: L {
                    en: "`Sig::set`, `Sig::toggle`, the list statements, `NodeRef`'s statements, and an action's `Call` all produce statements.",
                    ja: "`Sig::set`、`Sig::toggle`、リストを操作する文、`NodeRef` の文、アクションの `Call` は、どれも文を作ります。",
                },
                rows: &[
                    (
                        "then",
                        "struct.Stmt.html#method.then",
                        L {
                            en: "Runs `self`, then `next`.",
                            ja: "`self` を実行し、続けて `next` を実行します。",
                        },
                    ),
                    (
                        "Stmt::when",
                        "struct.Stmt.html#method.when",
                        L {
                            en: "Runs `then` only while `cond` is true.",
                            ja: "`cond` が真のときだけ `then` を実行します。",
                        },
                    ),
                    (
                        "Stmt::if_else",
                        "struct.Stmt.html#method.if_else",
                        L {
                            en: "Runs `then` while `cond` is true and `otherwise` while it is false.",
                            ja: "`cond` が真なら `then` を、偽なら `otherwise` を実行します。",
                        },
                    ),
                    (
                        "Stmt::view_transition",
                        "struct.Stmt.html#method.view_transition",
                        L {
                            en: "Runs `body` inside a view transition, so the browser animates its change.",
                            ja: "`body` を View Transitions の中で実行し、その変化をブラウザにアニメーションさせます。",
                        },
                    ),
                    (
                        "Stmt::none",
                        "struct.Stmt.html#method.none",
                        L {
                            en: "A statement that does nothing.",
                            ja: "何もしない文です。",
                        },
                    ),
                    (
                        "IntoStmt",
                        "trait.IntoStmt.html",
                        L {
                            en: "Converts a value into a statement, as `Call` does.",
                            ja: "`Call` と同じように、値を文に変換します。",
                        },
                    ),
                ],
            }],
        ),
        Reference::NodeRef => (
            L {
                en: "`NodeRef<S>` names an element on a page whose signals are `S`, for calling the element's DOM methods imperatively.",
                ja: "`NodeRef<S>` は、シグナルが `S` のページにある要素に付ける名前で、その要素の DOM のメソッドを命令的に呼び出すときに使います。",
            },
            &[
                Section {
                    heading: L {
                        en: "NodeRef",
                        ja: "NodeRef",
                    },
                    text: L {
                        en: "Declare one with `node_ref!` and attach it to an element with `El::node_ref`.",
                        ja: "`node_ref!` で宣言し、`El::node_ref` で要素に付けます。",
                    },
                    rows: &[
                        (
                            "node_ref!",
                            "macro.node_ref.html",
                            L {
                                en: "Declares a `NodeRef` constant named after its own path.",
                                ja: "定数自身のパスを名前とする `NodeRef` の定数を宣言します。",
                            },
                        ),
                        (
                            "id",
                            "struct.NodeRef.html#method.id",
                            L {
                                en: "An element id spelled as this reference's name.",
                                ja: "この参照の名前と同じ綴りの要素の id",
                            },
                        ),
                        (
                            "focus",
                            "struct.NodeRef.html#method.focus",
                            L {
                                en: "A statement that focuses the element after the next render.",
                                ja: "次の描画の後に要素へフォーカスを移す文",
                            },
                        ),
                        (
                            "blur",
                            "struct.NodeRef.html#method.blur",
                            L {
                                en: "A statement that removes focus from the element.",
                                ja: "要素からフォーカスを外す文",
                            },
                        ),
                        (
                            "select",
                            "struct.NodeRef.html#method.select",
                            L {
                                en: "A statement that selects the element's text, for an `<input>`.",
                                ja: "`<input>` 向けの、要素のテキストを選択する文",
                            },
                        ),
                        (
                            "scroll_into_view",
                            "struct.NodeRef.html#method.scroll_into_view",
                            L {
                                en: "A statement that scrolls the element into view.",
                                ja: "要素が見える位置までスクロールする文",
                            },
                        ),
                        (
                            "show_popover",
                            "struct.NodeRef.html#method.show_popover",
                            L {
                                en: "A statement that shows the element if it is a closed popover.",
                                ja: "要素が閉じたポップオーバーなら、それを表示する文",
                            },
                        ),
                        (
                            "hide_popover",
                            "struct.NodeRef.html#method.hide_popover",
                            L {
                                en: "A statement that hides the element if it is an open popover.",
                                ja: "要素が開いているポップオーバーなら、それを隠す文",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "DialogRef",
                        ja: "DialogRef",
                    },
                    text: L {
                        en: "`DialogRef::dialog` builds the `<dialog>` it names, so its statements always act on a dialog.",
                        ja: "`DialogRef::dialog` が名前を付けた `<dialog>` を作るので、`DialogRef` の文は常にダイアログを操作します。",
                    },
                    rows: &[
                        (
                            "dialog_ref!",
                            "macro.dialog_ref.html",
                            L {
                                en: "Declares a `DialogRef` constant named after its own path.",
                                ja: "定数自身のパスを名前とする `DialogRef` の定数を宣言します。",
                            },
                        ),
                        (
                            "id",
                            "struct.DialogRef.html#method.id",
                            L {
                                en: "An element id spelled as this reference's name, as `NodeRef::id` is.",
                                ja: "`NodeRef::id` と同じく、この参照の名前と同じ綴りの要素の id",
                            },
                        ),
                        (
                            "dialog",
                            "struct.DialogRef.html#method.dialog",
                            L {
                                en: "An empty `<dialog>` that this reference names.",
                                ja: "この参照が名前を付けた空の `<dialog>`",
                            },
                        ),
                        (
                            "show_modal",
                            "struct.DialogRef.html#method.show_modal",
                            L {
                                en: "A statement that opens the dialog with `showModal()`, unless it is open.",
                                ja: "ダイアログが開いていなければ、`showModal()` で開く文",
                            },
                        ),
                        (
                            "close",
                            "struct.DialogRef.html#method.close",
                            L {
                                en: "A statement that closes the dialog, if it is open.",
                                ja: "ダイアログが開いていれば、それを閉じる文",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Action => (
            L {
                en: "`Action<S, A>` is a server endpoint on the page whose signals are `S`, which receives arguments `A`.",
                ja: "`Action<S, A>` は、シグナルが `S` のページから呼ぶサーバーのエンドポイントで、引数 `A` を受け取ります。",
            },
            &[
                Section {
                    heading: L {
                        en: "Action",
                        ja: "Action",
                    },
                    text: L {
                        en: "Declare one on an `action` line of `routes!`, which names its handler, and call it from the page.",
                        ja: "`routes!` の `action` の行にハンドラと一緒に宣言し、ページから呼び出します。",
                    },
                    rows: &[
                        (
                            "routes!",
                            "macro.routes.html",
                            L {
                                en: "Declares the pages and actions of an application and the function that serves them.",
                                ja: "アプリケーションのページとアクション、それを配信する関数を宣言します。",
                            },
                        ),
                        (
                            "call",
                            "struct.Action.html#method.call",
                            L {
                                en: "A call to this action, which takes no arguments.",
                                ja: "引数を取らないこのアクションの呼び出し",
                            },
                        ),
                        (
                            "call_with",
                            "struct.Action.html#method.call_with",
                            L {
                                en: "A call to this action with `args`.",
                                ja: "引数 `args` を渡す、このアクションの呼び出し",
                            },
                        ),
                        (
                            "path",
                            "struct.Action.html#method.path",
                            L {
                                en: "The path the action is served on, without its `Base`.",
                                ja: "アクションを配信するパス（`Base` を除く）",
                            },
                        ),
                        (
                            "method",
                            "struct.Action.html#method.method",
                            L {
                                en: "The method the action is served on.",
                                ja: "アクションを配信するメソッド",
                            },
                        ),
                        (
                            "request",
                            "struct.Action.html#method.request",
                            L {
                                en: "The request the runtime sends to call this action with `args`.",
                                ja: "ランタイムが `args` でこのアクションを呼ぶときに送るリクエスト",
                            },
                        ),
                        (
                            "Method",
                            "enum.Method.html",
                            L {
                                en: "The HTTP method an `Action` is served on.",
                                ja: "`Action` を配信する HTTP メソッド",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Call",
                        ja: "Call",
                    },
                    text: L {
                        en: "A `Call` is usable wherever a `Stmt` is.",
                        ja: "`Call` は `Stmt` を書ける場所ならどこでも使えます。",
                    },
                    rows: &[
                        (
                            "indicator",
                            "struct.Call.html#method.indicator",
                            L {
                                en: "Sets `flag` to true while the response is awaited and to false after.",
                                ja: "リクエストの応答を待つあいだ `flag` を真にし、終わったら偽にします。",
                            },
                        ),
                        (
                            "error",
                            "struct.Call.html#method.error",
                            L {
                                en: "Sets `message` to the error text when the call fails.",
                                ja: "呼び出しが失敗すると `message` をエラーの文言にします。",
                            },
                        ),
                        (
                            "status",
                            "struct.Call.html#method.status",
                            L {
                                en: "Sets `status` to the HTTP status of a failed answer.",
                                ja: "失敗した応答の HTTP ステータスを `status` に入れます。",
                            },
                        ),
                        (
                            "latest",
                            "struct.Call.html#method.latest",
                            L {
                                en: "Aborts the unfinished earlier `latest` call to the same action.",
                                ja: "同じアクションへの `latest` の呼び出しのうち、まだ終わっていない前の呼び出しを取り消します。",
                            },
                        ),
                        (
                            "exclusive",
                            "struct.Call.html#method.exclusive",
                            L {
                                en: "Skips this call while another call to the same action awaits its answer.",
                                ja: "同じアクションへの別の呼び出しが応答を待っている間は、この呼び出しを送りません。",
                            },
                        ),
                        (
                            "timeout",
                            "struct.Call.html#method.timeout",
                            L {
                                en: "Fails this call when its answer has not ended `limit` after it was sent.",
                                ja: "送ってから `limit` が過ぎても応答が終わっていなければ、この呼び出しを失敗にします。",
                            },
                        ),
                        (
                            "on_error",
                            "struct.Call.html#method.on_error",
                            L {
                                en: "Runs the statement `handle` builds when this call fails.",
                                ja: "この呼び出しが失敗したとき、`handle` が作る文を実行します。",
                            },
                        ),
                        (
                            "on_success",
                            "struct.Call.html#method.on_success",
                            L {
                                en: "Runs `next` once the server's answer has arrived and its patches are applied.",
                                ja: "サーバーの応答が届き、そのパッチを適用し終えたら `next` を実行します。",
                            },
                        ),
                        (
                            "then",
                            "struct.Call.html#method.then",
                            L {
                                en: "Sends this call, then runs `next` without waiting for the answer.",
                                ja: "この呼び出しを送り、応答を待たずに `next` を実行します。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "StreamAction and StreamCall",
                        ja: "StreamAction と StreamCall",
                    },
                    text: L {
                        en: "A stream of patches is declared on a `stream` line of `routes!` and served on `GET`, and only its call can reconnect.",
                        ja: "差分のストリームは `routes!` の `stream` の行に宣言して `GET` で配信し、その呼び出しだけが再接続できます。",
                    },
                    rows: &[
                        (
                            "call",
                            "struct.StreamAction.html#method.call",
                            L {
                                en: "A call to this stream, which takes no arguments.",
                                ja: "引数を取らないこのストリームの呼び出し",
                            },
                        ),
                        (
                            "call_with",
                            "struct.StreamAction.html#method.call_with",
                            L {
                                en: "A call to this stream with `args`.",
                                ja: "引数 `args` を渡す、このストリームの呼び出し",
                            },
                        ),
                        (
                            "request",
                            "struct.StreamAction.html#method.request",
                            L {
                                en: "The request the runtime sends to open this stream with `args`.",
                                ja: "ランタイムが `args` でこのストリームを開くときに送るリクエスト",
                            },
                        ),
                        (
                            "connected",
                            "struct.StreamCall.html#method.connected",
                            L {
                                en: "Sets `flag` to true while the server's answer is open, and to false otherwise.",
                                ja: "サーバーの応答が開いている間は `flag` を真にし、それ以外のときは偽にします。",
                            },
                        ),
                        (
                            "reconnect",
                            "struct.StreamCall.html#method.reconnect",
                            L {
                                en: "Opens the stream again when its connection fails, breaks off, or answers 5xx.",
                                ja: "接続が失敗したとき、途中で切れたとき、5xx が返ったときに、ストリームを開き直します。",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Form => (
            L {
                en: "`Form<T>` holds a form's values, the values it started from, and what the visitor has done to it. `Validate` gives the rules that the browser and the server both evaluate.",
                ja: "`Form<T>` はフォームの値、開始時の値、訪問者がフォームでした操作を持ちます。`Validate` はブラウザとサーバーの両方が評価するルールを与えます。",
            },
            &[
                Section {
                    heading: L {
                        en: "Form",
                        ja: "Form",
                    },
                    text: L {
                        en: "Put one in the page's signal struct as a field of type `Form<T>`.",
                        ja: "ページのシグナルの構造体に、型が `Form<T>` のフィールドとして置きます。",
                    },
                    rows: &[
                        (
                            "Form::new",
                            "struct.Form.html#method.new",
                            L {
                                en: "A form that starts from `value`, untouched and not yet submitted.",
                                ja: "`value` から始まる、まだ触れられておらず送信もされていないフォーム",
                            },
                        ),
                        (
                            "Form::validate",
                            "struct.Form.html#method.validate",
                            L {
                                en: "Checks `value` against the rules of `T` on the server.",
                                ja: "サーバーで、`value` を `T` のルールで検査します。",
                            },
                        ),
                        (
                            "Form::value",
                            "struct.Form.html#method.value",
                            L {
                                en: "The values the form holds now.",
                                ja: "フォームがいま持つ値",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Sig<S, Form<T>>",
                        ja: "Sig<S, Form<T>>",
                    },
                    text: L {
                        en: "The methods on the form's signal read and change it on the page.",
                        ja: "フォームのシグナルのメソッドがページでフォームを読み書きします。",
                    },
                    rows: &[
                        (
                            "value",
                            "struct.Sig.html#method.value",
                            L {
                                en: "The values of the form.",
                                ja: "フォームの値",
                            },
                        ),
                        (
                            "field",
                            "struct.Sig.html#method.field",
                            L {
                                en: "The field `field` of the form's values, for binding a control to it.",
                                ja: "コントロールを束縛するための、フォームの値のフィールド `field`",
                            },
                        ),
                        (
                            "error",
                            "struct.Sig.html#method.error",
                            L {
                                en: "The message to show under `field`, or `\"\"`.",
                                ja: "`field` の下に表示するメッセージ。ないときは `\"\"`",
                            },
                        ),
                        (
                            "form",
                            "struct.Sig.html#method.form",
                            L {
                                en: "A `<form>` that runs `then` on submit if every rule passes.",
                                ja: "送信したとき、すべてのルールが通れば `then` を実行する `<form>`",
                            },
                        ),
                        (
                            "touch",
                            "struct.Sig.html#method.touch",
                            L {
                                en: "A statement that marks `field` as left by the visitor.",
                                ja: "`field` を訪問者が離れたフィールドとして記録する文",
                            },
                        ),
                        (
                            "reset",
                            "struct.Sig.html#method.reset",
                            L {
                                en: "A statement that puts back the starting values.",
                                ja: "開始時の値に戻す文",
                            },
                        ),
                        (
                            "is_dirty",
                            "struct.Sig.html#method.is_dirty",
                            L {
                                en: "Whether the form's values differ from the values it started from.",
                                ja: "フォームの値が開始時の値と違うかどうか",
                            },
                        ),
                        (
                            "is_field_dirty",
                            "struct.Sig.html#method.is_field_dirty",
                            L {
                                en: "Whether `field` differs from the value it started from.",
                                ja: "`field` が開始時の値と違うかどうか",
                            },
                        ),
                        (
                            "is_touched",
                            "struct.Sig.html#method.is_touched",
                            L {
                                en: "Whether the visitor has left `field`.",
                                ja: "訪問者が `field` を離れたかどうか",
                            },
                        ),
                        (
                            "is_submitted",
                            "struct.Sig.html#method.is_submitted",
                            L {
                                en: "Whether the visitor has submitted the form since it started or was reset.",
                                ja: "開始またはリセットのあと、訪問者がフォームを送信したかどうか",
                            },
                        ),
                        (
                            "is_valid",
                            "struct.Sig.html#method.is_valid",
                            L {
                                en: "Whether every rule of `T` passes for the form's values.",
                                ja: "フォームの値が `T` のルールをすべて通るかどうか",
                            },
                        ),
                        (
                            "reject",
                            "struct.Sig.html#method.reject",
                            L {
                                en: "A patch that shows `errors` under their fields until the next submit.",
                                ja: "次の送信まで、`errors` をそれぞれのフィールドの下に表示する差分",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Validate and Rules",
                        ja: "Validate と Rules",
                    },
                    text: L {
                        en: "`rules` is generic over the page, so the same expressions are built for the browser and for `Form::validate`.",
                        ja: "`rules` はページについてジェネリックなので、ブラウザ用と `Form::validate` 用に同じ式を作ります。",
                    },
                    rows: &[
                        (
                            "Validate::rules",
                            "trait.Validate.html#method.rules",
                            L {
                                en: "Adds a check to `rules` for each field of `form` that can be wrong.",
                                ja: "`form` のうち誤りうるフィールドごとに、`rules` に検査を追加します。",
                            },
                        ),
                        (
                            "Rules::check",
                            "struct.Rules.html#method.check",
                            L {
                                en: "Reports `message` for `field` while `ok` is false.",
                                ja: "`ok` が偽のあいだ、`field` に `message` を出します。",
                            },
                        ),
                        (
                            "FieldErrors::new",
                            "struct.FieldErrors.html#method.new",
                            L {
                                en: "No messages.",
                                ja: "メッセージのない状態",
                            },
                        ),
                        (
                            "FieldErrors::add",
                            "struct.FieldErrors.html#method.add",
                            L {
                                en: "These messages, with `message` for `field` in place of any it had.",
                                ja: "`field` のメッセージを `message` に置き換えたメッセージの集まり",
                            },
                        ),
                        (
                            "FieldErrors::message",
                            "struct.FieldErrors.html#method.message",
                            L {
                                en: "The message for `field`, if it has one.",
                                ja: "`field` のメッセージ。あれば返します。",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Patches => (
            L {
                en: "`Patches<S>` is what a handler returns: new signal values and re-rendered fragments, which the browser applies in order.",
                ja: "`Patches<S>` はハンドラが返す値です。シグナルの新しい値と描き直したフラグメントを持ち、ブラウザはそれらを順に適用します。",
            },
            &[
                Section {
                    heading: L {
                        en: "Patches",
                        ja: "Patches",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "Patches::new",
                            "struct.Patches.html#method.new",
                            L {
                                en: "An empty batch.",
                                ja: "空のまとまりです。",
                            },
                        ),
                        (
                            "signals",
                            "struct.Patches.html#method.signals",
                            L {
                                en: "Sets signals in the browser.",
                                ja: "ブラウザのシグナルを設定します。",
                            },
                        ),
                        (
                            "list",
                            "struct.Patches.html#method.list",
                            L {
                                en: "Changes items of list signals in the browser, sending only the items that change.",
                                ja: "変わる項目だけを送って、ブラウザのリストのシグナルの項目を変えます。",
                            },
                        ),
                        (
                            "all_signals",
                            "struct.Patches.html#method.all_signals",
                            L {
                                en: "Replaces every signal with the values in `signals`.",
                                ja: "すべてのシグナルを `signals` の値で置き換えます。",
                            },
                        ),
                        (
                            "element",
                            "struct.Patches.html#method.element",
                            L {
                                en: "Re-renders `fragment` and morphs it into the element with its id.",
                                ja: "`fragment` を描き直し、同じ id を持つ要素の違う部分だけを書き換えます。",
                            },
                        ),
                        (
                            "navigate",
                            "struct.Patches.html#method.navigate",
                            L {
                                en: "Loads the page at `url` once the patches before it are applied.",
                                ja: "それより前の差分が適用された後に、`url` のページを読み込みます。",
                            },
                        ),
                        (
                            "push_url",
                            "struct.Patches.html#method.push_url",
                            L {
                                en: "Shows `url` in the address bar as a new history entry, without loading it.",
                                ja: "`url` を新しい履歴の項目としてアドレスバーに出します。ページは読み込みません。",
                            },
                        ),
                        (
                            "replace_url",
                            "struct.Patches.html#method.replace_url",
                            L {
                                en: "Shows `url` in the address bar in place of the current URL.",
                                ja: "今の URL の代わりに `url` をアドレスバーに出します。",
                            },
                        ),
                        (
                            "to_event_stream",
                            "struct.Patches.html#method.to_event_stream",
                            L {
                                en: "The batch as a Server-Sent Events body, for use without the axum feature.",
                                ja: "このまとまりを Server-Sent Events の本文にしたものです。`axum` の feature を使わないときに使います。",
                            },
                        ),
                        (
                            "value_of",
                            "struct.Patches.html#method.value_of",
                            L {
                                en: "The value the batch leaves `signal` at, for a test of an action.",
                                ja: "このまとまりを適用したあとに `signal` が持つ値です。アクションのテストで使います。",
                            },
                        ),
                        (
                            "items_of",
                            "struct.Patches.html#method.items_of",
                            L {
                                en: "The items the batch adds to or replaces in `list`.",
                                ja: "このまとまりが `list` に加える項目と置き換える項目です。",
                            },
                        ),
                        (
                            "removed_keys_of",
                            "struct.Patches.html#method.removed_keys_of",
                            L {
                                en: "The keys the batch removes from `list` by `key`.",
                                ja: "このまとまりが `key` で `list` から取り除くキーです。",
                            },
                        ),
                        (
                            "navigates_to",
                            "struct.Patches.html#method.navigates_to",
                            L {
                                en: "The URL of the first `navigate` in the batch, for a test.",
                                ja: "このまとまりの最初の `navigate` の URL です。テストで使います。",
                            },
                        ),
                        (
                            "replaces",
                            "struct.Patches.html#method.replaces",
                            L {
                                en: "Whether the batch re-renders the fragment `F`, for a test.",
                                ja: "このまとまりがフラグメント `F` を描き直すかどうかです。テストで使います。",
                            },
                        ),
                        (
                            "SignalPatch",
                            "struct.SignalPatch.html",
                            L {
                                en: "New values for one or more signals of `S`.",
                                ja: "`S` の 1 つ以上のシグナルの新しい値です。",
                            },
                        ),
                        (
                            "SignalPatch::and",
                            "struct.SignalPatch.html#method.and",
                            L {
                                en: "Adds the values in `other`. A signal present in both takes `other`'s value.",
                                ja: "`other` の値を加えます。両方にあるシグナルは `other` の値になります。",
                            },
                        ),
                        (
                            "ListPatch",
                            "struct.ListPatch.html",
                            L {
                                en: "Item changes to list signals of `S`, sent without the rest of each list.",
                                ja: "`S` のリストのシグナルの項目への変更です。リストの残りの項目は送りません。",
                            },
                        ),
                        (
                            "ListPatch::and",
                            "struct.ListPatch.html#method.and",
                            L {
                                en: "Adds the changes in `other`, applied after the ones in `self`.",
                                ja: "`other` の変更を加えます。`self` の変更の後に適用されます。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Fragment",
                        ja: "Fragment",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "Fragment",
                            "trait.Fragment.html",
                            L {
                                en: "A part of the page the server can re-render and replace alone.",
                                ja: "サーバーがその部分だけを描き直して置き換えられる、ページの一部分です。",
                            },
                        ),
                        (
                            "Fragment::ID",
                            "trait.Fragment.html#associatedconstant.ID",
                            L {
                                en: "The element id of the fragment's root.",
                                ja: "フラグメントのルート要素の id です。",
                            },
                        ),
                        (
                            "Fragment::render",
                            "trait.Fragment.html#tymethod.render",
                            L {
                                en: "Renders the fragment's root element.",
                                ja: "フラグメントのルート要素を描画します。",
                            },
                        ),
                        (
                            "fragment",
                            "fn.fragment.html",
                            L {
                                en: "Renders `fragment` with its root id set to `Fragment::ID`.",
                                ja: "ルート要素の id を `Fragment::ID` にして `fragment` を描画します。",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Route => (
            L {
                en: "`Route<P, Q>` is a page served at a path, with path parameters `P` and search parameters `Q`. `Link<S>` is a link to that page.",
                ja: "`Route<P, Q>` はパスで配信するページで、パスのパラメータは `P`、クエリパラメータは `Q` です。`Link<S>` はそのページへのリンクです。",
            },
            &[
                Section {
                    heading: L {
                        en: "Route",
                        ja: "Route",
                    },
                    text: L {
                        en: "Declare one in `routes!` with its handler, and use the same value in the links to the page.",
                        ja: "`routes!` の中にハンドラと一緒に宣言し、同じ値をそのページへのリンクにも使います。",
                    },
                    rows: &[
                        (
                            "routes!",
                            "macro.routes.html",
                            L {
                                en: "Declares the pages and actions of an application and the function that serves them.",
                                ja: "アプリケーションのページとアクション、それを配信する関数を宣言します。",
                            },
                        ),
                        (
                            "link",
                            "struct.Route.html#method.link",
                            L {
                                en: "A link to this page, which has no parameters.",
                                ja: "パラメータを持たないこのページへのリンクです。",
                            },
                        ),
                        (
                            "link_with",
                            "struct.Route.html#method.link_with",
                            L {
                                en: "A link to this page with the arguments `args`.",
                                ja: "引数 `args` を渡すこのページへのリンクです。",
                            },
                        ),
                        (
                            "url",
                            "struct.Route.html#method.url",
                            L {
                                en: "The URL of this page with the arguments `args`, written on the server.",
                                ja: "サーバーで書き出す、引数 `args` を渡したこのページの URL です。",
                            },
                        ),
                        (
                            "path",
                            "struct.Route.html#method.path",
                            L {
                                en: "The path as axum matches it, with each parameter still written `{name}`.",
                                ja: "axum が照合する形のパスです。各パラメータは `{name}` のまま残ります。",
                            },
                        ),
                        (
                            "Base",
                            "struct.Base.html",
                            L {
                                en: "The path prefix an application is served under, such as `/app` behind a proxy.",
                                ja: "アプリケーションを配信するパスの接頭辞です。プロキシの裏の `/app` などに使います。",
                            },
                        ),
                        (
                            "Base::new",
                            "struct.Base.html#method.new",
                            L {
                                en: "The prefix `path`, which starts with `/` and does not end with one.",
                                ja: "接頭辞 `path` です。`/` で始まり、`/` で終わりません。",
                            },
                        ),
                        (
                            "Base::ROOT",
                            "struct.Base.html#associatedconstant.ROOT",
                            L {
                                en: "No prefix: the application is served at the root of its host.",
                                ja: "接頭辞なしです。アプリケーションをホストのルートで配信します。",
                            },
                        ),
                        (
                            "Base::as_str",
                            "struct.Base.html#method.as_str",
                            L {
                                en: "The prefix as text, empty for `Base::ROOT`.",
                                ja: "接頭辞の文字列です。`Base::ROOT` では空になります。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Parameters and links",
                        ja: "パラメータとリンク",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "Params",
                            "trait.Params.html",
                            L {
                                en: "The parameters of a `Route`.",
                                ja: "`Route` のパラメータです。",
                            },
                        ),
                        (
                            "Segment",
                            "trait.Segment.html",
                            L {
                                en: "A type one `{name}` segment of a `Route` holds.",
                                ja: "`Route` の `{name}` セグメント 1 つが持つ型です。",
                            },
                        ),
                        (
                            "derive(Segment)",
                            "derive.Segment.html",
                            L {
                                en: "Implements `hypersig::Segment` for an enum whose variants have no fields.",
                                ja: "どのバリアントもフィールドを持たない enum に `hypersig::Segment` を実装します。",
                            },
                        ),
                        (
                            "Search",
                            "trait.Search.html",
                            L {
                                en: "The search parameters of a `Route`: `()`, or a struct with `#[derive(Signals, Search)]`.",
                                ja: "`Route` のクエリパラメータです。`()` か、`#[derive(Signals, Search)]` を付けた構造体です。",
                            },
                        ),
                        (
                            "SearchValue",
                            "trait.SearchValue.html",
                            L {
                                en: "A `Search` field type: a `SearchScalar`, or an `Option` or `Vec` of one.",
                                ja: "`Search` のフィールドの型で、`SearchScalar` か、その `Option` か `Vec` です。",
                            },
                        ),
                        (
                            "SearchScalar",
                            "trait.SearchScalar.html",
                            L {
                                en: "A type that one value of the query string holds.",
                                ja: "クエリ文字列の値 1 つが持つ型です。",
                            },
                        ),
                        (
                            "derive(Search)",
                            "derive.Search.html",
                            L {
                                en: "Checks that each field of a struct can be written into a query string.",
                                ja: "構造体の各フィールドをクエリ文字列に書けることを検査します。",
                            },
                        ),
                        (
                            "PathSegment",
                            "struct.PathSegment.html",
                            L {
                                en: "One segment of a path, never empty, `.`, or `..`.",
                                ja: "パスのセグメント 1 つです。空、`.`、`..` にはなりません。",
                            },
                        ),
                        (
                            "Rest",
                            "struct.Rest.html",
                            L {
                                en: "The rest of a path, from a `{*name}` segment at its end.",
                                ja: "パスの末尾にある `{*name}` セグメントが受け取る残りの部分です。",
                            },
                        ),
                        (
                            "PathSegmentError",
                            "struct.PathSegmentError.html",
                            L {
                                en: "Text that `PathSegment::parse` or `Rest::parse` refused, with the rule it broke.",
                                ja: "`PathSegment::parse` か `Rest::parse` が受け付けなかった文字列と、違反していた規則です。",
                            },
                        ),
                        (
                            "Extends",
                            "trait.Extends.html",
                            L {
                                en: "Parameters that start with the parameters `P` of a parent `Route`.",
                                ja: "親の `Route` のパラメータ `P` で始まるパラメータです。",
                            },
                        ),
                        (
                            "Link",
                            "struct.Link.html",
                            L {
                                en: "A link to a `Route`, for `El::href` on the page whose signals are `S`.",
                                ja: "`Route` へのリンクです。シグナルが `S` のページで `El::href` に渡します。",
                            },
                        ),
                        (
                            "Link::navigate",
                            "struct.Link.html#method.navigate",
                            L {
                                en: "A statement that loads the linked page, as following the link would.",
                                ja: "リンクをたどったときと同じように、リンク先のページを読み込む文です。",
                            },
                        ),
                        (
                            "Link::push_url",
                            "struct.Link.html#method.push_url",
                            L {
                                en: "A statement that shows the linked URL as a new history entry.",
                                ja: "リンク先の URL を新しい履歴の項目として示す文です。",
                            },
                        ),
                        (
                            "Link::replace_url",
                            "struct.Link.html#method.replace_url",
                            L {
                                en: "A statement that shows the linked URL in place of the current one.",
                                ja: "今の URL の代わりにリンク先の URL を示す文です。",
                            },
                        ),
                        (
                            "Url",
                            "struct.Url.html",
                            L {
                                en: "The URL of a page, written on the server by `Route::url`.",
                                ja: "サーバーで `Route::url` が書き出す、ページの URL です。",
                            },
                        ),
                        (
                            "SafeUrl",
                            "struct.SafeUrl.html",
                            L {
                                en: "A URL whose scheme cannot run a script.",
                                ja: "スクリプトを動かせないスキームの URL です。",
                            },
                        ),
                        (
                            "SafeUrl::parse",
                            "struct.SafeUrl.html#method.parse",
                            L {
                                en: "Reads a URL from outside the program, such as one a visitor typed.",
                                ja: "訪問者が入力した URL のような、プログラムの外から来た URL を読みます。",
                            },
                        ),
                        (
                            "UnsafeUrl",
                            "struct.UnsafeUrl.html",
                            L {
                                en: "Text that `SafeUrl::parse` refused, with the rule it broke.",
                                ja: "`SafeUrl::parse` が受け付けなかった文字列と、違反していた規則です。",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::RouterExt => (
            L {
                en: "`RouterExt` adds the browser runtime to an axum `Router`, and the function `routes!` generates adds the pages and actions. Both need the `axum` feature, which is on by default.",
                ja: "`RouterExt` は axum の `Router` にブラウザのランタイムを追加し、`routes!` が生成する関数がページとアクションを追加します。どちらも `axum` の feature が必要で、この feature は既定で有効です。",
            },
            &[
                Section {
                    heading: L {
                        en: "RouterExt",
                        ja: "RouterExt",
                    },
                    text: L { en: "", ja: "" },
                    rows: &[
                        (
                            "hypersig_runtime",
                            "trait.RouterExt.html#tymethod.hypersig_runtime",
                            L {
                                en: "Serves the browser runtime at `RUNTIME_PATH`.",
                                ja: "ブラウザのランタイムを `RUNTIME_PATH` で配信します。",
                            },
                        ),
                        (
                            "serve",
                            "fn.serve.html",
                            L {
                                en: "Serves `app` on the address in `ADDR` or `PORT` until Ctrl+C or SIGTERM.",
                                ja: "`ADDR` か `PORT` のアドレスで、Ctrl+C か SIGTERM を受けるまで `app` を配信します。",
                            },
                        ),
                        (
                            "PageResponse",
                            "struct.PageResponse.html",
                            L {
                                en: "What a page handler returns: a page, a page with a status, a redirect, or a 404.",
                                ja: "ページのハンドラが返す値です。ページ、ステータスを付けたページ、リダイレクト、404 のいずれかです。",
                            },
                        ),
                        (
                            "PageResponse::with_status",
                            "struct.PageResponse.html#method.with_status",
                            L {
                                en: "Answers with `page` and the status `status`, such as a styled `404 Not Found`.",
                                ja: "`page` をステータス `status` で返します。見た目を整えた `404 Not Found` などに使います。",
                            },
                        ),
                        (
                            "Unserved",
                            "struct.Unserved.html",
                            L {
                                en: "A page request that reached no page, handed to the `else` handler of `routes!`.",
                                ja: "どのページにも一致しなかったページのリクエストです。`routes!` の `else` に書いたハンドラが受け取ります。",
                            },
                        ),
                        (
                            "Unserved::status",
                            "struct.Unserved.html#method.status",
                            L {
                                en: "The status the page for this request is sent with: 404 or 400.",
                                ja: "このリクエストに返すページのステータスです。404 か 400 になります。",
                            },
                        ),
                        (
                            "Unserved::within",
                            "struct.Unserved.html#method.within",
                            L {
                                en: "The parameters of `route`, where the request's path starts with that route's path.",
                                ja: "リクエストのパスが `route` のパスで始まるときの、`route` のパラメータです。",
                            },
                        ),
                        (
                            "RequestKind",
                            "enum.RequestKind.html",
                            L {
                                en: "Whether a request loads a page or calls an action.",
                                ja: "リクエストがページを読み込むものか、アクションを呼ぶものかを表します。",
                            },
                        ),
                        (
                            "RequestKind::of",
                            "enum.RequestKind.html#method.of",
                            L {
                                en: "The kind of the request whose head is `request`.",
                                ja: "ヘッダ部が `request` であるリクエストの種類です。",
                            },
                        ),
                        (
                            "ActionResponse",
                            "struct.ActionResponse.html",
                            L {
                                en: "What an action handler returns: patches for the page whose signals are `S`.",
                                ja: "アクションのハンドラが返す値です。シグナルが `S` のページへの差分です。",
                            },
                        ),
                        (
                            "Handler",
                            "trait.Handler.html",
                            L {
                                en: "A function that serves a page, an action, or a stream.",
                                ja: "ページ、アクション、ストリームを配信する関数です。",
                            },
                        ),
                        (
                            "IntoActionResponse",
                            "trait.IntoActionResponse.html",
                            L {
                                en: "A value an action's handler returns for the page whose signals are `S`.",
                                ja: "シグナルが `S` のページに向けて、アクションのハンドラが返す値です。",
                            },
                        ),
                        (
                            "PatchStream",
                            "trait.PatchStream.html",
                            L {
                                en: "A stream of patch batches for the page whose signals are `S`.",
                                ja: "シグナルが `S` のページへの差分のまとまりを流すストリームです。",
                            },
                        ),
                        (
                            "RequestError",
                            "struct.RequestError.html",
                            L {
                                en: "A request to an action or an upload that was refused.",
                                ja: "拒否された、アクションかアップロードへのリクエストです。",
                            },
                        ),
                        (
                            "RequestError::status",
                            "struct.RequestError.html#method.status",
                            L {
                                en: "The status of the response the error answers with.",
                                ja: "このエラーが返す応答のステータスです。",
                            },
                        ),
                    ],
                },
                Section {
                    heading: L {
                        en: "Uploads",
                        ja: "アップロード",
                    },
                    text: L {
                        en: "An upload is an action that also receives the files picked in a file input, declared on an `upload` line of `routes!`. Both carry the page's type.",
                        ja: "アップロードはファイル入力で選んだファイルも受け取るアクションで、`routes!` の `upload` の行に宣言します。どちらもページの型を持ちます。",
                    },
                    rows: &[
                        (
                            "Upload",
                            "struct.Upload.html",
                            L {
                                en: "A server endpoint that receives the files picked in a `FileInput`.",
                                ja: "`FileInput` で選んだファイルを受け取るサーバーのエンドポイントです。",
                            },
                        ),
                        (
                            "Upload::DEFAULT_MAX_BYTES",
                            "struct.Upload.html#associatedconstant.DEFAULT_MAX_BYTES",
                            L {
                                en: "The longest body an upload accepts by default: 8 MiB.",
                                ja: "アップロードが既定で受け付ける本文の上限です。8 MiB です。",
                            },
                        ),
                        (
                            "Upload::call_with",
                            "struct.Upload.html#method.call_with",
                            L {
                                en: "A call that sends the files picked in `files` and `args`.",
                                ja: "`files` で選んだファイルと `args` を送る呼び出しです。",
                            },
                        ),
                        (
                            "FileInput",
                            "struct.FileInput.html",
                            L {
                                en: "A file input on a page whose signals are `S`.",
                                ja: "シグナルが `S` のページにあるファイル入力です。",
                            },
                        ),
                        (
                            "FileInput::input",
                            "struct.FileInput.html#method.input",
                            L {
                                en: "The `<input type=\"file\">` element.",
                                ja: "`<input type=\"file\">` の要素です。",
                            },
                        ),
                        (
                            "FileInput::clear",
                            "struct.FileInput.html#method.clear",
                            L {
                                en: "A statement that empties the input, such as after its files were sent.",
                                ja: "入力を空にする文です。ファイルを送ったあとなどに使います。",
                            },
                        ),
                        (
                            "Uploaded",
                            "struct.Uploaded.html",
                            L {
                                en: "What an upload's handler receives: the arguments and the files.",
                                ja: "アップロードのハンドラが受け取る値です。引数とファイルです。",
                            },
                        ),
                        (
                            "UploadedFile",
                            "struct.UploadedFile.html",
                            L {
                                en: "One file of an upload, read into memory: its name, media type, and bytes.",
                                ja: "メモリに読み込んだ、アップロードの 1 つのファイルです。名前、メディアタイプ、中身を持ちます。",
                            },
                        ),
                        (
                            "FileName",
                            "struct.FileName.html",
                            L {
                                en: "The name a browser sent for an uploaded file, which the visitor chooses.",
                                ja: "アップロードしたファイルについてブラウザが送った名前です。訪問者が決めます。",
                            },
                        ),
                        (
                            "UploadStream",
                            "struct.UploadStream.html",
                            L {
                                en: "What a streaming upload's handler receives.",
                                ja: "ストリームで受け取るアップロードのハンドラが受け取る値です。",
                            },
                        ),
                        (
                            "StreamedFile",
                            "struct.StreamedFile.html",
                            L {
                                en: "One file of a streaming upload, read in chunks.",
                                ja: "チャンクで読む、ストリームのアップロードの 1 つのファイルです。",
                            },
                        ),
                    ],
                },
            ],
        ),
    }
}
