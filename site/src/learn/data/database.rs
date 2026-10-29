//! The page on where a handler keeps the data requests share, the
//! connection in the router's state, what a failed query answers, and how
//! a change reaches the streams of every server process.

use super::next;
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Database).title(site)),
        lead(site, Doc::Learn(Learn::Database).leads()),
        places(site),
        state(site),
        failures(site),
        sqlx(site),
        processes(site),
        next(
            site,
            &[
                L {
                    en: "[A database without SQL](learn/database-without-sql/) keeps the client of DynamoDB in the state the same way.",
                    ja: "[SQL を使わないデータベース](learn/database-without-sql/) では DynamoDB のクライアントを同じように状態に置きます。",
                },
                L {
                    en: "[SQL the compiler checks](learn/checked-sql/) turns a misspelled column into a compile error.",
                    ja: "[コンパイラが検査する SQL](learn/checked-sql/) では列名の綴りの誤りをコンパイルエラーにします。",
                },
                L {
                    en: "[Hosted PostgreSQL and MySQL](learn/hosted-databases/) lists the connection settings of Supabase, PlanetScale, Cloud SQL, and RDS.",
                    ja: "[ホスティングされた PostgreSQL と MySQL](learn/hosted-databases/) には、Supabase、PlanetScale、Cloud SQL、RDS の接続の設定をまとめています。",
                },
                L {
                    en: "[The database each host can reach](learn/database-hosts/) covers SQLite in a container and the bindings of a Cloudflare Worker.",
                    ja: "[デプロイ先ごとのデータベース](learn/database-hosts/) ではコンテナでの SQLite と Cloudflare Workers のバインディングを説明します。",
                },
            ],
        ),
    ])
}

/// The four places data can live, and the example of each.
fn places(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Where the data lives",
                ja: "データの置き場所",
            },
        ),
        para(
            site,
            L {
                en: "hypersig has no storage of its own. An action's handler is a Rust function that reads and writes the data however its code says, and hypersig sends the `Page` or `Patches` it returns to the browser. Which store fits depends on how long the data has to last and where the server runs.",
                ja: "hypersig 自体にはデータを保存する仕組みがありません。アクションのハンドラは Rust の関数で、データの読み書きはそのコードに書いたとおりに行われます。hypersig はハンドラが返す `Page` や `Patches` をブラウザに送るだけです。どこに置くかはデータをどれだけ保持する必要があるかと、サーバーをどこで動かすかで決まります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Store", "Lasts", "Example"],
                ja: &["置き場所", "保持される期間", "例"],
            },
            L {
                en: &[
                    &[
                        "The router's state in memory, such as an `Arc<Mutex<..>>`",
                        "Until the process stops, and only in that process",
                        "`todo` and the other examples",
                    ],
                    &[
                        "A database the server opens with a driver: SQLite, PostgreSQL, MySQL, DynamoDB",
                        "Past a restart, and every process on the database reads it",
                        "`database` (SQLite), `postgres`",
                    ],
                    &[
                        "A database the host binds to the worker: D1, KV, a Durable Object",
                        "Past a restart, and every isolate reads it",
                        "The Cloudflare template",
                    ],
                    &[
                        "Another service, called over its HTTP API",
                        "As long as the service keeps it",
                        "`api`, which [Calling another service's API](learn/api/) covers",
                    ],
                ],
                ja: &[
                    &[
                        "メモリにあるルーターの状態（`Arc<Mutex<..>>` など）",
                        "プロセスが止まるまで。そのプロセスの中だけ",
                        "`todo` などの例",
                    ],
                    &[
                        "サーバーがドライバで開くデータベース（SQLite、PostgreSQL、MySQL、DynamoDB）",
                        "再起動しても保持され、同じデータベースを使うどのプロセスも読めます",
                        "`database`（SQLite）、`postgres`",
                    ],
                    &[
                        "ホストが Worker に結び付けるデータベース（D1、KV、Durable Object）",
                        "再起動しても保持され、どの isolate も読めます",
                        "Cloudflare のテンプレート",
                    ],
                    &[
                        "HTTP の API で呼ぶ別のサービス",
                        "そのサービスが保持するあいだ",
                        "`api`（[ほかのサービスの API を呼ぶ](learn/api/) で説明します）",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The examples live under [`examples/`](https://github.com/imaimai17468/hypersig/tree/main/examples) in the repository, and each runs with `cargo run -p example-<name>`.",
                ja: "例はリポジトリの [`examples/`](https://github.com/imaimai17468/hypersig/tree/main/examples) にあり、どれも `cargo run -p example-<名前>` で動かせます。",
            },
        ),
    ])
}

