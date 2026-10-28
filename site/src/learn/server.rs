//! The pages on talking to the server, styling, and what to use when bindings
//! and statements are not enough.

use super::snippet::{Snippet, show};
use super::{Program, program};
use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn routes_in_depth(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Routes).title(site)),
        lead(site, Doc::Learn(Learn::Routes).leads()),
        heading(
            site,
            L {
                en: "Search parameters",
                ja: "クエリパラメータ",
            },
        ),
        para(
            site,
            L {
                en: "A route's second type, `Route<P, Q>`, types its query string. `Q` is a struct with `#[derive(Signals, Search)]`, and a link and the handler then take `(P, Q)`, or `Q` alone for a path without parameters. `#[derive(Search)]` requires each field to be a [`SearchValue`](api:hypersig/trait.SearchValue.html): a string, a number, a `bool`, an enum with `#[derive(Segment)]`, or an `Option` or a `Vec` of one, so a struct or a list of lists in a field is a compile error. A `Vec` field writes its name once per item, as `?color=red&color=blue`, and the server reads the repeated key back into the list. A field whose type checks its value in `Deserialize`, such as a newtype with `#[serde(try_from = \"i32\")]`, validates the query string, and an `Option` field with `unwrap_or` in the handler gives a default. The query string lists the fields in the order of their names and leaves out a field that is `None`, on the server and in the browser alike. A query string that does not deserialize into `Q` gets a 400.",
                ja: "ルートの 2 つ目の型 `Route<P, Q>` は、クエリ文字列の型です。`Q` は `#[derive(Signals, Search)]` を付けた構造体で、リンクとハンドラは `(P, Q)` を受け取ります。パラメータのないパスでは `Q` だけを受け取ります。`#[derive(Search)]` は、各フィールドが [`SearchValue`](api:hypersig/trait.SearchValue.html) であることを求めます。`SearchValue` は文字列、数値、`bool`、`#[derive(Segment)]` を付けた列挙型、またはそれらの `Option` か `Vec` なので、フィールドに構造体やリストのリストを置くとコンパイルエラーになります。`Vec` のフィールドは `?color=red&color=blue` のように要素ごとに名前を書き、サーバーは繰り返されたキーをリストに読み戻します。`#[serde(try_from = \"i32\")]` を付けた newtype のように `Deserialize` で値を検査する型のフィールドは、クエリ文字列を検証します。`Option` のフィールドにハンドラで `unwrap_or` を使えば、既定値を決められます。クエリ文字列はフィールドを名前の順に並べ、`None` のフィールドは書きません。これはサーバーでもブラウザでも同じです。`Q` にデシリアライズできないクエリ文字列には 400 を返します。",
            },
        ),
        show(Snippet::SearchParams),
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
                en: "A list whose filter, sort, and page live in the query string can keep the page loaded while they change. `link.push_url()` is a statement that shows the link's URL in the address bar as a new history entry, and `link.replace_url()` shows it in place of the current one, for a change the Back button should skip, such as each letter typed into a search field. The page keeps its signals and its focus, so the statement runs beside the call that loads the rows. An action does the same with `Patches::push_url(url)` and `Patches::replace_url(url)`, and the patches after them are applied. When the visitor presses Back or Forward, the browser loads the page at that entry's URL from the server, which renders it from the query string.",
                ja: "フィルター、並び順、ページ番号をクエリ文字列に持つリストは、それらが変わってもページを読み込み直さずに済みます。`link.push_url()` はリンクの URL を新しい履歴の項目としてアドレスバーに出す文です。`link.replace_url()` は今の URL と置き換えるので、検索欄に打つ 1 文字ごとのように、戻るボタンで飛ばしたい変更に使います。ページのシグナルとフォーカスはそのまま保持されるので、この文は行を読み込む呼び出しと並べて書きます。アクションでは `Patches::push_url(url)` と `Patches::replace_url(url)` で同じことをし、その後の差分も適用されます。訪問者が戻るボタンや進むボタンを押すと、ブラウザはその項目の URL のページをサーバーから読み込み、サーバーはクエリ文字列からページを描きます。",
            },
        ),
        show(Snippet::History),
        heading(
            site,
            L {
                en: "Pages for missing paths",
                ja: "見つからないパスのページ",
            },
        ),
        para(
            site,
            L {
                en: "`fn pages(Router<State>) else missing;` on the first line of `routes!` names a handler for each page request that reaches no page: a path no route matches, a segment that does not parse, a handler that answers `None`, and a query string that does not deserialize. The handler receives an [`Unserved`](api:hypersig/struct.Unserved.html) and may take an extractor, and its page is sent with the 404 or 400 that `Unserved::status` names. `Unserved::within(HOME)` reads the parameters of `HOME` from the start of the path, so a site under `/{lang}` shows the page in the language of the path. A page with another status is `PageResponse::with_status(status, page)`.",
                ja: "`routes!` の 1 行目に `fn pages(Router<State>) else missing;` と書くと、どのページにも届かなかったページのリクエストを受け取るハンドラを指定できます。対象は、どのルートにも合わないパス、パースできないセグメント、`None` を返したハンドラ、デシリアライズできないクエリ文字列です。ハンドラは [`Unserved`](api:hypersig/struct.Unserved.html) を受け取り、extractor も受け取れます。返したページは `Unserved::status` が示す 404 か 400 で送られます。`Unserved::within(HOME)` はパスの先頭から `HOME` のパラメータを読むので、`/{lang}` の下にあるサイトはパスの言語でページを出せます。ほかのステータスで返すページは `PageResponse::with_status(status, page)` で返します。",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks that the URL `push_url` or `replace_url` shows belongs to the page on screen. The URL of another page shows over this page's content, and reloading it loads the other page.",
                ja: "`push_url` や `replace_url` が出す URL が、画面のページのものかどうかは検査されません。別のページの URL を出すと、このページの内容のままその URL が表示され、再読み込みすると別のページが読み込まれます。",
            },
        ),
        head(site),
        heading(
            site,
            L {
                en: "Serving under a path prefix",
                ja: "パスの接頭辞の下で配信する",
            },
        ),
        para(
            site,
            L {
                en: "An application served under a path prefix, such as `/app` behind a proxy, names it once as a [`Base`](api:hypersig/struct.Base.html) constant: `fn pages(Router<State>) under APP;` on the first line of `routes!`. Links, URLs, redirects, the calls to the block's actions, streams, and uploads, the path a page loads the runtime from, each `Asset` path the page loads, and each root path in a stylesheet's `url(..)`, in the page and in the patches the block's handlers send, then start with `/app`, and the router still registers each path without it. The same router then runs inside `Router::nest(APP.as_str(), ...)` or behind a proxy that strips `/app`. The prefix is a constant because the routes and actions it prefixes are, so it is chosen when the program is built. A page served by a router nested under any other path answers every request with a 500 whose body names neither path, and the compiler does not check that an action a page calls comes from a block under the same base.",
                ja: "プロキシの裏の `/app` のようにパスの接頭辞の下で配信するアプリケーションは、接頭辞を [`Base`](api:hypersig/struct.Base.html) の定数として 1 か所で宣言します。`routes!` の 1 行目に `fn pages(Router<State>) under APP;` と書きます。するとリンク、URL、リダイレクト、そのブロックのアクション、ストリーム、アップロードの呼び出し、ページがランタイムを読み込むパス、ページが読み込む `Asset` のパス、スタイルシートの `url(..)` に書いたルートからのパス（ページの中と、ブロックのハンドラが送る差分の中の両方）が `/app` で始まり、ルーターは各パスを接頭辞なしのまま登録します。同じルーターを `Router::nest(APP.as_str(), ...)` の中でも、`/app` を取り除くプロキシの裏でも動かせます。接頭辞を付けるルートとアクションが定数なので、接頭辞も定数で、プログラムをビルドするときに決まります。ほかのパスに `nest` したルーターのページは、どのリクエストにも 500 を返します。本文にはどちらのパスも書きません。ページが呼ぶアクションが同じ接頭辞のブロックで宣言されているかどうかを、コンパイラは検査しません。",
            },
        ),
    ])
}

