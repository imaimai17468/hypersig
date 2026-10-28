//! The page on running a JavaScript library, such as a map or an editor,
//! inside a page.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show, show_file};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::JavaScript).title(site)),
        lead(site, Doc::Learn(Learn::JavaScript).leads()),
        heading(
            site,
            L {
                en: "Wrap the library in a custom element",
                ja: "ライブラリをカスタム要素で包む",
            },
        ),
        para(
            site,
            L {
                en: "Three methods connect a page to a custom element. `Page::script` loads the script that defines the element, `bind_attr` keeps an attribute of the element equal to a signal, and `El::on_custom` runs a statement on an event the element dispatches, with the event's `detail` as a typed expression. Declare the event once as a [`CustomEvent`](api:hypersig/html/struct.CustomEvent.html) whose type parameter is the type of its `detail`.",
                ja: "ページとカスタム要素は 3 つのメソッドでつながります。`Page::script` は要素を定義するスクリプトを読み込み、`bind_attr` は要素の属性をシグナルの値に合わせ、`El::on_custom` は要素が投げたイベントで文を実行します。イベントの `detail` は型付きの式として文に渡ります。イベントは型引数を `detail` の型にした [`CustomEvent`](api:hypersig/html/struct.CustomEvent.html) として 1 回宣言します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Item", "Signature", "What it does"],
                ja: &["項目", "シグネチャ", "役割"],
            },
            L {
                en: &[
                    &[
                        "[`Page::script`](api:hypersig/struct.Page.html#method.script)",
                        "`fn script(self, src: Asset) -> Self`",
                        "Adds a `<script type=\"module\">` for the file to `<head>`",
                    ],
                    &[
                        "[`El::bind_attr`](api:hypersig/html/struct.El.html#method.bind_attr)",
                        "`fn bind_attr(self, name: Attr, value: impl IntoExpr<S, String>) -> Self`",
                        "Renders the attribute with the expression's first value, and keeps it equal to the expression as signals change",
                    ],
                    &[
                        "[`El::on_custom`](api:hypersig/html/struct.El.html#method.on_custom)",
                        "`fn on_custom<T, B: IntoStmt<S>>(self, event: CustomEvent<T>, handle: impl FnOnce(Expr<S, T>) -> B) -> Self`",
                        "Runs the statement `handle` builds from the event's `detail` each time the event reaches the element",
                    ],
                    &[
                        "[`CustomEvent::new`](api:hypersig/html/struct.CustomEvent.html#method.new)",
                        "`const fn new(name: &'static str) -> CustomEvent<T>`",
                        "Declares an event of type `name`, as the script passes it to `new CustomEvent`",
                    ],
                ],
                ja: &[
                    &[
                        "[`Page::script`](api:hypersig/struct.Page.html#method.script)",
                        "`fn script(self, src: Asset) -> Self`",
                        "ファイルを読み込む `<script type=\"module\">` を `<head>` に追加します",
                    ],
                    &[
                        "[`El::bind_attr`](api:hypersig/html/struct.El.html#method.bind_attr)",
                        "`fn bind_attr(self, name: Attr, value: impl IntoExpr<S, String>) -> Self`",
                        "式の最初の値で属性を描画し、シグナルが変わるたびに属性を式の値に合わせます",
                    ],
                    &[
                        "[`El::on_custom`](api:hypersig/html/struct.El.html#method.on_custom)",
                        "`fn on_custom<T, B: IntoStmt<S>>(self, event: CustomEvent<T>, handle: impl FnOnce(Expr<S, T>) -> B) -> Self`",
                        "イベントが要素に届くたびに、`handle` がイベントの `detail` から作る文を実行します",
                    ],
                    &[
                        "[`CustomEvent::new`](api:hypersig/html/struct.CustomEvent.html#method.new)",
                        "`const fn new(name: &'static str) -> CustomEvent<T>`",
                        "スクリプトが `new CustomEvent` に渡す型名 `name` のイベントを宣言します",
                    ],
                ],
            },
        ),
        show(Snippet::CustomEventPage),
        para(
            site,
            L {
                en: "`Page::render` writes `<script type=\"module\" src=\"/star-rating.js\"></script>` into `<head>`, and the element as `<star-rating value=\"3\">`, with the signal's first value.",
                ja: "`Page::render` は `<head>` に `<script type=\"module\" src=\"/star-rating.js\"></script>` を書き、要素をシグナルの最初の値を入れた `<star-rating value=\"3\">` として書きます。",
            },
        ),
        para(
            site,
            L {
                en: "The browser listens on the element itself. An event dispatched without `bubbles` runs the statement when the script dispatches it on this element, and one with `bubbles` also when it is dispatched on an element inside. Calling `on_custom` twice for one event runs both statements in order. `CustomEvent::new` panics on an empty name or one that holds whitespace, and in a `const` that panic is a compile error.",
                ja: "ブラウザは要素そのものでイベントを待ち受けます。`bubbles` なしで投げたイベントは、スクリプトがこの要素で投げたときに文を実行します。`bubbles` 付きなら、中の要素で投げたときにも実行します。1 つのイベントに `on_custom` を 2 回呼ぶと、2 つの文を順に実行します。`CustomEvent::new` は空の名前や空白を含む名前で panic し、`const` の中ではその panic がコンパイルエラーになります。",
            },
        ),
        heading(
            site,
            L {
                en: "Example: a star rating",
                ja: "例：星の評価",
            },
        ),
        para(
            site,
            L {
                en: "The rating below is a custom element written in 34 lines of JavaScript. A click on a star dispatches `rate`, the statement puts its `detail` into the page's signal, and the signal goes back into the element's `value` attribute. The clear button changes only the signal, and the stars follow.",
                ja: "下の評価は 34 行の JavaScript で書いたカスタム要素です。星をクリックすると要素が `rate` を投げ、文がその `detail` をページのシグナルに入れ、シグナルが要素の `value` 属性に戻ります。クリアのボタンはシグナルだけを変え、星の表示がそれに合わせて変わります。",
            },
        ),
        demo::show(site, Demo::Rating),
        code::block(
            Lang::JavaScript,
            Some("star-rating.js"),
            demo::STAR_RATING_JS,
        ),
        heading(
            site,
            L {
                en: "Serving the script",
                ja: "スクリプトを配信する",
            },
        ),
        para(
            site,
            L {
                en: "`Page::script`, `Page::stylesheet`, and `Page::icon` take an [`Asset`](api:hypersig/struct.Asset.html), a constant such as `Asset::new(\"/star-rating.js\")`, and write its path into the page under the routes' `Base`. They serve nothing, so the router needs a service at each path. tower-http serves files from disk: `ServeFile` one file at one path, and `ServeDir` each file under a directory. Add it with its `fs` feature:",
                ja: "`Page::script`、`Page::stylesheet`、`Page::icon` は `Asset::new(\"/star-rating.js\")` のような定数の [`Asset`](api:hypersig/struct.Asset.html) を受け取り、そのパスをルートの `Base` の下に付けてページに書きます。ページにパスを書くだけでファイルは配信しないので、ルーターにはそれぞれのパスで配信するサービスが必要です。ディスクのファイルは tower-http で配信します。`ServeFile` は 1 つのファイルを 1 つのパスで、`ServeDir` はディレクトリの下のファイルをそれぞれ配信します。`fs` フィーチャーを有効にして追加してください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"tower-http = { version = "0.7", features = ["fs"] }"#,
        ),
        show(Snippet::StaticFiles),
        para(
            site,
            L {
                en: "The paths given to `ServeFile` and `ServeDir` are read from the directory the server was started in, so the server above expects `star-rating.js` and `assets/` there. Nothing checks that the router serves an `Asset`'s path: a path that no service answers loads nothing, and the browser's console shows the 404.",
                ja: "`ServeFile` と `ServeDir` に渡したパスは、サーバーを起動したディレクトリから読みます。上のサーバーは、そこに `star-rating.js` と `assets/` があるものとして動きます。`Asset` のパスをルーターが配信しているかどうかは検査されません。どのサービスも応答しないパスでは何も読み込まれず、ブラウザのコンソールに 404 が表示されます。",
            },
        ),
        para(
            site,
            L {
                en: "The scripts run after the runtime, in the order they were added. A custom element defined then is upgraded in place, so it keeps the attributes and the listeners the page gave it. Each `<script>` carries the page's `csp_nonce`, if there is one.",
                ja: "スクリプトはランタイムのあとに、追加した順に実行されます。そのときに定義されたカスタム要素はその場でアップグレードされるので、ページが与えた属性とリスナーはそのまま残ります。ページに `csp_nonce` があれば、各 `<script>` にそれが付きます。",
            },
        ),
        heading(
            site,
            L {
                en: "Write the names from the library's manifest",
                ja: "ライブラリのマニフェストから名前を書き出す",
            },
        ),
        para(
            site,
            L {
                en: "A component library such as Shoelace, Web Awesome, or Material Web ships a `custom-elements.json`, the Custom Elements Manifest, which lists each element's tag, attributes, and events. `cargo hypersig elements` reads it and prints a Rust module, which you save as a file of your crate and declare with `mod`. `cargo install --locked hypersig_cli` installs the command.",
                ja: "Shoelace、Web Awesome、Material Web のようなコンポーネントのライブラリは、要素ごとのタグ、属性、イベントを並べた `custom-elements.json`（Custom Elements Manifest）を同梱しています。`cargo hypersig elements` はこれを読んで Rust のモジュールを出力します。出力をクレートのファイルとして保存し、`mod` で宣言してください。コマンドは `cargo install --locked hypersig_cli` でインストールできます。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "cargo hypersig elements node_modules/@shoelace-style/shoelace/dist/custom-elements.json > src/shoelace.rs",
        ),
        para(
            site,
            L {
                en: "Each element gets a function that creates it, a module `attr` with a constant per attribute, and a module `event` with a `CustomEvent` per event. A `detail` that the manifest types as a primitive, a union of string literals, or an object literal of them becomes a Rust type, and the object literal a struct that derives `Signals`, so a handler reads its fields with `Expr::at`. Most events in these manifests give no type, and their `detail` is `()`. An attribute that `bind_attr` refuses, such as `href`, is left out with a comment that names the method that sets it. Here is the part of the module for Shoelace's rating:",
                ja: "要素ごとに、要素を作る関数、属性ごとの定数を持つモジュール `attr`、イベントごとの `CustomEvent` を持つモジュール `event` が出力されます。マニフェストが `detail` をプリミティブ型、文字列リテラルのユニオン、それらからなるオブジェクトリテラルとして書いていれば、`detail` は Rust の型になります。オブジェクトリテラルは `Signals` を derive した構造体になり、ハンドラは `Expr::at` でそのフィールドを読めます。これらのマニフェストでは型のないイベントが大半で、その `detail` は `()` になります。`href` のように `bind_attr` が受け付けない属性は出力されず、代わりに設定するメソッドをコメントで示します。Shoelace の評価の要素については、次の部分が出力されます。",
            },
        ),
        show_file(Snippet::ElementsModule, "src/shoelace.rs"),
        para(
            site,
            L {
                en: "The page names the element, its attribute, and its event through the module, so a misspelled name is a compile error:",
                ja: "ページは要素、属性、イベントをモジュールを通して指定するので、名前のつづりを間違えるとコンパイルエラーになります。",
            },
        ),
        show(Snippet::ElementsPage),
        heading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査する範囲",
            },
        ),
        para(
            site,
            L {
                en: "What the page does with the `detail` is checked: putting it into a signal of another type is E0277, with the message that `Expr<Review, i32>` is not an expression of type `String`:",
                ja: "ページが `detail` をどう使うかは検査されます。型の違うシグナルに入れると E0277 になり、`Expr<Review, i32>` は `String` 型の式ではない、というメッセージが出ます。",
            },
        ),
        show(Snippet::CustomEventType),
        para(
            site,
            L {
                en: "With the module `cargo hypersig elements` writes, a tag, an attribute, or an event the manifest does not list is a compile error too. The manifest describes the script, so nothing checks it against the script: an event the script never dispatches runs nothing, and a field the `detail` lacks reads as `null`. A `CustomEvent` you declare by hand, as the rating above does, is checked against nothing but your own declaration. A [`SafeUrl`](api:hypersig/struct.SafeUrl.html) inside a `detail` is not checked on its way in either, so the browser checks the `href`, `src`, or `poster` a signal gives an element, and removes the attribute where the URL's scheme is one a `SafeUrl` refuses.",
                ja: "`cargo hypersig elements` が書いたモジュールを使えば、マニフェストにないタグ、属性、イベントもコンパイルエラーになります。マニフェストはスクリプトの説明なので、スクリプトと突き合わせる検査はありません。スクリプトが投げないイベントでは何も実行されず、`detail` にないフィールドを読むと `null` になります。上の評価のように手で宣言した `CustomEvent` は、その宣言のほかに照らし合わせるものがありません。`detail` に入った [`SafeUrl`](api:hypersig/struct.SafeUrl.html) も入ってくるときには検査されません。そのため、シグナルが要素の `href`、`src`、`poster` を設定するときにブラウザが検査し、`SafeUrl` が受け付けないスキームの URL なら属性を取り除きます。",
            },
        ),
        heading(
            site,
            L {
                en: "A library that draws into an element",
                ja: "要素の中に描くライブラリ",
            },
        ),
        para(
            site,
            L {
                en: "A library that is not a custom element, such as Leaflet, draws into the element it is given and adds children, classes, and inline styles to it. A server patch that reaches the element would set it back to what the server rendered. Mark the element with [`El::foreign`](api:hypersig/html/struct.El.html#method.foreign), `fn foreign(self, node_ref: NodeRef<S>) -> Self`, and a patch keeps it as it is while its bindings keep running. Give the element an [`Id`](api:hypersig/html/struct.Id.html) as well, and the script finds it with `document.getElementById(\"map\")`. A patch keeps that id, and among its siblings it finds the element by it, so an element added before it does not make the map start over. Without an id, a patch finds the element by its node ref's name, the constant's full path, such as `office::MAP`. That path changes when the module or the constant is renamed, so a script should not look the element up by it.",
                ja: "Leaflet のようにカスタム要素ではないライブラリは、渡された要素の中に描き、子要素やクラス、インラインスタイルを追加します。サーバーのパッチがその要素に届くと、サーバーが描いた形に戻してしまいます。[`El::foreign`](api:hypersig/html/struct.El.html#method.foreign)（`fn foreign(self, node_ref: NodeRef<S>) -> Self`）を付けると、パッチはその要素をそのまま残し、要素の束縛は動き続けます。要素には [`Id`](api:hypersig/html/struct.Id.html) も付けてください。スクリプトは `document.getElementById(\"map\")` で要素を見つけます。パッチはその id を保持し、兄弟要素の中でもその id で要素を見つけるので、前に要素が増えても地図は作り直されません。id がない場合、パッチはノード参照の名前、つまり `office::MAP` のような定数の完全なパスで要素を見つけます。このパスはモジュールや定数の名前を変えると変わるので、スクリプトからはこの値で要素を探さないでください。",
            },
        ),
        show(Snippet::Foreign),
        para(
            site,
            L {
                en: "The page renders the element's children and static attributes once. A list item or a condition that holds the element is built anew with it when the browser renders the item or the condition again, so the element the script drew into is replaced by a new one. A `foreign` element takes the node ref in place of `El::node_ref`, and the [node ref's statements](learn/node-refs/) act on it as on any other.",
                ja: "要素の子要素と静的な属性は、ページが 1 回だけ描画します。要素を含むリストの項目や条件をブラウザが描画し直すと、要素も新しく作られるので、スクリプトが描いた要素は新しい要素に置き換わります。`foreign` の要素は `El::node_ref` の代わりにこのメソッドでノード参照を受け取り、[ノード参照の文](learn/node-refs/) はほかの要素と同じように使えます。",
            },
        ),
        para(
            site,
            L {
                en: "Next, [Values the types do not check](learn/unchecked-values/) lists the other places where a value reaches the browser unchecked.",
                ja: "次の [型で検査しない値](learn/unchecked-values/) では値が検査されずにブラウザに届くほかの箇所をまとめます。",
            },
        ),
    ])
}