/// The connection in the router's state, with SQLite as the example.
fn state(site: Site) -> Part {
    parts([
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
                en: "The first line of `routes!`, `fn pages(Router<AppState>);`, names the router's state type, and each page and action handler receives a clone of that state as its first argument. A handler is a [`Handler`](api:hypersig/trait.Handler.html): `Fn(St, A) -> O`, or `Fn(St, E, A) -> O` with an extractor `E` between the state and the arguments. Put the connection, or a pool of connections, in the state, and have each handler read and write through it. The pages, actions, and signals stay as they were, because a handler still returns the same `Page` and `Patches`.",
                ja: "`routes!` の最初の行 `fn pages(Router<AppState>);` がルーターの状態の型を決め、ページとアクションのハンドラは、その状態の複製を最初の引数で受け取ります。ハンドラは [`Handler`](api:hypersig/trait.Handler.html) で、形は `Fn(St, A) -> O` か、状態と引数のあいだにエクストラクタ `E` をはさんだ `Fn(St, E, A) -> O` です。状態に接続か接続のプールを置き、ハンドラはそれを通して読み書きします。ハンドラが返す `Page` と `Patches` は変わらないので、ページ、アクション、シグナルはそのまま使えます。",
            },
        ),
        para(
            site,
            L {
                en: "The sample below needs rusqlite, whose `bundled` feature compiles SQLite into the program, so the server does not depend on a SQLite installed on the machine:",
                ja: "下のサンプルには rusqlite が必要です。`bundled` の feature は SQLite をプログラムに組み込むので、サーバーはマシンにインストールされた SQLite に頼りません。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
rusqlite = { version = "0.39", features = ["bundled"] }"#,
        ),
        show(Snippet::Database),
        para(
            site,
            L {
                en: "axum clones the state for each request, so it is `Clone`, and a rusqlite `Connection` is `Send` but not `Sync`, so the state holds it in an `Arc<Mutex<..>>`. [rusqlite](https://docs.rs/rusqlite/0.39)'s calls block the thread until SQLite answers. The thread it blocks would be one of the tokio workers that serve every request, so each query runs in `tokio::task::spawn_blocking`, on threads kept for blocking work.",
                ja: "axum はリクエストごとに状態を複製するので、状態は `Clone` です。rusqlite の `Connection` は `Send` ですが `Sync` ではないので、状態は `Arc<Mutex<..>>` に入れて持ちます。[rusqlite](https://docs.rs/rusqlite/0.39) の呼び出しは、SQLite が答えるまでスレッドを止めます。止まるのはすべてのリクエストを処理する tokio のワーカーなので、クエリは `tokio::task::spawn_blocking` で、スレッドを止める処理のために用意された別のスレッドで動かします。",
            },
        ),
        subheading(
            site,
            L {
                en: "A handler of another state",
                ja: "状態の型が違うハンドラ",
            },
        ),
        para(
            site,
            L {
                en: "The state a handler takes has to be the router's. A handler of `AppState` named in a `Router<()>` fails to compile with E0277. The message names the function's type and says it `cannot serve this page or action`, and a note says that the state is the router's:",
                ja: "ハンドラが受け取る状態は、ルーターの状態と同じ型である必要があります。`AppState` を受け取るハンドラを `Router<()>` に書くと E0277 でコンパイルに失敗します。メッセージは関数の型を示して `cannot serve this page or action` と伝え、注記が状態はルーターのものであることを示します。",
            },
        ),
        show(Snippet::WrongState),
    ])
}

