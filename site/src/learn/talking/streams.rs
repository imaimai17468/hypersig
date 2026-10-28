//! The page on a response that stays open and sends patches as changes
//! happen on the server.

use super::needs_a_server;
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::learn::{Program, program};
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Streams).title(site)),
        lead(site, Doc::Learn(Learn::Streams).leads()),
        declaring(site),
        opening(site),
        network(site),
        connection(site),
        reconnect(site),
        unchecked(site),
        example(site),
    ])
}

/// The `stream` line of `routes!` and its handler.
fn declaring(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Declaring a stream",
                ja: "ストリームを宣言する",
            },
        ),
        para(
            site,
            L {
                en: "A [`StreamAction<S, A = ()>`](api:hypersig/struct.StreamAction.html), declared on a `stream` line of `routes!`, runs a handler that returns a stream of patches, and is always served on `GET`: `stream TICKS: StreamAction<Clock> = \"/ticks\" => ticks;`. Its path follows the rules of an action's path. The handler is a plain `fn(state, args)` or `fn(state, extractor, args)` that returns an `impl Stream<Item = Patches<S>> + Send + 'static`, such as one `futures::stream::unfold` builds. Each item of the stream is one batch, sent when the stream yields it.",
                ja: "`routes!` の `stream` の行に宣言した [`StreamAction<S, A = ()>`](api:hypersig/struct.StreamAction.html) は、差分のストリームを返すハンドラを動かします。配信するメソッドは常に `GET` です。`stream TICKS: StreamAction<Clock> = \"/ticks\" => ticks;` のように書きます。パスの規則はアクションのパスと同じです。ハンドラは `fn(state, args)` か `fn(state, extractor, args)` のふつうの関数で、`futures::stream::unfold` で作るような `impl Stream<Item = Patches<S>> + Send + 'static` を返します。ストリームの各要素が 1 つのまとまりで、ストリームがそれを出した時点で送られます。",
            },
        ),
        para(
            site,
            L {
                en: "An `async fn` returns a future of the stream rather than a stream, so it is a compile error, E0277, at the `routes!` line that names it.",
                ja: "`async fn` はストリームではなく、ストリームを返す future を返すので、それを指定した `routes!` の行でコンパイルエラー E0277 になります。",
            },
        ),
        show(Snippet::TalkStreamAsync),
    ])
}

/// `call`, `call_with`, and what they write into the HTML.
fn opening(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Opening a stream",
                ja: "ストリームを開く",
            },
        ),
        para(
            site,
            L {
                en: "[`StreamAction::call`](api:hypersig/struct.StreamAction.html#method.call), `fn call(&self) -> StreamCall<S>`, exists only on a `StreamAction<S, ()>`, and [`call_with`](api:hypersig/struct.StreamAction.html#method.call_with), `fn call_with(&self, args: impl IntoExpr<S, A>) -> StreamCall<S>`, takes an expression the browser evaluates when the stream opens. A [`StreamCall<S>`](api:hypersig/struct.StreamCall.html) is a statement. Put it in `on_load` to open the stream when the element appears, which suits changes made elsewhere, such as by other visitors.",
                ja: "[`StreamAction::call`](api:hypersig/struct.StreamAction.html#method.call)（`fn call(&self) -> StreamCall<S>`）は `StreamAction<S, ()>` にだけあり、[`call_with`](api:hypersig/struct.StreamAction.html#method.call_with)（`fn call_with(&self, args: impl IntoExpr<S, A>) -> StreamCall<S>`）は、ストリームを開くときにブラウザが評価する式を受け取ります。[`StreamCall<S>`](api:hypersig/struct.StreamCall.html) は文です。`on_load` に置けば要素が現れたときにストリームを開くので、ほかの訪問者など別の場所で起きた変更を受け取るのに向いています。",
            },
        ),
        para(
            site,
            L {
                en: "The stream closes once its element leaves the page, and the browser applies none of its later patches. A patch that replaces the element leaves one stream open, the one the new element's `on_load` opens.",
                ja: "要素がページから外れるとストリームは閉じ、ブラウザはそのあとの差分を適用しません。差分が要素を置き換えたときに開いているストリームは 1 つだけで、新しい要素の `on_load` が開いたものです。",
            },
        ),
    ])
}

