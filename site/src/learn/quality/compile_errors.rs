//! The page on reading the compile errors hypersig's traits and constants
//! produce.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::CompileErrors).title(site)),
        lead(site, Doc::Learn(Learn::CompileErrors).leads()),
        reading(site),
        handlers(site),
        constants(site),
        computed(site),
        others(site),
        next(site),
    ])
}

/// Which lines of an error to read first.
fn reading(site: Site) -> Part {
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
                en: "Most mistakes in a hypersig page fail to compile, and hypersig writes the first line of the error for its own traits, so that line names the mistake in the page's terms. The `note:` lines below it say what the trait accepts. The `help:` lines and the `required by a bound` notes point into hypersig's source, and you can skip them until the first line is not enough. Here is the whole error for an action whose handler returns a `String`:",
                ja: "hypersig のページの間違いは多くの場合コンパイルエラーになります。hypersig は自分のトレイトについてエラーの 1 行目を書いているので、1 行目がページの言葉で間違いを指します。その下の `note:` の行は、トレイトが受け取るものを書いています。`help:` の行と `required by a bound` の注記は hypersig のソースを指しているので、1 行目で足りないときまで読み飛ばしてかまいません。次は、ハンドラが `String` を返すアクションのエラーの全文です。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "error[E0277]: `String` is not a reply of an action on the page whose signals are `Board`
  --> src/main.rs:11:1
   |
11 | / hypersig::routes! {
12 | |     fn actions(Router<()>);
13 | |     action RENAME: Action<Board, String> = post \"/rename\" => rename;
14 | | }
   | |_^ the trait `IntoActionResponse<Board>` is not implemented for `String`
   |
   = note: an action's handler returns `Patches<Board>`, a `SignalPatch<Board>`, a `ListPatch<Board>`, a tuple of response parts and one of those, or a `Result` of one of those whose error implements `IntoResponse`
note: required by a bound in `hypersig::RouterExt::action`
  --> src/axum.rs
   |
   |     fn action<S, A, H, M, Fut, P>(self, action: Action<S, A>, handler: H) -> Self
   |        ------ required by a bound in this associated function
...",
        ),
        table_of(
            site,
            L {
                en: &["Line", "What to do with it"],
                ja: &["行", "読み方"],
            },
            L {
                en: &[
                    &[
                        "`error[E0277]: ...`",
                        "Read it first. It names the value and the page in hypersig's words.",
                    ],
                    &[
                        "`-->` and the marked lines",
                        "The place in your code. For a handler this is the `routes!` block or the handler's name on its line.",
                    ],
                    &[
                        "`= note:`",
                        "What hypersig accepts in that place",
                    ],
                    &[
                        "`help:`, `required by a bound`, `note: inside`",
                        "Lines of hypersig's source. Skip them until the lines above are not enough.",
                    ],
                ],
                ja: &[
                    &[
                        "`error[E0277]: ...`",
                        "最初に読む行です。値とページを hypersig の言葉で指します。",
                    ],
                    &[
                        "`-->` と印の付いた行",
                        "自分のコードの場所。ハンドラなら `routes!` のブロックか、その行のハンドラ名です。",
                    ],
                    &[
                        "`= note:`",
                        "その場所で hypersig が受け取るもの",
                    ],
                    &[
                        "`help:`、`required by a bound`、`note: inside`",
                        "hypersig のソースの行。上の行で足りないときまで読み飛ばします。",
                    ],
                ],
            },
        ),
    ])
}

/// Errors on a line of `routes!`.
fn handlers(site: Site) -> Part {
    parts([
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
        show(Snippet::ErrorHandler),
    ])
}

/// Errors from a constant's evaluation.
fn constants(site: Site) -> Part {
    parts([
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
                ja: "ルートのパス、`Id`、`attr!` と `tag!` の名前、`Origin`、`SafeUrl::new` は、それを持つ定数をコンパイラが評価するときに検査されます。規則に反すると `evaluation panicked:` のあとに規則が書かれ、次の行が evaluation of `MEMBER` failed のように定数を指します。そのあとの `note: inside` の行は規則を確かめる hypersig の `assert!` を示します。この検査は `const` で宣言した値にしか働かないので、関数の中で呼んだコンストラクタは同じメッセージで実行時にパニックします。",
            },
        ),
        show(Snippet::ErrorId),
    ])
}

/// Errors from writing a computed signal.
fn computed(site: Site) -> Part {
    parts([
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
        show(Snippet::ErrorComputed),
    ])
}

/// Other messages, and where to look up the rest.
fn others(site: Site) -> Part {
    parts([
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
        para(
            site,
            L {
                en: "The other Learn pages show the compile error for the mistake their own types prevent, such as a misspelled signal on [The signal struct](learn/signals/).",
                ja: "Learn のほかのページでは、そのページの型が防ぐ間違いのコンパイルエラーを示しています。たとえば綴りを間違えたシグナルのエラーは [シグナル構造体](learn/signals/) にあります。",
            },
        ),
    ])
}

/// The pages to read next.
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
                    en: "[Debugging a running page](learn/debugging/): the mistakes that compile and show up while the page runs.",
                    ja: "[動いているページを調べる](learn/debugging/): コンパイルは通り、ページが動いているときに現れる間違い。",
                },
                L {
                    en: "[Testing handlers](learn/testing/): tests that find those mistakes before a visitor does.",
                    ja: "[ハンドラのテスト](learn/testing/): そうした間違いを訪問者より先に見つけるテスト。",
                },
            ],
        ),
    ])
}