/// What the browser receives when a query fails.
fn failures(site: Site) -> Part {
    parts([
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
                en: "A page handler may return `Result<Page<S>, E>`, and an action handler `Result<Patches<S>, E>`, where `E` implements axum's `IntoResponse`. The handler passes a failed query on with `?`, and the error's `into_response` is the answer. The sample's `DbError` logs the database's message on the server, because that message can hold the query and the connection string, and answers with a sentence for the visitor:",
                ja: "ページのハンドラは `Result<Page<S>, E>` を、アクションのハンドラは `Result<Patches<S>, E>` を返せます。`E` は axum の `IntoResponse` を実装する型です。ハンドラは失敗したクエリを `?` でそのまま返し、エラーの `into_response` が応答になります。データベースが返したメッセージにはクエリや接続文字列が含まれることがあるので、サンプルの `DbError` はそのメッセージをサーバーのログに出し、訪問者には次の文で応答します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "HTTP/1.1 500 Internal Server Error
content-type: text/plain; charset=utf-8

The list could not be read. Try again.",
        ),
        para(
            site,
            L {
                en: "A browser that loads the page shows that text in place of the page. When an action fails, the runtime writes the same `text/plain` body to the signal given to [`Call::error`](api:hypersig/struct.Call.html#method.error), so write a sentence the visitor can act on. [Errors and logging](learn/errors/) covers the handler's errors in general.",
                ja: "ページを読み込んだブラウザは、ページの代わりにこの文を表示します。アクションが失敗したときは、ランタイムが同じ `text/plain` の本文を [`Call::error`](api:hypersig/struct.Call.html#method.error) に渡したシグナルに書きます。本文には、訪問者が次にどうすればよいかを書いてください。ハンドラのエラー全般は [エラーとログ](learn/errors/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "A `Result` is a page's answer only when its error implements `IntoResponse`. Without that impl the line of `routes!` fails to compile with E0277, which says that the trait bound `DbError: IntoResponse` is not satisfied:",
                ja: "`Result` がページの応答になるのは、エラーが `IntoResponse` を実装しているときだけです。実装がなければ、`routes!` のその行が E0277 でコンパイルに失敗し、トレイト境界 `DbError: IntoResponse` が満たされないと報告されます。",
            },
        ),
        show(Snippet::ErrorWithoutResponse),
    ])
}

/// sqlx, whose calls are `async`, for PostgreSQL and MySQL.
fn sqlx(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A driver with async calls",
                ja: "async で呼ぶドライバ",
            },
        ),
        para(
            site,
            L {
                en: "sqlx, which [`examples/postgres`](https://github.com/imaimai17468/hypersig/tree/main/examples/postgres) uses, has `async` functions, so a handler awaits the query and needs no `spawn_blocking`, and the worker thread serves other requests while it waits. Its pool, `PgPool`, is `Clone` and shares its connections between the clones, so it goes in the state without an `Arc`:",
                ja: "[`examples/postgres`](https://github.com/imaimai17468/hypersig/tree/main/examples/postgres) が使う sqlx の関数は `async` なので、ハンドラはクエリを `.await` するだけで済み、`spawn_blocking` は不要です。待っているあいだ、ワーカーのスレッドはほかのリクエストを処理します。sqlx のプール `PgPool` は `Clone` で、複製どうしが接続を共有するので、`Arc` に入れずに状態に置きます。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
sqlx = { version = "0.9", default-features = false, features = ["postgres", "runtime-tokio", "tls-rustls"] }"#,
        ),
        show(Snippet::SqlxPool),
        para(
            site,
            L {
                en: "For MySQL, turn on sqlx's `mysql` feature in place of `postgres`, use a `MySqlPool` and a `mysql://` URL, and write `?` in place of `$1` in the queries. The query in this sample is a string the compiler does not read. [SQL the compiler checks](learn/checked-sql/) shows the macros that check it when the program is built.",
                ja: "MySQL では、sqlx の feature を `postgres` の代わりに `mysql` にし、`MySqlPool` と `mysql://` の URL を使って、クエリの `$1` を `?` に書き換えます。このサンプルのクエリは、コンパイラが読まない文字列です。ビルドのときにクエリを検査するマクロは [コンパイラが検査する SQL](learn/checked-sql/) で説明します。",
            },
        ),
    ])
}

