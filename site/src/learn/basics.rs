//! The quick start and the page on building the page.

use super::snippet::{Snippet, show};
use super::{Program, program};
use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, note, para, parts, title};

pub fn quick_start(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::QuickStart).title(site)),
        lead(
            site,
            L {
                en: "Build a counter that counts in the browser and resets on the server, in one Rust file.",
                ja: "ブラウザで数を数え、サーバーで 0 に戻すカウンターを、Rust のファイル 1 つで作ります。",
            },
        ),
        heading(
            site,
            L {
                en: "Add the dependencies",
                ja: "依存クレートを追加する",
            },
        ),
        para(
            site,
            L {
                en: "hypersig is not on crates.io yet, so it comes from Git. Add it to `Cargo.toml` with axum, serde, and tokio:",
                ja: "hypersig はまだ crates.io に公開していないので、Git から取得します。axum、serde、tokio と一緒に `Cargo.toml` に追加してください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = { git = "https://github.com/imaimai17468/hypersig" }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }"#,
        ),
        heading(
            site,
            L {
                en: "Write the server",
                ja: "サーバーを書く",
            },
        ),
        para(
            site,
            L {
                en: "Write `src/main.rs`. It holds the page, the handler that answers calls from the reset button, and the router that serves both.",
                ja: "`src/main.rs` を書きます。このファイルにはページ、リセットボタンからの呼び出しに答えるハンドラ、その 2 つを配信するルーターが入ります。",
            },
        ),
        program(Program::QuickStart),
        heading(
            site,
            L {
                en: "Run it",
                ja: "動かす",
            },
        ),
        code::block(Lang::Shell, None, "cargo run"),
        para(
            site,
            L {
                en: "Open `http://127.0.0.1:3000`. The `+1` button counts in the browser without a request. The reset button calls `/reset`, and the handler's patch sets `count` back to 0. `hypersig_runtime()` serves the browser runtime from the crate, so no Node or JavaScript build is involved.",
                ja: "`http://127.0.0.1:3000` を開いてください。`+1` ボタンはリクエストを送らずにブラウザで数を増やします。リセットボタンは `/reset` を呼び出し、ハンドラが返す差分が `count` を 0 に戻します。ブラウザのランタイムは `hypersig_runtime()` がクレートから配信するので、Node も JavaScript のビルドも使いません。",
            },
        ),
        heading(
            site,
            L {
                en: "What each part does",
                ja: "各部分の役割",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`#[derive(Signals)]` on `Counter` adds `Counter::count()`, a `Sig<Counter, i32>` that names the field. A misspelled field is a compile error.",
                    ja: "`Counter` に付けた `#[derive(Signals)]` は、フィールドを指す `Sig<Counter, i32>` を返す `Counter::count()` を追加します。フィールド名を打ち間違えるとコンパイルエラーになります。",
                },
                L {
                    en: "`count.set(count + 1)` is a statement. `on_click` writes it into a `data-hs-on-click` attribute as JSON, and the runtime runs it on each click.",
                    ja: "`count.set(count + 1)` は文です。`on_click` はこれを JSON にして `data-hs-on-click` 属性に書き込み、ランタイムはクリックのたびにこの文を実行します。",
                },
                L {
                    en: "`span().bind_text(count.to_text())` keeps the text equal to the count. `Page::render` evaluates it with the initial values, so the HTML already shows `0`.",
                    ja: "`span().bind_text(count.to_text())` はテキストをカウントの値に合わせ続けます。`Page::render` が初期値でこの式を評価するので、HTML の時点で `0` が表示されています。",
                },
                L {
                    en: "`INDEX` is a `Route`, the page at `/`. `routes!` declares it together with its handler `index`, the function `pages` it writes serves it, and a link to it is `INDEX.link()`. [Pages and links](learn/pages/) covers routes with parameters.",
                    ja: "`INDEX` は `/` のページを表す `Route` です。`routes!` がハンドラ `index` と一緒にこのルートを宣言し、マクロが書く関数 `pages` がこのページを配信します。このページへのリンクは `INDEX.link()` です。パラメータを持つルートは [ページとリンク](learn/pages/) で説明します。",
                },
                L {
                    en: "`RESET` is an `Action<Counter>`. The page calls it with `RESET.call()` and the router serves it with `.action(RESET, reset)`, so both sides share its path, its method, its argument type, and the page it belongs to.",
                    ja: "`RESET` は `Action<Counter>` です。ページは `RESET.call()` で呼び出し、ルーターは `.action(RESET, reset)` で配信するので、パス、メソッド、引数の型、属するページを両側で共有します。",
                },
                L {
                    en: "The handler returns `Counter::count().patch(0)`. The server sends it as a Server-Sent Event, and the runtime writes the value into the signal, which rewrites the text.",
                    ja: "ハンドラは `Counter::count().patch(0)` を返します。サーバーはこれを Server-Sent Event として送り、ランタイムが値をシグナルに書き込むと、テキストが書き換わります。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Try it here",
                ja: "ここで試す",
            },
        ),
        para(
            site,
            L {
                en: "The counter below runs the browser side of the program on this page. It uses buttons from `hypersig_ui`, the component crate in the same repository.",
                ja: "下のカウンターはこのプログラムのブラウザ側をこのページで動かしています。ボタンには同じリポジトリにあるコンポーネントのクレート `hypersig_ui` のものを使っています。",
            },
        ),
        demo::show(site, Demo::Counter),
        note(
            site,
            L {
                en: "This site is a set of static files on GitHub Pages, so its demos run on signals alone and none of them calls an action.",
                ja: "このサイトは GitHub Pages に置いた静的ファイルなので、デモはシグナルだけで動き、アクションを呼び出すものはありません。",
            },
        ),
        para(
            site,
            L {
                en: "Next, [Building the page](learn/describing-the-page/) shows how the HTML is built.",
                ja: "HTML の組み立て方は次の [ページを組み立てる](learn/describing-the-page/) で説明します。",
            },
        ),
    ])
}

