//! The page on the batch of patches a handler answers with, and the events
//! it becomes.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::Patches).title(site)),
        lead(site, Doc::Learn(Learn::Patches).leads()),
        signals(site),
        paths(site),
        batch(site),
        body(site),
        unchecked(site),
        example(site),
    ])
}

/// `Sig::patch` and `SignalPatch::and`.
fn signals(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Setting a signal",
                ja: "シグナルを設定する",
            },
        ),
        para(
            site,
            L {
                en: "[`Sig::patch`](api:hypersig/struct.Sig.html#method.patch), `fn patch(self, value: T) -> SignalPatch<S>`, sets one signal to `value` once the browser receives it. [`SignalPatch::and`](api:hypersig/struct.SignalPatch.html#method.and), `fn and(self, other: Self) -> Self`, adds more signals to the same patch, and a signal present in both takes `other`'s value. A handler can return the `SignalPatch` itself, or convert it with `into()` into a [`Patches<S>`](api:hypersig/struct.Patches.html).",
                ja: "[`Sig::patch`](api:hypersig/struct.Sig.html#method.patch)（`fn patch(self, value: T) -> SignalPatch<S>`）は、ブラウザが受け取った時点でシグナルを 1 つ `value` に設定する差分です。[`SignalPatch::and`](api:hypersig/struct.SignalPatch.html#method.and)（`fn and(self, other: Self) -> Self`）は、同じ差分にシグナルを加えます。両方にあるシグナルは `other` の値になります。ハンドラは `SignalPatch` をそのまま返すことも、`into()` で [`Patches<S>`](api:hypersig/struct.Patches.html) に変換して返すこともできます。",
            },
        ),
        show(Snippet::Patches),
        para(
            site,
            L {
                en: "A value of another type than the signal's is a compile error, E0308. `patch` exists only on a `Sig<S, T>`, whose path starts at the page's struct. The `Sig<S, T, Local>` of a list item has no `patch`, so the server addresses fields by name and never by a list position: [List patches](learn/list-patches/) changes items by key. A field marked `#[signals(computed)]` has no `patch` either, because the browser computes it.",
                ja: "シグナルと違う型の値を渡すと、コンパイルエラー E0308 になります。`patch` があるのは、パスがページの構造体から始まる `Sig<S, T>` だけです。リストの項目の `Sig<S, T, Local>` には `patch` がないので、サーバーはフィールドを名前で指し、リストの位置では指しません。項目をキーで変えるには [リストの差分](learn/list-patches/) を使います。`#[signals(computed)]` を付けたフィールドにも `patch` はありません。その値はブラウザが計算するからです。",
            },
        ),
        show(Snippet::TalkWrongPatch),
    ])
}

/// Dotted paths and `all_signals`.
fn paths(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Fields inside fields",
                ja: "フィールドの中のフィールド",
            },
        ),
        para(
            site,
            L {
                en: "`at` reaches a field inside a field, and a patch of it sets that field alone: the browser leaves every signal the patch does not name as it was. [`Patches::all_signals`](api:hypersig/struct.Patches.html#method.all_signals), `fn all_signals(self, signals: &S) -> Self`, sends every field of the struct at once, for an answer that replaces the whole state.",
                ja: "`at` でフィールドの中のフィールドを指し、その差分はそのフィールドだけを設定します。差分が指さないシグナルは、ブラウザでそのまま残ります。[`Patches::all_signals`](api:hypersig/struct.Patches.html#method.all_signals)（`fn all_signals(self, signals: &S) -> Self`）は構造体のすべてのフィールドを一度に送ります。状態を丸ごと置き換える応答に使います。",
            },
        ),
        show(Snippet::TalkPatchPaths),
    ])
}

