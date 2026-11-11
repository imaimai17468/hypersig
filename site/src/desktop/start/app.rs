//! The page on `App`: the identifier, the window's size, the data directory, and the run's lifetime.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::TheApp).leads()),
        identifier(site),
        size(site),
        data_dir(site),
        run(site),
        lifetime(site),
        errors(site),
        menu(site),
        next(site),
    ])
}

fn identifier(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The identifier",
                ja: "識別子",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub const fn new(id: &'static str) -> App",
        ),
        para(
            site,
            L {
                en: "[`App::new`](api:hypersig_desktop/struct.App.html#method.new) takes the application's identifier, a reverse domain name such as `dev.example.notes`: two or more parts joined by dots, each part one or more ASCII letters, digits, and hyphens. `com.my-company.app2` is one; `notes`, `dev..notes`, `dev/notes`, and `dev.example.my_notes` are not. Every file system accepts such a name as a directory name, and the identifier names the directory the application keeps its files in.",
                ja: "[`App::new`](api:hypersig_desktop/struct.App.html#method.new) はアプリの識別子を受け取ります。識別子は `dev.example.notes` のような逆ドメイン形式の名前で、2 つ以上の部分をドットでつないだものです。それぞれの部分は 1 文字以上の ASCII の英字、数字、ハイフンからなります。`com.my-company.app2` は識別子として使えますが、`notes`、`dev..notes`、`dev/notes`、`dev.example.my_notes` は使えません。この形式の名前はどのファイルシステムでもディレクトリ名として使えます。アプリがファイルを置くディレクトリの名前は、この識別子で決まります。",
            },
        ),
        para(
            site,
            L {
                en: "`App::new` is a `const fn` that panics on any other text, and a panic while a `const` is evaluated is a compile error (E0080):",
                ja: "`App::new` は `const fn` で、それ以外の文字列を受け取るとパニックします。`const` の評価中のパニックはコンパイルエラー（E0080）になります。",
            },
        ),
        show(Snippet::DesktopIdConst),
        para(
            site,
            L {
                en: "Outside a `const`, the same call compiles and panics when it runs, so declare the application as a `const`:",
                ja: "`const` の外では同じ呼び出しがコンパイルを通り、実行したときにパニックします。そのため、アプリは `const` として宣言します。",
            },
        ),
        show(Snippet::DesktopIdPanic),
        para(
            site,
            L {
                en: "The check covers only the form of the name. Two applications with the same identifier share one directory, so the second to start hands its command line to the first while the first runs. [Packaging](desktop/packaging/) uses the same identifier for the package.",
                ja: "検査するのは名前の形式だけです。同じ識別子のアプリが 2 つあると 1 つのディレクトリを共有するので、一方が動いている間に起動したもう一方は、コマンドラインを先に動いている方に渡して終了します。[パッケージの作成](desktop/packaging/) でも、パッケージに同じ識別子を使います。",
            },
        ),
    ])
}

fn size(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The window's size",
                ja: "ウィンドウの大きさ",
            },
        ),
        para(
            site,
            L {
                en: "Three `const fn` methods take the `App` and return it changed, so they chain inside the `const`. Each applies to every window the application opens, the first and those opened later.",
                ja: "次の 3 つのメソッドは `const fn` で、`App` を受け取り、変更した `App` を返します。そのため `const` の中でつなげて呼べます。どの設定も最初のウィンドウとあとから開くウィンドウのすべてに適用されます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "Sets", "Default"],
                ja: &["メソッド", "設定するもの", "既定値"],
            },
            L {
                en: &[
                    &[
                        "[`size(width, height)`](api:hypersig_desktop/struct.App.html#method.size)",
                        "The size the window opens at, in logical pixels",
                        "1024 by 768",
                    ],
                    &[
                        "[`min_size(width, height)`](api:hypersig_desktop/struct.App.html#method.min_size)",
                        "The smallest size the person can resize the window to, in logical pixels",
                        "None",
                    ],
                    &[
                        "[`resizable(bool)`](api:hypersig_desktop/struct.App.html#method.resizable)",
                        "Whether the person can resize the window",
                        "`true`",
                    ],
                ],
                ja: &[
                    &[
                        "[`size(width, height)`](api:hypersig_desktop/struct.App.html#method.size)",
                        "ウィンドウを開くときの大きさ（論理ピクセル）",
                        "1024×768",
                    ],
                    &[
                        "[`min_size(width, height)`](api:hypersig_desktop/struct.App.html#method.min_size)",
                        "利用者がウィンドウを縮められる最小の大きさ（論理ピクセル）",
                        "なし",
                    ],
                    &[
                        "[`resizable(bool)`](api:hypersig_desktop/struct.App.html#method.resizable)",
                        "利用者がウィンドウの大きさを変えられるかどうか",
                        "`true`",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The arguments are `u32`. A `size` narrower or shorter than `min_size` panics, which in a `const` such as `APP` is a compile error.",
                ja: "引数の型は `u32` です。`size` の幅か高さが `min_size` より小さいとパニックします。`APP` のような `const` の中では、コンパイルエラーになります。",
            },
        ),
    ])
}

