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
    parts(vec![
        title(Doc::Learn(Learn::QuickStart).title(site)),
        lead(site, Doc::Learn(Learn::QuickStart).leads()),
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
                en: "Create a project with Rust 1.99 or later:",
                ja: "Rust 1.99 以降でプロジェクトを作ります。",
            },
        ),
        code::block(Lang::Shell, None, "cargo new counter\ncd counter"),
        para(
            site,
            L {
                en: "Replace the empty `[dependencies]` section that `cargo new` wrote in `Cargo.toml` with hypersig, axum, serde, and tokio:",
                ja: "`cargo new` が `Cargo.toml` に書いた空の `[dependencies]` の節を、hypersig、axum、serde、tokio を並べた次の節に置き換えてください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = "0.5"
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
                ja: "`src/main.rs` を書きます。このファイルにはページ、リセットボタンからの呼び出しに応答するハンドラ、その 2 つを配信するルーターが入ります。",
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
                en: "Open `http://127.0.0.1:3000`. The `+1` button counts in the browser without a request. The reset button calls `/reset`, and the handler's patch sets `count` back to 0. The runtime is the small JavaScript file that runs the page's statements and bindings in the browser. `hypersig_runtime()` serves it from the crate, so no Node or JavaScript build is involved.",
                ja: "`http://127.0.0.1:3000` を開いてください。`+1` ボタンはリクエストを送らずにブラウザで数を増やします。リセットボタンは `/reset` を呼び出し、ハンドラが返す差分が `count` を 0 に戻します。ランタイムはページの文と束縛をブラウザで実行する小さな JavaScript のファイルです。`hypersig_runtime()` がこのファイルをクレートから配信するので、Node も JavaScript のビルドも使いません。",
            },
        ),
        para(
            site,
            L {
                en: "While you edit, run the server with `cargo hypersig dev` in place of `cargo run`. Install the command first:",
                ja: "コードを書き換えながら動かすときは、`cargo run` の代わりに `cargo hypersig dev` でサーバーを起動します。先にコマンドをインストールしてください。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo install --locked hypersig_cli\ncargo hypersig dev",
        ),
        para(
            site,
            L {
                en: "`cargo hypersig dev` starts the server as `cargo run` does, and builds and starts it again each time a file it is built from changes. In a debug build, each open page keeps a stream to the server, so the tab reloads from the new server on its own. A build that fails leaves the last server running. The state the server holds in memory starts over at each restart.",
                ja: "`cargo hypersig dev` は `cargo run` と同じようにサーバーを起動し、ビルド元のファイルが変わるたびにビルドして起動し直します。デバッグビルドでは開いているページがサーバーへのストリームを保つので、タブは新しいサーバーから自動で読み込み直されます。ビルドが失敗したときは前のサーバーが動き続けます。サーバーがメモリに持つ状態は、起動し直すたびに初期化されます。",
            },
        ),
        para(
            site,
            L {
                en: "The reloaded tab keeps the signals the visitor changed, such as a half-typed draft. Just before the reload, the page saves in the tab's `sessionStorage` each top-level signal whose value differs from the server's last write of it, in the page or in a patch. The new page puts those back over the new server's values, and every other signal, such as a list the server patched, takes the new server's value. It does so only where the new page shows the same path and query and its signal struct has the same field names and types, which `#[derive(Signals)]` writes into the page in a debug build. A change inside a nested struct alone is not detected. Otherwise the page starts from the server's values. The code that does this is a script of its own, which only a debug build serves, so a release build's runtime carries none of it.",
                ja: "読み込み直したタブは、訪問者が変えたシグナル（書きかけの入力など）を保ちます。ページは読み込み直す直前に、最上位のシグナルのうち、サーバーが最後に書いた値（ページに書き込んだ値か差分で送った値）と異なるものをタブの `sessionStorage` に保存します。新しいページはそれを新しいサーバーの値の上に戻し、ほかのシグナル（サーバーが差分で書き換えた一覧など）には新しいサーバーの値を使います。戻すのは、新しいページのパスとクエリが同じで、シグナル構造体のフィールド名と型も同じときだけです。フィールド名と型は、デバッグビルドで `#[derive(Signals)]` がページに書き込みます。入れ子の構造体の中だけが変わった場合は検出されません。条件に合わないときは、ページはサーバーの値から始まります。この処理は独立したスクリプトにあり、デバッグビルドだけが配信するので、リリースビルドのランタイムには含まれません。",
            },
        ),
        para(
            site,
            L {
                en: "The same command can also start a project. `cargo hypersig new counter --host server` writes a directory `counter` with a counter like the one above and a Dockerfile, in place of `cargo new` and the two files written by hand. `--host cloudflare` and `--host vercel` write a project for those hosts, which [Choosing a host](learn/deploying/) compares.",
                ja: "このコマンドでプロジェクトを作ることもできます。`cargo hypersig new counter --host server` は、上と同じようなカウンターと Dockerfile を入れたディレクトリ `counter` を書き出すので、`cargo new` と、手で書いた 2 つのファイルの代わりになります。`--host cloudflare` と `--host vercel` はそれぞれのホスト向けのプロジェクトを書き出します。ホストの違いは [デプロイ先の選び方](learn/deploying/) で比べています。",
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
                    en: "`count.set(count + 1)` is a statement, a change to the signals that runs in the browser. `on_click` puts it into the page, and the runtime runs it on each click without a request. [Expressions and statements](learn/expressions/) covers what a statement can do.",
                    ja: "`count.set(count + 1)` は文（ブラウザで実行する、シグナルへの変更）です。`on_click` がこの文をページに書き込み、ランタイムはクリックのたびにリクエストを送らずに実行します。文で書けることは [式と文](learn/expressions/) で説明します。",
                },
                L {
                    en: "`span().bind_text(count.to_text())` keeps the text equal to the count. `Page::render` evaluates it with the initial values, so the HTML already shows `0`.",
                    ja: "`span().bind_text(count.to_text())` はテキストをカウントの値に合わせ続けます。`Page::render` が初期値でこの式を評価するので、HTML の時点で `0` が表示されています。",
                },
                L {
                    en: "`INDEX` is a `Route`, the page at `/`. `routes!` declares it on one line with its handler `index`. The first line, `fn pages(Router<()>);`, names the function the macro writes. That function registers every route of the block on a router, and `Router<()>` is a router without state. `main` calls `pages(Router::new())`, and a link to the page is `INDEX.link()`. [Pages and links](learn/pages/) covers routes with parameters.",
                    ja: "`INDEX` は `/` のページを表す `Route` です。`routes!` は、このルートをハンドラ `index` と同じ行で宣言します。1 行目の `fn pages(Router<()>);` はマクロが生成する関数の名前で、この関数がブロック内のルートをすべてルーターに登録します。`Router<()>` は状態を持たないルーターです。`main` は `pages(Router::new())` を呼び出し、このページへのリンクは `INDEX.link()` で作ります。パラメータを持つルートは [ページとリンク](learn/pages/) で説明します。",
                },
                L {
                    en: "`RESET` is an `Action<Counter>`. The page calls it with `RESET.call()` and the router serves it with `.action(RESET, reset)`, so both sides share its path, its method, its argument type, and the page it belongs to.",
                    ja: "`RESET` は `Action<Counter>` です。ページは `RESET.call()` で呼び出し、ルーターは `.action(RESET, reset)` で配信するので、パス、メソッド、引数の型、属するページを両側で共有します。",
                },
                L {
                    en: "A page handler and an action handler both take two arguments: the router's state, then what the request carries, which is the route's parameters for a page and the call's arguments for an action. This program has no state, the route `/` has no parameters, and `RESET.call()` sends no arguments, so each of them is `()`, and `(): ()` is an argument that receives `()`. [Talking to the server](learn/server/) adds state and arguments.",
                    ja: "ページのハンドラもアクションのハンドラも、引数を 2 つ取ります。1 つ目はルーターの状態、2 つ目はリクエストが運ぶ値で、ページならルートのパラメータ、アクションなら呼び出しの引数が 2 つ目に入ります。このプログラムには状態がなく、ルート `/` にはパラメータがなく、`RESET.call()` は引数を送らないので、どれも `()` です。`(): ()` は `()` を受け取る引数の書き方です。状態と引数を使う例は [サーバーとのやり取り](learn/server/) にあります。",
                },
                L {
                    en: "The handler returns `Counter::count().patch(0)`, a patch: a new value for one signal. The server sends it in the response, and the runtime writes the value into the signal, which rewrites the text.",
                    ja: "ハンドラは差分 `Counter::count().patch(0)`（シグナル 1 つの新しい値）を返します。サーバーはこれを応答で送り、ランタイムが値をシグナルに書き込むと、テキストが書き換わります。",
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
                en: "The counter below runs the browser side of the program on this page. Its code uses two crates the program above does not: the buttons come from `hypersig_ui`, the component crate in the same repository, and the layout from `hypersig::style`, which comes with hypersig. To run the code of a demo on this site in your own project, add `hypersig_ui = \"0.5\"` under `[dependencies]`, as [Installation](components/installation/) shows. [Styling](learn/styling/) covers `hypersig::style`.",
                ja: "下のカウンターはこのプログラムのブラウザ側をこのページで動かしています。このコードは上のプログラムにないものを 2 つ使っています。ボタンは同じリポジトリにあるコンポーネントのクレート `hypersig_ui` のもので、レイアウトは hypersig に含まれる `hypersig::style` で書いています。このサイトのデモのコードを自分のプロジェクトで動かすときは、[インストール](components/installation/) のとおり `[dependencies]` に `hypersig_ui = \"0.5\"` を加えてください。`hypersig::style` は [スタイル](learn/styling/) で説明します。",
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
                en: "Next, [Why hypersig](learn/why/) shows the HTML this program writes and compares hypersig with other frameworks. The guide itself starts at [Building the page](learn/describing-the-page/), which shows how the HTML is built.",
                ja: "次の [hypersig を選ぶ理由](learn/why/) では、このプログラムが書き出す HTML を示し、ほかのフレームワークと比べます。ガイドの本編は HTML の組み立て方を説明する [ページを組み立てる](learn/describing-the-page/) から始まります。",
            },
        ),
    ])
}

