//! The page on what hypersig checks so that a page and its actions resist
//! the attacks a visitor can make, and what it leaves to the application.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Security).title(site)),
        lead(site, Doc::Learn(Learn::Security).leads()),
        output(site),
        escaping(site),
        attributes(site),
        urls(site),
        requests(site),
        framing(site),
        server_code(site),
        application(site),
        nonce(site),
        tested(site),
        next(site),
    ])
}

/// What the HTML builder, the stylesheet, and the runtime refuse to write.
fn output(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the page writes",
                ja: "ページに書き出すもの",
            },
        ),
        table_of(
            site,
            L {
                en: &["What could go wrong", "What hypersig does"],
                ja: &["起こりうること", "hypersig がすること"],
            },
            L {
                en: &[
                    &[
                        "A value becomes markup or a script",
                        "Text and attribute values are escaped when the server writes them, and the runtime sets text with `textContent`. No public method inserts HTML that the builder did not write.",
                    ],
                    &[
                        "A link runs a script, such as `javascript:`",
                        "`El::href` and `El::src` take a `Link` to a route or a `SafeUrl`, which holds only `http:`, `https:`, `mailto:`, or a relative URL. `SafeUrl::parse` checks a URL a visitor typed, and a `SafeUrl` field of the signals checks it again when it deserializes. The runtime checks a `SafeUrl` that the browser builds, such as one in the `detail` of a custom event, when a signal writes it to `href`, `src`, or `poster`, and removes the attribute where the scheme is refused. `attr`, `bind_attr`, and `attr_when` take the name as an `Attr`, and no `Attr` names `href`, `src`, `poster`, `srcset`, `style`, an `on*` handler, `srcdoc`, `action`, `formaction`, `xlink:href`, `data`, `http-equiv`, `attributeName`, `ping`, `imagesrcset`, `background`, `manifest`, `codebase`, `archive`, `classid`, or `longdesc`, in any case, or a name holding anything but letters, digits, `-`, `_`, `.`, and `:`. The constants of `html::attr` leave those names out, and `attr!` refuses them at compile time. `el` takes its tag as a `Tag` that `tag!` checks at compile time, which holds no space, quote, or `=` that would start an attribute, and is never `script`, `style`, `base`, or `link`.",
                    ],
                    &[
                        "A URL sends the browser to another host",
                        "A `Url` comes only from `Route::url`. A leading or doubled `/` of a `Rest` value is written `%2F`, and `/.` goes in front of a path that an empty first segment would start with `//`, so no URL or link a route writes names another host.",
                    ],
                    &[
                        "A style value adds declarations or rules",
                        "A CSS string escapes every control character, `\"`, `\\`, and `<`. A bound style value that could end its declaration is left for the runtime, whose `setProperty` refuses it. `raw` refuses an escape, a comment, `;`, `{`, `}`, `<`, and an open quote or parenthesis, and `Stylesheet::new` refuses `</style` at compile time.",
                    ],
                    &[
                        "The runtime runs code from the page",
                        "The runtime evaluates expressions from a fixed table of operators. It calls neither `eval` nor `new Function`, so a page served with `Page::csp_nonce` runs under a Content-Security-Policy without `unsafe-eval` or `unsafe-inline`.",
                    ],
                    &[
                        "The browser reads a response as a script or a page its `Content-Type` does not name",
                        "Pages, the runtime, and the answers of actions, streams, and uploads, refusals included, are served with `X-Content-Type-Options: nosniff`.",
                    ],
                    &[
                        "A cache shows one visitor's page to another",
                        "A page holds the visitor's signals, so it is served with `Cache-Control: private, no-cache`. No shared cache keeps it, and the browser asks the server again before it shows a kept copy. `no-store` is left out because it also keeps the page out of the back-forward cache. A `Cache-Control` the handler sends replaces it.",
                    ],
                ],
                ja: &[
                    &[
                        "値がマークアップやスクリプトになる",
                        "サーバーはテキストと属性の値をエスケープして書き、ランタイムはテキストを `textContent` で設定します。ビルダーが書いていない HTML を差し込む公開のメソッドはありません。",
                    ],
                    &[
                        "`javascript:` のようなリンクがスクリプトを動かす",
                        "`El::href` と `El::src` はルートへの `Link` か `SafeUrl` を受け取ります。`SafeUrl` が持てるのは `http:`、`https:`、`mailto:` か相対 URL だけです。訪問者が入力した URL は `SafeUrl::parse` で確かめ、シグナルの `SafeUrl` のフィールドはデシリアライズのときにもう一度確かめます。カスタムイベントの `detail` のようにブラウザで作られた `SafeUrl` は、シグナルが `href`、`src`、`poster` に書くときにランタイムが確かめ、拒否するスキームなら属性を取り除きます。`attr`、`bind_attr`、`attr_when` は名前を `Attr` で受け取ります。`href`、`src`、`poster`、`srcset`、`style`、`on*` のハンドラ、`srcdoc`、`action`、`formaction`、`xlink:href`、`data`、`http-equiv`、`attributeName`、`ping`、`imagesrcset`、`background`、`manifest`、`codebase`、`archive`、`classid`、`longdesc` と、英数字、`-`、`_`、`.`、`:` 以外を含む名前には、大文字と小文字にかかわらず `Attr` がありません。`html::attr` の定数はこれらの名前を除いており、`attr!` はコンパイル時に拒否します。`el` はタグを `tag!` がコンパイル時に検査する `Tag` で受け取ります。`Tag` は属性を始める空白、引用符、`=` を含まず、`script`、`style`、`base`、`link` にもなりません。",
                    ],
                    &[
                        "URL がブラウザを別のホストへ送る",
                        "`Url` は `Route::url` からしか作れません。`Rest` の値の先頭の `/` と続いた `/` は `%2F` と書きます。最初のセグメントが空でパスが `//` で始まるときは前に `/.` を付けるので、ルートが書く URL とリンクが別のホストを指すことはありません。",
                    ],
                    &[
                        "スタイルの値が宣言や規則を追加する",
                        "CSS の文字列は制御文字、`\"`、`\\`、`<` をすべてエスケープします。束縛したスタイルの値のうち宣言を終わらせうるものはランタイムに任せ、ランタイムの `setProperty` がそれを拒否します。`raw` はエスケープ、コメント、`;`、`{`、`}`、`<` と、閉じていない引用符や括弧を拒否します。`Stylesheet::new` は `</style` をコンパイル時に拒否します。",
                    ],
                    &[
                        "ランタイムがページにあるコードを動かす",
                        "ランタイムは決まった演算子の表だけで式を評価します。`eval` も `new Function` も呼ばないので、`Page::csp_nonce` を付けたページは、`unsafe-eval` も `unsafe-inline` もない Content-Security-Policy のもとで動きます。",
                    ],
                    &[
                        "ブラウザがレスポンスを `Content-Type` にないスクリプトやページとして読む",
                        "ページ、ランタイム、アクションとストリームとアップロードの応答（拒否も含む）には `X-Content-Type-Options: nosniff` を付けて配信します。",
                    ],
                    &[
                        "キャッシュがある訪問者のページを別の訪問者に見せる",
                        "ページは訪問者のシグナルを持つので、`Cache-Control: private, no-cache` を付けて配信します。共有のキャッシュはページを保存せず、ブラウザは保存したページを表示する前にサーバーへ確かめます。`no-store` はページを戻る・進むのキャッシュからも外すので使いません。ハンドラが `Cache-Control` を送ると、そちらに置き換わります。",
                    ],
                ],
            },
        ),
    ])
}