/// The builder methods and the event each one adds.
fn batch(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A batch of events",
                ja: "イベントのまとまり",
            },
        ),
        para(
            site,
            L {
                en: "`Patches::new()` starts an empty batch, and each method below adds one change to it. The browser applies the changes in the order they were added.",
                ja: "`Patches::new()` は空のまとまりを作り、下の各メソッドが変更を 1 つずつ加えます。ブラウザは変更を加えた順に適用します。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "What the browser does"],
                ja: &["メソッド", "ブラウザがすること"],
            },
            L {
                en: &[
                    &[
                        "`signals(patch)`, `all_signals(&state)`",
                        "Sets the signals",
                    ],
                    &[
                        "`list(patch)`",
                        "Adds, replaces, or removes items, as [List patches](learn/list-patches/) shows",
                    ],
                    &[
                        "`element(&fragment)`",
                        "Replaces the element with the fragment's id, as [Fragments](learn/fragments/) shows",
                    ],
                    &[
                        "`navigate(url)`",
                        "Loads that page, and applies none of the changes after it, as [Links and redirects](learn/links/) shows",
                    ],
                    &[
                        "`push_url(url)`, `replace_url(url)`",
                        "Shows the URL in the address bar without loading it, as [Search parameters](learn/search-params/) shows",
                    ],
                ],
                ja: &[
                    &[
                        "`signals(patch)`、`all_signals(&state)`",
                        "シグナルを設定する",
                    ],
                    &[
                        "`list(patch)`",
                        "項目を加える、置き換える、取り除く。[リストの差分](learn/list-patches/) を参照",
                    ],
                    &[
                        "`element(&fragment)`",
                        "フラグメントの id を持つ要素を置き換える。[フラグメント](learn/fragments/) を参照",
                    ],
                    &[
                        "`navigate(url)`",
                        "そのページを読み込み、あとの変更は適用しない。[リンクとリダイレクト](learn/links/) を参照",
                    ],
                    &[
                        "`push_url(url)`、`replace_url(url)`",
                        "URL を読み込まずにアドレスバーに表示する。[検索パラメータ](learn/search-params/) を参照",
                    ],
                ],
            },
        ),
    ])
}

/// `to_event_stream`, for a server without axum.
fn body(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The response body",
                ja: "レスポンスの本文",
            },
        ),
        para(
            site,
            L {
                en: "With axum, the router writes the batch as the body of a `text/event-stream` answer. [`Patches::to_event_stream`](api:hypersig/struct.Patches.html#method.to_event_stream), `fn to_event_stream(&self) -> String`, returns that body for a server that does not use axum. Send the whole string: the body ends with a mark that the answer is complete, and the browser counts an answer cut off before it as a failed call.",
                ja: "axum では、ルーターがまとまりを `text/event-stream` の応答の本文として書きます。[`Patches::to_event_stream`](api:hypersig/struct.Patches.html#method.to_event_stream)（`fn to_event_stream(&self) -> String`）は、axum を使わないサーバーのためにその本文を返します。文字列は最後まで送ってください。本文の末尾には応答が完結したことを示す印があり、その前で切れた応答をブラウザは失敗した呼び出しとして扱います。",
            },
        ),
    ])
}

/// What the types leave unchecked, and what happens then.
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
                en: "A value is converted to JSON when the patch is built. An integer outside -2^53 to 2^53, which the browser would round, or a `Serialize` impl that returns an error, makes `patch` panic, and an action whose handler panics answers 500. A `navigate` in the middle of a batch leaves the changes after it unapplied, and nothing checks that the URL of `push_url` or `replace_url` belongs to the page on screen.",
                ja: "値は差分を作るときに JSON に変換されます。ブラウザが丸めてしまう -2^53 から 2^53 の範囲外の整数や、エラーを返す `Serialize` の実装があると `patch` はパニックし、ハンドラがパニックしたアクションは 500 を返します。まとまりの途中に置いた `navigate` のあとの変更は適用されません。`push_url` や `replace_url` の URL が画面のページのものかどうかも検査されません。",
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
                en: "This handler archives a message: one batch removes the message from the list and sets the unread count and the status line. The test at the end calls the handler and reads the batch with [`value_of`](api:hypersig/struct.Patches.html#method.value_of) and [`removed_keys_of`](api:hypersig/struct.Patches.html#method.removed_keys_of), which [Testing handlers](learn/testing/) covers with the rest of the methods for tests.",
                ja: "このハンドラはメッセージをアーカイブします。1 つのまとまりで、リストからメッセージを取り除き、未読の数と状態の行を設定します。最後のテストはハンドラを呼び、[`value_of`](api:hypersig/struct.Patches.html#method.value_of) と [`removed_keys_of`](api:hypersig/struct.Patches.html#method.removed_keys_of) でまとまりを読みます。テスト用のほかのメソッドは [ハンドラのテスト](learn/testing/) で説明します。",
            },
        ),
        show(Snippet::TalkInbox),
        para(
            site,
            L {
                en: "Next, [List patches](learn/list-patches/) sends only the items of a list that change, and [Fragments](learn/fragments/) sends HTML for a part of the page.",
                ja: "次はリストの変わった項目だけを送る [リストの差分](learn/list-patches/) と、ページの一部の HTML を送る [フラグメント](learn/fragments/) です。",
            },
        ),
    ])
}
