//! The page on the tray icon: `Os::tray`, `Tray`, `TrayItem`, and how the
//! application keeps running with no window.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Tray).leads()),
        api(site),
        running(site),
        systems(site),
    ])
}

/// `Os::tray` and an example.
fn api(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Showing a tray",
                ja: "トレイを表示する",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Os {\n    pub async fn tray(&self, tray: Tray) -> Result<(), Error>;\n}\n\nimpl Tray {\n    pub fn new(png: impl Into<Arc<[u8]>>, quit: impl Into<String>) -> Self;\n    pub fn tooltip(self, text: impl Into<String>) -> Self;\n    pub fn item(self, item: &TrayItem) -> Self;\n}\n\nimpl TrayItem {\n    pub fn new(label: impl Into<String>) -> Self;\n    pub fn clicks(&self) -> Clicks;\n}",
        ),
        para(
            site,
            L {
                en: "[`Tray::new`](api:hypersig_desktop/struct.Tray.html#method.new) takes the icon as the bytes of a PNG and the label of the Quit item, which the menu always ends with. Each [`TrayItem`](api:hypersig_desktop/struct.MenuItem.html) is a value the application keeps: `clicks` returns the clicks on it from then on, and `Clicks::next` waits for the next one. No item is named by a string the application matches, so a click reaches only the code that holds its item. [`Os::tray`](api:hypersig_desktop/struct.Os.html#method.tray) shows the tray, in place of one shown before:",
                ja: "[`Tray::new`](api:hypersig_desktop/struct.Tray.html#method.new) は、アイコンを PNG のバイト列として受け取り、終了の項目の名前も受け取ります。メニューの最後には必ずこの終了の項目が入ります。[`TrayItem`](api:hypersig_desktop/struct.MenuItem.html) はアプリが持っておく値で、`clicks` はそれ以降の項目のクリックを返し、`Clicks::next` は次のクリックを待ちます。項目を文字列で名付けて照合することはないので、クリックはその項目を持っているコードにだけ届きます。[`Os::tray`](api:hypersig_desktop/struct.Os.html#method.tray) はトレイを表示します。前に表示したトレイがあれば置き換えます。",
            },
        ),
        show(Snippet::DesktopTray),
        para(
            site,
            L {
                en: "It fails with [`ErrorKind::Tray`](api:hypersig_desktop/enum.ErrorKind.html#variant.Tray) when the icon is not a PNG of RGBA or gray-and-alpha pixels, or the system has no tray. In a test, `FakeOs::tray` returns the tray shown last, `Tray::labels` its items' labels, and `FakeOs::click_tray_item` clicks an item.",
                ja: "アイコンが RGBA かグレーとアルファの画素の PNG でないとき、または OS にトレイがないときは、[`ErrorKind::Tray`](api:hypersig_desktop/enum.ErrorKind.html#variant.Tray) で失敗します。テストでは、`FakeOs::tray` が最後に表示したトレイを返し、`Tray::labels` がその項目の名前を返し、`FakeOs::click_tray_item` が項目をクリックします。",
            },
        ),
    ])
}

/// Running with no window.
fn running(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Running with no window",
                ja: "ウィンドウがなくても動き続ける",
            },
        ),
        para(
            site,
            L {
                en: "Without a tray, closing the last window ends the application. While a tray is shown, the application keeps running with no window, as a chat application does, and an item such as Show above opens a window again. The tray's Quit item, Quit in the macOS menu, and Ctrl-C in the terminal end it.",
                ja: "トレイがなければ、最後のウィンドウを閉じるとアプリは終了します。トレイを表示している間は、チャットのアプリのように、ウィンドウがなくてもアプリは動き続けます。上の例の Show notes のような項目で、ウィンドウをもう一度開けます。トレイの終了の項目、macOS のメニューの Quit、ターミナルの Ctrl-C で終了します。",
            },
        ),
    ])
}

/// Where each system puts it.
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
                    en: "macOS puts the icon in the menu bar, and Windows in the notification area of the taskbar.",
                    ja: "macOS ではメニューバーに、Windows ではタスクバーの通知領域にアイコンが入ります。",
                },
                L {
                    en: "On Linux the icon is offered over D-Bus as a StatusNotifierItem, which KDE shows and GNOME shows only with an extension such as AppIndicator Support. Without a D-Bus session, `Os::tray` fails.",
                    ja: "Linux では、アイコンを D-Bus の StatusNotifierItem として渡します。KDE はこれを表示し、GNOME は AppIndicator Support のような拡張機能があるときだけ表示します。D-Bus のセッションがなければ、`Os::tray` は失敗します。",
                },
                L {
                    en: "On macOS, `Os::tray` returned `Ok` and Ctrl-C ended the application with the tray shown; the icon has not been looked at on screen on any system yet.",
                    ja: "macOS では、`Os::tray` が `Ok` を返し、トレイを表示したまま Ctrl-C でアプリが終了することを確かめました。アイコンが画面に表示されるかは、まだどの OS でも見ていません。",
                },
            ],
        ),
    ])
}