/// The escaping of text and attribute values, shown as the HTML it writes.
fn escaping(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Escaped text and attribute values",
                ja: "エスケープするテキストと属性の値",
            },
        ),
        para(
            site,
            L {
                en: "`El::text` and `El::attr` take the value as `impl Into<String>`. The server writes `<`, `>`, and `&` as character references, and in an attribute value both quotes too, so a value a visitor stored shows as text, opens no tag, and does not end its attribute:",
                ja: "`El::text` と `El::attr` は値を `impl Into<String>` で受け取ります。サーバーは `<`、`>`、`&` を文字参照で書き、属性の値では 2 種類の引用符も文字参照にします。そのため訪問者が保存した値は文字として表示され、タグを開かず、属性を終わらせることもありません。",
            },
        ),
        show(Snippet::Escaping),
        para(
            site,
            L {
                en: "A bound text is set in the browser with `textContent`, which the browser never reads as markup.",
                ja: "束縛したテキストはブラウザで `textContent` を使って設定します。ブラウザは `textContent` の値をマークアップとして読みません。",
            },
        ),
    ])
}

/// `Attr`, `attr!`, and the names they refuse.
fn attributes(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Attribute names",
                ja: "属性名",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn attr(self, name: Attr, value: impl Into<String>) -> Self",
        ),
        para(
            site,
            L {
                en: "[`El::attr`](api:hypersig/html/struct.El.html#method.attr) takes the name as an [`Attr`](api:hypersig/html/struct.Attr.html): a constant of [`html::attr`](api:hypersig/html/attr/index.html), or [`attr!`](api:hypersig/macro.attr.html) for a name the list lacks, such as a `data-` attribute. `attr!` checks the name at compile time and refuses each name the table above lists:",
                ja: "[`El::attr`](api:hypersig/html/struct.El.html#method.attr) は名前を [`Attr`](api:hypersig/html/struct.Attr.html) で受け取ります。[`html::attr`](api:hypersig/html/attr/index.html) の定数を使い、`data-` 属性のように一覧にない名前は [`attr!`](api:hypersig/macro.attr.html) で作ります。`attr!` はコンパイル時に名前を検査し、上の表に挙げた名前をすべて拒否します。",
            },
        ),
        show(Snippet::AttrEventHandler),
        para(
            site,
            L {
                en: "A handler goes on with a method such as `on_click`, which takes a statement built in Rust, and a URL goes on with `href` or `src`, below.",
                ja: "ハンドラは `on_click` のようなメソッドで付けます。これらは Rust で組み立てた文を受け取ります。URL は下の節の `href` か `src` で付けます。",
            },
        ),
    ])
}