/// The section on parts that several pages embed, and on what one call does
/// when it fails or succeeds.
#[expect(
    dead_code,
    reason = "the Learn group Building the page moves this section to its own page"
)]
fn shared_parts(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Parts shared across pages",
                ja: "複数のページで使う部品",
            },
        ),
        para(
            site,
            L {
                en: "A piece that sits on several pages, such as a sign-out button in an app's shell or a newsletter form in a footer, is a [`Part`](api:hypersig/trait.Part.html): its own signal struct, an element built by `Part::render`, and actions over that struct. A page whose struct has a field of the part's type embeds it with `html::part(Home::footer())`, and the part writes its paths under that field. Each call the part makes names the field in a header, so the one handler registered for the part's action serves every page that embeds it, and the signals it patches and the fragments it renders again land under the field the call came from. Calling the part's action outside the part, or embedding the part at a field of a list item, is a compile error.",
                ja: "アプリの枠にあるサインアウトのボタンや、フッターのニュースレターのフォームのように複数のページに置くものは、[`Part`](api:hypersig/trait.Part.html) として書きます。部品は自分のシグナル構造体と、`Part::render` で組み立てる要素と、その構造体に対するアクションを持ちます。部品の型のフィールドを持つページは `html::part(Home::footer())` で部品を埋め込み、部品はそのフィールドの下にパスを書きます。部品の中の呼び出しはヘッダーでフィールドを伝えるので、部品のアクションに登録したハンドラ 1 つが、部品を埋め込んだどのページにも応答します。ハンドラが書き換えるシグナルと描き直すフラグメントは、呼び出し元のフィールドの下に入ります。部品のアクションを部品の外から呼ぶことと、リストの項目のフィールドに部品を埋め込むことは、コンパイルエラーになります。",
            },
        ),
        show(Snippet::Parts),
        para(
            site,
            L {
                en: "[`Call::on_error`](api:hypersig/struct.Call.html#method.on_error) runs a statement with the error text when that one call fails, and `Call::on_success` runs one after its patches are applied. Both run where the call was made, and `let_in_stmt` names a value when the call is sent, so a page that adds several items before the server answers marks only the item whose save failed. The [`storefront`](https://github.com/imaimai17468/hypersig/tree/main/examples/storefront) example puts one newsletter part on two pages and saves a wish list this way.",
                ja: "[`Call::on_error`](api:hypersig/struct.Call.html#method.on_error) はその呼び出しが失敗したときにエラーの文言を受け取って文を実行し、`Call::on_success` は差分を適用し終えたあとに文を実行します。どちらも呼び出しを書いた場所で動き、`let_in_stmt` で送信時の値に名前を付けられるので、サーバーの応答より先に項目を複数追加するページでも、保存に失敗した項目だけに印を付けられます。[`storefront`](https://github.com/imaimai17468/hypersig/tree/main/examples/storefront) の例は、ニュースレターの部品を 2 つのページに置き、この方法でほしいものリストを保存します。",
            },
        ),
    ])
}