pub fn describing_the_page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DescribingThePage).title(site)),
        lead(
            site,
            L {
                en: "A page is a tree of `El` values built with Rust functions and rendered to HTML on the server.",
                ja: "ページは Rust の関数で組み立てた `El` の値の木で、サーバーが HTML に描画します。",
            },
        ),
        heading(
            site,
            L {
                en: "Elements",
                ja: "要素",
            },
        ),
        para(
            site,
            L {
                en: "The [`html`](api:hypersig/html/index.html) module has one function per common tag, such as `div()`, `p()`, and `button()`, and `el(\"tag\")` for any other. Each returns an `El<S>`, where `S` is the page's signal struct, and its methods return the element, so a tree is one expression.",
                ja: "[`html`](api:hypersig/html/index.html) モジュールには `div()`、`p()`、`button()` のようによく使うタグごとの関数と、それ以外のタグに使う `el(\"tag\")` があります。どれも `El<S>` を返します。`S` はページのシグナル構造体です。`El` のメソッドは要素そのものを返すので、木全体を 1 つの式で書けます。",
            },
        ),
        show(Snippet::Elements),
        para(
            site,
            L {
                en: "`text` appends static text, and `child` appends an element or a string. `href` points a link at a page from a `Route`, which [Pages and links](learn/pages/) covers. `attr` sets a plain attribute. `id` and `class` have their own methods, and `attr` panics on them and on names starting with `data-hs-`, which only the typed methods write.",
                ja: "`text` は固定のテキストを、`child` は要素か文字列を末尾に追加します。`href` は `Route` からページへのリンクを張ります。これは [ページとリンク](learn/pages/) で説明します。`attr` は普通の属性を設定します。`id` と `class` には専用のメソッドがあり、`attr` にこの 2 つや `data-hs-` で始まる名前を渡すとパニックします。`data-hs-` の属性は型付きのメソッドだけが書きます。",
            },
        ),
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
                en: "[`Page`](api:hypersig/struct.Page.html) wraps the tree in a full document. `Page::new` takes the title and the initial values of the signals, `child` appends to `<body>`, and `render` returns the HTML a handler sends. The document loads the runtime from `RUNTIME_PATH`, a path under `/_hypersig/` that names a hash of the file, and `RouterExt::hypersig_runtime` serves it there.",
                ja: "[`Page`](api:hypersig/struct.Page.html) は木を包んで完全な HTML 文書にします。`Page::new` はタイトルとシグナルの初期値を受け取り、`child` は `<body>` に要素を追加し、`render` はハンドラが送る HTML を返します。文書は `RUNTIME_PATH` からランタイムを読み込みます。このパスは `/_hypersig/` の下にあり、ファイルのハッシュを名前に含みます。`RouterExt::hypersig_runtime` がそのパスでファイルを配信します。",
            },
        ),
        heading(
            site,
            L {
                en: "Values that follow the signals",
                ja: "シグナルに追従する値",
            },
        ),
        para(
            site,
            L {
                en: "An element can show a value that changes in the browser. Each binding takes an expression over the signals and is rendered with the initial values first, so the HTML shows what the runtime would compute.",
                ja: "ブラウザで変わる値を要素に表示できます。束縛はそれぞれシグナルを使った式を受け取り、最初は初期値で描画されるので、HTML にはランタイムが計算するのと同じ値が入っています。",
            },
        ),
        show(Snippet::Bindings),
        bullets(
            site,
            &[
                L {
                    en: "`bind_text` replaces the element's text.",
                    ja: "`bind_text` は要素のテキストを置き換えます。",
                },
                L {
                    en: "`bind_attr` keeps an attribute equal to a string expression, and `attr_when` adds a boolean attribute such as `disabled` while a condition holds.",
                    ja: "`bind_attr` は属性を文字列の式の値に合わせ続けます。`attr_when` は、条件が成り立つ間だけ `disabled` のような真偽値の属性を付けます。",
                },
                L {
                    en: "`show` hides the element while a condition is false. `when(cond, el)` removes it instead, so its `on_load` runs again and its inputs start empty each time it returns.",
                    ja: "`show` は条件が偽の間、要素を隠します。`when(cond, el)` は要素を取り除くので、要素が戻るたびに `on_load` がもう一度実行され、入力欄は空から始まります。",
                },
                L {
                    en: "`class_when` and `css_when` add a class or a style while a condition holds. [Styling](learn/styling/) covers them.",
                    ja: "`class_when` と `css_when` は、条件が成り立つ間だけクラスやスタイルを付けます。詳しくは [スタイル](learn/styling/) で説明します。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The demo binds a text field to `name` and shows one of two paragraphs depending on whether it is empty.",
                ja: "このデモはテキスト欄を `name` に束縛し、空かどうかで 2 つの段落のどちらかを表示します。",
            },
        ),
        demo::show(site, Demo::Greeting),
        heading(
            site,
            L {
                en: "Components",
                ja: "コンポーネント",
            },
        ),
        para(
            site,
            L {
                en: "A component is a Rust function that returns an `El<S>`. Make it generic over `S` and take the signals it reads as arguments, and it works on any page.",
                ja: "コンポーネントは `El<S>` を返す Rust の関数です。`S` についてジェネリックにし、読むシグナルを引数で受け取れば、どのページでも使えます。",
            },
        ),
        show(Snippet::Component),
        para(
            site,
            L {
                en: "`hypersig_ui` is a set of such functions styled after shadcn/ui. The demos on this site use its buttons and text fields.",
                ja: "`hypersig_ui` は shadcn/ui に倣った見た目のこうした関数を集めたクレートです。このサイトのデモはそのボタンとテキスト欄を使っています。",
            },
        ),
    ])
}