/// `href`, `src`, and `SafeUrl`.
fn urls(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "URLs in href and src",
                ja: "href と src の URL",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn href(self, url: impl IntoHref<S>) -> Self\n\n// impl SafeUrl\npub const fn new(url: &'static str) -> Self\npub fn parse(url: impl Into<String>) -> Result<Self, UnsafeUrl>",
        ),
        para(
            site,
            L {
                en: "[`El::href`](api:hypersig/html/struct.El.html#method.href) and `El::src` take a `Link` or a `Url` from a route, a [`SafeUrl`](api:hypersig/struct.SafeUrl.html), or a signal or expression of `SafeUrl`. A `String` signal is none of these, so a URL a visitor typed reaches an `href` only after [`SafeUrl::parse`](api:hypersig/struct.SafeUrl.html#method.parse) has checked it:",
                ja: "[`El::href`](api:hypersig/html/struct.El.html#method.href) と `El::src` は、ルートの `Link` か `Url`、[`SafeUrl`](api:hypersig/struct.SafeUrl.html)、`SafeUrl` のシグナルか式を受け取ります。`String` のシグナルはどれにも当たらないので、訪問者が入力した URL は、[`SafeUrl::parse`](api:hypersig/struct.SafeUrl.html#method.parse) で確かめてからでないと `href` に渡せません。",
            },
        ),
        show(Snippet::HrefFromString),
        para(
            site,
            L {
                en: "`SafeUrl::new` in a `const` refuses `javascript:alert(1)` at compile time with E0080, and `SafeUrl::parse` returns an `UnsafeUrl` for it. [Values the types do not check](learn/unchecked-values/) shows `SafeUrl` in a page.",
                ja: "`const` の中の `SafeUrl::new` は `javascript:alert(1)` を E0080 でコンパイル時に拒否し、`SafeUrl::parse` はそれに `UnsafeUrl` を返します。ページでの `SafeUrl` の使い方は [型で検査しない値](learn/unchecked-values/) にあります。",
            },
        ),
    ])
}

