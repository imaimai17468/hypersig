//! The page on testing handlers: calling a page's, an action's, and a
//! stream's handler as a function, and sending a request to the router.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::learn::{Program, program};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Testing).title(site)),
        lead(site, Doc::Learn(Learn::Testing).leads()),
        handlers(site),
        page_values(site),
        patches(site),
        list_patches(site),
        streams(site),
        router(site),
        extractors(site),
        unchecked(site),
        next(site),
    ])
}

/// The example program, and why a handler can be called as a function.
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
                en: "A handler is an `async fn` that takes the router's state and the arguments, so a test calls it as a function and reads what it returned. The tests below need two crates the server does not, added to `Cargo.toml`:",
                ja: "ハンドラはルーターの状態と引数を受け取る `async fn` なので、テストは関数として呼び出して戻り値を読めます。下のテストは、サーバーが使わないクレートを 2 つ使います。`Cargo.toml` に次の節を追加してください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dev-dependencies]
http-body-util = "0.1"
tower = { version = "0.5", features = ["util"] }"#,
        ),
        para(
            site,
            L {
                en: "The program below is a page with a like button and an action that adds to a shared count. Its first three tests call `index` and `like` directly, and the last three send requests to the router. `cargo test` runs all of them.",
                ja: "下のプログラムは、いいねのボタンがあるページと、共有のカウントに数を足すアクションです。最初の 3 つのテストは `index` と `like` を直接呼び、残りの 3 つはルーターにリクエストを送ります。`cargo test` ですべて実行できます。",
            },
        ),
        program(Program::Testing),
    ])
}

