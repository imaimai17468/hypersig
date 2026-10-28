//! The pages on talking to the server, styling, and what to use when bindings
//! and statements are not enough.

use super::snippet::{Snippet, show};
use super::{Program, program};
use crate::Site;
use crate::code::{self, Lang};
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
                en: "An [`Action<S, A>`](api:hypersig/struct.Action.html) is a server endpoint for the page whose signals are `S`, receiving arguments of type `A`. Declare it on an `action` line of `routes!`, in the block that declares the pages, with its method, its path, and the handler that serves it: `action LIKE: Action<Profile> = post \"/like\" => like;`. The function the block generates registers the handler, so an action without one does not compile. The page calls it with `call()`, or `call_with(args)` where `args` is an expression evaluated in the browser when the call runs. A handler that needs several values takes a tuple, which the page passes as a tuple of expressions.",
                ja: "[`Action<S, A>`](api:hypersig/struct.Action.html) は、シグナルが `S` のページのためのサーバーのエンドポイントで、型 `A` の引数を受け取ります。ページを宣言する `routes!` のブロックに `action` の行を書き、メソッド、パス、配信するハンドラと一緒に宣言します。`action LIKE: Action<Profile> = post \"/like\" => like;` のように書きます。ブロックが生成する関数がハンドラを登録するので、ハンドラのないアクションはコンパイルできません。ページは `call()` か `call_with(args)` で呼び出します。`args` は、呼び出しを実行するときにブラウザで評価される式です。複数の値が必要なハンドラはタプルを受け取り、ページは式のタプルを渡します。",
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
                en: "`indicator(flag)` sets a `bool` signal to true while the request waits for its response, which the example uses to disable the button. `error(message)` sets a `String` signal to the error text when the call fails (a network error, a response that is not a success, or an answer that closes before the server ended it) and to `\"\"` each time the call is sent, so the same failure twice changes the signal twice.",
                ja: "`indicator(flag)` はリクエストの応答を待つ間 `bool` のシグナルを真にします。この例ではそれを使ってボタンを無効にしています。`error(message)` は、呼び出しが失敗したときに `String` のシグナルへエラーの文言を入れ、呼び出しを送るたびに `\"\"` に戻します。失敗とは、ネットワークのエラー、成功ではない応答、サーバーが終える前に閉じた応答のことです。戻してから入れ直すので、同じ失敗が 2 回続いてもシグナルは 2 回変わります。",
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
                ja: "呼び出しの前に文を置くと、ページがすぐに変わり、そのあとにリクエストが送られます。`count.set(count + 1).then(LIKE.call())` のように書きます。続いて、サーバーが保存したカウントをハンドラの差分が設定します。サーバーがエラーを返した場合、変更はブラウザに保持されます。`error` で失敗を表示できますが、値を元に戻す処理はありません。",
            },
        ),
        subheading(
            site,
            L {
                en: "What actions leave unchecked",
                ja: "アクションが検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "The compiler checks that each action has a handler, and two mistakes stay unchecked. The first is a `routes!` block whose generated function the application never calls. Its pages answer 404, and each call to its actions, streams, and uploads gets a 404 with an empty body, which the runtime writes to the console and `error` receives as a failed call. A call without `error` changes nothing on the page. rustc warns about a handler `fn` that only the uncalled function names, and not about a closure. The second is a page that calls an action of another block whose `under` names a different [`Base`](api:hypersig/struct.Base.html). The call goes to that block's prefix, which answers 404 unless the other block is served there too. Declaring every page and action in one block avoids both, because the application then calls one function and every line shares one base. A test that sends `LIKE.request(&3)` to the application's router with `oneshot` and expects a 200, as on [Testing](learn/testing/), fails when the function is not called.",
                ja: "コンパイラはアクションごとにハンドラがあることを検査します。検査されない間違いは 2 つ残ります。1 つ目は生成された関数をアプリケーションが呼ばない `routes!` のブロックです。そのブロックのページは 404 を返し、アクション、ストリーム、アップロードへの呼び出しはどれも本文が空の 404 を受け取ります。ランタイムはこれをコンソールに書き、`error` は失敗した呼び出しとして受け取ります。`error` のない呼び出しでは、ページは何も変わりません。呼ばれない関数の中でだけ使うハンドラの `fn` には rustc が警告し、クロージャには警告しません。2 つ目は `under` に別の [`Base`](api:hypersig/struct.Base.html) を書いたブロックのアクションをページが呼ぶ場合です。呼び出しはそのブロックの接頭辞に送られ、そのブロックもそこで配信していなければ 404 が返ります。すべてのページとアクションを 1 つのブロックに宣言すれば、どちらも起きません。アプリケーションが呼ぶ関数は 1 つになり、すべての行が同じ接頭辞を使うからです。[テストを書く](learn/testing/) の例のように `LIKE.request(&3)` をアプリケーションのルーターに `oneshot` で送って 200 を期待するテストは、関数を呼んでいなければ失敗します。",
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
                en: "[`Patches<S>`](api:hypersig/struct.Patches.html) is a batch the runtime applies in order. `sig.patch(value)` sets one signal, `and` adds more to the same event, and `Patches::new().all_signals(&state)` replaces all of them. `patch` exists only on a `Sig<S, T>`, whose path starts at the page's struct, and not on the `Sig<S, T, Local>` of a list item, so the server addresses fields by name and never by a list position.",
                ja: "[`Patches<S>`](api:hypersig/struct.Patches.html) は、ランタイムが順に適用する差分のまとまりです。`sig.patch(value)` はシグナルを 1 つ設定し、`and` は同じイベントに差分を追加し、`Patches::new().all_signals(&state)` はすべてのシグナルを置き換えます。`patch` があるのはパスがページの構造体から始まる `Sig<S, T>` だけで、リストの項目の `Sig<S, T, Local>` にはありません。そのため、サーバーはフィールドを名前で指し、リストの位置では指しません。",
            },
        ),
        show(Snippet::Patches),
        para(
            site,
            L {
                en: "Each patch is sent as one Server-Sent Event. `to_event_stream` returns the body for a server that does not use axum.",
                ja: "各差分は 1 つの Server-Sent Event として送られます。axum を使わないサーバー向けには、`to_event_stream` がレスポンスの本文を返します。",
            },
        ),
        fragments(site),
        para(
            site,
            L {
                en: "[Calls in depth](learn/calls/) covers patches that send only the items of a list that change, calls that overlap, streams that reconnect, and parts that several pages share.",
                ja: "リストの変わった項目だけを送る差分、重なった呼び出し、つなぎ直すストリーム、複数のページで使う部品は [呼び出しの詳細](learn/calls/) で説明します。",
            },
        ),
    ])
}

