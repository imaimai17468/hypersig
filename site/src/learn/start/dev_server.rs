//! The development server: `cargo hypersig dev`, the reload it causes in an
//! open tab, the signals the reload keeps, and `cargo hypersig new`.

use super::super::snippet::{Snippet, show};
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

/// The commands that install the CLI and blaze, as `cargo hypersig dev`
/// prints the second where blaze is missing.
const INSTALL: &str = "cargo install --locked hypersig_cli
curl -fsSL https://raw.githubusercontent.com/indexable-inc/blaze/main/install.sh | sh";

/// What `cargo hypersig dev` prints in a project named `counter`, after
/// Cargo's own lines, at the start and after a change.
const OUTPUT: &str =
    "Watching the `.rs` files, `Cargo.toml`, and `Cargo.lock` under /home/me/counter
Started `target/debug/counter`
Restarted `target/debug/counter`";

/// The attributes the quick start's page carries on `<body>` when a debug
/// build serves it, as the tests of `hypersig_runtime` assert them.
const DEV_BODY: &str = r#"<body data-hs-signals="..." data-hs-dev="/_hypersig/dev?since=..." data-hs-shape="count:i32">"#;

/// What the page of `Snippet::KeptSignals` leaves in `sessionStorage` under
/// `hypersig-dev` while it reloads, with the visitor's half-typed draft.
const KEPT: &str =
    r#"{"route":"/","shape":"draft:String,notes:Vec < String >","values":{"draft":"Buy mi"}}"#;

pub fn page(site: Site) -> Part {
    parts(
        intro(site)
            .into_iter()
            .chain(install(site))
            .chain(run(site))
            .chain(watched(site))
            .chain(builds(site))
            .chain(options(site))
            .chain(reload(site))
            .chain(kept(site))
            .chain(new_project(site))
            .chain(next(site)),
    )
}

fn intro(site: Site) -> [Part; 2] {
    [
        title(Doc::Learn(Learn::DevServer).title(site)),
        lead(site, Doc::Learn(Learn::DevServer).leads()),
    ]
}

fn install(site: Site) -> [Part; 4] {
    [
        heading(
            site,
            L {
                en: "Install it",
                ja: "インストールする",
            },
        ),
        para(
            site,
            L {
                en: "`cargo hypersig` comes from the crate `hypersig_cli`. Its `dev` command builds with [blaze](https://github.com/indexable-inc/blaze), an incremental build tool for Rust, so install both:",
                ja: "`cargo hypersig` はクレート `hypersig_cli` に入っています。その `dev` コマンドは Rust の差分ビルドツール [blaze](https://github.com/indexable-inc/blaze) でビルドするので、両方をインストールしてください。",
            },
        ),
        code::block(Lang::Shell, None, INSTALL),
        para(
            site,
            L {
                en: "Where no `blaze` command runs, `cargo hypersig dev` stops before it builds anything and prints the second line above.",
                ja: "`blaze` コマンドを実行できない環境では、`cargo hypersig dev` は何もビルドせずに止まり、上の 2 行目を表示します。",
            },
        ),
    ]
}

fn run(site: Site) -> [Part; 5] {
    [
        heading(
            site,
            L {
                en: "Run the server",
                ja: "サーバーを起動する",
            },
        ),
        para(
            site,
            L {
                en: "In the project's directory, run `cargo hypersig dev` where you would run `cargo run`. It builds the server, starts it, and builds and starts it again each time a file it is built from changes. Besides Cargo's own lines, it prints which files it watches and the binary it started:",
                ja: "プロジェクトのディレクトリで、`cargo run` の代わりに `cargo hypersig dev` を実行します。サーバーをビルドして起動し、ビルド元のファイルが変わるたびにビルドして起動し直します。Cargo が表示する行のほかに、監視するファイルと起動したバイナリを表示します。",
            },
        ),
        code::block(Lang::Shell, None, "cargo hypersig dev"),
        code::block(Lang::Html, None, OUTPUT),
        para(
            site,
            L {
                en: "The line names the binary and no address, because the server prints its own address where it prints one. The [Quick start](learn/)'s server prints none and listens on `http://127.0.0.1:3000`. The server writes to the same terminal, and one Ctrl+C stops both. The state the server holds in memory starts over at each restart.",
                ja: "この行が示すのはバイナリの名前だけです。アドレスは、表示するサーバーならサーバー自身が表示します。[クイックスタート](learn/) のサーバーは何も表示せず、`http://127.0.0.1:3000` で待ち受けます。サーバーは同じ端末に出力し、Ctrl+C を 1 回押すと両方が止まります。サーバーがメモリに持つ状態は、起動し直すたびに初期化されます。",
            },
        ),
    ]
}

