//! The page on finding what went wrong in a running page: panics while the
//! page is built, the runtime's console lines, and failed calls.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Debugging).title(site)),
        lead(site, Doc::Learn(Learn::Debugging).leads()),
        panics(site),
        console(site),
        failed_call(site),
        debug_build(site),
        next(site),
    ])
}

/// What panics while the page is built, and where the panic shows.
fn panics(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Debugging a running page",
                ja: "動いているページを調べる",
            },
        ),
        para(
            site,
            L {
                en: "A mistake the types cannot hold panics while the page is built: two elements with one `Id`, a computed field that no `Page::computed` call names, or a list item used outside its closure. The server prints the panic with its message and answers nothing, so the browser reports an empty response. A test that renders each page with `Page::render` finds these before a visitor does, as [Testing handlers](learn/testing/) shows.",
                ja: "型で表せない間違いは、ページを組み立てるときにパニックします。2 つの要素が同じ `Id` を持つ場合、どの `Page::computed` にも渡していない computed フィールドがある場合、リストの項目をクロージャの外で使った場合などです。サーバーはパニックのメッセージを出力して何も返さないので、ブラウザは空の応答として報告します。[ハンドラのテスト](learn/testing/) のように各ページを `Page::render` で描画するテストを書くと、訪問者が開く前に見つかります。",
            },
        ),
        show(Snippet::DuplicateId),
        subheading(
            site,
            L {
                en: "What panics while the page is built",
                ja: "ページを組み立てるときにパニックすること",
            },
        ),
        para(
            site,
            L {
                en: "[`Page::render`](api:hypersig/struct.Page.html#method.render), and `Page::simulate` with it, panics on each of these:",
                ja: "[`Page::render`](api:hypersig/struct.Page.html#method.render) と、同じ検査をする `Page::simulate` は、次のどれかでパニックします。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Two elements that carry one `NodeRef` name, one `FileInput` name, or one id from `Id::new`, with or without keys, where the runtime can reach both. Each item of a list may carry the same name, and two ids from `Id::from_text` may repeat.",
                    ja: "ランタイムが両方に届く 2 つの要素が、同じ `NodeRef` の名前、同じ `FileInput` の名前、`Id::new` の同じ id を持つこと（キーの有無は問いません）。リストの各項目が同じ名前を持つことと、`Id::from_text` の id が重なることは許されます。",
                },
                L {
                    en: "An element inside a list template whose id is set with `El::id`.",
                    ja: "リストのテンプレートの中の要素に `El::id` で id を付けること。",
                },
                L {
                    en: "A field marked `#[signals(computed)]` that no `Page::computed` call names, two calls that name one signal, or a computed signal that reads itself or one declared after it. A page that reads a computed field of a nested struct that no call names panics too.",
                    ja: "`#[signals(computed)]` を付けたフィールドをどの `Page::computed` も指していないこと、2 つの呼び出しが同じシグナルを指すこと、計算シグナルが自分自身やあとで宣言したシグナルを読むこと。入れ子の構造体の計算フィールドを、どの呼び出しも指していないのにページが読む場合もパニックします。",
                },
                L {
                    en: "An expression or a statement that reads a list item outside every list that renders it.",
                    ja: "式や文が、その項目を描画するどのリストの外でも項目を読むこと。",
                },
                L {
                    en: "A signal struct that holds an integer outside -2^53 to 2^53, which the browser would round, or one whose `Serialize` impl returns an error.",
                    ja: "シグナル構造体が -2^53 から 2^53 の範囲の外の整数を持つこと（ブラウザはこの値を丸めます）と、`Serialize` の実装がエラーを返すこと。",
                },
                L {
                    en: "In a debug build, a `Page::lang` that names the language of an `IntlLocale` while the page formats only with others.",
                    ja: "デバッグビルドで、`Page::lang` が `IntlLocale` の言語を指しているのに、ページがほかの言語でしか書式を整えないこと。",
                },
            ],
        ),
    ])
}

