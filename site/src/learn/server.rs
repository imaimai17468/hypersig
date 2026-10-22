//! The pages on talking to the server, styling, and what to use when bindings
//! and statements are not enough.

use super::snippet::{Snippet, show};
use super::{Program, program};
use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::prose::{Part, bullets, heading, lead, note, para, parts, subheading, title};

pub fn server(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Server).title(site)),
        lead(site, Doc::Learn(Learn::Server).leads()),
        heading(
            site,
            L {
                en: "Actions",
                ja: "アクション",
            },
        ),
        para(
            site,
            L {
                en: "An [`Action<S, A>`](api:hypersig/struct.Action.html) is a server endpoint for the page whose signals are `S`, receiving arguments of type `A`. Declare it as a `const` and use the same value on both sides. The page calls it with `call()`, or `call_with(args)` where `args` is an expression evaluated in the browser when the call runs, and the router serves it with `RouterExt::action`. A handler that needs several values takes a tuple, which the page passes as a tuple of expressions.",
                ja: "[`Action<S, A>`](api:hypersig/struct.Action.html) は、シグナルが `S` のページのためのサーバーのエンドポイントで、型 `A` の引数を受け取ります。`const` として宣言し、同じ値を両側で使います。ページは `call()` か `call_with(args)` で呼び出し、ルーターは `RouterExt::action` で配信します。`args` は、呼び出しを実行するときにブラウザで評価される式です。いくつもの値が要るハンドラはタプルを受け取り、ページは式のタプルを渡します。",
            },
        ),
        para(
            site,
            L {
                en: "The handler receives the router state and the arguments, already deserialized, and returns anything that converts into `Patches<S>`. A request whose arguments do not deserialize gets a 400 before the handler runs.",
                ja: "ハンドラはルーターの状態と、デシリアライズ済みの引数を受け取り、`Patches<S>` に変換できる値を返します。引数をデシリアライズできないリクエストには、ハンドラが動く前に 400 を返します。",
            },
        ),
        program(Program::Actions),
        subheading(
            site,
            L {
                en: "Busy and failed calls",
                ja: "実行中の呼び出しと失敗した呼び出し",
            },
        ),
        para(
            site,
            L {
                en: "`indicator(flag)` sets a `bool` signal to true while the request waits for its response, which the example uses to disable the button. `error(message)` sets a `String` signal to the error text when the call fails, which is a network error or a response that is not a success, and to `\"\"` each time the call is sent, so the same failure twice changes the signal twice.",
                ja: "`indicator(flag)` は、リクエストの応答を待つ間 `bool` のシグナルを真にします。この例ではそれを使ってボタンを無効にしています。`error(message)` は、呼び出しが失敗したときに `String` のシグナルへエラーの文を入れ、呼び出しを送るたびに `\"\"` に戻します。失敗とはネットワークのエラーか、成功ではない応答のことです。戻してから入れ直すので、同じ失敗が 2 回続いてもシグナルは 2 回変わります。",
            },
        ),
        subheading(
            site,
            L {
                en: "Optimistic updates",
                ja: "楽観的更新",
            },
        ),
        para(
            site,
            L {
                en: "A statement before the call changes the page at once, and the request follows: `count.set(count + 1).then(LIKE.call())`. The handler's patch then sets the count the server stored. When the server answers with an error, the change stays in the browser. `error` can show the failure, but nothing rolls the value back.",
                ja: "呼び出しの前に文を置くと、ページがすぐに変わり、そのあとにリクエストが送られます。`count.set(count + 1).then(LIKE.call())` のように書きます。続いて、ハンドラの差分がサーバーに保存したカウントを設定します。サーバーがエラーで答えた場合、変更はブラウザに残ります。`error` で失敗を表示できますが、値を元に戻す処理はありません。",
            },
        ),
        heading(
            site,
            L {
                en: "Patches",
                ja: "差分",
            },
        ),
        para(
            site,
            L {
                en: "[`Patches<S>`](api:hypersig/struct.Patches.html) is a batch the runtime applies in order. `sig.patch(value)` sets one signal, `and` adds more to the same event, and `Patches::new().all_signals(&state)` replaces all of them. Only a page-rooted signal has `patch`, so the server addresses fields by name and never by a list position.",
                ja: "[`Patches<S>`](api:hypersig/struct.Patches.html) は、ランタイムが順に適用する差分のまとまりです。`sig.patch(value)` はシグナルを 1 つ設定し、`and` は同じイベントに差分を足し、`Patches::new().all_signals(&state)` はすべてのシグナルを置き換えます。`patch` を持つのはページを起点とするシグナルだけなので、サーバーはフィールドを名前で指し、リストの位置では指しません。",
            },
        ),
        show(Snippet::Patches),
        para(
            site,
            L {
                en: "Each patch is sent as one Server-Sent Event. `to_event_stream` returns the body for a server that does not use axum.",
                ja: "差分は 1 つずつ Server-Sent Event として送られます。axum を使わないサーバーには、`to_event_stream` がレスポンスの本文を返します。",
            },
        ),
        subheading(
            site,
            L {
                en: "List patches",
                ja: "リストの差分",
            },
        ),
        para(
            site,
            L {
                en: "`patch` sends a list whole, so a chat that sends each message with `patch` sends every earlier one again. [`ListPatch`](api:hypersig/struct.ListPatch.html) sends only the items that change: `patch_append` and `patch_prepend` add items at either end of the list the browser holds, `patch_upsert` replaces the item with the same key or appends it, and `patch_remove` removes the items with the given keys. The key is an expression of the item, as in `each_keyed`, and the browser computes it for each item. As with `patch`, only a page-rooted `Vec` signal has these methods.",
                ja: "`patch` はリストを丸ごと送るので、チャットのメッセージを 1 件ずつ `patch` で送ると、それまでのメッセージも毎回送り直すことになります。[`ListPatch`](api:hypersig/struct.ListPatch.html) は変わる項目だけを送ります。`patch_append` と `patch_prepend` はブラウザが持つリストの末尾と先頭に項目を加え、`patch_upsert` は同じキーの項目を置き換えるか末尾に加え、`patch_remove` は指定したキーの項目を取り除きます。キーは `each_keyed` と同じく項目の式で、ブラウザが項目ごとに計算します。`patch` と同じく、これらのメソッドはページを起点とする `Vec` のシグナルにだけあります。",
            },
        ),
        show(Snippet::ListPatches),
        para(
            site,
            L {
                en: "Items added before an `each_keyed` list keep the items already on screen where they were: the runtime scrolls the nearest scrollable ancestor by the height of the new items. Chrome's scroll anchoring does the same once the list is scrolled, but not at the very top, which is where a list that loads older items is when it loads them. `Sig::prepend` does the same in the browser. A list drawn with `each` matches its elements by position, so its first element shows the first new item and nothing scrolls.",
                ja: "`each_keyed` のリストの先頭に項目を加えると、画面に出ていた項目はその位置に残ります。ランタイムが、いちばん近いスクロールできる祖先要素を、新しい項目の高さの分だけスクロールします。Chrome のスクロールアンカリングもリストをスクロールした後なら同じことをしますが、いちばん上では働きません。古い項目を読み込むリストは、ちょうどいちばん上にあるときに読み込みます。ブラウザの中では `Sig::prepend` が同じように動きます。`each` で描いたリストは要素を位置で対応させるので、先頭の要素に新しい最初の項目が表示され、スクロールはしません。",
            },
        ),
        fragments(site),
        calls(site),
        shared_parts(site),
    ])
}

