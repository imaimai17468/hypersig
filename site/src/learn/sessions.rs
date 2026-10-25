//! The page on signing in: a session kept by `tower-sessions`, the extractor
//! that reads the signed-in member from it, the session's cookie, and where
//! the sessions are stored.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, note, para, parts, title};

pub fn sessions(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Sessions).title(site)),
        lead(site, Doc::Learn(Learn::Sessions).leads()),
        layer(site),
        member(site),
        cookie(site),
        stores(site),
    ])
}

/// The layer, the dependency, and the sample.
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
                ja: "セッションはサーバーに置く記録で、ブラウザはその id を入れた Cookie で記録を指します。[tower-sessions](https://docs.rs/tower-sessions/0.14) がこの記録を持ち、id を作り、`Set-Cookie` ヘッダーを書くので、ハンドラはセッションの値を読み書きするだけで済みます。`Cargo.toml` に次の行を足してください。",
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
                en: "The sample uses 0.14 although 0.15 is out, because `tower-sessions-sqlx-store` 0.15, `tower-sessions-redis-store` 0.16, and `axum-login` 0.18 depend on `tower-sessions-core` 0.14, and a store built on it does not implement the `SessionStore` trait of a 0.15 layer. The repository's [`auth`](https://github.com/imaimai17468/hypersig/tree/main/examples/auth) example uses the same extractor, sign-in, and sign-out behind a sign-in form.",
                ja: "0.15 が出ていますが、サンプルは 0.14 を使います。`tower-sessions-sqlx-store` 0.15、`tower-sessions-redis-store` 0.16、`axum-login` 0.18 が `tower-sessions-core` 0.14 に依存していて、それを使うストアは 0.15 のレイヤーの `SessionStore` トレイトを実装しないためです。リポジトリの [`auth`](https://github.com/imaimai17468/hypersig/tree/main/examples/auth) の例は、同じエクストラクタ、サインイン、サインアウトをサインインのフォームと一緒に使います。",
            },
        ),
        show(Snippet::Sessions),
        para(
            site,
            L {
                en: "`SessionManagerLayer` wraps the router, so it runs around every page, action, and stream. It reads the id from the cookie and hands each handler a `Session`, which loads the record from the store the first time a handler reads it. After the handler returns its response, the layer saves a session the handler changed and sets the cookie on that response, or removes the cookie of a session the handler emptied. An action's answer carries the header like any response, so the cookie the sign-in sets is in the browser before the patch sends the browser to the home page. The layer saves nothing when the status is a 5xx, and a change a stream makes to the session after its response has started is not saved.",
                ja: "`SessionManagerLayer` はルーターを包むので、すべてのページ、アクション、ストリームの前後で動きます。Cookie から id を読み、各ハンドラに `Session` を渡します。`Session` は、ハンドラが初めて値を読むときにストアから記録を読み込みます。ハンドラが応答を返したあと、レイヤーは、ハンドラが変えたセッションを保存してその応答に Cookie を付けるか、ハンドラが空にしたセッションの Cookie を消します。アクションの応答もほかの応答と同じようにヘッダーを運ぶので、サインインで付けた Cookie は、差分がブラウザをホームへ移すより前にブラウザに入っています。ステータスが 5xx のときレイヤーは何も保存せず、ストリームが応答を始めたあとにセッションを変えても、その変更は保存されません。",
            },
        ),
    ])
}

