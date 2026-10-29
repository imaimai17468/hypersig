//! The page on a database read through an SDK in place of SQL: the
//! `DynamoDB` client in the router's state, what its strings leave unchecked, and the
//! crates for Firestore.

use super::next;
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DatabaseWithoutSql).title(site)),
        lead(site, Doc::Learn(Learn::DatabaseWithoutSql).leads()),
        client(site),
        unchecked(site),
        firestore(site),
        next(
            site,
            &[
                L {
                    en: "[Storing data](learn/database/) covers the connection in the state, what a failed query answers, and updates across processes.",
                    ja: "[データを保存する](learn/database/) では状態に置く接続、クエリが失敗したときの応答、プロセスをまたぐ更新を説明します。",
                },
                L {
                    en: "[The database each host can reach](learn/database-hosts/) covers the hosts that reach a database only over the network or through a binding.",
                    ja: "[デプロイ先ごとのデータベース](learn/database-hosts/) ではネットワーク越しかバインディングでしかデータベースを使えないデプロイ先を説明します。",
                },
            ],
        ),
    ])
}

/// The `DynamoDB` client in the router's state.
fn client(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The client in the router's state",
                ja: "ルーターの状態に置くクライアント",
            },
        ),
        para(
            site,
            L {
                en: "A database with an SDK in place of SQL takes the same shape as one with SQL: the client goes in the router's state and the handler awaits its calls. The sample below reads DynamoDB with `aws-sdk-dynamodb`, AWS's own crate, and needs these two crates:",
                ja: "SQL の代わりに SDK で使うデータベースも、SQL のものと形は同じで、クライアントをルーターの状態に置き、ハンドラはその呼び出しを `.await` します。下のサンプルは AWS 公式のクレート `aws-sdk-dynamodb` で DynamoDB を読みます。次の 2 つのクレートが必要です。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
aws-config = "1.13"
aws-sdk-dynamodb = "1.134""#,
        ),
        show(Snippet::DynamoDb),
        para(
            site,
            L {
                en: "`aws_config::defaults(..).load()` reads the region and the credentials from the environment, as the AWS CLI does, so they stay on the server. A `Client` is `Clone`, and its clones share one client, so the state holds it without an `Arc`, and the server builds it once when it starts. A call that fails, such as one without credentials, returns an error that `DbError` logs and answers with a 500, as the SQL sample does.",
                ja: "`aws_config::defaults(..).load()` は AWS CLI と同じようにリージョンと認証情報を環境から読むので、これらはサーバーから出ません。`Client` は `Clone` で、複製どうしが 1 つのクライアントを共有します。そのため状態には `Arc` に入れずに置き、サーバーの起動時に 1 回だけ作ります。認証情報がないときのように呼び出しが失敗すると、エラーが返ります。`DbError` はそれをログに出し、SQL のサンプルと同じように 500 で応答します。",
            },
        ),
    ])
}

/// The strings of a `DynamoDB` call that nothing checks.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型で検査しないもの",
            },
        ),
        para(
            site,
            L {
                en: "The table name, the key condition, and the attribute names are strings that the compiler does not read. A wrong table name or condition fails when `send` runs, and the visitor sees the 500. An item without a `title`, or one whose `title` is not a string, is a value `as_s` refuses, and the sample's `filter_map` leaves it out of the list without a message. Where a missing attribute is a mistake to hear about, log it in place of skipping it.",
                ja: "テーブル名、キーの条件、属性の名前は、コンパイラが読まない文字列です。テーブル名や条件を間違えると `send` を実行したときに失敗し、訪問者には 500 が届きます。`title` のない項目や、`title` が文字列でない項目は `as_s` が受け付けず、サンプルの `filter_map` は何も知らせずにその項目を一覧から外します。属性がないことを誤りとして知りたいときは、飛ばす代わりにログに出してください。",
            },
        ),
    ])
}

/// The crates for Firestore.
fn firestore(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Firestore",
                ja: "Firestore",
            },
        ),
        para(
            site,
            L {
                en: "For Firestore, Google has published no crate for reading and writing documents as of October 2026, only `google-cloud-firestore-admin-v1` for the admin API, and the community's [`firestore`](https://docs.rs/firestore) crate is the one in common use.",
                ja: "Firestore については、2026 年 10 月の時点で Google はドキュメントを読み書きするクレートを公開しておらず、公開しているのは管理用 API の `google-cloud-firestore-admin-v1` だけです。よく使われているのはコミュニティの [`firestore`](https://docs.rs/firestore) クレートです。",
            },
        ),
    ])
}