pub fn calls_in_depth(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::CallsInDepth).title(site)),
        lead(site, Doc::Learn(Learn::CallsInDepth).leads()),
        heading(
            site,
            L {
                en: "List patches",
                ja: "リストの差分",
            },
        ),
        para(
            site,
            L {
                en: "`patch` sends a list whole, so a chat that sends each message with `patch` sends every earlier one again. [`ListPatch`](api:hypersig/struct.ListPatch.html) sends only the items that change: `patch_append` and `patch_prepend` add items at either end of the list the browser holds, `patch_upsert` replaces the item with the same key or appends it, and `patch_remove` removes the items with the given keys. The key is an expression of the item, as in `each_keyed`, and the browser computes it for each item. As with `patch`, only a `Vec` signal reached from the page's struct has these methods.",
                ja: "`patch` はリストを丸ごと送るので、チャットのメッセージを 1 件ずつ `patch` で送ると、それまでのメッセージも毎回送り直すことになります。[`ListPatch`](api:hypersig/struct.ListPatch.html) は変わる項目だけを送ります。`patch_append` と `patch_prepend` はブラウザが持つリストの末尾と先頭に項目を加え、`patch_upsert` は同じキーの項目を置き換えるか末尾に加え、`patch_remove` は指定したキーの項目を取り除きます。キーは `each_keyed` と同じく項目の式で、ブラウザが項目ごとに計算します。`patch` と同じく、これらのメソッドはページの構造体から辿った `Vec` のシグナルにだけあります。",
            },
        ),
        show(Snippet::ListPatches),
        para(
            site,
            L {
                en: "Items added before an `each_keyed` list keep the items already on screen where they were: the runtime scrolls the nearest scrollable ancestor by the height of the new items. Chrome's scroll anchoring does the same once the list is scrolled, but not at the very top, and a list that loads older items is at the top when it loads them. `Sig::prepend` does the same in the browser. A list drawn with `each` matches its elements by position, so its first element shows the first new item and nothing scrolls.",
                ja: "`each_keyed` のリストの先頭に項目を加えると、画面に出ていた項目はその位置に残ります。ランタイムがいちばん近いスクロールできる祖先要素を、新しい項目の高さの分だけスクロールします。Chrome のスクロールアンカリングもリストをスクロールした後なら同じことをしますが、いちばん上では働きません。古い項目を読み込むリストは、読み込むときにちょうどいちばん上にあります。ブラウザの中では `Sig::prepend` が同じように動きます。`each` で描いたリストは要素を位置で対応させるので、先頭の要素に新しい最初の項目が表示され、スクロールはしません。",
            },
        ),
        subheading(
            site,
            L {
                en: "Loading more as the visitor scrolls",
                ja: "スクロールに合わせて続きを読み込む",
            },
        ),
        para(
            site,
            L {
                en: "A feed that loads its next page as the visitor reaches the bottom is built from `on_visible`, `patch_append`, and a `page` signal. An element after the items runs `on_visible` when it comes into view, which calls a `GET` action with the `page` signal as its argument. The handler appends the page's items with `patch_append`, so the items already in the browser are not sent again, and patches `page` to the next number and `more` to whether a full page came back. `indicator` sets `loading` while the call runs, and `show` hides the element during the load and once `more` is false. When the new items still leave it in view, showing it again runs `on_visible` again and loads the next page.",
                ja: "下まで来ると次のページを読み込むフィードは、`on_visible`、`patch_append`、`page` のシグナルで作ります。項目のあとに置いた要素は、見える位置に入ると `on_visible` を実行し、`page` のシグナルを引数にして `GET` のアクションを呼び出します。ハンドラは `patch_append` でそのページの項目を末尾に加えるので、ブラウザにある項目は送り直しません。あわせて `page` を次の番号に、`more` を 1 ページ分そろって返ったかどうかに設定します。呼び出しの間は `indicator` が `loading` を真にし、`show` は読み込み中と `more` が偽になったあとに要素を隠します。新しい項目を加えても要素が見える位置に残っていれば、再び表示されたときに `on_visible` がもう一度実行され、次のページを読み込みます。",
            },
        ),
        show(Snippet::LoadMore),
        calls(site),
        shared_parts(site),
    ])
}

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
                en: "Each call applies its patches when they arrive, so the answer to an earlier, slower search can overwrite the answer to a later one. [`Call::latest`](api:hypersig/struct.Call.html#method.latest) aborts the unfinished earlier `latest` call to the same action, and the patches it has not received are dropped. Calls from different elements to one action share it, because the handler decides which signals they patch. The server may stop the aborted handler at an `.await`, so put `latest` only on calls that read. A later call never aborts a call without `latest`, and an indicator stays true until every call that sets it has its answer.",
                ja: "呼び出しはそれぞれ差分が届いた時点で反映するので、前に送った遅い検索の応答が、あとの検索の応答を上書きすることがあります。[`Call::latest`](api:hypersig/struct.Call.html#method.latest) は、同じアクションへの `latest` の呼び出しで、まだ終わっていない前のものを取り消します。取り消した呼び出しでまだ届いていない差分は捨てられます。アクションがどのシグナルに差分を返すかはハンドラが決めるので、別々の要素から同じアクションを呼んでも、取り消しの対象は共通です。取り消されたハンドラはサーバーで `.await` の時点で止まることがあるので、`latest` は読み取りだけに付けてください。`latest` のない呼び出しが、あとの呼び出しに取り消されることはありません。indicator はそれを使う呼び出しがすべて応答を受け取るまで真のままです。",
            },
        ),
        show(Snippet::Latest),
        para(
            site,
            L {
                en: "A call that changes data goes without `latest`, and the runtime keeps its answers in order instead: once a later `POST` to the action has its complete answer, the events of an earlier one that are still arriving are dropped. [`Call::exclusive`](api:hypersig/struct.Call.html#method.exclusive) skips a call while another call to the same action awaits its answer, so a second click on an order button sends nothing. While a `POST` is awaited, the element that sent it carries `aria-busy=\"true\"`. [`Call::timeout`](api:hypersig/struct.Call.html#method.timeout) fails a call whose answer has not ended in the time it is given, with status 0. Without it, a call waits for as long as the server takes, which an upload of a large file needs.",
                ja: "データを変える呼び出しには `latest` を付けず、代わりにランタイムが応答の順序を守ります。同じアクションへのあとの `POST` が応答を最後まで受け取ると、前の `POST` でまだ届いているイベントは捨てられます。[`Call::exclusive`](api:hypersig/struct.Call.html#method.exclusive) は、同じアクションへの別の呼び出しが応答を待っている間、その呼び出しを送りません。注文のボタンを 2 回押しても、2 回目は何も送りません。`POST` の応答を待つ間、それを送った要素には `aria-busy=\"true\"` が付きます。[`Call::timeout`](api:hypersig/struct.Call.html#method.timeout) は、渡した時間のうちに応答が終わらなかった呼び出しを、ステータス 0 の失敗にします。付けなければ、呼び出しはサーバーが応答を終えるまで待ちます。大きなファイルのアップロードには、待ち続けるこの動きが必要です。",
            },
        ),
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
                en: "[`StreamCall::reconnect`](api:hypersig/struct.StreamCall.html#method.reconnect) opens a stream again when its connection fails, it breaks off, or the server answers with a 5xx status, as the clock on [Talking to the server](learn/server/) does. It waits up to 1 second, then up to twice as long each time, to at most 30 seconds. Each wait ends at a random point past its half, so pages that lost the server together come back spread over that time, and it ends early when the browser comes back online or the page becomes visible. The server sends the event `hs-end` last once the handler's stream finishes, so a response a host closes at its time limit, which ends before `hs-end`, counts as broken off. `reconnect` does not open the stream again after `hs-end` or a 4xx answer, or once the element has left the page. Each attempt evaluates the arguments again, so a stream that takes a signal its patches keep current, such as the id of the last message, can send only what came after it. A stream whose arguments stay the same should start with the current values, and the page catches up with what it missed. Only a call to a `StreamAction`, which is always served on `GET`, has `reconnect`, because sending a `POST` again could repeat a change.",
                ja: "[`StreamCall::reconnect`](api:hypersig/struct.StreamCall.html#method.reconnect) は、接続が失敗したとき、ストリームが途中で切れたとき、サーバーが 5xx を返したときに、ストリームを開き直します。[サーバーとのやり取り](learn/server/) の時計がこの例です。最初は最大 1 秒待ち、そのあとは待ち時間の上限を倍にしながら 30 秒まで延ばします。実際に待つ長さは上限の半分から上限までのうちランダムに決まるので、同時にサーバーを失ったページが開き直す時刻はばらつきます。ブラウザがオンラインに戻ったときや、ページが表示されたときは、待つのをやめてすぐに開き直します。サーバーはハンドラのストリームが終わると最後に `hs-end` のイベントを送ります。ホストが時間の上限で閉じたレスポンスは `hs-end` の前で終わるので、途中で切れたものとして扱います。`hs-end` で終わったとき、4xx の応答を受けたとき、要素がページから外れたときは、開き直しません。開き直すたびに引数を評価し直します。最後に受け取ったメッセージの id のように、ストリームの差分が更新し続けるシグナルを引数に渡せば、ハンドラはそれより後の分だけを送れます。引数が変わらないストリームでは、初めに現在の値を送ってください。そうすればページは見逃した変更に追いつきます。`POST` を送り直すと変更が繰り返されることがあるので、`reconnect` があるのは、常に `GET` で配信する `StreamAction` の呼び出しだけです。",
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
                en: "A [`Fragment`](api:hypersig/trait.Fragment.html) is a part of the page the server can render again on its own. It owns its element id, so the id in the first page and the id in a later patch cannot differ. `fragment(&value)` renders it in the page, and `Patches::new().element(&value)` sends it again. The runtime compares the HTML it receives with the element that has that id and rewrites only the parts that differ. An element without an id is matched with the next one of the same tag and `name`, so a message inserted before an input leaves the input and its text in place. Where a patch does replace the element that has focus, focus and the text selection move to the new element with the same id or `name`. It leaves the `open` attribute of a `<details>` and a `<dialog>` alone, so a section the visitor opened or a dialog a script showed stays as it was. A patch includes the CSS rules its elements use. The server renders a fragment sent again on its own, so it cannot compare its ids with the rest of the page. The browser compares them once the patch is applied and writes a warning to the console when the patch adds an id that another element already carries. The same holds for the name of a `NodeRef`, which [Node refs](learn/node-refs/) covers. The server sends the fragment's whole HTML each time, so a list that changes one item at a time costs less as a list signal drawn with `each_keyed`, which a [list patch](learn/calls/#list-patches) changes one item at a time, as `examples/todo` does.",
                ja: "[`Fragment`](api:hypersig/trait.Fragment.html) は、サーバーが単独で描画し直せるページの一部です。フラグメントが要素の id を持つので、最初のページの id とあとの差分の id が食い違うことはありません。`fragment(&value)` はページの中にフラグメントを描画し、`Patches::new().element(&value)` はそれをもう一度送ります。ランタイムは送られた HTML とその id の要素を突き合わせ、違う部分だけを書き換えます。id のない要素は、タグと `name` が同じ次の要素と対応づけるので、入力欄の前にメッセージが加わっても、入力欄も中の文字もそのまま保持されます。フォーカスのある要素を差分が置き換えたときは、同じ id か `name` を持つ新しい要素にフォーカスと選択範囲を移します。`<details>` と `<dialog>` の `open` 属性には触れないので、訪問者が開いた項目やスクリプトが表示したダイアログはそのまま保持されます。差分にはその要素が使う CSS のルールも含まれます。サーバーは送り直すフラグメントを単独で描画するので、ページの残りの部分とは比べられません。そこでブラウザが差分を適用したあとに比べ、ほかの要素がすでに持つ id を差分が追加したときは、コンソールに警告を書きます。[ノード参照](learn/node-refs/) で説明する `NodeRef` の名前も同じです。サーバーはフラグメントの HTML を毎回まるごと送ります。項目が 1 件ずつ変わるリストは、`each_keyed` で描くリストのシグナルにして [リストの差分](learn/calls/#list-patches) で 1 件ずつ変えるほうが送る量が少なくなります。`examples/todo` はこの形です。",
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
                en: "A [`StreamAction`](api:hypersig/struct.StreamAction.html), declared on a `stream` line of `routes!`, runs a handler that returns a stream of patches, on `GET`. The response stays open until the stream ends or the browser leaves the page. Call it from `on_load` to send changes made elsewhere, such as by other users.",
                ja: "`routes!` の `stream` の行に宣言した [`StreamAction`](api:hypersig/struct.StreamAction.html) は、差分のストリームを返すハンドラを `GET` で動かします。レスポンスは、ストリームが終わるかブラウザがページを離れるまで開いたままです。ほかのユーザーなど別の場所で起きた変更を送るには、`on_load` から呼び出してください。",
            },
        ),
        para(
            site,
            L {
                en: "The stream below uses the `futures` crate and tokio's timer, so add `futures` to the Quick start's `[dependencies]` and turn on tokio's `time` feature:",
                ja: "下のストリームは `futures` クレートと tokio のタイマーを使うので、クイックスタートの `[dependencies]` に `futures` を加え、tokio の `time` フィーチャーを有効にしてください。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"futures = "0.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net", "time"] }"#,
        ),
        program(Program::Streams),
        note(
            site,
            L {
                en: "These programs need a server, so this site shows them as code. Each is built by `cargo test` from `site/examples/` in the repository.",
                ja: "これらのプログラムにはサーバーが必要なので、このサイトではコードとして示します。どれもリポジトリの `site/examples/` にあり、`cargo test` でビルドしています。",
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
                        en: "A link to another site or to a `#fragment` is a [`SafeUrl`](api:hypersig/struct.SafeUrl.html), which [Values the types do not check](learn/unchecked-values/) covers. It checks the URL's scheme, and nothing checks that the address it names exists.",
                        ja: "ほかのサイトや `#fragment` へのリンクには [`SafeUrl`](api:hypersig/struct.SafeUrl.html) を使います。これは [型で検査しない値](learn/unchecked-values/) で説明します。`SafeUrl` が検査するのは URL のスキームだけで、その先のアドレスがあるかどうかは検査されません。",
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
