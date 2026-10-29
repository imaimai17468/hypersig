//! The page on declaring an action, calling it, and what the request and the
//! answer carry.

use super::needs_a_server;
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::learn::{Program, program};
use crate::prose::{Part, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Actions).title(site)),
        lead(site, Doc::Learn(Learn::Actions).leads()),
        declaring(site),
        calling(site),
        request(site),
        answer(site),
        refused(site),
        unchecked(site),
        example(site),
    ])
}

/// The `action` line of `routes!` and the handler it names.
fn declaring(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Declaring an action",
                ja: "アクションを宣言する",
            },
        ),
        para(
            site,
            L {
                en: "An [`Action<S, A = ()>`](api:hypersig/struct.Action.html) is a server endpoint for the page whose signals are `S`, and it receives arguments of type `A`. Declare it on an `action` line of `routes!`, in the block that declares the pages, with its method, its path, and the handler that serves it: `action LIKE: Action<Review> = post \"/like\" => like;`. The method is `post` or `get`. The function the block generates registers each handler, so an action without a handler does not compile: `routes!` reports the form each line takes. A path that does not start with `/`, holds a `{name}` segment, or holds a byte the browser percent-encodes is a compile error too.",
                ja: "[`Action<S, A = ()>`](api:hypersig/struct.Action.html) は、シグナルが `S` のページのためのサーバーのエンドポイントで、型 `A` の引数を受け取ります。ページを宣言する `routes!` のブロックに `action` の行を書き、メソッド、パス、配信するハンドラと一緒に宣言します。`action LIKE: Action<Review> = post \"/like\" => like;` のように書きます。メソッドは `post` か `get` です。ブロックが生成する関数が各ハンドラを登録するので、ハンドラのないアクションはコンパイルできません。そのときは `routes!` が各行の書き方をエラーで示します。`/` で始まらないパス、`{name}` のセグメントを含むパス、ブラウザがパーセントエンコードするバイトを含むパスもコンパイルエラーです。",
            },
        ),
        show(Snippet::TalkDeclare),
        para(
            site,
            L {
                en: "A handler is `async fn(state, args)`, or `async fn(state, extractor, args)` where the extractor implements axum's `FromRequestParts` for the router's state, such as a signed-in member read from a cookie. The arguments arrive deserialized into `A`. A handler that needs several values takes a tuple, which the page passes as a tuple of expressions. [Extractors and headers](learn/requests/) covers the extractor.",
                ja: "ハンドラは `async fn(state, args)` か、`async fn(state, extractor, args)` です。extractor はルーターの状態に対して axum の `FromRequestParts` を実装する型で、たとえばクッキーから読んだサインイン中の会員です。引数は `A` にデシリアライズされた状態で届きます。複数の値が必要なハンドラはタプルを受け取り、ページは式のタプルを渡します。extractor は [エクストラクタとヘッダー](learn/requests/) で説明します。",
            },
        ),
    ])
}

/// `call` and `call_with`, and what they write into the HTML.
fn calling(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Calling an action",
                ja: "アクションを呼び出す",
            },
        ),
        para(
            site,
            L {
                en: "[`call`](api:hypersig/struct.Action.html#method.call), `fn call(&self) -> Call<S>`, exists only on an `Action<S, ()>`. [`call_with`](api:hypersig/struct.Action.html#method.call_with), `fn call_with(&self, args: impl IntoExpr<S, A>) -> Call<S>`, takes the arguments as an expression, which the browser evaluates when the call runs. It can be a literal such as `&id`, a signal, or a tuple of either. The [`Call<S>`](api:hypersig/struct.Call.html) it returns is a statement, so it goes wherever a statement goes: `on_click`, `on_submit`, `on_load`, or after `then`.",
                ja: "[`call`](api:hypersig/struct.Action.html#method.call)（`fn call(&self) -> Call<S>`）は `Action<S, ()>` にだけあります。[`call_with`](api:hypersig/struct.Action.html#method.call_with)（`fn call_with(&self, args: impl IntoExpr<S, A>) -> Call<S>`）は引数を式として受け取り、ブラウザが呼び出しを実行するときにその式を評価します。式には `&id` のようなリテラル、シグナル、それらのタプルを渡せます。返る [`Call<S>`](api:hypersig/struct.Call.html) は文なので、`on_click`、`on_submit`、`on_load`、`then` のあとなど、文を書ける場所ならどこにでも置けます。",
            },
        ),
        show(Snippet::TalkCall),
        para(
            site,
            L {
                en: "The page never names the action's URL: the call takes its method and path from the `Action` constant, so the page and the router cannot disagree on them.",
                ja: "ページにアクションの URL を書くことはありません。呼び出しはメソッドとパスを `Action` の定数から取るので、ページとルーターで食い違うことはありません。",
            },
        ),
        subheading(
            site,
            L {
                en: "Arguments of the wrong type",
                ja: "型の違う引数",
            },
        ),
        para(
            site,
            L {
                en: "An expression whose type is not `A` is a compile error, E0277. So is the call of an action declared for another page's signals, because a `Call<Other>` is not a statement on a page whose signals are `Review`. `call()` on an action that takes arguments is E0599, because the method does not exist there.",
                ja: "型が `A` ではない式を渡すと、コンパイルエラー E0277 になります。別のページのシグナル用に宣言したアクションを呼ぶことも同じエラーになります。`Call<Other>` は、シグナルが `Review` のページの文ではないからです。引数を取るアクションで `call()` を書くと、そのメソッドがないので E0599 になります。",
            },
        ),
        show(Snippet::TalkWrongArgs),
    ])
}

