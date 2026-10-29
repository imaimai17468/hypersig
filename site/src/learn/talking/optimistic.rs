//! The page on changing the page before the server answers.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Optimistic).title(site)),
        lead(site, Doc::Learn(Learn::Optimistic).leads()),
        before(site),
        after(site),
        rollback(site),
        unchecked(site),
        example(site),
    ])
}

/// A statement that runs before the call.
fn before(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A statement before the call",
                ja: "呼び出しの前に置く文",
            },
        ),
        para(
            site,
            L {
                en: "A statement runs in the browser at once, and a call is a statement too, so a statement placed before the call changes the page before the request goes out: `count.set(count + 1).then(LIKE.call())`. [`Stmt::then`](api:hypersig/struct.Stmt.html#method.then), `fn then(self, next: impl IntoStmt<S>) -> Self`, runs `self` and then `next`. The count shows the new value while the request is on its way, and the patch the handler answers with then sets the count the server stored, which may differ when other visitors liked the post meanwhile.",
                ja: "文はブラウザですぐに実行され、呼び出しも文の一種です。そのため、呼び出しの前に文を置くと、リクエストを送る前にページが変わります。`count.set(count + 1).then(LIKE.call())` のように書きます。[`Stmt::then`](api:hypersig/struct.Stmt.html#method.then)（`fn then(self, next: impl IntoStmt<S>) -> Self`）は `self` を実行してから `next` を実行します。リクエストが届くまでの間、カウントには新しい値が表示され、そのあとハンドラが返す差分が、サーバーに保存されたカウントを設定します。その間にほかの訪問者が「いいね」を押していれば、この値は表示中の値と違うことがあります。",
            },
        ),
    ])
}

/// A statement that runs after the call is sent.
fn after(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A statement after the call is sent",
                ja: "呼び出しを送ったあとに置く文",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::then`](api:hypersig/struct.Call.html#method.then), `fn then(self, next: impl IntoStmt<S>) -> Stmt<S>`, sends the call and runs `next` without waiting for the answer. The call evaluates its arguments when it is sent, so `next` can change what they read: `COMMENT.call_with(draft).then(draft.set(\"\"))` sends the comment and empties the field at once. The patches the server answers with apply whenever they arrive. To run a statement after the answer, use [`on_success`](learn/call-status/) instead.",
                ja: "[`Call::then`](api:hypersig/struct.Call.html#method.then)（`fn then(self, next: impl IntoStmt<S>) -> Stmt<S>`）は、呼び出しを送り、応答を待たずに `next` を実行します。呼び出しは送るときに引数を評価するので、`next` が引数の読むシグナルを変えてもかまいません。`COMMENT.call_with(draft).then(draft.set(\"\"))` はコメントを送り、すぐに入力欄を空にします。サーバーが返す差分は、届いた時点で適用されます。応答のあとに文を実行するには、代わりに [`on_success`](learn/call-status/) を使います。",
            },
        ),
    ])
}

/// Undoing the change when the call fails.
fn rollback(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Undoing the change when the call fails",
                ja: "呼び出しが失敗したら変更を戻す",
            },
        ),
        para(
            site,
            L {
                en: "When the call fails, the change stays in the browser: nothing rolls the value back by itself. `error` can show the failure, and [`on_error`](api:hypersig/struct.Call.html#method.on_error) can undo the change with a statement of its own, such as `likes.set(likes - 1)`. The closure passed to `on_error` receives the error text as an `Expr<S, String>`, so setting the count to it is a compile error, E0277.",
                ja: "呼び出しが失敗しても、変更はブラウザに残ります。値を自動で元に戻す処理はありません。`error` で失敗を表示し、[`on_error`](api:hypersig/struct.Call.html#method.on_error) に `likes.set(likes - 1)` のような文を渡せば、変更を戻せます。`on_error` に渡すクロージャはエラーの文言を `Expr<S, String>` として受け取るので、それをカウントに設定するとコンパイルエラー E0277 になります。",
            },
        ),
        show(Snippet::TalkWrongRollback),
    ])
}

/// What an optimistic change leaves to the page.
fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型で検査しないこと",
            },
        ),
        para(
            site,
            L {
                en: "Nothing compares the change made in the browser with the one the handler makes. A statement that adds 1 while the handler adds 2 shows one value until the answer arrives and another after it. Each answer sets the value the server held when it answered, so two quick clicks can show 2, then 1 when the first answer arrives, then 2 when the second does. A statement in `on_error` that undoes the change runs once per failed call, so the same two clicks that both fail take the count back down twice.",
                ja: "ブラウザで加えた変更とハンドラが加える変更が同じかどうかは検査されません。文が 1 を足してハンドラが 2 を足すと、応答が届くまではある値が、届いたあとは別の値が表示されます。応答はそれぞれ、応答した時点でサーバーが持っていた値を設定します。そのため、すばやく 2 回押すと、表示は 2 になり、1 回目の応答で 1 に、2 回目の応答で 2 になることがあります。`on_error` に置いた変更を戻す文は失敗した呼び出しごとに 1 回実行されるので、2 回押してどちらも失敗すれば、カウントは 2 回減ります。",
            },
        ),
    ])
}

/// The complete example, and the pages that follow.
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
                en: "This post counts a like at once and takes it back if the call fails. Its comment form adds the comment to the list, sends it, and empties the field, all before the server answers.",
                ja: "この投稿は「いいね」をすぐに数え、呼び出しが失敗したら取り消します。コメントのフォームはコメントをリストに加えて送信し、入力欄を空にします。どれもサーバーの応答より先に起きます。",
            },
        ),
        show(Snippet::TalkOptimistic),
        para(
            site,
            L {
                en: "Next, [Calls that overlap](learn/overlapping-calls/) covers a second click while the first call waits, and [Patches](learn/patches/) covers what the handler sends back.",
                ja: "次は最初の呼び出しが応答を待つ間にもう一度押されたときを扱う [重なった呼び出し](learn/overlapping-calls/) と、ハンドラが返すものを扱う [差分](learn/patches/) です。",
            },
        ),
    ])
}
