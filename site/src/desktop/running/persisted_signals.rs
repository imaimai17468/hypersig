//! The page on the port the server keeps, and the signals it keeps with it.

use crate::Site;
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Desktop(DesktopPage::PersistedSignals).leads()),
        heading(
            site,
            L {
                en: "The port is part of the origin",
                ja: "ポートはオリジンの一部",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::persist`](api:hypersig/struct.Page.html#method.persist) keeps a signal in the browser's `localStorage`, as [Signals kept across page loads](learn/persisted-signals/) describes, and the browser keeps `localStorage` per origin: the scheme, the host, and the port. A window shows `http://127.0.0.1:<port>/`, so a server on another port would be another origin, and a page there would start from the values `Page::new` was given.",
                ja: "[`Page::persist`](api:hypersig/struct.Page.html#method.persist) は、[ページをまたいで保つシグナル](learn/persisted-signals/) で説明しているとおり、シグナルをブラウザの `localStorage` に保存します。ブラウザは `localStorage` をオリジン、つまりスキーム、ホスト、ポートの組ごとに分けて保存します。ウィンドウが表示するのは `http://127.0.0.1:<port>/` なので、サーバーが別のポートで待ち受けると別のオリジンになり、そのページは `Page::new` に渡した値から始まります。",
            },
        ),
        para(
            site,
            L {
                en: "So [`App::run`](api:hypersig_desktop/struct.App.html#method.run) records the port the system gave the server the first time, and listens on the same port each time the application starts again. In the example below, the page starts with the theme the person last chose, in every window and after a restart:",
                ja: "そのため [`App::run`](api:hypersig_desktop/struct.App.html#method.run) は、最初の起動で OS がサーバーに割り当てたポートを記録し、次からの起動でも同じポートで待ち受けます。次の例のページは、どのウィンドウでも、アプリを起動し直したあとでも、利用者が最後に選んだテーマで始まります。",
            },
        ),
        show(Snippet::DesktopPersistedSignalsTheme),
        para(
            site,
            L {
                en: "The web views of all the windows are made with one data store, so a value one window stores is the value the next page in any window starts from. On macOS, WKWebView keeps that data per application, by the bundle's identifier. On Windows and Linux it is kept under the directory [`App::data_dir`](api:hypersig_desktop/struct.App.html#method.data_dir) returns, which the identifier `App::new` takes decides, where the port is recorded too.",
                ja: "すべてのウィンドウの Web ビューは同じ保存先のデータを使うので、あるウィンドウで保存した値から、どのウィンドウの次のページも始まります。macOS の WKWebView はこのデータをアプリごとに、バンドルの識別子で分けて保存します。Windows と Linux では、`App::new` に渡した識別子で決まる [`App::data_dir`](api:hypersig_desktop/struct.App.html#method.data_dir) のディレクトリに保存し、ポートの記録も同じディレクトリに置きます。",
            },
        ),
        heading(
            site,
            L {
                en: "When another program holds the port",
                ja: "ほかのプログラムがポートを使っているとき",
            },
        ),
        para(
            site,
            L {
                en: "When the recorded port is taken, the server listens on a new port the system gives it, and records that one for the next start. The pages are then on another origin, so the signals they persisted start over from the values `Page::new` was given, and the values stored under the old port are not read again. The application logs a warning through `tracing` when this happens. Nothing else tells the application or the person.",
                ja: "記録したポートがほかのプログラムに使われていると、サーバーは OS が新しく割り当てたポートで待ち受け、次の起動のためにそのポートを記録します。ページは別のオリジンになるので、保存していたシグナルは `Page::new` に渡した値からやり直しになり、古いポートで保存した値はその後も読まれません。このときアプリは `tracing` で警告を記録します。それ以外の方法でアプリや利用者に知らせることはありません。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Persist a value the application can lose: a theme, a list's density, a panel left open. Keep a document or a setting the person would miss in a file under `App::data_dir`, which a handler reads and writes on the server.",
                    ja: "保存するのは、失われてもかまわない値にします。テーマ、リストの表示密度、開いたままのパネルなどです。文書や、失われると利用者が困る設定は、`App::data_dir` の下のファイルに置き、サーバーのハンドラで読み書きします。",
                },
                L {
                    en: "A value a page stores just before its window closes is kept: after the last window closes, the application waits a second before it exits, so the web view can write `localStorage` to disk. This was checked in WKWebView, where the value was kept in each of four runs.",
                    ja: "ウィンドウを閉じる直前にページが保存した値も残ります。最後のウィンドウが閉じたあと、アプリは 1 秒待ってから終了し、その間に Web ビューが `localStorage` をディスクに書き込みます。これは WKWebView で確かめており、4 回の実行でいずれも値が残りました。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "One copy at a time",
                ja: "同時に動かせるのは 1 つだけ",
            },
        ),
        para(
            site,
            L {
                en: "A second copy of the application started while the first runs stops before it opens a window or starts a server: `App::run` returns an [`Error`](api:hypersig_desktop/struct.Error.html) whose `kind()` is [`ErrorKind::AlreadyRunning`](api:hypersig_desktop/enum.ErrorKind.html#variant.AlreadyRunning). A `main` that returns that error prints `Error: another copy of this application is running` and exits with a failure. To exit quietly, match the kind:",
                ja: "1 つめが動いている間に起動した 2 つめのアプリは、ウィンドウを開く前、サーバーを起動する前に止まります。`App::run` は、`kind()` が [`ErrorKind::AlreadyRunning`](api:hypersig_desktop/enum.ErrorKind.html#variant.AlreadyRunning) の [`Error`](api:hypersig_desktop/struct.Error.html) を返します。このエラーを返す `main` は `Error: another copy of this application is running` と表示し、失敗として終了します。何も表示せずに終了させるには、エラーの種類で分岐します。",
            },
        ),
        show(Snippet::DesktopPersistedSignalsAlreadyRunning),
        para(
            site,
            L {
                en: "The check is per identifier, so applications with different identifiers run side by side. The first copy holds the check until its process ends, and the next start after that runs. The second copy does not bring the first one's window to the front and does not pass it its command-line arguments; a file named on the second copy's command line is not opened.",
                ja: "この確認は識別子ごとに行うので、識別子の違うアプリは同時に動かせます。1 つめのプロセスが終了すれば、そのあとの起動は通常どおり動きます。2 つめは 1 つめのウィンドウを前面に出さず、自分のコマンドライン引数も 1 つめに渡しません。2 つめのコマンドラインで指定したファイルは開かれません。",
            },
        ),
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        para(
            site,
            L {
                en: "[Who the server answers](desktop/security/) covers the checks on each request to that port. [Limits](desktop/limits/) covers the connections the windows share.",
                ja: "そのポートへのリクエストごとの確認は [サーバーが応答する相手](desktop/security/) で説明しています。ウィンドウが共有する接続については [制約](desktop/limits/) にあります。",
            },
        ),
    ])
}
