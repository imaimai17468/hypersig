//! The page on global shortcuts: `Os::shortcut`, `Shortcut`, and where each
//! system delivers them.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Shortcuts).leads()),
        api(site),
        systems(site),
    ])
}

/// `Os::shortcut` and an example.
fn api(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Registering a shortcut",
                ja: "ショートカットを登録する",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Shortcut {\n    pub const fn new(modifiers: Modifiers, key: Key) -> Self;\n}\n\nimpl Os {\n    pub async fn shortcut(&self, shortcut: Shortcut) -> Result<Presses, Error>;\n}\n\nimpl Presses {\n    pub async fn next(&mut self) -> Option<Press>;\n}",
        ),
        para(
            site,
            L {
                en: "A page's own shortcuts work only while its window has focus. [`Os::shortcut`](api:hypersig_desktop/struct.Os.html#method.shortcut) registers a [`Shortcut`](api:hypersig_desktop/struct.Shortcut.html) with the system for the rest of the run, so the application hears it while another application has focus, as push-to-talk does. The key is a [`Key`](api:hypersig_desktop/enum.Key.html) and the modifiers are [`Modifiers`](api:hypersig_desktop/struct.Modifiers.html) joined with `and`, so a shortcut is never a string. `Shortcut::new` in a `const` fails to compile without a modifier unless the key is a function key, since the application would take that key from every other application. `Modifiers::PRIMARY` is Command on macOS and Control elsewhere.",
                ja: "ページのショートカットは、そのウィンドウにフォーカスがあるときだけ働きます。[`Os::shortcut`](api:hypersig_desktop/struct.Os.html#method.shortcut) は [`Shortcut`](api:hypersig_desktop/struct.Shortcut.html) を実行中ずっと OS に登録するので、プッシュトゥトークのように、ほかのアプリにフォーカスがあってもアプリがキーを受け取れます。キーは [`Key`](api:hypersig_desktop/enum.Key.html)、修飾キーは `and` でつないだ [`Modifiers`](api:hypersig_desktop/struct.Modifiers.html) で表すので、ショートカットを文字列で書くことはありません。修飾キーのないキーを登録するとほかのすべてのアプリからそのキーを奪うので、`const` の中の `Shortcut::new` は、ファンクションキーのほかは修飾キーがないとコンパイルできません。`Modifiers::PRIMARY` は、macOS では Command、それ以外では Control です。",
            },
        ),
        para(
            site,
            L {
                en: "Each call returns its own [`Presses`](api:hypersig_desktop/struct.Presses.html), and `next` waits for the next `Press::Pressed` or `Press::Released`, so a key held down can be told from one tapped:",
                ja: "呼び出すたびに別の [`Presses`](api:hypersig_desktop/struct.Presses.html) が返り、`next` は次の `Press::Pressed` か `Press::Released` を待ちます。そのため、押し続けているのか、一度押しただけなのかを区別できます。",
            },
        ),
        show(Snippet::DesktopShortcut),
        para(
            site,
            L {
                en: "It fails with [`ErrorKind::Shortcut`](api:hypersig_desktop/enum.ErrorKind.html#variant.Shortcut) when the system does not register the shortcut, as when another application holds it. A shortcut asked for again is not registered twice. In a test, `FakeOs::has_shortcut` tells whether the handlers registered a shortcut, and `FakeOs::press_shortcut` presses or releases it. `examples/meter` shows \"Talking\" while its key is held.",
                ja: "ほかのアプリがすでに使っているなどの理由で OS がショートカットを登録しなかったときは、[`ErrorKind::Shortcut`](api:hypersig_desktop/enum.ErrorKind.html#variant.Shortcut) で失敗します。同じショートカットをもう一度求めても、二重には登録しません。テストでは、`FakeOs::has_shortcut` でハンドラがショートカットを登録したかを確かめ、`FakeOs::press_shortcut` でショートカットを押したり離したりします。`examples/meter` は、キーを押している間「Talking」と表示します。",
            },
        ),
    ])
}

/// Where each system delivers a shortcut.
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
                    en: "On macOS, a press and a release of `examples/meter`'s key sent from System Events reached the page.",
                    ja: "macOS では System Events から送った `examples/meter` のキーの押下と解放がページに届くことを確かめました。",
                },
                L {
                    en: "On Linux, the shortcut is grabbed from the X server, and CI presses `examples/meter`'s key there with xdotool. A Wayland session delivers it only while an X11 window has focus, and none at all without Xwayland, where `Os::shortcut` still returns `Ok`.",
                    ja: "Linux では X サーバーからショートカットを取得します。CI はそこで xdotool を使って `examples/meter` のキーを押しています。Wayland のセッションでは、X11 のウィンドウにフォーカスがあるときだけ届きます。Xwayland がなければまったく届きませんが、それでも `Os::shortcut` は `Ok` を返します。",
                },
                L {
                    en: "On Windows, the shortcut is registered with `RegisterHotKey`, and no run has pressed it yet.",
                    ja: "Windows では `RegisterHotKey` で登録します。まだどの実行でもキーを押して確かめていません。",
                },
            ],
        ),
    ])
}