/// The extractor, signing in, and signing out.
fn member(site: Site) -> Part {
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
                en: "`Member` is an extractor, so a handler that takes it runs only for a signed-in visitor, and its rejection is the answer to everyone else. A page answers with a redirect to the sign-in page. An action answers with a 401, because the runtime's `fetch` would follow a redirect and receive the sign-in page's HTML, which it cannot apply, and the 401's text reaches the signal the call's `error` names. [`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) tells the two apart. `PageResponse::redirect(LOGIN.url(()))` and `Patches::navigate` send the browser to another page, as [Redirects and navigation](learn/pages/#redirects-and-navigation) describes. A call with [`Call::status`](api:hypersig/struct.Call.html#method.status) also puts the 401 in an `i32` signal, so the page can show a link to the sign-in page when the signal equals `StatusCode::UNAUTHORIZED`, as the `auth` example does. `axum-login`'s `login_required!` answers every request it guards in one way, a redirect or a 401, chosen when the router is built, which is why the sample writes its own extractor.",
                ja: "`Member` はエクストラクタなので、これを受け取るハンドラはサインイン中の訪問者に対してだけ動き、それ以外の訪問者にはエクストラクタの拒否が応答になります。ページにはサインインのページへのリダイレクトを返します。アクションには 401 を返します。ランタイムの `fetch` はリダイレクトをたどってサインインのページの HTML を受け取ってしまい、それを差分として当てられないためです。401 の本文は、呼び出しの `error` が指すシグナルに入ります。この 2 つは [`RequestKind::of`](api:hypersig/enum.RequestKind.html#method.of) で見分けます。`PageResponse::redirect(LOGIN.url(()))` と `Patches::navigate` はブラウザを別のページへ送ります。詳しくは [リダイレクトとページの移動](learn/pages/#redirects-and-navigation) で説明します。[`Call::status`](api:hypersig/struct.Call.html#method.status) を付けた呼び出しは 401 を `i32` のシグナルにも入れるので、ページはそのシグナルが `StatusCode::UNAUTHORIZED` と等しいときにサインインのページへのリンクを出せます。`auth` の例はそうしています。`axum-login` の `login_required!` は、守るリクエストすべてに、ルーターを組むときに選んだリダイレクトか 401 のどちらか一方で応答するので、サンプルはエクストラクタを自分で書いています。",
            },
        ),
        para(
            site,
            L {
                en: "`checked` in the sample stands for the password check. The `auth` example keeps each password as an Argon2 hash that [password-auth](https://docs.rs/password-auth/1.0) writes, and checks a sign-in with `verify_password` on `tokio::task::spawn_blocking`, because Argon2 spends its time on the CPU on purpose, about 10 ms in a release build. A name no member has is checked against a hash as well, so the answer takes as long as a member's and does not tell which names exist.",
                ja: "サンプルの `checked` はパスワードの検査の代わりです。`auth` の例は、[password-auth](https://docs.rs/password-auth/1.0) が書く Argon2 のハッシュでパスワードを持ち、サインインを `tokio::task::spawn_blocking` の中の `verify_password` で検査します。Argon2 はわざと CPU に時間をかける作りで、リリースビルドで約 10 ms かかるためです。どのメンバーも持たない名前もハッシュと照合するので、応答にかかる時間はメンバーの名前のときと変わらず、どの名前が存在するかは応答からわかりません。",
            },
        ),
        para(
            site,
            L {
                en: "The sign-in calls `cycle_id` before it puts the name in the session. Without it, someone who placed the id of their own session in the visitor's browser would hold the same id after the visitor signs in, and would be signed in as the visitor. With it, the session that holds the member has a new id, and the store no longer has the old one. The sign-out calls `flush`, which deletes the record from the store and empties the session, and the layer then removes the cookie.",
                ja: "サインインでは、名前をセッションに入れる前に `cycle_id` を呼びます。これがないと、自分のセッションの id を訪問者のブラウザに仕込んだ人が、訪問者のサインイン後も同じ id を持ち続け、訪問者としてサインインした状態になります。`cycle_id` を呼べば、メンバーを持つセッションの id は新しくなり、古い id はストアから消えます。サインアウトでは `flush` を呼びます。`flush` はストアから記録を消してセッションを空にし、レイヤーがそのあと Cookie を消します。",
            },
        ),
    ])
}

/// The attributes of the session's cookie.
fn cookie(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The cookie",
                ja: "Cookie",
            },
        ),
        para(
            site,
            L {
                en: "The layer's cookie is named `id` and is `HttpOnly`, `Secure`, and `SameSite=Strict` unless the builder says otherwise. With `Strict`, the browser leaves the cookie off a visit that follows a link from another site, such as a link in an email, and that visit lands on the sign-in page. The sample sets `Lax`, which sends the cookie on such a visit. An action needs no `Strict`, because it refuses a request another site sends, as [Security](learn/security/) lists. `Secure` keeps the cookie to HTTPS. Chrome also stores it from `http://127.0.0.1`, where the repository's browser tests run, and other browsers were not checked.",
                ja: "レイヤーの Cookie は、ビルダーで変えない限り、名前が `id` で、`HttpOnly`、`Secure`、`SameSite=Strict` が付きます。`Strict` では、メールのリンクのように別のサイトのリンクからの訪問に、ブラウザが Cookie を付けません。その訪問はサインインのページに送られます。サンプルは `Lax` にして、そうした訪問にも Cookie が付くようにしています。アクションは別のサイトからのリクエストを拒否するので（[セキュリティ](learn/security/) に一覧があります）、`Strict` は要りません。`Secure` は Cookie を HTTPS に限ります。Chrome は `http://127.0.0.1` からもこの Cookie を保存し、リポジトリのブラウザのテストはそこで動いています。ほかのブラウザでは確かめていません。",
            },
        ),
        para(
            site,
            L {
                en: "Without an expiry the cookie has no `Max-Age`, so the browser drops it when it closes. With `with_expiry(Expiry::OnInactivity(duration))`, the cookie gets a `Max-Age` of `duration` each time the layer saves the session, and the visitor stays signed in across a restart of the browser. The layer saves only a session a handler changed, so a visitor who only reads pages is signed out `duration` after the last change. `with_always_save(true)` saves the session on every response, so the time counts from the last request.",
                ja: "有効期限を決めないと Cookie には `Max-Age` が付かず、ブラウザを閉じると消えます。`with_expiry(Expiry::OnInactivity(duration))` にすると、レイヤーがセッションを保存するたびに Cookie の `Max-Age` が `duration` になり、ブラウザを起動し直してもサインインしたままになります。ただし、レイヤーが保存するのはハンドラが変えたセッションだけなので、ページを読むだけの訪問者は、最後の変更から `duration` たつとサインアウトします。`with_always_save(true)` にするとどの応答でも保存するので、最後のリクエストから数えるようになります。",
            },
        ),
    ])
}

