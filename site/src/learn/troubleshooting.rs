//! The page on reading hypersig's compile errors and on finding what went
//! wrong in a running page.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, title};

pub fn troubleshooting(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Troubleshooting).title(site)),
        lead(site, Doc::Learn(Learn::Troubleshooting).leads()),
        compile_errors(site),
        run_time(site),
    ])
}

/// How to read the errors hypersig's types and constants produce.
fn compile_errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Reading a compile error",
                ja: "コンパイルエラーを読む",
            },
        ),
        para(
            site,
            L {
                en: "Most mistakes in a hypersig page fail to compile, and hypersig writes the first line of the error for its own traits, so that line names the mistake in the page's terms. The `note:` lines below it say what the trait accepts. The `help:` lines and the `required by a bound` notes point into hypersig's source, and you can skip them until the first line is not enough.",
                ja: "hypersig のページの間違いはたいていコンパイルエラーになります。hypersig は自分のトレイトについてエラーの 1 行目を書いているので、1 行目がページの言葉で間違いを指します。その下の `note:` の行は、トレイトが受け取るものを書いています。`help:` の行と `required by a bound` の注記は hypersig のソースを指しているので、1 行目で足りないときまで読み飛ばしてかまいません。",
            },
        ),
        subheading(
            site,
            L {
                en: "E0277 on a line of routes!",
                ja: "routes! の行での E0277",
            },
        ),
        para(
            site,
            L {
                en: "`routes!` hands each handler to the router, so an error about a handler points at the `routes!` block or at the handler's name on its line. `cannot serve this page or action` with `not a handler of the router's state and the arguments` means the parameters are off: a handler takes the router's state first and the arguments second, as `async fn add(state: AppState, draft: String)`, with at most one extractor between them. The first slot is `()` on a router without state, and the second is `()` for a page without path parameters or an action without arguments. `is not a reply of an action` means the return type is off, and its `note:` lists the types an action's handler returns.",
                ja: "`routes!` は各ハンドラをルーターに渡すので、ハンドラについてのエラーは `routes!` のブロックか、その行のハンドラ名を指します。`cannot serve this page or action` と `not a handler of the router's state and the arguments` が出たら、引数が合っていません。ハンドラは `async fn add(state: AppState, draft: String)` のように 1 つ目にルーターの状態を、2 つ目に引数を受け取り、そのあいだに置けるエクストラクタは 1 つまでです。状態のないルーターでは 1 つ目が `()` になり、パスのパラメータがないページと引数のないアクションでは 2 つ目が `()` になります。`is not a reply of an action` が出たら戻り値の型が合っていません。`note:` にアクションのハンドラが返せる型が並びます。",
            },
        ),
        show(Snippet::TroubleHandler),
        subheading(
            site,
            L {
                en: "E0080 on a constant",
                ja: "定数での E0080",
            },
        ),
        para(
            site,
            L {
                en: "A route's path, an `Id`, an `attr!` or `tag!` name, an `Origin`, and a `SafeUrl::new` are checked while the compiler evaluates the constant that holds them. A broken rule reads `evaluation panicked:` followed by the rule, and the line below names the constant, as in evaluation of `MEMBER` failed. The `note: inside` lines that follow show the `assert!` in hypersig that holds the rule. The check runs only for a value declared as a `const`, so a constructor called in a function body panics at run time with the same message.",
                ja: "ルートのパス、`Id`、`attr!` と `tag!` の名前、`Origin`、`SafeUrl::new` は、それを持つ定数をコンパイラが評価するときに検査されます。規則に反すると `evaluation panicked:` のあとに規則が書かれ、次の行が evaluation of `MEMBER` failed のように定数を指します。そのあとの `note: inside` の行は、規則を確かめる hypersig の `assert!` を示します。この検査は `const` で宣言した値にしか働かないので、関数の中で呼んだコンストラクタは同じメッセージで実行時にパニックします。",
            },
        ),
        show(Snippet::TroubleId),
        subheading(
            site,
            L {
                en: "Writing a computed signal",
                ja: "computed シグナルへの書き込み",
            },
        ),
        para(
            site,
            L {
                en: "A field marked `#[signals(computed)]` is a `Sig<S, T, Computed>`, which has no `set`, `toggle`, list statement, `bind`, or `patch`. `bind` reports E0277, a signal marked `Computed` cannot be written, and a method such as `set` reports E0599, the method `set` exists for struct `Sig<Cart, f64, Computed>`, but its trait bounds were not satisfied, with `Computed: Writable` among the bounds. Both mean the same thing: write a signal the expression reads, and the computed one follows. The reverse mistake, passing an unmarked field to `Page::computed`, reports E0277, `Page::computed` takes a field marked `#[signals(computed)]`.",
                ja: "`#[signals(computed)]` を付けたフィールドは `Sig<S, T, Computed>` になり、`set`、`toggle`、リストの文、`bind`、`patch` を持ちません。`bind` では E0277 の a signal marked `Computed` cannot be written が出ます。`set` などのメソッドを呼ぶと E0599 の the method `set` exists for struct `Sig<Cart, f64, Computed>`, but its trait bounds were not satisfied が出て、満たされない境界に `Computed: Writable` が並びます。どちらも意味は同じで、式が読むシグナルに書き込めば computed シグナルもそれに合わせて変わります。逆に印のないフィールドを `Page::computed` に渡すと、E0277 の `Page::computed` takes a field marked `#[signals(computed)]` が出ます。",
            },
        ),
        show(Snippet::TroubleComputed),
        subheading(
            site,
            L {
                en: "Other messages",
                ja: "ほかのメッセージ",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "E0599 `no associated function or constant named` on a struct with `#[derive(Signals)]`: the field name is misspelled, and the `help:` line names the field it is closest to.",
                    ja: "`#[derive(Signals)]` を付けた構造体での E0599 `no associated function or constant named`: フィールド名の綴りが違います。`help:` の行に一番近いフィールドが出ます。",
                },
                L {
                    en: "E0277, `Sig<Other, bool>` is not an expression of type `bool` on the page whose signals are `Counter`: the value has another type, or reads the signals of another page. The `help:` line says which page it belongs to.",
                    ja: "E0277 の `Sig<Other, bool>` is not an expression of type `bool` on the page whose signals are `Counter`: 値の型が違うか、別のページのシグナルを読んでいます。`help:` の行にどのページの値かが出ます。",
                },
                L {
                    en: "An error that names none of the above: each case under [`crates/hypersig/tests/ui/fail`](https://github.com/imaimai17468/hypersig/tree/main/crates/hypersig/tests/ui/fail) is one mistake that must not compile, with the compiler's message beside it in a `.stderr` file, so searching that directory for the first line of your error finds the mistake it stands for.",
                    ja: "上のどれにも当たらないエラー: [`crates/hypersig/tests/ui/fail`](https://github.com/imaimai17468/hypersig/tree/main/crates/hypersig/tests/ui/fail) の各ケースはコンパイルが通ってはいけない間違い 1 つで、隣の `.stderr` ファイルにコンパイラのメッセージがあります。エラーの 1 行目でこのディレクトリを検索すると、どの間違いのメッセージかがわかります。",
                },
            ],
        ),
    ])
}

