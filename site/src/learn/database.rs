//! The page on where a handler keeps the data requests share: memory, a
//! database it opens with a driver, a database the host binds, and the
//! settings of the hosted databases it is most often one of.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, subheading, table_of, title};

pub fn database(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Database).title(site)),
        lead(site, Doc::Learn(Learn::Database).leads()),
        places(site),
        drivers(site),
        hosted(site),
        processes(site),
        workers(site),
        checks(site),
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
                en: "hypersig has no storage of its own. An action's handler is a Rust function, it reads and writes the data in whatever way its code says, and hypersig sends the `Page` or `Patches` it returns to the browser. Which store fits depends on how long the data has to last and where the server runs.",
                ja: "hypersig 自体にはデータを保存する仕組みがありません。アクションのハンドラは Rust の関数で、データの読み書きはそのコードに書いたとおりに行われます。hypersig はハンドラが返す `Page` や `Patches` をブラウザに送るだけです。どこに置くかは、データをどれだけ残す必要があるかと、サーバーをどこで動かすかで決まります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Store", "Lasts", "Example"],
                ja: &["置き場所", "残る期間", "例"],
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
                        "再起動しても残り、同じデータベースを使うどのプロセスも読めます",
                        "`database`（SQLite）、`postgres`",
                    ],
                    &[
                        "ホストが Worker に結び付けるデータベース（D1、KV、Durable Object）",
                        "再起動しても残り、どの isolate も読めます",
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

/// The connection in the router's state, and blocking against `async`
/// drivers.
fn drivers(site: Site) -> Part {
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
                en: "Each page and action handler receives the router's state as its first argument. Put the connection, or a pool of connections, there, and have each handler read and write through it. The pages, actions, and signals stay as they were, because a handler still returns the same `Page` and `Patches`. A handler that returns `Result` passes a failed query on with `?`, and the error's `IntoResponse` is the answer, as [Requests, cookies, and errors](learn/requests/) describes. The runtime writes the body of a failed action to the signal given to `Call::error`, so write a message the visitor can act on there, and log the database's own message on the server, since it can hold the query and the connection string.",
                ja: "ページとアクションのハンドラは、最初の引数でルーターの状態を受け取ります。そこに接続か接続のプールを置き、ハンドラはそれを通して読み書きします。ハンドラが返す `Page` と `Patches` は変わらないので、ページ、アクション、シグナルはそのまま使えます。`Result` を返すハンドラは失敗したクエリを `?` でそのまま返し、応答はエラーの `IntoResponse` が決めます（[リクエスト、Cookie、エラー](learn/requests/) を参照してください）。失敗したアクションの本文はランタイムが `Call::error` に渡したシグナルに書くので、本文には訪問者が次にどうすればよいかを書いてください。データベースが返したメッセージにはクエリや接続文字列が含まれることがあるので、サーバーのログに出します。",
            },
        ),
        show(Snippet::Database),
        para(
            site,
            L {
                en: "This sample uses [rusqlite](https://docs.rs/rusqlite/0.39), whose calls block the thread until SQLite answers. A blocked thread is one of tokio's workers that serve every request, so each query runs in `tokio::task::spawn_blocking`, on threads kept for blocking work. sqlx, which [`examples/postgres`](https://github.com/imaimai17468/hypersig/tree/main/examples/postgres) uses, has `async` functions, so a handler awaits the query and needs no `spawn_blocking`. Its pool, `PgPool`, goes in the state as the SQLite connection does here. For MySQL, turn on sqlx's `mysql` feature in place of `postgres`, use a `MySqlPool` and a `mysql://` URL, and write `?` in place of `$1` in the queries.",
                ja: "このサンプルは [rusqlite](https://docs.rs/rusqlite/0.39) を使います。rusqlite の呼び出しは、SQLite が答えるまでスレッドを止めます。止まるのはすべてのリクエストを処理する tokio のワーカーなので、クエリは `tokio::task::spawn_blocking` で、スレッドを止める処理のために用意された別のスレッドで動かします。[`examples/postgres`](https://github.com/imaimai17468/hypersig/tree/main/examples/postgres) が使う sqlx の関数は `async` なので、ハンドラはクエリを `.await` するだけで済み、`spawn_blocking` は要りません。sqlx のプール `PgPool` は、ここでの SQLite の接続と同じように状態に置きます。MySQL では、sqlx の feature を `postgres` の代わりに `mysql` にし、`MySqlPool` と `mysql://` の URL を使って、クエリの `$1` を `?` に書き換えます。",
            },
        ),
        subheading(
            site,
            L {
                en: "A database without SQL",
                ja: "SQL を使わないデータベース",
            },
        ),
        para(
            site,
            L {
                en: "A database with an SDK in place of SQL takes the same shape: the client goes in the router's state and the handler awaits its calls. The sample below reads DynamoDB with `aws-sdk-dynamodb`, AWS's own crate. For Firestore, Google has published no crate for reading and writing documents as of October 2026, only `google-cloud-firestore-admin-v1` for the admin API, and the community's [`firestore`](https://docs.rs/firestore) crate is the one in common use.",
                ja: "SQL の代わりに SDK で使うデータベースも形は同じで、クライアントをルーターの状態に置き、ハンドラはその呼び出しを `.await` します。下のサンプルは AWS 公式のクレート `aws-sdk-dynamodb` で DynamoDB を読みます。Firestore については、2026 年 10 月の時点で Google はドキュメントを読み書きするクレートを公開しておらず、公開しているのは管理用 API の `google-cloud-firestore-admin-v1` だけです。よく使われているのはコミュニティの [`firestore`](https://docs.rs/firestore) クレートです。",
            },
        ),
        show(Snippet::DynamoDb),
    ])
}

