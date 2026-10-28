//! The page on keyboard patterns: focus moved with the arrow keys, the
//! active option of a listbox, and shortcuts that move focus through node
//! refs.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Keyboard).title(site)),
        lead(site, Doc::Learn(Learn::Keyboard).leads()),
        heading(
            site,
            L {
                en: "Arrow keys between items",
                ja: "矢印キーで項目間を移動する",
            },
        ),
        para(
            site,
            L {
                en: "A tab list, a menu, or a toolbar is one stop for Tab, and the arrow keys move between its items. [`El::roving_focus`](api:hypersig/html/struct.El.html#method.roving_focus), `fn roving_focus(self, orientation: Orientation) -> Self`, gives an element that behavior. [`Orientation`](api:hypersig/html/enum.Orientation.html) picks the keys: `Horizontal` uses left and right, swapped where the element's `direction` is `rtl`, and `Vertical` uses up and down.",
                ja: "タブの一覧、メニュー、ツールバーは、Tab キーでは 1 か所として扱い、その中の項目間は矢印キーで移動します。[`El::roving_focus`](api:hypersig/html/struct.El.html#method.roving_focus)（`fn roving_focus(self, orientation: Orientation) -> Self`）は要素にこの動作を与えます。[`Orientation`](api:hypersig/html/enum.Orientation.html) が使うキーを決めます。`Horizontal` は左右の矢印キーを使い、要素の `direction` が `rtl` なら左右を入れ替えます。`Vertical` は上下の矢印キーを使います。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "An item is an element inside this one that has a `tabindex` attribute and is neither disabled nor `aria-disabled=\"true\"`. Where a nearer element inside this one also has `roving_focus`, the item belongs to that one.",
                    ja: "項目とはこの要素の中にあり、`tabindex` 属性を持ち、無効でも `aria-disabled=\"true\"` でもない要素です。この要素の中に、より近い位置で `roving_focus` を持つ要素があれば、項目はそちらに属します。",
                },
                L {
                    en: "The two arrow keys move to the next and the previous item, wrapping at either end. Home and End move to the first and the last.",
                    ja: "2 つの矢印キーで次と前の項目に移動し、端まで行くと反対の端に戻ります。Home と End で最初と最後の項目に移動します。",
                },
                L {
                    en: "The browser calls `preventDefault` for a key it acts on. It leaves a key alone when a handler from `El::on_key` already took it, or while Alt, Control, or Meta is held.",
                    ja: "ブラウザは処理したキーに対して `preventDefault` を呼びます。`El::on_key` のハンドラがすでにそのキーを処理した場合と、Alt、Control、Meta が押されている間は、キーに何もしません。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "Only focus moves, so each item keeps the `tabindex` it is given. A tab list gives the selected tab `tabindex=\"0\"` and the others `-1` with `bind_attr`, so that Tab enters the list at the selected tab, and selects a tab when it gains focus with `Event::Focus`:",
                ja: "動くのはフォーカスだけなので、各項目の `tabindex` は与えたまま変わりません。タブの一覧では、`bind_attr` で選択中のタブに `tabindex=\"0\"`、ほかのタブに `-1` を与えます。こうすると Tab キーで一覧に入ったときに選択中のタブにフォーカスが移ります。また、`Event::Focus` を使って、フォーカスを受け取ったタブを選択します。",
            },
        ),
        show(Snippet::RovingFocus),
        para(
            site,
            L {
                en: "The types leave the items unchecked. An element without `tabindex` is not an item, so the arrow keys skip it, and giving every item `tabindex=\"0\"` makes Tab stop at each of them.",
                ja: "項目の内容は型では検査されません。`tabindex` のない要素は項目にならないので、矢印キーで飛ばされます。すべての項目に `tabindex=\"0\"` を与えると、Tab キーが項目ごとに止まります。",
            },
        ),
        heading(
            site,
            L {
                en: "The active option of a listbox",
                ja: "リストボックスのアクティブな選択肢",
            },
        ),
        para(
            site,
            L {
                en: "In a combobox, focus stays in the text field while the up and down arrows move through the options of a listbox. [`El::active_descendant`](api:hypersig/html/struct.El.html#method.active_descendant), `fn active_descendant(self, listbox: NodeRef<S>, active: Sig<S, String>) -> Self`, goes on the field. `listbox` names the listbox, and `active` holds the id of the active option.",
                ja: "コンボボックスでは、フォーカスをテキスト欄に置いたまま、上下の矢印キーでリストボックスの選択肢を移動します。[`El::active_descendant`](api:hypersig/html/struct.El.html#method.active_descendant)（`fn active_descendant(self, listbox: NodeRef<S>, active: Sig<S, String>) -> Self`）はテキスト欄に付けます。`listbox` はリストボックスを指し、`active` はアクティブな選択肢の id を持ちます。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "An option is an element inside `listbox` with `role=\"option\"` and an `id`, which is not `aria-disabled=\"true\"` and is not hidden by `El::show`.",
                    ja: "選択肢とは `listbox` の中にあって `role=\"option\"` と `id` を持ち、`aria-disabled=\"true\"` でも `El::show` で隠れてもいない要素です。",
                },
                L {
                    en: "The down arrow sets `active` to the id of the next shown option, and the up arrow to the previous one, wrapping at either end. Where `active` names no shown option, they start from the first or the last.",
                    ja: "下矢印キーで `active` を次に表示されている選択肢の id に、上矢印キーで前の選択肢の id にし、端まで行くと反対の端に戻ります。`active` が表示中のどの選択肢も指していなければ、最初か最後の選択肢から始めます。",
                },
                L {
                    en: "Enter clicks the option `active` names. The browser calls `preventDefault` for a key it acts on, scrolls the new option into view, and leaves a key alone while Alt, Control, or Meta is held.",
                    ja: "Enter で `active` が指す選択肢をクリックします。ブラウザは処理したキーに対して `preventDefault` を呼び、新しい選択肢が見える位置までスクロールします。Alt、Control、Meta が押されている間は、キーに何もしません。",
                },
                L {
                    en: "It runs before the handlers of `El::on_key`, which then still see the key.",
                    ja: "`El::on_key` のハンドラより先に動き、そのあとハンドラにもキーが渡ります。",
                },
            ],
        ),
        show(Snippet::ActiveDescendant),
        para(
            site,
            L {
                en: "The method also binds `aria-activedescendant` to `active`, so a screen reader reads the option `active` names. Nothing else changes, so style that option yourself, for example with `css_when`.",
                ja: "このメソッドは `aria-activedescendant` も `active` に束縛するので、スクリーンリーダーは `active` が指す選択肢を読み上げます。ほかには何も変わらないので、その選択肢の見た目は `css_when` などで自分で指定してください。",
            },
        ),
        para(
            site,
            L {
                en: "`active` holds an id, so a signal of another type is E0308, mismatched types:",
                ja: "`active` には id が入るので、ほかの型のシグナルを渡すと E0308（型の不一致）になります。",
            },
        ),
        show(Snippet::ActiveDescendantType),
        para(
            site,
            L {
                en: "The ids in `active` are not checked against the options. Where `active` holds an id that no option carries, `aria-activedescendant` names no element, and the next arrow key starts from the first or the last option.",
                ja: "`active` の id が選択肢の id と合っているかは検査されません。どの選択肢も持たない id が `active` に入っていると、`aria-activedescendant` はどの要素も指さず、次に矢印キーを押したときに最初か最後の選択肢から始まります。",
            },
        ),
        heading(
            site,
            L {
                en: "Shortcuts that move focus",
                ja: "フォーカスを動かすショートカット",
            },
        ),
        para(
            site,
            L {
                en: "A node ref's statements and a key handler together make the patterns a search box needs. [Events](learn/events/) covers `on_key`, `Key`, and `Shortcut`. Below, `/` anywhere on the page outside a text field moves focus to the search field, and Escape in the field clears it and removes focus from it:",
                ja: "ノード参照の文とキーのハンドラを組み合わせると、検索欄に必要な操作を作れます。`on_key`、`Key`、`Shortcut` は [イベント](learn/events/) で説明します。下の例では、テキスト欄の外でページのどこかにフォーカスがあるときに `/` を押すと検索欄にフォーカスが移り、検索欄で Escape を押すと内容を消してフォーカスを外します。",
            },
        ),
        show(Snippet::SearchKeys),
        heading(
            site,
            L {
                en: "The components built on them",
                ja: "これらを使ったコンポーネント",
            },
        ),
        para(
            site,
            L {
                en: "`hypersig_ui` builds its [tabs](components/tabs/), [menu](components/menu/), [listbox](components/listbox/), and [combobox](components/combobox/) with `roving_focus` and `active_descendant`, together with the roles and the `aria-*` attributes each pattern needs. Reach for those first, and for the two methods when a pattern of your own needs the keys.",
                ja: "`hypersig_ui` の [タブ](components/tabs/)、[メニュー](components/menu/)、[リストボックス](components/listbox/)、[コンボボックス](components/combobox/) は、`roving_focus` と `active_descendant` に、それぞれのパターンに必要なロールと `aria-*` 属性を組み合わせて作っています。まずはこれらを使い、独自のパターンでキー操作が必要なときにこの 2 つのメソッドを使ってください。",
            },
        ),
        para(
            site,
            L {
                en: "Next, [JavaScript libraries](learn/javascript/) runs a map or an editor inside a page.",
                ja: "次の [JavaScript のライブラリ](learn/javascript/) では、地図やエディタをページの中で動かす方法を説明します。",
            },
        ),
    ])
}