/// How to find what went wrong once the page runs.
fn run_time(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Debugging a running page",
                ja: "動いているページを調べる",
            },
        ),
        para(
            site,
            L {
                en: "A mistake the types cannot hold panics while the page is built: two elements with one `Id`, a computed field that no `Page::computed` call names, or a list item used outside its closure. The server prints the panic with its message and answers nothing, so the browser reports an empty response. A test that renders each page with `Page::render` finds these before a visitor does, as [Testing](learn/testing/) shows.",
                ja: "型で表せない間違いは、ページを組み立てるときにパニックします。2 つの要素が同じ `Id` を持つ場合、どの `Page::computed` にも渡していない computed フィールドがある場合、リストの項目をクロージャの外で使った場合などです。サーバーはパニックのメッセージを出力して何も返さないので、ブラウザは空の応答として報告します。[テストを書く](learn/testing/) のように各ページを `Page::render` で描画するテストを書くと、訪問者が開く前に見つかります。",
            },
        ),
        para(
            site,
            L {
                en: "In the browser, the runtime writes each problem it meets to the console with the prefix `hypersig:`, such as an expression that threw, an element patch whose id is not on the page, or two list items with one key. Open the developer tools' console first when a button does nothing.",
                ja: "ブラウザでは、ランタイムが見つけた問題を `hypersig:` で始まる行としてコンソールに書きます。例外を投げた式、ページにない id を指す要素の差分、同じキーを持つリストの 2 つの項目などです。ボタンを押しても何も起きないときは、まず開発者ツールのコンソールを開いてください。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "hypersig: POST /save failed with 409: Someone else saved these notes first.
hypersig: no element with id \"todo-3\" to patch",
        ),
        subheading(
            site,
            L {
                en: "A call that fails",
                ja: "失敗した呼び出し",
            },
        ),
        para(
            site,
            L {
                en: "An action or a stream that answers with a status outside 2xx, or gets no answer, fails. The runtime then writes `hypersig: METHOD PATH failed with` and the status and body to the console. It dispatches `hs-error` on `<body>`, a `CustomEvent` whose `detail` holds the `status` and the `message`, for a script such as an error tracker. It also sets the signals the call names: [`Call::error`](api:hypersig/struct.Call.html#method.error) gets the body of the answer, or the status and its reason when the body is empty, or the browser's message when no answer arrived, and [`Call::status`](api:hypersig/struct.Call.html#method.status) gets the status, or `0` when no answer arrived. A handler that fails with `Result<_, E>` writes the sentence the page shows, and the page tells one failure from another by comparing the status with a `StatusCode`, not by reading the text.",
                ja: "アクションやストリームが 2xx 以外のステータスで答えるか、答えが届かないと、呼び出しは失敗します。そのときランタイムは、コンソールに `hypersig: METHOD PATH failed with` とステータスと本文を書きます。`<body>` で `hs-error` を送ります。これは `detail` に `status` と `message` を持つ `CustomEvent` で、エラーを記録するスクリプトなどが受け取ります。呼び出しが指定したシグナルにも書き込みます。[`Call::error`](api:hypersig/struct.Call.html#method.error) には答えの本文が入り、本文が空ならステータスとその説明が、答えが届かなければブラウザのメッセージが入ります。[`Call::status`](api:hypersig/struct.Call.html#method.status) にはステータスが入り、答えが届かなければ `0` です。`Result<_, E>` で失敗するハンドラは、ページに出す文を自分で書きます。ページは失敗の種類を、文を読むのではなくステータスを `StatusCode` と比べて見分けます。",
            },
        ),
        show(Snippet::TroubleCallError),
        subheading(
            site,
            L {
                en: "What a debug build adds",
                ja: "デバッグビルドで加わるもの",
            },
        ),
        para(
            site,
            L {
                en: "A debug build serves `/_hypersig/dev.js` beside the runtime, and each page it serves loads it. The script keeps a stream open to the server, and when a new server answers in place of the one that rendered the page, as after `cargo hypersig dev` rebuilds it, the page reloads and keeps the signals the visitor changed, where the route and the struct's fields are the same. A release build serves neither the script nor the stream, so the reload and the kept signals stop there.",
                ja: "デバッグビルドはランタイムの隣に `/_hypersig/dev.js` を配信し、配信する各ページがそれを読み込みます。このスクリプトはサーバーへのストリームを開いたままにし、`cargo hypersig dev` がビルドし直したあとのように、ページを描画したサーバーに代わって新しいサーバーが答えると、ページを読み込み直します。ルートと構造体のフィールドが同じなら、訪問者が変えたシグナルはそのまま残ります。リリースビルドはスクリプトもストリームも配信しないので、読み込み直しもシグナルの引き継ぎもありません。",
            },
        ),
    ])
}
