//! Why hypersig: the problem it addresses, how the HTML carries the page's
//! behavior, and what that costs.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of, title};

/// The body of the page that `Snippet::WhyCounter` renders, with the HTML
/// entities decoded. The snippet asserts the exact HTML.
const COUNTER_HTML: &str = r#"<body data-hs-signals='{"count":0}'>
  <div>
    <button data-hs-on-click='["set","count",["add",["sig","count"],["lit",1]]]'>+1</button>
    <span data-hs-text='["str",["sig","count"]]'>0</span>
    <button data-hs-on-click='["call","POST","/reset",null,null,null]'>Reset</button>
    <a href="/history/0"
       data-hs-attr='[["href",["concat",["lit","/history/"],["uri",["sig","count"]]]]]'>History</a>
  </div>
</body>"#;

pub fn why(site: Site) -> Part {
    parts(
        intro(site)
            .into_iter()
            .chain(halves(site))
            .chain(html(site))
            .chain(hydration(site))
            .chain(typed(site))
            .chain(state(site))
            .chain(fetching(site))
            .chain(costs(site)),
    )
}

fn intro(site: Site) -> [Part; 2] {
    [
        title(Doc::Learn(Learn::Why).title(site)),
        lead(site, Doc::Learn(Learn::Why).leads()),
    ]
}

fn halves(site: Site) -> [Part; 4] {
    [
        heading(
            site,
            L {
                en: "The server side and the browser side",
                ja: "サーバー側とブラウザ側",
            },
        ),
        para(
            site,
            L {
                en: "A page served by a Rust server splits into a server side and a browser side. The server side builds the HTML and answers requests. The browser side reacts to clicks and typing, and the browser runs only JavaScript or wasm. Two designs are common today, and each gives up something.",
                ja: "Rust のサーバーが配信するページは、サーバー側とブラウザ側の 2 つに分かれます。サーバー側は HTML を組み立ててリクエストに答えます。ブラウザ側はクリックや入力に反応し、ブラウザが実行できるのは JavaScript か wasm だけです。今よく使われる作り方は 2 つあり、どちらにも引き換えにあきらめるものがあります。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Compile the browser side from Rust to wasm, as Leptos does. The page and the server share types, so a mismatch between them is a compile error. The browser downloads the wasm and hydrates: it runs the page's components again to attach listeners and to rebuild the state the server already had, and the build needs `cargo-leptos`.",
                    ja: "Leptos のように、ブラウザ側を Rust から wasm にコンパイルする作り方です。画面とサーバーが型を共有するので、両者の食い違いはコンパイルエラーになります。ブラウザは wasm をダウンロードしてハイドレーションします。つまり、ページのコンポーネントをもう一度実行してリスナーを付け直し、サーバーがすでに持っていた状態を組み立て直します。ビルドには `cargo-leptos` が要ります。",
                },
                L {
                    en: "Send HTML and describe the behavior in attributes, as htmx does. Nothing is built for the browser and nothing hydrates. The URLs, the target selectors, and the expressions in the attributes are strings, so a typo in one shows up when someone clicks.",
                    ja: "htmx のように、HTML を送って動きを属性に書く作り方です。ブラウザ向けには何もビルドせず、ハイドレーションもありません。ただ、属性に書く URL、対象のセレクタ、式は文字列なので、どれかに打ち間違いがあっても、誰かがクリックするまで分かりません。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "hypersig takes the structure of the page from the second design and the checks from the first. The page is HTML with attributes, and Rust writes every attribute from a typed value.",
                ja: "hypersig はページの作りを 2 つ目の作り方から、検査を 1 つ目の作り方から取り入れています。ページは属性付きの HTML で、Rust がすべての属性を型の付いた値から書きます。",
            },
        ),
    ]
}

