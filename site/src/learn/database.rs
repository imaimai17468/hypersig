//! The page on keeping the state handlers share in a database: where the
//! connection lives, blocking drivers, errors, updates across processes, and
//! the database each host can reach.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, title};

pub fn database(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Database).title(site)),
        lead(site, Doc::Learn(Learn::Database).leads()),
        heading(
            site,
            L {
                en: "The connection in the router's state",
                ja: "ルーターの状態に置く接続",
            },
        ),
        para(
            site,
            L {
                en: "Each page and action handler receives the router's state as its first argument. The examples keep their todos there in an `Arc<Mutex<..>>`, which is gone when the process stops. Put the database connection in that state in its place, and have each handler read and write through it. The pages, actions, and signals stay as they were, because a handler still returns the same `Page` and `Patches`.",
                ja: "ページとアクションのハンドラは、最初の引数でルーターの状態を受け取ります。リポジトリの例は todo をそこに `Arc<Mutex<..>>` で置いているので、プロセスが止まると todo も消えます。代わりにデータベースの接続をその状態に置き、ハンドラはその接続を通して読み書きします。ハンドラが返す `Page` と `Patches` は変わらないので、ページ、アクション、シグナルはそのまま使えます。",
            },
        ),
        show(Snippet::Database),
        para(
            site,
            L {
                en: "This sample uses [rusqlite](https://docs.rs/rusqlite/0.40), whose calls block the thread until SQLite answers. A blocked thread is one of tokio's workers that serve every request, so each query runs in `tokio::task::spawn_blocking`, on threads kept for blocking work. A driver with `async` functions, such as sqlx or tokio-postgres, is awaited in the handler and needs no `spawn_blocking`.",
                ja: "このサンプルは [rusqlite](https://docs.rs/rusqlite/0.40) を使います。rusqlite の呼び出しは、SQLite が答えるまでスレッドを止めます。止まるのはすべてのリクエストを処理する tokio のワーカーなので、クエリは `tokio::task::spawn_blocking` で、スレッドを止める処理のために用意された別のスレッドで動かします。sqlx や tokio-postgres のように `async` の関数を持つドライバなら、ハンドラの中で `.await` するだけで済み、`spawn_blocking` は要りません。",
            },
        ),
        heading(
            site,
            L {
                en: "When a query fails",
                ja: "クエリが失敗したとき",
            },
        ),
        para(
            site,
            L {
                en: "A handler that returns `Result` passes a failed query on with `?`, and the error's `IntoResponse` is the answer, as [Requests, cookies, and errors](learn/requests/) describes. The runtime writes the body of a failed action to the signal given to `Call::error`, so the visitor reads that body. Write a message the visitor can act on there, and log the database's own message on the server, since it can hold the query and the file path.",
                ja: "`Result` を返すハンドラは、失敗したクエリを `?` でそのまま返せます。[リクエスト、Cookie、エラー](learn/requests/) のとおり、応答はエラーの `IntoResponse` が決めます。失敗したアクションの本文はランタイムが `Call::error` に渡したシグナルに書くので、訪問者はその本文を読みます。本文には訪問者が次にどうすればよいかを書き、データベースが返したメッセージはサーバーのログに出してください。そのメッセージにはクエリやファイルのパスが含まれることがあります。",
            },
        ),
        heading(
            site,
            L {
                en: "Updates with more than one process",
                ja: "プロセスが複数あるときの更新",
            },
        ),
        para(
            site,
            L {
                en: "A stream that sends the list after each change hears of the change through a `tokio::sync::broadcast` channel in the router's state, and that channel belongs to one process. Once the state is in a database, a second process reads the same todos, but a write by one process sends nothing to the other process's streams, so a tab served by the other process shows the write when it loads the page again. To send it at once, the database has to tell each process: PostgreSQL's `LISTEN` and `NOTIFY` do this, and each process forwards what it hears to its own channel.",
                ja: "変更のたびに一覧を送るストリームは、ルーターの状態にある `tokio::sync::broadcast` のチャネルで変更を知ります。このチャネルは 1 つのプロセスのものです。状態をデータベースに置けば 2 つ目のプロセスも同じ todo を読みますが、一方のプロセスの書き込みは、もう一方のプロセスのストリームには届きません。もう一方のプロセスが配信したタブには、ページを読み込み直したときに書き込みが表示されます。すぐに届けるには、データベースから各プロセスに知らせる必要があります。PostgreSQL では `LISTEN` と `NOTIFY` でこれができ、各プロセスは受け取った知らせを自分のチャネルに送ります。",
            },
        ),
        heading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査するもの",
            },
        ),
        para(
            site,
            L {
                en: "The routes, actions, and signals the page uses are still Rust types, so a mistake in them still fails to compile. The SQL is a string that hypersig does not read: SQLite reports a misspelled column when the query runs, and a handler returns that as its error. sqlx's `query!` macro checks a query against a database when the program is built, and needs that database, or the files `cargo sqlx prepare` writes from it, at build time.",
                ja: "ページが使うルート、アクション、シグナルは Rust の型のままなので、それらの書き間違いは引き続きコンパイルエラーになります。SQL は hypersig が読まない文字列です。列名の綴りを間違えると、クエリを実行したときに SQLite がエラーを返し、ハンドラはそれをエラーとして返します。sqlx の `query!` マクロはビルドのときにデータベースに問い合わせてクエリを検査します。そのためビルドには、そのデータベースか、`cargo sqlx prepare` がそこから書き出すファイルが必要です。",
            },
        ),
        heading(
            site,
            L {
                en: "The database each host can reach",
                ja: "デプロイ先ごとのデータベース",
            },
        ),
        para(
            site,
            L {
                en: "SQLite keeps the database in a file, so the file has to sit on a disk that outlives the process. In [a container](learn/deploying/container/), mount a volume and keep the processes that open the file on one machine, because SQLite's WAL mode shares memory between them. A Cloudflare Worker has no disk that keeps a file from one request to the next, so its state goes to KV, D1, or a Durable Object, as [Cloudflare Workers](learn/deploying/cloudflare/) lists. A Vercel Function writes only to a `/tmp` of its own instance, so its state goes to a database the function reaches over the network.",
                ja: "SQLite はデータベースを 1 つのファイルに置くので、そのファイルはプロセスが終わっても残るディスクに置く必要があります。[コンテナ](learn/deploying/container/) ではボリュームをマウントし、ファイルを開くプロセスを 1 台のマシンに収めてください。SQLite の WAL モードは、ファイルを開くプロセスのあいだでメモリを共有します。Cloudflare Workers にはリクエストをまたいでファイルを残すディスクがないので、[Cloudflare Workers](learn/deploying/cloudflare/) に挙げた KV、D1、Durable Object に状態を置きます。Vercel Functions が書き込めるのはインスタンスごとの `/tmp` だけなので、状態は関数がネットワーク越しにつなぐデータベースに置きます。",
            },
        ),
        note(
            site,
            L {
                en: "The [`database`](https://github.com/imaimai17468/hypersig/tree/main/examples/database) example in the repository keeps the todo list of the `todo` example in a SQLite file. Run it with `cargo run -p example-database`. Its end-to-end test starts a second process on the same file and checks that each process serves the todos the other wrote.",
                ja: "リポジトリの [`database`](https://github.com/imaimai17468/hypersig/tree/main/examples/database) の例は、`todo` の例の一覧を SQLite のファイルに置きます。`cargo run -p example-database` で動かせます。その E2E テストは同じファイルを開く 2 つ目のプロセスを起動し、一方のプロセスが書いた todo をもう一方が表示することを確かめます。",
            },
        ),
    ])
}
