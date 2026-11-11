//! The page on smoke runs of the real window in CI: the `smoke` feature, its
//! two environment variables, a script that sets a verdict, and a workflow.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{DesktopPage, Doc};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

/// The script CI's desktop job runs in the notes example, read from the
/// file so the page shows what CI runs.
const NOTES_SMOKE: &str = include_str!("../../../../examples/notes/smoke.js");

/// A build with the `smoke` feature, and a run of it.
const COMMANDS: &str = "cargo build --release -p example-notes --features hypersig_desktop/smoke\nHYPERSIG_DESKTOP_SMOKE=1 target/release/example-notes";

/// The steps of the desktop job in `.github/workflows/ci.yml` that build and
/// run the notes example, as a workflow of its own.
const WORKFLOW: &str = r##"name: desktop
on: [push, pull_request]
jobs:
  desktop:
    strategy:
      fail-fast: false
      matrix:
        os: [macos-latest, windows-latest, ubuntu-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7
      - uses: dtolnay/rust-toolchain@d124f86f28ff42855784ff3319f48ac182c248f8 # 1.99
      # xvfb gives the window a display on a runner that has none.
      - if: runner.os == 'Linux'
        run: sudo apt-get update && sudo apt-get install -y --no-install-recommends libwebkit2gtk-4.1-dev libdbus-1-dev xvfb
      - run: cargo test -p example-notes
      - run: cargo build --release -p example-notes --features hypersig_desktop/smoke
      - name: The notes example saves a file from a second window
        shell: bash
        env:
          HYPERSIG_DESKTOP_SMOKE_SCRIPT: examples/notes/smoke.js
        run: |
          note="$RUNNER_TEMP/smoke-note.md"
          echo "# Smoke" > "$note"
          if [ "$RUNNER_OS" = Linux ]; then
            xvfb-run -a target/release/example-notes "$note"
          else
            target/release/example-notes "$note"
          fi
          grep -q 'Saved by the smoke run.' "$note""##;

pub(crate) fn page(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Desktop(DesktopPage::SmokeRuns).leads()),
        feature(site),
        first_page(site),
        script(site),
        workflow(site),
        next(site),
    ])
}

/// The `smoke` feature and why a build without it ignores the variables.
fn feature(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The smoke feature",
                ja: "smoke feature",
            },
        ),
        para(
            site,
            L {
                en: "A smoke run starts the application as a person would, in the system's real web view, checks its first page, and closes it, so CI finds what `FakeOs` cannot: a window that does not open, a server that refuses it, or a page that does not load. It needs a build with the `smoke` feature of `hypersig_desktop` and one of two environment variables set when the application starts:",
                ja: "スモークテストでは、利用者が起動するのと同じように OS の本物の Web ビューでアプリを起動し、最初のページを確かめてから閉じます。これで、`FakeOs` では見つからない問題を CI で見つけられます。ウィンドウが開かない、サーバーがウィンドウを拒否する、ページが読み込まれない、といった問題です。`hypersig_desktop` の `smoke` feature を有効にしたビルドと、アプリの起動時に設定した 2 つの環境変数のどちらかが必要です。",
            },
        ),
        code::block(Lang::Shell, None, COMMANDS),
        table_of(
            site,
            L {
                en: &["Variable", "What the run does"],
                ja: &["環境変数", "実行の内容"],
            },
            L {
                en: &[
                    &[
                        "`HYPERSIG_DESKTOP_SMOKE`",
                        "Checks the first page and closes the application",
                    ],
                    &[
                        "`HYPERSIG_DESKTOP_SMOKE_SCRIPT`",
                        "Checks the first page, then runs the JavaScript file the variable names and closes once it passes",
                    ],
                ][..],
                ja: &[
                    &[
                        "`HYPERSIG_DESKTOP_SMOKE`",
                        "最初のページを確かめてアプリを閉じる",
                    ],
                    &[
                        "`HYPERSIG_DESKTOP_SMOKE_SCRIPT`",
                        "最初のページを確かめたあと、この変数が指す JavaScript のファイルを実行し、成功したら閉じる",
                    ],
                ][..],
            },
        ),
        para(
            site,
            L {
                en: "Either variable alone starts a run. `HYPERSIG_DESKTOP_SMOKE` may have any value, an empty one included, and `HYPERSIG_DESKTOP_SMOKE_SCRIPT` must name a file the application can read. A build without the feature ignores both, because the script runs in the page and can call every action the router declares. A release built without `smoke` therefore runs no file an environment variable names. CI builds with the feature as a separate build, and examples/notes ships its packages without it.",
                ja: "どちらの変数も、それだけでスモークテストが始まります。`HYPERSIG_DESKTOP_SMOKE` の値は何でもよく、空でもかまいません。`HYPERSIG_DESKTOP_SMOKE_SCRIPT` には、アプリが読めるファイルを指定します。feature を有効にしていないビルドは、どちらの変数も無視します。スクリプトはページの中で動き、ルーターが宣言したすべてのアクションを呼べるからです。そのため、`smoke` なしでビルドしたリリース版は、環境変数で指定されたファイルを実行しません。CI では feature を有効にしたビルドを別に作り、examples/notes のパッケージは feature なしでビルドしています。",
            },
        ),
    ])
}