pub fn describing_the_page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DescribingThePage).title(site)),
        lead(site, Doc::Learn(Learn::DescribingThePage).leads()),
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
                en: "The [`html`](api:hypersig/html/index.html) module has one function per common tag, such as `div()`, `p()`, and `button()`, and `el(hypersig::tag!(\"tag\"))` for any other, which checks the tag at compile time and refuses `script`, `style`, and `base`. Each returns an `El<S>`, where `S` is the page's signal struct, and its methods return the element, so a tree is one expression.",
                ja: "[`html`](api:hypersig/html/index.html) モジュールには `div()`、`p()`、`button()` のようによく使うタグごとの関数と、それ以外のタグに使う `el(hypersig::tag!(\"tag\"))` があります。`tag!` はタグをコンパイル時に検査し、`script`、`style`、`base` を拒否します。どれも `El<S>` を返します。`S` はページのシグナル構造体です。`El` のメソッドは要素そのものを返すので、木全体を 1 つの式で書けます。",
            },
        ),
        show(Snippet::Elements),
        para(
            site,
            L {
                en: "`div::<Profile>()` names the page's signal struct because nothing else in this code tells the compiler which page the element belongs to. An element passed to `Page::new(...).child(...)`, or to a function that takes an `El<Profile>`, gets its `S` from there, which is why the Quick start writes `div()`. The children of an element take its `S` the same way, so only the outermost element of a tree written on its own needs the `::<Profile>`.",
                ja: "`div::<Profile>()` とページのシグナル構造体を書いているのは、このコードにはこの要素がどのページのものかをコンパイラに伝えるものがほかにないからです。`Page::new(...).child(...)` や `El<Profile>` を受け取る関数に渡す要素は、渡し先から `S` が推論されます。クイックスタートが `div()` と書けるのはそのためです。子要素も親から同じように `S` を受け取るので、単独で書いた木でも `::<Profile>` が要るのはいちばん外側の要素だけです。",
            },
        ),
        para(
            site,
            L {
                en: "`text` appends static text, and `child` appends an element or a string. `href` points a link at a page from a `Route`, which [Pages and links](learn/pages/) covers. `attr` sets a plain attribute, such as `.attr(attr::PLACEHOLDER, \"Name\")`, and takes the name as an [`Attr`](api:hypersig/html/struct.Attr.html). The module [`html::attr`](api:hypersig/html/attr/index.html) holds one for each attribute of HTML, SVG, and ARIA, so a misspelled name stops the build. A name outside those lists, such as `data-lat` or an attribute of a custom element, is written `hypersig::attr!(\"data-lat\")`, which checks it at compile time. Neither has an `Attr` for these names, ignoring case, so `attr`, `bind_attr`, and `attr_when` cannot write them:",
                ja: "`text` は固定のテキストを、`child` は要素か文字列を末尾に追加します。`href` は `Route` からページへのリンクを張ります。これは [ページとリンク](learn/pages/) で説明します。`attr` は `.attr(attr::PLACEHOLDER, \"Name\")` のように普通の属性を設定し、名前を [`Attr`](api:hypersig/html/struct.Attr.html) で受け取ります。HTML、SVG、ARIA の属性はモジュール [`html::attr`](api:hypersig/html/attr/index.html) に 1 つずつ定数があるので、名前を書き間違えるとビルドが止まります。`data-lat` やカスタム要素の属性のように一覧にない名前は `hypersig::attr!(\"data-lat\")` と書き、コンパイル時に検査されます。次の名前にはどちらの方法でも `Attr` を作れないので、`attr`、`bind_attr`、`attr_when` で書けません。大文字と小文字は区別しません。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`id` and `class`, which `El::id` and `El::class` set. `El::id` takes an [`Id`](api:hypersig/html/struct.Id.html), which you declare once, such as `const EMAIL: Id = Id::new(\"email\")`.",
                    ja: "`id` と `class`。`El::id` と `El::class` で設定します。`El::id` は、`const EMAIL: Id = Id::new(\"email\")` のように 1 度だけ宣言した [`Id`](api:hypersig/html/struct.Id.html) を受け取ります。",
                },
                L {
                    en: "An attribute whose value names another element by its id, such as `for`, `aria-describedby`, `aria-controls`, and `commandfor`. Methods such as `label_for(&EMAIL)` and `aria_describedby(&[&HINT])` set them from the same `Id` the element carries, so a reference cannot be spelled apart from its target.",
                    ja: "`for`、`aria-describedby`、`aria-controls`、`commandfor` のように、値がほかの要素を id で指す属性。`label_for(&EMAIL)` や `aria_describedby(&[&HINT])` などのメソッドが、指す先の要素と同じ `Id` から設定するので、参照と参照先の綴りが食い違うことはありません。",
                },
                L {
                    en: "`role`, which `El::role` sets from a [`Role`](api:hypersig/html/enum.Role.html), so a misspelled role stops the build too.",
                    ja: "`role`。`El::role` が [`Role`](api:hypersig/html/enum.Role.html) から設定するので、ロールの書き間違いもビルドで止まります。",
                },
                L {
                    en: "A name starting with `data-hs-`, which only the typed methods write, because the runtime reads those attributes.",
                    ja: "`data-hs-` で始まる名前。ランタイムがこの属性を読むので、型付きのメソッドだけが書きます。",
                },
                L {
                    en: "`href` and `src`, which `El::href` and `El::src` set from a `Link` or a [`SafeUrl`](api:hypersig/struct.SafeUrl.html), a URL whose scheme cannot run a script. [When bindings and statements are not enough](learn/escape-hatches/) covers `SafeUrl`.",
                    ja: "`href` と `src`。`El::href` と `El::src` が、`Link` か、スクリプトを実行できないスキームの URL を持つ [`SafeUrl`](api:hypersig/struct.SafeUrl.html) から設定します。`SafeUrl` は [束縛と文で足りないとき](learn/escape-hatches/) で説明します。",
                },
                L {
                    en: "A name starting with `on`, such as `onclick`. The `on_*` methods, such as `on_click`, attach a statement in its place.",
                    ja: "`onclick` のように `on` で始まる名前。代わりに `on_click` などの `on_*` のメソッドで文を付けます。",
                },
                L {
                    en: "`srcdoc`, `action`, `formaction`, `xlink:href`, `data`, and `http-equiv`, which have no method of their own. A form sends its values through an action call, as [Forms](learn/forms/) shows.",
                    ja: "`srcdoc`、`action`、`formaction`、`xlink:href`、`data`、`http-equiv`。これらを設定するメソッドはありません。フォームの値は [フォーム](learn/forms/) のとおりアクションの呼び出しで送ります。",
                },
                L {
                    en: "`attributeName`, whose value `href` would make an SVG animation write into a link.",
                    ja: "`attributeName`。値が `href` だと、SVG のアニメーションがリンクに書き込むことになるからです。",
                },
                L {
                    en: "A name holding a character other than a letter, a digit, `-`, `_`, `.`, and `:`.",
                    ja: "英字、数字、`-`、`_`、`.`、`:` 以外の文字を含む名前。",
                },
            ],
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
                    en: "`show` hides the element while a condition is false. `when(cond, el)` removes it instead, so each time it returns its inputs start empty and its `on_load`, the statement that runs when an element enters the document, runs again.",
                    ja: "`show` は条件が偽の間、要素を隠します。`when(cond, el)` は要素を取り除くので、要素が戻るたびに入力欄は空から始まり、`on_load`（要素が文書に入ったときに実行する文）がもう一度実行されます。",
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
                ja: "`hypersig_ui` はこうした関数を集めたクレートで、見た目は shadcn/ui に倣っています。このサイトのデモはそのボタンとテキスト欄を使っています。",
            },
        ),
    ])
}
