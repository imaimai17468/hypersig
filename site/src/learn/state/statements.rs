//! The page on statements.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Statements).title(site)),
        lead(site, Doc::Learn(Learn::Statements).leads()),
        heading(
            site,
            L {
                en: "A statement runs in the browser",
                ja: "文はブラウザで実行される",
            },
        ),
        para(
            site,
            L {
                en: "A [`Stmt<S>`](api:hypersig/struct.Stmt.html) is a change to the signals of the page whose struct is `S`. An event method such as `on_click` attaches it to an element, and the browser runs it on each event without a request. [Events](learn/events/) lists the events.",
                ja: "[`Stmt<S>`](api:hypersig/struct.Stmt.html) は、構造体 `S` のページのシグナルに対する変更です。`on_click` のようなイベントのメソッドが文を要素に付け、ブラウザはイベントのたびにリクエストを送らずに実行します。イベントの一覧は [イベント](learn/events/) にあります。",
            },
        ),
        heading(
            site,
            L {
                en: "Assigning a value",
                ja: "値を代入する",
            },
        ),
        para(
            site,
            L {
                en: "[`Sig::set`](api:hypersig/struct.Sig.html#method.set), `fn set(self, value: impl IntoExpr<S, T>) -> Stmt<S>`, assigns an expression of the signal's own type, and [`Sig::toggle`](api:hypersig/struct.Sig.html#method.toggle) flips a `bool`. The value is evaluated when the statement runs, so `count.set(count + 1)` reads the count at that moment.",
                ja: "[`Sig::set`](api:hypersig/struct.Sig.html#method.set)（`fn set(self, value: impl IntoExpr<S, T>) -> Stmt<S>`）はシグナル自身の型の式を代入し、[`Sig::toggle`](api:hypersig/struct.Sig.html#method.toggle) は `bool` を反転します。値は文を実行するときに評価されるので、`count.set(count + 1)` はその時点のカウントを読みます。",
            },
        ),
        para(
            site,
            L {
                en: "A constant of a type other than the literal types goes in by reference, as `&Column::Done`, because `IntoExpr` is implemented for `&T` where `T: Serialize`. Passing the enum by value is a compile error:",
                ja: "リテラルの型以外の定数は `&Column::Done` のように参照で渡します。`IntoExpr` は `T: Serialize` の `&T` に実装されているからです。列挙型を値で渡すとコンパイルエラーになります。",
            },
        ),
        show(Snippet::StateEnumByValue),
        heading(
            site,
            L {
                en: "One statement after another",
                ja: "文を順に実行する",
            },
        ),
        para(
            site,
            L {
                en: "[`Stmt::then`](api:hypersig/struct.Stmt.html#method.then), `fn then(self, next: impl IntoStmt<S>) -> Stmt<S>`, runs `next` after `self`. Each statement reads the values the ones before it wrote, including computed signals recomputed from them:",
                ja: "[`Stmt::then`](api:hypersig/struct.Stmt.html#method.then)（`fn then(self, next: impl IntoStmt<S>) -> Stmt<S>`）は `self` のあとに `next` を実行します。それぞれの文は、前の文が書き込んだ値を読みます。その値から計算し直した計算シグナルも同じです。",
            },
        ),
        show(Snippet::StateThenOrder),
        heading(
            site,
            L {
                en: "Branching",
                ja: "条件で分岐する",
            },
        ),
        para(
            site,
            L {
                en: "[`Stmt::when`](api:hypersig/struct.Stmt.html#method.when)`(cond, then)` runs `then` only while `cond` is true, and [`Stmt::if_else`](api:hypersig/struct.Stmt.html#method.if_else)`(cond, then, otherwise)` picks one of two statements. [`Stmt::none`](api:hypersig/struct.Stmt.html#method.none) does nothing, for a branch that has nothing to do. The condition is evaluated each time the statement runs, so one statement takes either branch on different clicks.",
                ja: "[`Stmt::when`](api:hypersig/struct.Stmt.html#method.when)`(cond, then)` は `cond` が真のときだけ `then` を実行し、[`Stmt::if_else`](api:hypersig/struct.Stmt.html#method.if_else)`(cond, then, otherwise)` は 2 つの文のどちらかを選びます。[`Stmt::none`](api:hypersig/struct.Stmt.html#method.none) は何もしない文で、することのない分岐に使います。条件は文を実行するたびに評価されるので、1 つの文がクリックごとに違う分岐をたどることがあります。",
            },
        ),
        heading(
            site,
            L {
                en: "Keeping a value for later statements",
                ja: "あとの文のために値を取っておく",
            },
        ),
        para(
            site,
            L {
                en: "[`Sig::let_in_stmt`](api:hypersig/struct.Sig.html#method.let_in_stmt) and `Expr::let_in_stmt` evaluate a value once, when the statement runs, and hand the closure an expression that reads that value. Statements that run later inside the closure, such as a call's `on_error`, still read it after the signals it came from have changed:",
                ja: "[`Sig::let_in_stmt`](api:hypersig/struct.Sig.html#method.let_in_stmt) と `Expr::let_in_stmt` は、文を実行するときに値を一度だけ評価し、その値を読む式をクロージャに渡します。呼び出しの `on_error` のように、クロージャの中で後から実行される文も、元のシグナルが変わったあとでその値を読めます。",
            },
        ),
        show(Snippet::StateLetInStmt),
        heading(
            site,
            L {
                en: "Other statements",
                ja: "ほかの文",
            },
        ),
        para(
            site,
            L {
                en: "An event method takes any [`IntoStmt<S>`](api:hypersig/trait.IntoStmt.html), and `then` joins any of them:",
                ja: "イベントのメソッドは [`IntoStmt<S>`](api:hypersig/trait.IntoStmt.html) を実装する値を何でも受け取り、`then` はそのどれでもつなげます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Statement", "Covered in"],
                ja: &["文", "説明するページ"],
            },
            L {
                en: &[
                    &[
                        "`push`, `retain`, `sort_by`, and the other list statements",
                        "[Lists](learn/lists/)",
                    ],
                    &[
                        "An action's `call()`, a `Call`",
                        "[Actions](learn/actions/)",
                    ],
                    &[
                        "`navigate`, `push_url`, and `replace_url` on a link",
                        "[Links and redirects](learn/links/)",
                    ],
                    &[
                        "`focus` and the other methods of a `NodeRef`",
                        "[Node refs](learn/node-refs/)",
                    ],
                    &["`Stmt::view_transition`", "[Styling](learn/styling/)"],
                ],
                ja: &[
                    &[
                        "`push`、`retain`、`sort_by` などのリストの文",
                        "[リスト](learn/lists/)",
                    ],
                    &[
                        "アクションの `call()`（`Call`）",
                        "[アクション](learn/actions/)",
                    ],
                    &[
                        "リンクの `navigate`、`push_url`、`replace_url`",
                        "[リンクとリダイレクト](learn/links/)",
                    ],
                    &[
                        "`NodeRef` の `focus` などのメソッド",
                        "[ノード参照](learn/node-refs/)",
                    ],
                    &["`Stmt::view_transition`", "[スタイル](learn/styling/)"],
                ],
            },
        ),
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型では検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`set` exists only on a signal the browser may write. A computed signal and a read-only view have none, so writing one is a compile error, which [Computed signals](learn/computed-signals/) shows.",
                    ja: "`set` があるのは、ブラウザが書き込めるシグナルだけです。計算シグナルと読み取り専用のビューには `set` がないので、書き込もうとするとコンパイルエラーになります。[計算シグナル](learn/computed-signals/) で示します。",
                },
                L {
                    en: "A statement that fails in the browser writes the error to the console with the prefix `hypersig:`, and the statements after it in the same chain do not run.",
                    ja: "ブラウザで失敗した文は `hypersig:` を先頭に付けたエラーをコンソールに書き出し、同じ連なりでそのあとに続く文は実行されません。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "A game turn",
                ja: "ゲームの 1 手",
            },
        ),
        para(
            site,
            L {
                en: "One click adds a bonus unless the game is over, then takes a life and ends the game when the last one is gone:",
                ja: "1 回のクリックで、ゲームが終わっていなければボーナスを加え、続いてライフを 1 つ減らし、最後のライフがなくなったらゲームを終えます。",
            },
        ),
        show(Snippet::Statements),
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
                en: "[Events](learn/events/) covers the events a statement runs on. [Binding form controls](learn/inputs/) writes signals from form controls without a statement.",
                ja: "[イベント](learn/events/) では文を実行するイベントを説明します。[入力欄の束縛](learn/inputs/) では文を書かずにフォームの入力欄からシグナルに書き込みます。",
            },
        ),
    ])
}