fn watched(site: Site) -> [Part; 3] {
    [
        heading(
            site,
            L {
                en: "What starts a build",
                ja: "ビルドが始まるきっかけ",
            },
        ),
        para(
            site,
            L {
                en: "`cargo hypersig dev` reads the modification time of these files every 50 ms:",
                ja: "`cargo hypersig dev` は、次のファイルの更新時刻を 50 ms ごとに読みます。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Each `.rs` file, `Cargo.toml`, and `Cargo.lock` under the directory it runs in, except inside `target`, `node_modules`, and directories whose name starts with `.`.",
                    ja: "実行したディレクトリの下にある `.rs` ファイル、`Cargo.toml`、`Cargo.lock`。ただし `target`、`node_modules`、名前が `.` で始まるディレクトリの中は除きます。",
                },
                L {
                    en: "Each file the last build read, such as a stylesheet that `include_str!` puts into the binary, except the downloaded crates under Cargo's home directory. The `Watching` line counts them as \"more files the build read\".",
                    ja: "前回のビルドが読んだファイル。たとえば `include_str!` でバイナリに入れたスタイルシートです。Cargo のホームディレクトリにあるダウンロード済みのクレートは除きます。`Watching` の行ではこれらを「more files the build read」として数えます。",
                },
                L {
                    en: "A change waits 50 ms more for the rest of an editor's writes, then starts a build. A file saved while a build runs starts the next one.",
                    ja: "変更を見つけると、エディタの残りの書き込みを待つために 50 ms 待ってからビルドを始めます。ビルド中に保存したファイルは、次のビルドを始めます。",
                },
                L {
                    en: "A change that builds the same binary, such as an edit to a file the server is not built from, leaves the running server as it is.",
                    ja: "サーバーのビルド元でないファイルの編集のように、ビルドしてもバイナリが変わらない変更では、動いているサーバーをそのままにします。",
                },
            ],
        ),
    ]
}

fn builds(site: Site) -> [Part; 4] {
    [
        heading(
            site,
            L {
                en: "How a change is built",
                ja: "変更をビルドする方法",
            },
        ),
        para(
            site,
            L {
                en: "Each build runs `cargo check` and `blaze build` at the same time. blaze compiles only the functions that changed, so the wait after an edit is mostly the check's. The check decides whether the code compiles: when it prints an error, the server of the last build keeps running until a change that compiles. When the check passes and blaze fails, the line `blaze did not build it, so Cargo builds it` appears and `cargo build` writes the binary.",
                ja: "ビルドのたびに `cargo check` と `blaze build` を同時に実行します。blaze は変わった関数だけをコンパイルするので、編集のあとの待ち時間はほぼ `cargo check` の時間です。コードをコンパイルできるかどうかは `cargo check` で判定します。エラーが表示されたときは、コンパイルできる変更が来るまで前のビルドのサーバーが動き続けます。`cargo check` が通って blaze が失敗したときは、`blaze did not build it, so Cargo builds it` と表示され、`cargo build` がバイナリを書き出します。",
            },
        ),
        para(
            site,
            L {
                en: "blaze's first build on a machine compiles every dependency and takes longer than Cargo's. So Cargo builds the first server, and builds each change until blaze's first build ends.",
                ja: "そのマシンで初めての blaze のビルドは依存クレートをすべてコンパイルするので、Cargo より時間がかかります。そのため、最初のサーバーは Cargo がビルドし、blaze の最初のビルドが終わるまでの変更も Cargo がビルドします。",
            },
        ),
        para(
            site,
            L {
                en: "When the server exits on its own, for example after a panic in `main`, `The server stopped with ...; it starts again after the next change.` appears, and the next change starts it again even when the binary is the same.",
                ja: "`main` でパニックしたときのようにサーバーが自分で終了すると、`The server stopped with ...; it starts again after the next change.` と表示されます。次の変更では、バイナリが同じでもサーバーを起動し直します。",
            },
        ),
    ]
}

