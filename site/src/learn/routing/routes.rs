//! The page on declaring routes, the types of their path parameters, and
//! the handlers that serve them.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::learn::{Program, program};
use crate::prose::{Part, bullets, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Routes).title(site)),
        lead(site, Doc::Learn(Learn::Routes).leads()),
        declaring(site),
        parameters(site),
        segment_types(site),
        handler(site),
        compile_errors(site),
        unchecked(site),
        example(site),
        next(site),
    ])
}

/// `Route` and the lines of `routes!`.
fn declaring(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Declaring a route",
                ja: "ルートを宣言する",
            },
        ),
        para(
            site,
            L {
                en: "A [`Route<P = (), Q = ()>`](api:hypersig/struct.Route.html) is a page served at a path whose parameters are `P`, with search parameters `Q`. You declare it as a constant in [`routes!`](api:hypersig/macro.routes.html), on a line that names the handler serving it: `const MEMBER: Route<i32> = \"/members/{id}\" => member;`. A route without a handler cannot be declared.",
                ja: "[`Route<P = (), Q = ()>`](api:hypersig/struct.Route.html) は、パラメータが `P`、検索パラメータが `Q` のパスで配信するページです。[`routes!`](api:hypersig/macro.routes.html) の中で定数として宣言し、同じ行に配信するハンドラを書きます。`const MEMBER: Route<i32> = \"/members/{id}\" => member;` のように書きます。ハンドラのないルートは宣言できません。",
            },
        ),
        para(
            site,
            L {
                en: "The first line of the block, such as `fn pages(Router<()>);`, names the function the macro writes and the router state its handlers take. That function takes a `Router<State>` and returns it with every route of the block registered on `GET`. `routes!` needs the `axum` feature, which is on by default.",
                ja: "ブロックの 1 行目は `fn pages(Router<()>);` のように書き、マクロが生成する関数の名前と、ハンドラが受け取るルーターの状態を指定します。生成された関数は `Router<State>` を受け取り、ブロックのルートをすべて `GET` に登録して返します。`routes!` には `axum` フィーチャーが必要です。このフィーチャーはデフォルトで有効です。",
            },
        ),
    ])
}

/// How `{name}` segments map to the values of `P`.
fn parameters(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Path parameters",
                ja: "パスのパラメータ",
            },
        ),
        para(
            site,
            L {
                en: "A parameter is a whole segment written `{name}`, as in axum. `P` is `()` for a path without one, one [`Segment`](api:hypersig/trait.Segment.html) type for one, and a tuple of two to six for more, in path order. The name inside the braces is for the reader and for axum. The position decides which value fills it.",
                ja: "パラメータは axum と同じく `{name}` と書いたセグメント 1 つ分です。`P` は、パラメータのないパスでは `()`、1 つなら [`Segment`](api:hypersig/trait.Segment.html) の型 1 つ、それより多ければパスの順に並べた 2 つから 6 つのタプルです。波かっこの中の名前は読む人と axum のためのもので、どの値が入るかは位置で決まります。",
            },
        ),
        para(
            site,
            L {
                en: "The macro counts the segments, so a path whose count differs from `P` fails to compile:",
                ja: "マクロがセグメントを数えるので、数が `P` と合わないパスはコンパイルできません。",
            },
        ),
        show(Snippet::WrongRoute),
    ])
}