/// Where the sessions are stored, for one process and for more.
fn stores(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Where the sessions are stored",
                ja: "セッションを置く場所",
            },
        ),
        para(
            site,
            L {
                en: "`MemoryStore` keeps the sessions in the process's memory. A restart signs every visitor out, and a second process does not see them, so a request that a load balancer sends to another instance is signed out. That is the same limit as a `broadcast` channel in [Updates with more than one process](learn/database/#updates-with-more-than-one-process). On a host that runs more than one instance, or that starts and stops instances as requests come as Vercel does, pass the layer a store that keeps the sessions in a database. `tower-sessions-sqlx-store` keeps them in PostgreSQL, SQLite, or MySQL through sqlx 0.8, so an application on sqlx 0.9 builds both versions and cannot hand the store its own pool. `tower-sessions-redis-store` connects through the `fred` client, so it does not share a connection with the `redis` crate that `examples/redis` uses.",
                ja: "`MemoryStore` はセッションをプロセスのメモリに置きます。プロセスを起動し直すと全員がサインアウトし、2 つ目のプロセスからは見えないので、ロードバランサーが別のインスタンスに送ったリクエストはサインインしていない扱いになります。[プロセスが複数あるときの更新](learn/database/#updates-with-more-than-one-process) の `broadcast` のチャネルと同じ制限です。インスタンスを複数動かすホストや、Vercel のようにリクエストに合わせてインスタンスを起動したり止めたりするホストでは、セッションをデータベースに置くストアをレイヤーに渡してください。`tower-sessions-sqlx-store` は sqlx 0.8 を通して PostgreSQL、SQLite、MySQL に置きます。そのため sqlx 0.9 を使うアプリでは両方のバージョンがビルドされ、アプリのプールをストアに渡せません。`tower-sessions-redis-store` は `fred` クライアントでつなぐので、`examples/redis` が使う `redis` クレートと接続を共有しません。",
            },
        ),
        note(
            site,
            L {
                en: "On Cloudflare Workers, tower-sessions 0.14 needs two features in the worker's `Cargo.toml`: `getrandom = { version = \"0.2\", features = [\"js\"] }`, without which it does not build for `wasm32-unknown-unknown`, and `time = { version = \"0.3\", features = [\"wasm-bindgen\"] }`, without which saving a session panics with `time not implemented on this platform`. With both, the layer kept a session under `wrangler dev`. A worker's isolates share no memory, so `MemoryStore` there signs a visitor out whenever a request reaches another isolate. A store that keeps the sessions in D1 or KV implements `SessionStore`, and this guide has not written one.",
                ja: "Cloudflare Workers では、tower-sessions 0.14 に Worker の `Cargo.toml` で 2 つのフィーチャーが要ります。1 つは `getrandom = { version = \"0.2\", features = [\"js\"] }` で、これがないと `wasm32-unknown-unknown` 向けにビルドできません。もう 1 つは `time = { version = \"0.3\", features = [\"wasm-bindgen\"] }` で、これがないとセッションを保存するときに `time not implemented on this platform` でパニックします。両方を入れると、`wrangler dev` でレイヤーがセッションを保てました。Worker の isolate どうしはメモリを共有しないので、そこで `MemoryStore` を使うと、リクエストが別の isolate に届くたびに訪問者はサインインしていない扱いになります。セッションを D1 や KV に置くストアは `SessionStore` を実装すれば作れますが、このガイドではまだ書いていません。",
            },
        ),
    ])
}