pub fn pages(site: Site) -> Part {
    parts(
        [
            title(Doc::Learn(Learn::Pages).title(site)),
            lead(site, Doc::Learn(Learn::Pages).leads()),
            heading(
                site,
                L {
                    en: "Routes",
                    ja: "ルート",
                },
            ),
            para(
                site,
                L {
                    en: "A [`Route<P>`](api:hypersig/struct.Route.html) is a path whose parameters are `P`. A parameter is a whole segment written `{name}`, as in axum, and `P` is `()` for a path without one, one [`Segment`](api:hypersig/trait.Segment.html) for one, and a tuple for two to six, in path order. A `Segment` is an integer, a [`PathSegment`](api:hypersig/struct.PathSegment.html), or an enum with `#[derive(Segment)]` whose variants have no fields. A `PathSegment` holds any text but an empty one, `.`, and `..`, because axum does not match an empty segment and the browser removes a dot segment from a link. `PathSegment::new` refuses those three at compile time, `PathSegment::parse` returns an error for text read from data, and a request whose segment decodes to one, such as `/orgs/%2E%2E`, gets a 404 before the handler runs. A variant's segment is the string serde writes for it, so `#[serde(rename_all = \"lowercase\")]` gives `/ja/`. A last segment written `{*name}` takes the rest of the path, `/`s included, into a [`Rest`](api:hypersig/struct.Rest.html), as in `\"/files/{*path}\"`. A `Rest` is not empty and has no part that is `.` or `..`, for the same reasons. Declare it in [`routes!`](api:hypersig/macro.routes.html), on a line that names the handler serving it, so a route without a handler cannot be declared. The macro counts the segments, so a path whose count differs from `P` fails to compile. `routes!` needs the `axum` feature, which is on by default.",
                    ja: "[`Route<P>`](api:hypersig/struct.Route.html) は、パラメータが `P` のパスです。パラメータは axum と同じく `{name}` と書いたセグメント 1 つ分です。`P` は、パラメータのないパスでは `()`、1 つなら [`Segment`](api:hypersig/trait.Segment.html) 1 つ、2 つから 6 つならパスの順に並べたタプルです。`Segment` になるのは整数、[`PathSegment`](api:hypersig/struct.PathSegment.html)、そしてフィールドのないヴァリアントだけを持ち `#[derive(Segment)]` を付けた列挙型です。`PathSegment` は、空の文字列、`.`、`..` を除く文字列を持ちます。axum は空のセグメントに一致せず、ブラウザはリンクから `.` と `..` のセグメントを取り除くためです。`PathSegment::new` はこの 3 つをコンパイル時に拒否し、データから読んだ文字列には `PathSegment::parse` がエラーを返します。`/orgs/%2E%2E` のように、デコードするとこの 3 つのどれかになるセグメントを持つリクエストには、ハンドラを呼ぶ前に 404 を返します。列挙型のセグメントは serde がそのヴァリアントに対して書き出す文字列なので、`#[serde(rename_all = \"lowercase\")]` を付けると `/ja/` になります。最後のセグメントを `{*name}` と書くと、`\"/files/{*path}\"` のように、`/` を含むパスの残りを [`Rest`](api:hypersig/struct.Rest.html) で受け取ります。`Rest` も同じ理由で、空にならず、`.` や `..` の部分を持ちません。ルートは [`routes!`](api:hypersig/macro.routes.html) の中で、それを配信するハンドラを書いた行に宣言します。そのため、ハンドラのないルートは宣言できません。マクロがセグメントを数えるので、数が `P` と合わないパスはコンパイルが通りません。`routes!` には `axum` フィーチャーが必要です。このフィーチャーはデフォルトで有効です。",
                },
            ),
            show(Snippet::WrongRoute),
            para(
                site,
                L {
                    en: "The first line of `routes!` names a function and the router state, and the function registers each route on `GET` with its handler. A handler receives the router state and the parameters, already parsed, and returns the `Page`. A path whose segments do not parse into the parameter types gets a 404, and so does a handler that returns `None`, as `member` does below for an id it does not know.",
                    ja: "`routes!` の 1 行目には関数の名前とルーターの状態を書き、その関数が各ルートをハンドラと一緒に `GET` に登録します。ハンドラはルーターの状態と解析済みのパラメータを受け取り、`Page` を返します。セグメントをパラメータの型に解析できないパスには 404 を返し、`None` を返したハンドラにも 404 を返します。下の `member` は知らない id に対して `None` を返します。",
                },
            ),
            program(Program::Pages),
            heading(
                site,
                L {
                    en: "Links",
                    ja: "リンク",
                },
            ),
            para(
                site,
                L {
                    en: "`El::href` takes a [`Link`](api:hypersig/struct.Link.html) for a page the router serves, which `link()` builds for a route without parameters and `link_with(params)` for one with them. The parameters are expressions of the route's types, as the arguments of an action call are. Literals give a fixed `href`. A parameter that reads a signal or a list item, as on the team page above, is evaluated in the browser and kept up to date. Both sides percent-encode a value as `encodeURIComponent` does, so the first HTML and the runtime write the same URL.",
                    ja: "`El::href` は、ルーターが配信するページへの [`Link`](api:hypersig/struct.Link.html) を受け取ります。パラメータのないルートでは `link()` が、パラメータのあるルートでは `link_with(params)` がリンクを作ります。パラメータは、アクションを呼び出すときの引数と同じく、ルートの型の式です。リテラルを渡すと `href` は固定になります。上のチームのページのようにシグナルやリストの項目を読むパラメータは、ブラウザで評価され、値が変わるたびに更新されます。両側とも `encodeURIComponent` と同じ方法で値をパーセントエンコードするので、最初の HTML とランタイムは同じ URL を書きます。",
                },
            ),
            show(Snippet::Links),
            heading(
                site,
                L {
                    en: "Pages under a page",
                    ja: "ページの下のページ",
                },
            ),
            para(
                site,
                L {
                    en: "`in PARENT` after the path declares a route under the page of an earlier line. Its path is the parent's path followed by more segments, and its parameters are the parent's followed by its own, so a link to the child needs the parent's values too. A path that does not continue the parent's, or parameters that do not start with the parent's, fail to compile.",
                    ja: "パスのあとに `in PARENT` と書くと、それより前の行のページの下にルートを宣言します。子のパスは親のパスにセグメントを続けたもので、パラメータは親のパラメータに自分のパラメータを続けたものです。そのため、子へのリンクには親の値も必要です。親のパスの続きになっていないパスや、親のパラメータで始まらないパラメータは、コンパイルが通りません。",
                },
            ),
            show(Snippet::ChildRoutes),
        ]
        .into_iter()
        .chain([
            heading(
                site,
                L {
                    en: "Redirects and navigation",
                    ja: "リダイレクトとページの移動",
                },
            ),
            para(
                site,
                L {
                    en: "The server writes a page's URL with `url(params)`, which takes the route's parameter values themselves and encodes them as a link does. A page handler that sends the browser elsewhere returns a [`PageResponse`](api:hypersig/struct.PageResponse.html), built with `PageResponse::redirect(url)` or converted from a `Page`. An action does the same with `Patches::navigate(url)`, which the browser follows once the patches before it are applied.",
                    ja: "サーバーは `url(params)` でページの URL を書きます。`url` はルートのパラメータの値そのものを受け取り、リンクと同じ方法でエンコードします。ブラウザを別のページへ送るページのハンドラは、[`PageResponse`](api:hypersig/struct.PageResponse.html) を返します。`PageResponse` は `PageResponse::redirect(url)` で作るか、`Page` から変換します。アクションでは `Patches::navigate(url)` で同じことをし、ブラウザはそれより前の差分を適用してから移動します。",
                },
            ),
            para(
                site,
                L {
                    en: "In the browser, `link.navigate()` is a statement that loads the linked page, for a handler that should leave the page after it runs other statements.",
                    ja: "ブラウザでは、`link.navigate()` がリンク先のページを読み込む文です。ほかの文を実行してからページを離れるハンドラに使います。",
                },
            ),
            show(Snippet::Navigation),
            heading(
                site,
                L {
                    en: "Error pages",
                    ja: "エラーのページ",
                },
            ),
            para(
                site,
                L {
                    en: "A page handler that cannot build its page, such as one whose database does not answer, returns an error page of its own with [`PageResponse::with_status(status, page)`](api:hypersig/struct.PageResponse.html#method.with_status). The browser shows the page, and the status tells a crawler or a monitor that the request failed. A handler that returns `PageResponse` converts its usual page with `.into()`. The page for a path no route matches is named once in `routes!`, as [Routes in depth](learn/routes/#pages-for-missing-paths) shows.",
                    ja: "データベースが応答しないときのように、ページを組み立てられなかったハンドラは、[`PageResponse::with_status(status, page)`](api:hypersig/struct.PageResponse.html#method.with_status) で自分のエラーのページを返します。ブラウザはそのページを表示し、ステータスはクローラーや監視の仕組みにリクエストの失敗を伝えます。`PageResponse` を返すハンドラは、通常のページを `.into()` で変換します。どのルートにも合わないパスのページは、[ルートの詳細](learn/routes/#pages-for-missing-paths) のとおり `routes!` の中で 1 回だけ指定します。",
                },
            ),
            show(Snippet::ErrorPage),
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
                        en: "A parent after `in` is a page itself, so a path prefix with no page of its own cannot be one.",
                        ja: "`in` のあとに書く親はそれ自体がページなので、自分のページを持たないパスの接頭辞は親にできません。",
                    },
                    L {
                        en: "A link is a plain `<a href>`, so following it loads the next page from the server, and the signals of the page you leave are gone.",
                        ja: "リンクは通常の `<a href>` なので、たどると次のページをサーバーから読み込み、離れたページのシグナルは失われます。",
                    },
                    L {
                        en: "A link to another site or to a `#fragment` is a [`SafeUrl`](api:hypersig/struct.SafeUrl.html), which [When bindings and statements are not enough](learn/escape-hatches/) covers. It checks the URL's scheme, and nothing checks that the address it names exists.",
                        ja: "ほかのサイトや `#fragment` へのリンクには [`SafeUrl`](api:hypersig/struct.SafeUrl.html) を使います。これは [束縛と文で足りないとき](learn/escape-hatches/) で説明します。`SafeUrl` が検査するのは URL のスキームだけで、その先のアドレスがあるかどうかは検査されません。",
                    },
                ],
            ),
            para(
                site,
                L {
                    en: "[Routes in depth](learn/routes/) covers typed query strings, URLs that change without loading a page, pages for missing paths, what search results and link previews show, and an application served under a path prefix.",
                    ja: "型の付いたクエリ文字列、ページを読み込まずに変える URL、存在しないパスのページ、検索結果とリンクのプレビューに出る内容、パスの接頭辞の下で配信するアプリは [ルートの詳細](learn/routes/) で説明します。",
                },
            ),
        ]),
    )
}

