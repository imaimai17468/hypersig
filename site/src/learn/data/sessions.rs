//! The page on signing in: a session kept by `tower-sessions`, the actions
//! that sign a visitor in and out, the requests and responses they make,
//! and the password check.

use super::next;
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, subheading, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Sessions).title(site)),
        lead(site, Doc::Learn(Learn::Sessions).leads()),
        layer(site),
        sign_in(site),
        traffic(site),
        passwords(site),
        next(
            site,
            &[
                L {
                    en: "[Pages for signed-in members](learn/members-only/) reads the member from the session and turns everyone else away.",
                    ja: "[サインインした人だけのページ](learn/members-only/) ではセッションからメンバーを読み、それ以外の訪問者を断ります。",
                },
                L {
                    en: "[The session cookie](learn/session-cookie/) covers the cookie's attributes and how long it lasts.",
                    ja: "[セッションの Cookie](learn/session-cookie/) では Cookie の属性と保持される期間を説明します。",
                },
                L {
                    en: "[Where sessions are stored](learn/session-stores/) replaces `MemoryStore` for a server with more than one instance.",
                    ja: "[セッションの保存先](learn/session-stores/) ではインスタンスが複数あるサーバーのために `MemoryStore` を置き換えます。",
                },
                L {
                    en: "[Testing handlers](learn/testing/) calls `sign_in` from a test with a `Session` the test builds.",
                    ja: "[ハンドラのテスト](learn/testing/) ではテストが作った `Session` を渡して `sign_in` を呼びます。",
                },
            ],
        ),
    ])
}

/// What a session is, and the dependency.
fn layer(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A session layer",
                ja: "セッションのレイヤー",
            },
        ),
        para(
            site,
            L {
                en: "A session is a record on the server that the browser points to with a cookie holding its id. [tower-sessions](https://docs.rs/tower-sessions/0.14) keeps that record, generates the id, and writes the `Set-Cookie` header, so a handler only reads and writes values in the session. Add it to `Cargo.toml`:",
                ja: "セッションはサーバーに置く記録で、ブラウザはその id を入れた Cookie で記録を指します。[tower-sessions](https://docs.rs/tower-sessions/0.14) がこの記録を持ち、id を作り、`Set-Cookie` ヘッダーを書くので、ハンドラはセッションの値を読み書きするだけで済みます。`Cargo.toml` に次の行を追加してください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
tower-sessions = { version = "0.14", default-features = false, features = ["axum-core", "memory-store"] }"#,
        ),
        para(
            site,
            L {
                en: "The sample uses 0.14 although 0.15 is out, because `tower-sessions-sqlx-store` 0.15, `tower-sessions-redis-store` 0.16, and `axum-login` 0.18 depend on `tower-sessions-core` 0.14, and a store built on it does not implement the `SessionStore` trait of a 0.15 layer. The repository's [`auth`](https://github.com/imaimai17468/hypersig/tree/main/examples/auth) example uses the same sign-in and sign-out behind a sign-in form.",
                ja: "0.15 が出ていますが、サンプルは 0.14 を使います。`tower-sessions-sqlx-store` 0.15、`tower-sessions-redis-store` 0.16、`axum-login` 0.18 が `tower-sessions-core` 0.14 に依存しており、それを使うストアは 0.15 のレイヤーの `SessionStore` トレイトを実装しないためです。リポジトリの [`auth`](https://github.com/imaimai17468/hypersig/tree/main/examples/auth) の例は同じサインインとサインアウトをサインインのフォームと一緒に使います。",
            },
        ),
    ])
}

