//! The page on testing a page without a browser: `Page::simulate`, the
//! events it fires, what a test reads afterwards, and what only a browser
//! runs.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::TestingPages).title(site)),
        lead(site, Doc::Learn(Learn::TestingPages).leads()),
        simulation(site),
        events(site),
        targets(site),
        reading(site),
        effects(site),
        browser(site),
        next(site),
    ])
}

/// `Page::simulate` and a first example.
fn simulation(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Clicking and typing without a browser",
                ja: "ブラウザなしでクリックと入力を試す",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn simulate(&self) -> Simulation<'_, S>",
        ),
        para(
            site,
            L {
                en: "[`Page::simulate`](api:hypersig/struct.Page.html#method.simulate) returns a [`Simulation`](api:hypersig/testing/struct.Simulation.html) that starts from the page's initial values. Its events run the statements the page attached to them, such as `count.set(count + 1)`, on the server's evaluator, and recompute each `Page::computed` signal where the runtime does: between the steps of a `then` chain and after the event. It panics where `Page::render` does, so a page with two elements that share an id fails here too.",
                ja: "[`Page::simulate`](api:hypersig/struct.Page.html#method.simulate) は、ページの初期値から始まる [`Simulation`](api:hypersig/testing/struct.Simulation.html) を返します。イベントは、ページがそのイベントに付けた文（`count.set(count + 1)` など）をサーバーの評価器で実行し、ランタイムと同じ時点で `Page::computed` のシグナルを計算し直します。その時点は `then` でつないだ文の各段のあいだと、イベントのあとです。`Page::render` がパニックするところでは `simulate` もパニックするので、同じ id を持つ 2 つの要素があるページはここでも失敗します。",
            },
        ),
        para(
            site,
            L {
                en: "The test below clicks a counter's button twice, types a note, and clicks Save. The page's handler and its routes are as in [Testing handlers](learn/testing/), and a test of a page handler calls the handler first to get the `Page`.",
                ja: "下のテストはカウンターのボタンを 2 回押し、メモを入力して、保存のボタンを押します。ページのハンドラとルートは [ハンドラのテスト](learn/testing/) と同じ形で、ページのハンドラをテストするときは、まずハンドラを呼んで `Page` を受け取ります。",
            },
        ),
        show(Snippet::TestingSimulation),
    ])
}

/// The events a simulation fires.
fn events(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "The events",
                ja: "イベント",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn click(&mut self, target: impl Target<S>) -> Effects<S>\npub fn input(&mut self, target: impl Target<S>, text: &str) -> Effects<S>\npub fn change(&mut self, target: impl Target<S>, text: &str) -> Effects<S>\npub fn submit(&mut self, target: impl Target<S>) -> Effects<S>\npub fn pause(&mut self) -> Effects<S>",
        ),
        table_of(
            site,
            L {
                en: &["Method", "What it runs"],
                ja: &["メソッド", "実行するもの"],
            },
            L {
                en: &[
                    &[
                        "[`click`](api:hypersig/testing/struct.Simulation.html#method.click)",
                        "The `on_click` of the element or the closest element around it, as a click bubbles. A bound checkbox or radio is then checked, and the `on_input` and `on_change` around it run. A `<button>` inside a `<form>`, unless its `type` is `button` or `reset`, then submits the form. A button that names its form with `form_owner` is not followed to it.",
                    ],
                    &[
                        "[`input`](api:hypersig/testing/struct.Simulation.html#method.input)",
                        "One `input` event: the control's `bind` reads `text` as the runtime does, then the closest `on_input` runs. `on_change` does not, because the browser fires `change` when the visitor leaves the field.",
                    ],
                    &[
                        "[`change`](api:hypersig/testing/struct.Simulation.html#method.change)",
                        "`input` as above, then `change`, on which a `bind_select` reads `text` as the JSON of an option's value, and the closest `on_change` runs. It stands for picking an option or leaving a field.",
                    ],
                    &[
                        "[`submit`](api:hypersig/testing/struct.Simulation.html#method.submit)",
                        "The closest `on_submit` from a `<form>` up",
                    ],
                    &[
                        "[`pause`](api:hypersig/testing/struct.Simulation.html#method.pause)",
                        "Each statement given to `on_debounced`, once, as the browser runs it when the visitor stops typing",
                    ],
                ],
                ja: &[
                    &[
                        "[`click`](api:hypersig/testing/struct.Simulation.html#method.click)",
                        "要素か、その外側でいちばん近い要素の `on_click`。クリックが外側へ伝わるのと同じです。束縛したチェックボックスやラジオボタンはそのあとチェックが入り、外側の `on_input` と `on_change` が動きます。`<form>` の中の `<button>` は、`type` が `button` か `reset` でなければ、そのあとフォームを送信します。`form_owner` でフォームを指定したボタンからは、そのフォームをたどりません。",
                    ],
                    &[
                        "[`input`](api:hypersig/testing/struct.Simulation.html#method.input)",
                        "1 回の `input` イベント。入力欄の `bind` がランタイムと同じように `text` を読み、いちばん近い `on_input` が動きます。`on_change` は動きません。ブラウザが `change` を送るのは、訪問者が入力欄を離れたときだからです。",
                    ],
                    &[
                        "[`change`](api:hypersig/testing/struct.Simulation.html#method.change)",
                        "上の `input` のあとに `change`。`bind_select` は `text` を選択肢の値の JSON として読み、いちばん近い `on_change` が動きます。選択肢を選ぶ操作や、入力欄を離れる操作にあたります。",
                    ],
                    &[
                        "[`submit`](api:hypersig/testing/struct.Simulation.html#method.submit)",
                        "`<form>` から外側へたどって、いちばん近い `on_submit`",
                    ],
                    &[
                        "[`pause`](api:hypersig/testing/struct.Simulation.html#method.pause)",
                        "`on_debounced` に渡した文を 1 回ずつ。訪問者が入力をやめたときにブラウザが動かすのと同じです",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`input` reads text as a number control does in the browser: text that holds no number leaves the signal alone, and an `i32` or `i64` is truncated. A date control leaves its signal alone for text that holds no `YYYY-MM-DD` date, and an emptied `Option` control sets `None`. Each event panics where it runs nothing: a click with no `on_click` on the element or around it, typing into a control that has neither a `bind` nor an `on_input` around it, typing into a checkbox or a radio, which `click` checks, and `submit` on an element that is not a `<form>`.",
                ja: "`input` は、ブラウザの数値の入力欄と同じように文字列を読みます。数値でない文字列ではシグナルを変えず、`i32` と `i64` には小数点以下を切り捨てた値を書きます。日付の入力欄は `YYYY-MM-DD` の日付でない文字列ではシグナルを変えず、空にした `Option` の入力欄は `None` を書きます。どのイベントも、何も実行しないときはパニックします。要素にも外側にも `on_click` がないクリック、`bind` も外側の `on_input` もない入力欄への入力、`click` でチェックするチェックボックスやラジオボタンへの入力、`<form>` でない要素への `submit` です。",
            },
        ),
    ])
}