/// The settings that hosted PostgreSQL and MySQL services ask of sqlx.
fn hosted(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Hosted PostgreSQL and MySQL",
                ja: "ホスティングされた PostgreSQL と MySQL",
            },
        ),
        para(
            site,
            L {
                en: "Supabase, PlanetScale, Cloud SQL, AlloyDB, RDS, and Aurora run PostgreSQL or MySQL, so `examples/postgres` connects to any of them once `DATABASE_URL` holds the service's connection string. They differ in the settings the table lists, as their documentation gave them in October 2026. sqlx built with its `tls-rustls` feature checks a server's certificate against the Mozilla list of root certificates, and reads `sslrootcert` as the path of a certificate file.",
                ja: "Supabase、PlanetScale、Cloud SQL、AlloyDB、RDS、Aurora は PostgreSQL か MySQL を動かすので、`DATABASE_URL` にサービスの接続文字列を入れれば `examples/postgres` はどれにもつながります。違いは表に挙げた設定で、内容は 2026 年 10 月の各サービスのドキュメントによります。`tls-rustls` の feature でビルドした sqlx は、サーバーの証明書を Mozilla のルート証明書の一覧で検証し、`sslrootcert` は証明書のファイルのパスとして読みます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Service", "Connection", "TLS", "Note"],
                ja: &["サービス", "接続", "TLS", "注意"],
            },
            L {
                en: &[
                    &[
                        "Supabase (PostgreSQL)",
                        "`postgresql://postgres:PASSWORD@db.REF.supabase.co:5432/postgres`, over IPv6 unless the project has the IPv4 add-on. The session pooler takes user `postgres.REF` on port 5432",
                        "Off by default. `sslmode=verify-full` with `sslrootcert` naming the CA file from the database settings",
                        "The transaction pooler on port 6543 does not support prepared statements, and sqlx prepares every query by default, so use the direct connection or the session pooler",
                    ],
                    &[
                        "PlanetScale (PostgreSQL)",
                        "Port 5432 reaches PostgreSQL, and 6432 reaches PgBouncer in transaction mode",
                        "Required: `sslmode=verify-full`",
                        "PlanetScale's own string adds `sslrootcert=system`, a word sqlx would read as a file name, so leave it out",
                    ],
                    &[
                        "PlanetScale (Vitess, MySQL)",
                        "A `mysql://` URL with sqlx's `mysql` feature",
                        "Clients without full TLS support do not connect",
                        "Foreign keys are a setting of the database, and work only while it is unsharded",
                    ],
                    &[
                        "Cloud SQL, AlloyDB (from Cloud Run)",
                        "The Unix socket under `/cloudsql/INSTANCE_CONNECTION_NAME`, as `postgres://USER:PASSWORD@/DB?host=/cloudsql/INSTANCE_CONNECTION_NAME`. sqlx adds the `.s.PGSQL.5432` that PostgreSQL's socket name ends in",
                        "The socket is encrypted without a setting",
                        "Add the instance to the Cloud Run service's Cloud SQL connections",
                    ],
                    &[
                        "RDS, Aurora",
                        "A password, or an IAM token from `aws-sdk-rds`'s `auth_token::AuthTokenGenerator`",
                        "`sslmode=verify-full` with the AWS CA file",
                        "A token lasts 15 minutes, so each new connection of the pool needs a fresh one",
                    ],
                ],
                ja: &[
                    &[
                        "Supabase（PostgreSQL）",
                        "`postgresql://postgres:PASSWORD@db.REF.supabase.co:5432/postgres`。IPv4 のアドオンがないプロジェクトでは IPv6 だけでつながります。セッションプーラーはユーザー `postgres.REF`、ポート 5432 です",
                        "既定では無効です。`sslmode=verify-full` にし、`sslrootcert` にデータベースの設定画面から入手した CA のファイルを指定します",
                        "ポート 6543 のトランザクションプーラーは prepared statement に対応せず、sqlx は既定ですべてのクエリを prepare するので、直接の接続かセッションプーラーを使ってください",
                    ],
                    &[
                        "PlanetScale（PostgreSQL）",
                        "ポート 5432 は PostgreSQL に、6432 はトランザクションモードの PgBouncer につながります",
                        "必須です。`sslmode=verify-full`",
                        "PlanetScale が示す接続文字列には `sslrootcert=system` が付いていますが、sqlx はこれをファイル名として読むので外してください",
                    ],
                    &[
                        "PlanetScale（Vitess、MySQL）",
                        "sqlx の `mysql` の feature と `mysql://` の URL",
                        "TLS に完全には対応しないクライアントはつながりません",
                        "外部キーはデータベースごとの設定で、シャーディングしていないあいだだけ使えます",
                    ],
                    &[
                        "Cloud SQL、AlloyDB（Cloud Run から）",
                        "`/cloudsql/INSTANCE_CONNECTION_NAME` の Unix ソケットに、`postgres://USER:PASSWORD@/DB?host=/cloudsql/INSTANCE_CONNECTION_NAME` でつなぎます。PostgreSQL のソケット名の末尾の `.s.PGSQL.5432` は sqlx が付けます",
                        "ソケットは設定なしで暗号化されます",
                        "Cloud Run のサービスの Cloud SQL 接続にインスタンスを追加してください",
                    ],
                    &[
                        "RDS、Aurora",
                        "パスワードか、`aws-sdk-rds` の `auth_token::AuthTokenGenerator` で作る IAM のトークン",
                        "`sslmode=verify-full` と AWS の CA のファイル",
                        "トークンの有効期間は 15 分なので、プールが新しく開く接続ごとに新しいトークンが要ります",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Vercel runs one process per instance of a function and adds instances as requests grow, and each process opens its own pool, so the connections to the database multiply with the instances. Keep `max_connections` small there, and connect through the service's pooler where it has one that supports prepared statements. Vercel's own helper for this, `attachDatabasePool`, is a Node.js API.",
                ja: "Vercel は関数のインスタンスごとに 1 つのプロセスを動かし、リクエストが増えるとインスタンスを増やします。プロセスはそれぞれプールを開くので、データベースへの接続はインスタンスの数だけ増えます。Vercel では `max_connections` を小さくし、サービスに prepared statement を使えるプーラーがあればそれを経由してください。このための Vercel のヘルパー `attachDatabasePool` は Node.js の API です。",
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
                en: "A stream that sends the list after each change hears of the change through a `tokio::sync::broadcast` channel in the router's state, and that channel belongs to one process. In `examples/database`, a second process on the same SQLite file reads the same todos, but a write by one process sends nothing to the other process's streams, so a tab served by the other process shows the write when it loads the page again. The migration of `examples/postgres` creates a trigger that calls `pg_notify` after each statement that changes the table, and PostgreSQL delivers the notification when the transaction commits. Each process listens on that channel with sqlx's `PgListener` and forwards what it hears to its own `broadcast`, so every open tab shows the write at once.",
                ja: "変更のたびに一覧を送るストリームは、ルーターの状態にある `tokio::sync::broadcast` のチャネルで変更を知ります。このチャネルは 1 つのプロセスのものです。`examples/database` では、同じ SQLite のファイルを開いた 2 つ目のプロセスも同じ todo を読みますが、一方のプロセスの書き込みはもう一方のプロセスのストリームには届きません。もう一方のプロセスが配信したタブには、ページを読み込み直したときに書き込みが表示されます。`examples/postgres` のマイグレーションは、テーブルを変更する文のたびに `pg_notify` を呼ぶトリガーを作ります。PostgreSQL はその知らせをトランザクションのコミット時に届けます。各プロセスは sqlx の `PgListener` でそのチャネルを聞き、受け取った知らせを自分の `broadcast` に流すので、開いているすべてのタブに書き込みがすぐ表示されます。",
            },
        ),
        note(
            site,
            L {
                en: "The end-to-end tests of both examples start a second process on the same database. For `database` they check that each process serves what the other wrote, and for `postgres` that an open tab of one process shows a write through the other without a reload. The `postgres` test runs PostgreSQL in Docker.",
                ja: "どちらの例の E2E テストも、同じデータベースを使う 2 つ目のプロセスを起動します。`database` では一方のプロセスが書いた todo をもう一方が表示することを、`postgres` では一方のプロセスで開いたタブに、もう一方を通した書き込みが読み込み直さずに表示されることを確かめます。`postgres` のテストは Docker で PostgreSQL を動かします。",
            },
        ),
    ])
}