fn options(site: Site) -> [Part; 4] {
    [
        heading(
            site,
            L {
                en: "Build options and server arguments",
                ja: "ビルドのオプションとサーバーの引数",
            },
        ),
        para(
            site,
            L {
                en: "The arguments before `--` go to `cargo check` and `blaze build`, and the arguments after it go to the server:",
                ja: "`--` より前の引数は `cargo check` と `blaze build` に、後ろの引数はサーバーに渡します。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo hypersig dev [<build options>] [-- <args>]\ncargo hypersig dev -p web -- --verbose",
        ),
        table_of(
            site,
            L {
                en: &["Situation", "What `cargo hypersig dev` does"],
                ja: &["状況", "`cargo hypersig dev` の動き"],
            },
            L {
                en: &[
                    &[
                        "The build has more than one binary",
                        "Stops with `the build has ...; name one with --bin or -p`, which lists them",
                    ],
                    &[
                        "The build has no binary, as a Cloudflare project",
                        "Stops and says that a Cloudflare project runs under `npx wrangler dev`",
                    ],
                    &[
                        "The options include `--release`, `-r`, or `--profile`",
                        "Prints a warning that pages will not reload, because only a debug build serves the stream they reload from",
                    ],
                    &[
                        "The options change the target directory",
                        "Reads it once at the start, so a later change to it needs `cargo hypersig dev` started again",
                    ],
                ],
                ja: &[
                    &[
                        "ビルドにバイナリが 2 つ以上ある",
                        "`the build has ...; name one with --bin or -p` と、バイナリの名前を並べて止まります",
                    ],
                    &[
                        "Cloudflare のプロジェクトのように、ビルドにバイナリがない",
                        "Cloudflare のプロジェクトは `npx wrangler dev` で動かす、と表示して止まります",
                    ],
                    &[
                        "オプションに `--release`、`-r`、`--profile` がある",
                        "読み込み直しに使うストリームはデバッグビルドしか配信しないので、ページが読み込み直されないという警告を表示します",
                    ],
                    &[
                        "オプションでターゲットディレクトリを変える",
                        "ターゲットディレクトリは起動時に 1 回だけ読むので、あとで変えたときは `cargo hypersig dev` を起動し直してください",
                    ],
                ],
            },
        ),
    ]
}

fn reload(site: Site) -> [Part; 6] {
    [
        heading(
            site,
            L {
                en: "The open tab reloads",
                ja: "開いているタブが読み込み直される",
            },
        ),
        para(
            site,
            L {
                en: "In a debug build, [`hypersig_runtime()`](api:hypersig/trait.RouterExt.html#tymethod.hypersig_runtime) also serves a stream at `/_hypersig/dev`, and each page names it on `<body>`:",
                ja: "デバッグビルドでは、[`hypersig_runtime()`](api:hypersig/trait.RouterExt.html#tymethod.hypersig_runtime) が `/_hypersig/dev` でストリームも配信し、各ページは `<body>` にそのパスを書きます。",
            },
        ),
        code::block(Lang::Html, None, DEV_BODY),
        para(
            site,
            L {
                en: "The page loads `/_hypersig/dev.js` once, beside the runtime, and keeps a request to the `data-hs-dev` URL open. In the browser's network panel, the request closes after two seconds and a new one opens, so a server that shuts down gracefully waits for it no longer than that. `since` names the server process that rendered the page. When another process answers the same URL, as after a restart, it sends the event `hs-reload`, and the page reloads from it. Any restart of a debug build does this, including `cargo run` run again by hand.",
                ja: "ページはランタイムの隣にある `/_hypersig/dev.js` を 1 回読み込み、`data-hs-dev` の URL へのリクエストを開いたままにします。ブラウザのネットワークパネルでは、リクエストが 2 秒で閉じて新しいリクエストが開くのが見えます。そのため、グレースフルに終了するサーバーがこのリクエストを待つのは長くても 2 秒です。`since` はページを描画したサーバーのプロセスを表します。再起動のあとのように別のプロセスが同じ URL に応答すると、そのプロセスはイベント `hs-reload` を送り、ページはそのプロセスから読み込み直されます。デバッグビルドならどの再起動でもこうなり、手で `cargo run` を実行し直した場合も同じです。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "While no server answers, the page asks again every 50 ms, and every second after three seconds. Each refused request writes an error to the console.",
                    ja: "どのサーバーも応答しない間、ページは 50 ms ごとに問い合わせ直し、3 秒たつと 1 秒ごとになります。拒否されたリクエストはそのたびにコンソールにエラーを書きます。",
                },
                L {
                    en: "A page under a [`Base`](api:hypersig/struct.Base.html) reads the stream under its base, as `/app/_hypersig/dev`.",
                    ja: "[`Base`](api:hypersig/struct.Base.html) の下のページは `/app/_hypersig/dev` のように接頭辞の下のストリームを読みます。",
                },
                L {
                    en: "When the stream answers 404, as on a server that serves the runtime's file some other way, the page stops asking.",
                    ja: "ランタイムのファイルを別の方法で配信するサーバーのように、ストリームが 404 を返すと、ページは問い合わせをやめます。",
                },
            ],
        ),
        note(
            site,
            L {
                en: "A release build, and a build for wasm32-unknown-unknown such as a Cloudflare Worker, serves no stream, and its pages carry no `data-hs-dev` and open no request.",
                ja: "リリースビルドと、Cloudflare Workers のような wasm32-unknown-unknown 向けのビルドはストリームを配信しません。そのページには `data-hs-dev` がなく、リクエストも開きません。",
            },
        ),
    ]
}

