//! The page on a call sent while an earlier call to the same action waits.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::OverlappingCalls).title(site)),
        lead(site, Doc::Learn(Learn::OverlappingCalls).leads()),
        latest(site),
        order(site),
        exclusive(site),
        timeout(site),
        unchecked(site),
        example(site),
    ])
}

/// `latest`, for reads whose newest answer is the one to show.
fn latest(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The newest search wins",
                ja: "最新の検索を採る",
            },
        ),
        para(
            site,
            L {
                en: "Each call applies its patches when they arrive, so the answer to an earlier, slower search can overwrite the answer to a later one. [`Call::latest`](api:hypersig/struct.Call.html#method.latest), `fn latest(self) -> Self`, aborts the unfinished earlier `latest` call to the same action, and the patches it has not received are dropped. The calls of every element to one action share it, because the handler decides which signals they patch, and a call made inside a component that is a `Part` shares it with that part's calls only.",
                ja: "呼び出しはそれぞれ差分が届いた時点で反映するので、前に送った遅い検索の応答が、あとの検索の応答を上書きすることがあります。[`Call::latest`](api:hypersig/struct.Call.html#method.latest)（`fn latest(self) -> Self`）は、同じアクションへの `latest` の呼び出しで、まだ終わっていない前のものを取り消します。取り消した呼び出しでまだ届いていない差分は捨てられます。アクションがどのシグナルに差分を返すかはハンドラが決めるので、別々の要素から同じアクションを呼んでも、取り消しの対象は共通です。`Part` であるコンポーネントの中の呼び出しは、その部品の呼び出しとだけ共通です。",
            },
        ),
        show(Snippet::Latest),
        para(
            site,
            L {
                en: "The browser aborts the earlier request, and the server may then stop its handler at an `.await`, so put `latest` only on calls that read. A later call never aborts a call without `latest`. The aborted call runs neither `on_error` nor `on_success`, and an indicator stays true until every call that sets it has its answer.",
                ja: "ブラウザは前のリクエストを中断し、サーバーはそのハンドラを `.await` の時点で止めることがあります。そのため、`latest` は読み取るだけの呼び出しに付けてください。`latest` のない呼び出しが、あとの呼び出しに取り消されることはありません。取り消された呼び出しでは `on_error` も `on_success` も実行されません。indicator は、それを使う呼び出しがすべて応答を受け取るまで真のままです。",
            },
        ),
    ])
}

/// The order a `POST`'s answers apply in.
fn order(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Calls that change data keep their order",
                ja: "データを変える呼び出しは順序を保つ",
            },
        ),
        para(
            site,
            L {
                en: "A call that changes data goes without `latest`, and its answers apply in order instead: once a later `POST` to the action has its complete answer, the patches of an earlier one that are still arriving are dropped. Both requests reach the server and both handlers run, so the data holds both changes, and the page shows the answer to the later one.",
                ja: "データを変える呼び出しには `latest` を付けません。代わりに、応答は順序どおりに反映されます。同じアクションへのあとの `POST` が応答を最後まで受け取ると、前の `POST` でまだ届いている差分は捨てられます。どちらのリクエストもサーバーに届き、どちらのハンドラも動くので、データには両方の変更が入り、ページにはあとの呼び出しへの応答が表示されます。",
            },
        ),
    ])
}

/// `exclusive`, for a button that must not send twice.
fn exclusive(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "One call at a time",
                ja: "一度に 1 つの呼び出し",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::exclusive`](api:hypersig/struct.Call.html#method.exclusive), `fn exclusive(self) -> Self`, skips a call while another call to the same action awaits its answer, so a second click on an order button sends nothing while the first order is on its way. The skipped call sets no indicator and no error, and runs neither `on_error` nor `on_success`. A call made inside a component that is a `Part` waits only for that part's calls. While a `POST` waits, the element that sent it carries `aria-busy=\"true\"`, with or without `exclusive`.",
                ja: "[`Call::exclusive`](api:hypersig/struct.Call.html#method.exclusive)（`fn exclusive(self) -> Self`）は、同じアクションへの別の呼び出しが応答を待っている間、その呼び出しを送りません。注文のボタンを 2 回押しても、最初の注文の応答を待つ間は、2 回目は何も送りません。送らなかった呼び出しは indicator も error も設定せず、`on_error` も `on_success` も実行しません。`Part` であるコンポーネントの中の呼び出しは、その部品の呼び出しだけを待ちます。`POST` が応答を待つ間、それを送った要素には、`exclusive` の有無にかかわらず `aria-busy=\"true\"` が付きます。",
            },
        ),
    ])
}

