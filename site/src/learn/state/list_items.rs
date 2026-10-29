//! The page on the fields and browser state of one list item.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::ListItems).title(site)),
        lead(site, Doc::Learn(Learn::ListItems).leads()),
        heading(
            site,
            L {
                en: "The fields of one item",
                ja: "1 つの項目のフィールド",
            },
        ),
        para(
            site,
            L {
                en: "`Sig` has a third type parameter that says where its path starts. It defaults to [`Root`](api:hypersig/enum.Root.html), the page's signal struct, so `Sig<S, T>` is a field reached from the page. [`Item::at`](api:hypersig/struct.Item.html#method.at), `fn at<U>(self, field: Sig<T, U>) -> Sig<S, U, Local>`, gives a field of an item, whose path starts at the item and is resolved per item in the browser. [`Local`](api:hypersig/enum.Local.html) is writable, so `set`, `toggle`, `bind`, and the list statements work on an item's field as on a page field:",
                ja: "`Sig` には、パスの起点を表す 3 つ目の型パラメータがあります。省略すると [`Root`](api:hypersig/enum.Root.html)（ページのシグナル構造体）になるので、`Sig<S, T>` はページから辿ったフィールドです。[`Item::at`](api:hypersig/struct.Item.html#method.at)（`fn at<U>(self, field: Sig<T, U>) -> Sig<S, U, Local>`）は項目のフィールドを返し、そのパスは項目から始まり、ブラウザで項目ごとに解決されます。[`Local`](api:hypersig/enum.Local.html) は書き込めるので、`set`、`toggle`、`bind`、リストの文はページのフィールドと同じように項目のフィールドにも使えます。",
            },
        ),
        show(Snippet::StateItemFields),
        heading(
            site,
            L {
                en: "The server cannot patch one item's field",
                ja: "サーバーは 1 つの項目のフィールドを差分で変えられない",
            },
        ),
        para(
            site,
            L {
                en: "A `Sig<S, T, Local>` has no `patch` method, so the server cannot address one item's field by position, which would land on another item once the list changed:",
                ja: "`Sig<S, T, Local>` には `patch` メソッドがないので、サーバーは位置を指定して 1 つの項目のフィールドを書き換えられません。位置で指定すると、リストが変わったあとは別の項目に書き込んでしまうからです。",
            },
        ),
        show(Snippet::StatePatchOnItem),
        para(
            site,
            L {
                en: "The server instead replaces the whole list with a patch of the list field, changes the items it names by key with a list patch such as `patch_append` or `patch_remove`, or re-renders a part of the page as a fragment. [List patches](learn/list-patches/) covers list patches, and [Fragments](learn/fragments/) covers fragments.",
                ja: "サーバーはリストのフィールドの差分でリスト全体を置き換えるか、`patch_append` や `patch_remove` のようなリストの差分でキーを指定した項目だけを変えるか、ページの一部をフラグメントとして描画し直します。リストの差分は [リストの差分](learn/list-patches/) で、フラグメントは [フラグメント](learn/fragments/) で説明します。",
            },
        ),
        heading(
            site,
            L {
                en: "Choosing per item",
                ja: "項目ごとに選ぶ",
            },
        ),
        para(
            site,
            L {
                en: "The closure passed to `each` runs once on the server to build a template, so it cannot branch on an item's value in Rust: an `if` there would decide for every item at once. An expression decides per item in the browser, through `show`, `if_else`, or a binding:",
                ja: "`each` に渡すクロージャはテンプレートを作るためにサーバーで 1 回だけ実行されるので、Rust の側で項目の値によって分岐することはできません。そこに `if` を書くと、すべての項目についてまとめて決めてしまいます。項目ごとの選択は `show`、`if_else`、束縛を通して、ブラウザで式が行います。",
            },
        ),
        show(Snippet::StateItemBranch),
        heading(
            site,
            L {
                en: "Lists inside list items",
                ja: "リスト項目の中のリスト",
            },
        ),
        para(
            site,
            L {
                en: "A field of an item can be a list too, and `each` takes it like any list signal. The inner closure can read both its own item and the item of the list around it:",
                ja: "項目のフィールドもリストにでき、`each` はほかのリストのシグナルと同じようにそれを受け取ります。内側のクロージャは自分の項目と、それを囲むリストの項目の両方を読めます。",
            },
        ),
        show(Snippet::StateNestedLists),
        heading(
            site,
            L {
                en: "Browser-only state of an item",
                ja: "ブラウザだけで持つ項目の状態",
            },
        ),
        para(
            site,
            L {
                en: "A patch from the server writes each item it carries whole. `patch_upsert` puts the server's item in place of the one with the same key, and `patch` replaces the whole list. A field held on the item only for the browser, such as whether a card is open for editing, takes the value the server sent, so a card someone is editing closes when another user's change to it arrives. Keep such state at the page's root as the key of the item it applies to, and compare it with each item's key:",
                ja: "サーバーからの差分は、運ぶ項目を丸ごと書き込みます。`patch_upsert` は同じキーの項目をサーバーの項目で置き換え、`patch` はリスト全体を置き換えます。カードを編集中かどうかのように、ブラウザのためだけに項目に持たせたフィールドはサーバーが送った値になるので、編集中のカードはほかのユーザーの変更が届いた時点で閉じます。こうした状態は対象の項目のキーとしてページの構造体の直下に持ち、各項目のキーと比べてください。",
            },
        ),
        show(Snippet::EditState),
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型では検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "An `Item` and the signals built from it belong to the closure they were passed to. The types do not stop one from being moved out, and reading it after the closure returns, or rendering what was built from it outside its list, panics.",
                    ja: "`Item` とそこから作ったシグナルは、渡されたクロージャに属します。外へ持ち出すことは型では防げず、クロージャが戻ったあとに読んだり、そこから作ったものをそのリストの外で描画したりすると panic します。",
                },
                L {
                    en: "`Item::at` refuses a field marked `#[signals(computed)]`, and `html::part` takes only a field from the page root, so neither a computed signal nor a `Part` can sit at an item's field.",
                    ja: "`Item::at` は `#[signals(computed)]` を付けたフィールドを拒否し、`html::part` はページのルートからのフィールドしか受け取りません。そのため、項目のフィールドに計算シグナルや `Part` を置くことはできません。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        para(
            site,
            L {
                en: "[Actions](learn/actions/) calls the server from a statement, and [List patches](learn/list-patches/) answers with changes to a list.",
                ja: "[アクション](learn/actions/) では文からサーバーを呼び出し、[リストの差分](learn/list-patches/) ではリストへの変更で応答します。",
            },
        ),
    ])
}
