//! The page on the checks that keep other programs off the server.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

/// A request another program on the machine sends, and the answer.
const REFUSED: &str = "GET / HTTP/1.1
host: 127.0.0.1:52817

HTTP/1.1 403 Forbidden";

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Desktop(DesktopPage::Security).leads()),
        heading(
            site,
            L {
                en: "Which requests are answered",
                ja: "応答するリクエスト",
            },
        ),
        para(
            site,
            L {
                en: "[`App::run`](api:hypersig_desktop/struct.App.html#method.run) serves the application's router on `127.0.0.1`, which no other machine can reach. Any program on the same machine can open a connection to that port, so the server checks every request before the router sees it, including a request for a path no route matches:",
                ja: "[`App::run`](api:hypersig_desktop/struct.App.html#method.run) はアプリのルーターを `127.0.0.1` で配信します。このアドレスにはほかのマシンからは接続できません。同じマシンのプログラムならどれでもそのポートに接続できるので、サーバーはルーターに渡す前にすべてのリクエストを確かめます。どのルートにも一致しないパスへのリクエストも対象です。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Request", "Answer"][..],
                ja: &["リクエスト", "応答"][..],
            },
            L {
                en: &[
                    &[
                        "`Host` is not `127.0.0.1:<port>`, as with `localhost:<port>` or another domain",
                        "`403 Forbidden`",
                    ][..],
                    &[
                        "`Host` is right, without the cookie the first window received",
                        "`403 Forbidden`",
                    ],
                    &["`Host` is right, with that cookie", "the router's answer"],
                ][..],
                ja: &[
                    &[
                        "`Host` が `127.0.0.1:<port>` でない（`localhost:<port>` や別のドメインなど）",
                        "`403 Forbidden`",
                    ][..],
                    &[
                        "`Host` は正しいが、最初のウィンドウが受け取った Cookie がない",
                        "`403 Forbidden`",
                    ],
                    &["`Host` が正しく、その Cookie がある", "ルーターの応答"],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "The answer to the first window's first page sets a cookie, once in each run of the application. The web views of all the windows are made with one data store, which holds the cookies, so every window of the application sends it. The cookie is `HttpOnly`, so no script on a page reads it. A request from `curl`, from another application, or from a page in the person's browser carries no such cookie and gets this:",
                ja: "最初のウィンドウの最初のページへの応答が Cookie を設定します。設定はアプリを起動するたびに 1 回だけです。すべてのウィンドウの Web ビューは、Cookie を含めて同じ保存先のデータを使うので、どのウィンドウもこの Cookie を送ります。Cookie は `HttpOnly` なので、ページのスクリプトからは読めません。`curl` やほかのアプリ、利用者のブラウザで開いたページからのリクエストにはこの Cookie がないので、次の応答になります。",
            },
        ),
        code::block(Lang::JavaScript, None, REFUSED),
        para(
            site,
            L {
                en: "The `Host` check stops a page of another site whose domain was made to resolve to `127.0.0.1`: its requests name that domain in `Host`. Neither check is something the application configures, and neither can be turned off.",
                ja: "`Host` の確認は、ドメインが `127.0.0.1` に解決されるよう細工された別のサイトのページを止めます。そのページのリクエストは `Host` にそのドメインを書くからです。どちらの確認もアプリで設定するものではなく、無効にすることもできません。",
            },
        ),
        heading(
            site,
            L {
                en: "No window leaves the application",
                ja: "ウィンドウはアプリの外へ移動しない",
            },
        ),
        para(
            site,
            L {
                en: "A window shows only pages of `http://127.0.0.1:<port>/`. A link to another site, or a `window.open` of one, opens it in the person's browser when it is an `http`, `https`, or `mailto` URL, and the window stays on its page. A URL of any other scheme, such as `file:`, is not opened at all. A page of the application asked for with `target=\"_blank\"` or `window.open` opens in a new window of the application.",
                ja: "ウィンドウが表示するのは `http://127.0.0.1:<port>/` のページだけです。別のサイトへのリンクや、別のサイトを開く `window.open` は、URL が `http`、`https`、`mailto` のいずれかなら利用者のブラウザで開き、ウィンドウは元のページのままです。`file:` のようなほかのスキームの URL は開きません。`target=\"_blank\"` や `window.open` で求められたアプリのページは、アプリの新しいウィンドウで開きます。",
            },
        ),
        heading(
            site,
            L {
                en: "Session cookies without `Secure`",
                ja: "`Secure` を付けないセッションの Cookie",
            },
        ),
        para(
            site,
            L {
                en: "WKWebView, which shows the window on macOS, does not send a `Secure` cookie to `http://127.0.0.1`, and neither does Playwright's build of WebKit, the engine of WebKitGTK on Linux. A session layer that marks its cookie `Secure`, as `tower-sessions` does by default, signs no one in inside the window: the requests after signing in arrive without the session cookie, so the server sees no one signed in.",
                ja: "macOS でウィンドウを表示する WKWebView は、`http://127.0.0.1` に `Secure` の Cookie を送りません。Linux の WebKitGTK のエンジンである WebKit を Playwright 向けにビルドしたものも同じです。`tower-sessions` の既定のように Cookie に `Secure` を付けるセッションのレイヤーでは、ウィンドウの中でサインインできません。サインインしたあとのリクエストにセッションの Cookie が付かないので、サーバーからはサインインしていないように見えます。",
            },
        ),
        para(
            site,
            L {
                en: "tower-sessions 0.14's `SessionManagerLayer::with_secure(false)` sets the cookie without `Secure`. Build the router in a function that takes the choice, and pass `false` in the desktop build and `true` in the web build:",
                ja: "tower-sessions 0.14 の `SessionManagerLayer::with_secure(false)` を使うと、Cookie に `Secure` を付けずに設定します。ルーターを作る関数にこの値を引数として渡し、デスクトップ版では `false`、Web 版では `true` を渡します。",
            },
        ),
        show(Snippet::DesktopSecuritySessions),
        para(
            site,
            L {
                en: "Without `Secure`, the browser sends the cookie over plain HTTP. In the desktop build that HTTP stays on `127.0.0.1`, and the checks above answer `403` to every program that is not one of the application's windows, so the session cookie reaches only the application's own server from its own windows. Nothing checks that the web build passes `true`; a web build that passes `false` sends its session cookie over plain HTTP wherever the site is served without HTTPS. [The session cookie](learn/session-cookie/) lists the cookie's other attributes.",
                ja: "`Secure` がないと、ブラウザは暗号化しない HTTP でも Cookie を送ります。デスクトップ版ではその HTTP は `127.0.0.1` から出ず、上の確認がアプリのウィンドウ以外のプログラムすべてに `403` を返します。そのため、セッションの Cookie はアプリ自身のウィンドウからアプリ自身のサーバーにしか届きません。Web 版で `true` を渡しているかどうかは検査されません。Web 版で `false` を渡すと、HTTPS を使わずに配信している場所では、セッションの Cookie が暗号化されない HTTP で送られます。Cookie のほかの属性は [セッションの Cookie](learn/session-cookie/) で説明しています。",
            },
        ),
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        para(
            site,
            L {
                en: "[Signals kept between runs](desktop/persisted-signals/) covers the port the server listens on. [Opening windows](desktop/windows/) opens the application's pages in new windows from a handler.",
                ja: "サーバーが待ち受けるポートについては [起動をまたいで残るシグナル](desktop/persisted-signals/) で説明しています。ハンドラからアプリのページを新しいウィンドウで開く方法は [ウィンドウを開く](desktop/windows/) にあります。",
            },
        ),
    ])
}