fn html(site: Site) -> [Part; 6] {
    [
        heading(
            site,
            L {
                en: "The behavior written in the HTML",
                ja: "HTML に書かれるページの動き",
            },
        ),
        para(
            site,
            L {
                en: "Here is the counter from the [Quick start](learn/), with a link added. `HISTORY` is a page whose path includes the count.",
                ja: "[クイックスタート](learn/) のカウンターに、リンクを 1 つ足したものです。`HISTORY` は、カウントをパスに含むページです。",
            },
        ),
        show(Snippet::WhyCounter),
        para(
            site,
            L {
                en: "`Page::render` writes this body, shown here with the HTML entities decoded:",
                ja: "`Page::render` は次の body を書きます。読みやすいように、HTML の文字参照を元の文字に戻して示します。",
            },
        ),
        code::block(Lang::Html, None, COUNTER_HTML),
        para(
            site,
            L {
                en: "Each attribute comes from one value in the Rust code. `data-hs-signals` is the `Counter` struct, serialized. `data-hs-on-click` is the statement `count.set(count + 1)` as JSON, and `count` in it is the path `Counter::count()` names. `/reset` is the path of `RESET`, and the `href` comes from `HISTORY`. A misspelled field, a value of the wrong type, or a link with a parameter its route does not take fails to compile before any of this is written.",
                ja: "それぞれの属性は Rust のコードの値 1 つから作られます。`data-hs-signals` は `Counter` 構造体をシリアライズしたものです。`data-hs-on-click` は文 `count.set(count + 1)` を JSON にしたもので、その中の `count` は `Counter::count()` が指すパスです。`/reset` は `RESET` のパスで、`href` は `HISTORY` から作られます。フィールド名の打ち間違い、型の違う値、ルートが受け取らないパラメータを渡すリンクは、この HTML が書かれる前にコンパイルエラーになります。",
            },
        ),
    ]
}

fn hydration(site: Site) -> [Part; 3] {
    [
        heading(
            site,
            L {
                en: "No hydration",
                ja: "ハイドレーションをしない",
            },
        ),
        para(
            site,
            L {
                en: "The page's code is written in Rust and is not sent to the browser. When the page loads, the runtime reads the state from `data-hs-signals` into a store, adds one listener per event type on `<body>`, and reads the `data-hs-*` attributes. It downloads no component code and runs no component again, because the attributes already hold every expression and statement the browser runs. Like Qwik's resumability, the state and the listeners are sent in the HTML.",
                ja: "ページのコードは Rust で書かれ、ブラウザには送られません。ページを読み込むと、ランタイムは `data-hs-signals` から状態を読んでストアに入れ、`<body>` にイベントの種類ごとに 1 つのリスナーを付け、`data-hs-*` 属性を読みます。ブラウザが実行する式と文はすべて属性に入っているので、コンポーネントのコードをダウンロードすることも、コンポーネントを実行し直すこともありません。Qwik の resumability と同じく、状態とリスナーは HTML に含めて送られます。",
            },
        ),
        para(
            site,
            L {
                en: "`Page::render` also evaluates each binding with the initial values, so the HTML already shows `0` and the link already points to `/history/0`. The page displays correctly before the runtime loads, and the runtime is the same file for every page: about 42 KB, or about 14 KB gzipped, embedded in the crate.",
                ja: "`Page::render` は束縛も初期値で評価するので、HTML の時点で `0` が表示され、リンクも `/history/0` を指しています。ランタイムを読み込む前からページは正しく表示され、ランタイムはどのページでも同じファイルです。大きさは約 42 KB（gzip で約 14 KB）で、クレートに埋め込まれています。",
            },
        ),
    ]
}

fn typed(site: Site) -> [Part; 4] {
    [
        heading(
            site,
            L {
                en: "Typed from page to server",
                ja: "画面からサーバーまで型で検査",
            },
        ),
        para(
            site,
            L {
                en: "The rule that keeps the attributes correct is that the browser never receives a string written by hand. Each kind of string has a type that produces it:",
                ja: "属性を正しく保つ決まりは、手で書いた文字列をブラウザに渡さないことです。文字列の種類ごとに、それを作る型があります。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Signal names come from `#[derive(Signals)]`, which adds one typed accessor per field.",
                    ja: "シグナルの名前は `#[derive(Signals)]` から作られます。この derive はフィールドごとに型付きのアクセサを 1 つ足します。",
                },
                L {
                    en: "Expressions and statements are `Expr` and `Stmt` values, whose operators check the types of both sides.",
                    ja: "式と文は `Expr` と `Stmt` の値で、その演算子は両辺の型を検査します。",
                },
                L {
                    en: "Action URLs come from `Action<S, A>`, which also fixes the page an action belongs to and the type of its arguments.",
                    ja: "アクションの URL は `Action<S, A>` から作られます。この型は、アクションが属するページと引数の型も決めます。",
                },
                L {
                    en: "Page links come from `Route<P, Q>`, declared in `routes!` with the handler that serves it, so a link carries the right path parameters and query.",
                    ja: "ページへのリンクは `Route<P, Q>` から作られます。ルートは配信するハンドラと一緒に `routes!` で宣言するので、リンクは正しいパスのパラメータとクエリを持ちます。",
                },
                L {
                    en: "Patch targets come from `Fragment::ID`, which the page and the patch read from the same constant.",
                    ja: "差分を適用する先は `Fragment::ID` から作られます。ページと差分は同じ定数を読みます。",
                },
                L {
                    en: "Element names for DOM calls come from `NodeRef`.",
                    ja: "DOM の呼び出しで使う要素の名前は `NodeRef` から作られます。",
                },
            ],
        ),
        show(Snippet::WrongType),
    ]
}

