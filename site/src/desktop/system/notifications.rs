//! The page on the system's notifications: `Os::notify`, `Notification`,
//! `Notified::clicked`, and what each system needs.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Notifications).leads()),
        api(site),
        systems(site),
        next(site),
    ])
}

/// `Os::notify` and an example.
fn api(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Showing a notification",
                ja: "通知を表示する",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Os {\n    pub async fn notify(&self, notification: Notification) -> Result<Notified, Error>;\n}\n\nimpl Notification {\n    pub fn new(title: impl Into<String>) -> Self;\n    pub fn body(self, body: impl Into<String>) -> Self;\n}\n\nimpl Notified {\n    pub async fn clicked(self) -> bool;\n}",
        ),
        para(
            site,
            L {
                en: "[`Os::notify`](api:hypersig_desktop/struct.Os.html#method.notify) shows a [`Notification`](api:hypersig_desktop/struct.Notification.html), a title and an optional body, among the system's notifications, and returns a [`Notified`](api:hypersig_desktop/struct.Notified.html). `Notified::clicked` returns `true` once the person clicks the notification and `false` once they dismiss it, so it can wait for as long as the notification stays in the system's list. A handler spawns a task for it and answers the page at once:",
                ja: "[`Os::notify`](api:hypersig_desktop/struct.Os.html#method.notify) は、タイトルと省略できる本文から成る [`Notification`](api:hypersig_desktop/struct.Notification.html) を OS の通知として表示し、[`Notified`](api:hypersig_desktop/struct.Notified.html) を返します。`Notified::clicked` は、利用者が通知をクリックすると `true` を、閉じると `false` を返します。通知が OS の一覧に残っている間はずっと待つことがあるので、ハンドラはこれをタスクで待ち、ページにはすぐに応答します。",
            },
        ),
        show(Snippet::DesktopNotify),
        para(
            site,
            L {
                en: "It fails with [`ErrorKind::Notification`](api:hypersig_desktop/enum.ErrorKind.html#variant.Notification) when the system does not show it, as on Linux with no notification server running. In a test, `FakeOs::notifications` returns each notification the handlers showed, and `FakeOs::click_notification` clicks one.",
                ja: "OS が通知を表示しなかったとき（通知のサーバーが動いていない Linux など）は、[`ErrorKind::Notification`](api:hypersig_desktop/enum.ErrorKind.html#variant.Notification) で失敗します。テストでは、`FakeOs::notifications` がハンドラが表示した通知を順に返し、`FakeOs::click_notification` が通知をクリックします。",
            },
        ),
    ])
}

/// What each system needs.
fn systems(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "On each system",
                ja: "OS ごとの違い",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "macOS shows the notification under the application installed with the identifier `App::new` names, which a package from [Packaging](desktop/packaging/) carries. A run of the executable outside that bundle, such as `cargo run`, shows it under another application's name. macOS asks the person once whether the application may notify, and shows nothing if they refuse.",
                    ja: "macOS は `App::new` で指定した識別子でインストールされたアプリの名前で通知を表示します。[パッケージの作成](desktop/packaging/) で作ったパッケージには、この識別子が入っています。`cargo run` のようにバンドルの外で実行ファイルを動かすと、ほかのアプリの名前で表示されます。macOS は通知してよいかを利用者に一度だけ尋ね、拒否されると何も表示しません。",
                },
                L {
                    en: "Linux sends it to the notification server of the desktop over D-Bus, which GNOME, KDE, and most others run.",
                    ja: "Linux では、デスクトップの通知のサーバーに D-Bus で送ります。GNOME や KDE をはじめ、多くのデスクトップがこのサーバーを動かしています。",
                },
                L {
                    en: "Windows shows it as a toast.",
                    ja: "Windows ではトーストとして表示します。",
                },
                L {
                    en: "On macOS, `Os::notify` returned `Ok`; whether the notification appeared on screen has not been looked at on any system yet.",
                    ja: "macOS では `Os::notify` が `Ok` を返すことを確かめました。通知が画面に表示されるかは、まだどの OS でも見ていません。",
                },
            ],
        ),
    ])
}

fn next(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        bullets(
            site,
            &[L {
                en: "[The count on the icon](desktop/badge/): `Os::badge`, for the number of unread messages.",
                ja: "[アイコンの数](desktop/badge/): 未読のメッセージの数を表示する `Os::badge`。",
            }],
        ),
    ])
}