/// The HTTP request a call sends.
fn request(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the request carries",
                ja: "リクエストの中身",
            },
        ),
        para(
            site,
            L {
                en: "Each call is one `fetch` to the action's path, with the headers `accept: text/event-stream` and `hypersig-call: 1`. A `POST` sends the arguments as its JSON body with `content-type: application/json`, and an action without arguments sends the body `null`. A `GET` sends them as JSON in the `args` query parameter, and leaves the parameter out when there are none. A call made inside a [component](learn/components/) that is a `Part` adds the header `hypersig-part`. The browser's network tab shows these two requests for a rating and a search:",
                ja: "呼び出しは 1 回ごとに、アクションのパスへの `fetch` になります。ヘッダーは `accept: text/event-stream` と `hypersig-call: 1` です。`POST` は引数を JSON の本文として `content-type: application/json` で送り、引数のないアクションは本文 `null` を送ります。`GET` は引数を JSON にして `args` クエリパラメータで送り、引数がなければこのパラメータを付けません。`Part` である [コンポーネント](learn/components/) の中の呼び出しには、ヘッダー `hypersig-part` が加わります。評価と検索の呼び出しは、ブラウザのネットワークタブでは次のように見えます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "POST /rate\naccept: text/event-stream\nhypersig-call: 1\ncontent-type: application/json\n\n[5,\"Lovely\"]\n\nGET /find?args=%22tea%22\naccept: text/event-stream\nhypersig-call: 1",
        ),
        para(
            site,
            L {
                en: "Use `get` for a call that only reads, such as a search, and `post` for one that changes data. The two differ in the browser as well: only a `POST` marks its element `aria-busy` while it waits, and only for a `POST` are the late patches of an earlier call dropped, as [Calls that overlap](learn/overlapping-calls/) shows.",
                ja: "検索のように読み取るだけの呼び出しには `get` を、データを変える呼び出しには `post` を使います。ブラウザでの動きも違います。応答を待つ間に要素へ `aria-busy` を付けるのは `POST` だけで、前の呼び出しの遅れて届いた差分を捨てるのも `POST` だけです。詳しくは [重なった呼び出し](learn/overlapping-calls/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "[`Action::request`](api:hypersig/struct.Action.html#method.request), `fn request(&self, args: &A) -> Request<Body>`, builds the request the browser sends, for a test that hands it to the router with `oneshot`, as on [Testing handlers](learn/testing/). It panics if `args` does not serialize to JSON.",
                ja: "[`Action::request`](api:hypersig/struct.Action.html#method.request)（`fn request(&self, args: &A) -> Request<Body>`）は、ブラウザが送るものと同じリクエストを作ります。[ハンドラのテスト](learn/testing/) のように、テストから `oneshot` でルーターに渡すために使います。`args` を JSON にシリアライズできないときはパニックします。",
            },
        ),
        show(Snippet::TalkRequest),
    ])
}