fn data_dir(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The data directory",
                ja: "データのディレクトリ",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn data_dir(&self) -> Result<PathBuf, hypersig_desktop::Error>",
        ),
        para(
            site,
            L {
                en: "[`App::data_dir`](api:hypersig_desktop/struct.App.html#method.data_dir) returns the directory where the application keeps its files, a directory named by the identifier under the system's place for application data:",
                ja: "[`App::data_dir`](api:hypersig_desktop/struct.App.html#method.data_dir) は、アプリがファイルを置くディレクトリを返します。OS がアプリのデータ用に決めている場所の下にある、識別子と同じ名前のディレクトリです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["System", "Directory", "Found from"],
                ja: &["OS", "ディレクトリ", "参照する環境変数"],
            },
            L {
                en: &[
                    &["macOS", "`~/Library/Application Support/<id>`", "`HOME`"],
                    &["Windows", "`%LOCALAPPDATA%\\<id>`", "`LOCALAPPDATA`"],
                    &[
                        "Linux and others",
                        "`$XDG_DATA_HOME/<id>`, or `~/.local/share/<id>` without it",
                        "`XDG_DATA_HOME`, then `HOME`",
                    ],
                ],
                ja: &[
                    &["macOS", "`~/Library/Application Support/<id>`", "`HOME`"],
                    &["Windows", "`%LOCALAPPDATA%\\<id>`", "`LOCALAPPDATA`"],
                    &[
                        "Linux など",
                        "`$XDG_DATA_HOME/<id>`。設定がなければ `~/.local/share/<id>`",
                        "`XDG_DATA_HOME`、なければ `HOME`",
                    ],
                ],
            },
        ),
        show(Snippet::DesktopDataDir),
        para(
            site,
            L {
                en: "`data_dir` only computes the path. It fails with `ErrorKind::NoDataDir` when the variable is unset or not an absolute path. `App::run` creates the directory before it polls the future that builds the router, so that future can open a file inside it. `run` keeps the port and the lock there too, and on Windows and Linux the web view's data; WKWebView on macOS keeps its data per application under `~/Library/WebKit`.",
                ja: "`data_dir` はパスを求めるだけで、ディレクトリは作りません。環境変数が設定されていないときや、絶対パスでないときは `ErrorKind::NoDataDir` で失敗します。`App::run` は、ルーターを作る future を実行する前にこのディレクトリを作ります。そのため、その future の中でディレクトリ内のファイルを開けます。`run` はポートとロックもここに置き、Windows と Linux では Web ビューのデータも置きます。macOS の WKWebView は、データをアプリごとに `~/Library/WebKit` の下に置きます。",
            },
        ),
    ])
}