/// The handles a test names an element by.
fn targets(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Naming an element",
                ja: "要素を指す",
            },
        ),
        para(
            site,
            L {
                en: "A test names the element by the `NodeRef`, `DialogRef`, `FileInput`, or `Id` the page put on it, each of which implements the sealed trait [`Target<S>`](api:hypersig/testing/trait.Target.html). A renamed handle fails to compile in the test too. A list's template carries the same `NodeRef` on every item, and [`testing::nth`](api:hypersig/testing/fn.nth.html) picks the `index`th of them, from 0, in document order.",
                ja: "テストは、ページが要素に付けた `NodeRef`、`DialogRef`、`FileInput`、`Id` で要素を指します。どれも封印されたトレイト [`Target<S>`](api:hypersig/testing/trait.Target.html) を実装しています。名前を変えると、テストのほうもコンパイルが通らなくなります。リストのテンプレートはどの項目にも同じ `NodeRef` を付けるので、[`testing::nth`](api:hypersig/testing/fn.nth.html) で、文書の順に 0 から数えて `index` 番目の要素を選びます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn nth<T>(target: T, index: usize) -> Nth<T>",
        ),
        para(
            site,
            L {
                en: "A `NodeRef<S>` is a `Target<S>` only for the page whose signals are `S`, so a test cannot click a handle of another page:",
                ja: "`NodeRef<S>` が `Target<S>` になるのは、シグナルが `S` のページに対してだけです。そのため、別のページのハンドルをクリックするテストはコンパイルできません。",
            },
        ),
        show(Snippet::SimulationWrongPage),
    ])
}