/// `Page::value_of` and `Page::render` in a test of a page handler.
fn page_values(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Reading a page's initial values",
                ja: "ページの初期値を読む",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn value_of<T: DeserializeOwned, M: FromPage>(&self, signal: Sig<S, T, M>) -> T\npub fn render(&self) -> String",
        ),
        para(
            site,
            L {
                en: "[`Page::value_of`](api:hypersig/struct.Page.html#method.value_of) returns the value a signal starts at in the browser: the value the page was built with, or for a computed signal, the value computed from the others. It takes the `Sig` the derive generated, so a test names a signal as the page does, and a misspelled field fails to compile.",
                ja: "[`Page::value_of`](api:hypersig/struct.Page.html#method.value_of) は、ブラウザでのシグナルの初期値を返します。ページを組み立てたときの値で、計算シグナルなら、ほかのシグナルから計算した値です。derive が生成した `Sig` を受け取るので、テストもページと同じ書き方でシグナルを指し、フィールド名を間違えるとコンパイルが通りません。",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::render`](api:hypersig/struct.Page.html#method.render) returns the document the server sends, with each bound text and attribute written from the initial values, so a test can look for a text in it, as `the_page_starts_at_the_stored_count` looks for `>7</span>`. It panics where two elements share an id or a `NodeRef` name, so a test that renders each page finds those before a visitor does. [Debugging a running page](learn/debugging/) lists the other panics. To read one element in place of searching the whole document, use `Simulation::element` from [Testing a page without a browser](learn/testing-pages/).",
                ja: "[`Page::render`](api:hypersig/struct.Page.html#method.render) はサーバーが送る文書を返します。束縛したテキストと属性は初期値で書かれているので、`the_page_starts_at_the_stored_count` が `>7</span>` を探すように、テストはその中から文字列を探せます。2 つの要素が同じ id や `NodeRef` の名前を持つと `render` はパニックするので、ページごとに描画するテストを書いておくと、訪問者が開く前にそれがわかります。ほかのパニックは [動いているページを調べる](learn/debugging/) に挙げています。文書全体を探す代わりに要素を 1 つ読むなら、[ブラウザなしでページをテストする](learn/testing-pages/) の `Simulation::element` を使います。",
            },
        ),
    ])
}

/// `Patches::value_of`, `navigates_to`, and `replaces` in a test of an action.
fn patches(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Reading an action's patches",
                ja: "アクションの差分を読む",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn value_of<T: DeserializeOwned>(&self, signal: Sig<S, T>) -> Option<T>\npub fn navigates_to(&self) -> Option<&Url>\npub fn replaces<F: Fragment<Signals = S>>(&self) -> bool",
        ),
        table_of(
            site,
            L {
                en: &["Method of `Patches<S>`", "What it returns"],
                ja: &["`Patches<S>` のメソッド", "返すもの"],
            },
            L {
                en: &[
                    &[
                        "[`value_of`](api:hypersig/struct.Patches.html#method.value_of)",
                        "The value the batch sets the signal to, or `None` where the batch leaves it alone",
                    ],
                    &[
                        "[`navigates_to`](api:hypersig/struct.Patches.html#method.navigates_to)",
                        "The URL of the first `navigate` in the batch, where the browser goes. A later one is never followed.",
                    ],
                    &[
                        "[`replaces`](api:hypersig/struct.Patches.html#method.replaces)",
                        "Whether the batch re-renders the fragment `F`",
                    ],
                ],
                ja: &[
                    &[
                        "[`value_of`](api:hypersig/struct.Patches.html#method.value_of)",
                        "差分がシグナルに書く値。差分がそのシグナルに触れないときは `None`",
                    ],
                    &[
                        "[`navigates_to`](api:hypersig/struct.Patches.html#method.navigates_to)",
                        "差分の最初の `navigate` の URL で、ブラウザの移動先。2 つ目以降には移動しません。",
                    ],
                    &[
                        "[`replaces`](api:hypersig/struct.Patches.html#method.replaces)",
                        "差分がフラグメント `F` を描画し直すかどうか",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "A handler that refuses returns its error, which a test compares as a value, as `more_than_five_likes_are_refused` compares the status and the sentence. `Patches<S>` and the `Sig` share the signal struct `S`, so a test cannot read a batch with the field of another page's struct:",
                ja: "拒否するハンドラはエラーを返すので、`more_than_five_likes_are_refused` がステータスと文を比べるように、テストはそのエラーを値として比べます。`Patches<S>` と `Sig` は同じシグナル構造体 `S` を持つので、別のページの構造体のフィールドで差分を読むテストはコンパイルできません。",
            },
        ),
        show(Snippet::TestingWrongPage),
    ])
}

/// `Patches::items_of` and `removed_keys_of`.
fn list_patches(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Reading a list patch",
                ja: "リストの差分を読む",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn items_of<T: DeserializeOwned>(&self, list: Sig<S, Vec<T>>) -> Vec<T>\npub fn removed_keys_of<T: Serialize, K: DeserializeOwned, E: IntoExpr<S, K>>(\n    &self,\n    list: Sig<S, Vec<T>>,\n    key: impl FnOnce(Item<S, T>) -> E,\n) -> Vec<K>",
        ),
        para(
            site,
            L {
                en: "A handler that answers with a list patch sends only the items that change, so `value_of` does not see them. [`Patches::items_of`](api:hypersig/struct.Patches.html#method.items_of) returns the items a batch appends, prepends, or upserts into a list, in the order the batch sends them, which for a prepend is not the order the browser shows. [`Patches::removed_keys_of`](api:hypersig/struct.Patches.html#method.removed_keys_of) returns the keys it removes. The second takes the key closure the handler passed to `patch_remove`, and leaves out a removal by another key, so the test also checks the key the browser compares.",
                ja: "リストの差分で答えるハンドラは変わる項目だけを送るので、`value_of` からは見えません。[`Patches::items_of`](api:hypersig/struct.Patches.html#method.items_of) は、差分がリストの末尾か先頭に加える項目と upsert で入れる項目を、差分が送る順に返します。先頭に加える項目では、この順はブラウザに表示される順と違います。[`Patches::removed_keys_of`](api:hypersig/struct.Patches.html#method.removed_keys_of) は取り除くキーを返します。`removed_keys_of` はハンドラが `patch_remove` に渡したのと同じキーのクロージャを受け取り、別のキーでの削除は含めません。そのため、テストはブラウザが比べるキーも確かめます。",
            },
        ),
        show(Snippet::TestingListPatches),
    ])
}

/// Collecting a stream's batches in a test.
fn streams(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Reading a stream's batches",
                ja: "ストリームの差分を読む",
            },
        ),
        para(
            site,
            L {
                en: "A stream's handler returns a `Stream` of `Patches`, so a test collects it with `StreamExt::collect` from the `futures` crate and reads each batch with the methods above. A stream that never ends, such as a clock, is read with `StreamExt::take(n)` before `collect`.",
                ja: "ストリームのハンドラは `Patches` の `Stream` を返すので、テストは `futures` クレートの `StreamExt::collect` で集め、上のメソッドで差分を 1 つずつ読みます。時計のように終わらないストリームは、`collect` の前に `StreamExt::take(n)` で数を区切ります。",
            },
        ),
        show(Snippet::TestingStream),
    ])
}

/// Tests that send a request through the router, and the headers it carries.
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
                en: "The last three tests of the program send a request to the `Router` with `oneshot` from `tower::ServiceExt`, which runs it in the process without opening a port. Write a test at this level for what the router adds to the handler: an extractor that refuses a visitor, the status of a refusal, the page for a URL no route matches, and the 400 for arguments that do not deserialize. The action's request comes from `LIKE.request(&3)`, and a page's URL from `Route::url`, so a renamed action or route moves the test with it.",
                ja: "プログラムの最後の 3 つのテストは `tower::ServiceExt` の `oneshot` で `Router` にリクエストを送ります。`oneshot` はポートを開かずにプロセスの中でリクエストを処理します。このレベルのテストは、ルーターがハンドラに加えるものを確かめるために書きます。訪問者を拒否するエクストラクタ、拒否したときのステータス、どのルートにも合わない URL のページ、変換できない引数への 400 です。アクションのリクエストは `LIKE.request(&3)` から、ページの URL は `Route::url` から作るので、アクションやルートの名前を変えるとテストも一緒に変わります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "// impl<S, A: Serialize> Action<S, A>\npub fn request(&self, args: &A) -> axum::http::Request<axum::body::Body>\n// impl<S, A> StreamAction<S, A>\npub fn request(&self, args: &A) -> axum::http::Request<axum::body::Body>\nwhere\n    A: serde::Serialize",
        ),
        para(
            site,
            L {
                en: "[`Action::request`](api:hypersig/struct.Action.html#method.request) builds the request the runtime sends, and [`StreamAction::request`](api:hypersig/struct.StreamAction.html#method.request) the `GET` that opens a stream, so a test names the action, not its path or its headers. `args` has the action's argument type, so a test cannot send arguments the handler does not take. The request carries these:",
                ja: "[`Action::request`](api:hypersig/struct.Action.html#method.request) はランタイムが送るリクエストを、[`StreamAction::request`](api:hypersig/struct.StreamAction.html#method.request) はストリームを開く `GET` を作ります。そのため、テストはパスやヘッダーを書かずにアクションの名前で書けます。`args` はアクションの引数の型なので、ハンドラが受け取らない引数をテストが送ることもできません。リクエストには次のものが入ります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Part of the request", "Value"],
                ja: &["リクエストの部分", "値"],
            },
            L {
                en: &[
                    &[
                        "`Accept`",
                        "`text/event-stream`, which [`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) reads to tell a call (`RequestKind::Runtime`) from a page load (`RequestKind::Navigation`)",
                    ],
                    &[
                        "`hypersig-call`",
                        "The header the runtime adds to every call",
                    ],
                    &[
                        "A `POST`'s body",
                        "The arguments as JSON, with `Content-Type: application/json`",
                    ],
                    &[
                        "A `GET`'s URL",
                        "The arguments as JSON in the `args` query parameter, left out for `()`",
                    ],
                    &[
                        "`Sec-Fetch-Site`",
                        "Not set. The router passes a request without it.",
                    ],
                ],
                ja: &[
                    &[
                        "`Accept`",
                        "`text/event-stream`。[`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) はこのヘッダーで、呼び出し（`RequestKind::Runtime`）とページの読み込み（`RequestKind::Navigation`）を見分けます",
                    ],
                    &[
                        "`hypersig-call`",
                        "ランタイムがすべての呼び出しに付けるヘッダー",
                    ],
                    &[
                        "`POST` の本文",
                        "JSON にした引数。`Content-Type: application/json` が付きます",
                    ],
                    &[
                        "`GET` の URL",
                        "`args` のクエリパラメータに入れた JSON の引数。`()` のときは付きません",
                    ],
                    &[
                        "`Sec-Fetch-Site`",
                        "付きません。ルーターはこのヘッダーのないリクエストを通します。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The router refuses a `POST` without that `Content-Type`, a `GET` without `hypersig-call`, and a `Sec-Fetch-Site` other than `same-origin`, as `a_call_from_another_site_is_refused` shows with `cross-site` and a 403. [Security](learn/security/) lists every refusal with its status. The body of an answered action is the batch as Server-Sent Events, which [`Patches::to_event_stream`](api:hypersig/struct.Patches.html#method.to_event_stream) writes in the same form. A test compares the body with the batch it expects, as `the_router_answers_the_action_with_its_patches` does, in place of a text written by hand.",
                ja: "ルーターは、その `Content-Type` のない `POST`、`hypersig-call` のない `GET`、`same-origin` 以外の `Sec-Fetch-Site` を拒否します。`a_call_from_another_site_is_refused` は `cross-site` を付けて 403 を確かめています。拒否の一覧とステータスは [セキュリティ](learn/security/) にあります。応答したアクションの本文は差分を Server-Sent Events にしたもので、[`Patches::to_event_stream`](api:hypersig/struct.Patches.html#method.to_event_stream) が同じ形式で書きます。`the_router_answers_the_action_with_its_patches` のように、テストは手で書いた文字列の代わりに、期待する差分と本文を比べます。",
            },
        ),
    ])
}

