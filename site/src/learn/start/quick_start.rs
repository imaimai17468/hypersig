//! The quick start: a first program, run with `cargo run`, and what each of
//! its parts does.

use super::super::snippet::{Snippet, show};
use super::super::{Program, program};
use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

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
hypersig = "0.14"
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
                en: "While you edit the code, run `cargo hypersig dev` in place of `cargo run`. It builds and starts the server again after each change, and the open tab reloads with the signals the visitor changed. [The development server](learn/dev-server/) covers installing it, and `cargo hypersig new`, which writes a project like this one.",
                ja: "コードを書き換えながら動かすときは、`cargo run` の代わりに `cargo hypersig dev` を使います。変更のたびにサーバーをビルドして起動し直し、開いているタブは訪問者が変えたシグナルを保ったまま読み込み直されます。インストールの方法と、このようなプロジェクトを書き出す `cargo hypersig new` は [開発サーバー](learn/dev-server/) で説明します。",
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
                    en: "`#[derive(Signals)]` on `Counter` adds `Counter::count()`, a [`Sig<Counter, i32>`](api:hypersig/struct.Sig.html) that names the field. A misspelled field is a compile error.",
                    ja: "`Counter` に付けた `#[derive(Signals)]` はフィールドを指す [`Sig<Counter, i32>`](api:hypersig/struct.Sig.html) を返す `Counter::count()` を追加します。フィールド名を打ち間違えるとコンパイルエラーになります。",
                },
                L {
                    en: "`count.set(count + 1)` is a statement, a [`Stmt<Counter>`](api:hypersig/struct.Stmt.html) that changes the signals in the browser. [`on_click`](api:hypersig/html/struct.El.html#method.on_click) puts it into the page, and the runtime runs it on each click without a request. [Statements](learn/statements/) covers what a statement can do.",
                    ja: "`count.set(count + 1)` は文で、ブラウザでシグナルを変える [`Stmt<Counter>`](api:hypersig/struct.Stmt.html) です。[`on_click`](api:hypersig/html/struct.El.html#method.on_click) がこの文をページに書き込み、ランタイムはクリックのたびにリクエストを送らずに実行します。文で書けることは [文](learn/statements/) で説明します。",
                },
                L {
                    en: "`span().bind_text(count.to_text())` keeps the text equal to the count. [`Page::render`](api:hypersig/struct.Page.html#method.render) evaluates it with the initial values, so the HTML already shows `0`.",
                    ja: "`span().bind_text(count.to_text())` はテキストをカウントの値に合わせ続けます。[`Page::render`](api:hypersig/struct.Page.html#method.render) が初期値でこの式を評価するので、HTML の時点で `0` が表示されています。",
                },
                L {
                    en: "`INDEX` is a [`Route`](api:hypersig/struct.Route.html), the page at `/`. [`routes!`](api:hypersig/macro.routes.html) declares it on one line with its handler `index`. The first line, `fn pages(Router<()>);`, names the function the macro writes. That function registers every route of the block on a router, and `Router<()>` is a router without state. `main` calls `pages(Router::new())`, and a link to the page is `INDEX.link()`. [Routes](learn/routes/) covers routes with parameters.",
                    ja: "`INDEX` は `/` のページを表す [`Route`](api:hypersig/struct.Route.html) です。[`routes!`](api:hypersig/macro.routes.html) は、このルートをハンドラ `index` と同じ行で宣言します。1 行目の `fn pages(Router<()>);` はマクロが生成する関数の名前で、この関数がブロック内のルートをすべてルーターに登録します。`Router<()>` は状態を持たないルーターです。`main` は `pages(Router::new())` を呼び出し、このページへのリンクは `INDEX.link()` で作ります。パラメータを持つルートは [ルート](learn/routes/) で説明します。",
                },
                L {
                    en: "`RESET` is an [`Action<Counter>`](api:hypersig/struct.Action.html), declared in the same block on one line with its handler `reset`, so `pages` serves it too. The page calls it with `RESET.call()`, so both sides share its path, its method, its argument type, and the page it belongs to.",
                    ja: "`RESET` は [`Action<Counter>`](api:hypersig/struct.Action.html) で、同じブロックにハンドラ `reset` と同じ行で宣言するので、`pages` がこれも配信します。ページは `RESET.call()` で呼び出すので、パス、メソッド、引数の型、属するページを両側で共有します。",
                },
                L {
                    en: "A page handler and an action handler both take two arguments: the router's state, then what the request carries, which is the route's parameters for a page and the call's arguments for an action. This program has no state, the route `/` has no parameters, and `RESET.call()` sends no arguments, so each of them is `()`, and `(): ()` is an argument that receives `()`. [Actions](learn/actions/) adds state and arguments.",
                    ja: "ページのハンドラもアクションのハンドラも、引数を 2 つ取ります。1 つ目はルーターの状態、2 つ目はリクエストが運ぶ値で、ページならルートのパラメータ、アクションなら呼び出しの引数が 2 つ目に入ります。このプログラムには状態がなく、ルート `/` にはパラメータがなく、`RESET.call()` は引数を送らないので、どれも `()` です。`(): ()` は `()` を受け取る引数の書き方です。状態と引数を使う例は [アクション](learn/actions/) にあります。",
                },
                L {
                    en: "The handler returns `Counter::count().patch(0)`, a patch: a new value for one signal, which converts into [`Patches<Counter>`](api:hypersig/struct.Patches.html). The server sends it in the response, and the runtime writes the value into the signal, which rewrites the text.",
                    ja: "ハンドラは差分 `Counter::count().patch(0)`（シグナル 1 つの新しい値）を [`Patches<Counter>`](api:hypersig/struct.Patches.html) に変換して返します。サーバーはこれを応答で送り、ランタイムが値をシグナルに書き込むと、テキストが書き換わります。",
                },
                L {
                    en: "[`hypersig_runtime()`](api:hypersig/trait.RouterExt.html#tymethod.hypersig_runtime) adds the route that serves the runtime's file at [`RUNTIME_PATH`](api:hypersig/constant.RUNTIME_PATH.html), the path each page's `<script>` loads.",
                    ja: "[`hypersig_runtime()`](api:hypersig/trait.RouterExt.html#tymethod.hypersig_runtime) は、ランタイムのファイルを [`RUNTIME_PATH`](api:hypersig/constant.RUNTIME_PATH.html) で配信するルートを追加します。各ページの `<script>` はこのパスから読み込みます。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "A mistake the compiler stops",
                ja: "コンパイラが止めるミス",
            },
        ),
        para(
            site,
            L {
                en: "`Counter::count()` exists because `Counter` has a field `count`. A misspelled name is a method that does not exist, so `cargo run` stops before the server starts:",
                ja: "`Counter::count()` があるのは、`Counter` に `count` というフィールドがあるからです。名前を打ち間違えると存在しないメソッドを呼ぶことになるので、`cargo run` はサーバーを起動する前に止まります。",
            },
        ),
        show(Snippet::WrongField),
        para(
            site,
            L {
                en: "Setting `count` to a value of another type, such as `count.set(\"zero\")`, stops the build the same way. [Why hypersig](learn/why/) lists each string in the page and the type it comes from.",
                ja: "`count.set(\"zero\")` のように型の違う値を代入しても、同じようにビルドが止まります。ページの中の文字列と、それぞれを作る型の一覧は [hypersig を選ぶ理由](learn/why/) にあります。",
            },
        ),
        heading(
            site,
            L {
                en: "Mistakes that still compile",
                ja: "コンパイルが通ってしまうミス",
            },
        ),
        table_of(
            site,
            L {
                en: &["Mistake", "What happens"],
                ja: &["ミス", "起きること"],
            },
            L {
                en: &[
                    &[
                        "`main` leaves out `.hypersig_runtime()`",
                        "The page shows `0`, but the request for `RUNTIME_PATH` answers 404, so the buttons do nothing",
                    ],
                    &[
                        "`main` serves `Router::new()` without passing it to `pages`",
                        "`/` and `/reset` answer 404",
                    ],
                ],
                ja: &[
                    &[
                        "`main` で `.hypersig_runtime()` を書き忘れる",
                        "ページには `0` が表示されますが、`RUNTIME_PATH` へのリクエストが 404 になるので、ボタンを押しても何も起きません",
                    ],
                    &[
                        "`main` が `Router::new()` を `pages` に渡さずに配信する",
                        "`/` と `/reset` が 404 を返します",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "[What the compiler does not check](learn/why/#what-the-compiler-does-not-check) lists the others.",
                ja: "ほかのものは [コンパイラが検査しないこと](learn/why/#what-the-compiler-does-not-check) にまとめています。",
            },
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
                en: "The counter below runs the browser side of the program on this page. Its code uses two crates the program above does not: the buttons come from `hypersig_ui`, the component crate in the same repository, and the layout from `hypersig::style`, which comes with hypersig. To run the code of a demo on this site in your own project, add `hypersig_ui = \"0.14\"` under `[dependencies]`, as [Installation](components/installation/) shows. [Styling](learn/styling/) covers `hypersig::style`.",
                ja: "下のカウンターはこのプログラムのブラウザ側をこのページで動かしています。このコードは上のプログラムにないものを 2 つ使っています。ボタンは同じリポジトリにあるコンポーネントのクレート `hypersig_ui` のもので、レイアウトは hypersig に含まれる `hypersig::style` で書いています。このサイトのデモのコードを自分のプロジェクトで動かすときは、[インストール](components/installation/) のとおり `[dependencies]` に `hypersig_ui = \"0.14\"` を加えてください。`hypersig::style` は [スタイル](learn/styling/) で説明します。",
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
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "[Why hypersig](learn/why/) shows the HTML this program writes and compares hypersig with other frameworks.",
                    ja: "[hypersig を選ぶ理由](learn/why/) では、このプログラムが書き出す HTML を示し、ほかのフレームワークと比べます。",
                },
                L {
                    en: "[The development server](learn/dev-server/) runs the program again after each change.",
                    ja: "[開発サーバー](learn/dev-server/) は変更のたびにこのプログラムを動かし直します。",
                },
                L {
                    en: "[Elements and attributes](learn/elements/) starts the guide, with how the HTML is built.",
                    ja: "ガイドの本編は HTML の組み立て方を説明する [要素と属性](learn/elements/) から始まります。",
                },
            ],
        ),
    ])
}
