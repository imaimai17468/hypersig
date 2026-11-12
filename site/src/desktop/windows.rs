//! The page on opening, closing, and focusing windows, from a handler and from a page.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, note, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::Windows).leads()),
        from_handler(site),
        window(site),
        arrange(site),
        at_start(site),
        from_page(site),
        title(site),
        one_server(site),
        closing(site),
    ])
}

fn from_handler(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Open a window from a handler",
                ja: "ハンドラからウィンドウを開く",
            },
        ),
        code::block(Lang::Rust, None, OPEN_SIGNATURES),
        para(
            site,
            L {
                en: "A handler reaches the windows through the [`Os`](api:hypersig_desktop/struct.Os.html) that `App::run` gave the setup, which the setup puts in the router's state. [`Os::windows`](api:hypersig_desktop/struct.Os.html#method.windows) returns [`Windows`](api:hypersig_desktop/struct.Windows.html), and [`Windows::open`](api:hypersig_desktop/struct.Windows.html#method.open) opens a page in a new window. The page is a [`Url`](api:hypersig/struct.Url.html), which only `Route::url` produces, so a window opens only a path that a route declares, with arguments of the route's types.",
                ja: "ハンドラは、`App::run` がセットアップに渡した [`Os`](api:hypersig_desktop/struct.Os.html) を通してウィンドウを扱います。セットアップはこの `Os` をルーターの状態に入れておきます。[`Os::windows`](api:hypersig_desktop/struct.Os.html#method.windows) は [`Windows`](api:hypersig_desktop/struct.Windows.html) を返し、[`Windows::open`](api:hypersig_desktop/struct.Windows.html#method.open) はページを新しいウィンドウで開きます。ページは [`Url`](api:hypersig/struct.Url.html) で渡します。`Url` は `Route::url` からしか作れないので、ウィンドウで開けるのはルートが宣言したパスだけで、引数もルートの型に合ったものだけです。",
            },
        ),
        show(Snippet::DesktopOpen),
        para(
            site,
            L {
                en: "`open` returns once the window has been made, before its page has loaded. The window stays hidden until its page has loaded, and comes to the front with the keyboard when it is shown. It has the size the `App` sets for every window. In a test, [`FakeOs`](api:hypersig_desktop/testing/struct.FakeOs.html) records the page instead of opening a window, and `open_windows` returns the pages of the windows still open; [Testing without a window](desktop/testing/) covers it.",
                ja: "`open` はウィンドウを作った時点で、ページの読み込みが終わる前に戻ります。ウィンドウはページの読み込みが終わるまで表示されず、表示されるときには前面に出て、キーボードの入力を受け取ります。大きさは `App` がすべてのウィンドウに設定したものです。テストでは、[`FakeOs`](api:hypersig_desktop/testing/struct.FakeOs.html) がウィンドウを開く代わりにページを記録し、`open_windows` がまだ開いているウィンドウのページを返します。詳しくは [ウィンドウなしのテスト](desktop/testing/) で説明します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["`open` fails with", "When"],
                ja: &["`open` が返すエラー", "返すとき"],
            },
            L {
                en: &[
                    &[
                        "`ErrorKind::Window`",
                        "The window or its web view could not be made, or the first window's page did not load",
                    ],
                    &["`ErrorKind::Closed`", "The application is closing"],
                ],
                ja: &[
                    &[
                        "`ErrorKind::Window`",
                        "ウィンドウか Web ビューを作れなかった、または最初のウィンドウのページを読み込めなかった",
                    ],
                    &["`ErrorKind::Closed`", "アプリが終了しようとしている"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The `Url` is checked against the routes, not against the data behind them. `NOTE.url(7)` compiles when there is no note 7, and the window then shows what the handler answers, a 404 for a handler that returns `None`.",
                ja: "`Url` はルートとは照合されますが、その先のデータとは照合されません。7 番のメモがなくても `NOTE.url(7)` はコンパイルでき、そのときウィンドウにはハンドラの応答が表示されます。`None` を返すハンドラなら 404 です。",
            },
        ),
    ])
}

const OPEN_SIGNATURES: &str = "impl Os {
    pub fn windows(&self) -> Windows<'_>
}

impl Windows<'_> {
    pub async fn open(&self, page: &Url) -> Result<Window, Error>
}";

