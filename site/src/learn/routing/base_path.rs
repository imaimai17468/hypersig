//! The page on serving an application under a path prefix with `Base`.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::BasePath).title(site)),
        lead(site, Doc::Learn(Learn::BasePath).leads()),
        naming(site),
        prefixed(site),
        serving(site),
        unchecked(site),
        next(site),
    ])
}

/// `Base::new` and `under BASE`.
fn naming(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Naming the prefix",
                ja: "接頭辞を宣言する",
            },
        ),
        para(
            site,
            L {
                en: "An application served under a path prefix, such as `/app` behind a proxy, names it once as a [`Base`](api:hypersig/struct.Base.html) constant, built with `const fn Base::new(path: &'static str) -> Base`, and writes `under APP` on the first line of `routes!`: `fn pages(Router<State>) under APP;`. `APP` is the name of a `Base` constant in scope, without a path before it. `Base::ROOT` is no prefix at all, and `Base::as_str` gives the prefix as text.",
                ja: "プロキシの裏の `/app` のようにパスの接頭辞の下で配信するアプリケーションは、接頭辞を [`Base`](api:hypersig/struct.Base.html) の定数として 1 か所で宣言します。定数は `const fn Base::new(path: &'static str) -> Base` で作り、`routes!` の 1 行目に `fn pages(Router<State>) under APP;` と `under APP` を書きます。`APP` はスコープにある `Base` の定数の名前で、前にパスを付けずに書きます。`Base::ROOT` は接頭辞なしを表し、`Base::as_str` は接頭辞を文字列で返します。",
            },
        ),
        para(
            site,
            L {
                en: "The prefix is a constant because the routes and actions it prefixes are, so it is chosen when the program is built, with `option_env!` where a build sets it. A prefix that does not start with `/`, ends with `/`, is `/` alone, holds `{`, `}`, or `//`, or holds a byte the browser percent-encodes fails to compile in a `const`:",
                ja: "接頭辞を付けるルートとアクションが定数なので、接頭辞も定数で、プログラムをビルドするときに決まります。ビルドで設定するなら `option_env!` で読みます。`/` で始まらない接頭辞、`/` で終わる接頭辞、`/` だけの接頭辞、`{`、`}`、`//` を含む接頭辞、ブラウザがパーセントエンコードするバイトを含む接頭辞は、`const` の中ではコンパイルできません。",
            },
        ),
        show(Snippet::WrongBase),
    ])
}

/// What starts with the prefix and what does not.
fn prefixed(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What starts with the prefix",
                ja: "接頭辞で始まるもの",
            },
        ),
        para(
            site,
            L {
                en: "In the pages and in the patches the block's handlers send, these start with `/app`:",
                ja: "ページの中と、ブロックのハンドラが送る差分の中では、次のものが `/app` で始まります。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Links, `Url`s, and redirects to the block's routes. The route at `/` becomes `/app` alone.",
                    ja: "ブロックのルートへのリンク、`Url`、リダイレクト。`/` のルートは `/app` だけになります。",
                },
                L {
                    en: "The calls to the block's actions, streams, and uploads.",
                    ja: "ブロックのアクション、ストリーム、アップロードの呼び出し。",
                },
                L {
                    en: "Each `Asset` path the page loads.",
                    ja: "ページが読み込む `Asset` のパス。",
                },
                L {
                    en: "Each root path in a stylesheet's `url(..)`.",
                    ja: "スタイルシートの `url(..)` に書いたルートからのパス。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The router still registers each path without the prefix, which `Route::path` and `Action::path` return:",
                ja: "ルーターは各パスを接頭辞なしのまま登録します。そのパスは `Route::path` と `Action::path` が返します。",
            },
        ),
        show(Snippet::BasePath),
    ])
}

/// `Router::nest` or a proxy that strips the prefix.
fn serving(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Serving it",
                ja: "配信する",
            },
        ),
        para(
            site,
            L {
                en: "The same router runs inside `Router::nest(APP.as_str(), app)`, or as it is behind a proxy that strips `/app` before the request arrives.",
                ja: "同じルーターを `Router::nest(APP.as_str(), app)` の中でも、リクエストが届く前に `/app` を取り除くプロキシの裏でも、そのまま動かせます。",
            },
        ),
    ])
}

/// What `Base` leaves to the program.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What stays unchecked",
                ja: "検査されないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A page served by a router nested under any other path answers every request with a 500, whose body names neither path.",
                    ja: "ほかのパスに `nest` したルーターのページは、どのリクエストにも 500 を返します。本文にはどちらのパスも書きません。",
                },
                L {
                    en: "The compiler does not check that an action, a stream, or an upload a page calls comes from a block under the same base. One from a block without `under APP` calls its path without the prefix, which a proxy answers with its own 404 and the router inside `nest` answers with a 404. One block per application keeps every line under one base.",
                    ja: "ページが呼ぶアクション、ストリーム、アップロードが同じ接頭辞のブロックで宣言されているかどうかを、コンパイラは検査しません。`under APP` のないブロックのものは接頭辞なしのパスを呼ぶので、プロキシは独自の 404 を返し、`nest` の中のルーターも 404 を返します。アプリケーションのブロックを 1 つにすれば、どの行も同じ接頭辞の下に入ります。",
                },
                L {
                    en: "Nothing checks that the router serves the file an `Asset` path names.",
                    ja: "`Asset` のパスが指すファイルをルーターが配信しているかどうかは検査されません。",
                },
                L {
                    en: "A `Url` already starts with the base, so a canonical URL joins it to an `Origin` without `Origin::under`, which [The page and its head](learn/page/) covers.",
                    ja: "`Url` はすでに接頭辞で始まるので、canonical の URL では `Origin::under` を使わずに `Origin` へつなぎます。これは [ページと head](learn/page/) で説明します。",
                },
            ],
        ),
    ])
}

/// The pages after this one.
fn next(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        bullets(
            site,
            &[L {
                en: "[Choosing a host](learn/deploying/) compares the hosts the templates deploy to.",
                ja: "テンプレートのデプロイ先は [デプロイ先の選び方](learn/deploying/) で比べます。",
            }],
        ),
    ])
}