/// What an action, a stream, or an upload refuses before its handler runs.
fn requests(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What a request must carry",
                ja: "リクエストが満たすこと",
            },
        ),
        para(
            site,
            L {
                en: "An action, a stream, or an upload answers a request with an error before its handler runs, so a page on another site cannot make a visitor's browser call it with the visitor's cookie.",
                ja: "アクション、ストリーム、アップロードは、次のリクエストにハンドラが動く前にエラーを返します。そのため、別のサイトのページが、訪問者のブラウザに訪問者の Cookie を付けて呼ばせることはできません。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Request", "Answer"],
                ja: &["リクエスト", "応答"],
            },
            L {
                en: &[
                    &[
                        "`Sec-Fetch-Site` is not `same-origin`, `same-site` included",
                        "403",
                    ],
                    &[
                        "A POST whose body is not `application/json`, which a form cannot send and a script on another site sends only after a preflight this server does not answer",
                        "415",
                    ],
                    &[
                        "A GET without the `hypersig-call` header the runtime adds, as from an `<img>`, a link, an `EventSource`, or a `no-cors` `fetch`, none of which can add a header",
                        "403",
                    ],
                    &[
                        "An upload without the `hypersig-upload` header, which needs a preflight",
                        "403",
                    ],
                    &[
                        "An upload longer than the `max_bytes` of its line, 8 MiB unless set",
                        "413",
                    ],
                    &[
                        "Arguments that do not deserialize into the action's argument type",
                        "400",
                    ],
                ],
                ja: &[
                    &[
                        "`Sec-Fetch-Site` が `same-origin` でない（`same-site` も含む）",
                        "403",
                    ],
                    &[
                        "本文が `application/json` でない POST。フォームはこの形式を送れず、別のサイトのスクリプトはプリフライトのあとでしか送れず、このサーバーはプリフライトに応答しません",
                        "415",
                    ],
                    &[
                        "ランタイムが付ける `hypersig-call` のヘッダーのない GET。`<img>`、リンク、`EventSource`、`no-cors` の `fetch` は、どれもこのヘッダーを付けられません",
                        "403",
                    ],
                    &[
                        "プリフライトが必要な `hypersig-upload` のヘッダーのないアップロード",
                        "403",
                    ],
                    &[
                        "行の `max_bytes`（指定しなければ 8 MiB）より長いアップロード",
                        "413",
                    ],
                    &["アクションの引数の型にデシリアライズできない引数", "400"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The body of each refusal is one fixed sentence. What the parser said about the arguments or a page's query string names the fields of the type and quotes what the visitor sent, so it stays in the `source` of the `RequestError` or the `Unserved` and is not sent back.",
                ja: "拒否の本文はどれも決まった一文です。引数やページのクエリ文字列についてパーサーが出すメッセージには、型のフィールドと訪問者が送った値が含まれます。そのためこのメッセージは送り返さず、`RequestError` や `Unserved` の `source` に保持します。",
            },
        ),
        para(
            site,
            L {
                en: "A script on another site that adds `hypersig-call` to its `fetch` makes the browser send a preflight first, which this server does not answer, so the GET is not sent, in a browser without `Sec-Fetch-Site` too. A GET action should still change nothing, because the runtime aborts a GET once the element that sent it leaves the page, and the server may then stop its handler at an `.await`.",
                ja: "別のサイトのスクリプトが `fetch` に `hypersig-call` を付けると、ブラウザは先にプリフライトを送ります。このサーバーはプリフライトに応答しないので、`Sec-Fetch-Site` を送らないブラウザでも GET は送られません。それでも GET のアクションでは何も変更しないでください。ランタイムは、GET を送った要素がページから外れるとその GET を取り消し、サーバーはそのときハンドラを `.await` の時点で止めることがあるためです。",
            },
        ),
    ])
}

