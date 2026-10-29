//! The page on where each deploy target lets the server keep its data: a
//! SQLite file on a container's volume, a database a Vercel Function
//! reaches over the network, and the bindings of a Cloudflare Worker.

use super::next;
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::DatabaseHosts).title(site)),
        lead(site, Doc::Learn(Learn::DatabaseHosts).leads()),
        container(site),
        vercel(site),
        workers(site),
        next(
            site,
            &[
                L {
                    en: "[Deploying](learn/deploying/) builds and runs the server on each of these hosts.",
                    ja: "[デプロイ](learn/deploying/) ではそれぞれのデプロイ先でサーバーをビルドして動かします。",
                },
                L {
                    en: "[Hosted PostgreSQL and MySQL](learn/hosted-databases/) lists the connection settings of the databases a function reaches over the network.",
                    ja: "[ホスティングされた PostgreSQL と MySQL](learn/hosted-databases/) には、関数がネットワーク越しにつなぐデータベースの接続の設定をまとめています。",
                },
            ],
        ),
    ])
}

/// SQLite's file on a container's volume.
fn container(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "SQLite in a container",
                ja: "コンテナでの SQLite",
            },
        ),
        para(
            site,
            L {
                en: "SQLite keeps the database in a file, so the file has to sit on a disk that outlives the process. In [a container](learn/deploying/container/), mount a volume and keep the processes that open the file on one machine, because SQLite's WAL mode shares memory between them.",
                ja: "SQLite はデータベースを 1 つのファイルに置くので、そのファイルはプロセスが終わっても残るディスクに置く必要があります。[コンテナ](learn/deploying/container/) ではボリュームをマウントし、ファイルを開くプロセスを 1 台のマシンに収めてください。SQLite の WAL モードはファイルを開くプロセスのあいだでメモリを共有するためです。",
            },
        ),
    ])
}

/// The `/tmp` of a Vercel Function.
fn vercel(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Vercel Functions",
                ja: "Vercel Functions",
            },
        ),
        para(
            site,
            L {
                en: "A Vercel Function writes only to a `/tmp` of its own instance, so its data goes to a database the function reaches over the network.",
                ja: "Vercel Functions が書き込めるのはインスタンスごとの `/tmp` だけなので、データは関数がネットワーク越しにつなぐデータベースに置きます。",
            },
        ),
    ])
}

/// The bindings of a Cloudflare Worker, and Hyperdrive.
fn workers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Cloudflare Workers",
                ja: "Cloudflare Workers",
            },
        ),
        para(
            site,
            L {
                en: "A Cloudflare Worker is compiled to wasm, where tokio has no networking. sqlx opens its connections with tokio's `TcpStream`, so a worker reaches its data through the bindings of `wrangler.toml`. The handlers take them from the `Env` that `fetch` receives: the [Cloudflare template](learn/deploying/cloudflare/) puts `env.d1(\"DB\")` in its `AppState` and saves its count in D1, and KV and Durable Objects come from `env.kv` and `env.durable_object` in the same way.",
                ja: "Cloudflare Workers の Worker は wasm にコンパイルされ、wasm では tokio のネットワーク機能が使えません。sqlx は tokio の `TcpStream` で接続を開くので、Worker は `wrangler.toml` のバインディングを通してデータを使います。ハンドラはそれを `fetch` が受け取る `Env` から取り出します。[Cloudflare のテンプレート](learn/deploying/cloudflare/) は `env.d1(\"DB\")` を `AppState` に入れ、数を D1 に保存します。KV と Durable Object も同じように `env.kv` と `env.durable_object` で取り出せます。",
            },
        ),
        para(
            site,
            L {
                en: "For PostgreSQL, such as Supabase, Cloudflare's Hyperdrive gives the worker a connection string, and Cloudflare's [tokio-postgres example](https://github.com/cloudflare/workers-rs/tree/main/examples/tokio-postgres) connects to it over a `worker::Socket`. Cloudflare publishes no Rust example for MySQL over Hyperdrive.",
                ja: "Supabase などの PostgreSQL には、Cloudflare の Hyperdrive が Worker に接続文字列を渡し、Cloudflare の [tokio-postgres の例](https://github.com/cloudflare/workers-rs/tree/main/examples/tokio-postgres) は `worker::Socket` でそれにつなぎます。Hyperdrive 経由の MySQL について、Cloudflare は Rust の例を公開していません。",
            },
        ),
    ])
}