fn run(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Running the application",
                ja: "アプリを動かす",
            },
        ),
        code::block(Lang::Rust, None, RUN_SIGNATURE),
        para(
            site,
            L {
                en: "[`App::run`](api:hypersig_desktop/struct.App.html#method.run) takes the `App` by value, the first page as a [`Url`](api:hypersig/struct.Url.html) from `Route::url`, and the setup closure. It does these in order:",
                ja: "[`App::run`](api:hypersig_desktop/struct.App.html#method.run) は、`App` を値で受け取り、最初のページを `Route::url` が返す [`Url`](api:hypersig/struct.Url.html) で受け取り、さらにセットアップのクロージャを受け取ります。`run` は次の順に動きます。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Creates the data directory, and returns `ErrorKind::NoDataDir` or `ErrorKind::Io` if it cannot.",
                    ja: "データのディレクトリを作ります。作れなければ `ErrorKind::NoDataDir` か `ErrorKind::Io` を返します。",
                },
                L {
                    en: "If another copy of the application is running, hands it this start's command line and returns `Ok(())`, or returns `ErrorKind::AlreadyRunning` when that copy does not take it.",
                    ja: "アプリの別のコピーが動いていれば、この起動のコマンドラインをそのコピーに渡して `Ok(())` を返します。そのコピーが受け取らなければ `ErrorKind::AlreadyRunning` を返します。",
                },
                L {
                    en: "Starts a multi-threaded Tokio runtime, and the server's listener on `127.0.0.1`.",
                    ja: "マルチスレッドの Tokio ランタイムを起動し、`127.0.0.1` でサーバーの待ち受けを始めます。",
                },
                L {
                    en: "Calls the closure on the calling thread with an [`Os`](api:hypersig_desktop/struct.Os.html), and runs the future it returns on the runtime.",
                    ja: "呼び出したスレッドで [`Os`](api:hypersig_desktop/struct.Os.html) を渡してクロージャを呼び、クロージャが返した future をランタイムの上で動かします。",
                },
                L {
                    en: "Makes the first window, hidden, and requests the first page. The server answers it once the future has returned the router, and the window is shown once the page has loaded.",
                    ja: "最初のウィンドウを非表示のまま作り、最初のページを要求します。future がルーターを返すとサーバーがその要求に応答し、ページの読み込みが終わるとウィンドウが表示されます。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The window is made while the future runs, so a setup that opens a database or reads a file does not hold back the window. The future is `Send + 'static`: it owns what it uses, which is why the samples write `async move` when they move the `Os` or a path into it.",
                ja: "ウィンドウは future が動いている間に作られます。そのため、データベースを開いたりファイルを読んだりするセットアップがあっても、ウィンドウの準備は待たされません。future には `Send + 'static` が必要で、使う値を所有していなければなりません。サンプルが `Os` やパスを future に移すときに `async move` と書くのはそのためです。",
            },
        ),
        para(
            site,
            L {
                en: "The future's output implements [`IntoRouter`](api:hypersig_desktop/trait.IntoRouter.html), which two types do: `Router`, and `Result<Router, E>` for an `E` that converts into `Box<dyn Error + Send + Sync>`. An `Err` stops the run: `run` closes the window and returns `ErrorKind::Setup`, with the setup's error as its `source()`.",
                ja: "future の出力は [`IntoRouter`](api:hypersig_desktop/trait.IntoRouter.html) を実装する型です。実装しているのは `Router` と、`E` が `Box<dyn Error + Send + Sync>` に変換できるときの `Result<Router, E>` の 2 つです。`Err` を返すと実行は止まります。`run` はウィンドウを閉じて `ErrorKind::Setup` を返し、その `source()` はセットアップが返したエラーです。",
            },
        ),
        show(Snippet::DesktopSetup),
        para(
            site,
            L {
                en: "`Router` here is `Router<()>`, a router whose state is already given. A `Router<AppState>` that has not had `with_state` called on it does not implement `IntoRouter`, and fails to compile (E0277):",
                ja: "ここでの `Router` は `Router<()>`、つまり状態がすでに渡されたルーターです。`with_state` を呼んでいない `Router<AppState>` は `IntoRouter` を実装していないので、コンパイルエラー（E0277）になります。",
            },
        ),
        show(Snippet::DesktopNoState),
        para(
            site,
            L {
                en: "Nothing checks that the router the future returns serves the first page. A `Url` from a route that is not merged into this router compiles, and the first window then shows the router's answer to that path, a 404 when nothing serves it.",
                ja: "future が返すルーターが最初のページを配信するかどうかは検査しません。このルーターに組み込んでいないルートの `Url` を渡してもコンパイルは通り、最初のウィンドウにはそのパスへのルーターの応答が表示されます。そのパスを配信するものがなければ 404 です。",
            },
        ),
    ])
}

const RUN_SIGNATURE: &str =
    "pub fn run<S, F>(self, first_page: &Url, setup: S) -> Result<(), Error>
where
    S: FnOnce(Os) -> F,
    F: Future<Output: IntoRouter> + Send + 'static,";