fn kept(site: Site) -> [Part; 9] {
    [
        heading(
            site,
            L {
                en: "The signals the reload keeps",
                ja: "読み込み直しで保つシグナル",
            },
        ),
        para(
            site,
            L {
                en: "The reloaded tab keeps the signals the visitor changed, such as a half-typed draft. Take this page, whose input is bound to `draft` and whose list `notes` only the server changes:",
                ja: "読み込み直したタブは、書きかけの入力のように訪問者が変えたシグナルを保ちます。入力欄を `draft` に束縛し、一覧 `notes` はサーバーだけが変える次のページで考えます。",
            },
        ),
        show(Snippet::KeptSignals),
        para(
            site,
            L {
                en: "Just before the reload, the page saves in the tab's `sessionStorage`, under the key `hypersig-dev`, each top-level signal whose value differs from the server's last write of it, in the page or in a patch. After the visitor adds one note and types `Buy mi`, `notes` still holds what the server's patch wrote and `draft` does not, so only `draft` is saved:",
                ja: "ページは読み込み直す直前に、最上位のシグナルのうち、サーバーが最後に書いた値（ページに書き込んだ値か差分で送った値）と異なるものを、タブの `sessionStorage` にキー `hypersig-dev` で保存します。訪問者がメモを 1 つ追加してから `Buy mi` と入力した場合、`notes` はサーバーの差分が書いた値のままで、`draft` はそうではないので、保存されるのは `draft` だけです。",
            },
        ),
        code::block(Lang::JavaScript, None, KEPT),
        para(
            site,
            L {
                en: "The new page puts the saved values back over the new server's values, and every other signal takes the new server's value. Here the input shows `Buy mi` again, and the list is empty, because this server keeps no notes and its page starts with none. The entry is removed as it is read, so a later visit starts from the server's values.",
                ja: "新しいページは保存した値を新しいサーバーの値の上に戻し、ほかのシグナルには新しいサーバーの値を使います。この例では入力欄に `Buy mi` が戻り、一覧は空になります。このサーバーはメモを保存せず、ページは空の一覧から始まるからです。保存した値は読んだときに削除されるので、次に開いたときはサーバーの値から始まります。",
            },
        ),
        para(
            site,
            L {
                en: "The values go back only where the new page shows the same path and query, and its signal struct has the same [`Signals::SHAPE`](api:hypersig/trait.Signals.html#associatedconstant.SHAPE), the list of its field names and types. Otherwise the page starts from the server's values.",
                ja: "値を戻すのは、新しいページのパスとクエリが同じで、シグナル構造体の [`Signals::SHAPE`](api:hypersig/trait.Signals.html#associatedconstant.SHAPE)（フィールド名と型の一覧）も同じときだけです。条件に合わないときは、ページはサーバーの値から始まります。",
            },
        ),
        para(
            site,
            L {
                en: "These cases are not checked. In each, the reload restores less than the visitor typed, or a value shaped like the old struct:",
                ja: "次の場合は何も検査されず、戻る値が少なくなるか、古い構造体の値が戻ります。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A field whose type is another `Signals` struct adds only that type's name to `SHAPE`. A change inside the nested struct alone is not detected, and the value restored into that field keeps its old fields until the next reload.",
                    ja: "型が別の `Signals` 構造体であるフィールドは、`SHAPE` にその型の名前しか加えません。入れ子の構造体の中だけが変わった場合は検出されず、そのフィールドに戻した値は、次に読み込み直すまで古いフィールドのままです。",
                },
                L {
                    en: "A `Signals` impl written by hand leaves `SHAPE` empty, and its page restores nothing.",
                    ja: "手で書いた `Signals` の実装は `SHAPE` を空のままにするので、そのページは何も戻しません。",
                },
                L {
                    en: "Where `sessionStorage` is full or blocked, nothing is saved, and the reloaded page starts from the server's values.",
                    ja: "`sessionStorage` がいっぱいか使えない環境では何も保存されず、読み込み直したページはサーバーの値から始まります。",
                },
            ],
        ),
    ]
}

