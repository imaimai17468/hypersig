//! The Reference section, one page per group of items.
//!
//! Each row links to the item in the rustdoc output that the site serves
//! under `api/`, and its summary is the first sentence of the item's doc
//! comment, shortened where it runs past one line of the table. The binary checks every link against that output when it is
//! given `--rustdoc`, so a renamed or removed item fails the build.

use crate::Site;
use crate::doc::{Doc, Reference};
use crate::lang::L;
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
        let rows: Vec<Vec<&str>> = cells
            .iter()
            .map(|[name, summary]| vec![name.as_str(), summary.as_str()])
            .collect();
        let rows: Vec<&[&str]> = rows.iter().map(Vec::as_slice).collect();
        children.push(table_of(
            site,
            L {
                en: &["Item", "Summary"],
                ja: &["項目", "概要"],
            },
            rows.as_slice(),
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
    let rows: Vec<Vec<&str>> = rows
        .iter()
        .map(|[page, summary]| vec![page.as_str(), summary.as_str()])
        .collect();
    let rows: Vec<&[&str]> = rows.iter().map(Vec::as_slice).collect();
    parts([
        title(Doc::Reference(Reference::Overview).title(site)),
        lead(
            site,
            L {
                en: "The items of the `hypersig` crate, grouped by what they build. Each name links to its full documentation.",
                ja: "`hypersig` クレートの項目を、組み立てるものごとにまとめています。名前をクリックすると、その項目の完全なドキュメントを開けます。",
            },
        ),
        para(
            site,
            L {
                en: "The [complete API documentation](api:hypersig/index.html) is the rustdoc output of the same commit as this site, with every method, its signature, and its examples. The pages here list the items you reach for most, each with the first sentence of its doc comment, shortened where it runs long.",
                ja: "[API ドキュメントの全体](api:hypersig/index.html)は、このサイトと同じコミットから rustdoc で生成したもので、すべてのメソッドのシグネチャと例を載せています。ここに並べたのはよく使う項目で、それぞれにドキュメントコメントの最初の 1 文の訳を添えています。長い文は短くしています。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Page", "What it covers"],
                ja: &["ページ", "扱う内容"],
            },
            rows.as_slice(),
        ),
        para(
            site,
            L {
                en: "Styles live in `hypersig::style`, re-exported from the `hypersig_style` crate. See [Styling](learn/styling/) and the [`style` module](api:hypersig/style/index.html).",
                ja: "スタイルは `hypersig::style` にあり、`hypersig_style` クレートを再エクスポートしています。[スタイル](learn/styling/)と [`style` モジュール](api:hypersig/style/index.html)をご覧ください。",
            },
        ),
    ])
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
            &[Section {
                heading: L {
                    en: "Page",
                    ja: "Page",
                },
                text: L {
                    en: "`Page::render` evaluates every binding with the initial signals, so the first HTML already shows what the runtime would compute.",
                    ja: "`Page::render` はすべての束縛をシグナルの初期値で評価します。そのため、最初の HTML にはランタイムが計算するはずの結果がすでに表示されています。",
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
                        "computed",
                        "struct.Page.html#method.computed",
                        L {
                            en: "Keeps `signal` equal to `value`, recomputed when a signal `value` reads changes.",
                            ja: "`signal` を `value` と等しく保ちます。`value` が読むシグナルが変わると計算し直します。",
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
                        "lang",
                        "struct.Page.html#method.lang",
                        L {
                            en: "Sets the language of the page, the `lang` attribute of `<html>`, to the BCP 47 tag `tag`.",
                            ja: "ページの言語（`<html>` の `lang` 属性）を BCP 47 のタグ `tag` に設定します。",
                        },
                    ),
                    (
                        "runtime_src",
                        "struct.Page.html#method.runtime_src",
                        L {
                            en: "Loads the runtime from `src` instead of `RUNTIME_PATH`, for a page served as a static file.",
                            ja: "ランタイムを `RUNTIME_PATH` ではなく `src` から読み込みます。静的ファイルとして配信するページに使います。",
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
                        "RUNTIME_PATH",
                        "constant.RUNTIME_PATH.html",
                        L {
                            en: "The path `Page` loads the runtime from.",
                            ja: "`Page` がランタイムを読み込むパスです。",
                        },
                    ),
                    (
                        "RUNTIME_JS",
                        "constant.RUNTIME_JS.html",
                        L {
                            en: "The browser runtime, built from `runtime/` in the repository.",
                            ja: "ブラウザのランタイムです。リポジトリの `runtime/` からビルドしたものです。",
                        },
                    ),
                ],
            }],
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
                            "href",
                            "html/struct.El.html#method.href",
                            L {
                                en: "Points the element's `href` at a page the router serves.",
                                ja: "要素の `href` を、ルーターが配信するページに向けます。",
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
                                ja: "`node_ref` を付け、その文がこの要素に作用するようにします。",
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
                        en: "Each binding takes an expression over the signals and is kept up to date as they change.",
                        ja: "束縛はどれもシグナルを読む式を受け取り、シグナルが変わるたびに最新の値に更新されます。",
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
                                ja: "`value` と等しい値を持つ場合のスタイルを追加します。",
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
                                ja: "フォームの入力部品をシグナルに双方向で束縛します。",
                            },
                        ),
                        (
                            "bind_checked",
                            "html/struct.El.html#method.bind_checked",
                            L {
                                en: "Keeps a checkbox's `checked` state equal to `when`, in one direction.",
                                ja: "チェックボックスの `checked` の状態を、一方向で `when` と等しく保ちます。",
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
                                en: "Runs `stmt` on `event`. A second call for the same event runs after the first.",
                                ja: "`event` が起きると `stmt` を実行します。同じイベントに 2 回目の呼び出しをすると、その文は 1 回目の後に実行されます。",
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
                                ja: "`change` で `stmt` を実行します。",
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
                            "on_key",
                            "html/struct.El.html#method.on_key",
                            L {
                                en: "Runs `stmt` when `key` is pressed while focus is in this element.",
                                ja: "フォーカスがこの要素の中にあるあいだに `key` が押されると `stmt` を実行します。",
                            },
                        ),
                        (
                            "on_debounced",
                            "html/struct.El.html#method.on_debounced",
                            L {
                                en: "Runs `stmt` once `event` has stopped firing for `quiet`.",
                                ja: "`event` が `quiet` のあいだ発生しなくなったら `stmt` を実行します。",
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
                                en: "Runs `stmt` once when the element enters the document.",
                                ja: "要素が文書に入ったときに `stmt` を 1 回実行します。",
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
                                en: "Runs `stmt` once, `delay` after the element enters the document, if it is still in it then.",
                                ja: "要素が文書に入ってから `delay` 後に、そのとき要素がまだ文書にあれば `stmt` を 1 回実行します。",
                            },
                        ),
                        (
                            "effect",
                            "html/struct.El.html#method.effect",
                            L {
                                en: "Runs `stmt` when the element enters the document and whenever a signal it reads changes.",
                                ja: "要素が文書に入ったときと、`stmt` が読むシグナルが変わるたびに `stmt` を実行します。",
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
                                en: "Moves focus between the items inside this element on the arrow keys, Home, and End.",
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
                ja: "`hypersig::html` モジュールは、要素、リスト、条件付きで現れる部分を組み立てます。",
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
                                en: "Creates an element with any tag.",
                                ja: "任意のタグの要素を作ります。",
                            },
                        ),
                        (
                            "each",
                            "html/fn.each.html",
                            L {
                                en: "Renders `render` once per item of `list`, in the browser, kept in step with the list.",
                                ja: "`list` の項目ごとに `render` をブラウザで 1 回描画し、リストの変化に合わせて保ちます。",
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
                                ja: "シグナルが `S` のページに属する HTML 要素です。",
                            },
                        ),
                        (
                            "Node",
                            "html/struct.Node.html",
                            L {
                                en: "A node in the tree: an element, a run of text, or a template the browser expands.",
                                ja: "木のノードです。要素、ひと続きのテキスト、ブラウザが展開するテンプレートのいずれかです。",
                            },
                        ),
                        (
                            "Event",
                            "html/enum.Event.html",
                            L {
                                en: "A DOM event a statement can run on.",
                                ja: "文を実行するきっかけにできる DOM イベントです。",
                            },
                        ),
                        (
                            "Key",
                            "html/enum.Key.html",
                            L {
                                en: "A key that `El::on_key` filters `keydown` by.",
                                ja: "`El::on_key` が `keydown` を絞り込むのに使うキーです。",
                            },
                        ),
                        (
                            "Orientation",
                            "html/enum.Orientation.html",
                            L {
                                en: "A horizontal or vertical direction, which `El::roving_focus` reads as its arrow keys.",
                                ja: "横か縦の向きです。`El::roving_focus` はこれを見て使う矢印キーを決めます。",
                            },
                        ),
                        (
                            "Stylesheet",
                            "html/struct.Stylesheet.html",
                            L {
                                en: "CSS text whose class selectors `Stylesheet::class` checks at compile time.",
                                ja: "CSS のテキストです。そのクラスセレクタを `Stylesheet::class` がコンパイル時に検査します。",
                            },
                        ),
                        (
                            "Class",
                            "html/struct.Class.html",
                            L {
                                en: "A class name that `El::class` and `El::class_when` put on an element.",
                                ja: "`El::class` と `El::class_when` が要素に付けるクラス名です。",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::Signals => (
            L {
                en: "`Signals` marks the page's state struct, and `Sig<S, T>` is a typed reference to one of its fields.",
                ja: "`Signals` はページの状態を持つ構造体に付けるトレイトで、`Sig<S, T>` はそのフィールドの 1 つを指す型付きの参照です。",
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
                                ja: "フィールドがシグナルである構造体です。",
                            },
                        ),
                        (
                            "Sig",
                            "struct.Sig.html",
                            L {
                                en: "A typed reference to a signal of type `T` inside the signal struct `S`.",
                                ja: "シグナル構造体 `S` の中にある、型が `T` のシグナルを指す型付きの参照です。",
                            },
                        ),
                        (
                            "Item",
                            "struct.Item.html",
                            L {
                                en: "The current item of a list, inside `each` or a list predicate such as `Sig::count`.",
                                ja: "`each` や `Sig::count` のようなリストの述語の中で、いま扱っているリストの項目です。",
                            },
                        ),
                        (
                            "Root",
                            "enum.Root.html",
                            L {
                                en: "Marks a `Sig` rooted at the page's signal struct.",
                                ja: "ページのシグナル構造体を起点とする `Sig` の印です。",
                            },
                        ),
                        (
                            "Local",
                            "enum.Local.html",
                            L {
                                en: "Marks a `Sig` rooted at the current item of an `each` or a list predicate.",
                                ja: "`each` やリストの述語の中で、いま扱っている項目を起点とする `Sig` の印です。",
                            },
                        ),
                        (
                            "MAX_DEPTH",
                            "constant.MAX_DEPTH.html",
                            L {
                                en: "The deepest chain of fields a `Sig` can name.",
                                ja: "`Sig` がたどれるフィールドの連なりの最大の深さです。",
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
                                ja: "このシグナルが持つ構造体のフィールド `field` です。",
                            },
                        ),
                        (
                            "set",
                            "struct.Sig.html#method.set",
                            L {
                                en: "A statement that assigns `value` to this signal in the browser.",
                                ja: "ブラウザでこのシグナルに `value` を代入する文です。",
                            },
                        ),
                        (
                            "toggle",
                            "struct.Sig.html#method.toggle",
                            L {
                                en: "A statement that flips this flag.",
                                ja: "このフラグを反転する文です。",
                            },
                        ),
                        (
                            "patch",
                            "struct.Sig.html#method.patch",
                            L {
                                en: "A patch that sets this signal to `value` when the server sends it.",
                                ja: "サーバーが送ると、このシグナルを `value` にする差分です。",
                            },
                        ),
                        (
                            "path",
                            "struct.Sig.html#method.path",
                            L {
                                en: "The dotted field path this signal is stored under.",
                                ja: "このシグナルを格納する、ドットで区切ったフィールドのパスです。",
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
                                ja: "このリストに `value` を追加する文です。",
                            },
                        ),
                        (
                            "extend",
                            "struct.Sig.html#method.extend",
                            L {
                                en: "A statement that appends `count` items, building each from its position `i`.",
                                ja: "`count` 個の項目を追加する文です。各項目は位置 `i` から作ります。",
                            },
                        ),
                        (
                            "remove_at",
                            "struct.Sig.html#method.remove_at",
                            L {
                                en: "A statement that removes the item at `index`, if there is one.",
                                ja: "`index` の位置に項目があれば、それを取り除く文です。",
                            },
                        ),
                        (
                            "clear",
                            "struct.Sig.html#method.clear",
                            L {
                                en: "A statement that empties this list.",
                                ja: "このリストを空にする文です。",
                            },
                        ),
                        (
                            "retain",
                            "struct.Sig.html#method.retain",
                            L {
                                en: "A statement that keeps only the items for which `keep` is true.",
                                ja: "`keep` が真になる項目だけを残す文です。",
                            },
                        ),
                        (
                            "set_all",
                            "struct.Sig.html#method.set_all",
                            L {
                                en: "A statement that sets `field` to `value` on every item.",
                                ja: "すべての項目で `field` を `value` にする文です。",
                            },
                        ),
                        (
                            "update_where",
                            "struct.Sig.html#method.update_where",
                            L {
                                en: "A statement that sets `field` to `value` on the items for which `when` is true.",
                                ja: "`when` が真になる項目で `field` を `value` にする文です。",
                            },
                        ),
                        (
                            "swap",
                            "struct.Sig.html#method.swap",
                            L {
                                en: "A statement that exchanges the items at `a` and `b`, if both exist.",
                                ja: "`a` と `b` の位置に項目が両方あれば、それらを入れ替える文です。",
                            },
                        ),
                        (
                            "sort_by",
                            "struct.Sig.html#method.sort_by",
                            L {
                                en: "A statement that orders the items by `key`, ascending, keeping the order of equal keys.",
                                ja: "項目を `key` の昇順に並べる文です。キーが等しい項目は元の順序を保ちます。",
                            },
                        ),
                        (
                            "count",
                            "struct.Sig.html#method.count",
                            L {
                                en: "The number of items for which `matches` is true.",
                                ja: "`matches` が真になる項目の数です。",
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
                                ja: "項目のフィールド `field` です。",
                            },
                        ),
                        (
                            "value",
                            "struct.Item.html#method.value",
                            L {
                                en: "The item itself, as a signal that can be read and written.",
                                ja: "項目そのものです。読み書きできるシグナルとして扱えます。",
                            },
                        ),
                        (
                            "index",
                            "struct.Item.html#method.index",
                            L {
                                en: "The item's position in its list, starting at 0.",
                                ja: "リストの中での項目の位置です。0 から数えます。",
                            },
                        ),
                        (
                            "remove",
                            "struct.Item.html#method.remove",
                            L {
                                en: "A statement that removes this item from its list.",
                                ja: "この項目をリストから取り除く文です。",
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
                        en: "`+`, `-`, `*`, and `%` work on `i32` and `f64`, `/` on `f64`, and `!` on `bool`.",
                        ja: "`+`、`-`、`*`、`%` は `i32` と `f64` に、`/` は `f64` に、`!` は `bool` に使えます。",
                    },
                    rows: &[
                        (
                            "Expr::lit",
                            "struct.Expr.html#method.lit",
                            L {
                                en: "A constant value, serialized to JSON once at render time.",
                                ja: "定数の値です。描画のときに 1 回だけ JSON にシリアライズします。",
                            },
                        ),
                        (
                            "if_else",
                            "fn.if_else.html",
                            L {
                                en: "The expression `then` while `cond` is true, and `otherwise` while it is false.",
                                ja: "`cond` が真のあいだは式 `then`、偽のあいだは `otherwise` になる式です。",
                            },
                        ),
                        (
                            "Expr::now",
                            "struct.Expr.html#method.now",
                            L {
                                en: "The browser's current time, in milliseconds since the Unix epoch.",
                                ja: "ブラウザの現在時刻です。Unix エポックからのミリ秒で表します。",
                            },
                        ),
                        (
                            "Expr::random_below",
                            "struct.Expr.html#method.random_below",
                            L {
                                en: "A random integer from 0 up to but not including `bound`, drawn in the browser.",
                                ja: "0 以上 `bound` 未満のランダムな整数で、ブラウザで選びます。",
                            },
                        ),
                        (
                            "at",
                            "struct.Expr.html#method.at",
                            L {
                                en: "The field `field` of the struct this expression evaluates to.",
                                ja: "この式を評価してできる構造体のフィールド `field` です。",
                            },
                        ),
                        (
                            "with",
                            "struct.Expr.html#method.with",
                            L {
                                en: "A copy of this struct with `field` replaced by `value`.",
                                ja: "この構造体の `field` を `value` に置き換えた複製です。",
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
                                ja: "この値が `rhs` と等しいかどうかです。",
                            },
                        ),
                        (
                            "not_equals",
                            "struct.Expr.html#method.not_equals",
                            L {
                                en: "Whether this value differs from `rhs`, comparing JSON forms.",
                                ja: "この値が `rhs` と異なるかどうかです。JSON の形で比べます。",
                            },
                        ),
                        (
                            "gt",
                            "struct.Expr.html#method.gt",
                            L {
                                en: "Whether this value is strictly greater than `rhs`.",
                                ja: "この値が `rhs` より大きいかどうかです。",
                            },
                        ),
                        (
                            "ge",
                            "struct.Expr.html#method.ge",
                            L {
                                en: "Whether this value is greater than or equal to `rhs`.",
                                ja: "この値が `rhs` 以上かどうかです。",
                            },
                        ),
                        (
                            "lt",
                            "struct.Expr.html#method.lt",
                            L {
                                en: "Whether this value is strictly less than `rhs`.",
                                ja: "この値が `rhs` より小さいかどうかです。",
                            },
                        ),
                        (
                            "le",
                            "struct.Expr.html#method.le",
                            L {
                                en: "Whether this value is less than or equal to `rhs`.",
                                ja: "この値が `rhs` 以下かどうかです。",
                            },
                        ),
                        (
                            "and",
                            "struct.Expr.html#method.and",
                            L {
                                en: "Whether both this value and `rhs` are true.",
                                ja: "この値と `rhs` が両方とも真かどうかです。",
                            },
                        ),
                        (
                            "or",
                            "struct.Expr.html#method.or",
                            L {
                                en: "Whether this value or `rhs` is true.",
                                ja: "この値と `rhs` のどちらかが真かどうかです。",
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
                                ja: "この値と `rhs` のうち小さいほうです。",
                            },
                        ),
                        (
                            "max",
                            "struct.Expr.html#method.max",
                            L {
                                en: "The larger of this value and `rhs`.",
                                ja: "この値と `rhs` のうち大きいほうです。",
                            },
                        ),
                        (
                            "to_f64",
                            "struct.Expr.html#method.to_f64",
                            L {
                                en: "This integer as an `f64`, which holds every `i32` exactly.",
                                ja: "この整数を `f64` にしたものです。`f64` はどの `i32` も正確に表せます。",
                            },
                        ),
                        (
                            "to_fixed",
                            "struct.Expr.html#method.to_fixed",
                            L {
                                en: "This number as text with `digits` digits after the point.",
                                ja: "この数を、小数点以下 `digits` 桁のテキストにしたものです。",
                            },
                        ),
                        (
                            "to_text",
                            "struct.Expr.html#method.to_text",
                            L {
                                en: "This value as display text.",
                                ja: "この値を表示用のテキストにしたものです。",
                            },
                        ),
                        (
                            "concat",
                            "struct.Expr.html#method.concat",
                            L {
                                en: "Appends `rhs` to this string.",
                                ja: "この文字列の後ろに `rhs` をつなげます。",
                            },
                        ),
                        (
                            "trim",
                            "struct.Expr.html#method.trim",
                            L {
                                en: "This string without leading and trailing whitespace.",
                                ja: "この文字列から前後の空白を除いたものです。",
                            },
                        ),
                        (
                            "to_lowercase",
                            "struct.Expr.html#method.to_lowercase",
                            L {
                                en: "This string in lowercase.",
                                ja: "この文字列を小文字にしたものです。",
                            },
                        ),
                        (
                            "to_uppercase",
                            "struct.Expr.html#method.to_uppercase",
                            L {
                                en: "This string in uppercase.",
                                ja: "この文字列を大文字にしたものです。",
                            },
                        ),
                        (
                            "contains",
                            "struct.Expr.html#method.contains",
                            L {
                                en: "Whether `needle` occurs in this string.",
                                ja: "この文字列に `needle` が含まれるかどうかです。",
                            },
                        ),
                        (
                            "starts_with",
                            "struct.Expr.html#method.starts_with",
                            L {
                                en: "Whether this string starts with `prefix`.",
                                ja: "この文字列が `prefix` で始まるかどうかです。",
                            },
                        ),
                        (
                            "char_count",
                            "struct.Expr.html#method.char_count",
                            L {
                                en: "The number of Unicode scalar values, as Rust's `chars().count()`.",
                                ja: "Unicode スカラー値の数です。Rust の `chars().count()` と同じ数え方です。",
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
                                ja: "項目の数です。",
                            },
                        ),
                        (
                            "nth",
                            "struct.Expr.html#method.nth",
                            L {
                                en: "The item at `index`, or `None` past either end.",
                                ja: "`index` の位置の項目です。両端より外なら `None` です。",
                            },
                        ),
                        (
                            "is_some",
                            "struct.Expr.html#method.is_some",
                            L {
                                en: "Whether this option holds a value.",
                                ja: "この Option が値を持つかどうかです。",
                            },
                        ),
                        (
                            "is_none",
                            "struct.Expr.html#method.is_none",
                            L {
                                en: "Whether this option is `None`.",
                                ja: "この Option が `None` かどうかです。",
                            },
                        ),
                        (
                            "unwrap_or",
                            "struct.Expr.html#method.unwrap_or",
                            L {
                                en: "The held value, or `default` for `None`.",
                                ja: "持っている値です。`None` なら `default` です。",
                            },
                        ),
                        (
                            "some",
                            "struct.Expr.html#method.some",
                            L {
                                en: "This value wrapped in `Some`.",
                                ja: "この値を `Some` で包んだものです。",
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
                en: "`NodeRef<S>` names an element on a page whose signals are `S`, for imperative DOM calls.",
                ja: "`NodeRef<S>` は、シグナルが `S` のページにある要素に名前を付け、その要素へ DOM の命令的な呼び出しをするために使います。",
            },
            &[Section {
                heading: L {
                    en: "NodeRef",
                    ja: "NodeRef",
                },
                text: L {
                    en: "Attach one to an element with `El::node_ref`.",
                    ja: "`El::node_ref` で要素に付けます。",
                },
                rows: &[
                    (
                        "NodeRef::new",
                        "struct.NodeRef.html#method.new",
                        L {
                            en: "A reference named `name`.",
                            ja: "`name` という名前の参照です。",
                        },
                    ),
                    (
                        "focus",
                        "struct.NodeRef.html#method.focus",
                        L {
                            en: "A statement that focuses the element after the next render.",
                            ja: "次の描画の後に要素へフォーカスを移す文です。",
                        },
                    ),
                    (
                        "blur",
                        "struct.NodeRef.html#method.blur",
                        L {
                            en: "A statement that removes focus from the element.",
                            ja: "要素からフォーカスを外す文です。",
                        },
                    ),
                    (
                        "select",
                        "struct.NodeRef.html#method.select",
                        L {
                            en: "A statement that selects the element's text, for an `<input>`.",
                            ja: "要素のテキストを選択する文です。`<input>` に使います。",
                        },
                    ),
                    (
                        "scroll_into_view",
                        "struct.NodeRef.html#method.scroll_into_view",
                        L {
                            en: "A statement that scrolls the element into view.",
                            ja: "要素が見える位置までスクロールする文です。",
                        },
                    ),
                    (
                        "show_popover",
                        "struct.NodeRef.html#method.show_popover",
                        L {
                            en: "A statement that shows the element if it is a popover that is not open.",
                            ja: "要素が開いていないポップオーバーなら、それを表示する文です。",
                        },
                    ),
                    (
                        "hide_popover",
                        "struct.NodeRef.html#method.hide_popover",
                        L {
                            en: "A statement that hides the element if it is an open popover.",
                            ja: "要素が開いているポップオーバーなら、それを隠す文です。",
                        },
                    ),
                ],
            }],
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
                        en: "Declare one as a `const` and use the same value on the page and in the router.",
                        ja: "`const` として宣言し、同じ値をページとルーターの両方で使います。",
                    },
                    rows: &[
                        (
                            "Action::post",
                            "struct.Action.html#method.post",
                            L {
                                en: "An action served on `POST path`.",
                                ja: "`POST path` で配信するアクションです。",
                            },
                        ),
                        (
                            "Action::get",
                            "struct.Action.html#method.get",
                            L {
                                en: "An action served on `GET path`. Use it for a long-lived stream of patches.",
                                ja: "`GET path` で配信するアクションです。差分を長く流し続けるストリームに使います。",
                            },
                        ),
                        (
                            "call",
                            "struct.Action.html#method.call",
                            L {
                                en: "A call to this action, which takes no arguments.",
                                ja: "引数を取らないこのアクションの呼び出しです。",
                            },
                        ),
                        (
                            "call_with",
                            "struct.Action.html#method.call_with",
                            L {
                                en: "A call to this action with `args`.",
                                ja: "引数 `args` を渡すこのアクションの呼び出しです。",
                            },
                        ),
                        (
                            "path",
                            "struct.Action.html#method.path",
                            L {
                                en: "The path the action is served on.",
                                ja: "アクションを配信するパスです。",
                            },
                        ),
                        (
                            "method",
                            "struct.Action.html#method.method",
                            L {
                                en: "The method the action is served on.",
                                ja: "アクションを配信するメソッドです。",
                            },
                        ),
                        (
                            "Method",
                            "enum.Method.html",
                            L {
                                en: "The HTTP method an `Action` is served on.",
                                ja: "`Action` を配信する HTTP メソッドです。",
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
                                en: "Sets `flag` to true while the request is in flight and to false after.",
                                ja: "リクエストの応答を待つあいだ `flag` を真にし、終わったら偽にします。",
                            },
                        ),
                        (
                            "error",
                            "struct.Call.html#method.error",
                            L {
                                en: "Sets `message` to the error text when the call fails, and to `\"\"` when it succeeds.",
                                ja: "呼び出しが失敗すると `message` をエラーの文言にし、成功すると `\"\"` にします。",
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
            ],
        ),
        Reference::Patches => (
            L {
                en: "`Patches<S>` is what a handler answers with: new signal values and re-rendered fragments, applied in order.",
                ja: "`Patches<S>` はハンドラが返す答えです。シグナルの新しい値と描き直したフラグメントを持ち、ブラウザは順に適用します。",
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
                            "to_event_stream",
                            "struct.Patches.html#method.to_event_stream",
                            L {
                                en: "The batch as a Server-Sent Events body, for use without the axum feature.",
                                ja: "このまとまりを Server-Sent Events の本文にしたものです。`axum` の feature を使わないときに使います。",
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
                                en: "A part of the page the server can re-render and replace on its own.",
                                ja: "サーバーが単独で描き直して置き換えられる、ページの一部分です。",
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
                en: "`Route<P, Q>` is a page served at a path whose parameters are `P`, with search parameters `Q`, and `Link<S>` points to one.",
                ja: "`Route<P, Q>` は、パスのパラメータが `P`、検索パラメータが `Q` のパスで配信するページで、`Link<S>` はそのページを指します。",
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
                                en: "Declares the pages of an application and the function that serves them.",
                                ja: "アプリケーションのページと、それを配信する関数を宣言します。",
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
                                ja: "引数 `args` を渡したこのページの URL で、サーバーで書き出します。",
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
                                en: "The parameters of a `Route`: `()`, one `Segment`, or a tuple of two to six.",
                                ja: "`Route` のパラメータです。`()`、`Segment` 1 つ、要素が 2 個から 6 個のタプルのいずれかです。",
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
                                ja: "`Route` の検索パラメータです。`()` か、`#[derive(Signals, Search)]` を付けた構造体です。",
                            },
                        ),
                        (
                            "SearchValue",
                            "trait.SearchValue.html",
                            L {
                                en: "A type one field of `Search` parameters holds: one value of the query string.",
                                ja: "`Search` パラメータのフィールド 1 つが持つ型で、クエリ文字列の値 1 つに当たります。",
                            },
                        ),
                        (
                            "derive(Search)",
                            "derive.Search.html",
                            L {
                                en: "Checks that each field of a struct can be one value of a query string.",
                                ja: "構造体の各フィールドがクエリ文字列の値 1 つになれることを検査します。",
                            },
                        ),
                        (
                            "Rest",
                            "struct.Rest.html",
                            L {
                                en: "The rest of a path, from a `{*name}` segment at its end, which may hold `/`.",
                                ja: "パスの末尾にある `{*name}` セグメントが受け取る残りの部分で、`/` を含められます。",
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
                            "Url",
                            "struct.Url.html",
                            L {
                                en: "The URL of a page, written on the server by `Route::url`.",
                                ja: "ページの URL で、サーバーで `Route::url` が書き出します。",
                            },
                        ),
                    ],
                },
            ],
        ),
        Reference::RouterExt => (
            L {
                en: "`RouterExt` adds hypersig routes to an axum `Router`. It needs the `axum` feature, which is on by default.",
                ja: "`RouterExt` は axum の `Router` に hypersig のルートを足します。`axum` の feature が必要で、この feature は既定で有効です。",
            },
            &[Section {
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
                        "action",
                        "trait.RouterExt.html#tymethod.action",
                        L {
                            en: "Serves `action` with a handler that answers with one batch of patches.",
                            ja: "差分のまとまりを 1 つ返すハンドラで `action` を配信します。",
                        },
                    ),
                    (
                        "stream",
                        "trait.RouterExt.html#tymethod.stream",
                        L {
                            en: "Serves `action` with a handler that answers with a stream of patches.",
                            ja: "差分のストリームを返すハンドラで `action` を配信します。",
                        },
                    ),
                    (
                        "PageResponse",
                        "struct.PageResponse.html",
                        L {
                            en: "What a page handler answers with: a page, a redirect, or a 404.",
                            ja: "ページのハンドラが返す答えです。ページ、リダイレクト、404 のいずれかです。",
                        },
                    ),
                    (
                        "RequestError",
                        "struct.RequestError.html",
                        L {
                            en: "A request to an action whose arguments did not deserialize.",
                            ja: "引数をデシリアライズできなかった、アクションへのリクエストです。",
                        },
                    ),
                ],
            }],
        ),
    }
}
