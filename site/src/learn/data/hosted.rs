//! The page on the hosted PostgreSQL and MySQL services: the connection
//! string, TLS, and pooler each one asks of sqlx, and the pools of a host
//! that adds instances as requests grow.

use super::next;
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::HostedDatabases).title(site)),
        lead(site, Doc::Learn(Learn::HostedDatabases).leads()),
        settings(site),
        vercel(site),
        next(
            site,
            &[
                L {
                    en: "[Storing data](learn/database/) builds the pool from `DATABASE_URL` and covers `PgListener` behind a pooler.",
                    ja: "[データを保存する](learn/database/) では `DATABASE_URL` からプールを作り、プーラーの裏での `PgListener` についても説明します。",
                },
                L {
                    en: "[The database each host can reach](learn/database-hosts/) covers Cloudflare's Hyperdrive, which hands a worker a connection string for PostgreSQL.",
                    ja: "[デプロイ先ごとのデータベース](learn/database-hosts/) では Worker に PostgreSQL の接続文字列を渡す Cloudflare の Hyperdrive を説明します。",
                },
            ],
        ),
    ])
}

/// The settings that hosted PostgreSQL and MySQL services ask of sqlx.
fn settings(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The settings each service asks for",
                ja: "サービスごとの設定",
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
                        "トークンの有効期間は 15 分なので、プールが新しく開く接続ごとに新しいトークンが必要です",
                    ],
                ],
            },
        ),
    ])
}

/// The pool of each Vercel instance, and the poolers that keep it small.
fn vercel(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Pools on Vercel",
                ja: "Vercel でのプール",
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
