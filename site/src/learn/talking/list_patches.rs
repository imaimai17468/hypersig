//! The page on patches that change items of a list without sending the rest
//! of it, and on loading more items as the visitor scrolls.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, subheading, table_of, title};

pub fn page(site: Site) -> Part {
    parts([
        title(Doc::Learn(Learn::ListPatches).title(site)),
        lead(site, Doc::Learn(Learn::ListPatches).leads()),
        methods(site),
        prepend(site),
        unchecked(site),
        load_more(site),
    ])
}

/// The four list patches and how they combine.
fn methods(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The four list patches",
                ja: "4 つのリストの差分",
            },
        ),
        para(
            site,
            L {
                en: "`patch` sends a list whole, so a chat that sends each message with `patch` sends every earlier one again. A [`ListPatch<S>`](api:hypersig/struct.ListPatch.html) sends only the items that change, and the browser applies it to the list it holds, so the server does not keep a copy of that list. Four methods of a `Vec` signal build one:",
                ja: "`patch` はリストを丸ごと送るので、チャットのメッセージを 1 件ずつ `patch` で送ると、それまでのメッセージも毎回送り直すことになります。[`ListPatch<S>`](api:hypersig/struct.ListPatch.html) は変わる項目だけを送り、ブラウザが手元のリストにそれを適用します。そのため、サーバーはそのリストの写しを持たずに済みます。`Vec` のシグナルの 4 つのメソッドが `ListPatch` を作ります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Method", "What the browser does with its list"],
                ja: &["メソッド", "ブラウザがリストに対してすること"],
            },
            L {
                en: &[
                    &[
                        "[`patch_append`](api:hypersig/struct.Sig.html#method.patch_append)`(self, items: &[T])`",
                        "Adds `items` after the items it holds",
                    ],
                    &[
                        "[`patch_prepend`](api:hypersig/struct.Sig.html#method.patch_prepend)`(self, items: &[T])`",
                        "Adds `items` before the items it holds",
                    ],
                    &[
                        "[`patch_upsert`](api:hypersig/struct.Sig.html#method.patch_upsert)`(self, items: &[T], key)`",
                        "Replaces the first item with each new item's key, and adds an item whose key no item has at the end. Of two items in `items` with one key, the later wins",
                    ],
                    &[
                        "[`patch_remove`](api:hypersig/struct.Sig.html#method.patch_remove)`(self, keys: &[K], key)`",
                        "Removes each item whose key is one of `keys`",
                    ],
                ],
                ja: &[
                    &[
                        "[`patch_append`](api:hypersig/struct.Sig.html#method.patch_append)`(self, items: &[T])`",
                        "手元の項目のあとに `items` を加える",
                    ],
                    &[
                        "[`patch_prepend`](api:hypersig/struct.Sig.html#method.patch_prepend)`(self, items: &[T])`",
                        "手元の項目の前に `items` を加える",
                    ],
                    &[
                        "[`patch_upsert`](api:hypersig/struct.Sig.html#method.patch_upsert)`(self, items: &[T], key)`",
                        "新しい項目ごとに、同じキーを持つ最初の項目を置き換え、どの項目も持たないキーの項目は末尾に加える。`items` に同じキーの項目が 2 つあれば、あとのものを使う",
                    ],
                    &[
                        "[`patch_remove`](api:hypersig/struct.Sig.html#method.patch_remove)`(self, keys: &[K], key)`",
                        "キーが `keys` のどれかに一致する項目を取り除く",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each returns a `ListPatch<S>`. `key` is a closure `impl FnOnce(Item<S, T>) -> E` that builds an expression of the item, as in `each_keyed`. The closure runs once on the server to build the expression, and the browser evaluates it for each item and compares the keys. [`ListPatch::and`](api:hypersig/struct.ListPatch.html#method.and) adds the changes of another `ListPatch`, applied after its own, and [`Patches::list`](api:hypersig/struct.Patches.html#method.list) adds a `ListPatch` to a batch beside signals and fragments.",
                ja: "どれも `ListPatch<S>` を返します。`key` は `impl FnOnce(Item<S, T>) -> E` のクロージャで、`each_keyed` と同じく項目の式を作ります。クロージャはサーバーで 1 回だけ実行されて式を作り、ブラウザが項目ごとにその式を評価して、キーを比べます。[`ListPatch::and`](api:hypersig/struct.ListPatch.html#method.and) はほかの `ListPatch` の変更を自分の変更のあとに加え、[`Patches::list`](api:hypersig/struct.Patches.html#method.list) は `ListPatch` をシグナルやフラグメントと同じまとまりに加えます。",
            },
        ),
        show(Snippet::ListPatches),
        para(
            site,
            L {
                en: "As with `patch`, only a `Vec` signal reached from the page's struct has these methods. The list of a list item's field is a `Sig<S, Vec<T>, Local>`, and calling one of them on it is a compile error, E0599, because the server cannot name the item the browser is rendering. [The state of a list item](learn/list-items/) covers that state in the browser.",
                ja: "`patch` と同じく、これらのメソッドはページの構造体から辿った `Vec` のシグナルにだけあります。リストの項目のフィールドのリストは `Sig<S, Vec<T>, Local>` で、これに対して呼ぶとコンパイルエラー E0599 になります。ブラウザが描画中の項目を、サーバーからは指せないからです。ブラウザ側の項目の状態は [リスト項目の状態](learn/list-items/) で説明します。",
            },
        ),
        show(Snippet::TalkItemPatch),
    ])
}