/// The request and the response that stays open.
fn network(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What goes over the network",
                ja: "ネットワークを流れるもの",
            },
        ),
        para(
            site,
            L {
                en: "Opening the stream sends the `GET` an action call sends: `accept: text/event-stream`, `hypersig-call: 1`, and the arguments as JSON in the `args` query parameter. The answer is `200` with `content-type: text/event-stream` and `x-accel-buffering: no`, which stops nginx from holding the patches back in its buffer. It stays open while the handler's stream runs. Each batch arrives as soon as the stream yields it, and while none comes the server sends a comment line now and then, so a proxy does not close a quiet connection. When the handler's stream finishes, the server ends the response. The network tab shows the request of the clock below as:",
                ja: "ストリームを開くと、アクションの呼び出しと同じ `GET` が送られます。ヘッダーは `accept: text/event-stream` と `hypersig-call: 1` で、引数は JSON にして `args` クエリパラメータに入ります。応答は `200` で、`content-type: text/event-stream` と `x-accel-buffering: no` が付きます。後者は、nginx が差分をバッファに溜めて遅らせないようにするヘッダーです。応答はハンドラのストリームが動いている間開いたままです。ストリームが出したまとまりは、出した時点で届きます。まとまりが来ない間は、静かな接続をプロキシが閉じないよう、サーバーがときどきコメント行を送ります。ハンドラのストリームが終わると、サーバーは応答を終えます。下の時計のリクエストはネットワークタブでは次のように見えます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "GET /ticks\naccept: text/event-stream\nhypersig-call: 1",
        ),
        para(
            site,
            L {
                en: "[`StreamAction::request`](api:hypersig/struct.StreamAction.html#method.request), `fn request(&self, args: &A) -> Request<Body>`, builds the same `GET` for a test.",
                ja: "[`StreamAction::request`](api:hypersig/struct.StreamAction.html#method.request)（`fn request(&self, args: &A) -> Request<Body>`）は、テストのために同じ `GET` を作ります。",
            },
        ),
    ])
}

/// `connected`, `error`, and `status` on a stream.
fn connection(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Showing the connection",
                ja: "接続の状態を表示する",
            },
        ),
        para(
            site,
            L {
                en: "[`StreamCall::connected`](api:hypersig/struct.StreamCall.html#method.connected), `fn connected(self, flag: Sig<S, bool>) -> Self`, sets `flag` to true when the server answers with a 2xx status and to false when the stream ends or breaks off. It stays false while the stream connects and while `reconnect` waits to open it again, so a page can show whether it is receiving changes. Several streams may set one flag, which is true while any of them is open. A stream has no `indicator`, because its answer is never awaited to the end; calling it is E0599. `error` and `status` work as on an action's call, and also go back to `\"\"` and `0` when the stream connects.",
                ja: "[`StreamCall::connected`](api:hypersig/struct.StreamCall.html#method.connected)（`fn connected(self, flag: Sig<S, bool>) -> Self`）は、サーバーが 2xx で応答したときに `flag` を真にし、ストリームが終わるか途中で切れたときに偽にします。接続中と、`reconnect` が開き直すまで待つ間は偽のままなので、ページは変更を受け取っているかどうかを表示できます。複数のストリームで 1 つのフラグを使うこともでき、その場合はどれかが開いている間だけ真になります。ストリームの応答は最後まで待つものではないので、ストリームには `indicator` がなく、呼ぶと E0599 になります。`error` と `status` はアクションの呼び出しと同じように動き、ストリームが接続したときにも `\"\"` と `0` に戻ります。",
            },
        ),
    ])
}