/// What each host lets the server keep, and Cloudflare's bindings.
fn workers(site: Site) -> Part {
    parts([
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
                en: "SQLite keeps the database in a file, so the file has to sit on a disk that outlives the process. In [a container](learn/deploying/container/), mount a volume and keep the processes that open the file on one machine, because SQLite's WAL mode shares memory between them. A Vercel Function writes only to a `/tmp` of its own instance, so its data goes to a database the function reaches over the network.",
                ja: "SQLite はデータベースを 1 つのファイルに置くので、そのファイルはプロセスが終わっても残るディスクに置く必要があります。[コンテナ](learn/deploying/container/) ではボリュームをマウントし、ファイルを開くプロセスを 1 台のマシンに収めてください。SQLite の WAL モードは、ファイルを開くプロセスのあいだでメモリを共有します。Vercel Functions が書き込めるのはインスタンスごとの `/tmp` だけなので、データは関数がネットワーク越しにつなぐデータベースに置きます。",
            },
        ),
        para(
            site,
            L {
                en: "A Cloudflare Worker is compiled to wasm, where tokio has no networking, and sqlx opens its connections with tokio's `TcpStream`, so a worker reaches its data through the bindings of `wrangler.toml`. The handlers take them from the `Env` that `fetch` receives: the [Cloudflare template](learn/deploying/cloudflare/) puts `env.d1(\"DB\")` in its `AppState` and saves its count in D1, and KV and Durable Objects come from `env.kv` and `env.durable_object` in the same way. For PostgreSQL, such as Supabase, Cloudflare's Hyperdrive gives the worker a connection string, and Cloudflare's [tokio-postgres example](https://github.com/cloudflare/workers-rs/tree/main/examples/tokio-postgres) connects to it over a `worker::Socket`. Cloudflare publishes no Rust example for MySQL over Hyperdrive.",
                ja: "Cloudflare Workers の Worker は wasm にコンパイルされ、wasm では tokio のネットワーク機能が使えません。sqlx は tokio の `TcpStream` で接続を開くので、Worker は `wrangler.toml` のバインディングを通してデータを使います。ハンドラはそれを `fetch` が受け取る `Env` から取り出します。[Cloudflare のテンプレート](learn/deploying/cloudflare/) は `env.d1(\"DB\")` を `AppState` に入れ、数を D1 に保存します。KV と Durable Object も同じように `env.kv` と `env.durable_object` で取り出せます。Supabase などの PostgreSQL には、Cloudflare の Hyperdrive が Worker に接続文字列を渡し、Cloudflare の [tokio-postgres の例](https://github.com/cloudflare/workers-rs/tree/main/examples/tokio-postgres) は `worker::Socket` でそれにつなぎます。Hyperdrive 経由の MySQL について、Cloudflare は Rust の例を公開していません。",
            },
        ),
    ])
}

