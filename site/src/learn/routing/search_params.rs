//! The page on typed query strings and on changing the URL without loading
//! a page.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::SearchParams).title(site)),
        lead(site, Doc::Learn(Learn::SearchParams).leads()),
        typed(site),
        written(site),
        read(site),
        search_errors(site),
        history(site),
        next(site),
    ])
}

/// `Route<P, Q>` and `#[derive(Search)]`.
fn typed(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A typed query string",
                ja: "型の付いたクエリ文字列",
            },
        ),
        para(
            site,
            L {
                en: "A route's second type, `Route<P, Q>`, types its query string. `Q` is a struct with `#[derive(Signals, Search)]`, which implements [`Search`](api:hypersig/trait.Search.html). `link_with`, `url`, and the handler then take `(P, Q)`, or `Q` alone for a path without parameters. A field can come from a signal, so a link to the next page of results reads the page the visitor is on.",
                ja: "ルートの 2 つ目の型 `Route<P, Q>` はクエリ文字列の型です。`Q` は `#[derive(Signals, Search)]` を付けた構造体で、[`Search`](api:hypersig/trait.Search.html) を実装します。すると `link_with`、`url`、ハンドラは `(P, Q)` を受け取ります。パラメータのないパスでは `Q` だけを受け取ります。フィールドはシグナルからも取れるので、結果の次のページへのリンクは、訪問者が今いるページの番号を読めます。",
            },
        ),
        show(Snippet::SearchParams),
    ])
}

/// How the fields become the query string.
fn written(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "How the fields are written",
                ja: "フィールドの書き方",
            },
        ),
        para(
            site,
            L {
                en: "`#[derive(Search)]` requires each field's type to be a [`SearchValue`](api:hypersig/trait.SearchValue.html): a [`SearchScalar`](api:hypersig/trait.SearchScalar.html), or an `Option` or a `Vec` of one. The query string lists the fields in the order of their names, compared byte by byte, on the server and in the browser alike, and percent-encodes each name and value as `encodeURIComponent` does.",
                ja: "`#[derive(Search)]` は、各フィールドの型が [`SearchValue`](api:hypersig/trait.SearchValue.html) であることを求めます。`SearchValue` は [`SearchScalar`](api:hypersig/trait.SearchScalar.html)、またはその `Option` か `Vec` です。クエリ文字列はフィールドを名前のバイト順に並べ、名前と値を `encodeURIComponent` と同じ方法でパーセントエンコードします。これはサーバーでもブラウザでも同じです。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Field type", "Written as"],
                ja: &["フィールドの型", "書き方"],
            },
            L {
                en: &[
                    &[
                        "`String`, `f64`, `f32`, `bool`, `i32`, `i64`, `u32`, `u64`, `PathSegment`, an enum with `#[derive(Segment)]`",
                        "`name=value`, the value written as JavaScript's `String` writes it",
                    ],
                    &["`Option<T>`", "`name=value`, or nothing for `None`"],
                    &[
                        "`Vec<T>`",
                        "`name=a&name=b`, once per item in item order, or nothing when empty",
                    ],
                ],
                ja: &[
                    &[
                        "`String`、`f64`、`f32`、`bool`、`i32`、`i64`、`u32`、`u64`、`PathSegment`、`#[derive(Segment)]` を付けた列挙型",
                        "`name=value`。値は JavaScript の `String` と同じ方法で書きます",
                    ],
                    &["`Option<T>`", "`name=value`。`None` なら何も書きません"],
                    &[
                        "`Vec<T>`",
                        "`name=a&name=b`。要素ごとに要素の順で書き、空なら何も書きません",
                    ],
                ],
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "An `i64` or `u64` past 2^53 is written with its exact digits, so the handler receives it exactly.",
                    ja: "2^53 を超える `i64` や `u64` は正確な数字で書くので、ハンドラには正確な値が届きます。",
                },
                L {
                    en: "`String` writes `-0.0` as `0`, so an `f64` of `-0.0` arrives as `0.0`. serde writes a NaN or an infinity as `null`, so its field is left out, and a required field then gets a 400 where an `Option` arrives as `None`.",
                    ja: "`String` は `-0.0` を `0` と書くので、`-0.0` の `f64` は `0.0` として届きます。serde は NaN と無限大を `null` と書くので、そのフィールドは書かれません。必須のフィールドなら 400 になり、`Option` なら `None` として届きます。",
                },
                L {
                    en: "The server compares names by their UTF-8 bytes and the browser by their UTF-16 units, which agree for ASCII names. A `#[serde(rename)]` to a name outside ASCII can put two fields in a different order on the two sides.",
                    ja: "サーバーは名前を UTF-8 のバイトで、ブラウザは UTF-16 の単位で比べます。ASCII の名前なら両者は一致します。`#[serde(rename)]` で ASCII 以外の名前にすると、両側で 2 つのフィールドの順が変わることがあります。",
                },
            ],
        ),
    ])
}