fn state(site: Site) -> [Part; 2] {
    [
        heading(
            site,
            L {
                en: "The server owns the state",
                ja: "状態はサーバーが持つ",
            },
        ),
        para(
            site,
            L {
                en: "The browser keeps signals for what changes on the page, and the server keeps the data. An action answers with patches over Server-Sent Events: new signal values, or a fragment of HTML that replaces an element of the page. A statement runs in the browser at once, and an action call sent with it reaches the server after, so the page changes before the server answers without extra code. [Talking to the server](learn/server/) covers actions, patches, and streams.",
                ja: "ブラウザはページの上で変わるものをシグナルとして持ち、データはサーバーが持ちます。アクションは Server-Sent Events で差分を返します。差分はシグナルの新しい値か、ページの要素と差し替える HTML の断片です。文はブラウザでただちに実行され、一緒に送ったアクションの呼び出しはそのあとでサーバーに届くので、サーバーの応答を待たずに画面が変わります。そのための追加のコードは要りません。アクション、差分、ストリームは [サーバーとのやり取り](learn/server/) で説明します。",
            },
        ),
    ]
}

fn fetching(site: Site) -> [Part; 6] {
    [
        heading(
            site,
            L {
                en: "The unit of fetching",
                ja: "データを取る単位",
            },
        ),
        para(
            site,
            L {
                en: "In RSC and Next.js, data fetching is written with `await` inside server components, so each component starts its fetch separately and the results appear in arrival order, one Suspense boundary at a time. Depending on where the boundaries sit, the page fills in piece by piece, and a child whose fetch waits for its parent's `await` puts the fetches in series. In Remix and TanStack Start, each route's loader runs in parallel before the page is shown, but the loaders run again after a change, so toggling one checkbox fetches the page's or the list's data again.",
                ja: "RSC と Next.js では、データを取得する処理をサーバーコンポーネントの中に `await` で書くので、取得はコンポーネントごとに別々に始まり、結果は Suspense の境界ごとに届いた順で表示されます。境界の置き方しだいで画面が少しずつ埋まり、親の `await` が終わるまで子の取得が始まらない書き方では、取得が直列に並びます。Remix と TanStack Start では、ルートごとの loader がページを表示する前に並列に実行されます。ただし変更のあとにも loader が再実行されるので、チェックボックスを 1 つ切り替えただけでもページや一覧のデータを取得し直します。",
            },
        ),
        para(
            site,
            L {
                en: "In hypersig, the first data is already in the page's HTML as the signal struct's JSON, so the browser fetches nothing to show the page. After a change, the action's handler returns only the values of the signals that changed, as in `Counter::count().patch(0)`, and the browser rewrites only what shows those values, so toggling one checkbox fetches neither the page nor the list again.",
                ja: "hypersig では、最初のデータはページの HTML にシグナル構造体の JSON として入っているので、表示のためにブラウザからデータを取得することはありません。変更のあとはアクションのハンドラが変わったシグナルの値だけを `Counter::count().patch(0)` のように返し、ブラウザはその値を使っている表示だけを書き換えるので、チェックボックスを 1 つ切り替えてもページや一覧を取得し直すことはありません。",
            },
        ),
        para(
            site,
            L {
                en: "What the browser runs on load and on an update, compared with other frameworks. An effect is the code that reads signal values and rewrites the DOM, and hypersig has one for each display attribute such as `data-hs-text` or `data-hs-show`.",
                ja: "読み込みと更新でブラウザが実行するものを、ほかのフレームワークと比べると次のようになります。表の「エフェクト」は、シグナルの値を読んで DOM を書き換える処理で、hypersig では `data-hs-text` や `data-hs-show` のような表示の属性 1 つにつき 1 つあります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["", "hypersig", "React", "Vue", "Solid", "Qwik"],
                ja: &["", "hypersig", "React", "Vue", "Solid", "Qwik"],
            },
            L {
                en: &[
                    &[
                        "App code sent to the browser",
                        "None (expressions travel as data in HTML attributes)",
                        "Every component",
                        "Every component",
                        "Every component",
                        "Only the handlers used, when they are used",
                    ],
                    &[
                        "What the browser runs on load",
                        "Every effect, once each",
                        "The whole component tree",
                        "The whole component tree",
                        "Each component once, to build its reactive dependencies",
                        "Nothing (state and handler locations are in the HTML)",
                    ],
                    &[
                        "What reruns on an update",
                        "Only the effects that read the value",
                        "The component holding the state and its descendants",
                        "The component's render function, diffed through a virtual DOM",
                        "Only the effects that read the value",
                        "Only the effects that read the value",
                    ],
                ],
                ja: &[
                    &[
                        "ブラウザに送るアプリのコード",
                        "なし（式は HTML の属性にデータとして入る）",
                        "コンポーネント全部",
                        "コンポーネント全部",
                        "コンポーネント全部",
                        "操作されたハンドラの分だけ、操作したときに",
                    ],
                    &[
                        "読み込み時にブラウザが実行するもの",
                        "すべてのエフェクトを 1 回ずつ",
                        "コンポーネントのツリー全体",
                        "コンポーネントのツリー全体",
                        "各コンポーネントを 1 回（リアクティブな依存関係を作る）",
                        "なし（状態とハンドラの場所は HTML に入っている）",
                    ],
                    &[
                        "更新で再実行する範囲",
                        "値を読んだエフェクトだけ",
                        "状態を持つコンポーネントと子孫",
                        "コンポーネントの描画関数（仮想 DOM で差分を取る）",
                        "値を読んだエフェクトだけ",
                        "値を読んだエフェクトだけ",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The cost is on failure. When the server answers with an error, the change made in the browser stays, and nothing rolls it back by itself. `Call::on_error` runs a statement where the call was made, so the page can undo or mark the one change whose save failed.",
                ja: "負担になるのは失敗したときです。サーバーがエラーを返しても、ブラウザで変えた値はそのまま残り、ひとりでに元へ戻ることはありません。`Call::on_error` は呼び出しを書いた場所で文を実行するので、保存に失敗した 1 つの変更だけを元に戻したり、印を付けたりできます。",
            },
        ),
    ]
}

fn costs(site: Site) -> [Part; 3] {
    [
        heading(
            site,
            L {
                en: "What it costs",
                ja: "引き換えにあきらめるもの",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The browser runs only what `Expr` and `Stmt` express: arithmetic, comparisons, text, lists, conditions, and calls to the server and the DOM. A computation outside that set runs on the server. [When bindings and statements are not enough](learn/escape-hatches/) covers the browser features a page reaches beyond them, and the applications that need a JavaScript library and fit hypersig poorly.",
                    ja: "ブラウザで実行できるのは、`Expr` と `Stmt` で書けることだけです。四則演算、比較、テキスト、リスト、条件、サーバーと DOM の呼び出しが書けます。それ以外の計算はサーバーで実行します。束縛と文のほかにページから使えるブラウザの機能と、JavaScript のライブラリが必要で hypersig に向かないアプリケーションは、[束縛と文で足りないとき](learn/escape-hatches/) で説明します。",
                },
                L {
                    en: "Signal values cross as JSON, so a signal's type implements serde's traits, and an `i64` past 2^53 loses precision in the browser.",
                    ja: "シグナルの値は JSON で送るので、シグナルの型は serde のトレイトを実装します。`i64` は 2^53 を超えると、ブラウザで精度が落ちます。",
                },
                L {
                    en: "Following a link loads the next page from the server. There is no client-side navigation that keeps the current page on screen.",
                    ja: "リンクをたどると、次のページをサーバーから読み込みます。今のページを画面に残したまま移るクライアント側の遷移はありません。",
                },
                L {
                    en: "Pages are served through axum, which is the only server hypersig integrates with.",
                    ja: "ページは axum で配信します。hypersig が組み込みで対応しているサーバーは axum だけです。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Next, the [Quick start](learn/) builds the counter and runs it.",
                ja: "次は [クイックスタート](learn/) で、このカウンターを作って動かします。",
            },
        ),
    ]
}