/// The actions that sign a visitor in and out.
fn sign_in(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Signing in and out",
                ja: "サインインとサインアウト",
            },
        ),
        para(
            site,
            L {
                en: "The sign-in is an action whose handler takes the session as its extractor, `sign_in(state, session: Session, credentials: Credentials)`, the form `Fn(St, E, A)` of [`Handler`](api:hypersig/trait.Handler.html). The form on the sign-in page sends the `Credentials` signal as the action's arguments:",
                ja: "サインインは、セッションをエクストラクタとして受け取るハンドラ `sign_in(state, session: Session, credentials: Credentials)` のアクションです。これは [`Handler`](api:hypersig/trait.Handler.html) の `Fn(St, E, A)` の形です。サインインのページのフォームは `Credentials` のシグナルをアクションの引数として送ります。",
            },
        ),
        show(Snippet::Sessions),
        para(
            site,
            L {
                en: "`SessionManagerLayer` wraps the router, so it runs around every page, action, and stream. It reads the id from the cookie and hands each handler a `Session`, which loads the record from the store the first time a handler reads it. After the handler returns its response, the layer saves a session the handler changed and sets the cookie on that response, or removes the cookie of a session the handler emptied. The layer saves nothing when the status is a 5xx, and a change a stream makes to the session after its response has started is not saved.",
                ja: "`SessionManagerLayer` はルーターを包むので、すべてのページ、アクション、ストリームの前後で動きます。Cookie から id を読み、各ハンドラに `Session` を渡します。`Session` は、ハンドラが初めて値を読むときにストアから記録を読み込みます。ハンドラが応答を返したあと、レイヤーは、ハンドラが変えたセッションを保存してその応答に Cookie を付けるか、ハンドラが空にしたセッションの Cookie を削除します。ステータスが 5xx のときレイヤーは何も保存せず、ストリームが応答を始めたあとにセッションを変えても、その変更は保存されません。",
            },
        ),
        subheading(
            site,
            L {
                en: "A new id at sign-in",
                ja: "サインインで id を新しくする",
            },
        ),
        para(
            site,
            L {
                en: "The sign-in calls `cycle_id` before it puts the name in the session. Without it, someone who placed the id of their own session in the visitor's browser would hold the same id after the visitor signs in, and would be signed in as the visitor. With it, the session that holds the member has a new id, and the store no longer has the old one. The sign-out calls `flush`, which deletes the record from the store and empties the session, and the layer then removes the cookie.",
                ja: "サインインでは名前をセッションに入れる前に `cycle_id` を呼びます。これがないと、自分のセッションの id を訪問者のブラウザに仕込んだ人が、訪問者のサインイン後も同じ id を持ち続け、訪問者としてサインインした状態になります。`cycle_id` を呼べば、メンバーを持つセッションの id は新しくなり、古い id はストアから削除されます。サインアウトでは `flush` を呼びます。`flush` はストアから記録を削除してセッションを空にし、レイヤーがそのあと Cookie を削除します。",
            },
        ),
        subheading(
            site,
            L {
                en: "The page the browser goes to next",
                ja: "ブラウザが次に移るページ",
            },
        ),
        para(
            site,
            L {
                en: "[`Patches::navigate`](api:hypersig/struct.Patches.html#method.navigate) is `fn navigate(self, url: Url) -> Self`, and loads the page at `url` once the patches before it are applied. A `Url` comes from a route, as `HOME.url(())`, so a path written as a string fails to compile with E0308:",
                ja: "[`Patches::navigate`](api:hypersig/struct.Patches.html#method.navigate) は `fn navigate(self, url: Url) -> Self` で、その前の差分を当てたあとに `url` のページを読み込みます。`Url` は `HOME.url(())` のようにルートから作るので、文字列で書いたパスは E0308 でコンパイルに失敗します。",
            },
        ),
        show(Snippet::NavigateText),
        para(
            site,
            L {
                en: "A name and password that do not match answer with a patch of `problem`, which the page shows under the form. The session stays unchanged, so the layer sets no cookie. [Links and redirects](learn/links/) covers sending the browser to another page in general.",
                ja: "名前とパスワードが一致しないときは `problem` の差分で応答し、ページはそれをフォームの下に表示します。セッションは変わらないので、レイヤーは Cookie を付けません。ブラウザを別のページへ送る方法全般は [リンクとリダイレクト](learn/links/) で説明します。",
            },
        ),
    ])
}

/// The requests and responses of signing in and out.
fn traffic(site: Site) -> Part {
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
                en: "The sign-in's answer carries the cookie in a `Set-Cookie` header, in the same response as the patch that sends the browser home. The browser stores the cookie before it moves, so the request for the home page already carries it:",
                ja: "サインインの応答は、ブラウザをホームへ移す差分と同じ応答の `Set-Cookie` ヘッダーで Cookie を運びます。ブラウザは移動する前に Cookie を保存するので、ホームのページへのリクエストにはもう Cookie が付いています。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "HTTP/1.1 200 OK
set-cookie: session=NopwG88FsWT6bJHopbZRSw; HttpOnly; SameSite=Lax; Secure; Path=/",
        ),
        para(
            site,
            L {
                en: "The sign-out's answer, beside the patch that sends the browser to the sign-in page, replaces the cookie with an empty one that has already expired, and the browser deletes it:",
                ja: "サインアウトの応答は、ブラウザをサインインのページへ移す差分と一緒に、Cookie を期限切れの空の Cookie で置き換えます。ブラウザはそれを削除します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "HTTP/1.1 200 OK
set-cookie: session=; Path=/; Max-Age=0; Expires=Tue, 28 Oct 2025 16:14:13 GMT",
        ),
    ])
}

/// The password check that `checked` stands for.
fn passwords(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Checking the password",
                ja: "パスワードの検査",
            },
        ),
        para(
            site,
            L {
                en: "`checked` in the sample stands for the password check. The `auth` example keeps each password as an Argon2 hash that [password-auth](https://docs.rs/password-auth/1.0) writes, and checks a sign-in with `verify_password` on `tokio::task::spawn_blocking`, because Argon2 spends its time on the CPU on purpose, about 10 ms in a release build. A name no member has is checked against a hash as well, so the answer takes as long as a member's and does not tell which names exist.",
                ja: "サンプルの `checked` はパスワードの検査の代わりです。`auth` の例は、[password-auth](https://docs.rs/password-auth/1.0) が書く Argon2 のハッシュでパスワードを持ち、サインインを `tokio::task::spawn_blocking` の中の `verify_password` で検査します。Argon2 は意図的に CPU に時間をかける作りで、リリースビルドで約 10 ms かかるためです。どのメンバーも持たない名前もハッシュと照合するので、応答にかかる時間はメンバーの名前のときと変わらず、どの名前が存在するかは応答からわかりません。",
            },
        ),
    ])
}