/// The sections on calls that overlap and streams that drop.
fn calls(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Calls that overlap",
                ja: "重なった呼び出し",
            },
        ),
        para(
            site,
            L {
                en: "Each call applies its patches when they arrive, so the answer to an earlier, slower search can overwrite the answer to a later one. [`Call::latest`](api:hypersig/struct.Call.html#method.latest) aborts the unfinished earlier `latest` call to the same action, and the patches it has not received are dropped. Calls from different elements to one action share it, because the handler decides which signals they patch. The server may stop the aborted handler at an `.await`, so mark only reads. A call without `latest` is never aborted, and an indicator stays true until every call that sets it has its answer.",
                ja: "呼び出しは差分が届いた順に反映するので、前に送った遅い検索の答えが、あとの検索の答えを上書きすることがあります。[`Call::latest`](api:hypersig/struct.Call.html#method.latest) は、同じアクションへの `latest` の呼び出しで、まだ終わっていない前のものを取り消します。取り消した呼び出しの、まだ届いていない差分は捨てます。アクションがどのシグナルに差分を返すかはハンドラが決めるので、別々の要素から同じアクションを呼んでも、取り消しの対象は共通です。取り消されたハンドラはサーバーで `.await` の時点で止まることがあるので、`latest` は読み取りだけに付けてください。`latest` のない呼び出しは取り消されません。indicator は、それを使う呼び出しがすべて答えを受け取るまで真のままです。",
            },
        ),
        show(Snippet::Latest),
        heading(
            site,
            L {
                en: "Streams that drop",
                ja: "切れたストリーム",
            },
        ),
        para(
            site,
            L {
                en: "[`StreamCall::reconnect`](api:hypersig/struct.StreamCall.html#method.reconnect) opens a stream again when its connection fails, it breaks off, or the server answers with a 5xx status, as the clock above does. It waits 1 second, then twice as long each time up to 30 seconds. The server sends the event `hs-end` last once the handler's stream finishes, so a response a host closes at its time limit, which ends before `hs-end`, counts as broken off. A stream that ended with `hs-end`, a 4xx answer, and an element that has left the page stop it. Each attempt evaluates the arguments again, so a stream that takes a signal its patches keep current, such as the id of the last message, can send only what came after it. A stream whose arguments stay the same should start with the current values, and the page catches up with what it missed. Only a call to a `StreamAction`, which is always served on `GET`, has `reconnect`, because sending a `POST` again could repeat a change.",
                ja: "[`StreamCall::reconnect`](api:hypersig/struct.StreamCall.html#method.reconnect) は、接続が失敗したとき、ストリームが途中で切れたとき、サーバーが 5xx を返したときに、ストリームを開き直します。上の時計がこの例です。1 秒待ち、そのあとは倍々に最大 30 秒まで待ちます。サーバーはハンドラのストリームが終わると最後に `hs-end` のイベントを送ります。ホストが時間の上限で閉じたレスポンスは `hs-end` の前で終わるので、途中で切れたものとして扱います。`hs-end` で終わったストリーム、4xx の答え、要素がページから外れたときには送り直しません。送り直すたびに引数を評価し直します。最後に受け取ったメッセージの id のように、ストリームの差分が更新し続けるシグナルを引数に渡せば、ハンドラはそれより後の分だけを送れます。引数が変わらないストリームでは、初めに現在の値を送ってください。ページは見逃した変更に追いつきます。`POST` を送り直すと変更を繰り返しうるので、`reconnect` があるのは、常に `GET` で配信する `StreamAction` の呼び出しだけです。",
            },
        ),
    ])
}

