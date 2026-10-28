//! The page on statements that run without an event: timers, the load of an
//! element, and changes of signals, with the browser's clock and random
//! numbers.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Timers).title(site)),
        lead(site, Doc::Learn(Learn::Timers).leads()),
        heading(
            site,
            L {
                en: "Five ways to run a statement",
                ja: "文を実行する 5 つの方法",
            },
        ),
        para(
            site,
            L {
                en: "Each method below is on [`El<S>`](api:hypersig/html/struct.El.html) and takes a statement, a `Stmt<S>` or anything that turns into one. The statement runs in the browser at the time the table gives, with no click or keystroke.",
                ja: "下のメソッドはどれも [`El<S>`](api:hypersig/html/struct.El.html) のメソッドで、文（`Stmt<S>` か、文に変換できる値）を受け取ります。文は、クリックやキー入力がなくても、表に書いたタイミングでブラウザで実行されます。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "When the statement runs"],
                ja: &["メソッド", "文を実行するタイミング"],
            },
            L {
                en: &[
                    &[
                        "[`on_interval(period: Duration, stmt)`](api:hypersig/html/struct.El.html#method.on_interval)",
                        "Every `period` while the element is in the document",
                    ],
                    &[
                        "[`on_timeout(delay: Duration, stmt)`](api:hypersig/html/struct.El.html#method.on_timeout)",
                        "Once, `delay` after the element is added to the document, unless the element has been removed by then",
                    ],
                    &[
                        "[`on_load(stmt)`](api:hypersig/html/struct.El.html#method.on_load)",
                        "Once, when the element is added to the document",
                    ],
                    &[
                        "[`effect(stmt)`](api:hypersig/html/struct.El.html#method.effect)",
                        "When the element is added, and again whenever a signal the statement reads changes",
                    ],
                    &[
                        "[`on_signal_change(signal: Sig<S, T, R>, stmt)`](api:hypersig/html/struct.El.html#method.on_signal_change)",
                        "Whenever the signal, or any field inside it, changes, whatever changed it",
                    ],
                ],
                ja: &[
                    &[
                        "[`on_interval(period: Duration, stmt)`](api:hypersig/html/struct.El.html#method.on_interval)",
                        "要素が文書にある間、`period` ごとに実行します",
                    ],
                    &[
                        "[`on_timeout(delay: Duration, stmt)`](api:hypersig/html/struct.El.html#method.on_timeout)",
                        "要素が文書に追加されてから `delay` 後に 1 回だけ実行します。その時点で要素が取り除かれていれば実行しません",
                    ],
                    &[
                        "[`on_load(stmt)`](api:hypersig/html/struct.El.html#method.on_load)",
                        "要素が文書に追加されたときに 1 回だけ実行します",
                    ],
                    &[
                        "[`effect(stmt)`](api:hypersig/html/struct.El.html#method.effect)",
                        "要素が追加されたときと、文が読むシグナルが変わるたびに実行します",
                    ],
                    &[
                        "[`on_signal_change(signal: Sig<S, T, R>, stmt)`](api:hypersig/html/struct.El.html#method.on_signal_change)",
                        "シグナルか、その中のフィールドが変わるたびに実行します。何が変えたかは問いません",
                    ],
                ],
            },
        ),
        heading(
            site,
            L {
                en: "Intervals and timeouts",
                ja: "一定間隔の実行と 1 回だけの実行",
            },
        ),
        para(
            site,
            L {
                en: "`on_interval` and `on_timeout` take a [`std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html). One element holds one interval and one timeout: a second call of either replaces the first.",
                ja: "`on_interval` と `on_timeout` は [`std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html) を受け取ります。1 つの要素が持てる一定間隔の実行と 1 回だけの実行は 1 つずつで、どちらも 2 回目の呼び出しが 1 回目を置き換えます。",
            },
        ),
        show(Snippet::Timers),
        para(
            site,
            L {
                en: "The interval runs every 100 milliseconds, and its statement adds one only while `running` is true. The hint appears 3 seconds after the page loads.",
                ja: "100 ミリ秒ごとに文を実行し、`running` が true の間だけ 1 を足します。ヒントはページを読み込んでから 3 秒後に表示されます。",
            },
        ),
        para(
            site,
            L {
                en: "A number of milliseconds where the `Duration` belongs is E0308, mismatched types, so the unit is always written out:",
                ja: "`Duration` を渡すところにミリ秒の数を渡すと E0308（型の不一致）になるので、単位は必ずコードに書かれます。",
            },
        ),
        show(Snippet::IntervalMillis),
        para(
            site,
            L {
                en: "The type does not bound the length. `on_interval` panics on a period under 1 millisecond, which would run the statement as often as the browser lets a timer run. Both methods panic on a time longer than 2^31 - 1 milliseconds, about 24.8 days, which the browser's timer cannot wait. The panic happens when the element is built, so a test that builds the page finds it.",
                ja: "時間の長さは型では制限されません。`on_interval` は 1 ミリ秒未満の間隔で panic します。そのような間隔では、ブラウザのタイマーが許すかぎり頻繁に文が実行されてしまうからです。どちらのメソッドも 2^31 - 1 ミリ秒（約 24.8 日）より長い時間で panic します。ブラウザのタイマーはそれより長く待てません。panic は要素を組み立てるときに起きるので、ページを組み立てるテストで見つかります。",
            },
        ),
        heading(
            site,
            L {
                en: "Reacting to a change",
                ja: "変化に反応する",
            },
        ),
        para(
            site,
            L {
                en: "`effect` runs its statement when the element is added, and again whenever a signal the statement reads changes. The signals it depends on are the ones the statement reads when it runs, so there is no list of dependencies to keep in step. A signal read only after an action's response arrives is not recorded. `on_signal_change` names the signal to watch, for a statement that does not read it, such as an action call that saves a list each time the list changes. `T` is the signal's value type and `R` says where the signal is reached from, such as the page root or a list item, so any signal of the page fits.",
                ja: "`effect` は、要素が追加されたときと、文が読むシグナルが変わるたびに文を実行します。依存するシグナルは文が実行時に読むシグナルなので、依存するシグナルの一覧を書いて保守する必要はありません。アクションの応答が届いたあとで初めて読むシグナルは記録されません。`on_signal_change` は監視するシグナルを引数で指定します。リストが変わるたびにリストを保存するアクションを呼ぶ場合のように、文がそのシグナルを読まないときに使います。`T` はシグナルの値の型、`R` はページのルートやリストの項目など、シグナルをどこからたどるかを表す型なので、ページのどのシグナルでも渡せます。",
            },
        ),
        show(Snippet::EffectWatch),
        para(
            site,
            L {
                en: "A signal of another page is E0308, as it is for every method that takes a `Sig<S, ..>`.",
                ja: "ほかのページのシグナルを渡すと、`Sig<S, ..>` を受け取るほかのメソッドと同じく E0308 になります。",
            },
        ),
        heading(
            site,
            L {
                en: "The clock and random numbers",
                ja: "時計と乱数",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "Value"],
                ja: &["関数", "値"],
            },
            L {
                en: &[
                    &[
                        "[`Expr::now() -> Expr<S, f64>`](api:hypersig/struct.Expr.html#method.now)",
                        "The browser's current time, in milliseconds since the Unix epoch",
                    ],
                    &[
                        "[`Expr::random_below(bound: impl IntoExpr<S, i32>) -> Expr<S, i32>`](api:hypersig/struct.Expr.html#method.random_below)",
                        "A random integer from 0 up to but not including `bound`, drawn by the browser",
                    ],
                ],
                ja: &[
                    &[
                        "[`Expr::now() -> Expr<S, f64>`](api:hypersig/struct.Expr.html#method.now)",
                        "ブラウザの現在時刻。Unix エポックからのミリ秒です",
                    ],
                    &[
                        "[`Expr::random_below(bound: impl IntoExpr<S, i32>) -> Expr<S, i32>`](api:hypersig/struct.Expr.html#method.random_below)",
                        "0 以上 `bound` 未満のランダムな整数。ブラウザが生成します",
                    ],
                ],
            },
        ),
        show(Snippet::Clock),
        para(
            site,
            L {
                en: "Neither value exists on the server, so the first HTML leaves a binding that reads either one empty, and the browser fills it in. In the first HTML of the page above, the `<span>` that shows the time is empty, and the one that shows the face holds `1`.",
                ja: "ブラウザの時計の値もブラウザが生成する乱数も、サーバーにはありません。そのため、どちらかを読む束縛は最初の HTML では空のままで、ブラウザが値を入れます。上のページの最初の HTML では、時刻を表示する `<span>` は空で、出た目を表示する `<span>` には `1` が入っています。",
            },
        ),
        heading(
            site,
            L {
                en: "Example: saving a draft",
                ja: "例：下書きを保存する",
            },
        ),
        para(
            site,
            L {
                en: "The editor below focuses its field when it loads, counts every change to the text however it was made, and notes the time every 30 seconds:",
                ja: "下のエディタは読み込まれたときに入力欄にフォーカスし、テキストの変更を変更の仕方によらずすべて数え、30 秒ごとに時刻を記録します。",
            },
        ),
        show(Snippet::Effects),
        para(
            site,
            L {
                en: "Next, [Keyboard patterns](learn/keyboard/) moves focus with the arrow keys, and [Actions](learn/actions/) sends a statement's values to the server.",
                ja: "次の [キーボード操作](learn/keyboard/) では矢印キーでフォーカスを動かす方法を説明します。[アクション](learn/actions/) では文の値をサーバーに送る方法を説明します。",
            },
        ),
    ])
}