/// The section on what a page tells search engines and link previews,
/// apart from `pages` because clippy rejects an array of parts larger than
/// 16 KiB on the stack.
fn head(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Search results and link previews",
                ja: "検索結果とリンクのプレビュー",
            },
        ),
        para(
            site,
            L {
                en: "`Page::description` writes the summary a search result shows, and `Page::icon` the icon of the tab. A canonical URL and the image of a link preview must be absolute, so `Page::canonical` and `OpenGraph::image` join a path to an [`Origin`](api:hypersig/struct.Origin.html), whose constant fails to compile when it holds more than the scheme, the host, and the port. An application that a proxy serves under a path such as `/blog` adds that path with `Origin::under`, which rejects a malformed prefix the same way. [`OpenGraph`](api:hypersig/struct.OpenGraph.html) takes the card's title, text, and URL from the page, so each is written once.",
                ja: "`Page::description` は検索結果に出る要約を、`Page::icon` はタブのアイコンを書きます。canonical の URL とリンクのプレビューの画像は絶対 URL でなければならないので、`Page::canonical` と `OpenGraph::image` はパスを [`Origin`](api:hypersig/struct.Origin.html) につなぎます。`Origin` の定数は、スキーム、ホスト、ポート以外を含むとコンパイルエラーになります。プロキシが `/blog` のようなパスの下に配信するアプリは、`Origin::under` でそのパスを追加します。`under` も、形の正しくない接頭辞を同じようにコンパイルエラーにします。[`OpenGraph`](api:hypersig/struct.OpenGraph.html) はカードのタイトル、本文、URL をページから取るので、どれも 1 回書くだけで済みます。",
            },
        ),
        show(Snippet::Head),
        para(
            site,
            L {
                en: "`Page::bind_title(expr)` makes the tab's title follow a `String` expression, such as an unread count. The server writes its first value into `<title>`, and the runtime writes each later value into `document.title`. The card of `OpenGraph` keeps the title given to `Page::new`.",
                ja: "`Page::bind_title(expr)` は未読の件数のように変わる `String` の式にタブのタイトルを合わせます。最初の値はサーバーが `<title>` に書き、その後の値はランタイムが `document.title` に書きます。`OpenGraph` のカードには `Page::new` に渡したタイトルが保持されます。",
            },
        ),
    ])
}