/// What a handler returns and the response it becomes.
fn answer(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the handler answers",
                ja: "ハンドラの応答",
            },
        ),
        para(
            site,
            L {
                en: "A handler returns a type that implements [`IntoActionResponse<S>`](api:hypersig/trait.IntoActionResponse.html): `Patches<S>`, a `SignalPatch<S>`, a `ListPatch<S>`, a tuple of response parts and one of those, such as a header that sets a cookie, or a `Result` of one of those whose error implements `IntoResponse`. The answer is `200` with `content-type: text/event-stream`, and its body carries the patches in order. An `Err` answers with the error's own response, and a handler that panics answers `500` with an empty body. [Patches](learn/patches/) covers the patches, and [Errors and logging](learn/errors/) covers errors.",
                ja: "ハンドラは [`IntoActionResponse<S>`](api:hypersig/trait.IntoActionResponse.html) を実装する型を返します。`Patches<S>`、`SignalPatch<S>`、`ListPatch<S>`、レスポンスのパーツ（クッキーを設定するヘッダーなど）とそれらのどれかのタプル、またはエラーが `IntoResponse` を実装する、それらのどれかの `Result` です。応答は `200` で、`content-type: text/event-stream` です。本文には差分が順に入ります。`Err` ではエラー自身のレスポンスを返し、ハンドラがパニックしたときは本文が空の `500` を返します。差分は [差分](learn/patches/) で、エラーは [エラーとログ](learn/errors/) で説明します。",
            },
        ),
        show(Snippet::TalkReply),
        para(
            site,
            L {
                en: "A handler that returns anything else, such as the `String` it was given, is a compile error, E0277, at the `routes!` line that names it.",
                ja: "受け取った `String` をそのまま返すなど、ほかの型を返すハンドラは、それを指定した `routes!` の行でコンパイルエラー E0277 になります。",
            },
        ),
        show(Snippet::TalkWrongReply),
    ])
}

/// The requests the router answers before the handler runs.
fn refused(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Requests the router refuses",
                ja: "ルーターが断るリクエスト",
            },
        ),
        para(
            site,
            L {
                en: "The router answers these requests itself, and the handler does not run. A request without `Sec-Fetch-Site` passes the check in the first row. Each body is one fixed sentence in `text/plain`, which the page receives as a failed call, as [Busy and failed calls](learn/call-status/) describes.",
                ja: "次のリクエストにはルーターがみずから応答し、ハンドラは動きません。`Sec-Fetch-Site` のないリクエストは 1 行目の検査を通ります。本文はそれぞれ決まった 1 文の `text/plain` で、ページはこれを失敗した呼び出しとして受け取ります。失敗の扱いは [呼び出し中と失敗](learn/call-status/) で説明します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Request", "Status", "Body"],
                ja: &["リクエスト", "ステータス", "本文"],
            },
            L {
                en: &[
                    &[
                        "`Sec-Fetch-Site` other than `same-origin`, so a page on another site, or on a sibling subdomain",
                        "403",
                        "the action was called from another site",
                    ],
                    &[
                        "A `GET` without `hypersig-call`, such as a link or an `<img>` pointed at the action",
                        "403",
                        "the call lacks the `hypersig-call` header the hypersig runtime adds",
                    ],
                    &[
                        "A `POST` whose `content-type` is not `application/json`, such as a plain HTML form",
                        "415",
                        "the action's body is not `application/json`",
                    ],
                    &[
                        "Arguments that do not deserialize into `A`",
                        "400",
                        "the action's arguments did not deserialize into its argument type",
                    ],
                    &[
                        "A `hypersig-part` header that is not a dotted field path",
                        "400",
                        "the call's part header is not a dotted field path",
                    ],
                ],
                ja: &[
                    &[
                        "`Sec-Fetch-Site` が `same-origin` 以外。別のサイトや、兄弟のサブドメインのページから送られたもの",
                        "403",
                        "the action was called from another site",
                    ],
                    &[
                        "`hypersig-call` のない `GET`。アクションを指すリンクや `<img>` など",
                        "403",
                        "the call lacks the `hypersig-call` header the hypersig runtime adds",
                    ],
                    &[
                        "`content-type` が `application/json` ではない `POST`。ふつうの HTML フォームなど",
                        "415",
                        "the action's body is not `application/json`",
                    ],
                    &[
                        "`A` にデシリアライズできない引数",
                        "400",
                        "the action's arguments did not deserialize into its argument type",
                    ],
                    &[
                        "ドットで区切ったフィールドのパスではない `hypersig-part` ヘッダー",
                        "400",
                        "the call's part header is not a dotted field path",
                    ],
                ],
            },
        ),
    ])
}