/// `value_of`, `element`, and `count`.
fn reading(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Reading the page after the events",
                ja: "イベントのあとのページを読む",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn value_of<T: DeserializeOwned, M: FromPage>(&self, signal: Sig<S, T, M>) -> T\npub fn element(&self, target: impl Target<S>) -> Element\npub fn count(&self, target: impl Target<S>) -> usize",
        ),
        table_of(
            site,
            L {
                en: &["Method", "What it returns"],
                ja: &["メソッド", "返すもの"],
            },
            L {
                en: &[
                    &[
                        "[`Simulation::value_of`](api:hypersig/testing/struct.Simulation.html#method.value_of)",
                        "The value a signal holds now, read as `Page::value_of` reads it",
                    ],
                    &[
                        "[`Simulation::element`](api:hypersig/testing/struct.Simulation.html#method.element)",
                        "The element as the page shows it now. It panics where no element or several carry the handle.",
                    ],
                    &[
                        "[`Element::text`](api:hypersig/testing/struct.Element.html#method.text)",
                        "The text of the element and everything inside it, as `textContent` reads it",
                    ],
                    &[
                        "[`Element::attr`](api:hypersig/testing/struct.Element.html#method.attr)",
                        "The value of an attribute named by an `Attr`. A bound attribute holds the value its expression gives now, and one set with `attr_when` or `disabled` holds `\"\"` while it is on. A bound control's own value is the attribute `attr::VALUE`, apart from a `<textarea>`, whose text it is.",
                    ],
                    &[
                        "[`Element::is_visible`](api:hypersig/testing/struct.Element.html#method.is_visible)",
                        "`false` where a `show` on the element or around it is false",
                    ],
                    &[
                        "[`Simulation::count`](api:hypersig/testing/struct.Simulation.html#method.count)",
                        "How many elements the page shows now that carry the handle: once per list item, and 0 for an element inside a `when` that is false",
                    ],
                ],
                ja: &[
                    &[
                        "[`Simulation::value_of`](api:hypersig/testing/struct.Simulation.html#method.value_of)",
                        "シグナルのいまの値。`Page::value_of` と同じように読みます",
                    ],
                    &[
                        "[`Simulation::element`](api:hypersig/testing/struct.Simulation.html#method.element)",
                        "いまページに表示されている要素。ハンドルを持つ要素がないときや複数あるときはパニックします。",
                    ],
                    &[
                        "[`Element::text`](api:hypersig/testing/struct.Element.html#method.text)",
                        "要素とその中のすべてのテキスト。`textContent` と同じ読み方です",
                    ],
                    &[
                        "[`Element::attr`](api:hypersig/testing/struct.Element.html#method.attr)",
                        "`Attr` で指定した属性の値。束縛した属性は式のいまの値を持ち、`attr_when` や `disabled` で付けた属性は、付いているあいだ `\"\"` を持ちます。束縛した入力欄自身の値は `attr::VALUE` の属性で読みます。ただし `<textarea>` ではテキストが値です。",
                    ],
                    &[
                        "[`Element::is_visible`](api:hypersig/testing/struct.Element.html#method.is_visible)",
                        "要素か外側の要素の `show` が偽なら `false`",
                    ],
                    &[
                        "[`Simulation::count`](api:hypersig/testing/struct.Simulation.html#method.count)",
                        "ハンドルを持つ要素のうち、いまページに表示されている数。リストの項目ごとに 1 つと数え、条件が偽の `when` の中にある要素は 0 個と数えます",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The test below checks a list: `count` sees one `DONE` per item and no `HINT` while `editing` is false, and `nth` clicks the second item.",
                ja: "下のテストはリストを確かめます。`count` は項目ごとに 1 つの `DONE` を数え、`editing` が偽のあいだ `HINT` は 0 個です。`nth` で 2 つ目の項目をクリックします。",
            },
        ),
        show(Snippet::TestingListItems),
    ])
}