/// What a run checks on the first page, and when it fails.
fn first_page(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The check of the first page",
                ja: "最初のページの確認",
            },
        ),
        para(
            site,
            L {
                en: "The first page is the one `App::run` names. When its window is shown, the run writes one line to standard output, where `N` is the milliseconds since `App::run` was called:",
                ja: "最初のページは `App::run` に渡したページです。そのウィンドウが表示されると、スモークテストは標準出力に 1 行を書きます。`N` は `App::run` を呼んでからのミリ秒数です。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "hypersig_desktop: first page shown N ms after run",
        ),
        para(
            site,
            L {
                en: "The run then checks that the page is one hypersig rendered. With `HYPERSIG_DESKTOP_SMOKE` alone, it closes the application once the check passes, and `App::run` returns `Ok(())`. The line is written before the check, so a run whose check fails writes it too. A run fails in these cases, and `App::run` returns an `Err`, so a `main` that returns it exits with a status other than 0:",
                ja: "続いて、そのページが hypersig の描画したページであることを確かめます。`HYPERSIG_DESKTOP_SMOKE` だけを設定した場合は、確認が通るとアプリを閉じ、`App::run` は `Ok(())` を返します。この行は確認の前に書かれるので、確認に失敗したときにも出力されます。次の場合にスモークテストは失敗し、`App::run` が `Err` を返します。そのエラーを返す `main` は 0 以外の終了コードで終わります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Cause", "`ErrorKind`"],
                ja: &["原因", "`ErrorKind`"],
            },
            L {
                en: &[
                    &[
                        "The first page is not one hypersig rendered, as when the server refused the window",
                        "`Smoke`",
                    ],
                    &[
                        "A window's page did not finish loading within 10 seconds",
                        "`Smoke`",
                    ],
                    &[
                        "The file `HYPERSIG_DESKTOP_SMOKE_SCRIPT` names cannot be read",
                        "`Io`",
                    ],
                ][..],
                ja: &[
                    &[
                        "最初のページが hypersig の描画したページでない（サーバーがウィンドウを拒否したときなど）",
                        "`Smoke`",
                    ],
                    &[
                        "ウィンドウのページが 10 秒以内に読み込みを終えなかった",
                        "`Smoke`",
                    ],
                    &[
                        "`HYPERSIG_DESKTOP_SMOKE_SCRIPT` が指すファイルを読めない",
                        "`Io`",
                    ],
                ][..],
            },
        ),
    ])
}

