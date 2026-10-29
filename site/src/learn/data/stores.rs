//! The page on where the sessions are stored: `MemoryStore` in one
//! process, the stores that keep them in a database, and the setup a
//! Cloudflare Worker needs.

use super::next;
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::SessionStores).title(site)),
        lead(site, Doc::Learn(Learn::SessionStores).leads()),
        memory(site),
        databases(site),
        workers(site),
        next(
            site,
            &[
                L {
                    en: "[Storing data](learn/database/) keeps the application's own data in the same databases.",
                    ja: "[データを保存する](learn/database/) ではアプリ自身のデータを同じデータベースに置きます。",
                },
                L {
                    en: "[Deploying](learn/deploying/) covers the hosts that run more than one instance.",
                    ja: "[デプロイ](learn/deploying/) ではインスタンスを複数動かすデプロイ先を説明します。",
                },
            ],
        ),
    ])
}

/// `MemoryStore` and its limits.
fn memory(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Sessions in memory",
                ja: "メモリに置くセッション",
            },
        ),
        para(
            site,
            L {
                en: "`MemoryStore` keeps the sessions in the process's memory. A restart signs every visitor out, and a second process does not see them, so a request that a load balancer sends to another instance is signed out. That is the same limit as a `broadcast` channel in [Updates with more than one process](learn/database/#updates-with-more-than-one-process).",
                ja: "`MemoryStore` はセッションをプロセスのメモリに置きます。プロセスを起動し直すと全員がサインアウトし、2 つ目のプロセスからは見えないので、ロードバランサーが別のインスタンスに送ったリクエストはサインインしていない扱いになります。[プロセスが複数あるときの更新](learn/database/#updates-with-more-than-one-process) の `broadcast` のチャネルと同じ制限です。",
            },
        ),
    ])
}

/// The stores that keep the sessions in a database.
fn databases(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Sessions in a database",
                ja: "データベースに置くセッション",
            },
        ),
        para(
            site,
            L {
                en: "On a host that runs more than one instance, or that starts and stops instances as requests come as Vercel does, pass the layer a store that keeps the sessions in a database. `SessionManagerLayer::new` takes any store that implements `SessionStore`, so the rest of the code stays as it is.",
                ja: "インスタンスを複数動かすホストや、Vercel のようにリクエストに合わせてインスタンスを起動したり止めたりするホストでは、セッションをデータベースに置くストアをレイヤーに渡してください。`SessionManagerLayer::new` は `SessionStore` を実装するどのストアも受け取るので、ほかのコードはそのままです。",
            },
        ),
        para(
            site,
            L {
                en: "`tower-sessions-sqlx-store` keeps them in PostgreSQL, SQLite, or MySQL through sqlx 0.8, so an application on sqlx 0.9 builds both versions and cannot hand the store its own pool. `tower-sessions-redis-store` connects through the `fred` client, so it does not share a connection with the `redis` crate that `examples/redis` uses.",
                ja: "`tower-sessions-sqlx-store` は sqlx 0.8 を通して PostgreSQL、SQLite、MySQL に置きます。そのため sqlx 0.9 を使うアプリでは両方のバージョンがビルドされ、アプリのプールをストアに渡せません。`tower-sessions-redis-store` は `fred` クライアントでつなぐので、`examples/redis` が使う `redis` クレートと接続を共有しません。",
            },
        ),
    ])
}

/// What tower-sessions needs on Cloudflare Workers.
fn workers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "On Cloudflare Workers",
                ja: "Cloudflare Workers では",
            },
        ),
        note(
            site,
            L {
                en: "On Cloudflare Workers, tower-sessions 0.14 needs two features in the worker's `Cargo.toml`: `getrandom = { version = \"0.2\", features = [\"js\"] }`, without which it does not build for `wasm32-unknown-unknown`, and `time = { version = \"0.3\", features = [\"wasm-bindgen\"] }`, without which saving a session panics with `time not implemented on this platform`. With both, the layer kept a session under `wrangler dev`. A worker's isolates share no memory, so `MemoryStore` there signs a visitor out whenever a request reaches another isolate. A store that keeps the sessions in D1 or KV implements `SessionStore`, and this guide has not written one.",
                ja: "Cloudflare Workers では、tower-sessions 0.14 に Worker の `Cargo.toml` で 2 つのフィーチャーが必要です。1 つは `getrandom = { version = \"0.2\", features = [\"js\"] }` で、これがないと `wasm32-unknown-unknown` 向けにビルドできません。もう 1 つは `time = { version = \"0.3\", features = [\"wasm-bindgen\"] }` で、これがないとセッションを保存するときに `time not implemented on this platform` でパニックします。両方を入れると、`wrangler dev` でレイヤーがセッションを保てました。Worker の isolate どうしはメモリを共有しないので、そこで `MemoryStore` を使うと、リクエストが別の isolate に届くたびに訪問者はサインインしていない扱いになります。セッションを D1 や KV に置くストアは `SessionStore` を実装すれば作れますが、このガイドではまだ書いていません。",
            },
        ),
    ])
}