/// Telling the other processes' streams about a change.
fn processes(site: Site) -> Part {
    parts([
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
                en: "A stream that sends the list after each change hears of the change through a `tokio::sync::broadcast` channel in the router's state, and that channel belongs to one process. In `examples/database`, a second process on the same SQLite file reads the same todos, but a write by one process sends nothing to the other process's streams, so a tab served by the other process shows the write when it loads the page again.",
                ja: "変更のたびに一覧を送るストリームは、ルーターの状態にある `tokio::sync::broadcast` のチャネルで変更を知ります。このチャネルは 1 つのプロセスのものです。`examples/database` では、同じ SQLite のファイルを開いた 2 つ目のプロセスも同じ todo を読みますが、一方のプロセスの書き込みはもう一方のプロセスのストリームには届きません。もう一方のプロセスが配信したタブには、ページを読み込み直したときに書き込みが表示されます。",
            },
        ),
        subheading(
            site,
            L {
                en: "PostgreSQL's LISTEN and NOTIFY",
                ja: "PostgreSQL の LISTEN と NOTIFY",
            },
        ),
        para(
            site,
            L {
                en: "The migration of `examples/postgres` creates a trigger that calls `pg_notify` after each statement that changes the table, and PostgreSQL delivers the notification when the transaction commits:",
                ja: "`examples/postgres` のマイグレーションはテーブルを変更する文のたびに `pg_notify` を呼ぶトリガーを作ります。PostgreSQL はその知らせをトランザクションのコミット時に届けます。",
            },
        ),
        code::block(
            Lang::Html,
            Some("migrations/20261023000000_create_todos.sql"),
            "CREATE FUNCTION notify_todos_changed() RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('todos_changed', '');
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER todos_changed
    AFTER INSERT OR UPDATE OR DELETE ON todos
    FOR EACH STATEMENT EXECUTE FUNCTION notify_todos_changed();",
        ),
        para(
            site,
            L {
                en: "Each process listens on that channel with sqlx's `PgListener` and forwards what it hears to its own `broadcast`, so every open tab shows the write at once. A write by this process reaches its own streams the same way, through the trigger, so an action that writes needs to send nothing to `changed` itself:",
                ja: "各プロセスは sqlx の `PgListener` でそのチャネルを聞き、受け取った知らせを自分の `broadcast` に流すので、開いているすべてのタブに書き込みがすぐ表示されます。このプロセス自身の書き込みも同じようにトリガーを通して自分のストリームに届くので、書き込むアクションが自分で `changed` に送る必要はありません。",
            },
        ),
        show(Snippet::PgListen),
        para(
            site,
            L {
                en: "The page opens the stream with `on_load(UPDATES.call().reconnect())`, and the stream sends a patch that replaces `titles` once when it opens and again after each notification. A read that fails is logged and skipped, so the tab keeps the list it shows. A receiver of `broadcast` that falls behind the channel's 16 messages gets `RecvError::Lagged`, and the stream reads the list again then too. [Streams](learn/streams/) covers the stream itself.",
                ja: "ページは `on_load(UPDATES.call().reconnect())` でストリームを開きます。ストリームは開いたときに 1 回、そのあと知らせのたびに、`titles` を置き換える差分を送ります。読み込みに失敗したときはログに出して飛ばすので、タブは表示中の一覧を保ちます。`broadcast` の受信側がチャネルの 16 件より遅れると `RecvError::Lagged` を受け取り、このときもストリームは一覧を読み直します。ストリームそのものは [ストリーム](learn/streams/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "`PgListener` waits for notifications on the connection that sent `LISTEN`, so on PostgreSQL's side that connection has to stay with the listener too. A pooler in transaction mode hands its connection to PostgreSQL to another client after each transaction, and [PgBouncer's feature table](https://www.pgbouncer.org/features.html) lists `LISTEN` as never supported in that mode, so a listener opened through one hears nothing. Where the pool connects through such a pooler, as on Vercel, open the listener with `PgListener::connect` and a connection string that reaches the database directly or through the pooler's session mode.",
                ja: "`PgListener` は `LISTEN` を送った接続で知らせを待つので、その接続は PostgreSQL の側でもリスナーのものであり続ける必要があります。トランザクションモードのプーラーは、トランザクションのたびに PostgreSQL への接続を別のクライアントに渡します。[PgBouncer の機能表](https://www.pgbouncer.org/features.html) も、このモードの `LISTEN` を「Never」としています。そのため、このプーラーを経由して開いたリスナーには知らせが届きません。Vercel のようにプールがこうしたプーラーを経由するときは、データベースに直接つなぐか、プーラーのセッションモードを通る接続文字列を `PgListener::connect` に渡してリスナーを開いてください。",
            },
        ),
        subheading(
            site,
            L {
                en: "Redis's PUBLISH and SUBSCRIBE",
                ja: "Redis の PUBLISH と SUBSCRIBE",
            },
        ),
        para(
            site,
            L {
                en: "Without PostgreSQL, a service that delivers each message to every subscriber can take its place. `examples/redis` keeps the todos in Redis, and each write is a Lua script that changes the keys and runs `PUBLISH` on one channel. Redis runs the script with no other command in between, so no write reaches the keys without telling the other processes. In each process a task spawned in `main`, as `listen` is above, subscribes with `SUBSCRIBE` and sends on `changed` for each message. The streams are the same code in both examples. Redis keeps no message for a subscriber that is not connected, so after the subscription breaks, the task subscribes again and sends on `changed` once more, and the streams read the list again.",
                ja: "PostgreSQL を使わないときは、購読しているすべての相手にメッセージを届けるサービスで代わりができます。`examples/redis` は todo を Redis に置き、書き込みはどれも、キーを変更して 1 つのチャネルに `PUBLISH` する Lua スクリプトです。Redis はスクリプトの途中にほかのコマンドを挟まずに実行するので、ほかのプロセスに知らせずにキーを変える書き込みはありません。各プロセスでは、上の `listen` と同じように `main` で起動したタスクが `SUBSCRIBE` で購読し、メッセージを受け取るたびに `changed` に送ります。ストリームのコードは 2 つの例で同じです。Redis は接続していない購読者向けにメッセージを保存しないので、購読が切れるとタスクは購読し直し、もう一度 `changed` に送ります。ストリームはそこで一覧を読み直します。",
            },
        ),
        note(
            site,
            L {
                en: "The end-to-end tests of these examples start a second process on the same database. For `database` they check that each process serves what the other wrote, and for `postgres` and `redis` that an open tab of one process shows a write through the other without a reload. The `postgres` and `redis` tests run their database in Docker.",
                ja: "これらの例の E2E テストは同じデータベースを使う 2 つ目のプロセスを起動します。`database` では一方のプロセスが書いた todo をもう一方が表示することを、`postgres` と `redis` では一方のプロセスで開いたタブに、もう一方を通した書き込みが読み込み直さずに表示されることを確かめます。`postgres` と `redis` のテストはデータベースを Docker で動かします。",
            },
        ),
    ])
}
