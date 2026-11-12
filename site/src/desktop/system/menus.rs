//! The page on the menu bar: `Os::menus`, `Menu`, `MenuItem`, and where each
//! system shows it.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Menus).leads()),
        api(site),
        systems(site),
    ])
}

/// `Os::menus` and an example.
fn api(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Putting menus in the bar",
                ja: "メニューバーにメニューを置く",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Os {\n    pub fn menus(&self, menus: impl IntoIterator<Item = Menu>) -> Result<(), Error>;\n}\n\nimpl Menu {\n    pub fn new(title: impl Into<String>) -> Self;\n    pub fn item(self, item: &MenuItem) -> Self;\n    pub fn separator(self) -> Self;\n}",
        ),
        para(
            site,
            L {
                en: "A [`Menu`](api:hypersig_desktop/struct.Menu.html) is a title and its items, and a [`MenuItem`](api:hypersig_desktop/struct.MenuItem.html) is a value the application keeps, as a tray's item is: `clicks` returns its clicks from then on. No item is named by a string the application matches, so a click reaches only the code that holds the item. [`Os::menus`](api:hypersig_desktop/struct.Os.html#method.menus) puts the menus in the bar, in place of the ones put there before:",
                ja: "[`Menu`](api:hypersig_desktop/struct.Menu.html) はタイトルとその項目からなります。[`MenuItem`](api:hypersig_desktop/struct.MenuItem.html) はトレイの項目と同じくアプリが持っておく値で、`clicks` はそれ以降のクリックを返します。項目を文字列で名付けて照合することはないので、クリックはその項目を持っているコードにだけ届きます。[`Os::menus`](api:hypersig_desktop/struct.Os.html#method.menus) はメニューをメニューバーに置きます。前に置いたメニューがあれば置き換えます。",
            },
        ),
        show(Snippet::DesktopMenus),
        para(
            site,
            L {
                en: "A tray's items are `MenuItem`s too (`TrayItem` names the same type), so one item can sit in both. In a test, `FakeOs::menus` returns the menus put in the bar last, and `FakeOs::click_menu_item` clicks an item.",
                ja: "トレイの項目も `MenuItem` なので（`TrayItem` は同じ型の別名です）、1 つの項目を両方に置けます。テストでは、`FakeOs::menus` が最後に置いたメニューを返し、`FakeOs::click_menu_item` で項目をクリックします。",
            },
        ),
    ])
}

/// Where each system shows it.
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
                    en: "On macOS the menus go between the Edit and Window menus of the application's menu bar, which keeps the application, Edit, and Window menus that copying, pasting, and quitting need. A run saw a menu appear there and System Events click its item through to the code that held it.",
                    ja: "macOS では、アプリのメニューバーの Edit と Window のあいだにメニューが入ります。コピー、ペースト、終了に必要なアプリ、Edit、Window のメニューは残ります。メニューがそこに表示され、System Events でクリックした項目がそれを持つコードに届くことを実行して確かめました。",
                },
                L {
                    en: "On Linux a bar holding the menus is put at the top of every window, opened then or later, above the page. No run has looked at it yet.",
                    ja: "Linux では、メニューを入れたバーが、そのとき開いているウィンドウにもあとで開くウィンドウにも、ページの上に置かれます。まだどの実行でも見ていません。",
                },
                L {
                    en: "Windows shows no menu bar: muda puts one in a window only through an `unsafe` call, and this crate forbids `unsafe`. `Os::menus` returns `Ok(())` there and shows nothing.",
                    ja: "muda がウィンドウにメニューバーを置くには `unsafe` の呼び出しが必要で、このクレートは `unsafe` を禁止しているので、Windows ではメニューバーを表示しません。`Os::menus` は `Ok(())` を返し、何も表示しません。",
                },
                L {
                    en: "A context menu of the system's is not offered, for the same reason on Windows and macOS. A menu shown inside the page, as Discord's are, needs a `contextmenu` event that hypersig does not have yet.",
                    ja: "OS の右クリックメニューは Windows と macOS で同じ理由から用意していません。Discord のようにページの中に出すメニューには `contextmenu` イベントが必要ですが、hypersig にはまだありません。",
                },
            ],
        ),
    ])
}