/// `reconnect`, and why only a stream has it.
fn reconnect(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Streams that drop",
                ja: "切れたストリーム",
            },
        ),
        para(
            site,
            L {
                en: "[`StreamCall::reconnect`](api:hypersig/struct.StreamCall.html#method.reconnect), `fn reconnect(self) -> Self`, opens a stream again when its connection fails, it breaks off, or the server answers with a 5xx status, as the clock below does. It waits longer after each failure in a row, for a random part of each wait, so pages that lost the server together do not all come back at the same moment, and it stops waiting when the browser comes back online or the page becomes visible. A stream that delivers a batch starts the waits over. A response a host closes at its time limit ends before the handler's stream finished, so it counts as broken off. `reconnect` does not open the stream again once the handler's stream has finished, after a 4xx answer, or once the element has left the page. While the stream is failing, the `error` signal holds the last failure.",
                ja: "[`StreamCall::reconnect`](api:hypersig/struct.StreamCall.html#method.reconnect)（`fn reconnect(self) -> Self`）は、接続が失敗したとき、ストリームが途中で切れたとき、サーバーが 5xx を返したときに、ストリームを開き直します。下の時計がこの例です。失敗が続くほど長く待ち、待つ長さには乱数が入るので、同時にサーバーを失ったページが一斉に戻ることはありません。ブラウザがオンラインに戻ったときや、ページが表示されたときは、待つのをやめてすぐに開き直します。まとまりを 1 つでも届けたストリームは、待ち時間を最初からやり直します。ホストが時間の上限で閉じたレスポンスは、ハンドラのストリームが終わる前に切れるので、途中で切れたものとして扱います。ハンドラのストリームが終わったとき、4xx の応答を受けたとき、要素がページから外れたときは、開き直しません。ストリームが失敗し続けている間、`error` のシグナルには最後の失敗が入ります。",
            },
        ),
        para(
            site,
            L {
                en: "Each attempt evaluates the arguments again and runs the handler with them. A stream that takes a signal its patches keep current, such as the id of the last message, can send only what came after it. A stream whose arguments stay the same should start with the current values, and the page catches up with what it missed.",
                ja: "開き直すたびに引数を評価し直し、その引数でハンドラを実行します。最後に受け取ったメッセージの id のように、ストリームの差分が更新し続けるシグナルを引数に渡せば、ハンドラはそれより後の分だけを送れます。引数が変わらないストリームでは、初めに現在の値を送ってください。そうすればページは見逃した変更に追いつきます。",
            },
        ),
        show(Snippet::TalkSince),
        para(
            site,
            L {
                en: "Only a call to a `StreamAction`, which is always served on `GET`, has `reconnect`, because sending a `POST` again could repeat a change. On an action's `Call` it is a compile error, E0599.",
                ja: "`POST` を送り直すと変更が繰り返されることがあるので、`reconnect` があるのは、常に `GET` で配信する `StreamAction` の呼び出しだけです。アクションの `Call` で呼ぶとコンパイルエラー E0599 になります。",
            },
        ),
        show(Snippet::TalkReconnectOnAction),
    ])
}

/// What the types leave unchecked.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型で検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "Nothing limits how long a stream stays open or how many visitors hold one. Each open stream is one request held by the server, and a host that ends requests after a time limit closes the stream there, as [Deploying in a container](learn/deploying/container/) measures on Cloud Run. A stream without `reconnect` that drops stays closed, and only `connected` or `error` shows it. A stream hears of a change another visitor made through something the application provides, such as a `tokio::sync::broadcast` channel in the router's state, which reaches the streams of one process only. [Storing data](learn/database/#updates-with-more-than-one-process) covers an application that runs more than one process.",
                ja: "ストリームが開いている時間や、ストリームを開いている訪問者の数には上限がありません。開いているストリームは 1 つずつ、サーバーが保持するリクエストです。一定の時間でリクエストを終えるホストでは、その時間でストリームが閉じます。Cloud Run での計測は [コンテナでデプロイする](learn/deploying/container/) にあります。`reconnect` のないストリームは切れたら閉じたままで、それを示すのは `connected` か `error` だけです。ほかの訪問者が加えた変更をストリームが知るには、ルーターの状態に置いた `tokio::sync::broadcast` のチャネルなど、アプリケーションが用意する仕組みを使います。このチャネルが届くのは 1 つのプロセスのストリームだけです。複数のプロセスで動かすアプリケーションについては [データを保存する](learn/database/#updates-with-more-than-one-process) で説明します。",
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
                en: "This clock counts the seconds the page has been connected, sent by the server once a second. The stream below uses the `futures` crate and tokio's timer, so add `futures` to the Quick start's `[dependencies]` and turn on tokio's `time` feature:",
                ja: "この時計は、ページが接続している秒数を数え、サーバーが 1 秒ごとに送ります。下のストリームは `futures` クレートと tokio のタイマーを使うので、クイックスタートの `[dependencies]` に `futures` を加え、tokio の `time` フィーチャーを有効にしてください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"futures = "0.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net", "time"] }"#,
        ),
        program(Program::Streams),
        needs_a_server(site),
        para(
            site,
            L {
                en: "Next, [List patches](learn/list-patches/) sends a chat's messages one at a time, and [Testing handlers](learn/testing/) tests an action and a stream without a browser.",
                ja: "次はチャットのメッセージを 1 件ずつ送る [リストの差分](learn/list-patches/) と、ブラウザなしでアクションやストリームをテストする [ハンドラのテスト](learn/testing/) です。",
            },
        ),
    ])
}