/// A handler that takes an extractor, called directly and through the router.
fn extractors(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "A handler that takes an extractor",
                ja: "エクストラクタを受け取るハンドラ",
            },
        ),
        para(
            site,
            L {
                en: "A handler that takes an extractor, such as the `Session` of [Signing in](learn/sessions/), is called with a value the test builds, as the tests below do for that page's `sign_in`. `Session::new(None, Arc::new(MemoryStore::default()), None)` makes an empty session with a store of its own, and the test reads what the handler wrote with `Session::get`. What the layer adds, the cookie in the answer, shows only through the router, so a second test sends the action through `oneshot` on the router with its `SessionManagerLayer`.",
                ja: "[サインインとセッション](learn/sessions/) の `Session` のようなエクストラクタを受け取るハンドラは、テストが作った値を渡して呼び出します。下のテストはそのページの `sign_in` をこうして呼びます。`Session::new(None, Arc::new(MemoryStore::default()), None)` は自分専用のストアを持つ空のセッションを作り、テストはハンドラが書いた値を `Session::get` で読みます。レイヤーが加えるもの（応答の Cookie）はルーターを通さないと現れないので、2 つ目のテストは `SessionManagerLayer` を付けたルーターに `oneshot` でアクションを送ります。",
            },
        ),
        show(Snippet::TestingSessions),
    ])
}

