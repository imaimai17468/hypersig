//! The page on what a call shows while it waits, and when it fails or
//! succeeds.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::CallStatus).title(site)),
        lead(site, Doc::Learn(Learn::CallStatus).leads()),
        waiting(site),
        failing(site),
        outcomes(site),
        unchecked(site),
        example(site),
    ])
}

/// `indicator` and `aria-busy`.
fn waiting(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "While a call waits",
                ja: "呼び出しが応答を待つ間",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::indicator`](api:hypersig/struct.Call.html#method.indicator), `fn indicator(self, flag: Sig<S, bool>) -> Self`, sets `flag` to true when the call is sent and to false once its answer has arrived. Several calls may set one flag, which then turns false once every one of them has its answer, so one `saving` signal can cover every save button on a page. Bind the flag like any other signal, for example with `attr_when(attr::DISABLED, flag)` on the button or `show(flag)` on a spinner.",
                ja: "[`Call::indicator`](api:hypersig/struct.Call.html#method.indicator)（`fn indicator(self, flag: Sig<S, bool>) -> Self`）は、呼び出しを送ったときに `flag` を真にし、応答が届き終えたら偽にします。複数の呼び出しで 1 つのフラグを使うこともでき、その場合はすべての呼び出しが応答を受け取ったときに偽になります。ページ上のどの保存ボタンにも 1 つの `saving` シグナルを使えます。フラグはほかのシグナルと同じように束縛します。たとえばボタンに `attr_when(attr::DISABLED, flag)` を、スピナーに `show(flag)` を付けます。",
            },
        ),
        para(
            site,
            L {
                en: "While a `POST` waits, the element whose handler sent it carries `aria-busy=\"true\"`, with or without an indicator, unless the page gave that element an `aria-busy` of its own. A screen reader announces the element as busy, and a stylesheet can select `[aria-busy=\"true\"]`. A `GET` sets no `aria-busy`.",
                ja: "`POST` が応答を待つ間、それを送ったハンドラの要素には、indicator の有無にかかわらず `aria-busy=\"true\"` が付きます。ただし、ページがその要素に独自の `aria-busy` を付けている場合は付きません。スクリーンリーダーはその要素を処理中として読み上げ、スタイルシートは `[aria-busy=\"true\"]` で選択できます。`GET` には `aria-busy` が付きません。",
            },
        ),
        para(
            site,
            L {
                en: "The flag must be a `bool` signal. A signal of another type is a compile error, E0308.",
                ja: "フラグは `bool` のシグナルでなければなりません。ほかの型のシグナルを渡すと、コンパイルエラー E0308 になります。",
            },
        ),
        show(Snippet::TalkWrongIndicator),
    ])
}

/// `error`, `status`, and the console.
fn failing(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "When a call fails",
                ja: "呼び出しが失敗したとき",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::error`](api:hypersig/struct.Call.html#method.error), `fn error(self, message: Sig<S, String>) -> Self`, sets `message` to `\"\"` each time the call is sent and to the error text when it fails, so the same failure twice changes the signal twice. The text depends on what came back:",
                ja: "[`Call::error`](api:hypersig/struct.Call.html#method.error)（`fn error(self, message: Sig<S, String>) -> Self`）は、呼び出しを送るたびに `message` を `\"\"` に戻し、失敗したときにエラーの文言を入れます。戻してから入れ直すので、同じ失敗が 2 回続いてもシグナルは 2 回変わります。文言は返ってきたものによって決まります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["The answer", "The text"],
                ja: &["応答", "文言"],
            },
            L {
                en: &[
                    &[
                        "A status outside 2xx with a `text/plain` body, which axum sends for a `String` or a `&str`",
                        "The body, so the handler writes the sentence the page shows",
                    ],
                    &[
                        "A status outside 2xx with an empty body or one of another type, such as a proxy's HTML error page",
                        "The status and its reason, such as `502 Bad Gateway`",
                    ],
                    &[
                        "No answer: the connection failed or the browser is offline",
                        "The browser's own message for the failed `fetch`",
                    ],
                    &[
                        "A 2xx answer that was redirected",
                        "`the answer was redirected to` and the URL",
                    ],
                    &[
                        "A 2xx answer that is not `text/event-stream`",
                        "`the answer is text/html, not an event stream`, with the type it had",
                    ],
                    &[
                        "An answer that closes before the handler has finished, such as one a host cuts at its time limit",
                        "`the answer closed before the server ended it`",
                    ],
                    &[
                        "No complete answer within the [`timeout`](learn/overlapping-calls/)",
                        "`the answer took longer than 10000 ms`, with the limit it had",
                    ],
                ],
                ja: &[
                    &[
                        "2xx 以外のステータスで、本文が `text/plain`。axum は `String` や `&str` をこの形で送る",
                        "本文。ページに出る文はハンドラが書く",
                    ],
                    &[
                        "2xx 以外のステータスで、本文が空かほかの型。プロキシが返す HTML のエラーページなど",
                        "ステータスとその理由。`502 Bad Gateway` など",
                    ],
                    &[
                        "応答がない。接続の失敗やオフライン",
                        "失敗した `fetch` についてブラウザが出すメッセージ",
                    ],
                    &[
                        "リダイレクトされた 2xx の応答",
                        "`the answer was redirected to` と、その URL",
                    ],
                    &[
                        "`text/event-stream` ではない 2xx の応答",
                        "`the answer is text/html, not an event stream`。型の部分は実際の型",
                    ],
                    &[
                        "ハンドラが終わる前に閉じた応答。ホストが時間の上限で切ったものなど",
                        "`the answer closed before the server ended it`",
                    ],
                    &[
                        "[`timeout`](learn/overlapping-calls/) の時間内に応答が終わらない",
                        "`the answer took longer than 10000 ms`。数値は指定した上限",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "[`Call::status`](api:hypersig/struct.Call.html#method.status), `fn status(self, status: Sig<S, i32>) -> Self`, sets `status` to `0` when the call is sent and to the HTTP status of a failed answer. It stays `0` in the last five rows of the table, where no status the page can act on arrived. Compare it with a status written from `StatusCode`, such as `i32::from(StatusCode::UNAUTHORIZED.as_u16())`, to show a sign-in link for a 401 and the text for anything else, without reading the text.",
                ja: "[`Call::status`](api:hypersig/struct.Call.html#method.status)（`fn status(self, status: Sig<S, i32>) -> Self`）は、呼び出しを送ったときに `status` を `0` にし、失敗した応答の HTTP ステータスを入れます。上の表の下 5 行では、ページが扱えるステータスが届いていないので `0` のままです。`i32::from(StatusCode::UNAUTHORIZED.as_u16())` のように `StatusCode` から書いたステータスと比べれば、文言を読まずに、401 ならサインインのリンクを、それ以外なら文言を表示するといった出し分けができます。",
            },
        ),
        para(
            site,
            L {
                en: "Every failure is also written to the browser's console, as `hypersig: POST /save failed with 409: Someone else saved these notes first.`. This happens with or without `error`.",
                ja: "失敗はどれも、`hypersig: POST /save failed with 409: Someone else saved these notes first.` のようにブラウザのコンソールにも書かれます。これは `error` の有無にかかわらず起きます。",
            },
        ),
    ])
}