/// Items added above the ones on screen.
fn prepend(site: Site) -> Part {
    parts([
        subheading(
            site,
            L {
                en: "Items added above the ones on screen",
                ja: "画面の項目より上に加える項目",
            },
        ),
        para(
            site,
            L {
                en: "Items added before an `each_keyed` list keep the items already on screen where they were: the browser scrolls the nearest scrollable ancestor by the height of the new items. Chrome's scroll anchoring does the same once the list is scrolled, but not at the very top, and a list that loads older items is at the top when it loads them. `Sig::prepend` does the same in the browser. A list drawn with `each` matches its elements by position, so its first element shows the first new item and nothing scrolls.",
                ja: "`each_keyed` のリストの先頭に項目を加えると、画面に出ていた項目はその位置に残ります。ブラウザがいちばん近いスクロールできる祖先要素を、新しい項目の高さの分だけスクロールします。Chrome のスクロールアンカリングもリストをスクロールした後なら同じことをしますが、いちばん上では働きません。古い項目を読み込むリストは、読み込むときにちょうどいちばん上にあります。ブラウザの中では `Sig::prepend` が同じように動きます。`each` で描いたリストは要素を位置で対応させるので、先頭の要素に新しい最初の項目が表示され、スクロールはしません。",
            },
        ),
    ])
}

/// What the types leave unchecked.
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
                en: "Nothing checks that the key of `patch_upsert` or `patch_remove` is the key the list's `each_keyed` draws with, or that the keys are unique. A key that differs finds no item, so an upsert adds the item at the end instead of replacing it, and a removal removes nothing. Two items with one key in an `each_keyed` list are matched by position, and the browser writes a warning naming the key to the console. An item that holds an integer outside -2^53 to 2^53 makes the method panic when the patch is built. In a test, [`items_of`](api:hypersig/struct.Patches.html#method.items_of) and [`removed_keys_of`](api:hypersig/struct.Patches.html#method.removed_keys_of) read what a batch adds and removes, and `removed_keys_of` takes the key closure, so the test also checks the key the browser compares.",
                ja: "`patch_upsert` や `patch_remove` のキーが、リストの `each_keyed` が描画に使うキーと同じかどうかは検査されません。キーが一意かどうかも検査されません。キーが違うとどの項目にも一致しないので、upsert は項目を置き換えずに末尾に加え、削除は何も取り除きません。`each_keyed` のリストに同じキーの項目が 2 つあると、それらは位置で対応づけられ、ブラウザはそのキーを示す警告をコンソールに書きます。-2^53 から 2^53 の範囲外の整数を持つ項目があると、差分を作るときにメソッドがパニックします。テストでは [`items_of`](api:hypersig/struct.Patches.html#method.items_of) と [`removed_keys_of`](api:hypersig/struct.Patches.html#method.removed_keys_of) で、まとまりが加える項目と取り除くキーを読めます。`removed_keys_of` はキーのクロージャを受け取るので、ブラウザが比べるキーもテストで確かめられます。",
            },
        ),
    ])
}

/// The feed that loads its next page at the bottom, and the pages that follow.
fn load_more(site: Site) -> Part {
    parts([
        heading(
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
        para(
            site,
            L {
                en: "Next, [Fragments](learn/fragments/) sends HTML for a part of the page, and [Streams](learn/streams/) sends list patches as they happen, such as a chat's new messages.",
                ja: "次はページの一部の HTML を送る [フラグメント](learn/fragments/) と、チャットの新しいメッセージのように、起きた時点でリストの差分を送る [ストリーム](learn/streams/) です。",
            },
        ),
    ])
}