/// The four kinds of `Segment`.
fn segment_types(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What a segment can hold",
                ja: "セグメントに入る型",
            },
        ),
        table_of(
            site,
            L {
                en: &["Type", "The segment holds", "A request gets a 404 for"],
                ja: &["型", "セグメントの中身", "404 になるリクエスト"],
            },
            L {
                en: &[
                    &[
                        "`i32`, `i64`, `u32`, `u64`",
                        "The number's digits",
                        "Text that does not parse into the type",
                    ],
                    &[
                        "[`PathSegment`](api:hypersig/struct.PathSegment.html)",
                        "Any text, percent-encoded",
                        "A segment that decodes to empty, `.`, or `..`",
                    ],
                    &[
                        "An enum with `#[derive(Segment)]`",
                        "The string serde writes for the variant",
                        "Text that names no variant",
                    ],
                    &[
                        "[`Rest`](api:hypersig/struct.Rest.html), from a last segment `{*name}`",
                        "The rest of the path, `/`s included",
                        "A part that decodes to `.` or `..`",
                    ],
                ],
                ja: &[
                    &[
                        "`i32`、`i64`、`u32`、`u64`",
                        "数値の数字",
                        "その型として解析できない文字列",
                    ],
                    &[
                        "[`PathSegment`](api:hypersig/struct.PathSegment.html)",
                        "パーセントエンコードした任意の文字列",
                        "デコードすると空、`.`、`..` になるセグメント",
                    ],
                    &[
                        "`#[derive(Segment)]` を付けた列挙型",
                        "serde がそのヴァリアントに対して書き出す文字列",
                        "どのヴァリアントにも当たらない文字列",
                    ],
                    &[
                        "最後のセグメント `{*name}` から受け取る [`Rest`](api:hypersig/struct.Rest.html)",
                        "`/` を含むパスの残り",
                        "デコードすると `.` か `..` になる部分",
                    ],
                ],
            },
        ),
        show(Snippet::RouteSegments),
        path_segment(site),
        enum_segment(site),
        rest(site),
    ])
}

/// `PathSegment` and the three values it refuses.
fn path_segment(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Text in a segment",
                ja: "セグメントに入れる文字列",
            },
        ),
        para(
            site,
            L {
                en: "A `PathSegment` holds any text except an empty string, `.`, and `..`. axum does not match a `{name}` segment against an empty one, so `/orgs/` answers 404. The browser removes a `.` or `..` segment from a link before it sends the request, so `/orgs/..` opens `/`, and it reads `%2e` as `.` too, so no encoding keeps a dot. A `/` or a `?` in the text is percent-encoded and stays inside the segment.",
                ja: "`PathSegment` には、空の文字列、`.`、`..` を除く任意の文字列が入ります。axum は `{name}` のセグメントを空のセグメントに一致させないので、`/orgs/` には 404 を返します。ブラウザはリクエストを送る前にリンクから `.` と `..` のセグメントを取り除くので、`/orgs/..` を開くと `/` になります。ブラウザは `%2e` も `.` として読むので、エンコードしてもドットは残りません。文字列の中の `/` や `?` はパーセントエンコードされ、セグメントの中にとどまります。",
            },
        ),
        para(
            site,
            L {
                en: "`const fn PathSegment::new(text: &'static str) -> PathSegment` takes text written in the program. In a `const` it refuses those three values at compile time, and outside one it panics. `PathSegment::parse(text: impl Into<String>) -> Result<PathSegment, PathSegmentError>` reads text built at run time, such as a name from a database. The server refuses the same values in a request, so a handler never receives `..` from a client that sent `/orgs/%2E%2E` by hand. That request gets a 404 before the handler runs.",
                ja: "`const fn PathSegment::new(text: &'static str) -> PathSegment` はプログラムに書いた文字列を受け取ります。`const` の中ではこの 3 つの値をコンパイル時に拒否し、`const` の外ではパニックします。データベースの名前のように実行時に作る文字列は `PathSegment::parse(text: impl Into<String>) -> Result<PathSegment, PathSegmentError>` で読みます。サーバーもリクエストの中の同じ値を拒否するので、`/orgs/%2E%2E` を手で送ったクライアントからハンドラが `..` を受け取ることはありません。このリクエストには、ハンドラが動く前に 404 を返します。",
            },
        ),
    ])
}