/// Frames, `Page::framing`, and the headers each `Framing` sends.
fn framing(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Frames",
                ja: "枠への表示",
            },
        ),
        para(
            site,
            L {
                en: "A page of another site that shows the page in a frame and leads the visitor to click a button gets past these checks, because the click sends its request from the page's own origin. A page is therefore served with `X-Frame-Options: SAMEORIGIN` and `Content-Security-Policy: frame-ancestors 'self'`, and the browser shows it in a frame only on its own origin. `Page::framing` with `Framing::Origins` adds origins that may frame it, and `Framing::Anywhere` lets any site, for a page that changes nothing for the visitor.",
                ja: "別のサイトのページがこのページを枠に表示し、訪問者にボタンを押させると、上の検査は通ってしまいます。押したときのリクエストはこのページ自身のオリジンから出るためです。そのためページは `X-Frame-Options: SAMEORIGIN` と `Content-Security-Policy: frame-ancestors 'self'` を付けて配信され、ブラウザは同じオリジンの枠にしか表示しません。枠に表示してよいオリジンは `Page::framing` に `Framing::Origins` を渡して追加できます。訪問者のために何も変えないページは、`Framing::Anywhere` を渡すと、どのサイトの枠にも表示できます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn framing(self, framing: Framing) -> Self\n\npub enum Framing {\n    Origins(Vec<Origin>),\n    Anywhere,\n}",
        ),
        table_of(
            site,
            L {
                en: &[
                    "[`Framing`](api:hypersig/enum.Framing.html)",
                    "Headers of the page",
                ],
                ja: &[
                    "[`Framing`](api:hypersig/enum.Framing.html)",
                    "ページのヘッダー",
                ],
            },
            L {
                en: &[
                    &[
                        "Unless [`Page::framing`](api:hypersig/struct.Page.html#method.framing) is called, the same as `Origins(vec![])`",
                        "`X-Frame-Options: SAMEORIGIN` and `Content-Security-Policy: frame-ancestors 'self'`",
                    ],
                    &[
                        "`Origins(vec![PORTAL])`",
                        "`Content-Security-Policy: frame-ancestors 'self' https://portal.example`, and no `X-Frame-Options`",
                    ],
                    &["`Anywhere`", "Neither header"],
                ],
                ja: &[
                    &[
                        "[`Page::framing`](api:hypersig/struct.Page.html#method.framing) を呼ばないとき。`Origins(vec![])` と同じです",
                        "`X-Frame-Options: SAMEORIGIN` と `Content-Security-Policy: frame-ancestors 'self'`",
                    ],
                    &[
                        "`Origins(vec![PORTAL])`",
                        "`Content-Security-Policy: frame-ancestors 'self' https://portal.example`。`X-Frame-Options` は付きません",
                    ],
                    &["`Anywhere`", "どちらも付きません"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "An `Origin` is a constant that `Origin::new` checks at compile time. A `Content-Security-Policy` the handler sends goes out beside `frame-ancestors` as a second header, and an `X-Frame-Options` the handler sends is kept in place of the page's.",
                ja: "`Origin` は `Origin::new` がコンパイル時に検査する定数です。ハンドラが送る `Content-Security-Policy` は `frame-ancestors` とは別の 2 つ目のヘッダーとして送られ、ハンドラが送る `X-Frame-Options` はページのものの代わりにそのまま残ります。",
            },
        ),
    ])
}

/// Why data from the browser cannot choose or run code on the server.
fn server_code(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Data from the browser and code on the server",
                ja: "ブラウザから届くデータとサーバーのコード",
            },
        ),
        para(
            site,
            L {
                en: "Some frameworks have let a request run code on the server: the request names a server function, a module, or a class, and the framework finds it and calls it while it reads the request. In hypersig the request has no way to name code, for these reasons.",
                ja: "フレームワークによっては、リクエストでサーバーのコードを動かせたことがあります。リクエストの中でサーバーの関数、モジュール、クラスの名前を指定し、フレームワークがリクエストを読みながらそれを探して呼ぶ仕組みでした。hypersig では次の理由で、リクエストでコードを指定する手段がありません。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "The router decides which handler runs from the request's path and method, and the routes are fixed when the program starts. A request cannot add a route or pick a handler by name.",
                    ja: "どのハンドラが動くかは、ルーターがリクエストのパスとメソッドから決め、ルートはプログラムの起動時に決まっています。リクエストがルートを追加したり、名前でハンドラを選んだりすることはできません。",
                },
                L {
                    en: "The request carries only the arguments, as JSON. `serde_json` reads them into the argument type the action declared at compile time, and anything that does not fit gets a 400. The JSON has no form for a function, a reference to a server object, or a class to construct, so reading it builds data and calls nothing.",
                    ja: "リクエストが運ぶのは JSON の引数だけです。`serde_json` が、アクションがコンパイル時に宣言した引数の型に読み込み、合わないものには 400 を返します。この JSON には、関数、サーバーのオブジェクトへの参照、組み立てるクラスを書く形がないので、読み込みはデータを作るだけで、何も呼びません。",
                },
                L {
                    en: "`serde_json` stops at 128 levels of nesting, so a deeply nested body is refused before it can exhaust the stack.",
                    ja: "`serde_json` は入れ子が 128 段を超えると読むのをやめるので、深く入れ子にした本文はスタックを使い切る前に拒否されます。",
                },
                L {
                    en: "The server evaluates only the expressions the application's Rust code built, to write the first HTML. It never evaluates an expression or a template that arrived in a request, and it reads no file named by one.",
                    ja: "サーバーが評価するのは、最初の HTML を書くためにアプリの Rust のコードが組み立てた式だけです。リクエストで届いた式やテンプレートを評価することも、リクエストが指したファイルを読むこともありません。",
                },
                L {
                    en: "The crates forbid `unsafe`, so a malformed request cannot corrupt memory in them.",
                    ja: "クレートは `unsafe` を禁じているので、不正なリクエストでクレートのメモリを破壊することはできません。",
                },
            ],
        ),
    ])
}

