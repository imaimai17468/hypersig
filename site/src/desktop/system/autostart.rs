//! The page on starting the application at login: `Os::autostart` and the
//! entry each system keeps.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Autostart).leads()),
        api(site),
        systems(site),
    ])
}

/// `Os::autostart` and an example.
fn api(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Turning it on and off",
                ja: "有効にする、無効にする",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Autostart<'_> {\n    pub async fn enable(&self) -> Result<(), Error>;\n    pub async fn disable(&self) -> Result<(), Error>;\n    pub async fn is_enabled(&self) -> Result<bool, Error>;\n}",
        ),
        para(
            site,
            L {
                en: "[`Os::autostart`](api:hypersig_desktop/struct.Os.html#method.autostart) returns the system's entry that starts the application when the person logs in. `enable` writes it, `disable` removes it, and `is_enabled` reads whether it is there, so a settings page shows a switch in the state the system holds. A chat application with a tray starts this way and waits in the tray:",
                ja: "[`Os::autostart`](api:hypersig_desktop/struct.Os.html#method.autostart) は、利用者がログインしたときにアプリを起動する OS の登録を返します。`enable` で登録し、`disable` で取り除き、`is_enabled` で登録があるかを読むので、設定のページは OS の状態どおりのスイッチを表示できます。トレイを持つチャットのアプリは、こうして起動してトレイで待ちます。",
            },
        ),
        show(Snippet::DesktopAutostart),
        para(
            site,
            L {
                en: "Each fails with [`ErrorKind::Autostart`](api:hypersig_desktop/enum.ErrorKind.html#variant.Autostart) when the entry cannot be read or written. In a test, the `FakeOs` keeps whether it is on.",
                ja: "登録を読み書きできないときは、どれも [`ErrorKind::Autostart`](api:hypersig_desktop/enum.ErrorKind.html#variant.Autostart) で失敗します。テストでは、`FakeOs` が有効かどうかを保持します。",
            },
        ),
    ])
}

/// What each system keeps.
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
                    en: "The entry is named by the identifier `App::new` takes, and starts the program running now: on Linux the AppImage the person installed, which `APPIMAGE` names, and elsewhere the running binary. A run from `cargo run` writes an entry for the binary under `target`.",
                    ja: "登録には `App::new` に渡した識別子の名前が付き、今動いているプログラムを起動します。Linux では `APPIMAGE` が指す、利用者がインストールした AppImage を、それ以外では動いているバイナリを起動します。`cargo run` で動かすと、`target` の下のバイナリが登録されます。",
                },
                L {
                    en: "macOS keeps a launch agent in `~/Library/LaunchAgents`, Windows a value under the `Run` key of the registry, and Linux a desktop entry in `~/.config/autostart`, which the `auto-launch` crate writes. On macOS, a run saw `enable` write the launch agent, `is_enabled` read it back, and `disable` remove it.",
                    ja: "macOS は `~/Library/LaunchAgents` に起動エージェントを、Windows はレジストリの `Run` キーに値を、Linux は `~/.config/autostart` にデスクトップエントリを置きます。どれも `auto-launch` クレートが書き込みます。macOS では、`enable` で起動エージェントが書かれ、`is_enabled` がそれを読み取り、`disable` で取り除かれることを実行して確かめました。",
                },
            ],
        ),
    ])
}