/// `#[derive(Segment)]`.
fn enum_segment(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Enums as segments",
                ja: "列挙型のセグメント",
            },
        ),
        para(
            site,
            L {
                en: "`#[derive(Segment)]` implements `Segment` for an enum whose variants have no fields. A variant's segment is the string serde writes for it, so a link that reads the enum from a signal and the server write the same URL. The derive follows `#[serde(rename_all = \"...\")]` on the enum and `#[serde(rename = \"...\")]` on a variant, so `rename_all = \"kebab-case\"` turns `TeamSettings` into `/team-settings`. It rejects any other serde attribute at compile time, because one could make serde write a variant differently.",
                ja: "`#[derive(Segment)]` は、フィールドのないヴァリアントだけを持つ列挙型に `Segment` を実装します。ヴァリアントのセグメントは serde がそのヴァリアントに対して書き出す文字列なので、シグナルから列挙型を読むリンクとサーバーは同じ URL を書きます。derive は列挙型の `#[serde(rename_all = \"...\")]` とヴァリアントの `#[serde(rename = \"...\")]` に従うので、`rename_all = \"kebab-case\"` を付けると `TeamSettings` は `/team-settings` になります。serde の属性がヴァリアントの書き方を変えないように、ほかの serde の属性はコンパイル時に拒否します。",
            },
        ),
    ])
}

/// `Rest` and `{*name}`.
fn rest(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "The rest of a path",
                ja: "パスの残り",
            },
        ),
        para(
            site,
            L {
                en: "A last segment written `{*name}`, as in `\"/files/{*path}\"`, takes the rest of the path, `/`s included, into a `Rest`. Only the last value of `P` can be a `Rest`. A link percent-encodes each `/`-separated part and keeps the `/`s between them. A `Rest` is not empty and has no part that is `.` or `..`, for the reasons a `PathSegment` gives, and an empty part, as in `docs/` or `a//b`, is kept. `Rest::new` and `Rest::parse` check it as `PathSegment::new` and `PathSegment::parse` do, and the server refuses the same values in a request, so a handler that opens the file a `Rest` names never receives `../`.",
                ja: "`\"/files/{*path}\"` のように最後のセグメントを `{*name}` と書くと、`/` を含むパスの残りを `Rest` で受け取ります。`Rest` にできるのは `P` の最後の値だけです。リンクは `/` で区切った部分をそれぞれパーセントエンコードし、間の `/` はそのまま残します。`Rest` は `PathSegment` と同じ理由で空にならず、`.` や `..` の部分も持ちません。`docs/` や `a//b` のような空の部分は残ります。`Rest::new` と `Rest::parse` は `PathSegment::new` と `PathSegment::parse` と同じように検査し、サーバーもリクエストの中の同じ値を拒否します。そのため、`Rest` が指すファイルを開くハンドラが `../` を受け取ることはありません。",
            },
        ),
    ])
}

/// What a page handler takes and returns.
fn handler(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The handler",
                ja: "ハンドラ",
            },
        ),
        para(
            site,
            L {
                en: "A page handler is an `async fn(State, Args) -> R`. `State` is the router state of the first line, and `Args` is `P`, already parsed. It may take one `FromRequestParts` extractor between the two, as [`Handler`](api:hypersig/trait.Handler.html) describes. `R` is a `Page<S>`, an `Option<Page<S>>`, or a [`PageResponse`](api:hypersig/struct.PageResponse.html).",
                ja: "ページのハンドラは `async fn(State, Args) -> R` です。`State` は 1 行目に書いたルーターの状態で、`Args` は解析済みの `P` です。[`Handler`](api:hypersig/trait.Handler.html) のとおり、この 2 つの間に `FromRequestParts` のエクストラクタを 1 つ受け取れます。`R` は `Page<S>`、`Option<Page<S>>`、[`PageResponse`](api:hypersig/struct.PageResponse.html) のどれかです。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A path whose segments do not parse into the parameter types gets a 404, and the handler does not run.",
                    ja: "セグメントをパラメータの型に解析できないパスには 404 を返し、ハンドラは動きません。",
                },
                L {
                    en: "A handler that returns `None` gets a 404, as `member` does in the example below for an id it does not know.",
                    ja: "`None` を返したハンドラにも 404 を返します。下の例の `member` は知らない id に対して `None` を返します。",
                },
                L {
                    en: "Without an `else` handler on the first line, those 404s have an empty body. [Error pages](learn/error-pages/) shows how to send a page of your own with them.",
                    ja: "1 行目に `else` のハンドラがなければ、これらの 404 の本文は空です。独自のページを一緒に送る方法は [エラーページ](learn/error-pages/) で説明します。",
                },
            ],
        ),
    ])
}