/// What these tests do not check.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What these tests leave out",
                ja: "これらのテストで確かめられないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A handler called as a function skips the router: its extractors, the refusals of a request from another site, and the reading of the arguments from JSON. Test those through `oneshot`.",
                    ja: "関数として呼んだハンドラはルーターを通りません。エクストラクタ、別のサイトからのリクエストの拒否、JSON からの引数の読み込みは起きないので、これらは `oneshot` で確かめます。",
                },
                L {
                    en: "A `routes!` block whose generated function the application never calls compiles, and its pages and actions answer 404. A router test that expects a 200 from `LIKE.request(&3)` fails in that case.",
                    ja: "生成された関数をアプリが呼ばない `routes!` のブロックもコンパイルは通り、そのページとアクションは 404 を返します。`LIKE.request(&3)` に 200 を期待するルーターのテストは、そのとき失敗します。",
                },
                L {
                    en: "`Page::value_of` and `Patches::value_of` panic where the value does not deserialize into the signal's type, which happens where a `Serialize` impl skips the field or disagrees with its `Deserialize` impl.",
                    ja: "`Page::value_of` と `Patches::value_of` は、値がシグナルの型にデシリアライズできないときにパニックします。`Serialize` の実装がフィールドを飛ばすときや、`Deserialize` の実装と食い違うときに起きます。",
                },
                L {
                    en: "What the page does with the answer: the runtime applying the batch, and the statements that run when a signal changes. [Testing a page without a browser](learn/testing-pages/) covers the events, and a browser test the rest.",
                    ja: "ページが応答をどう扱うか。ランタイムが差分を当てる処理と、シグナルが変わったときに動く文です。イベントは [ブラウザなしでページをテストする](learn/testing-pages/) で、残りはブラウザのテストで確かめます。",
                },
            ],
        ),
    ])
}

/// The pages to read next.
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
            &[
                L {
                    en: "[Testing a page without a browser](learn/testing-pages/): clicking and typing on a page in a test.",
                    ja: "[ブラウザなしでページをテストする](learn/testing-pages/): テストの中でページをクリックし、入力します。",
                },
                L {
                    en: "[Security](learn/security/): what the router refuses before a handler runs.",
                    ja: "[セキュリティ](learn/security/): ハンドラが動く前にルーターが拒否するもの。",
                },
                L {
                    en: "[Debugging a running page](learn/debugging/): what the server and the browser report when something fails.",
                    ja: "[動いているページを調べる](learn/debugging/): 何かが失敗したときにサーバーとブラウザが報告するもの。",
                },
            ],
        ),
    ])
}