pub fn styling(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Styling).title(site)),
        lead(site, Doc::Learn(Learn::Styling).leads()),
        heading(
            site,
            L {
                en: "Atomic styles",
                ja: "アトミックなスタイル",
            },
        ),
        para(
            site,
            L {
                en: "[`Style`](api:hypersig/style/struct.Style.html) has one method per standard CSS property, generated from MDN's CSS data. `El::css` adds each declaration as one class named after the declaration, as in the atomic CSS of Panda CSS and Tailwind CSS. The page writes the rules its elements use into `<head>`, and a patch that contains new elements includes their rules.",
                ja: "[`Style`](api:hypersig/style/struct.Style.html) には、標準の CSS プロパティごとに 1 つずつメソッドがあり、MDN の CSS データから生成しています。`El::css` は、Panda CSS や Tailwind CSS のアトミック CSS と同じく、宣言ごとにその宣言にちなんだ名前のクラスを 1 つ付けます。ページは要素が使うルールを `<head>` に書き、新しい要素を含む差分には、その要素のルールも含まれます。",
            },
        ),
        show(Snippet::Styles),
        bullets(
            site,
            &[
                L {
                    en: "`on(PseudoClass::Hover, ...)` and `on(Media::Md, ...)` apply a style under a condition. Conditions cover pseudo-classes and pseudo-elements, media features, container sizes, and the state of a marked ancestor or sibling (`group` and `peer`).",
                    ja: "`on(PseudoClass::Hover, ...)` や `on(Media::Md, ...)` は条件付きでスタイルを適用します。条件には、疑似クラスと疑似要素、メディア特性、コンテナのサイズ、印を付けた祖先や兄弟の状態（`group` と `peer`）があります。",
                },
                L {
                    en: "`css_when(style, cond)` adds a style while a signal condition holds, and `css_match(value, cases)` picks one style per value of a signal.",
                    ja: "`css_when(style, cond)` はシグナルの条件が成り立つ間だけスタイルを付け、`css_match(value, cases)` はシグナルの値ごとにスタイルを 1 つ選びます。",
                },
                L {
                    en: "A `Token` is a CSS custom property with a typed value. `Page::theme` sets tokens for the page, for example under `Media::Dark`.",
                    ja: "`Token` は型の付いた値を持つ CSS カスタムプロパティです。`Page::theme` はたとえば `Media::Dark` のもとでのトークンをページに設定します。",
                },
                L {
                    en: "`Global` holds rules for elements by tag with a small reset, `@font-face`, and `@keyframes`. `hypersig::style::pattern` has layouts such as `stack` and `grid`.",
                    ja: "`Global` はタグで指定する要素のルールを、小さなリセット、`@font-face`、`@keyframes` と一緒に持ちます。`hypersig::style::pattern` には `stack` や `grid` のようなレイアウトがあります。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The badge below takes its color from `css_match` over an enum, and the green comes from a token with a dark value.",
                ja: "下のバッジは、列挙型に対する `css_match` で色が決まります。緑はダークモード用の値を持つトークンの色です。",
            },
        ),
        demo::show(site, Demo::Status),
        heading(
            site,
            L {
                en: "Values the types do not check",
                ja: "型が検査しない値",
            },
        ),
        para(
            site,
            L {
                en: "A value with several parts, such as `margin: 0 auto`, a shadow, or a `calc()`, goes through `raw(...)`, which is passed to the browser unchecked, and so is a value outside a property's range, such as a negative width. A page that loads an existing stylesheet keeps it: `Stylesheet::class(\"name\")` checks at compile time that the stylesheet has a selector for the class.",
                ja: "`margin: 0 auto`、影、`calc()` のように複数の部分からなる値は `raw(...)` で書き、検査されないままブラウザに渡ります。負の幅のように、プロパティの範囲を外れた値も検査されません。既存のスタイルシートを読み込むページは、そのまま使い続けられます。`Stylesheet::class(\"name\")` は、そのスタイルシートにクラスのセレクタがあることをコンパイル時に検査します。",
            },
        ),
        para(
            site,
            L {
                en: "The [`style`](https://github.com/imaimai17468/hypersig/tree/main/examples/style) example in the repository puts each feature on one page, and the [`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) example shows the components of `hypersig_ui`, which are built with these features.",
                ja: "リポジトリの [`style`](https://github.com/imaimai17468/hypersig/tree/main/examples/style) の例はそれぞれの機能を 1 つのページにまとめています。[`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) の例は、これらの機能を使って作った `hypersig_ui` のコンポーネントを示します。",
            },
        ),
    ])
}

