//! The page on the microphone and the camera: `App::allow`, `Device`, and
//! what each system needs before a page gets a device.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Devices).leads()),
        api(site),
        systems(site),
    ])
}

/// `App::allow` and an example.
fn api(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Allowing a device",
                ja: "デバイスを許可する",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<U> App<U> {\n    pub const fn allow(self, device: Device) -> Self;\n}\n\npub enum Device {\n    Microphone,\n    Camera,\n    Screen,\n}",
        ),
        para(
            site,
            L {
                en: "A page asks for the microphone or the camera with `navigator.mediaDevices.getUserMedia`, as a voice or video call does. The web view grants a page only the devices [`App::allow`](api:hypersig_desktop/struct.App.html#method.allow) names, without asking the person, and refuses the others with `NotAllowedError`. The application decides in Rust which devices its pages get, and no page can ask for more:",
                ja: "ページは音声通話やビデオ通話のように、`navigator.mediaDevices.getUserMedia` でマイクやカメラを求めます。Web ビューは [`App::allow`](api:hypersig_desktop/struct.App.html#method.allow) で指定したデバイスだけを利用者に確かめずにページに許可し、それ以外は `NotAllowedError` で拒否します。ページが使えるデバイスはアプリが Rust で決めるので、ページがそれ以上を求めることはできません。",
            },
        ),
        show(Snippet::DesktopDevices),
        para(
            site,
            L {
                en: "The system still asks the person once whether the application may use the device. The page of `examples/meter` loads a custom element that shows how loud the microphone hears the room, and CI checks on Windows and Linux that it is granted the microphone and refused the camera.",
                ja: "それでも OS はアプリがそのデバイスを使ってよいかを一度だけ利用者に確かめます。`examples/meter` は、ページが読み込むカスタム要素で、マイクが拾う音の大きさを表示します。CI は Windows と Linux で、この例にマイクが許可され、カメラが拒否されることを確かめています。",
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
                    en: "On macOS, WKWebView offers no `navigator.mediaDevices` unless the bundle's `Info.plist` has `NSMicrophoneUsageDescription` for the microphone and `NSCameraUsageDescription` for the camera, each a sentence the system shows when it asks the person. A run outside a bundle, such as `cargo run`, has neither. `examples/meter` adds the key from its `Info.plist` with cargo-packager's `macos.info-plist-path`, as [Packaging](desktop/packaging/) shows.",
                    ja: "macOS の WKWebView はバンドルの `Info.plist` に、マイクなら `NSMicrophoneUsageDescription`、カメラなら `NSCameraUsageDescription` がなければ、`navigator.mediaDevices` を用意しません。どちらの値も、OS が利用者に確かめるときに表示する文です。`cargo run` のようにバンドルの外で動かすときは、どちらのキーもありません。`examples/meter` は [パッケージの作成](desktop/packaging/)で示すように、cargo-packager の `macos.info-plist-path` で自分の `Info.plist` のキーを加えています。",
                },
                L {
                    en: "On macOS, a page in a bundle with these keys was seen to get `navigator.mediaDevices`. No page has yet been seen to get the microphone itself, because the system's question could not be answered where this was tried. CI's macOS runner has no device, so it does not run the meter.",
                    ja: "macOS では、バンドルのキーがあればページに `navigator.mediaDevices` が用意されることを確かめました。ただし、試した環境では OS の確認に答えられなかったため、ページが実際にマイクを使えるところはまだ見ていません。CI の macOS のランナーにはデバイスがないので、meter の例を動かしていません。",
                },
                L {
                    en: "On Windows, WebView2 keeps a grant in the web view's data, so a later version that no longer names the device is still granted it until that data is removed.",
                    ja: "Windows の WebView2 は許可を Web ビューのデータに保存します。そのため、あとのバージョンでデバイスの指定を外しても、そのデータを消すまではデバイスが許可されたままになります。",
                },
                L {
                    en: "`Device::Screen` answers `getDisplayMedia`, as sharing the screen in a call does. Only WebKitGTK, on Linux, asks the application about it, and it shares the screen only from a click of the person's, not from a script's `click()`. CI checks there that a click on the meter's button shares the mock screen. WebView2, on Windows, answers `getDisplayMedia` on its own, whether or not the application names the screen; called from a script, it neither shared nor refused within 10 seconds. WKWebView, on macOS, has no `getDisplayMedia`, so a page there cannot share the screen.",
                    ja: "`Device::Screen` は、通話で画面を共有するときの `getDisplayMedia` に答えます。アプリに確かめるのは Linux の WebKitGTK だけです。WebKitGTK が画面を共有するのは人がクリックしたときだけで、スクリプトの `click()` では共有しません。CI はそこで、meter の例のボタンをクリックすると模擬の画面を共有できることを確かめています。Windows の WebView2 は、アプリが画面を指定したかどうかにかかわらず `getDisplayMedia` に自分で答えます。スクリプトから呼んだときは、10 秒たっても共有も拒否もしませんでした。macOS の WKWebView には `getDisplayMedia` がないため、ページから画面を共有できません。",
                },
                L {
                    en: "On Linux, WebKitGTK turns its media streams on only for an application that allows a device.",
                    ja: "Linux の WebKitGTK はデバイスを許可したアプリでだけメディアストリームを有効にします。",
                },
                L {
                    en: "Hardened runtime entitlements for a signed macOS package are not set yet; [issue #226](https://github.com/imaimai17468/hypersig/issues/226) covers signing.",
                    ja: "署名した macOS のパッケージに必要な Hardened Runtime のエンタイトルメントは、まだ設定していません。署名は [issue #226](https://github.com/imaimai17468/hypersig/issues/226) で扱います。",
                },
            ],
        ),
    ])
}
