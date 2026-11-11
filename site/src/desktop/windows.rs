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
                en: "A window asked for before the first window's page has loaded opens after it has, because the later windows need what the first one receives. The first page in turn waits for the setup's future to return the router, so the future spawns a task that opens the windows instead of awaiting `open` itself, as `examples/notes` does for the files named on its command line:",
                ja: "最初のウィンドウのページが読み込まれる前に要求したウィンドウは、読み込みが終わってから開きます。あとのウィンドウには、最初のウィンドウが受け取るものが必要だからです。一方、最初のページはセットアップの future がルーターを返すのを待っています。そのため future の中では `open` を await せず、ウィンドウを開くタスクを spawn します。`examples/notes` はコマンドラインで渡されたファイルをこの方法で開きます。",
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
                en: "Closing a window, with its close button or with `Window::close`, ends its page and its streams. Closing the last one closes the application, on macOS as well, where an application often stays open without windows. The server is then given five seconds to finish the requests it is answering, and `App::run` returns; [The application](desktop/app/) covers what it returns.",
                ja: "閉じるボタンや `Window::close` でウィンドウを閉じると、そのページとストリームも終わります。最後のウィンドウを閉じるとアプリが終了します。ウィンドウがなくてもアプリが動き続けることの多い macOS でも同じです。そのあとサーバーには応答中の要求を終えるための 5 秒の猶予が与えられ、`App::run` が戻ります。`run` が返す値は [アプリの設定](desktop/app/) で説明します。",
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
