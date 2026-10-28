//! The page on building the page.

use super::snippet::{Snippet, show};
use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn describing_the_page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::DescribingThePage).title(site)),
        lead(site, Doc::Learn(Learn::DescribingThePage).leads()),
        heading(
            site,
            L {
                en: "Elements",
                ja: "要素",
            },
        ),
        para(
            site,
            L {
                en: "The [`html`](api:hypersig/html/index.html) module has one function per common tag, such as `div()`, `p()`, and `button()`, and `el(hypersig::tag!(\"tag\"))` for any other, which checks the tag at compile time and refuses `script`, `style`, `base`, and `link`. Each returns an `El<S>`, where `S` is the page's signal struct, and its methods return the element, so a tree is one expression.",
                ja: "[`html`](api:hypersig/html/index.html) モジュールには `div()`、`p()`、`button()` のようによく使うタグごとの関数と、それ以外のタグに使う `el(hypersig::tag!(\"tag\"))` があります。`tag!` はタグをコンパイル時に検査し、`script`、`style`、`base`、`link` を拒否します。どれも `El<S>` を返します。`S` はページのシグナル構造体です。`El` のメソッドは要素そのものを返すので、木全体を 1 つの式で書けます。",
            },
        ),
        show(Snippet::Elements),
        para(
            site,
            L {
                en: "`div::<Profile>()` names the page's signal struct because nothing else in this code tells the compiler which page the element belongs to. An element passed to `Page::new(...).child(...)`, or to a function that takes an `El<Profile>`, gets its `S` from there, which is why the Quick start writes `div()`. The children of an element take its `S` the same way, so only the outermost element of a tree written on its own needs the `::<Profile>`.",
                ja: "`div::<Profile>()` とページのシグナル構造体を書いているのは、このコードにはこの要素がどのページのものかをコンパイラに伝えるものがほかにないからです。`Page::new(...).child(...)` や `El<Profile>` を受け取る関数に渡す要素は、渡し先から `S` が推論されます。クイックスタートが `div()` と書けるのはそのためです。子要素も親から同じように `S` を受け取るので、単独で書いた木でも `::<Profile>` が必要なのはいちばん外側の要素だけです。",
            },
        ),
        para(
            site,
            L {
                en: "`text` appends static text, and `child` appends an element or a string. `href` points a link at a page from a `Route`, which [Pages and links](learn/pages/) covers. `attr` sets a plain attribute, such as `.attr(attr::PLACEHOLDER, \"Name\")`, and takes the name as an [`Attr`](api:hypersig/html/struct.Attr.html). The module [`html::attr`](api:hypersig/html/attr/index.html) holds one for each attribute of HTML, SVG, and ARIA, so a misspelled name stops the build. A name outside those lists, such as `data-lat` or an attribute of a custom element, is written `hypersig::attr!(\"data-lat\")`, which checks it at compile time. Neither has an `Attr` for these names, ignoring case, so `attr`, `bind_attr`, and `attr_when` cannot write them:",
                ja: "`text` は固定のテキストを、`child` は要素か文字列を末尾に追加します。`href` は `Route` からページへのリンクを張ります。これは [ページとリンク](learn/pages/) で説明します。`attr` は `.attr(attr::PLACEHOLDER, \"Name\")` のように普通の属性を設定し、名前を [`Attr`](api:hypersig/html/struct.Attr.html) で受け取ります。HTML、SVG、ARIA の属性はモジュール [`html::attr`](api:hypersig/html/attr/index.html) に 1 つずつ定数があるので、名前を書き間違えるとビルドが止まります。`data-lat` やカスタム要素の属性のように一覧にない名前は `hypersig::attr!(\"data-lat\")` と書き、コンパイル時に検査されます。次の名前にはどちらの方法でも `Attr` を作れないので、`attr`、`bind_attr`、`attr_when` で書けません。大文字と小文字は区別しません。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`id` and `class`, which `El::id` and `El::class` set. `El::id` takes an [`Id`](api:hypersig/html/struct.Id.html), which you declare once, such as `const EMAIL: Id = Id::new(\"email\")`.",
                    ja: "`id` と `class`。`El::id` と `El::class` で設定します。`El::id` は、`const EMAIL: Id = Id::new(\"email\")` のように 1 度だけ宣言した [`Id`](api:hypersig/html/struct.Id.html) を受け取ります。",
                },
                L {
                    en: "An attribute whose value names another element by its id, such as `for`, `aria-describedby`, `aria-controls`, and `commandfor`. Methods such as `label_for(&EMAIL)` and `aria_describedby(&[&HINT])` set them from the same `Id` the element carries, so a reference cannot be spelled apart from its target.",
                    ja: "`for`、`aria-describedby`、`aria-controls`、`commandfor` のように、値がほかの要素を id で指す属性。`label_for(&EMAIL)` や `aria_describedby(&[&HINT])` などのメソッドが、指す先の要素と同じ `Id` から設定するので、参照と参照先の綴りが食い違うことはありません。",
                },
                L {
                    en: "`role`, which `El::role` sets from a [`Role`](api:hypersig/html/enum.Role.html), so a misspelled role stops the build too.",
                    ja: "`role`。`El::role` が [`Role`](api:hypersig/html/enum.Role.html) から設定するので、ロールの書き間違いもビルドで止まります。",
                },
                L {
                    en: "A name starting with `data-hs-`, which only the typed methods write, because the runtime reads those attributes.",
                    ja: "`data-hs-` で始まる名前。ランタイムがこの属性を読むので、型付きのメソッドだけが書きます。",
                },
                L {
                    en: "`href`, `src`, `poster`, and `srcset`, which `El::href`, `El::src`, `El::poster`, and `El::srcset` set from a `Link` or a [`SafeUrl`](api:hypersig/struct.SafeUrl.html), a URL whose scheme cannot run a script. [When bindings and statements are not enough](learn/escape-hatches/) covers `SafeUrl`.",
                    ja: "`href`、`src`、`poster`、`srcset`。`El::href`、`El::src`、`El::poster`、`El::srcset` が、`Link` か、スクリプトを実行できないスキームの URL を持つ [`SafeUrl`](api:hypersig/struct.SafeUrl.html) から設定します。`SafeUrl` は [束縛と文で足りないとき](learn/escape-hatches/) で説明します。",
                },
                L {
                    en: "A name starting with `on`, such as `onclick`. The `on_*` methods, such as `on_click`, attach a statement in its place.",
                    ja: "`onclick` のように `on` で始まる名前。代わりに `on_click` などの `on_*` のメソッドで文を付けます。",
                },
                L {
                    en: "`srcdoc`, `action`, `formaction`, `xlink:href`, `data`, `http-equiv`, `ping`, `imagesrcset`, `background`, `manifest`, `codebase`, `archive`, `classid`, and `longdesc`, which load a URL or a document and have no method of their own. A form sends its values through an action call, as [Forms](learn/forms/) shows.",
                    ja: "`srcdoc`、`action`、`formaction`、`xlink:href`、`data`、`http-equiv`、`ping`、`imagesrcset`、`background`、`manifest`、`codebase`、`archive`、`classid`、`longdesc`。どれも URL か文書を読み込む属性で、設定するメソッドはありません。フォームの値は [フォーム](learn/forms/) のとおりアクションの呼び出しで送ります。",
                },
                L {
                    en: "`style`, because `El::css` and `El::bind_style` write the element's CSS.",
                    ja: "`style`。要素の CSS は `El::css` と `El::bind_style` で書きます。",
                },
                L {
                    en: "`attributeName`, whose value `href` would make an SVG animation write into a link.",
                    ja: "`attributeName`。値が `href` だと、SVG のアニメーションがリンクに書き込むことになるからです。",
                },
                L {
                    en: "A name holding a character other than a letter, a digit, `-`, `_`, `.`, and `:`.",
                    ja: "英字、数字、`-`、`_`、`.`、`:` 以外の文字を含む名前。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "The page",
                ja: "ページ",
            },
        ),
        para(
            site,
            L {
                en: "[`Page`](api:hypersig/struct.Page.html) wraps the tree in a full document. `Page::new` takes the title and the initial values of the signals, `child` appends to `<body>`, and `render` returns the HTML a handler sends. The document loads the runtime from `RUNTIME_PATH`, a path under `/_hypersig/` that names a hash of the file, and `RouterExt::hypersig_runtime` serves it there.",
                ja: "[`Page`](api:hypersig/struct.Page.html) は木を包んで完全な HTML 文書にします。`Page::new` はタイトルとシグナルの初期値を受け取り、`child` は `<body>` に要素を追加し、`render` はハンドラが送る HTML を返します。文書は `RUNTIME_PATH` からランタイムを読み込みます。このパスは `/_hypersig/` の下にあり、ファイルのハッシュを名前に含みます。`RouterExt::hypersig_runtime` がそのパスでファイルを配信します。",
            },
        ),
        heading(
            site,
            L {
                en: "Values that follow the signals",
                ja: "シグナルに追従する値",
            },
        ),
        para(
            site,
            L {
                en: "An element can show a value that changes in the browser. Each binding takes an expression over the signals and is rendered with the initial values first, so the HTML shows what the runtime would compute.",
                ja: "ブラウザで変わる値を要素に表示できます。束縛はそれぞれシグナルを使った式を受け取り、最初は初期値で描画されるので、HTML にはランタイムが計算するのと同じ値が入っています。",
            },
        ),
        show(Snippet::Bindings),
        bullets(
            site,
            &[
                L {
                    en: "`bind_text` replaces the element's text.",
                    ja: "`bind_text` は要素のテキストを置き換えます。",
                },
                L {
                    en: "`bind_attr` keeps an attribute equal to a string expression, and `attr_when` adds a boolean attribute such as `disabled` while a condition holds.",
                    ja: "`bind_attr` は属性を文字列の式の値に合わせ続けます。`attr_when` は、条件が成り立つ間だけ `disabled` のような真偽値の属性を付けます。",
                },
                L {
                    en: "`show` hides the element while a condition is false. `when(cond, el)` removes it instead, so each time it returns its inputs start empty and its `on_load`, the statement that runs when an element enters the document, runs again.",
                    ja: "`show` は条件が偽の間、要素を隠します。`when(cond, el)` は要素を取り除くので、要素が戻るたびに入力欄は空から始まり、`on_load`（要素が文書に入ったときに実行する文）がもう一度実行されます。",
                },
                L {
                    en: "`class_when` and `css_when` add a class or a style while a condition holds. [Styling](learn/styling/) covers them.",
                    ja: "`class_when` と `css_when` は、条件が成り立つ間だけクラスやスタイルを付けます。詳しくは [スタイル](learn/styling/) で説明します。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The demo binds a text field to `name` and shows one of two paragraphs depending on whether it is empty.",
                ja: "このデモはテキスト欄を `name` に束縛し、空かどうかで 2 つの段落のどちらかを表示します。",
            },
        ),
        demo::show(site, Demo::Greeting),
        heading(
            site,
            L {
                en: "Components",
                ja: "コンポーネント",
            },
        ),
        para(
            site,
            L {
                en: "A component is a Rust function that returns an `El<S>`. Make it generic over `S` and take the signals it reads as arguments, and it works on any page.",
                ja: "コンポーネントは `El<S>` を返す Rust の関数です。`S` についてジェネリックにし、読むシグナルを引数で受け取れば、どのページでも使えます。",
            },
        ),
        show(Snippet::Component),
        para(
            site,
            L {
                en: "`hypersig_ui` is a set of such functions styled after shadcn/ui. The demos on this site use its buttons and text fields.",
                ja: "`hypersig_ui` はこうした関数を集めたクレートで、見た目は shadcn/ui に倣っています。このサイトのデモはそのボタンとテキスト欄を使っています。",
            },
        ),
    ])
}