/// What the types leave to the handler and to the application.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What actions leave unchecked",
                ja: "アクションが検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "The type of `A` holds, and its values do not. The visitor's browser can send any value of `A`, whatever the page's expressions would compute, and anyone can call the action's URL without loading the page. A handler that guards the page does not guard its actions, so each handler checks that the visitor may act on the arguments it receives, such as the owner of an order whose id it takes. [Security](learn/security/) covers what the router checks for you.",
                ja: "`A` の型は守られますが、値は守られません。訪問者のブラウザは、ページの式が計算する値とは関係なく、`A` のどんな値でも送れます。ページを読み込まずにアクションの URL を呼ぶこともできます。ページを守るハンドラはアクションを守らないので、各ハンドラで、受け取った引数を訪問者が操作してよいかを確かめてください。たとえば id を受け取る注文なら、その注文の持ち主かどうかを確かめます。ルーターが確かめることは [セキュリティ](learn/security/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "The compiler checks that each action has a handler, and two mistakes stay unchecked. The first is a `routes!` block whose generated function the application never calls. Its pages answer 404, and each call to its actions, streams, and uploads gets a 404 with an empty body, which the browser writes to the console and `error` receives as a failed call. A call without `error` changes nothing on the page. rustc warns about a handler `fn` that only the uncalled function names, and not about a closure. The second is a page that calls an action of another block whose `under` names a different [`Base`](api:hypersig/struct.Base.html). The call goes to that block's prefix, which answers 404 unless the other block is served there too. Declaring every page and action in one block avoids both, because the application then calls one function and every line shares one base. A test that sends `LIKE.request(&())` to the application's router with `oneshot` and expects a 200, as on [Testing handlers](learn/testing/), fails when the function is not called.",
                ja: "コンパイラはアクションごとにハンドラがあることを検査します。検査されない間違いは 2 つ残ります。1 つ目は生成された関数をアプリケーションが呼ばない `routes!` のブロックです。そのブロックのページは 404 を返し、アクション、ストリーム、アップロードへの呼び出しはどれも本文が空の 404 を受け取ります。ブラウザはこれをコンソールに書き、`error` は失敗した呼び出しとして受け取ります。`error` のない呼び出しでは、ページは何も変わりません。呼ばれない関数の中でだけ使うハンドラの `fn` には rustc が警告し、クロージャには警告しません。2 つ目は `under` に別の [`Base`](api:hypersig/struct.Base.html) を書いたブロックのアクションをページが呼ぶ場合です。呼び出しはそのブロックの接頭辞に送られ、そのブロックもそこで配信していなければ 404 が返ります。すべてのページとアクションを 1 つのブロックに宣言すれば、どちらも起きません。アプリケーションが呼ぶ関数は 1 つになり、すべての行が同じ接頭辞を使うからです。[ハンドラのテスト](learn/testing/) の例のように `LIKE.request(&())` をアプリケーションのルーターに `oneshot` で送って 200 を期待するテストは、関数を呼んでいなければ失敗します。",
            },
        ),
    ])
}

/// The complete program, and the pages that follow.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A complete example",
                ja: "完全な例",
            },
        ),
        para(
            site,
            L {
                en: "This profile page saves a name with an argument and counts likes on the server. The save button is disabled while its call waits, the error text shows under the form, and the like count goes up before the server answers.",
                ja: "このプロフィールのページは引数付きの呼び出しで名前を保存し、サーバーで「いいね」を数えます。保存の呼び出しが応答を待つ間はボタンを無効にし、エラーの文言をフォームの下に表示し、「いいね」の数はサーバーの応答より先に増やします。",
            },
        ),
        program(Program::Actions),
        needs_a_server(site),
        para(
            site,
            L {
                en: "Next, [Busy and failed calls](learn/call-status/) covers `indicator` and `error`, [Optimistic updates](learn/optimistic-updates/) covers the change before the call, and [Patches](learn/patches/) covers what the handler sends back.",
                ja: "次に読むページは `indicator` と `error` を扱う [呼び出し中と失敗](learn/call-status/)、呼び出しの前に画面を変える [楽観的更新](learn/optimistic-updates/)、ハンドラが返すものを扱う [差分](learn/patches/) です。",
            },
        ),
    ])
}
