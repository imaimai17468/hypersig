//! The page on running a JavaScript library, such as a map or an editor,
//! inside a page.

use super::snippet::{Snippet, show, show_file};
use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn javascript(site: Site) -> Part {
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
                ja: "ページとカスタム要素は 3 つのメソッドでつながります。`Page::script` は要素を定義するスクリプトを読み込み、`bind_attr` は要素の属性をシグナルの値に合わせ、`El::on_custom` は要素が投げたイベントで文を実行します。イベントの `detail` は型付きの式として文に渡ります。イベントは、型引数を `detail` の型にした [`CustomEvent`](api:hypersig/html/struct.CustomEvent.html) として 1 回宣言します。",
            },
        ),
        para(
            site,
            L {
                en: "The rating below is a custom element written in 34 lines of JavaScript. A click on a star dispatches `rate`, the statement puts its `detail` into the page's signal, and the signal goes back into the element's `value` attribute. The clear button changes only the signal, and the stars follow.",
                ja: "下の評価は、34 行の JavaScript で書いたカスタム要素です。星をクリックすると要素が `rate` を投げ、文がその `detail` をページのシグナルに入れ、シグナルが要素の `value` 属性に戻ります。クリアのボタンはシグナルだけを変え、星の表示がそれに合わせて変わります。",
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
                en: "`Page::script`, `Page::stylesheet`, and `Page::icon` write a path into the page and serve nothing, so the router needs a service at each path. tower-http serves files from disk: `ServeFile` one file at one path, and `ServeDir` each file under a directory. Add it with its `fs` feature:",
                ja: "`Page::script`、`Page::stylesheet`、`Page::icon` はページにパスを書くだけでファイルは配信しないので、ルーターにはそれぞれのパスで配信するサービスが要ります。ディスクのファイルは tower-http で配信します。`ServeFile` は 1 つのファイルを 1 つのパスで、`ServeDir` はディレクトリの下のファイルをそれぞれ配信します。`fs` フィーチャーを有効にして追加してください。",
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
                en: "The paths given to `ServeFile` and `ServeDir` are read from the directory the server was started in, so the server above expects `star-rating.js` and `assets/` there.",
                ja: "`ServeFile` と `ServeDir` に渡したパスは、サーバーを起動したディレクトリから読みます。上のサーバーは、そこに `star-rating.js` と `assets/` があるものとして動きます。",
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
                en: "What the page does with the `detail` is checked: putting it into a signal of another type is a compile error. With the module `cargo hypersig elements` writes, a tag, an attribute, or an event the manifest does not list is a compile error too. The manifest describes the script, so nothing checks it against the script: an event the script never dispatches runs nothing, and a field the `detail` lacks reads as `null`. A `CustomEvent` you declare by hand, as the rating above does, is checked against nothing but your own declaration.",
                ja: "ページが `detail` をどう使うかは検査されます。型の違うシグナルに入れると、コンパイルエラーになります。`cargo hypersig elements` が書いたモジュールを使えば、マニフェストにないタグ、属性、イベントもコンパイルエラーになります。マニフェストはスクリプトの説明なので、スクリプトと突き合わせる検査はありません。スクリプトが投げないイベントでは何も実行されず、`detail` にないフィールドを読むと `null` になります。上の評価のように手で宣言した `CustomEvent` は、その宣言のほかに照らし合わせるものがありません。",
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
                en: "A library that is not a custom element, such as Leaflet, draws into the element it is given and adds children, classes, and inline styles to it. A server patch that reaches the element would set it back to what the server rendered. Mark the element with `El::foreign`, and a patch keeps it as it is while its bindings keep running. Among its siblings a patch finds it by its node ref's name, so an element added before it does not make the map start over.",
                ja: "Leaflet のようにカスタム要素ではないライブラリは、渡された要素の中に描き、子要素やクラス、インラインスタイルを追加します。サーバーのパッチがその要素に届くと、サーバーが描いた形に戻してしまいます。`El::foreign` を付けると、パッチはその要素をそのまま残し、要素の束縛は動き続けます。兄弟要素の中では node ref の名前でその要素を見つけるので、前に要素が増えても地図は作り直されません。",
            },
        ),
        show(Snippet::Foreign),
    ])
}