fn lifetime(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "When the application closes",
                ja: "アプリが終了するとき",
            },
        ),
        para(
            site,
            L {
                en: "`run` returns after one of three things happens: the last window closes while no tray icon is shown, the person chooses Quit in the macOS menu or in the tray's menu, or Ctrl-C is pressed in the terminal that started the application. The last window's closing ends the application on macOS as well as on Windows and Linux. The server then stops taking requests and is given five seconds to finish the ones it is answering, and `run` returns once it has stopped.",
                ja: "`run` が戻るのは、トレイのアイコンがない状態で最後のウィンドウが閉じたとき、macOS のメニューかトレイのメニューで終了が選ばれたとき、アプリを起動したターミナルで Ctrl-C が押されたときのいずれかです。最後のウィンドウを閉じるとアプリが終了するのは、Windows と Linux だけでなく macOS でも同じです。そのあとサーバーは新しい要求を受け付けなくなり、応答中の要求を終えるために 5 秒の猶予が与えられます。サーバーが止まると `run` が戻ります。",
            },
        ),
        para(
            site,
            L {
                en: "It returns `Ok(())` when the application closed this way, and the first error otherwise, such as the setup's `Err` or a server that stopped on an I/O error. Code after `run` in `main` runs after the windows are gone and the server has stopped.",
                ja: "このように終了したときは `Ok(())` を返し、それ以外のときは最初に起きたエラーを返します。たとえばセットアップの `Err` や、I/O エラーで止まったサーバーのエラーです。`main` の中で `run` のあとに書いたコードは、ウィンドウがすべて閉じ、サーバーが止まってから実行されます。",
            },
        ),
    ])
}

fn errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Errors",
                ja: "エラー",
            },
        ),
        para(
            site,
            L {
                en: "[`Error`](api:hypersig_desktop/struct.Error.html) has a [`kind()`](api:hypersig_desktop/struct.Error.html#method.kind) and, for most kinds, a `source()` with the cause. Its `Debug` writes the same message as its `Display`, so a `main` that returns `Result<(), hypersig_desktop::Error>` prints, for example, `Error: another copy of this application is running`. `run` returns these kinds of [`ErrorKind`](api:hypersig_desktop/enum.ErrorKind.html):",
                ja: "[`Error`](api:hypersig_desktop/struct.Error.html) は [`kind()`](api:hypersig_desktop/struct.Error.html#method.kind) を持ち、ほとんどの種類では原因を `source()` で返します。`Debug` は `Display` と同じメッセージを書くので、`Result<(), hypersig_desktop::Error>` を返す `main` は、たとえば `Error: another copy of this application is running` と表示します。`run` が返す [`ErrorKind`](api:hypersig_desktop/enum.ErrorKind.html) は次のとおりです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Kind", "When"],
                ja: &["種類", "返すとき"],
            },
            L {
                en: &[
                    &[
                        "`NoDataDir`",
                        "`HOME`, `LOCALAPPDATA`, or `XDG_DATA_HOME`, whichever the system uses, is unset or not an absolute path",
                    ],
                    &[
                        "`AlreadyRunning`",
                        "Another copy of the application is running and did not take this start's command line; this one opened no window",
                    ],
                    &["`Setup`", "The setup's future returned an `Err`"],
                    &[
                        "`Window`",
                        "A window or its web view could not be made or could not load",
                    ],
                    &["`Io`", "A file, the port, or the server failed"],
                    &[
                        "`Smoke`",
                        "A check of a [smoke run](desktop/smoke-runs/) failed",
                    ],
                ],
                ja: &[
                    &[
                        "`NoDataDir`",
                        "OS が使う `HOME`、`LOCALAPPDATA`、`XDG_DATA_HOME` のどれかが未設定か、絶対パスでない",
                    ],
                    &[
                        "`AlreadyRunning`",
                        "アプリの別のコピーが動いていて、この起動のコマンドラインを受け取らなかった。こちらはウィンドウを開いていない",
                    ],
                    &["`Setup`", "セットアップの future が `Err` を返した"],
                    &[
                        "`Window`",
                        "ウィンドウか Web ビューを作れなかった、または読み込めなかった",
                    ],
                    &["`Io`", "ファイル、ポート、サーバーのどれかが失敗した"],
                    &[
                        "`Smoke`",
                        "[スモークテスト](desktop/smoke-runs/) の検査が失敗した",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The other kinds come from the methods of `Os`: `Closed` from `Windows::open`, `Window::close`, or `Window::focus` called while the application is closing, `Clipboard` from [the clipboard](desktop/clipboard/), `NotExternal` from [`Os::open_external`](desktop/external-links/), and `Update` from [updates](desktop/updates/). `ErrorKind` is `#[non_exhaustive]`, so a `match` on it needs a `_` arm. A second copy started while the first runs can end quietly this way:",
                ja: "それ以外の種類は `Os` のメソッドが返します。アプリの終了中に `Windows::open`、`Window::close`、`Window::focus` を呼ぶと `Closed`、[クリップボード](desktop/clipboard/) は `Clipboard`、[`Os::open_external`](desktop/external-links/) は `NotExternal`、[更新](desktop/updates/) は `Update` を返します。`ErrorKind` は `#[non_exhaustive]` なので、`match` には `_` の分岐が必要です。最初のコピーが動いている間に起動した 2 つめのコピーは、次のようにすればエラーを表示せずに終了できます。",
            },
        ),
        show(Snippet::DesktopErrors),
        note(
            site,
            L {
                en: "The second copy does not tell the first one anything: the first copy's window does not come to the front, and the second copy's setup closure is never called, so it opens nothing it was given on its command line.",
                ja: "2 つめのコピーは最初のコピーに何も伝えません。最初のコピーのウィンドウは前面に出ません。また、2 つめのコピーではセットアップのクロージャが呼ばれないので、そのコマンドラインで渡したものは何も開かれません。",
            },
        ),
    ])
}

