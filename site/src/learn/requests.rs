//! The page on what a handler reads from the request and writes beside its
//! answer: extractors, headers such as cookies, errors, and the requests an
//! action refuses.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, title};

pub fn requests(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Requests).title(site)),
        lead(
            site,
            L {
                en: "A handler reads the request through an axum extractor, sets cookies by returning headers beside its answer, and fails with `?`.",
                ja: "ハンドラは axum の extractor でリクエストを読み、答えと組にしたヘッダーで Cookie を設定し、`?` で失敗を返します。",
            },
        ),
        heading(
            site,
            L {
                en: "Extractors and headers",
                ja: "extractor とヘッダー",
            },
        ),
        para(
            site,
            L {
                en: "A page or action handler takes the state and its arguments, and may take one [`FromRequestParts`](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html) extractor between them: `HeaderMap`, a tuple of extractors, or the application's own type, such as the signed-in member read from a session cookie. When the extractor fails, its rejection is the response and the handler does not run. A pair of headers and a page or patches, such as `([(SET_COOKIE, value)], patches)`, sends the headers with the answer.",
                ja: "ページとアクションのハンドラは状態と引数を受け取り、そのあいだに [`FromRequestParts`](https://docs.rs/axum/0.8/axum/extract/trait.FromRequestParts.html) の extractor を 1 つ受け取れます。`HeaderMap`、extractor のタプル、セッションの Cookie から読んだサインイン中のメンバーのようなアプリケーションの型が使えます。extractor が失敗すると、その rejection が応答になり、ハンドラは動きません。`([(SET_COOKIE, value)], patches)` のようにヘッダーとページや差分を組にすると、ヘッダーが答えと一緒に送られます。",
            },
        ),
        show(Snippet::Extractors),
        para(
            site,
            L {
                en: "An extractor that guards actions answers a signed-out request with a status such as 401. The browser's `fetch` follows a redirect, so an action answered with one receives the sign-in page's HTML and the page shows nothing. The `Accept` header tells the two apart: the runtime asks for `text/event-stream`.",
                ja: "アクションを守る extractor は、サインインしていないリクエストに 401 などの状態で答えます。ブラウザの `fetch` はリダイレクトをたどるので、リダイレクトで答えたアクションはサインインのページの HTML を受け取り、ページには何も表示されません。2 つは `Accept` ヘッダーで見分けられます。ランタイムは `text/event-stream` を求めます。",
            },
        ),
        heading(
            site,
            L {
                en: "Errors",
                ja: "エラー",
            },
        ),
        para(
            site,
            L {
                en: "A handler may return `Result<_, E>` where `E` implements `IntoResponse`, so `?` works. An action whose response has a status outside 2xx fails on the page: the runtime writes the status and the body to the signal given to `Call::error`.",
                ja: "ハンドラは、`E` が `IntoResponse` を実装する `Result<_, E>` を返せるので、`?` が使えます。応答の状態が 2xx 以外のアクションは、ページでは失敗になります。ランタイムは、`Call::error` に渡したシグナルに状態と本文を書きます。",
            },
        ),
        show(Snippet::HandlerErrors),
        heading(
            site,
            L {
                en: "Requests an action refuses",
                ja: "アクションが断るリクエスト",
            },
        ),
        para(
            site,
            L {
                en: "Before its handler runs, an action answers a request whose `Sec-Fetch-Site` is not `same-origin` with a 403, and a POST whose body is not `application/json` with a 415. A form on another site cannot send JSON, and a script there that tries needs a preflight request this server does not answer, so neither can call an action with the visitor's cookie.",
                ja: "アクションは、ハンドラが動く前に、`Sec-Fetch-Site` が `same-origin` でないリクエストに 403 を、本文が `application/json` でない POST に 415 を返します。別のサイトのフォームは JSON を送れず、そこでスクリプトが送ろうとするとプリフライトのリクエストが要り、このサーバーはそれに答えません。どちらも訪問者の Cookie を付けてアクションを呼び出せません。",
            },
        ),
        note(
            site,
            L {
                en: "A browser that sends no `Sec-Fetch-Site` still reaches a GET action from a link on another site, so a GET action should change nothing. The `auth` example in the repository signs in with a cookie session end to end.",
                ja: "`Sec-Fetch-Site` を送らないブラウザからは、別のサイトのリンクで GET のアクションに届くので、GET のアクションでは何も変更しないでください。リポジトリの `auth` の例は、Cookie のセッションでのサインインを最初から最後まで示します。",
            },
        ),
    ])
}