fn window(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Close or focus a window",
                ja: "ウィンドウを閉じる、前面に出す",
            },
        ),
        code::block(Lang::Rust, None, WINDOW_SIGNATURES),
        para(
            site,
            L {
                en: "The [`Window`](api:hypersig_desktop/struct.Window.html) that `open` returns closes the window with [`close`](api:hypersig_desktop/struct.Window.html#method.close), as its close button does, and brings it to the front with the keyboard with [`focus`](api:hypersig_desktop/struct.Window.html#method.focus). `Window` is `Clone`, so a handler can keep it in its state, and `Eq`, so two values compare equal when they name the same window.",
                ja: "`open` が返す [`Window`](api:hypersig_desktop/struct.Window.html) を使うと、[`close`](api:hypersig_desktop/struct.Window.html#method.close) で閉じるボタンと同じようにウィンドウを閉じ、[`focus`](api:hypersig_desktop/struct.Window.html#method.focus) でウィンドウを前面に出してキーボードの入力を受け取らせることができます。`Window` は `Clone` なのでハンドラの状態に保持でき、`Eq` なので同じウィンドウを指す 2 つの値は等しいと判定されます。",
            },
        ),
        show(Snippet::DesktopClose),
        para(
            site,
            L {
                en: "Both fail only with `ErrorKind::Closed`, while the application is closing. A `Window` does not say whether its window is still open: after the person has closed it, `close` and `focus` return `Ok(())` and do nothing.",
                ja: "どちらも失敗するのはアプリの終了中だけで、そのとき `ErrorKind::Closed` を返します。`Window` からは、そのウィンドウがまだ開いているかどうかはわかりません。利用者がウィンドウを閉じたあとに `close` や `focus` を呼ぶと、何もせずに `Ok(())` を返します。",
            },
        ),
    ])
}

const WINDOW_SIGNATURES: &str = "impl Window {
    pub fn close(&self) -> Result<(), Error>
    pub fn focus(&self) -> Result<(), Error>
}";

fn arrange(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Place, move, and size a window",
                ja: "ウィンドウの配置、位置、大きさを変える",
            },
        ),
        code::block(Lang::Rust, None, ARRANGE_SIGNATURES),
        para(
            site,
            L {
                en: "[`place`](api:hypersig_desktop/struct.Window.html#method.place) maximizes, minimizes, or makes the window fullscreen, and `Placement::Normal` returns it. [`layer`](api:hypersig_desktop/struct.Window.html#method.layer) with `Layer::AlwaysOnTop` keeps it above the windows of every other application, as a call's small window does. `move_to` puts its top-left corner at a point in logical pixels from the top left of the primary screen, and `resize` sizes its page area, within `App::min_size`. Each returns `Ok(())` on a window already closed and fails only while the application is closing, as `close` does.",
                ja: "[`place`](api:hypersig_desktop/struct.Window.html#method.place) はウィンドウを最大化、最小化、全画面にし、`Placement::Normal` で元に戻します。[`layer`](api:hypersig_desktop/struct.Window.html#method.layer) に `Layer::AlwaysOnTop` を渡すと、通話の小さなウィンドウのように、ほかのすべてのアプリのウィンドウより上に置きます。`move_to` は左上の角を主画面の左上からの論理ピクセルの位置に動かし、`resize` はページの領域の大きさを `App::min_size` の範囲で変えます。どれも、閉じたウィンドウでは `Ok(())` を返し、`close` と同じくアプリの終了中だけ失敗します。",
            },
        ),
        para(
            site,
            L {
                en: "[`Windows::focused`](api:hypersig_desktop/struct.Windows.html#method.focused) returns the window that had the keyboard last, while it is open, so the handler of a button in a page finds that page's window there, and a global shortcut finds the window the person used last. In a test, `FakeOs::window` returns the `Arranged` record of what the handlers gave a window:",
                ja: "[`Windows::focused`](api:hypersig_desktop/struct.Windows.html#method.focused) は、最後にキーボードの入力を受けていたウィンドウを、そのウィンドウが開いている間返します。そのため、ページのボタンのハンドラはそのページのウィンドウを、グローバルショートカットは利用者が最後に使ったウィンドウを、ここで見つけられます。テストでは、`FakeOs::window` が、ハンドラがウィンドウに与えた内容を `Arranged` として返します。",
            },
        ),
        show(Snippet::DesktopArrange),
        para(
            site,
            L {
                en: "On macOS, a run saw `resize`, `move_to`, each placement, and the return to `Placement::Normal` reach the window. `layer` has not been looked at on any system yet. Wayland lets no application place its windows, so on Linux under Wayland `move_to` leaves the window where the compositor put it.",
                ja: "macOS では、`resize`、`move_to`、それぞれの配置、`Placement::Normal` への戻りがウィンドウに届くことを実行して確かめました。`layer` はまだどの OS でも見ていません。Wayland ではアプリがウィンドウの位置を決められないので、Linux の Wayland では `move_to` を呼んでもウィンドウはコンポジターが置いた場所のままです。",
            },
        ),
    ])
}

