//! The quick start: a first program, run with `cargo run`, and what each of
//! its parts does.

use super::super::{Program, program};
use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, note, para, parts, title};

pub fn page(site: Site) -> Part {
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
hypersig = "0.13"
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
                en: "While you edit, run the server with `cargo hypersig dev` in place of `cargo run`. It builds with [blaze](https://github.com/indexable-inc/blaze), an incremental build tool for Rust. Install both first:",
                ja: "コードを書き換えながら動かすときは、`cargo run` の代わりに `cargo hypersig dev` でサーバーを起動します。`cargo hypersig dev` は Rust の差分ビルドツールの [blaze](https://github.com/indexable-inc/blaze) でビルドします。先に両方をインストールしてください。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo install --locked hypersig_cli\ncurl -fsSL https://raw.githubusercontent.com/indexable-inc/blaze/main/install.sh | sh\ncargo hypersig dev",
        ),
        para(
            site,
            L {
                en: "`cargo hypersig dev` starts the server as `cargo run` does, and builds and starts it again each time a file it is built from changes. In a debug build, each open page keeps a stream to the server, so the tab reloads from the new server on its own. After a change, blaze compiles only the functions that changed, and `cargo check` runs beside it to decide whether the code compiles, so a compile error stops the restart and leaves the last server running. blaze's first build on a machine compiles every dependency and takes longer than Cargo's, so Cargo builds the first server, and each change until that build ends. The state the server holds in memory starts over at each restart.",
                ja: "`cargo hypersig dev` は `cargo run` と同じようにサーバーを起動し、ビルド元のファイルが変わるたびにビルドして起動し直します。デバッグビルドでは開いているページがサーバーへのストリームを保つので、タブは新しいサーバーから自動で読み込み直されます。変更のあと、blaze は変わった関数だけをコンパイルし、同時に走る `cargo check` が、コードをコンパイルできるかどうかを判定します。コンパイルエラーがあれば起動し直さず、前のサーバーが動き続けます。そのマシンで初めての blaze のビルドは依存クレートをすべてコンパイルするので、Cargo より時間がかかります。そのため、最初のサーバーと、そのビルドが終わるまでの変更は Cargo がビルドします。サーバーがメモリに持つ状態は、起動し直すたびに初期化されます。",
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
                    en: "`RESET` is an `Action<Counter>`, declared in the same block on one line with its handler `reset`, so `pages` serves it too. The page calls it with `RESET.call()`, so both sides share its path, its method, its argument type, and the page it belongs to.",
                    ja: "`RESET` は `Action<Counter>` で、同じブロックにハンドラ `reset` と同じ行で宣言するので、`pages` がこれも配信します。ページは `RESET.call()` で呼び出すので、パス、メソッド、引数の型、属するページを両側で共有します。",
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
                en: "The counter below runs the browser side of the program on this page. Its code uses two crates the program above does not: the buttons come from `hypersig_ui`, the component crate in the same repository, and the layout from `hypersig::style`, which comes with hypersig. To run the code of a demo on this site in your own project, add `hypersig_ui = \"0.13\"` under `[dependencies]`, as [Installation](components/installation/) shows. [Styling](learn/styling/) covers `hypersig::style`.",
                ja: "下のカウンターはこのプログラムのブラウザ側をこのページで動かしています。このコードは上のプログラムにないものを 2 つ使っています。ボタンは同じリポジトリにあるコンポーネントのクレート `hypersig_ui` のもので、レイアウトは hypersig に含まれる `hypersig::style` で書いています。このサイトのデモのコードを自分のプロジェクトで動かすときは、[インストール](components/installation/) のとおり `[dependencies]` に `hypersig_ui = \"0.13\"` を加えてください。`hypersig::style` は [スタイル](learn/styling/) で説明します。",
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