/// What `Effects` records in place of running it.
fn effects(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Calls, links, and browser statements",
                ja: "呼び出し、リンク、ブラウザの文",
            },
        ),
        para(
            site,
            L {
                en: "A statement that needs the server or the browser is recorded in the [`Effects`](api:hypersig/testing/struct.Effects.html) the event returns, and not run.",
                ja: "サーバーかブラウザが必要な文は実行せず、イベントが返す [`Effects`](api:hypersig/testing/struct.Effects.html) に記録します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method of `Effects<S>`", "What it returns"],
                ja: &["`Effects<S>` のメソッド", "返すもの"],
            },
            L {
                en: &[
                    &[
                        "[`calls(&SAVE)`](api:hypersig/testing/struct.Effects.html#method.calls)",
                        "The arguments of each call to `SAVE`, in order, typed as the action's arguments. An `Action`, a `StreamAction`, and an `Upload` each work.",
                    ],
                    &[
                        "[`navigates_to`](api:hypersig/testing/struct.Effects.html#method.navigates_to), [`pushes_url`](api:hypersig/testing/struct.Effects.html#method.pushes_url), [`replaces_url`](api:hypersig/testing/struct.Effects.html#method.replaces_url)",
                        "The URL of the first `Link::navigate`, `Link::push_url`, or `Link::replace_url` the event ran",
                    ],
                    &[
                        "[`runs_in_browser(FIELD.focus())`](api:hypersig/testing/struct.Effects.html#method.runs_in_browser)",
                        "Whether the event asked the browser to focus `FIELD`. It covers the statements of a `NodeRef` and a `DialogRef` and `FileInput::clear`, and any other statement gives `false`, because the simulation runs it itself.",
                    ],
                ],
                ja: &[
                    &[
                        "[`calls(&SAVE)`](api:hypersig/testing/struct.Effects.html#method.calls)",
                        "`SAVE` の呼び出しごとの引数を順に、アクションの引数の型で返します。`Action`、`StreamAction`、`Upload` のどれでも使えます。",
                    ],
                    &[
                        "[`navigates_to`](api:hypersig/testing/struct.Effects.html#method.navigates_to)、[`pushes_url`](api:hypersig/testing/struct.Effects.html#method.pushes_url)、[`replaces_url`](api:hypersig/testing/struct.Effects.html#method.replaces_url)",
                        "イベントが最初に実行した `Link::navigate`、`Link::push_url`、`Link::replace_url` の URL",
                    ],
                    &[
                        "[`runs_in_browser(FIELD.focus())`](api:hypersig/testing/struct.Effects.html#method.runs_in_browser)",
                        "イベントがブラウザに `FIELD` へのフォーカスを求めたかどうか。`NodeRef` と `DialogRef` の文と `FileInput::clear` が対象で、ほかの文では `false` です。ほかの文はシミュレーションが自分で実行するからです。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The simulation makes the writes the runtime makes when it sends a call, so the indicator turns `true`, the error signal `\"\"`, and the status `0`, and it never answers the call. The indicator therefore stays `true`, `on_success` and `on_error` do not run, and a second `exclusive` call to the same action is skipped. A test that needs the answer calls the handler, as [Testing handlers](learn/testing/) shows. A statement that reads `Expr::now`, `Expr::random_below`, or a date in the visitor's zone makes the simulation panic, because only the browser has that value, and so does one that computes a number JSON cannot hold.",
                ja: "シミュレーションは、ランタイムが呼び出しを送るときと同じ書き込みをするので、インジケーターは `true` に、エラーのシグナルは `\"\"` に、ステータスは `0` になります。呼び出しに応答は返りません。そのためインジケーターは `true` のままで、`on_success` と `on_error` は動かず、同じアクションへの 2 回目の `exclusive` の呼び出しは送られません。応答が必要なテストは、[ハンドラのテスト](learn/testing/) のようにハンドラを呼びます。`Expr::now`、`Expr::random_below`、訪問者のタイムゾーンでの日付を読む文では、シミュレーションはパニックします。その値はブラウザにしかないからです。JSON で表せない数を計算する文でもパニックします。",
            },
        ),
    ])
}

/// What a Rust test does not run, and the browser tests that do.
fn browser(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What runs only in the browser",
                ja: "ブラウザでだけ動くもの",
            },
        ),
        para(
            site,
            L {
                en: "A simulation runs only the statement of the event it fires. The statements of `effect`, `on_signal_change`, `on_load`, `on_visible`, a timer, and a key shortcut do not run, even where the browser would run one after a click changed a signal it reads. It fires only `click`, `input`, `change`, and `submit`, so a statement on a double click, focus, blur, the pointer, a key, or a custom event never runs. The runtime applying a server's answer, focus, and the page's layout are outside a Rust test too. Drive those in a browser. The repository's [`e2e`](https://github.com/imaimai17468/hypersig/tree/main/e2e) directory starts each example and clicks through it in Chrome with Playwright, and a test of your application can follow its layout.",
                ja: "シミュレーションが実行するのは、発生させたイベントの文だけです。`effect`、`on_signal_change`、`on_load`、`on_visible`、タイマー、キーボードショートカットの文は実行しません。ブラウザでは、クリックでそれらが読むシグナルが変わると実行される文も含みます。発生させるイベントは `click`、`input`、`change`、`submit` だけなので、ダブルクリック、フォーカス、ブラー、ポインター、キー、カスタムイベントに付けた文も実行しません。ランタイムがサーバーの答えを当てる処理、フォーカス、ページのレイアウトも Rust のテストでは確かめられません。これらはブラウザで動かして確かめます。リポジトリの [`e2e`](https://github.com/imaimai17468/hypersig/tree/main/e2e) ディレクトリは例を 1 つずつ起動し、Playwright で Chrome を操作します。アプリのブラウザのテストも同じ構成で書けます。",
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
                    en: "[Reading a compile error](learn/compile-errors/): what the first line of a hypersig error names.",
                    ja: "[コンパイルエラーの読み方](learn/compile-errors/): hypersig のエラーの 1 行目が指すもの。",
                },
                L {
                    en: "[Debugging a running page](learn/debugging/): the panics and console lines a test did not catch.",
                    ja: "[動いているページを調べる](learn/debugging/): テストで見つからなかったパニックとコンソールの行。",
                },
            ],
        ),
    ])
}
