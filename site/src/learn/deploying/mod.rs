//! The pages on deploying an application from a template: which host each
//! template is for, then one page per host.

pub mod cloudflare;
pub mod container;
pub mod vercel;

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, lead, para, parts, table_of, title};

pub fn deploying(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Deploying).title(site)),
        lead(site, Doc::Learn(Learn::Deploying).leads()),
        table_of(
            site,
            L {
                en: &["Page", "Created with", "Runs as", "Needs besides Rust 1.99"],
                ja: &[
                    "ページ",
                    "作成のコマンド",
                    "動かし方",
                    "Rust 1.99 のほかに必要なもの",
                ],
            },
            L {
                en: &[
                    &[
                        "[In a container](learn/deploying/container/)",
                        "`cargo hypersig new --host server`",
                        "A binary in a container image, on a host such as Fly.io, Cloud Run, or Render",
                        "Docker",
                    ],
                    &[
                        "[Cloudflare Workers](learn/deploying/cloudflare/)",
                        "`cargo hypersig new --host cloudflare`",
                        "A worker compiled to wasm, run by workerd",
                        "A Cloudflare account, and Node.js 22 for `wrangler`",
                    ],
                    &[
                        "[Vercel](learn/deploying/vercel/)",
                        "`cargo hypersig new --host vercel`",
                        "A binary that Vercel builds and runs as a Vercel Function",
                        "A Vercel account, and Node.js 22 for `vercel`",
                    ],
                ],
                ja: &[
                    &[
                        "[コンテナ](learn/deploying/container/)",
                        "`cargo hypersig new --host server`",
                        "コンテナのイメージに入れたバイナリを、Fly.io、Cloud Run、Render のようなホストで動かします",
                        "Docker",
                    ],
                    &[
                        "[Cloudflare Workers](learn/deploying/cloudflare/)",
                        "`cargo hypersig new --host cloudflare`",
                        "wasm にコンパイルした Worker を workerd で動かします",
                        "Cloudflare のアカウントと、`wrangler` を動かす Node.js 22",
                    ],
                    &[
                        "[Vercel](learn/deploying/vercel/)",
                        "`cargo hypersig new --host vercel`",
                        "Vercel がバイナリをビルドし、Vercel の関数として動かします",
                        "Vercel のアカウントと、`vercel` を動かす Node.js 22",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`cargo install --locked hypersig_cli` installs `cargo hypersig`, which holds the three templates. They hold the same pages, actions, and streams. What differs is the code that hands a request to the axum `Router`, the files around it, and what the host keeps between requests, so each page also covers turning a server project you already have into that template's shape with `cargo hypersig add`.",
                ja: "`cargo install --locked hypersig_cli` で、3 つのテンプレートを持つ `cargo hypersig` をインストールできます。3 つのテンプレートのページ、アクション、ストリームのコードは同じです。違うのは、リクエストを axum の `Router` に渡すコードと、その周りのファイルと、ホストがリクエストのあいだに何を残すかです。そのため、どのページにも既存のサーバーのプロジェクトを `cargo hypersig add` でそのテンプレートの形に移す方法を載せています。",
            },
        ),
        para(
            site,
            L {
                en: "Every template also writes `AGENTS.md`, the instructions a coding agent such as Claude Code or Codex reads: the checks to run after a change, where the [`llms.txt`](llms.txt) index of this guide is, how hypersig code is written, and what the host keeps between requests. `CLAUDE.md` holds `@AGENTS.md`, which Claude Code reads as an import of that file. `cargo hypersig add` writes the two files only where the project has none, even with `--force`.",
                ja: "どのテンプレートも `AGENTS.md` を書き出します。Claude Code や Codex のようなコーディングエージェントが読む指示で、変更のあとに実行するチェック、このガイドの索引 [`llms.txt`](llms.txt) の場所、hypersig のコードの書き方、ホストがリクエストのあいだに何を残すかを書いています。`CLAUDE.md` には `@AGENTS.md` の 1 行だけがあり、Claude Code はこれを `AGENTS.md` の取り込みとして読みます。`cargo hypersig add` は、この 2 つのファイルがプロジェクトにないときだけ書き出し、`--force` を付けても既存のものは書き換えません。",
            },
        ),
    ])
}
