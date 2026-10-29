//! The page on `radio_group`: a `<fieldset>` of native radio buttons that
//! sets a signal to the chosen option's value.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::RadioGroup).leads()),
        demo::show(site, Demo::RadioGroup),
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
                en: "The radios are native `<input type=\"radio\">` elements that share a `name`, so the browser gives the group one Tab stop and moves between its radios with the arrow keys, checking each one it reaches. The `<fieldset>` and its `<legend>` name the group, so a screen reader announces \"Plan, group\" as focus enters it, then \"Pro, radio button, 2 of 3\". Each radio sits inside the `<label>` that holds its text, as a [checkbox](components/checkbox/) does.",
                ja: "ラジオボタンは同じ `name` を持つネイティブの `<input type=\"radio\">` なので、ブラウザはグループに Tab の停止位置を 1 つだけ与え、矢印キーで移ったラジオボタンにチェックを付けます。`<fieldset>` とその `<legend>` がグループの名前になるので、フォーカスが入るとスクリーンリーダーは「Plan、グループ」と読み上げ、続いて「Pro、ラジオボタン、3 個中 2 個目」のように読み上げます。各ラジオボタンは[チェックボックス](components/checkbox/)と同じく、テキストを入れた `<label>` の中にあります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            L {
                en: &[
                    &["Tab", "Moves focus into the group, to the checked radio."],
                    &[
                        "Arrow keys",
                        "Move to the next or previous radio and check it.",
                    ],
                    &["Space", "Checks the radio that has focus."],
                ],
                ja: &[
                    &[
                        "Tab",
                        "グループの中の、チェックの付いたラジオボタンにフォーカスを移します。",
                    ],
                    &[
                        "矢印キー",
                        "次または前のラジオボタンに移り、チェックを付けます。",
                    ],
                    &[
                        "Space",
                        "フォーカスのあるラジオボタンにチェックを付けます。",
                    ],
                ],
            },
        ),
    ])
}

fn function(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`radio_group`",
                ja: "`radio_group`",
            },
        ),
        para(
            site,
            L {
                en: "[`radio_group`](api:hypersig_ui/fn.radio_group.html) takes the legend, the signal, and the options as pairs of a value and its text. Choosing a radio sets the signal to its value, and the radio whose value equals the signal is checked:",
                ja: "[`radio_group`](api:hypersig_ui/fn.radio_group.html) は、凡例、シグナル、そして値とテキストの組で表した選択肢を受け取ります。ラジオボタンを選ぶとシグナルがその値になり、シグナルと等しい値のラジオボタンにチェックが付きます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn radio_group<S, T: Serialize + PartialEq, L: Into<String>>(\n    \
             title: impl Into<String>,\n    \
             selected: Sig<S, T>,\n    \
             options: impl IntoIterator<Item = (T, L)>,\n) -> El<S>",
        ),
        para(
            site,
            L {
                en: "With the signal holding `Plan::Hobby`, `radio_group(\"Plan\", Billing::plan(), [(Plan::Hobby, \"Hobby\"), (Plan::Pro, \"Pro\")])` renders as follows, without the `class` and the shared `name`, which comes from the signal's path:",
                ja: "シグナルが `Plan::Hobby` のとき、`radio_group(\"Plan\", Billing::plan(), [(Plan::Hobby, \"Hobby\"), (Plan::Pro, \"Pro\")])` は、`class` と、シグナルのパスから作られる共通の `name` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<fieldset>\n  <legend>Plan</legend>\n  \
             <label><span><input name=\"…\" type=\"radio\" checked=\"\"><span aria-hidden=\"true\"></span></span><span>Hobby</span></label>\n  \
             <label><span><input name=\"…\" type=\"radio\"><span aria-hidden=\"true\"></span></span><span>Pro</span></label>\n\
             </fieldset>",
        ),
        para(
            site,
            L {
                en: "Each option's value has the signal's type `T`, so options whose values are text, where the signal holds an enum, fail to compile with E0271, a type mismatch on the iterator's `Item`:",
                ja: "選択肢の値はシグナルの型 `T` なので、シグナルが列挙型を持つところで値を文字列にした選択肢は、イテレーターの `Item` の型の不一致を表す E0271 でコンパイルできません。",
            },
        ),
        show(Snippet::UiRadioWrongOption),
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
                    en: "Nothing checks that every value of `T` has an option. While the signal holds a value no option has, no radio is checked, and Tab stops at the first radio.",
                    ja: "`T` のすべての値に選択肢があるかどうかは検査しません。シグナルがどの選択肢にもない値を持つ間は、どのラジオボタンにもチェックが付かず、Tab キーは最初のラジオボタンで止まります。",
                },
                L {
                    en: "Two groups bound to one signal on a page share one `name`, so the browser treats their radios as one group.",
                    ja: "1 つのページで同じシグナルに結び付けた 2 つのグループは同じ `name` を持つので、ブラウザはそれらのラジオボタンを 1 つのグループとして扱います。",
                },
                L {
                    en: "`radio_group` has no error message. A choice that a form requires uses `radio_group_in_form`, on [Fields bound to a form](components/form-fields/).",
                    ja: "`radio_group` にはエラーメッセージがありません。フォームが必須とする選択には、[フォームに結び付けた入力欄](components/form-fields/)の `radio_group_in_form` を使います。",
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
                en: "Example: a plan",
                ja: "例: プラン",
            },
        ),
        show(Snippet::UiRadioGroup),
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
                en: "[Select](components/select/) takes the same options in less room. [Toggle group](components/toggle-group/) picks one value outside a form, and [Listbox](components/listbox/) takes its options from an enum's variants.",
                ja: "同じ選択肢を狭い場所で選ぶには[セレクト](components/select/)を使います。フォームの外で値を 1 つ選ぶのは[トグルグループ](components/toggle-group/)で、列挙型のバリアントから選択肢を作るのは[リストボックス](components/listbox/)です。",
            },
        ),
    ])
}