/// The SQL that sqlx's macros check when the program is built.
fn checks(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "SQL the compiler checks",
                ja: "コンパイラが検査する SQL",
            },
        ),
        para(
            site,
            L {
                en: "The routes, actions, and signals the page uses are Rust types. SQL is a string that hypersig does not read, but sqlx's `query!` and `query_as!` macros check it when the program is built: each asks the database to prepare its query against the tables the migrations made, so a misspelled column, a parameter of the wrong type, or a struct field the result does not fill fails to compile. `query_file!` and `query_file_as!` read the query from a `.sql` file, so the SQL stays in files of its own, and `examples/postgres` keeps each query under `queries/` this way. A proc macro cannot tell cargo which files it read, so the example's `build.rs` names `queries/`, `migrations/`, and `.sqlx/` with `cargo::rerun-if-changed`, and an edited query is checked at the next build. The queries of `examples/database` and of the Cloudflare template are plain strings, and a mistake in them fails when the query runs.",
                ja: "ページが使うルート、アクション、シグナルは Rust の型です。SQL は hypersig が読まない文字列ですが、sqlx の `query!` と `query_as!` のマクロはビルドのときに SQL を検査します。マクロはマイグレーションが作ったテーブルに対してクエリの prepare をデータベースに頼むので、列名の綴りの誤り、型の違う引数、結果で埋まらない構造体のフィールドはコンパイルエラーになります。`query_file!` と `query_file_as!` はクエリを `.sql` ファイルから読むので、SQL を独立したファイルに置けます。`examples/postgres` はクエリをこの方法で `queries/` に置いています。proc macro は読んだファイルを cargo に伝えられないので、例の `build.rs` が `cargo::rerun-if-changed` で `queries/`、`migrations/`、`.sqlx/` を指定し、書き換えたクエリが次のビルドで検査されるようにしています。`examples/database` と Cloudflare のテンプレートのクエリはただの文字列なので、誤りはクエリを実行したときに失敗します。",
            },
        ),
        para(
            site,
            L {
                en: "A build reads what the macros need from the crate's `.sqlx` directory, which `cargo sqlx prepare` writes from a database with the migrations applied, so the build itself needs no database. The repository's `.cargo/config.toml` sets `SQLX_OFFLINE=true`, so a `DATABASE_URL` set for the server does not send the build to a database. A query that `.sqlx` does not hold fails to compile with a message to run `cargo sqlx prepare`. After changing a query or a migration, write `.sqlx` again and commit it. CI runs `cargo sqlx prepare --check`, which fails when it was not written again.",
                ja: "ビルドはマクロが使う情報をクレートの `.sqlx` ディレクトリから読みます。`.sqlx` は、マイグレーションを適用したデータベースから `cargo sqlx prepare` が書き出すので、ビルド自体にデータベースは要りません。リポジトリの `.cargo/config.toml` は `SQLX_OFFLINE=true` を設定しているので、サーバーのために設定した `DATABASE_URL` があっても、ビルドはデータベースに問い合わせません。`.sqlx` にないクエリは、`cargo sqlx prepare` を実行するよう促すメッセージとともにコンパイルエラーになります。クエリかマイグレーションを変えたら `.sqlx` を書き出し直してコミットしてください。CI は `cargo sqlx prepare --check` を実行し、書き出し直していなければ失敗します。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo install --locked sqlx-cli --no-default-features --features postgres,rustls
export DATABASE_URL=postgres://postgres:password@localhost/postgres
cargo sqlx migrate run --source examples/postgres/migrations
cd examples/postgres && cargo sqlx prepare",
        ),
    ])
}