/// The lines the runtime writes to the console.
fn console(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "The browser's console",
                ja: "ブラウザのコンソール",
            },
        ),
        para(
            site,
            L {
                en: "In the browser, the runtime writes each problem it meets to the console with the prefix `hypersig:`, such as an expression that threw, an element patch whose id is not on the page, or two list items with one key. Open the developer tools' console first when a button does nothing.",
                ja: "ブラウザでは、ランタイムが見つけた問題を `hypersig:` で始まる行としてコンソールに書きます。例外を投げた式、ページにない id を指す要素の差分、同じキーを持つリストの 2 つの項目などです。ボタンを押しても何も起きないときは、まず開発者ツールのコンソールを開いてください。",
            },
        ),
        code::block(
            Lang::Shell,
            None,
            "hypersig: POST /save failed with 409: Someone else saved these notes first.
hypersig: no element with id \"todo-3\" to patch",
        ),
        table_of(
            site,
            L {
                en: &["Line", "What happened", "What the page does"],
                ja: &["行", "起きたこと", "ページの動き"],
            },
            L {
                en: &[
                    &[
                        "`hypersig: METHOD PATH failed with ...`",
                        "An action, a stream, or an upload got a status outside 2xx, or no answer",
                        "Sets the call's `error` and `status` signals, as the next section shows",
                    ],
                    &[
                        "`hypersig: no element with id \"...\" to patch`",
                        "An element patch or a fragment names an id that is not on the page",
                        "Leaves the page as it was",
                    ],
                    &[
                        "`hypersig: duplicate key ... in a list; matching it by position`",
                        "Two items of a keyed list have one key",
                        "Matches the second item by its position",
                    ],
                    &[
                        "`hypersig: no element with ...`",
                        "A statement of a `NodeRef`, such as `focus`, names an element that is not on the page",
                        "Runs nothing",
                    ],
                    &[
                        "`hypersig: no file input named \"...\"`",
                        "An upload names a `FileInput` that is not on the page",
                        "Sends the arguments without files",
                    ],
                    &[
                        "`hypersig:` and an error",
                        "An expression or a statement threw",
                        "Skips that binding or statement",
                    ],
                ],
                ja: &[
                    &[
                        "`hypersig: METHOD PATH failed with ...`",
                        "アクション、ストリーム、アップロードが 2xx 以外のステータスを受け取ったか、応答が届かなかった",
                        "呼び出しの `error` と `status` のシグナルに書き込みます（次の節）",
                    ],
                    &[
                        "`hypersig: no element with id \"...\" to patch`",
                        "要素の差分かフラグメントがページにない id を指した",
                        "ページを変えません",
                    ],
                    &[
                        "`hypersig: duplicate key ... in a list; matching it by position`",
                        "キー付きのリストの 2 つの項目が同じキーを持つ",
                        "2 つ目の項目を位置で対応させます",
                    ],
                    &[
                        "`hypersig: no element with ...`",
                        "`focus` のような `NodeRef` の文がページにない要素を指した",
                        "何も実行しません",
                    ],
                    &[
                        "`hypersig: no file input named \"...\"`",
                        "アップロードがページにない `FileInput` を指した",
                        "ファイルなしで引数だけを送ります",
                    ],
                    &[
                        "`hypersig:` とエラー",
                        "式か文が例外を投げた",
                        "その束縛か文を飛ばします",
                    ],
                ],
            },
        ),
    ])
}

