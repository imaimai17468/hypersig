//! The page on the count on the application's icon: `Os::badge`, and how
//! each system shows it.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Badge).leads()),
        api(site),
        systems(site),
        next(site),
    ])
}

/// `Os::badge` and an example.
fn api(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Setting the count",
                ja: "数を設定する",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Os {\n    pub fn badge(&self, count: u32) -> Result<(), Error>;\n}",
        ),
        para(
            site,
            L {
                en: "[`Os::badge`](api:hypersig_desktop/struct.Os.html#method.badge) shows `count` on the application's icon, and `0` clears it. The count stays until the next call, whichever window is open or focused, and goes when the application exits. It fails only while the application is closing. A handler that changes the number of unread messages sets the count as well as the signal the page shows:",
                ja: "[`Os::badge`](api:hypersig_desktop/struct.Os.html#method.badge) はアプリのアイコンに `count` を表示し、`0` で消します。数は、どのウィンドウが開いていても、どれにフォーカスがあっても、次に呼ぶまで残り、アプリが終わると消えます。失敗するのはアプリが終了している途中だけです。未読の数を変えるハンドラは、ページに表示するシグナルと一緒にアイコンの数も設定します。",
            },
        ),
        show(Snippet::DesktopBadge),
        para(
            site,
            L {
                en: "In a test, `FakeOs::badge` returns the count the handlers last set, `0` when none is shown.",
                ja: "テストでは、`FakeOs::badge` がハンドラが最後に設定した数を返します。表示していなければ `0` です。",
            },
        ),
    ])
}

/// What each system shows.
fn systems(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "How each system shows it",
                ja: "OS ごとの表示",
            },
        ),
        table_of(
            site,
            L {
                en: &["System", "Where", "What"],
                ja: &["OS", "場所", "表示"],
            },
            L {
                en: &[
                    &["macOS", "The Dock icon", "The number"],
                    &[
                        "Linux",
                        "The launcher entry of `<program>.desktop`",
                        "The number, where the dock reads Unity's launcher API, as Ubuntu's does",
                    ],
                    &[
                        "Windows",
                        "The taskbar button of each window",
                        "A red dot, since a button has no room for a number",
                    ],
                ][..],
                ja: &[
                    &["macOS", "Dock のアイコン", "数"],
                    &[
                        "Linux",
                        "`<program>.desktop` のランチャーの項目",
                        "数。Ubuntu の Dock のように Unity のランチャーの API を読む Dock に表示される",
                    ],
                    &[
                        "Windows",
                        "各ウィンドウのタスクバーのボタン",
                        "赤い点。ボタンには数を書く余地がないため",
                    ],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "On Linux, `<program>` is the name of the running executable, which is the name cargo-packager gives the desktop file of a Debian package and an AppImage. GNOME without an extension shows no count. On macOS the Dock's label was read back as the count set; on Windows and Linux the badge has not been looked at on screen yet.",
                ja: "Linux の `<program>` は実行中の実行ファイルの名前で、cargo-packager が Debian のパッケージと AppImage のデスクトップファイルに付ける名前と同じです。拡張機能のない GNOME では数は表示されません。macOS では、設定した数が Dock のラベルに入っていることを読み出して確かめました。Windows と Linux では、まだ画面で確かめていません。",
            },
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
                en: "[Testing without a window](desktop/testing/): `FakeOs` and what it records.",
                ja: "[ウィンドウなしのテスト](desktop/testing/): `FakeOs` と、それが記録するもの。",
            }],
        ),
    ])
}
