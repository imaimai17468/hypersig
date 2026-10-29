//! The page on pages and actions only a signed-in member reaches: the
//! extractor that reads the member from the session, the refusal each kind
//! of request gets, and the link a page shows after a 401.

use super::next;
use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::MembersOnly).title(site)),
        lead(site, Doc::Learn(Learn::MembersOnly).leads()),
        extractor(site),
        refusals(site),
        sign_in_again(site),
        unchecked(site),
        next(
            site,
            &[
                L {
                    en: "[Signing in](learn/sessions/) writes the member's name into the session this extractor reads.",
                    ja: "[サインインとセッション](learn/sessions/) ではこのエクストラクタが読むメンバーの名前をセッションに書きます。",
                },
                L {
                    en: "[Extractors and headers](learn/requests/) covers extractors in general.",
                    ja: "[エクストラクタとヘッダー](learn/requests/) ではエクストラクタ全般を説明します。",
                },
                L {
                    en: "[Security](learn/security/) lists what the router refuses before a handler runs.",
                    ja: "[セキュリティ](learn/security/) には、ハンドラが動く前にルーターが拒否するものをまとめています。",
                },
            ],
        ),
    ])
}

/// The extractor that reads the member.
fn extractor(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The signed-in member",
                ja: "サインイン中のメンバー",
            },
        ),
        para(
            site,
            L {
                en: "`Member` below is an axum extractor: it implements `FromRequestParts<AppState>`, reads the `Session`, and finds the name the sign-in put there. A handler that takes it, as `home(state, member: Member, ())`, runs only for a signed-in visitor, and the extractor's rejection is the answer to everyone else. The repository's [`auth`](https://github.com/imaimai17468/hypersig/tree/main/examples/auth) example uses the same extractor:",
                ja: "下の `Member` は axum のエクストラクタです。`FromRequestParts<AppState>` を実装し、`Session` を読んで、サインインが入れた名前を探します。`home(state, member: Member, ())` のようにこれを受け取るハンドラはサインイン中の訪問者に対してだけ動き、それ以外の訪問者にはエクストラクタの拒否が応答になります。リポジトリの [`auth`](https://github.com/imaimai17468/hypersig/tree/main/examples/auth) の例も同じエクストラクタを使います。",
            },
        ),
        show(Snippet::Members),
    ])
}

/// The redirect a page gets and the 401 an action gets.
fn refusals(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A redirect for a page, a 401 for an action",
                ja: "ページにはリダイレクト、アクションには 401",
            },
        ),
        para(
            site,
            L {
                en: "[`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) is `fn of(request: &Parts) -> RequestKind`. It returns `RequestKind::Runtime` for a request the runtime sends to call an action, a stream, or an upload, and `RequestKind::Navigation` for a browser loading a page. The extractor answers each kind differently:",
                ja: "[`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) は `fn of(request: &Parts) -> RequestKind` です。ランタイムがアクション、ストリーム、アップロードを呼んだリクエストには `RequestKind::Runtime` を、ブラウザがページを読み込んだときは `RequestKind::Navigation` を返します。エクストラクタは種類ごとに違う応答を返します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Request", "Answer without a member"],
                ja: &["リクエスト", "メンバーがいないときの応答"],
            },
            L {
                en: &[
                    &[
                        "`GET /`, the browser loading the page",
                        "`303 See Other` with `location: /login`",
                    ],
                    &[
                        "`POST /notes`, the runtime calling the action",
                        "`401 Unauthorized` with the `text/plain` body `Sign in again.`",
                    ],
                ],
                ja: &[
                    &[
                        "`GET /`（ブラウザがページを読み込む）",
                        "`location: /login` 付きの `303 See Other`",
                    ],
                    &[
                        "`POST /notes`（ランタイムがアクションを呼ぶ）",
                        "`text/plain` の本文 `Sign in again.` 付きの `401 Unauthorized`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "An action gets no redirect because the runtime's `fetch` would follow it and receive the sign-in page's HTML, which it cannot apply as patches. The 401's text reaches the signal the call's `error` names. `axum-login`'s `login_required!` answers every request it guards in one way, a redirect or a 401, chosen when the router is built, so the sample writes its own extractor.",
                ja: "アクションにリダイレクトを返さないのは、ランタイムの `fetch` がリダイレクトをたどってサインインのページの HTML を受け取ってしまい、それを差分として当てられないためです。401 の本文は、呼び出しの `error` が指すシグナルに入ります。`axum-login` の `login_required!` は、守るリクエストすべてに、ルーターを組むときに選んだリダイレクトか 401 のどちらか一方で応答するので、サンプルはエクストラクタを自分で書いています。",
            },
        ),
        subheading(
            site,
            L {
                en: "The address of the sign-in page",
                ja: "サインインのページのアドレス",
            },
        ),
        para(
            site,
            L {
                en: "[`PageResponse::redirect`](api:hypersig/struct.PageResponse.html#method.redirect) is `fn redirect(url: Url) -> PageResponse`, and answers with a `303 See Other` to `url`. As with `Patches::navigate`, the `Url` comes from a route, as `LOGIN.url(())`, so a path written as a string fails to compile with E0308. [Links and redirects](learn/links/) covers the redirect in general.",
                ja: "[`PageResponse::redirect`](api:hypersig/struct.PageResponse.html#method.redirect) は `fn redirect(url: Url) -> PageResponse` で、`url` への `303 See Other` で応答します。`Patches::navigate` と同じく `Url` は `LOGIN.url(())` のようにルートから作るので、文字列で書いたパスは E0308 でコンパイルに失敗します。リダイレクト全般は [リンクとリダイレクト](learn/links/) で説明します。",
            },
        ),
        show(Snippet::RedirectText),
    ])
}