/// What a failed call writes, and how the page shows it.
fn failed_call(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "A call that fails",
                ja: "失敗した呼び出し",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn error(self, message: Sig<S, String>) -> Self\npub fn status(self, status: Sig<S, i32>) -> Self",
        ),
        para(
            site,
            L {
                en: "An action or a stream that answers with a status outside 2xx, or gets no answer, fails. The runtime then writes `hypersig: METHOD PATH failed with` and the status and body to the console. It dispatches `hs-error` on `<body>`, a `CustomEvent` whose `detail` holds the `status` and the `message`, for a script such as an error tracker. It also sets the signals the call names: [`Call::error`](api:hypersig/struct.Call.html#method.error) gets the body of the answer, or the status and its reason when the body is empty, or the browser's message when no answer arrived, and [`Call::status`](api:hypersig/struct.Call.html#method.status) gets the status, or `0` when no answer arrived. A handler that fails with `Result<_, E>` writes the sentence the page shows, and the page tells one failure from another by comparing the status with a `StatusCode`, not by reading the text.",
                ja: "アクションやストリームが 2xx 以外のステータスで答えるか、答えが届かないと、呼び出しは失敗します。そのときランタイムはコンソールに `hypersig: METHOD PATH failed with` とステータスと本文を書きます。`<body>` で `hs-error` を送ります。これは `detail` に `status` と `message` を持つ `CustomEvent` で、エラーを記録するスクリプトなどが受け取ります。呼び出しが指定したシグナルにも書き込みます。[`Call::error`](api:hypersig/struct.Call.html#method.error) には答えの本文が入り、本文が空ならステータスとその説明が、答えが届かなければブラウザのメッセージが入ります。[`Call::status`](api:hypersig/struct.Call.html#method.status) にはステータスが入り、答えが届かなければ `0` です。`Result<_, E>` で失敗するハンドラは、ページに出す文を自分で書きます。ページは失敗の種類を、文を読むのではなくステータスを `StatusCode` と比べて見分けます。",
            },
        ),
        show(Snippet::CallError),
        para(
            site,
            L {
                en: "`error` takes a `String` signal and `status` an `i32` signal of the same page, so a call cannot write a status into a text field. [Busy and failed calls](learn/call-status/) covers the indicator and the other signals of a call.",
                ja: "`error` は同じページの `String` のシグナルを、`status` は `i32` のシグナルを受け取るので、ステータスをテキストのフィールドに書く呼び出しはコンパイルできません。インジケーターなど、呼び出しのほかのシグナルは [呼び出し中と失敗](learn/call-status/) で扱います。",
            },
        ),
        show(Snippet::CallStatusType),
    ])
}

/// What a debug build serves beside the page.
fn debug_build(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "What a debug build adds",
                ja: "デバッグビルドで加わるもの",
            },
        ),
        para(
            site,
            L {
                en: "A tab showing a page from a debug build reloads once a new server answers in place of the one that rendered the page, as after `cargo hypersig dev` rebuilds it, and keeps the signals the visitor changed, where the route and the struct's fields are the same. A page from a release build does neither. [The development server](learn/dev-server/) covers `cargo hypersig dev`.",
                ja: "デバッグビルドが配信したページを開いたタブは、`cargo hypersig dev` がビルドし直したあとのように、ページを描画したサーバーに代わって新しいサーバーが答えると、ページを読み込み直します。ルートと構造体のフィールドが同じなら、訪問者が変えたシグナルはそのまま保持されます。リリースビルドが配信したページでは、どちらも起きません。`cargo hypersig dev` は [開発サーバー](learn/dev-server/) で扱います。",
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
                    en: "[Testing handlers](learn/testing/): rendering each page in a test, so its panics show before a visitor's request.",
                    ja: "[ハンドラのテスト](learn/testing/): ページごとに描画するテストで、訪問者のリクエストより先にパニックを見つけます。",
                },
                L {
                    en: "[Errors and logging](learn/errors/): the status and the sentence a handler answers a failure with, and what the server logs.",
                    ja: "[エラーとログ](learn/errors/): ハンドラが失敗を伝えるステータスと文、サーバーが記録するもの。",
                },
                L {
                    en: "[Upgrading](learn/upgrading/): the errors a newer release raises in code written for an older one.",
                    ja: "[新しいリリースに上げる](learn/upgrading/): 古いリリース向けのコードで、新しいリリースが出すエラー。",
                },
            ],
        ),
    ])
}