/// `on_error` and `on_success`.
fn outcomes(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A statement for failure or success",
                ja: "失敗や成功のときに実行する文",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::on_error`](api:hypersig/struct.Call.html#method.on_error), `fn on_error<B: IntoStmt<S>>(self, handle: impl FnOnce(Expr<S, String>) -> B) -> Self`, runs the statement `handle` builds when the call fails, and `handle` receives the same text `error` writes. [`Call::on_success`](api:hypersig/struct.Call.html#method.on_success), `fn on_success(self, next: impl IntoStmt<S>) -> Self`, runs `next` once the answer has arrived and its patches are applied. Both run where the call was made: inside a list item they change that item, so a cart that orders several lines at once marks only the line whose order failed. Neither runs for a `latest` call that a later one aborted, or once the page navigates away.",
                ja: "[`Call::on_error`](api:hypersig/struct.Call.html#method.on_error)（`fn on_error<B: IntoStmt<S>>(self, handle: impl FnOnce(Expr<S, String>) -> B) -> Self`）は、呼び出しが失敗したときに `handle` が作る文を実行します。`handle` は `error` が書くのと同じ文言を受け取ります。[`Call::on_success`](api:hypersig/struct.Call.html#method.on_success)（`fn on_success(self, next: impl IntoStmt<S>) -> Self`）は、応答が届いて差分を適用し終えたあとに `next` を実行します。どちらも呼び出しを書いた場所で実行されます。リストの項目の中ならその項目を変えるので、複数の行を同時に注文するカートでも、注文に失敗した行だけに印を付けられます。あとの呼び出しに取り消された `latest` の呼び出しや、ページが別のページへ移ったあとでは、どちらも実行されません。",
            },
        ),
        show(Snippet::TalkOutcomes),
    ])
}

/// What happens when nothing on the page reads the failure.
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
                en: "Nothing requires a call to report its failure. A call without `error`, `status`, or `on_error` that fails changes nothing on the page, and only the console line shows it. Nothing checks that the page binds the `error` signal either, so a signal that no element shows leaves the visitor without a message. The text of a `text/plain` error body reaches the visitor as it was sent, so a handler should answer with a sentence meant for them and log the details on the server.",
                ja: "呼び出しに失敗を知らせる設定を付けることは必須ではありません。`error`、`status`、`on_error` のどれもない呼び出しが失敗しても、ページは何も変わらず、コンソールの行だけが失敗を示します。`error` のシグナルをページが束縛しているかどうかも検査されないので、どの要素も表示しないシグナルを渡すと、訪問者には何も伝わりません。`text/plain` のエラーの本文は送られたまま訪問者に届きます。ハンドラは訪問者に向けた文を返し、詳しい内容はサーバーのログに書いてください。",
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
                en: "This save button is disabled while its call waits, shows the error text in an alert, and offers a sign-in link when the server answers 401.",
                ja: "この保存ボタンは呼び出しが応答を待つ間は無効になり、エラーの文言をアラートに表示し、サーバーが 401 を返したときはサインインのリンクを出します。",
            },
        ),
        show(Snippet::TalkCallStatus),
        para(
            site,
            L {
                en: "Next, [Optimistic updates](learn/optimistic-updates/) changes the page before the answer arrives, and [Calls that overlap](learn/overlapping-calls/) covers a second call sent while the first one waits.",
                ja: "次は応答が届く前にページを変える [楽観的更新](learn/optimistic-updates/) と、最初の呼び出しが応答を待つ間に送った 2 つ目の呼び出しを扱う [重なった呼び出し](learn/overlapping-calls/) です。",
            },
        ),
    ])
}