fn new_project(site: Site) -> [Part; 7] {
    [
        heading(
            site,
            L {
                en: "Start a project with cargo hypersig new",
                ja: "cargo hypersig new でプロジェクトを作る",
            },
        ),
        para(
            site,
            L {
                en: "`cargo hypersig new` writes a project from a template, in place of `cargo new` and the two files the [Quick start](learn/) writes by hand:",
                ja: "`cargo hypersig new` はテンプレートからプロジェクトを書き出します。`cargo new` と、[クイックスタート](learn/) で手で書いた 2 つのファイルの代わりになります。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo hypersig new counter\ncd counter\ncargo hypersig dev",
        ),
        para(
            site,
            L {
                en: "Without `--host`, the template is `server`: a counter like the quick start's, whose buttons come from `hypersig_ui` and whose `main` calls [`hypersig::serve`](api:hypersig/fn.serve.html) on port 3000, with two tests of its handlers, Clippy's pedantic lints on in `Cargo.toml`, and a Dockerfile. `--host cloudflare` and `--host vercel` write a project for those hosts, which [Choosing a host](learn/deploying/) compares.",
                ja: "`--host` を付けないときのテンプレートは `server` です。クイックスタートと同じようなカウンターで、ボタンは `hypersig_ui` のもの、`main` は [`hypersig::serve`](api:hypersig/fn.serve.html) をポート 3000 で呼び出します。ハンドラのテストが 2 つあり、`Cargo.toml` で Clippy の pedantic のリントを有効にし、Dockerfile も入っています。`--host cloudflare` と `--host vercel` はそれぞれのホスト向けのプロジェクトを書き出します。ホストの違いは [デプロイ先の選び方](learn/deploying/) で比べています。",
            },
        ),
        table_of(
            site,
            L {
                en: &["`--host`", "Files it writes", "Run it on this machine"],
                ja: &["`--host`", "書き出すファイル", "手元で動かすコマンド"],
            },
            L {
                en: &[
                    &[
                        "`server`",
                        "`Cargo.toml`, `src/main.rs`, `Dockerfile`, `.dockerignore`, `.gitignore`, `AGENTS.md`, `CLAUDE.md`",
                        "`cargo hypersig dev`",
                    ],
                    &[
                        "`cloudflare`",
                        "`Cargo.toml`, `src/lib.rs`, `wrangler.toml`, `rust-toolchain.toml`, `.gitignore`, `AGENTS.md`, `CLAUDE.md`",
                        "`npx wrangler dev`",
                    ],
                    &[
                        "`vercel`",
                        "`Cargo.toml`, `api/app.rs`, `vercel.json`, `rust-toolchain.toml`, `.vercelignore`, `.gitignore`, `AGENTS.md`, `CLAUDE.md`",
                        "`cargo run`",
                    ],
                ],
                ja: &[
                    &[
                        "`server`",
                        "`Cargo.toml`、`src/main.rs`、`Dockerfile`、`.dockerignore`、`.gitignore`、`AGENTS.md`、`CLAUDE.md`",
                        "`cargo hypersig dev`",
                    ],
                    &[
                        "`cloudflare`",
                        "`Cargo.toml`、`src/lib.rs`、`wrangler.toml`、`rust-toolchain.toml`、`.gitignore`、`AGENTS.md`、`CLAUDE.md`",
                        "`npx wrangler dev`",
                    ],
                    &[
                        "`vercel`",
                        "`Cargo.toml`、`api/app.rs`、`vercel.json`、`rust-toolchain.toml`、`.vercelignore`、`.gitignore`、`AGENTS.md`、`CLAUDE.md`",
                        "`cargo run`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "After writing the files, it prints the commands that run the project on this machine and deploy it.",
                ja: "ファイルを書き出したあと、プロジェクトを手元で動かすコマンドとデプロイするコマンドを表示します。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The last part of the path names the package. It takes lowercase ASCII letters, digits, `-`, and `_`, starting with a letter or `_`, and not a Rust keyword or a name Cargo refuses, such as `test` or `build`. A Cloudflare Worker's name holds no `_`. A refused name stops the command before it writes a file.",
                    ja: "パスの最後の部分がパッケージ名になります。使えるのは ASCII の小文字、数字、`-`、`_` で、先頭は文字か `_` です。Rust のキーワードや、`test`、`build` のように Cargo が拒否する名前は使えません。Cloudflare Workers の名前には `_` を使えません。使えない名前なら、ファイルを 1 つも書かずに止まります。",
                },
                L {
                    en: "A path that already exists stops the command, which then names `cargo hypersig add <host>`. `add` moves the project in the current directory to a host, and [Choosing a host](learn/deploying/) covers it.",
                    ja: "パスがすでにあれば止まり、`cargo hypersig add <host>` を案内します。`add` は今のディレクトリのプロジェクトをホスト向けに移すコマンドで、[デプロイ先の選び方](learn/deploying/) で説明します。",
                },
                L {
                    en: "The new directory becomes a Git repository, as with `cargo new`, unless it is inside one already.",
                    ja: "新しいディレクトリは、すでに Git リポジトリの中にある場合を除き、`cargo new` と同じように Git リポジトリになります。",
                },
            ],
        ),
    ]
}

fn next(site: Site) -> [Part; 2] {
    [
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
                    en: "[Elements and attributes](learn/elements/) starts the guide, with how the HTML is built.",
                    ja: "ガイドの本編は HTML の組み立て方を説明する [要素と属性](learn/elements/) から始まります。",
                },
                L {
                    en: "[Signals kept across page loads](learn/persisted-signals/) keeps chosen signals in the browser's storage on every page load, in any build.",
                    ja: "[ページをまたいで保つシグナル](learn/persisted-signals/) では、選んだシグナルを、どのビルドでも、ページを読み込むたびにブラウザのストレージに保ちます。",
                },
                L {
                    en: "[Choosing a host](learn/deploying/) builds the project for a host.",
                    ja: "[デプロイ先の選び方](learn/deploying/) ではプロジェクトをホスト向けにビルドします。",
                },
            ],
        ),
    ]
}