const ARRANGE_SIGNATURES: &str = "impl Window {
    pub fn place(&self, placement: Placement) -> Result<(), Error>
    pub fn layer(&self, layer: Layer) -> Result<(), Error>
    pub fn move_to(&self, x: i32, y: i32) -> Result<(), Error>
    pub fn resize(&self, width: u32, height: u32) -> Result<(), Error>
}

impl Windows<'_> {
    pub async fn focused(&self) -> Result<Option<Window>, Error>
}";

fn at_start(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Open windows at start",
                ja: "起動時にウィンドウを開く",
            },
        ),
        para(
            site,
            L {
                en: "The setup's future can await `open` before it returns the router. A window asked for before the first window's page has loaded is made at once, and loads its page once the first page has, because the later windows need what the first one receives. `examples/notes` opens the files named on its command line this way:",
                ja: "セットアップの future は、ルーターを返す前に `open` を await できます。最初のウィンドウのページが読み込まれる前に要求したウィンドウはすぐに作られ、最初のページが読み込まれたあとで自分のページを読み込みます。あとのウィンドウには、最初のウィンドウが受け取るものが必要だからです。`examples/notes` はコマンドラインで渡されたファイルをこの方法で開きます。",
            },
        ),
        show(Snippet::DesktopAtStart),
    ])
}

fn from_page(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Open a window from a page",
                ja: "ページからウィンドウを開く",
            },
        ),
        para(
            site,
            L {
                en: "A page opens a window too, with a link whose `target` is `_blank` or with `window.open` in its own script. hypersig has no typed `target` method, so the link sets the attribute with `attr::TARGET`:",
                ja: "ページからもウィンドウを開けます。`target` が `_blank` のリンクを使うか、ページ自身のスクリプトで `window.open` を呼びます。hypersig には `target` を型付きで設定するメソッドがないので、リンクには `attr::TARGET` で属性を設定します。",
            },
        ),
        show(Snippet::DesktopBlank),
        para(
            site,
            L {
                en: "What happens depends on the URL the page asks for:",
                ja: "どうなるかは、ページが要求した URL によって決まります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["URL", "Opens in"],
                ja: &["URL", "開く場所"],
            },
            L {
                en: &[
                    &[
                        "A page of the application, such as `/notes/7`",
                        "A new window of the application, as `Windows::open` opens one",
                    ],
                    &[
                        "An `http`, `https`, or `mailto` URL of anything else",
                        "The person's browser or mail program; see [Links to other sites](desktop/external-links/)",
                    ],
                    &["Any other URL", "Nothing"],
                ],
                ja: &[
                    &[
                        "`/notes/7` のようなアプリのページ",
                        "`Windows::open` と同じように、アプリの新しいウィンドウ",
                    ],
                    &[
                        "それ以外を指す `http`、`https`、`mailto` の URL",
                        "利用者のブラウザかメールのプログラム。[ほかのサイトへのリンク](desktop/external-links/) を参照",
                    ],
                    &["その他の URL", "どこにも開かない"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Nothing returns a `Window` for a window a page opened, so no handler can close or focus it. The `href` comes from a `Url` as in the sample, which keeps the path to a declared route; a URL that the page's own script builds is not checked.",
                ja: "ページが開いたウィンドウについては `Window` が返されないので、ハンドラからそのウィンドウを閉じたり前面に出したりはできません。サンプルのように `href` を `Url` から作れば、パスは宣言済みのルートに限られます。ページ自身のスクリプトが組み立てた URL は検査されません。",
            },
        ),
        note(
            site,
            L {
                en: "WebKitGTK ignores a `window.open` that no click started unless the web view allows it, and `hypersig_desktop` allows it, so a script opens a window on Linux as it does in WKWebView and WebView2.",
                ja: "WebKitGTK はクリックをきっかけとしない `window.open` を、Web ビューで許可しない限り無視します。`hypersig_desktop` はこれを許可しているので、Linux でも WKWebView や WebView2 と同じようにスクリプトからウィンドウを開けます。",
            },
        ),
    ])
}

