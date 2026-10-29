//! The page on serving a page as a static file, with the runtime written
//! next to it.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::StaticPages).title(site)),
        lead(site, Doc::Learn(Learn::StaticPages).leads()),
        heading(
            site,
            L {
                en: "A page without a server",
                ja: "サーバーのないページ",
            },
        ),
        para(
            site,
            L {
                en: "A page whose statements change only signals, and call no action, needs no server once it is rendered. `Page::render` returns the whole document as a `String`, which a program writes to a file. The browser then needs the runtime from a URL the static host serves, so the program writes the runtime's source next to the page and points the page at it:",
                ja: "文がシグナルだけを変え、アクションを呼ばないページは、描画したあとはサーバーを必要としません。`Page::render` は文書全体を `String` で返すので、プログラムはそれをファイルに書き出せます。ブラウザは、静的なホストが配信する URL からランタイムを読み込む必要があります。そこで、プログラムはランタイムのソースをページの隣に書き出し、ページからそれを読み込ませます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Item", "Signature", "What it is"],
                ja: &["項目", "シグネチャ", "内容"],
            },
            L {
                en: &[
                    &[
                        "[`Page::runtime_src`](api:hypersig/struct.Page.html#method.runtime_src)",
                        "`fn runtime_src(self, src: Asset) -> Self`",
                        "Loads the runtime from `src` instead of `RUNTIME_PATH`",
                    ],
                    &[
                        "[`RUNTIME_JS`](api:hypersig/constant.RUNTIME_JS.html)",
                        "`const RUNTIME_JS: &str`",
                        "The browser runtime's source, to write next to the page",
                    ],
                    &[
                        "[`RUNTIME_PATH`](api:hypersig/constant.RUNTIME_PATH.html)",
                        "`const RUNTIME_PATH: &str`",
                        "The path a page loads the runtime from without `runtime_src`, where `RouterExt::hypersig_runtime` serves it",
                    ],
                    &[
                        "[`Page::render_under`](api:hypersig/struct.Page.html#method.render_under)",
                        "`fn render_under(&self, base: Base) -> String`",
                        "Renders the document for a site served under `base`",
                    ],
                ],
                ja: &[
                    &[
                        "[`Page::runtime_src`](api:hypersig/struct.Page.html#method.runtime_src)",
                        "`fn runtime_src(self, src: Asset) -> Self`",
                        "ランタイムを `RUNTIME_PATH` ではなく `src` から読み込みます",
                    ],
                    &[
                        "[`RUNTIME_JS`](api:hypersig/constant.RUNTIME_JS.html)",
                        "`const RUNTIME_JS: &str`",
                        "ブラウザのランタイムのソース。ページの隣に書き出します",
                    ],
                    &[
                        "[`RUNTIME_PATH`](api:hypersig/constant.RUNTIME_PATH.html)",
                        "`const RUNTIME_PATH: &str`",
                        "`runtime_src` がないときにページがランタイムを読み込むパス。`RouterExt::hypersig_runtime` がここで配信します",
                    ],
                    &[
                        "[`Page::render_under`](api:hypersig/struct.Page.html#method.render_under)",
                        "`fn render_under(&self, base: Base) -> String`",
                        "`base` の下で配信するサイトのために文書を描画します",
                    ],
                ],
            },
        ),
        show(Snippet::StaticPage),
        para(
            site,
            L {
                en: "The page's `<head>` then holds `<script type=\"module\" src=\"./hypersig.js\"></script>`. A URL relative to the page, such as `./hypersig.js`, is written as given. The first HTML already shows each binding's initial value, so the page reads correctly before the script runs.",
                ja: "ページの `<head>` には `<script type=\"module\" src=\"./hypersig.js\"></script>` が入ります。`./hypersig.js` のようにページからの相対 URL は書いたとおりに出力されます。最初の HTML には各束縛の初期値がすでに入っているので、スクリプトが動く前からページは正しく表示されます。",
            },
        ),
        para(
            site,
            L {
                en: "`runtime_src` takes an [`Asset`](api:hypersig/struct.Asset.html), so a plain string is E0308, mismatched types. An `Asset` is built only from text written in the program, because the runtime's URL decides what code the page runs:",
                ja: "`runtime_src` は [`Asset`](api:hypersig/struct.Asset.html) を受け取るので、素の文字列を渡すと E0308（型の不一致）になります。ランタイムの URL はページが実行するコードを決めるので、`Asset` はプログラムに書いたテキストからしか作れません。",
            },
        ),
        show(Snippet::RuntimeSrcText),
        heading(
            site,
            L {
                en: "A site under a path prefix",
                ja: "パスの接頭辞の下にあるサイト",
            },
        ),
        para(
            site,
            L {
                en: "A GitHub Pages project site is served at `/<repository>`, such as `/hypersig`. `Page::render_under` takes that prefix as a [`Base`](api:hypersig/struct.Base.html), a constant from `Base::new`, and writes the runtime's path and each `Asset` path that starts with a single `/` under it, as a page served by a route under that base would. A relative URL, or one on another host, stays as written.",
                ja: "GitHub Pages のプロジェクトサイトは、`/hypersig` のように `/<リポジトリ名>` で配信されます。`Page::render_under` はこの接頭辞を [`Base`](api:hypersig/struct.Base.html)（`Base::new` で作る定数）として受け取り、ランタイムのパスと、`/` 1 つで始まる各 `Asset` のパスをその下に書きます。その `Base` の下のルートが配信するページと同じ書き方です。相対 URL やほかのホストの URL は、書いたとおりのままです。",
            },
        ),
        show(Snippet::RenderUnder),
        para(
            site,
            L {
                en: "`Base::new` panics, and in a `const` fails to compile, on a path that does not start with `/`, ends with `/`, or is `/` alone. [Serving under a path prefix](learn/base-path/) covers a `Base` for a served application.",
                ja: "`Base::new` は、`/` で始まらないパス、`/` で終わるパス、`/` だけのパスで panic し、`const` の中ではコンパイルエラーになります。配信するアプリケーションでの `Base` の使い方は [パスの接頭辞の下で配信する](learn/base-path/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "This site is built that way: a Rust program renders each page with hypersig and `render_under`, and writes the files that GitHub Pages serves. The live demos on these pages run on signals alone for the same reason.",
                ja: "このサイトもその方法で作っています。Rust のプログラムが hypersig の `render_under` で各ページを描画し、GitHub Pages が配信するファイルを書き出します。各ページのデモがシグナルだけで動くのも同じ理由です。",
            },
        ),
        heading(
            site,
            L {
                en: "What a static host leaves out",
                ja: "静的なホストで抜け落ちるもの",
            },
        ),
        para(
            site,
            L {
                en: "A static file carries none of the headers a served page has, `X-Frame-Options` and `frame-ancestors` among them, and a `<meta>` cannot carry `frame-ancestors`, so set them in the host's configuration where the host allows it.",
                ja: "静的なファイルには、配信するページに付く `X-Frame-Options` や `frame-ancestors` のヘッダーが付きません。`<meta>` では `frame-ancestors` を指定できないので、ヘッダーを設定できるホストではホストの設定で送ってください。",
            },
        ),
        para(
            site,
            L {
                en: "The types do not tell a static page from a served one, so a static page that calls an action still compiles. The static host has no handler at the action's path, so the call gets whatever error the host answers with, and fails as [Busy and failed calls](learn/call-status/) describes. A runtime file missing from the host shows as a 404 in the browser's console, and the page then shows its first HTML and reacts to nothing.",
                ja: "静的なページと配信するページは型では区別されないので、静的なページがアクションを呼んでもコンパイルは通ります。静的なホストにはアクションのパスを処理するハンドラがないので、呼び出しはホストが返すエラーを受け取り、[呼び出し中と失敗](learn/call-status/) で説明するとおりに失敗します。ランタイムのファイルがホストにない場合は、ブラウザのコンソールに 404 が表示され、ページは最初の HTML を表示するだけで、操作に反応しません。",
            },
        ),
        para(
            site,
            L {
                en: "Next, [Security](learn/security/) covers what hypersig checks in what a visitor sends.",
                ja: "次の [セキュリティ](learn/security/) では訪問者が送るものについて hypersig が検査することを説明します。",
            },
        ),
    ])
}