/// The script, its verdict, and the notes example's script.
fn script(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A script that sets a verdict",
                ja: "結果を設定するスクリプト",
            },
        ),
        para(
            site,
            L {
                en: "With `HYPERSIG_DESKTOP_SMOKE_SCRIPT`, the run evaluates the file in the first page once the check passes, and in the page of each window that loads after that. It then reads `window.hypersigSmoke` in the first page every 200 milliseconds, as a string:",
                ja: "`HYPERSIG_DESKTOP_SMOKE_SCRIPT` を設定すると、確認が通ったあとにそのファイルを最初のページで評価し、それ以降に読み込まれた各ウィンドウのページでも評価します。そのあと 200 ミリ秒ごとに、最初のページの `window.hypersigSmoke` を文字列として読みます。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`\"pass\"`: the run writes `hypersig_desktop: smoke script passed`, closes the application, and `App::run` returns `Ok(())`.",
                    ja: "`\"pass\"`: `hypersig_desktop: smoke script passed` を書き、アプリを閉じます。`App::run` は `Ok(())` を返します。",
                },
                L {
                    en: "Any other value: the run fails with `ErrorKind::Smoke`, and the error's message holds the value, so a script can set the reason it failed.",
                    ja: "それ以外の値: `ErrorKind::Smoke` で失敗します。エラーのメッセージにはその値が入るので、スクリプトは失敗の理由を設定できます。",
                },
                L {
                    en: "Nothing, or an empty string, for 20 seconds after the script started: the run fails with `ErrorKind::Smoke`.",
                    ja: "スクリプトの開始から 20 秒たっても何も設定されていない（空文字列も含む）: `ErrorKind::Smoke` で失敗します。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The script below is the one CI runs in examples/notes, started with one file on the command line, so the application opens the list window and a window for that note. In the note's window, the script edits the text and clicks Save; in the list window, it waits for the save to reach the list over its stream. The page has loaded when the script runs, but the hypersig runtime may not have started on it yet, so the note's window waits a second before it types:",
                ja: "下のスクリプトは、CI が examples/notes で実行しているものです。コマンドラインに 1 つのファイルを渡して起動するので、アプリは一覧のウィンドウとそのメモのウィンドウを開きます。スクリプトは、メモのウィンドウではテキストを書き換えて保存のボタンを押し、一覧のウィンドウでは保存がストリームで一覧に届くのを待ちます。スクリプトが動く時点でページの読み込みは終わっていますが、hypersig のランタイムがまだ起動していないことがあるので、メモのウィンドウでは 1 秒待ってから入力します。",
            },
        ),
        code::block(
            Lang::JavaScript,
            Some("examples/notes/smoke.js"),
            NOTES_SMOKE,
        ),
    ])
}

/// A GitHub Actions workflow that runs the notes example's smoke run.
fn workflow(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A workflow",
                ja: "ワークフローの例",
            },
        ),
        para(
            site,
            L {
                en: "The workflow below takes the steps of this repository's desktop job that test, build, and run examples/notes on macOS, Windows, and Linux. A Linux runner has no display, so `xvfb-run` gives the window one, and the job installs WebKitGTK, which `hypersig_desktop` links on Linux. The last line checks the file the script saved, so the job fails if the save never reached the disk:",
                ja: "下のワークフローは、このリポジトリの desktop ジョブのうち、examples/notes を macOS、Windows、Linux でテストし、ビルドして起動するステップを取り出したものです。Linux のランナーにはディスプレイがないので `xvfb-run` でウィンドウにディスプレイを用意し、`hypersig_desktop` が Linux でリンクする WebKitGTK をインストールしています。最後の行はスクリプトが保存したファイルを確かめるので、保存がディスクに届かなければジョブは失敗します。",
            },
        ),
        code::block(Lang::Shell, Some(".github/workflows/desktop.yml"), WORKFLOW),
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
            &[
                L {
                    en: "[Testing without a window](desktop/testing/): `FakeOs`, for the handlers and pages under `cargo test`.",
                    ja: "[ウィンドウなしのテスト](desktop/testing/): `cargo test` でハンドラとページを試すための `FakeOs`。",
                },
                L {
                    en: "[Packaging](desktop/packaging/): the build that ships, without the `smoke` feature.",
                    ja: "[パッケージの作成](desktop/packaging/): `smoke` feature なしで配布するビルド。",
                },
            ],
        ),
    ])
}