fn title(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The window's title",
                ja: "ウィンドウのタイトル",
            },
        ),
        para(
            site,
            L {
                en: "A window's title is its page's `<title>`, the first argument of `Page::new`. The sample's note window is titled `Note 7`. The title is empty until the page has it, and follows the page whenever `document.title` changes, as on a navigation to another page. Nothing else sets it.",
                ja: "ウィンドウのタイトルはそのページの `<title>`、つまり `Page::new` の最初の引数です。サンプルのメモのウィンドウなら `Note 7` になります。ページがタイトルを持つまでは空で、別のページへの移動などで `document.title` が変わるたびにページに合わせて変わります。それ以外の方法でタイトルを設定することはできません。",
            },
        ),
    ])
}

fn one_server(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Every window shows the same server",
                ja: "どのウィンドウも同じサーバーを表示する",
            },
        ),
        para(
            site,
            L {
                en: "All the windows show pages of the one server `App::run` started, and share their web view's cookies and `localStorage`, as two tabs of one browser do. A handler's state is the same value for every window, so two windows that open a [`StreamAction`](api:hypersig/struct.StreamAction.html) fed by one `tokio::sync::broadcast` channel show the same state: a change one window sends reaches the other over its stream.",
                ja: "どのウィンドウも `App::run` が起動した 1 つのサーバーのページを表示し、Web ビューの cookie と `localStorage` を共有します。1 つのブラウザの 2 つのタブと同じです。ハンドラの状態はどのウィンドウに対しても同じ値なので、1 つの `tokio::sync::broadcast` チャネルから流れる [`StreamAction`](api:hypersig/struct.StreamAction.html) を 2 つのウィンドウで開けば、両方が同じ状態を表示します。一方のウィンドウが送った変更は、ストリームを通してもう一方に届きます。",
            },
        ),
        para(
            site,
            L {
                en: "[`examples/notes`](https://github.com/imaimai17468/hypersig/tree/main/examples/notes) works this way: a note saved in its own window updates the save count in the list window. [`examples/todo`](https://github.com/imaimai17468/hypersig/tree/main/examples/todo) does with `--features desktop`, and its CI run opens a second window and checks that a todo added there reaches the first. [Streams](learn/streams/) covers `StreamAction`.",
                ja: "[`examples/notes`](https://github.com/imaimai17468/hypersig/tree/main/examples/notes) はこの方法で動いています。メモを専用のウィンドウで保存すると、一覧のウィンドウの保存回数が更新されます。[`examples/todo`](https://github.com/imaimai17468/hypersig/tree/main/examples/todo) も `--features desktop` を付けると同じように動きます。CI では 2 つめのウィンドウを開き、そこで追加した todo が最初のウィンドウに届くことを確かめています。`StreamAction` については [ストリーム](learn/streams/) で説明します。",
            },
        ),
        note(
            site,
            L {
                en: "The windows share six HTTP/1.1 connections to the server, and each open stream holds one. Keep at most five streams open across all windows; [Limits](desktop/limits/) covers what happens beyond that.",
                ja: "ウィンドウはサーバーへの HTTP/1.1 の接続 6 本を共有し、開いているストリームはそれぞれ 1 本を使い続けます。すべてのウィンドウを合わせて、開いておくストリームは 5 本までにします。それを超えたときに起きることは [制約](desktop/limits/) で説明します。",
            },
        ),
    ])
}

fn closing(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Closing the last window",
                ja: "最後のウィンドウを閉じる",
            },
        ),
        para(
            site,
            L {
                en: "Closing a window, with its close button or with `Window::close`, ends its page and its streams. Closing the last one closes the application, on macOS as well, where an application often stays open without windows, unless a [tray icon](desktop/tray/) is shown. The server is then given five seconds to finish the requests it is answering, and `App::run` returns; [The application](desktop/app/) covers what it returns.",
                ja: "閉じるボタンや `Window::close` でウィンドウを閉じると、そのページとストリームも終わります。最後のウィンドウを閉じるとアプリが終了します。ウィンドウがなくてもアプリが動き続けることの多い macOS でも同じです。ただし、[トレイのアイコン](desktop/tray/) を表示している間は終了しません。そのあとサーバーには応答中の要求を終えるための 5 秒の猶予が与えられ、`App::run` が戻ります。`run` が返す値は [アプリの設定](desktop/app/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "Next, [File dialogs](desktop/dialogs/) covers how a handler asks the person for a file to open or a place to save one.",
                ja: "次の [ファイルのダイアログ](desktop/dialogs/) では、ハンドラが `Os` を通して開くファイルや保存先を選んでもらう方法を説明します。",
            },
        ),
    ])
}
