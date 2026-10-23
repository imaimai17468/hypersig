//! The page on testing an application: calling its handlers in a test,
//! sending requests to its router, and what only a browser runs.

use super::{Program, program};
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn testing(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Testing).title(site)),
        lead(site, Doc::Learn(Learn::Testing).leads()),
        handlers(site),
        router(site),
        browser(site),
    ])
}

/// Tests that call a page's and an action's handler as functions.
fn handlers(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Calling a handler",
                ja: "ハンドラを呼ぶ",
            },
        ),
        para(
            site,
            L {
                en: "A handler is an `async fn` that takes the router's state and the arguments, so a test calls it as a function and reads what it returned. The tests below need three crates the server does not, added to `Cargo.toml`:",
                ja: "ハンドラはルーターの状態と引数を受け取る `async fn` なので、テストは関数として呼び出して戻り値を読めます。下のテストは、サーバーが使わないクレートを 3 つ使います。`Cargo.toml` に次の節を足してください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dev-dependencies]
http-body-util = "0.1"
serde_json = "1"
tower = { version = "0.5", features = ["util"] }"#,
        ),
        program(Program::Testing),
        para(
            site,
            L {
                en: "[`Page::value_of`](api:hypersig/struct.Page.html#method.value_of) reads the value a signal starts at in the browser, and [`Patches::value_of`](api:hypersig/struct.Patches.html#method.value_of) the value a batch sets it to, or `None` where the batch leaves it alone. Both take the `Sig` the derive generated, so a test names a signal as the page does, and a misspelled field fails to compile. A handler that refuses returns its error, which a test compares as a value. [`Patches::navigates_to`](api:hypersig/struct.Patches.html#method.navigates_to) returns the URL a batch sends the browser to, and [`Patches::replaces`](api:hypersig/struct.Patches.html#method.replaces) whether it re-renders a fragment. A stream's handler returns a `Stream` of `Patches`, so a test collects it with `StreamExt::collect` from the `futures` crate and reads each batch.",
                ja: "[`Page::value_of`](api:hypersig/struct.Page.html#method.value_of) はブラウザでのシグナルの初期値を、[`Patches::value_of`](api:hypersig/struct.Patches.html#method.value_of) は差分がシグナルに書く値を返します。差分がそのシグナルに触れないときは `None` です。どちらも derive が生成した `Sig` を受け取るので、テストもページと同じ書き方でシグナルを指し、フィールド名を間違えるとコンパイルが通りません。拒否するハンドラはエラーを返すので、テストはそのエラーを値として比べます。[`Patches::navigates_to`](api:hypersig/struct.Patches.html#method.navigates_to) は差分がブラウザを移動させる先の URL を、[`Patches::replaces`](api:hypersig/struct.Patches.html#method.replaces) はフラグメントを描画し直すかどうかを返します。ストリームのハンドラは `Patches` の `Stream` を返すので、テストは `futures` クレートの `StreamExt::collect` で集めてから差分を 1 つずつ読みます。",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::render`](api:hypersig/struct.Page.html#method.render) returns the document the server sends, with each bound text and attribute written from the initial values, so a test can look for a text in it. It panics where two elements share an id or a `NodeRef` name, so a test that renders each page finds those before a visitor does.",
                ja: "[`Page::render`](api:hypersig/struct.Page.html#method.render) はサーバーが送る文書を返します。束縛したテキストと属性は初期値で書かれているので、テストはその中から文字列を探せます。2 つの要素が同じ id や `NodeRef` の名前を持つと `render` はパニックするので、ページごとに描画するテストを書いておくと、訪問者が開く前にそれがわかります。",
            },
        ),
    ])
}

/// Tests that send a request through the router, and the headers it needs.
fn router(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Sending a request to the router",
                ja: "ルーターにリクエストを送る",
            },
        ),
        para(
            site,
            L {
                en: "The last two tests send a request to the `Router` with `oneshot` from `tower::ServiceExt`, which runs it in the process without opening a port. Write a test at this level for what the router adds to the handler: an extractor that refuses a visitor, the status of a refusal, the page for a URL no route matches, and the 400 for arguments that do not deserialize. The path and the method come from `LIKE.path()` and `LIKE.method()`, and a page's URL from `Route::url`, so a renamed action or route moves the test with it.",
                ja: "最後の 2 つのテストは、`tower::ServiceExt` の `oneshot` で `Router` にリクエストを送ります。`oneshot` はポートを開かずにプロセスの中でリクエストを処理します。このレベルのテストは、ルーターがハンドラに足すものを確かめるために書きます。訪問者を拒否するエクストラクタ、拒否したときのステータス、どのルートにも合わない URL のページ、変換できない引数への 400 です。パスとメソッドは `LIKE.path()` と `LIKE.method()` から、ページの URL は `Route::url` から作るので、アクションやルートの名前を変えるとテストも一緒に変わります。",
            },
        ),
        para(
            site,
            L {
                en: "The headers are the ones the runtime sends, and the router refuses a request without them. A `POST` action needs `Content-Type: application/json`, with the arguments as the JSON body. A `GET` action and a stream need `Accept: text/event-stream`, with the arguments as JSON in the `args` query parameter. The runtime sends `Accept: text/event-stream` with a `POST` as well, and [`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) reads that header to tell a call from a page load, so a test of an extractor that answers the two differently sends it with every call. A `Sec-Fetch-Site` other than `same-origin` is refused, and a request without one passes. The body of an answered action is the batch as Server-Sent Events, which [`Patches::to_event_stream`](api:hypersig/struct.Patches.html#method.to_event_stream) writes, so a test compares it with the batch it expects in place of a text written by hand.",
                ja: "ヘッダーはランタイムが送るものと同じで、ルーターはこれがないリクエストを拒否します。`POST` のアクションには `Content-Type: application/json` が要り、引数は JSON の本文で送ります。`GET` のアクションとストリームには `Accept: text/event-stream` が要り、引数は `args` のクエリパラメータに JSON で入れます。ランタイムは `POST` にも `Accept: text/event-stream` を付け、[`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) はこのヘッダーで呼び出しとページの読み込みを見分けます。そのため、2 つに違う応答をするエクストラクタのテストでは、どの呼び出しにもこのヘッダーを付けてください。`Sec-Fetch-Site` が `same-origin` 以外なら拒否され、ヘッダー自体がなければ通ります。応答したアクションの本文は差分を Server-Sent Events にしたもので、[`Patches::to_event_stream`](api:hypersig/struct.Patches.html#method.to_event_stream) が同じ形式で書くので、テストは手で書いた文字列の代わりに、期待する差分と比べます。",
            },
        ),
    ])
}

/// What a Rust test does not run, and the browser tests that do.
fn browser(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What runs only in the browser",
                ja: "ブラウザでだけ動くもの",
            },
        ),
        para(
            site,
            L {
                en: "A Rust test runs no JavaScript, so a statement such as `count.set(count + 1)`, a binding that changes after the page loads, and the runtime applying a batch are outside its reach. Drive those in a browser. The repository's [`e2e`](https://github.com/imaimai17468/hypersig/tree/main/e2e) directory starts each example and clicks through it in Chrome with Playwright, and a test of your application can follow its layout.",
                ja: "Rust のテストは JavaScript を実行しないので、`count.set(count + 1)` のような文、ページを読み込んだあとに変わる束縛、ランタイムが差分を当てる処理は確かめられません。これらはブラウザで動かして確かめます。リポジトリの [`e2e`](https://github.com/imaimai17468/hypersig/tree/main/e2e) ディレクトリは、例を 1 つずつ起動し、Playwright で Chrome を操作します。アプリのブラウザのテストも同じ構成で書けます。",
            },
        ),
    ])
}