/// How the server reads the query string back.
fn read(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "How the server reads them",
                ja: "サーバーでの読み方",
            },
        ),
        para(
            site,
            L {
                en: "The server reads the query string back into `Q` before the handler runs. A key that appears more than once fills a `Vec` field, `+` reads as a space, and a key the struct does not have is ignored. A query string that does not deserialize into `Q` gets a 400: a missing field that is not an `Option` or a `Vec`, a repeated key for a field that is not a `Vec`, or a value the field's `Deserialize` rejects. Without an `else` handler, the body of that 400 is the text \"the query string does not deserialize into the page's search parameters\".",
                ja: "サーバーはハンドラが動く前にクエリ文字列を `Q` に読み戻します。繰り返されたキーは `Vec` のフィールドに入り、`+` は空白として読み、構造体にないキーは無視します。`Q` にデシリアライズできないクエリ文字列には 400 を返します。`Option` でも `Vec` でもないフィールドがない場合、`Vec` でないフィールドのキーが繰り返された場合、フィールドの `Deserialize` が値を拒否した場合がこれにあたります。`else` のハンドラがなければ、この 400 の本文は \"the query string does not deserialize into the page's search parameters\" という文です。",
            },
        ),
        para(
            site,
            L {
                en: "So a field whose type checks its value in `Deserialize`, such as a newtype with `#[serde(try_from = \"i32\")]`, validates the query string, and an `Option` field with `unwrap_or` or `map_or` in the handler gives a default. A newtype that serde writes as one value becomes a `SearchScalar` with `impl SearchScalar for PageNumber {}`:",
                ja: "そのため、`#[serde(try_from = \"i32\")]` を付けた newtype のように `Deserialize` で値を検査する型のフィールドは、クエリ文字列を検証します。`Option` のフィールドにハンドラで `unwrap_or` や `map_or` を使えば、既定値を決められます。serde が 1 つの値として書く newtype は、`impl SearchScalar for PageNumber {}` と書くと `SearchScalar` になります。",
            },
        ),
        show(Snippet::SearchFields),
    ])
}

/// The field types `#[derive(Search)]` refuses, and the claim it trusts.
fn search_errors(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査すること",
            },
        ),
        para(
            site,
            L {
                en: "A query string holds text values under each key, so a struct in a field, a list of lists, and an `Option` of a list are compile errors:",
                ja: "クエリ文字列は各キーの下に文字列の値を持つだけなので、フィールドの構造体、リストのリスト、リストの `Option` はコンパイルエラーになります。",
            },
        ),
        show(Snippet::WrongSearch),
        para(
            site,
            L {
                en: "The compiler does not check an `impl SearchScalar` of your own. A type that serde writes as a list or an object gives links whose query string the server answers with 400.",
                ja: "自分で書いた `impl SearchScalar` は、コンパイラが検査しません。serde がリストやオブジェクトとして書く型だと、そのリンクのクエリ文字列にサーバーは 400 を返します。",
            },
        ),
    ])
}

/// `push_url` and `replace_url`, on a link and in an action's answer.
fn history(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Changing the URL without loading a page",
                ja: "ページを読み込まずに URL を変える",
            },
        ),
        para(
            site,
            L {
                en: "A list whose filter, sort, and page live in the query string can keep the page loaded while they change. `Link::push_url(self) -> Stmt<S>` is a statement that shows the link's URL in the address bar as a new history entry. `Link::replace_url(self) -> Stmt<S>` shows it in place of the current one, for a change the Back button should skip, such as each letter typed into a search field. A URL equal to the current one adds no entry.",
                ja: "フィルター、並び順、ページ番号をクエリ文字列に持つリストは、それらが変わってもページを読み込み直さずに済みます。`Link::push_url(self) -> Stmt<S>` は、リンクの URL を新しい履歴の項目としてアドレスバーに出す文です。`Link::replace_url(self) -> Stmt<S>` は今の URL と置き換えるので、検索欄に打つ 1 文字ごとのように、戻るボタンで飛ばしたい変更に使います。今の URL と同じ URL では履歴の項目は増えません。",
            },
        ),
        para(
            site,
            L {
                en: "The page keeps its signals, its focus, and its scroll position, so the statement runs beside the call that loads the rows. An action does the same with `Patches::push_url(url: Url)` and `Patches::replace_url(url: Url)`. Unlike `navigate`, the patches after them are applied. When the visitor presses Back or Forward, the browser loads the page at that entry's URL from the server, which renders it from the query string.",
                ja: "ページのシグナル、フォーカス、スクロール位置はそのまま保持されるので、この文は行を読み込む呼び出しと並べて書きます。アクションでは `Patches::push_url(url: Url)` と `Patches::replace_url(url: Url)` で同じことをします。`navigate` と違い、その後の差分も適用されます。訪問者が戻るボタンや進むボタンを押すと、ブラウザはその項目の URL のページをサーバーから読み込み、サーバーはクエリ文字列からページを描きます。",
            },
        ),
        show(Snippet::History),
        para(
            site,
            L {
                en: "Nothing checks that the URL `push_url` or `replace_url` shows belongs to the page on screen. The URL of another page shows over this page's content, and reloading it loads the other page.",
                ja: "`push_url` や `replace_url` が出す URL が、画面のページのものかどうかは検査されません。別のページの URL を出すと、このページの内容のままその URL が表示され、再読み込みすると別のページが読み込まれます。",
            },
        ),
    ])
}

/// The pages after this one.
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
                    en: "[Error pages](learn/error-pages/) shows a page of your own with the 400 for a query string that does not deserialize.",
                    ja: "[エラーページ](learn/error-pages/) ではデシリアライズできないクエリ文字列への 400 と一緒に独自のページを出します。",
                },
                L {
                    en: "[Links and redirects](learn/links/) covers `navigate`, which loads the next page.",
                    ja: "次のページを読み込む `navigate` は [リンクとリダイレクト](learn/links/) で説明します。",
                },
            ],
        ),
    ])
}