/// The section on parts that several pages embed, and on what one call does
/// when it fails or succeeds.
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
                en: "A piece that sits on several pages, such as a sign-out button in an app's shell or a newsletter form in a footer, is a [`Part`](api:hypersig/trait.Part.html): its own signal struct, an element built by `Part::render`, and actions over that struct. A page whose struct has a field of the part's type embeds it with `html::part(Shop::footer())`, and the part writes its paths under that field. Each call the part makes names the field in a header, so the one handler registered for the part's action serves every page that embeds it, and the signals it patches and the fragments it renders again land under the field the call came from. Calling the part's action outside the part, or embedding the part at a field of a list item, is a compile error.",
                ja: "アプリの枠にあるサインアウトのボタンや、フッターのニュースレターのフォームのように複数のページに置くものは、[`Part`](api:hypersig/trait.Part.html) として書きます。部品は自分のシグナル構造体と、`Part::render` で組み立てる要素と、その構造体に対するアクションを持ちます。部品の型のフィールドを持つページは `html::part(Shop::footer())` で部品を埋め込み、部品はそのフィールドの下にパスを書きます。部品の中の呼び出しはヘッダーでフィールドを伝えるので、部品のアクションに登録したハンドラ 1 つが、部品を埋め込んだどのページにも応答します。ハンドラが書き換えるシグナルと描き直すフラグメントは、呼び出し元のフィールドの下に入ります。部品のアクションを部品の外から呼ぶことと、リストの項目のフィールドに部品を埋め込むことは、コンパイルエラーになります。",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::on_error`](api:hypersig/struct.Call.html#method.on_error) runs a statement with the error text when that one call fails, and `Call::on_success` runs one after its patches are applied. Both run where the call was made, and `let_in_stmt` names a value when the call is sent, so a page that adds several items before the server answers marks only the item whose save failed. The [`storefront`](https://github.com/imaimai17468/hypersig/tree/main/examples/storefront) example puts one newsletter part on two pages and saves a wish list this way.",
                ja: "[`Call::on_error`](api:hypersig/struct.Call.html#method.on_error) はその呼び出しが失敗したときにエラーの文言を受け取って文を実行し、`Call::on_success` は差分を適用し終えたあとに文を実行します。どちらも呼び出しを書いた場所で動き、`let_in_stmt` で送信時の値に名前を付けられるので、サーバーの応答より先に項目を複数足すページでも、保存に失敗した項目だけに印を付けられます。[`storefront`](https://github.com/imaimai17468/hypersig/tree/main/examples/storefront) の例は、ニュースレターの部品を 2 つのページに置き、この方法でほしいものリストを保存します。",
            },
        ),
    ])
}

/// The sections on fragments and streams, apart from `server` because
/// clippy rejects an array of parts larger than 16 KiB on the stack.
fn fragments(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Fragments",
                ja: "フラグメント",
            },
        ),
        para(
            site,
            L {
                en: "A [`Fragment`](api:hypersig/trait.Fragment.html) is a part of the page the server can render again on its own. It owns its element id, so the id in the first page and the id in a later patch cannot differ. `fragment(&value)` renders it in the page, and `Patches::new().element(&value)` sends it again. The runtime compares the HTML it receives with the element that has that id and rewrites only the parts that differ. A patch includes the CSS rules its elements use. The server renders a fragment sent again on its own, so it cannot compare it with the rest of the page. The browser does once the patch is applied, and writes a warning to the console when the patch adds an id or a node ref name that another element already carries.",
                ja: "[`Fragment`](api:hypersig/trait.Fragment.html) は、サーバーが単独で描画し直せるページの一部です。フラグメントが要素の id を持つので、最初のページの id とあとの差分の id が食い違うことはありません。`fragment(&value)` はページの中にフラグメントを描画し、`Patches::new().element(&value)` はそれをもう一度送ります。ランタイムは送られた HTML とその id の要素を突き合わせ、違う部分だけを書き換えます。差分にはその要素が使う CSS のルールも含まれます。サーバーは送り直すフラグメントを単独で描画するので、ページの残りの部分とは比べられません。そこでブラウザが差分を当てたあとに比べ、ほかの要素がすでに持つ id や node ref の名前を差分が足したときは、コンソールに警告を書きます。",
            },
        ),
        program(Program::Fragments),
        heading(
            site,
            L {
                en: "Streams",
                ja: "ストリーム",
            },
        ),
        para(
            site,
            L {
                en: "A [`StreamAction`](api:hypersig/struct.StreamAction.html) served with `RouterExt::stream` runs a handler that returns a stream of patches, on `GET`. The response stays open until the stream ends or the browser leaves the page. Call it from `on_load` to send changes made elsewhere, such as by other users.",
                ja: "[`StreamAction`](api:hypersig/struct.StreamAction.html) を `RouterExt::stream` で配信すると、差分のストリームを返すハンドラが `GET` で動きます。レスポンスは、ストリームが終わるかブラウザがページを離れるまで開いたままです。ほかのユーザーなど別の場所で起きた変更を送るには、`on_load` から呼び出してください。",
            },
        ),
        program(Program::Streams),
        note(
            site,
            L {
                en: "These programs need a server, so this site shows them as code. Each is built by `cargo test` from `site/examples/` in the repository.",
                ja: "これらのプログラムにはサーバーが要るので、このサイトではコードとして示します。どれもリポジトリの `site/examples/` にあり、`cargo test` でビルドしています。",
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
                    en: "A [`Route<P>`](api:hypersig/struct.Route.html) is a path whose parameters are `P`. A parameter is a whole segment written `{name}`, as in axum, and `P` is `()` for a path without one, one [`Segment`](api:hypersig/trait.Segment.html) for one, and a tuple for two to six, in path order. A `Segment` is an integer, a `String`, or an enum whose variants have no fields with `#[derive(Segment)]`, whose segment is the string serde writes for the variant, so `#[serde(rename_all = \"lowercase\")]` gives `/ja/`. A last segment written `{*name}` takes the rest of the path, `/`s included, into a [`Rest`](api:hypersig/struct.Rest.html), as in `\"/files/{*path}\"`. Declare it in [`routes!`](api:hypersig/macro.routes.html), on a line that names the handler serving it, so a route without a handler cannot be declared. The macro counts the segments, so a path whose count differs from `P` fails to compile. `routes!` needs the `axum` feature, which is on by default.",
                    ja: "[`Route<P>`](api:hypersig/struct.Route.html) は、パラメータが `P` のパスです。パラメータは axum と同じく `{name}` と書いたセグメント 1 つ分です。`P` は、パラメータのないパスでは `()`、1 つなら [`Segment`](api:hypersig/trait.Segment.html) 1 つ、2 つから 6 つならパスの順に並べたタプルです。`Segment` になるのは整数、`String`、そしてフィールドのないヴァリアントだけを持ち `#[derive(Segment)]` を付けた列挙型です。列挙型のセグメントは serde がそのヴァリアントに書く文字列なので、`#[serde(rename_all = \"lowercase\")]` を付けると `/ja/` になります。最後のセグメントを `{*name}` と書くと、`\"/files/{*path}\"` のように、`/` を含むパスの残りを [`Rest`](api:hypersig/struct.Rest.html) に受け取ります。ルートは [`routes!`](api:hypersig/macro.routes.html) の中で、それを配信するハンドラを書いた行に宣言します。そのため、ハンドラのないルートは宣言できません。マクロがセグメントを数えるので、数が `P` と合わないパスはコンパイルが通りません。`routes!` には `axum` フィーチャーが要ります。このフィーチャーはデフォルトで有効です。",
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
                    en: "`El::href` takes a [`Link`](api:hypersig/struct.Link.html), which `link()` builds for a route without parameters and `link_with(params)` for one with them. The parameters are expressions of the route's types, as the arguments of an action call are. Literals give a fixed `href`. A parameter that reads a signal or a list item, as on the team page above, is evaluated in the browser and kept up to date. Both sides percent-encode a value as `encodeURIComponent` does, so the first HTML and the runtime write the same URL.",
                    ja: "`El::href` は [`Link`](api:hypersig/struct.Link.html) を受け取ります。パラメータのないルートでは `link()` が、パラメータのあるルートでは `link_with(params)` がリンクを作ります。パラメータは、アクションを呼び出すときの引数と同じく、ルートの型の式です。リテラルを渡すと `href` は固定になります。上のチームのページのようにシグナルやリストの項目を読むパラメータは、ブラウザで評価され、値が変わるたびに更新されます。両側とも `encodeURIComponent` と同じ方法で値をパーセントエンコードするので、最初の HTML とランタイムは同じ URL を書きます。",
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
                    ja: "パスのあとに `in PARENT` と書くと、それより前の行のページの下にルートを宣言します。子のパスは親のパスにセグメントを続けたもので、パラメータは親のパラメータに自分のパラメータを続けたものです。そのため、子へのリンクには親の値も要ります。親のパスの続きになっていないパスや、親のパラメータで始まらないパラメータは、コンパイルが通りません。",
                },
            ),
            show(Snippet::ChildRoutes),
            para(
                site,
                L {
                    en: "An application served under a path prefix, such as `/app` behind a proxy, names it once as a [`Base`](api:hypersig/struct.Base.html) constant: `fn pages(Router<State>) under APP;` on the first line of `routes!`, and `Action::post(\"/save\").under(APP)` on each action, stream, and upload. Links, URLs, redirects, action calls, and the path a page loads the runtime from then start with `/app`, and the router still registers each path without it. The same router then runs inside `Router::nest(APP.as_str(), ...)` or behind a proxy that strips `/app`. The prefix is a constant because the routes and actions it prefixes are, so it is chosen when the program is built. A page served by a router nested under any other path answers every request with a 500 that names both paths, and the compiler does not check that an action a page calls was declared under the same base.",
                    ja: "プロキシの裏の `/app` のようにパスの接頭辞の下で配信するアプリケーションは、接頭辞を [`Base`](api:hypersig/struct.Base.html) の定数として 1 か所で宣言します。`routes!` の 1 行目に `fn pages(Router<State>) under APP;` と書き、アクション、ストリーム、アップロードにはそれぞれ `Action::post(\"/save\").under(APP)` のように付けます。するとリンク、URL、リダイレクト、アクションの呼び出し、ページがランタイムを読み込むパスが `/app` で始まり、ルーターは各パスを接頭辞なしで登録します。同じルーターを `Router::nest(APP.as_str(), ...)` の中でも、`/app` を取り除くプロキシの裏でも動かせます。接頭辞を付けるルートとアクションが定数なので、接頭辞も定数で、プログラムをビルドするときに決まります。ほかのパスに `nest` したルーターのページは、すべてのリクエストに両方のパスを示す 500 を返します。ページが呼ぶアクションが同じ接頭辞で宣言されているかどうかは、コンパイラは検査しません。",
                },
            ),
        ]
        .into_iter()
        .chain([
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
                    en: "Changing the URL without loading a page",
                    ja: "ページを読み込まずに URL を変える",
                },
            ),
            para(
                site,
                L {
                    en: "A list whose filter, sort, and page live in the query string can keep the page loaded while they change. `link.push_url()` is a statement that shows the link's URL in the address bar as a new history entry, and `link.replace_url()` shows it in place of the current one, for a change the Back button should skip, such as each letter typed into a search field. The page keeps its signals and its focus, so the statement runs beside the call that loads the rows. An action does the same with `Patches::push_url(url)` and `Patches::replace_url(url)`, and the patches after them are applied. When the visitor presses Back or Forward, the browser loads the page at that entry's URL from the server, which renders it from the query string.",
                    ja: "フィルター、並び順、ページ番号をクエリ文字列に持つリストは、それらが変わってもページを読み込み直さずに済みます。`link.push_url()` はリンクの URL を新しい履歴の項目としてアドレスバーに出す文です。`link.replace_url()` は今の URL と置き換えるので、検索欄に打つ 1 文字ごとのように、戻るボタンで飛ばしたい変更に使います。ページのシグナルとフォーカスはそのまま残るので、この文は行を読み込む呼び出しと並べて書きます。アクションでは `Patches::push_url(url)` と `Patches::replace_url(url)` で同じことをし、その後の差分も適用されます。訪問者が戻るボタンや進むボタンを押すと、ブラウザはその項目の URL のページをサーバーから読み込み、サーバーはクエリ文字列からページを描きます。",
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
                    ja: "`routes!` の 1 行目に `fn pages(Router<State>) else missing;` と書くと、どのページにも届かなかったページのリクエストを受け取るハンドラを指定できます。対象は、どのルートにも合わないパス、パースできないセグメント、`None` を返したハンドラ、デシリアライズできないクエリ文字列です。ハンドラは [`Unserved`](api:hypersig/struct.Unserved.html) を受け取り、extractor も受け取れます。返したページは `Unserved::status` が示す 404 か 400 で送られます。`Unserved::within(HOME)` はパスの先頭から `HOME` のパラメータを読むので、`/{lang}` の下にあるサイトはパスの言語でページを出せます。ほかの状態を付けたページは `PageResponse::with_status(status, page)` で返します。",
                },
            ),
            head(site),
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
                        ja: "リンクは普通の `<a href>` なので、たどると次のページをサーバーから読み込み、離れたページのシグナルは失われます。",
                    },
                    L {
                        en: "Nothing checks that the URL `push_url` or `replace_url` shows belongs to the page on screen. The URL of another page shows over this page's content, and reloading it loads the other page.",
                        ja: "`push_url` や `replace_url` が出す URL が、画面のページのものかどうかは検査されません。別のページの URL を出すと、このページの内容のままその URL が表示され、再読み込みすると別のページが読み込まれます。",
                    },
                    L {
                        en: "`attr(\"href\", ...)` still writes any URL, for a link to another site or to a `#fragment`.",
                        ja: "ほかのサイトや `#fragment` へのリンクのために、`attr(\"href\", ...)` で任意の URL を書くこともできます。",
                    },
                ],
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
                ja: "`Page::description` は検索結果に出る要約を、`Page::icon` はタブのアイコンを書きます。canonical の URL とリンクのプレビューの画像は絶対 URL でなければならないので、`Page::canonical` と `OpenGraph::image` はパスを [`Origin`](api:hypersig/struct.Origin.html) につなぎます。`Origin` の定数は、スキーム、ホスト、ポート以外を含むとコンパイルエラーになります。プロキシが `/blog` のようなパスの下に配信するアプリは、`Origin::under` でそのパスを足します。`under` も、形の正しくない接頭辞を同じようにコンパイルエラーにします。[`OpenGraph`](api:hypersig/struct.OpenGraph.html) はカードのタイトル、本文、URL をページから取るので、どれも 1 回書くだけで済みます。",
            },
        ),
        show(Snippet::Head),
        para(
            site,
            L {
                en: "`Page::bind_title(expr)` makes the tab's title follow a `String` expression, such as an unread count. The server writes its first value into `<title>`, and the runtime writes each later value into `document.title`. The card of `OpenGraph` keeps the title given to `Page::new`.",
                ja: "`Page::bind_title(expr)` は、未読の件数のように変わる `String` の式にタブのタイトルを合わせます。最初の値はサーバーが `<title>` に書き、その後の値はランタイムが `document.title` に書きます。`OpenGraph` のカードには `Page::new` に渡したタイトルが残ります。",
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
                    ja: "`Token` は型の付いた値を持つ CSS カスタムプロパティです。`Page::theme` は、たとえば `Media::Dark` のもとでのトークンをページに設定します。",
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
                en: "A [`NodeRef`](api:hypersig/struct.NodeRef.html) names one element for a DOM call that no binding covers. Declare it as a `const`, attach it with `El::node_ref`, and use its statements: `focus`, `blur`, `select`, `scroll_into_view`, `show_popover`, and `hide_popover`. Each item of a list may carry the same name, and any other two elements with one name make `Page::render` panic, so a test that renders the page finds the second. Ids set with `El::id` from a `&'static str` are checked by the same rule, and a fixed id inside a list template panics on its own, because every item repeats it. An id built as a `String` may come from stored data, so two of them alike render. The reset button below stops the stopwatch and moves focus to the start button.",
                ja: "[`NodeRef`](api:hypersig/struct.NodeRef.html) は、束縛では扱えない DOM の呼び出しのために要素を 1 つ指す名前です。`const` として宣言し、`El::node_ref` で要素に付け、`focus`、`blur`、`select`、`scroll_into_view`、`show_popover`、`hide_popover` の文を使います。リストの項目ごとに同じ名前を付けるのはかまいませんが、それ以外で 2 つの要素に同じ名前を付けると `Page::render` が panic するので、ページを描画するテストで見つかります。`El::id` に `&'static str` で付けた id も同じ規則で検査し、リストのテンプレートの中の固定の id は、項目ごとに同じ id が並ぶので、それだけで panic します。`String` で作った id は保存したデータから作られることがあるので、2 つ同じでもそのまま描画します。下のリセットボタンはストップウォッチを止め、フォーカスをスタートボタンに移します。",
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
                ja: "`roving_focus` は、タブの一覧やメニューに要るように、矢印キーと Home、End で要素の中の項目間にフォーカスを動かします。`active_descendant` は、コンボボックスに要るように、フォーカスをテキスト欄に置いたまま上下の矢印キーでリストボックスの選択肢を移動させます。`hypersig_ui` のタブ、メニュー、リストボックス、コンボボックスは、この 2 つを使って作っています。",
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
                en: "`El::href` takes only a link built from a [`Route`](learn/pages/). A link to another site, or to a `#fragment` on the same page, goes through `attr(\"href\", url)`, which writes the string as it is and checks nothing.",
                ja: "`El::href` が受け取るのは [`Route`](learn/pages/) から作ったリンクだけです。ほかのサイトや、同じページの `#fragment` へのリンクは `attr(\"href\", url)` で書きます。`attr` は文字列をそのまま書き、何も検査しません。",
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
                en: "A page that calls no action needs no server. `Page::runtime_src` loads the runtime from another URL, and `RUNTIME_JS` holds its source to write next to the page. This site is built that way: a Rust program renders each page with hypersig and writes the files that GitHub Pages serves. A static file carries none of the headers a served page has, `X-Frame-Options` and `frame-ancestors` among them, and a `<meta>` cannot carry `frame-ancestors`, so set them in the host's configuration where the host allows it.",
                ja: "アクションを呼び出さないページにはサーバーは不要です。`Page::runtime_src` は別の URL からランタイムを読み込み、`RUNTIME_JS` はページの隣に書き出すランタイムのソースを持ちます。このサイトもその方法で作っています。Rust のプログラムが hypersig で各ページを描画し、GitHub Pages が配信するファイルを書き出します。静的なファイルには、配信するページに付く `X-Frame-Options` や `frame-ancestors` のヘッダーが付きません。`<meta>` では `frame-ancestors` を指定できないので、ヘッダーを設定できるホストではホストの設定で送ってください。",
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
                    ja: "お絵かきツールや表計算ソフトのように、主な処理が JavaScript のライブラリの中で動くアプリケーション。hypersig は [JavaScript のライブラリ](learn/javascript/) で示すようにライブラリと値をやり取りするだけで、ライブラリが持つ状態はページのシグナルに入りません。",
                },
                L {
                    en: "An application that works without a connection to the server, such as one used offline.",
                    ja: "オフラインで使うものなど、サーバーにつながらなくても動くアプリケーション。",
                },
            ],
        ),
    ])
}
