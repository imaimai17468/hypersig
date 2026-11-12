//! The page on finding, downloading, and installing new versions.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of};

/// An answer of the update endpoint, as the crate's tests serve it, with a
/// URL and a signature written out for the page.
const ANSWER: &str = r#"HTTP/1.1 200 OK
content-type: application/json

{
  "version": "1.1.0",
  "notes": "Faster saves.",
  "url": "https://updates.example.com/notes/1.1.0/hypersig-notes.app.tar.gz",
  "signature": "<the contents of hypersig-notes.app.tar.gz.sig>",
  "format": "app"
}"#;

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Desktop(DesktopPage::Updates).leads()),
        heading(
            site,
            L {
                en: "Name where new versions are announced",
                ja: "新しいバージョンを知らせる場所を指定する",
            },
        ),
        para(
            site,
            L {
                en: "Updates are behind the `updater` feature of hypersig_desktop, which adds [`Updates`](api:hypersig_desktop/struct.Updates.html), [`App::updates`](api:hypersig_desktop/struct.App.html#method.updates), and [`Updater`](api:hypersig_desktop/struct.Updater.html). `Updates::new` is `pub const fn new(version: &'static str, endpoint: &'static str, public_key: &'static str) -> Updates`:",
                ja: "更新の機能は hypersig_desktop の `updater` feature で有効になり、[`Updates`](api:hypersig_desktop/struct.Updates.html)、[`App::updates`](api:hypersig_desktop/struct.App.html#method.updates)、[`Updater`](api:hypersig_desktop/struct.Updater.html) が使えるようになります。`Updates::new` のシグネチャは `pub const fn new(version: &'static str, endpoint: &'static str, public_key: &'static str) -> Updates` です。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Argument", "What to pass"][..],
                ja: &["引数", "渡すもの"][..],
            },
            L {
                en: &[
                    &[
                        "`version`",
                        "The running version, `env!(\"CARGO_PKG_VERSION\")` in the application's crate.",
                    ][..],
                    &[
                        "`endpoint`",
                        "The URL asked for a newer version. It may hold `{{target}}`, `{{arch}}`, and `{{current_version}}`.",
                    ],
                    &[
                        "`public_key`",
                        "The public key `cargo packager signer generate` wrote, the contents of the `.pub` file.",
                    ],
                ][..],
                ja: &[
                    &[
                        "`version`",
                        "動いているバージョンです。アプリのクレートで `env!(\"CARGO_PKG_VERSION\")` を渡します。",
                    ][..],
                    &[
                        "`endpoint`",
                        "新しいバージョンを問い合わせる URL です。`{{target}}`、`{{arch}}`、`{{current_version}}` を含められます。",
                    ],
                    &[
                        "`public_key`",
                        "`cargo packager signer generate` が書き出した公開鍵、つまり `.pub` ファイルの中身です。",
                    ],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "Before each request, `{{target}}` becomes `linux`, `windows`, or `macos`, `{{arch}}` becomes `x86_64`, `i686`, `aarch64`, or `armv7`, and `{{current_version}}` becomes `version`, so `https://updates.example.com/notes/{{target}}/{{arch}}/{{current_version}}` asks `https://updates.example.com/notes/macos/aarch64/1.0.0` on an Apple silicon Mac running 1.0.0. `Updates::new` and `App::updates` are `const fn`, so the whole setting fits in the `const` that names the application. `include_str!` of the `.pub` file gives a `&'static str` too.",
                ja: "リクエストのたびに、`{{target}}` は `linux`、`windows`、`macos` のどれかに、`{{arch}}` は `x86_64`、`i686`、`aarch64`、`armv7` のどれかに、`{{current_version}}` は `version` に置き換わります。たとえば `https://updates.example.com/notes/{{target}}/{{arch}}/{{current_version}}` は、1.0.0 を動かしている Apple シリコンの Mac では `https://updates.example.com/notes/macos/aarch64/1.0.0` への問い合わせになります。`Updates::new` と `App::updates` は `const fn` なので、設定全体をアプリを表す `const` に書けます。`.pub` ファイルを `include_str!` で読み込んでも `&'static str` が得られます。",
            },
        ),
        heading(
            site,
            L {
                en: "Find, download, and install a version",
                ja: "バージョンを探し、ダウンロードしてインストールする",
            },
        ),
        para(
            site,
            L {
                en: "`App::updates` turns the `App` into an `App<Updates>`, whose `run` hands the setup an `Updater` beside the `Os`, so only an application that named its updates has one to call. The setup puts it in the router's state, and a handler asks with `updater.check()`, `async fn check(&self) -> Result<Option<Update>, Error>`. It answers `Ok(None)` when the endpoint announces nothing newer than `version`. An [`Update`](api:hypersig_desktop/struct.Update.html) gives `version()`, the announced version, and `notes()`, the release notes when the endpoint sent some. `Update::download` consumes it and returns a [`Downloaded`](api:hypersig_desktop/struct.Downloaded.html) once the package is downloaded and its signature verified, and `Downloaded::install` installs it. In the example below, one action runs all three steps when the button is clicked, and reports the result in a signal:",
                ja: "`App::updates` は `App` を `App<Updates>` に変え、その `run` は setup に `Os` と並べて `Updater` を渡します。そのため、更新の取得先を指定したアプリだけが、呼び出せる `Updater` を持ちます。setup はそれをルーターの状態に入れ、ハンドラは `updater.check()`（`async fn check(&self) -> Result<Option<Update>, Error>`）で問い合わせます。エンドポイントが `version` より新しいバージョンを知らせていなければ `Ok(None)` が返ります。[`Update`](api:hypersig_desktop/struct.Update.html) の `version()` は知らされたバージョンを、`notes()` はエンドポイントがリリースノートを送ったときにその内容を返します。`Update::download` は `Update` を消費し、パッケージをダウンロードして署名を確かめたあとで [`Downloaded`](api:hypersig_desktop/struct.Downloaded.html) を返します。`Downloaded::install` はそれをインストールします。次の例では、ボタンを押すと 1 つのアクションがこの 3 つを実行し、結果をシグナルに表示します。",
            },
        ),
        show(Snippet::DesktopUpdatesCheck),
        para(
            site,
            L {
                en: "The new version runs once the application starts again. On Windows, the installer replaces the running program and cargo-packager-updater 0.2.3 ends the process once it has started it, so `install` closes the application first, as Quit does: it returns `Ok(())` at once, the windows close, the server finishes the requests it is answering, and the NSIS installer starts as the last thing `App::run` does. When the installer cannot start, `App::run` returns the error. No run has installed a package on Windows yet. On macOS the `.app` bundle, and on Linux the AppImage, are replaced on disk, and `install` returns while the application keeps running. In cargo-packager-updater 0.2.3's source, a `.app` in a folder the person cannot write to is moved into place after macOS asks for an administrator's password.",
                ja: "新しいバージョンはアプリを次に起動したときから動きます。Windows ではインストーラーが実行中のプログラムを置き換え、cargo-packager-updater 0.2.3 はインストーラーを起動したあとでプロセスを終了します。そのため `install` は、Quit と同じようにまずアプリを閉じます。`install` はすぐに `Ok(())` を返し、ウィンドウが閉じ、サーバーが処理中のリクエストを終えてから、`App::run` の最後に NSIS のインストーラーが起動します。インストーラーを起動できなかったときは、`App::run` がそのエラーを返します。Windows でパッケージをインストールした実行は、まだありません。macOS では `.app` バンドルが、Linux では AppImage がディスク上で置き換わり、`install` が戻ったあともアプリは動き続けます。cargo-packager-updater 0.2.3 のソースでは、利用者が書き込めないフォルダにある `.app` は、macOS が管理者のパスワードを求めたあとで置き換えられます。",
            },
        ),
        note(
            site,
            L {
                en: "hypersig_desktop's tests cover finding a version, downloading it, and refusing a package whose signature does not match. On Linux, CI also runs the whole update: `examples/update` built as 0.1.0 finds a 0.2.0 AppImage that cargo-packager signed with a key made in the job, served on `127.0.0.1`, and installs it over the file `APPIMAGE` names. On macOS and Windows, an update has not been installed by any run.",
                ja: "hypersig_desktop のテストで確かめているのは、バージョンを見つけること、ダウンロードすること、署名の合わないパッケージを拒否することです。Linux では、CI が更新の流れを通しでも実行しています。0.1.0 としてビルドした `examples/update` が、ジョブの中で作った鍵で cargo-packager が署名した 0.2.0 の AppImage を `127.0.0.1` で見つけ、`APPIMAGE` が指すファイルの上にインストールします。macOS と Windows では、まだどの実行でも更新をインストールしていません。",
            },
        ),
        heading(
            site,
            L {
                en: "What the endpoint answers",
                ja: "エンドポイントが返すもの",
            },
        ),
        para(
            site,
            L {
                en: "The endpoint answers `204 No Content` when there is no newer version, which `check` returns as `Ok(None)`. Otherwise it answers `200 OK` with JSON naming the version and, for the system and architecture in the request's URL, the package's URL, the contents of its `.sig` file, and its format. This is the answer hypersig_desktop's tests serve to a version before 1.1.0, with the URL and the signature written out:",
                ja: "新しいバージョンがなければ、エンドポイントは `204 No Content` を返し、`check` はそれを `Ok(None)` として返します。新しいバージョンがあれば `200 OK` と JSON を返します。JSON にはバージョンと、リクエストの URL が示す OS とアーキテクチャ向けのパッケージの URL、その `.sig` ファイルの中身、形式を書きます。次は hypersig_desktop のテストが 1.1.0 より前のバージョンに返す応答で、URL と署名だけを具体的な値に書き換えています。",
            },
        ),
        code::block(Lang::JavaScript, None, ANSWER),
        table_of(
            site,
            L {
                en: &["System", "`format`", "`url` points at"][..],
                ja: &["OS", "`format`", "`url` が指すもの"][..],
            },
            L {
                en: &[
                    &["macOS", "`app`", "the `.app.tar.gz` that signing made"][..],
                    &["Windows", "`nsis`", "the NSIS installer `.exe`"],
                    &["Linux", "`appimage`", "the AppImage"],
                ][..],
                ja: &[
                    &["macOS", "`app`", "署名のときにできた `.app.tar.gz`"][..],
                    &["Windows", "`nsis`", "NSIS のインストーラーの `.exe`"],
                    &["Linux", "`appimage`", "AppImage"],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "The `.dmg`, the `.deb`, and the pacman package are not formats an update installs from. `version` may start with `v`, and `notes` and `pub_date`, an RFC 3339 date, may be left out. Instead of one answer per system, the endpoint may also answer every system at once under `platforms`, keyed `macos-aarch64` and so on, as the [cargo-packager-updater documentation](https://docs.rs/cargo-packager-updater/0.2.3/cargo_packager_updater/) describes. A file served as is can then be the endpoint.",
                ja: "`.dmg`、`.deb`、pacman のパッケージからは更新をインストールできません。`version` は `v` で始まってもかまいません。`notes` と、RFC 3339 の日時を書く `pub_date` は省略できます。OS ごとに応答を返す代わりに、`platforms` の下に `macos-aarch64` のようなキーですべての OS の分をまとめて返すこともできます。書き方は [cargo-packager-updater のドキュメント](https://docs.rs/cargo-packager-updater/0.2.3/cargo_packager_updater/) にあります。この形なら、そのまま配信するファイルをエンドポイントにできます。",
            },
        ),
        heading(
            site,
            L {
                en: "When an update fails",
                ja: "更新が失敗するとき",
            },
        ),
        para(
            site,
            L {
                en: "Every failure of `check`, `download`, and `install` is an [`Error`](api:hypersig_desktop/struct.Error.html) whose `kind()` is [`ErrorKind::Update`](api:hypersig_desktop/enum.ErrorKind.html#variant.Update), and its `Display` says what failed:",
                ja: "`check`、`download`、`install` の失敗は、どれも `kind()` が [`ErrorKind::Update`](api:hypersig_desktop/enum.ErrorKind.html#variant.Update) の [`Error`](api:hypersig_desktop/struct.Error.html) になり、その `Display` が何に失敗したかを示します。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`check` fails when the endpoint cannot be reached, and when it answers something other than `204` or the JSON above.",
                    ja: "`check` が失敗するのは、エンドポイントに接続できないとき、`204` と上の JSON 以外の応答が返ったときです。",
                },
                L {
                    en: "`download` fails when the package cannot be downloaded, and when the public key does not verify its signature. A refused package is never installed, since only a `Downloaded` has `install`.",
                    ja: "`download` が失敗するのは、パッケージをダウンロードできないときと、公開鍵で署名を確かめられないときです。`install` は `Downloaded` にしかないので、拒否したパッケージがインストールされることはありません。",
                },
                L {
                    en: "`install` fails when the package cannot be put in place.",
                    ja: "`install` が失敗するのは、パッケージを所定の場所に置けないときです。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "`Updates::new` checks its arguments, and in a `const` a wrong one is a compile error. `version` is a semantic version such as `1.2.3`. `endpoint` starts with `https://` and a host; plain `http` is accepted only to `127.0.0.1` or `localhost`, for a test on the same machine. `public_key` is the base64 of a minisign public key: an `untrusted comment:` line, then the key. A key of that shape that does not verify the package still fails, at `download`. A test puts `testing::FakeOs::updater` in the state, whose `check` returns `Ok(None)`, so a test of the handler sees the newest-version branch.",
                ja: "`Updates::new` は引数を検査し、`const` の中では誤った引数がコンパイルエラーになります。`version` は `1.2.3` のようなセマンティック バージョンです。`endpoint` は `https://` とホストで始まります。暗号化しない `http` は、同じマシンでのテストのために `127.0.0.1` と `localhost` に限って使えます。`public_key` は minisign の公開鍵を base64 にしたもので、`untrusted comment:` の行と鍵の行から成ります。形が正しくても、その鍵でパッケージの署名を確かめられなければ、`download` で失敗します。テストでは `testing::FakeOs::updater` を状態に入れます。その `check` は `Ok(None)` を返すので、ハンドラのテストでは最新版のときの分岐を通ります。",
            },
        ),
        heading(
            site,
            L {
                en: "Certificates on Linux",
                ja: "Linux での証明書",
            },
        ),
        para(
            site,
            L {
                en: "On Linux, the first check sets `SSL_CERT_FILE` and `SSL_CERT_DIR` with `std::env::set_var` when they are unset, while other threads of the application may read the environment. Setting both before `App::run` avoids that write.",
                ja: "Linux では、最初の問い合わせのときに `SSL_CERT_FILE` と `SSL_CERT_DIR` が設定されていなければ、`std::env::set_var` でそれらを設定します。このとき、アプリのほかのスレッドが環境変数を読んでいることがあります。`App::run` の前に 2 つとも設定しておけば、この書き込みは起きません。",
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
                en: "[Packaging](desktop/packaging/) builds and signs the packages the endpoint points at. [Testing without a window](desktop/testing/) calls handlers like `update` under `cargo test`.",
                ja: "エンドポイントが指すパッケージの作り方と署名のしかたは [パッケージの作成](desktop/packaging/) で説明しています。`update` のようなハンドラを `cargo test` で呼ぶ方法は [ウィンドウなしのテスト](desktop/testing/) にあります。",
            },
        ),
    ])
}
