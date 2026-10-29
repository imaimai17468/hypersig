//! The page on expressions.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Expressions).title(site)),
        lead(site, Doc::Learn(Learn::Expressions).leads()),
        heading(
            site,
            L {
                en: "An expression over the signals",
                ja: "シグナルを使った式",
            },
        ),
        para(
            site,
            L {
                en: "An [`Expr<S, T>`](api:hypersig/struct.Expr.html) evaluates to a `T` in the browser, over the signals of the page whose struct is `S`. A `Sig` is an expression, and the operators and methods build larger ones. The server evaluates the same expressions for the first HTML.",
                ja: "[`Expr<S, T>`](api:hypersig/struct.Expr.html) は、構造体 `S` のページのシグナルを使い、ブラウザで評価すると `T` になる式です。`Sig` も式で、演算子やメソッドで大きな式を組み立てます。最初の HTML を作るときは、サーバーが同じ式を評価します。",
            },
        ),
        show(Snippet::Operators),
        heading(
            site,
            L {
                en: "Using one expression twice",
                ja: "1 つの式を 2 回使う",
            },
        ),
        para(
            site,
            L {
                en: "A `Sig` is `Copy`, so `price` and `quantity` appear in several expressions as they are. An `Expr` is not `Copy`. Its methods, such as `gt` and `to_fixed`, borrow it, and the operators take it either by value or by reference: `subtotal * 0.9` moves `subtotal`, and `&subtotal * 0.9` leaves it for the next use, as the sample does.",
                ja: "`Sig` は `Copy` なので、`price` と `quantity` はそのまま複数の式に使えます。`Expr` は `Copy` ではありません。`gt` や `to_fixed` のようなメソッドは式を借用し、演算子は値でも参照でも受け取ります。`subtotal * 0.9` は `subtotal` をムーブし、`&subtotal * 0.9` は次に使えるように残します。上のサンプルは後者を使っています。",
            },
        ),
        heading(
            site,
            L {
                en: "Constants",
                ja: "定数",
            },
        ),
        para(
            site,
            L {
                en: "`Expr::lit(&value)` turns any `Serialize` value into a constant expression, and `Expr::text(\"...\")` starts a `concat` chain with a constant string. Where a method takes an expression, it also accepts an `i32`, `i64`, `f64`, `bool`, `String`, or `&str` literal, and a reference to any `Serialize` value, through the [`IntoExpr`](api:hypersig/trait.IntoExpr.html) trait. An expression of the wrong type is a compile error, with a message that names the type the place expects:",
                ja: "`Expr::lit(&value)` は `Serialize` を実装する値を定数の式に変え、`Expr::text(\"...\")` は定数の文字列から `concat` の連なりを始めます。式を受け取るメソッドは [`IntoExpr`](api:hypersig/trait.IntoExpr.html) トレイトを通して、`i32`、`i64`、`f64`、`bool`、`String`、`&str` のリテラルと、`Serialize` を実装する任意の値への参照も受け取ります。型の違う式はコンパイルエラーになり、メッセージにはその場所が求める型が出ます。",
            },
        ),
        show(Snippet::WrongType),
        heading(
            site,
            L {
                en: "The operations",
                ja: "使える操作",
            },
        ),
        table_of(
            site,
            L {
                en: &["On", "Operations"],
                ja: &["対象", "操作"],
            },
            L {
                en: &[
                    &[
                        "`i32`, `i64`, and `f64`",
                        "`+`, `-`, `*`, `%`, `min`, `max`, `to_f64`, and `format`",
                    ],
                    &[
                        "`f64`",
                        "`/`, `floor`, `ceil`, `round`, `log10`, `pow`, and `to_fixed`",
                    ],
                    &[
                        "`String`",
                        "`concat`, `trim`, `to_lowercase`, `to_uppercase`, `contains`, `starts_with`, `is_empty`, `char_count`, `all_chars`, `any_char`",
                    ],
                    &["`bool`", "`and`, `or`, `!`"],
                    &[
                        "Ordered values",
                        "`gt`, `ge`, `lt`, `le` on the numbers, `String`, `Timestamp`, and `CalendarDate`",
                    ],
                    &[
                        "Any value",
                        "`equals`, `not_equals`, `some`, and `if_else(cond, then, otherwise)`",
                    ],
                    &[
                        "Display text",
                        "`to_text` on the numbers, `bool`, `String`, `CalendarDate`, and an enum with `#[derive(Segment)]`",
                    ],
                    &[
                        "Structs and `Option`",
                        "`at`, `with`, `is_some`, `is_none`, `unwrap_or`, which [Signals inside signals](learn/nested-signals/) shows",
                    ],
                    &[
                        "Lists",
                        "`len`, `is_empty`, `nth`, `map`, `count`, `sum_of`, `max_of`, `min_of`, which [Lists](learn/lists/) shows",
                    ],
                ],
                ja: &[
                    &[
                        "`i32`、`i64`、`f64`",
                        "`+`、`-`、`*`、`%`、`min`、`max`、`to_f64`、`format`",
                    ],
                    &[
                        "`f64`",
                        "`/`、`floor`、`ceil`、`round`、`log10`、`pow`、`to_fixed`",
                    ],
                    &[
                        "`String`",
                        "`concat`、`trim`、`to_lowercase`、`to_uppercase`、`contains`、`starts_with`、`is_empty`、`char_count`、`all_chars`、`any_char`",
                    ],
                    &["`bool`", "`and`、`or`、`!`"],
                    &[
                        "順序のある値",
                        "数値、`String`、`Timestamp`、`CalendarDate` の `gt`、`ge`、`lt`、`le`",
                    ],
                    &[
                        "任意の値",
                        "`equals`、`not_equals`、`some`、`if_else(cond, then, otherwise)`",
                    ],
                    &[
                        "表示用のテキスト",
                        "数値、`bool`、`String`、`CalendarDate`、`#[derive(Segment)]` を付けた列挙型の `to_text`",
                    ],
                    &[
                        "構造体と `Option`",
                        "`at`、`with`、`is_some`、`is_none`、`unwrap_or`。[入れ子のシグナル](learn/nested-signals/) で示します",
                    ],
                    &[
                        "リスト",
                        "`len`、`is_empty`、`nth`、`map`、`count`、`sum_of`、`max_of`、`min_of`。[リスト](learn/lists/) で示します",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Plural forms, dates, and relative times have expressions of their own, such as `plural` and `format_date`. [Expr](reference/expr/) in the reference lists every operation.",
                ja: "複数形、日付、相対時刻には `plural` や `format_date` のような専用の式があります。すべての操作は、リファレンスの [Expr](reference/expr/) に一覧があります。",
            },
        ),
        heading(
            site,
            L {
                en: "Numbers",
                ja: "数値",
            },
        ),
        para(
            site,
            L {
                en: "Arithmetic needs the [`Number`](api:hypersig/trait.Number.html) trait, which `i32`, `i64`, and `f64` implement. JavaScript keeps every number as a 64-bit float, so integer division would give a different result in the browser than in Rust, and only `f64` has `/`. Dividing an integer is a compile error; convert it with `to_f64()` first:",
                ja: "算術演算には [`Number`](api:hypersig/trait.Number.html) トレイトが必要で、`i32`、`i64`、`f64` が実装しています。JavaScript はすべての数を 64 ビットの浮動小数点数として持つので、整数の割り算はブラウザと Rust で結果が変わります。そのため `/` は `f64` にしかありません。整数を割るとコンパイルエラーになるので、先に `to_f64()` で変換します。",
            },
        ),
        show(Snippet::StateIntegerDivision),
        para(
            site,
            L {
                en: "[`NumberFormat`](api:hypersig/struct.NumberFormat.html) writes a number with a fixed count of decimals, grouped digits, and a prefix or a suffix. The browser and the first HTML write the same text, whatever the visitor's locale, and `NumberFormat::write` returns that text on the server:",
                ja: "[`NumberFormat`](api:hypersig/struct.NumberFormat.html) は、小数点以下の桁数、桁区切り、前後に付ける文字を決めて数を書きます。訪問者のロケールにかかわらず、ブラウザと最初の HTML は同じテキストを書き、サーバーでは `NumberFormat::write` がそのテキストを返します。",
            },
        ),
        show(Snippet::StateNumberFormat),
        bullets(
            site,
            &[
                L {
                    en: "An `i64` is exact in the browser from -2^53 to 2^53. A value past that from the server panics when the page or the patch is built, and arithmetic in the browser that leaves the range rounds without a check. Keep money in minor units, such as cents, in an `i64`.",
                    ja: "ブラウザで `i64` を正確に扱えるのは -2^53 から 2^53 までです。サーバーから送る値がこの範囲を外れると、ページや差分を組み立てるときに panic し、ブラウザでの計算がこの範囲を外れた場合は検査されずに丸められます。金額はセントのような最小単位の `i64` で持ってください。",
                },
                L {
                    en: "An `i32` result outside `i32`'s range is not checked in the browser. It fails to deserialize when an action call passes it, and the action answers 400.",
                    ja: "`i32` の範囲を外れた `i32` の結果はブラウザでは検査されません。アクションの呼び出しがその値を渡すとデシリアライズに失敗し、アクションは 400 を返します。",
                },
                L {
                    en: "`round` is JavaScript's `Math.round`, so `-2.5` rounds to `-2` where Rust gives `-3`. `to_fixed` and `NumberFormat` round a halfway value away from zero, as `Number.prototype.toFixed` does, where Rust's `format!` rounds it to even.",
                    ja: "`round` は JavaScript の `Math.round` なので、`-2.5` は `-2` になります。Rust では `-3` です。`to_fixed` と `NumberFormat` は `Number.prototype.toFixed` と同じく、ちょうど半分の値を 0 から遠い方へ丸めます。Rust の `format!` は偶数の方へ丸めます。",
                },
                L {
                    en: "`log10` and `pow` use each platform's math library, so the browser and the server can differ in the last bit.",
                    ja: "`log10` と `pow` はそれぞれの環境の数学ライブラリを使うので、ブラウザとサーバーで最後のビットが異なることがあります。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Strings",
                ja: "文字列",
            },
        ),
        para(
            site,
            L {
                en: "String operations follow JavaScript on the server as in the browser. `trim` removes what JavaScript's `trim` removes, `char_count` counts Unicode scalar values as Rust's `chars().count()` does, and strings are ordered by UTF-16 code unit. `all_chars` and `any_char` test each character against a [`CharClass`](api:hypersig/enum.CharClass.html) of ASCII characters, so the two sides agree whatever Unicode version each has.",
                ja: "文字列の操作はブラウザでもサーバーでも JavaScript に合わせて動きます。`trim` は JavaScript の `trim` と同じ文字を取り除き、`char_count` は Rust の `chars().count()` と同じく Unicode のスカラー値を数え、文字列は UTF-16 のコード単位で順序付けます。`all_chars` と `any_char` は各文字が ASCII 文字の [`CharClass`](api:hypersig/enum.CharClass.html) に入るかを調べるので、両者の Unicode のバージョンが違っても結果は一致します。",
            },
        ),
        show(Snippet::StateStrings),
        heading(
            site,
            L {
                en: "Comparing and choosing",
                ja: "比較と選択",
            },
        ),
        para(
            site,
            L {
                en: "`equals` and `not_equals` take any `T: PartialEq` and compare the two values as serde serializes them. That agrees with a derived `PartialEq` and may disagree with a hand-written one. [`if_else`](api:hypersig/fn.if_else.html), `fn if_else<S, T>(cond: impl IntoExpr<S, bool>, then: impl IntoExpr<S, T>, otherwise: impl IntoExpr<S, T>) -> Expr<S, T>`, gives `then` while `cond` is true and `otherwise` while it is false, and both branches must have the same type `T`.",
                ja: "`equals` と `not_equals` は任意の `T: PartialEq` を受け取り、2 つの値を serde がシリアライズした形で比べます。derive した `PartialEq` とは結果が一致しますが、手書きの `PartialEq` とは一致しないことがあります。[`if_else`](api:hypersig/fn.if_else.html)（`fn if_else<S, T>(cond: impl IntoExpr<S, bool>, then: impl IntoExpr<S, T>, otherwise: impl IntoExpr<S, T>) -> Expr<S, T>`）は、`cond` が真の間は `then`、偽の間は `otherwise` になります。2 つの分岐は同じ型 `T` でなければなりません。",
            },
        ),
        heading(
            site,
            L {
                en: "Naming a value once",
                ja: "値に一度だけ名前を付ける",
            },
        ),
        para(
            site,
            L {
                en: "The browser evaluates an expression at each place it is used. [`Expr::let_in`](api:hypersig/struct.Expr.html#method.let_in) evaluates a value once and hands its closure an expression that stands for it, so a value that costs a pass over a list is computed once however often the body uses it. The expression the closure receives means nothing outside the closure, and the types do not stop it from being moved out.",
                ja: "ブラウザは、式を使った場所ごとにその式を評価します。[`Expr::let_in`](api:hypersig/struct.Expr.html#method.let_in) は値を一度だけ評価し、その値を表す式をクロージャに渡します。そのため、リストを一巡する必要がある値も、本体で何回使っても計算は 1 回です。クロージャが受け取る式はクロージャの外では意味を持ちませんが、外へ持ち出すことは型では防げません。",
            },
        ),
        show(Snippet::StateLetIn),
        heading(
            site,
            L {
                en: "Values only the browser has",
                ja: "ブラウザにしかない値",
            },
        ),
        para(
            site,
            L {
                en: "`Expr::random_below(n)` draws an integer from 0 up to `n`, and `Expr::now()` reads the browser's clock in milliseconds. The server cannot know either value, so the first HTML leaves out what a binding that reads one would show, and the browser computes it once the runtime has started.",
                ja: "`Expr::random_below(n)` は 0 以上 `n` 未満の整数を引き、`Expr::now()` はブラウザの時計をミリ秒で読みます。どちらの値もサーバーにはわからないので、これらを読む束縛が表示するものは最初の HTML に入らず、ランタイムの起動後にブラウザが計算します。",
            },
        ),
        heading(
            site,
            L {
                en: "A temperature converter",
                ja: "気温の変換器",
            },
        ),
        para(
            site,
            L {
                en: "The converter below computes Fahrenheit and a description from one `f64`, and each button is one statement.",
                ja: "下の変換器は 1 つの `f64` から華氏の値と説明文を計算します。ボタンはそれぞれ 1 つの文です。",
            },
        ),
        demo::show(site, Demo::Temperature),
        note(
            site,
            L {
                en: "The browser runs only the operations hypersig provides. Anything else, such as calling a JavaScript library, goes through an action to the server, and its result comes back one network round trip later.",
                ja: "ブラウザが実行できるのは hypersig が用意した操作だけです。JavaScript ライブラリの呼び出しのような、それ以外の処理はアクションでサーバーに任せることになり、結果はネットワークの往復 1 回分だけ遅れて返ります。",
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
                en: "[Statements](learn/statements/) uses expressions to change the signals. [Values that follow the signals](learn/bindings/) puts them into text, visibility, classes, and attributes.",
                ja: "[文](learn/statements/) では式を使ってシグナルを変更します。[シグナルに追従する値](learn/bindings/) では式をテキスト、表示と非表示、クラス、属性に使います。",
            },
        ),
    ])
}