pub fn escape_hatches(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::EscapeHatches).title(site)),
        lead(site, Doc::Learn(Learn::EscapeHatches).leads()),
        heading(
            site,
            L {
                en: "Node refs",
                ja: "ノード参照",
            },
        ),
        para(
            site,
            L {
                en: "A [`NodeRef`](api:hypersig/struct.NodeRef.html) names one element for a DOM call that no binding covers. Declare it with [`node_ref!`](api:hypersig/macro.node_ref.html), attach it with `El::node_ref`, and use its statements: `focus`, `blur`, `select`, `scroll_into_view`, `show_popover`, and `hide_popover`. `node_ref!` names the constant after its module and identifier, so two constants never share a name. Each item of a list may carry the same constant. One constant on any other two elements, or one fixed id from `El::id` on two elements, makes `Page::render` panic, so a test that renders the page finds the second. The reset button below stops the stopwatch and moves focus to the start button.",
                ja: "[`NodeRef`](api:hypersig/struct.NodeRef.html) は、束縛では扱えない DOM の呼び出しのために要素を 1 つ指す名前です。[`node_ref!`](api:hypersig/macro.node_ref.html) で宣言し、`El::node_ref` で要素に付け、`focus`、`blur`、`select`、`scroll_into_view`、`show_popover`、`hide_popover` の文を使います。`node_ref!` はモジュールと識別子から定数の名前を作るので、2 つの定数が同じ名前になることはありません。リストの項目ごとに同じ定数を付けるのはかまいません。それ以外で 2 つの要素に同じ定数や、`El::id` で同じ固定の id を付けると `Page::render` が panic するので、ページを描画するテストで見つかります。下のリセットボタンはストップウォッチを止め、フォーカスをスタートボタンに移します。",
            },
        ),
        demo::show(site, Demo::Stopwatch),
        heading(
            site,
            L {
                en: "Timers and reactions",
                ja: "タイマーと変化への反応",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`on_interval(period, stmt)` runs a statement while the element is in the document, and `on_timeout(delay, stmt)` runs one once.",
                    ja: "`on_interval(period, stmt)` は要素が文書にある間、文を繰り返し実行し、`on_timeout(delay, stmt)` は文を 1 回だけ実行します。",
                },
                L {
                    en: "`on_load(stmt)` runs once when the element enters the document.",
                    ja: "`on_load(stmt)` は、要素が文書に入ったときに 1 回実行します。",
                },
                L {
                    en: "`effect(stmt)` runs when the element enters the document and again whenever a signal the statement reads changes.",
                    ja: "`effect(stmt)` は、要素が文書に入ったときと、文が読むシグナルが変わるたびに実行します。",
                },
                L {
                    en: "`on_signal_change(sig, stmt)` runs whenever the signal, or any field inside it, changes, whatever changed it.",
                    ja: "`on_signal_change(sig, stmt)` は、シグナルかその中のフィールドが変わるたびに、何が変えたかにかかわらず実行します。",
                },
                L {
                    en: "`Expr::now()` reads the browser's clock, and `Expr::random_below(n)` returns a random integer.",
                    ja: "`Expr::now()` はブラウザの時計を読み、`Expr::random_below(n)` はランダムな整数を返します。",
                },
            ],
        ),
        show(Snippet::Effects),
        heading(
            site,
            L {
                en: "Keyboard patterns",
                ja: "キーボード操作のパターン",
            },
        ),
        para(
            site,
            L {
                en: "`roving_focus` moves focus between the items inside an element with the arrow keys, Home, and End, as a tab list or a menu needs. `active_descendant` keeps focus on a text field while the up and down arrows move through the options of a listbox, as a combobox needs. `hypersig_ui` builds its tabs, menus, listbox, and combobox with them.",
                ja: "`roving_focus` は、タブの一覧やメニューが必要とするように、矢印キーと Home、End で要素の中の項目間にフォーカスを動かします。`active_descendant` は、コンボボックスが必要とするように、フォーカスをテキスト欄に置いたまま上下の矢印キーでリストボックスの選択肢を移動させます。`hypersig_ui` のタブ、メニュー、リストボックス、コンボボックスは、この 2 つを使って作っています。",
            },
        ),
        heading(
            site,
            L {
                en: "Links the router does not serve",
                ja: "ルーターが配信しないページへのリンク",
            },
        ),
        para(
            site,
            L {
                en: "`El::href` and `El::src` take a link built from a [`Route`](learn/pages/), or a [`SafeUrl`](api:hypersig/struct.SafeUrl.html) for an address the router does not serve, such as another site or a `#fragment` on the same page. A `SafeUrl` holds only a URL whose scheme cannot run a script: `http:`, `https:`, `mailto:`, or none at all. `SafeUrl::new` takes a URL written in the program, and in a `const` a `javascript:` URL fails to compile. `SafeUrl::parse` reads a URL from outside the program, such as one a visitor typed, and returns an [`UnsafeUrl`](api:hypersig/struct.UnsafeUrl.html) error where `SafeUrl::new` would refuse it. `attr` panics on `href` and `src`, so neither can be written from a plain string.",
                ja: "`El::href` と `El::src` は、[`Route`](learn/pages/) から作ったリンクのほかに、ルーターが配信しないアドレスのための [`SafeUrl`](api:hypersig/struct.SafeUrl.html) を受け取ります。ほかのサイトや、同じページの `#fragment` へのリンクがこれにあたります。`SafeUrl` に入るのは、スクリプトを実行できないスキーム（`http:`、`https:`、`mailto:`）の URL と、スキームのない URL だけです。`SafeUrl::new` はプログラムに書いた URL を受け取り、`const` の中では `javascript:` の URL がコンパイルエラーになります。`SafeUrl::parse` は訪問者が入力したものなど、プログラムの外から来た URL を読み、`SafeUrl::new` が受け付けない URL には [`UnsafeUrl`](api:hypersig/struct.UnsafeUrl.html) のエラーを返します。`attr` は `href` と `src` を渡すとパニックするので、どちらも素の文字列からは書けません。",
            },
        ),
        show(Snippet::SafeUrl),
        para(
            site,
            L {
                en: "The host is not checked, so `https://evil.example/a.js` is a `SafeUrl` too. A visitor's URL suits a link the visitor follows, and not the `src` of an `<iframe>`, which would show whatever page the host serves inside this one.",
                ja: "ホストは検査しないので、`https://evil.example/a.js` も `SafeUrl` です。訪問者の URL は訪問者自身がたどるリンクには使えますが、`<iframe>` の `src` には使えません。そのホストが配信するページを、このページの中にそのまま表示するからです。",
            },
        ),
        heading(
            site,
            L {
                en: "Static pages",
                ja: "静的なページ",
            },
        ),
        para(
            site,
            L {
                en: "A page that calls no action needs no server. `Page::runtime_src` loads the runtime from another URL, and `RUNTIME_JS` holds its source to write next to the page. `Page::render_under` renders a page for a site served under a `Base`, such as a GitHub Pages project site at `/hypersig`. This site is built that way: a Rust program renders each page with hypersig and writes the files that GitHub Pages serves. A static file carries none of the headers a served page has, `X-Frame-Options` and `frame-ancestors` among them, and a `<meta>` cannot carry `frame-ancestors`, so set them in the host's configuration where the host allows it.",
                ja: "アクションを呼び出さないページにはサーバーは不要です。`Page::runtime_src` は別の URL からランタイムを読み込み、`RUNTIME_JS` はページの隣に書き出すランタイムのソースを持ちます。`Page::render_under` は、`/hypersig` にある GitHub Pages のプロジェクトサイトのように `Base` の下で配信するサイトのページを描画します。このサイトもその方法で作っています。Rust のプログラムが hypersig で各ページを描画し、GitHub Pages が配信するファイルを書き出します。静的なファイルには、配信するページに付く `X-Frame-Options` や `frame-ancestors` のヘッダーが付きません。`<meta>` では `frame-ancestors` を指定できないので、ヘッダーを設定できるホストではホストの設定で送ってください。",
            },
        ),
        show(Snippet::StaticPage),
        heading(
            site,
            L {
                en: "Where hypersig does not fit",
                ja: "hypersig が向かない場面",
            },
        ),
        para(
            site,
            L {
                en: "The server holds the state, and the browser runs only the operations hypersig provides and the scripts a page loads. It does not suit these applications:",
                ja: "状態はサーバーが持ち、ブラウザは hypersig が用意した操作と、ページが読み込んだスクリプトしか実行しません。そのため、次のようなアプリケーションには向きません。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "An application whose main work runs inside a JavaScript library, such as a drawing tool or a spreadsheet. hypersig only passes values in and out of the library, as [JavaScript libraries](learn/javascript/) shows, and the state the library keeps is not among the page's signals.",
                    ja: "作図ツールや表計算ソフトのように、主な処理が JavaScript のライブラリの中で動くアプリケーション。hypersig は [JavaScript のライブラリ](learn/javascript/) で示すようにライブラリと値をやり取りするだけで、ライブラリが持つ状態はページのシグナルに入りません。",
                },
                L {
                    en: "An application that works without a connection to the server, such as one used offline.",
                    ja: "オフラインで使うものなど、サーバーにつながらなくても動くアプリケーション。",
                },
                L {
                    en: "A page whose buttons and forms must work with JavaScript turned off. A page cannot set the `action` or `formaction` attribute, so a form calls an action only through the runtime, which keeps the browser from submitting any form itself. A browser that does not run the runtime shows the first HTML, and submitting a form there loads the page's own URL again with the form's named fields in the query, which runs no action.",
                    ja: "JavaScript を無効にしても、ボタンやフォームが動く必要のあるページ。ページには `action` 属性も `formaction` 属性も書けないので、フォームからアクションを呼べるのはランタイムだけです。ランタイムは、ブラウザがフォームを自分で送ることも止めます。ランタイムを実行しないブラウザは最初の HTML を表示します。そこでフォームを送ると、名前の付いたフィールドをクエリに入れて同じページの URL を読み込み直すだけで、アクションは実行されません。",
                },
            ],
        ),
    ])
}