/// What hypersig leaves to the application.
fn application(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the application checks",
                ja: "アプリが確かめること",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Who the visitor is and what they may do. Every action's URL can be called directly, without loading the page, and the browser can send any value of the argument type, so each handler checks that the visitor may act on the arguments it receives. The `auth` example reads the signed-in member with an extractor in each handler.",
                    ja: "訪問者が誰で、何をしてよいか。アクションの URL はページを読み込まずに直接呼べ、ブラウザは引数の型の値なら何でも送れるので、各ハンドラが、訪問者がその引数を扱ってよいかを確かめてください。`auth` の例はハンドラごとにエクストラクタでサインイン中のメンバーを読みます。",
                },
                L {
                    en: "The values of the signals. The browser holds them and can change them, so a price, a role, or an owner the server relies on is read again on the server.",
                    ja: "シグナルの値。ブラウザが持っており、書き換えられるので、サーバーが頼る価格、役割、持ち主は、サーバーで読み直してください。",
                },
                L {
                    en: "The `Content-Security-Policy` header. `Page::csp_nonce` puts the nonce on the page's elements, and the application sends the header with a nonce of at least 128 random bits per response. The page's `frame-ancestors` goes out as a header of its own beside the application's policy, and the browser enforces both.",
                    ja: "`Content-Security-Policy` のヘッダー。`Page::csp_nonce` はページの要素に nonce を付けます。ヘッダーはアプリが送り、nonce はレスポンスごとに 128 ビット以上の乱数にしてください。ページの `frame-ancestors` はアプリのポリシーとは別のヘッダーで送られ、ブラウザは両方を適用します。",
                },
                L {
                    en: "The host of a URL. A `SafeUrl` may name any host, so a URL a visitor typed suits a link the visitor follows, and not the `src` of an `<iframe>`, which shows that host's page inside this one. An `<iframe>`, an `<embed>`, or a `<frame>` panics when its `src` reads a signal. Nothing in hypersig refuses a visitor's URL passed to one as a fixed `SafeUrl`, so the application refuses it.",
                    ja: "URL のホスト。`SafeUrl` はどのホストでも持てるので、訪問者が入力した URL は訪問者がたどるリンクに使い、`<iframe>` の `src` には使わないでください。`<iframe>` はそのホストのページをこのページの中に表示します。`<iframe>`、`<embed>`、`<frame>` の `src` がシグナルを読むと panic します。訪問者の URL を固定の `SafeUrl` で渡すときは、アプリで拒否してください。",
                },
                L {
                    en: "Ids built from stored data. Rendering panics when two fixed ids are the same, but two identical `String` ids render, and a link or a patch to that id reaches the first of them. Build an id from a value visitors stored, such as a tag name, together with the record's key, so that it stays unique.",
                    ja: "保存したデータから作る id。固定の id が 2 つ同じなら描画はパニックしますが、`String` の id が 2 つ同じでもそのまま描画され、その id へのリンクや差分は最初の要素に届きます。タグ名のように訪問者が保存した値から id を作るときは、レコードのキーと組み合わせて一意にしてください。",
                },
                L {
                    en: "Its own queries and files, such as SQL built from arguments, and the path an uploaded file is stored at. `UploadedFile::file_name` returns a `FileName`, whose `base_name` names no directory and whose `as_sent` is the text the browser sent, but two visitors can send the same name, so store the file under a name the application chooses.",
                    ja: "引数から組み立てる SQL や、アップロードしたファイルを保存するパスのような、アプリ自身のクエリとファイル。`UploadedFile::file_name` は `FileName` を返します。その `base_name` はディレクトリを含まず、`as_sent` はブラウザが送った文字列そのものです。ただし 2 人の訪問者が同じ名前を送ることもあるので、ファイルはアプリが決めた名前で保存してください。",
                },
            ],
        ),
    ])
}