/// The mistakes `routes!` refuses.
fn compile_errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the compiler checks",
                ja: "コンパイラが検査すること",
            },
        ),
        para(
            site,
            L {
                en: "A parameter type that is not a `Segment`, such as `String`, is a compile error. Use `PathSegment` for text:",
                ja: "`String` のように `Segment` でないパラメータの型はコンパイルエラーになります。文字列には `PathSegment` を使います。",
            },
        ),
        show(Snippet::WrongSegment),
        para(
            site,
            L {
                en: "`routes!` also refuses these at compile time:",
                ja: "`routes!` は次のものもコンパイル時に拒否します。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "A line without `=> handler`, or a handler whose arguments or answer do not fit its line.",
                    ja: "`=> handler` のない行と、引数や戻り値が行に合わないハンドラ。",
                },
                L {
                    en: "A path that does not start with `/`, or a `{` or `}` that does not enclose a whole named segment.",
                    ja: "`/` で始まらないパスと、名前のあるセグメント全体を囲んでいない `{` や `}`。",
                },
                L {
                    en: "A path that holds a byte the browser percent-encodes, such as a space, `?`, `#`, `%`, or a non-ASCII character, because the request would reach no handler.",
                    ja: "空白、`?`、`#`、`%`、ASCII 以外の文字のように、ブラウザがパーセントエンコードするバイトを含むパス。そのリクエストはどのハンドラにも届かないためです。",
                },
                L {
                    en: "A `{*name}` segment that is not last, or whose value is not a `Rest`.",
                    ja: "最後にない `{*name}` のセグメントと、値が `Rest` でない `{*name}` のセグメント。",
                },
            ],
        ),
    ])
}

/// What routes leave to the program.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What routes leave unchecked",
                ja: "ルートが検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Nothing checks that the function `routes!` writes is called, so declare every page of the application in one `routes!` block and call its one function. A block whose function is never called answers 404 on every page, and rustc reports its handler functions as never used, though not the function itself or a handler written as a closure.",
                    ja: "`routes!` が書く関数が呼び出されているかどうかは検査されません。アプリケーションのページはすべて 1 つの `routes!` ブロックに宣言し、その関数 1 つを呼び出してください。関数が一度も呼び出されないブロックは、どのページにも 404 を返します。このとき rustc はハンドラの関数が使われていないと報告しますが、関数そのものとクロージャで書いたハンドラは報告しません。",
                },
                L {
                    en: "An `i64` or `u64` parameter past 2^53 reaches the handler exactly. A page whose signals hold such a value panics while rendering, because the browser would round it.",
                    ja: "2^53 を超える `i64` や `u64` のパラメータもハンドラには正確に届きます。ただし、そのような値をシグナルに持つページは描画の途中でパニックします。ブラウザが値を丸めてしまうためです。",
                },
            ],
        ),
    ])
}

/// A complete program with two routes.
fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A complete example",
                ja: "完全な例",
            },
        ),
        para(
            site,
            L {
                en: "Two pages: a team at `/` and a member at `/members/{id}`. `team` links to each member from the list item's id, and `member` answers `None`, so a 404, for an id it does not know.",
                ja: "`/` のチームと `/members/{id}` のメンバーの 2 ページです。`team` はリストの項目の id から各メンバーへリンクし、`member` は知らない id に対して `None` を返すので、そのリクエストは 404 になります。",
            },
        ),
        program(Program::Pages),
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
                    en: "[Links and redirects](learn/links/) builds links, URLs, and redirects from a route.",
                    ja: "[リンクとリダイレクト](learn/links/) ではルートからリンク、URL、リダイレクトを作ります。",
                },
                L {
                    en: "[Pages under a page](learn/child-pages/) declares a route that continues another.",
                    ja: "[子のページ](learn/child-pages/) では別のルートの続きになるルートを宣言します。",
                },
                L {
                    en: "[Search parameters](learn/search-params/) types the query string.",
                    ja: "[検索パラメータ](learn/search-params/) ではクエリ文字列に型を付けます。",
                },
            ],
        ),
    ])
}