/// `timeout`, and why a stream has none.
fn timeout(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A time limit",
                ja: "時間の上限",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::timeout`](api:hypersig/struct.Call.html#method.timeout), `fn timeout(self, limit: Duration) -> Self`, fails a call whose answer has not ended `limit` after it was sent. The browser then aborts the request and drops the patches still to come, the `status` signal stays `0`, and the `error` signal shows `the answer took longer than 10000 ms`. Without it, a call waits for as long as the server takes, which an upload of a large file needs. The server may still finish the handler after the browser stopped waiting, so a call that changes data can have taken effect.",
                ja: "[`Call::timeout`](api:hypersig/struct.Call.html#method.timeout)（`fn timeout(self, limit: Duration) -> Self`）は、送ってから `limit` が過ぎても応答が終わらない呼び出しを失敗にします。このときブラウザはリクエストを中断し、まだ届いていない差分を捨てます。`status` のシグナルは `0` のままで、`error` のシグナルには `the answer took longer than 10000 ms` が入ります。付けなければ、呼び出しはサーバーが応答を終えるまで待ちます。大きなファイルのアップロードには、この待ち方が必要です。ブラウザが待つのをやめたあとも、サーバーはハンドラを最後まで実行することがあります。そのため、データを変える呼び出しは、失敗になっても変更が反映されている場合があります。",
            },
        ),
        para(
            site,
            L {
                en: "A [stream](learn/streams/) stays open until its handler ends it, so a [`StreamCall`](api:hypersig/struct.StreamCall.html) has no `timeout`, and calling it is a compile error, E0599. It has no `latest` or `exclusive` either.",
                ja: "[ストリーム](learn/streams/) はハンドラが終えるまで開いたままなので、[`StreamCall`](api:hypersig/struct.StreamCall.html) には `timeout` がなく、呼ぶとコンパイルエラー E0599 になります。`latest` と `exclusive` もありません。",
            },
        ),
        show(Snippet::TalkStreamTimeout),
    ])
}

/// What the types leave to the page.
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
                en: "`latest` compiles on a `POST` that changes data. When a later call aborts it, the handler may have made part of its change before it stopped at an `.await`, and the page receives none of its answer. `exclusive` and `latest` group calls by action, not by element, so two buttons that call one action with different arguments still skip or abort each other. A call that failed by `timeout` may have changed the data, so a page that sends it again can repeat the change.",
                ja: "`latest` は、データを変える `POST` に付けてもコンパイルできます。あとの呼び出しに取り消されると、ハンドラは `.await` で止まるまでに変更の一部を済ませていることがあり、ページはその応答を何も受け取りません。`exclusive` と `latest` は呼び出しを要素ではなくアクションでまとめます。そのため、同じアクションを別の引数で呼ぶ 2 つのボタンも、互いに送らなかったり取り消したりします。`timeout` で失敗した呼び出しはデータを変えていることがあるので、ページがもう一度送ると、変更が繰り返されることがあります。",
            },
        ),
    ])
}

/// The complete example, and the pages that follow.
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
                en: "This checkout sends one order at a time and gives up on a shipping quote after 10 seconds.",
                ja: "この購入画面は注文を一度に 1 つだけ送り、送料の見積もりは 10 秒で打ち切ります。",
            },
        ),
        show(Snippet::TalkExclusive),
        para(
            site,
            L {
                en: "Next, [Patches](learn/patches/) covers what the answers carry, and [Streams](learn/streams/) covers an answer that stays open.",
                ja: "次は応答の中身を扱う [差分](learn/patches/) と、開いたままの応答を扱う [ストリーム](learn/streams/) です。",
            },
        ),
    ])
}