/// A Content-Security-Policy with a nonce, as a complete example.
fn nonce(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Sending a Content-Security-Policy",
                ja: "Content-Security-Policy を送る",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn csp_nonce(self, nonce: impl Into<String>) -> Self",
        ),
        para(
            site,
            L {
                en: "[`Page::csp_nonce`](api:hypersig/struct.Page.html#method.csp_nonce) puts the nonce on the page's `<style>` and `<script>` elements, and the runtime puts the same nonce on each `<style>` it adds for the rules a patch brings. It panics on an empty nonce or one that holds a character a Content-Security-Policy nonce cannot hold, which is anything but a letter, a digit, `+`, `/`, `-`, `_`, and a trailing `=`. Nothing checks that the nonce is random or that it matches the header, so the middleware below makes both from one value.",
                ja: "[`Page::csp_nonce`](api:hypersig/struct.Page.html#method.csp_nonce) は、ページの `<style>` と `<script>` の要素に nonce を付けます。差分が運ぶ規則のためにランタイムが加える `<style>` にも、同じ nonce が付きます。nonce が空のときと、Content-Security-Policy の nonce に使えない文字（英字、数字、`+`、`/`、`-`、`_`、末尾の `=` 以外）を含むときはパニックします。nonce が乱数であることと、ヘッダーと一致することは検査しないので、下のミドルウェアは 1 つの値から両方を作ります。",
            },
        ),
        para(
            site,
            L {
                en: "The middleware gives each request 128 random bits from the `getrandom` crate, hands the nonce to the page's handler through an `Extension`, and adds the policy to the response. The page also lets `https://portal.example` show it in a frame, and the test at the end reads both `Content-Security-Policy` headers the response carries.",
                ja: "このミドルウェアは `getrandom` クレートでリクエストごとに 128 ビットの乱数を作り、`Extension` でページのハンドラに nonce を渡し、応答にポリシーを加えます。ページは `https://portal.example` の枠にも表示できるようにしています。最後のテストは応答に付いた 2 つの `Content-Security-Policy` のヘッダーを読みます。",
            },
        ),
        show(Snippet::Csp),
        para(
            site,
            L {
                en: "A nonce does not reach a `style` attribute, so with a nonce the values of `bind_style` and `bind_token` are written as rules of a nonced `<style>`, and a `style` set with `El::attr` needs `style-src-attr 'unsafe-inline'`.",
                ja: "nonce は `style` 属性には届きません。そのため nonce があるときは、`bind_style` と `bind_token` の値を nonce 付きの `<style>` の規則として書きます。`El::attr` で付けた `style` には `style-src-attr 'unsafe-inline'` が必要です。",
            },
        ),
    ])
}

/// How the checks above are kept in place.
fn tested(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "How the checks are tested",
                ja: "検査の確かめ方",
            },
        ),
        para(
            site,
            L {
                en: "Each refusal above has a test in the repository: the escaping and the attribute checks in the tests of the HTML builder and of the style values, the requests in `crates/hypersig/tests/axum.rs` and `upload.rs`, the frame headers in `framing.rs`, the caching and sniffing headers in `headers.rs`, and the compile errors in the trybuild cases under `crates/hypersig/tests/ui/fail`. The crates have been reviewed with Trail of Bits' sharp-edges, insecure-defaults, rust-review, and differential-review skills, and each finding they confirmed was fixed, is listed above as the application's to check, or fails only the request that caused it. No outside security audit has been done.",
                ja: "上で挙げた検査には、それぞれリポジトリにテストがあります。エスケープと属性の検査は HTML ビルダーとスタイルの値のテストに、リクエストは `crates/hypersig/tests/axum.rs` と `upload.rs` に、枠のヘッダーは `framing.rs` に、キャッシュと `nosniff` のヘッダーは `headers.rs` に、コンパイルエラーは `crates/hypersig/tests/ui/fail` の trybuild のケースにあります。クレートは Trail of Bits の sharp-edges、insecure-defaults、rust-review、differential-review の Skill で見直し、確かめられた指摘は、直したか、アプリが確かめることとして上に挙げたか、その指摘を起こしたリクエストだけが失敗するかのいずれかです。外部のセキュリティ監査は受けていません。",
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
                    en: "[Signing in](learn/sessions/): reading the visitor in each handler, the first item of what the application checks.",
                    ja: "[サインインとセッション](learn/sessions/): アプリが確かめることの 1 つ目として、各ハンドラで訪問者を読みます。",
                },
                L {
                    en: "[Testing handlers](learn/testing/): sending a request through the router to check a refusal.",
                    ja: "[ハンドラのテスト](learn/testing/): ルーターにリクエストを送って拒否を確かめます。",
                },
            ],
        ),
    ])
}
