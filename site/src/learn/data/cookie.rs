//! The page on the session's cookie: the attributes the layer gives it, the
//! `SameSite` the sample picks, and how long the cookie lasts.

use super::next;
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::SessionCookie).title(site)),
        lead(site, Doc::Learn(Learn::SessionCookie).leads()),
        attributes(site),
        expiry(site),
        next(
            site,
            &[
                L {
                    en: "[Where sessions are stored](learn/session-stores/) covers the record the cookie's id points to.",
                    ja: "[セッションの保存先](learn/session-stores/) では Cookie の id が指す記録の置き場所を説明します。",
                },
                L {
                    en: "[Security](learn/security/) lists the requests from another site that an action refuses.",
                    ja: "[セキュリティ](learn/security/) には、アクションが拒否する別のサイトからのリクエストをまとめています。",
                },
            ],
        ),
    ])
}

/// The attributes of the session's cookie.
fn attributes(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The cookie's attributes",
                ja: "Cookie の属性",
            },
        ),
        para(
            site,
            L {
                en: "`SessionManagerLayer` gives the cookie these attributes unless the builder says otherwise. The sample on [Signing in](learn/sessions/) changes two of them:",
                ja: "`SessionManagerLayer` は、ビルダーで変えない限り、Cookie に次の属性を付けます。[サインインとセッション](learn/sessions/) のサンプルは、そのうち 2 つを変えています。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Attribute", "Default", "The sample", "Builder method"],
                ja: &["属性", "既定", "サンプル", "ビルダーのメソッド"],
            },
            L {
                en: &[
                    &["Name", "`id`", "`session`", "`with_name`"],
                    &["`HttpOnly`", "On", "On", "`with_http_only`"],
                    &["`Secure`", "On", "On", "`with_secure`"],
                    &["`SameSite`", "`Strict`", "`Lax`", "`with_same_site`"],
                    &["`Path`", "`/`", "`/`", "`with_path`"],
                    &["`Max-Age`", "None", "None", "`with_expiry`"],
                ],
                ja: &[
                    &["名前", "`id`", "`session`", "`with_name`"],
                    &["`HttpOnly`", "付く", "付く", "`with_http_only`"],
                    &["`Secure`", "付く", "付く", "`with_secure`"],
                    &["`SameSite`", "`Strict`", "`Lax`", "`with_same_site`"],
                    &["`Path`", "`/`", "`/`", "`with_path`"],
                    &["`Max-Age`", "なし", "なし", "`with_expiry`"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "With `Strict`, the browser leaves the cookie off a visit that follows a link from another site, such as a link in an email, and that visit lands on the sign-in page. The sample sets `Lax`, which sends the cookie on such a visit. An action needs no `Strict`, because it refuses a request another site sends, as [Security](learn/security/) lists.",
                ja: "`Strict` では、メールのリンクのように別のサイトのリンクからの訪問に、ブラウザが Cookie を付けません。その訪問はサインインのページに送られます。サンプルは `Lax` にして、そうした訪問にも Cookie が付くようにしています。アクションは別のサイトからのリクエストを拒否するので（[セキュリティ](learn/security/) に一覧があります）、`Strict` は必要ありません。",
            },
        ),
        para(
            site,
            L {
                en: "`HttpOnly` keeps the cookie out of `document.cookie`, so a script on the page cannot read the id. `Secure` keeps the cookie to HTTPS. Chrome also stores it from `http://127.0.0.1`, where the repository's browser tests run, and other browsers were not checked. A browser that drops it lands back on the sign-in page after signing in.",
                ja: "`HttpOnly` は Cookie を `document.cookie` から隠すので、ページのスクリプトは id を読めません。`Secure` は Cookie を HTTPS に限ります。Chrome は `http://127.0.0.1` からもこの Cookie を保存し、リポジトリのブラウザのテストはそこで動いています。ほかのブラウザでは確かめていません。Cookie を捨てるブラウザでは、サインインしたあともサインインのページに戻ります。",
            },
        ),
    ])
}

/// The cookie's `Max-Age`, and when the layer renews it.
fn expiry(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "How long the cookie lasts",
                ja: "Cookie が保持される期間",
            },
        ),
        para(
            site,
            L {
                en: "Without an expiry the cookie has no `Max-Age`, so the browser drops it when it closes. With `with_expiry(Expiry::OnInactivity(duration))`, the cookie gets a `Max-Age` of `duration` each time the layer saves the session, and the visitor stays signed in across a restart of the browser:",
                ja: "有効期限を決めないと Cookie には `Max-Age` が付かず、ブラウザを閉じると消えます。`with_expiry(Expiry::OnInactivity(duration))` にすると、レイヤーがセッションを保存するたびに Cookie の `Max-Age` が `duration` になり、ブラウザを起動し直してもサインインしたままになります。",
            },
        ),
        show(Snippet::SessionExpiry),
        code::block(
            Lang::Html,
            None,
            "set-cookie: session=NopwG88FsWT6bJHopbZRSw; HttpOnly; SameSite=Lax; Secure; Path=/; Max-Age=604800",
        ),
        para(
            site,
            L {
                en: "The layer saves only a session a handler changed, so a visitor who only reads pages is signed out `duration` after the last change. `with_always_save(true)` saves the session on every response, so the time counts from the last request.",
                ja: "レイヤーが保存するのはハンドラが変えたセッションだけなので、ページを読むだけの訪問者は最後の変更から `duration` たつとサインアウトします。`with_always_save(true)` にするとどの応答でも保存するので、最後のリクエストから数えるようになります。",
            },
        ),
    ])
}