fn menu(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The menu bar on macOS",
                ja: "macOS のメニューバー",
            },
        ),
        para(
            site,
            L {
                en: "macOS sends the shortcuts for copying, pasting, closing, and quitting to the menu bar, not to the web view, so without menu items for them a text field would take no pasted text. On macOS, `run` puts three menus in the menu bar:",
                ja: "macOS はコピー、ペースト、ウィンドウを閉じる、終了するといった操作のショートカットを Web ビューではなくメニューバーに送ります。メニューにそれらの項目がなければ、テキストフィールドにペーストもできません。そのため macOS では、`run` がメニューバーに次の 3 つのメニューを置きます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Menu", "Items"],
                ja: &["メニュー", "項目"],
            },
            L {
                en: &[
                    &[
                        "The application's menu",
                        "Hiding the application (⌘H), hiding the others (⌥⌘H), showing all, and Quit (⌘Q)",
                    ],
                    &[
                        "Edit",
                        "Undo (⌘Z), Redo (⇧⌘Z), Cut (⌘X), Copy (⌘C), Paste (⌘V), and Select All (⌘A)",
                    ],
                    &["Window", "Minimize (⌘M) and Close Window (⌘W)"],
                ],
                ja: &[
                    &[
                        "アプリのメニュー",
                        "アプリを隠す（⌘H）、ほかを隠す（⌥⌘H）、すべてを表示、Quit（⌘Q）",
                    ],
                    &[
                        "Edit",
                        "Undo（⌘Z）、Redo（⇧⌘Z）、Cut（⌘X）、Copy（⌘C）、Paste（⌘V）、Select All（⌘A）",
                    ],
                    &["Window", "Minimize（⌘M）、Close Window（⌘W）"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Quit closes every window and stops the server as closing the last window does, with the same five seconds for the requests in progress. The menus cannot be changed or extended. Windows and Linux hand these keys to the web view, and the application has no menu bar there.",
                ja: "Quit は最後のウィンドウを閉じたときと同じように、すべてのウィンドウを閉じてサーバーを止めます。応答中の要求に与える猶予も同じ 5 秒です。メニューを変更したり項目を追加したりはできません。Windows と Linux ではこれらのキーを Web ビューが受け取るので、アプリにメニューバーはありません。",
            },
        ),
    ])
}

fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "Next, [Opening windows](desktop/windows/) covers the windows after the first: from a handler through `Os`, and from a page.",
            ja: "次の [ウィンドウを開く](desktop/windows/) では 2 つめ以降のウィンドウを、ハンドラから `Os` を通して開く方法と、ページから開く方法を説明します。",
        },
    )
}
