//! The page on `select_field`: a labeled native `<select>` bound to a
//! signal.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Select).leads()),
        demo::show(site, Demo::Select),
        accessibility(site),
        function(site),
        unchecked(site),
        example(site),
        next(site),
    ])
}

fn accessibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Accessibility",
                ja: "アクセシビリティ",
            },
        ),
        para(
            site,
            L {
                en: "It is a native `<select>`, named by its `<label>`, so a screen reader announces \"Region, Tokyo, popup button\" or the like, and the keyboard and a phone's picker wheel work as on any select. Its keys are the platform's, which differ between systems: on most, Space or Alt+Down opens the list, the arrow keys move through the options, typing jumps to an option by its first letters, and Enter chooses.",
                ja: "`<label>` で名前の付いたネイティブの `<select>` なので、スクリーンリーダーは「Region、Tokyo、ポップアップボタン」のように読み上げ、キーボードやスマートフォンのピッカーもほかのセレクトと同じように動きます。キー操作はプラットフォームのもので、システムによって異なります。多くのシステムでは、Space か Alt+↓ で一覧を開き、矢印キーで選択肢を移り、文字を入力するとその文字で始まる選択肢に移り、Enter で選びます。",
            },
        ),
        para(
            site,
            L {
                en: "The closed control is drawn like a text field with a chevron. Where the browser supports `appearance: base-select` and `::picker(select)`, which browser-compat-data 8.1.6 lists for Chrome 135 and Safari 27, the open list is drawn as a card too. Elsewhere it is the browser's own list.",
                ja: "閉じたコントロールは山形の印を付けたテキストフィールドのように描きます。ブラウザが `appearance: base-select` と `::picker(select)` に対応していれば、開いた一覧もカードとして描きます。browser-compat-data 8.1.6 では、対応しているのは Chrome 135 と Safari 27 です。それ以外のブラウザでは、ブラウザ自身の一覧になります。",
            },
        ),
    ])
}

fn function(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`select_field`",
                ja: "`select_field`",
            },
        ),
        para(
            site,
            L {
                en: "[`select_field`](api:hypersig_ui/fn.select_field.html) takes the label, the signal, and the options as pairs of a value and its text, as `radio_group` does:",
                ja: "[`select_field`](api:hypersig_ui/fn.select_field.html) は `radio_group` と同じく、ラベル、シグナル、そして値とテキストの組で表した選択肢を受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn select_field<S, T: Serialize + PartialEq, L: Into<String>>(\n    \
             title: impl Into<String>,\n    \
             selected: Sig<S, T>,\n    \
             options: impl IntoIterator<Item = (T, L)>,\n) -> El<S>",
        ),
        para(
            site,
            L {
                en: "The value can be any type that serializes, such as an enum or a number. With the signal holding `Plan::Hobby`, `select_field(\"Plan\", Billing::plan(), [(Plan::Hobby, \"Hobby\"), (Plan::Pro, \"Pro\")])` renders as follows, without the `class`, the chevron's `<svg>`, the ids from the signal's path, and each option's `value`, which the page reads back into the signal:",
                ja: "値は列挙型や数値など、シリアライズできる型なら何でも使えます。シグナルが `Plan::Hobby` のとき、`select_field(\"Plan\", Billing::plan(), [(Plan::Hobby, \"Hobby\"), (Plan::Pro, \"Pro\")])` は、`class`、山形の `<svg>`、シグナルのパスから作られる id、ページがシグナルに読み戻す各選択肢の `value` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div>\n  <label for=\"…\">Plan</label>\n  <div>\n    <select id=\"…\">\n      <option selected=\"\">Hobby</option>\n      <option>Pro</option>\n    </select>\n    <svg …></svg>\n  </div>\n</div>",
        ),
        para(
            site,
            L {
                en: "Each option's value has the signal's type `T`, so the options of another enum fail to compile with E0271:",
                ja: "選択肢の値はシグナルの型 `T` なので、別の列挙型の選択肢は E0271 でコンパイルできません。",
            },
        ),
        show(Snippet::UiSelectOtherEnum),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
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
                    en: "Nothing checks that the signal's value has an option. While it holds a value no option has, the browser shows the first option as chosen, though the signal still holds the other value.",
                    ja: "シグナルの値に対応する選択肢があるかどうかは検査しません。どの選択肢にもない値を持つ間、ブラウザは最初の選択肢を選択中として表示しますが、シグナルは別の値のままです。",
                },
                L {
                    en: "Two options with one value both match the signal, and the browser shows the last of them as chosen.",
                    ja: "同じ値の選択肢が 2 つあると両方がシグナルと一致し、ブラウザはそのうち最後のものを選択中として表示します。",
                },
                L {
                    en: "`select_field` has no error message. A choice that a form requires uses `select_field_in_form`, on [Fields bound to a form](components/form-fields/).",
                    ja: "`select_field` にはエラーメッセージがありません。フォームが必須とする選択には、[フォームに結び付けた入力欄](components/form-fields/)の `select_field_in_form` を使います。",
                },
            ],
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a region and a number of replicas",
                ja: "例: リージョンとレプリカ数",
            },
        ),
        show(Snippet::UiSelect),
    ])
}

fn next(site: Site) -> Part {
    parts([
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
                en: "[Radio group](components/radio-group/) shows every option at once. [Combobox](components/combobox/) narrows a long list as the visitor types. [Binding form controls](learn/inputs/) covers `El::bind_select`, which this component is built on.",
                ja: "すべての選択肢を一度に見せるには[ラジオグループ](components/radio-group/)を使います。長い一覧を入力で絞り込むには[コンボボックス](components/combobox/)を使います。このコンポーネントが使っている `El::bind_select` は[入力欄の束縛](learn/inputs/)で説明しています。",
            },
        ),
    ])
}
