//! The page on the SQL that sqlx's macros check when the program is built,
//! the build script that keeps the check current, and the `.sqlx`
//! directory that lets a machine without a database build.

use super::next;
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show_file};
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::CheckedSql).title(site)),
        lead(site, Doc::Learn(Learn::CheckedSql).leads()),
        macros(site),
        rebuild(site),
        offline(site),
        next(
            site,
            &[
                L {
                    en: "[Storing data](learn/database/) puts the pool these queries run on in the router's state.",
                    ja: "[データを保存する](learn/database/) ではこれらのクエリを実行するプールをルーターの状態に置きます。",
                },
                L {
                    en: "[Hosted PostgreSQL and MySQL](learn/hosted-databases/) lists the connection strings `DATABASE_URL` takes for each service.",
                    ja: "[ホスティングされた PostgreSQL と MySQL](learn/hosted-databases/) には、サービスごとに `DATABASE_URL` に入れる接続文字列をまとめています。",
                },
            ],
        ),
    ])
}

/// The macros and what they check.
fn macros(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The query macros",
                ja: "クエリのマクロ",
            },
        ),
        para(
            site,
            L {
                en: "The routes, actions, and signals the page uses are Rust types. SQL is a string that hypersig does not read, but sqlx's `query!` and `query_as!` macros check it when the program is built: each asks the database to prepare its query against the tables the migrations made, so a misspelled column, a parameter of the wrong type, or a struct field the result does not fill fails to compile. The macros need sqlx's `macros` feature, and `migrate` for `sqlx::migrate!`.",
                ja: "ページが使うルート、アクション、シグナルは Rust の型です。SQL は hypersig が読まない文字列ですが、sqlx の `query!` と `query_as!` のマクロはビルドのときに SQL を検査します。マクロはマイグレーションが作ったテーブルに対してクエリの prepare をデータベースに頼むので、列名の綴りの誤り、型の違う引数、結果で埋まらない構造体のフィールドはコンパイルエラーになります。マクロには sqlx の `macros` の feature が、`sqlx::migrate!` には `migrate` の feature が必要です。",
            },
        ),
        para(
            site,
            L {
                en: "`query_file!` and `query_file_as!` read the query from a `.sql` file, so the SQL stays in files of its own. [`examples/postgres`](https://github.com/imaimai17468/hypersig/tree/main/examples/postgres) keeps each query under `queries/` this way, and reads the list with `sqlx::query_file_as!(Todo, \"queries/list_todos.sql\")`, where `Todo` is the struct each row fills. In the file, `id AS \"id: TodoId\"` tells the macro to read the `BIGINT` column `id` into `TodoId`, a newtype of `i64`:",
                ja: "`query_file!` と `query_file_as!` はクエリを `.sql` ファイルから読むので、SQL を独立したファイルに置けます。[`examples/postgres`](https://github.com/imaimai17468/hypersig/tree/main/examples/postgres) はクエリをこの方法で `queries/` に置き、一覧を `sqlx::query_file_as!(Todo, \"queries/list_todos.sql\")` で読みます。`Todo` は各行が埋める構造体です。ファイルの `id AS \"id: TodoId\"` は `BIGINT` の列 `id` を `i64` の newtype である `TodoId` に読み込むようマクロに指示します。",
            },
        ),
        code::block(
            Lang::Html,
            Some("queries/list_todos.sql"),
            r#"SELECT id AS "id: TodoId", title, done
FROM todos
ORDER BY id"#,
        ),
        para(
            site,
            L {
                en: "The queries of `examples/database` and of the Cloudflare template are plain strings, as is the one on [Storing data](learn/database/), and a mistake in them fails when the query runs.",
                ja: "`examples/database` と Cloudflare のテンプレートのクエリは、[データを保存する](learn/database/) のものと同じくただの文字列なので、誤りはクエリを実行したときに失敗します。",
            },
        ),
    ])
}

/// The build script that names the files the macros read.
fn rebuild(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Checking a query again after an edit",
                ja: "書き換えたクエリを検査し直す",
            },
        ),
        para(
            site,
            L {
                en: "A proc macro cannot tell cargo which files it read, so after an edit to a `.sql` file alone, cargo would keep the old build and its old check. The example's `build.rs` names `queries/`, `migrations/`, and `.sqlx/` with `cargo::rerun-if-changed`, and an edited query is checked at the next build:",
                ja: "proc macro は読んだファイルを cargo に伝えられないので、`.sql` ファイルだけを書き換えても、cargo は前のビルドと前の検査をそのまま使います。例の `build.rs` が `cargo::rerun-if-changed` で `queries/`、`migrations/`、`.sqlx/` を指定し、書き換えたクエリが次のビルドで検査されるようにしています。",
            },
        ),
        show_file(Snippet::PostgresBuild, "build.rs"),
    ])
}

/// Building without a database, from `.sqlx`.
fn offline(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building without a database",
                ja: "データベースなしでビルドする",
            },
        ),
        para(
            site,
            L {
                en: "In your own project, `cargo sqlx prepare` writes what the macros need to the crate's `.sqlx` directory from a database with the migrations applied, and a build with `SQLX_OFFLINE=true` reads `.sqlx` in place of asking a database. Set it under `[env]` in the project's `.cargo/config.toml`, so a `DATABASE_URL` set for the server does not send the build to a database and a machine or a CI runner without one still builds:",
                ja: "自分のプロジェクトでは、マイグレーションを適用したデータベースから `cargo sqlx prepare` がマクロの使う情報をクレートの `.sqlx` ディレクトリに書き出し、`SQLX_OFFLINE=true` を付けたビルドはデータベースに問い合わせる代わりに `.sqlx` を読みます。この変数はプロジェクトの `.cargo/config.toml` の `[env]` に設定してください。サーバーのために `DATABASE_URL` を設定していてもビルドはデータベースに問い合わせず、データベースのないマシンや CI のランナーでもビルドできます。",
            },
        ),
        code::block(
            Lang::Toml,
            Some(".cargo/config.toml"),
            r#"[env]
SQLX_OFFLINE = "true""#,
        ),
        para(
            site,
            L {
                en: "A query that `.sqlx` does not hold then fails to compile with a message to run `cargo sqlx prepare`. After changing a query or a migration, run it again and commit `.sqlx`:",
                ja: "このとき、`.sqlx` にないクエリは `cargo sqlx prepare` を実行するよう促すメッセージとともにコンパイルエラーになります。クエリかマイグレーションを変えたら、もう一度実行して `.sqlx` をコミットしてください。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo install --locked sqlx-cli --no-default-features --features postgres,rustls
export DATABASE_URL=postgres://postgres:password@localhost/postgres
cargo sqlx migrate run
cargo sqlx prepare",
        ),
        para(
            site,
            L {
                en: "In CI, `cargo sqlx prepare --check` fails when `.sqlx` was not written again. [`examples/postgres`](https://github.com/imaimai17468/hypersig/tree/main/examples/postgres) is set up this way.",
                ja: "CI で `cargo sqlx prepare --check` を実行すると、`.sqlx` を書き出し直していないときに失敗します。[`examples/postgres`](https://github.com/imaimai17468/hypersig/tree/main/examples/postgres) はこの構成です。",
            },
        ),
    ])
}