/// The 401 in an `i32` signal, and the link it shows.
fn sign_in_again(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A link to sign in again",
                ja: "サインインし直すためのリンク",
            },
        ),
        para(
            site,
            L {
                en: "A visitor whose session ended while the page was open gets the 401 on the next call. [`Call::status`](api:hypersig/struct.Call.html#method.status) is `fn status(self, status: Sig<S, i32>) -> Self`: it sets the signal to `0` when the call is sent and to the status of a failed answer. The home page above compares `failure` with `StatusCode::UNAUTHORIZED` and shows a link to the sign-in page while they are equal, as the `auth` example does. The link is in the HTML from the start, hidden while the condition is false.",
                ja: "ページを開いているあいだにセッションが終わった訪問者は、次の呼び出しで 401 を受け取ります。[`Call::status`](api:hypersig/struct.Call.html#method.status) は `fn status(self, status: Sig<S, i32>) -> Self` で、呼び出しを送るときにシグナルを `0` にし、失敗した応答のステータスをそこに書きます。上のホームのページは `failure` を `StatusCode::UNAUTHORIZED` と比べ、等しいあいだサインインのページへのリンクを表示します。`auth` の例もそうしています。リンクは最初から HTML にあり、条件が偽のあいだは隠れています。",
            },
        ),
        para(
            site,
            L {
                en: "The status stays `0` when no answer the runtime can read arrives, such as when the connection fails, so the link appears only for a 401.",
                ja: "接続が失敗したときのように、ランタイムが読める応答が届かなければステータスは `0` のままなので、リンクが表示されるのは 401 のときだけです。",
            },
        ),
    ])
}

/// What the extractor leaves to the reader of the code.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型で検査しないもの",
            },
        ),
        para(
            site,
            L {
                en: "Nothing makes a handler take `Member`. A page or an action whose handler leaves it out is served to every visitor, signed in or not, so list the handlers that take it when you review a change to `routes!`. The key `MEMBER` and the type `String` that `session.get` reads are not checked against what the sign-in inserted either. A different key reads `None`, and the visitor is sent to the sign-in page after signing in. A value that does not deserialize as the type is an error, which the extractor answers with a 500.",
                ja: "ハンドラに `Member` を受け取らせる仕組みはありません。ハンドラが `Member` を受け取らないページやアクションは、サインインしているかどうかに関係なくすべての訪問者に配信されます。`routes!` を変えたときは、どのハンドラが `Member` を受け取るかを確かめてください。`session.get` が読むキー `MEMBER` と型 `String` も、サインインが入れたものと照らし合わせて検査されるわけではありません。キーが違うと `None` を読み、訪問者はサインインしたあともサインインのページに送られます。値がその型として読めなければエラーになり、エクストラクタは 500 で応答します。",
            },
        ),
    ])
}
