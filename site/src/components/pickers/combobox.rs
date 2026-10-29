//! The page on `Combobox`, `ComboboxState`, and `ComboboxText`: a text field
//! with a list of options that the text filters.

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
        lead(site, Doc::Components(Component::Combobox).leads()),
        demo::show(site, Demo::Combobox),
        accessibility(site),
        handle(site),
        state(site),
        text(site),
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
                en: "It follows the WAI-ARIA combobox pattern with list autocomplete. The text field has `role=\"combobox\"`, `aria-autocomplete=\"list\"`, `aria-expanded`, `aria-controls`, and `aria-activedescendant`. Focus stays in the text field while the arrow keys move through the options shown, so the visitor can keep typing, and a screen reader announces each option the arrows reach. Typing shows the options that contain the text, ignoring case.",
                ja: "リスト型の自動補完を持つ WAI-ARIA combobox パターンに従います。テキストフィールドは `role=\"combobox\"`、`aria-autocomplete=\"list\"`、`aria-expanded`、`aria-controls`、`aria-activedescendant` を持ちます。矢印キーで表示中の選択肢を移る間もフォーカスはテキストフィールドにとどまるので、訪問者はそのまま入力を続けられ、スクリーンリーダーは矢印キーで移った選択肢を読み上げます。入力すると、大文字と小文字を区別せずに、その文字列を含む選択肢が表示されます。",
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
                    &[
                        "Typing",
                        "Opens the list and shows the options that contain the text.",
                    ],
                    &[
                        "Down",
                        "Opens the list, then moves to the next option shown.",
                    ],
                    &["Up", "Moves to the previous option shown."],
                    &[
                        "Enter",
                        "Chooses the option the arrow keys moved to, and closes the list.",
                    ],
                    &[
                        "Escape",
                        "Closes the list, or clears the text when the list is already closed.",
                    ],
                    &["Tab", "Leaves the field and closes the list."],
                ],
                ja: &[
                    &[
                        "文字の入力",
                        "一覧を開き、その文字列を含む選択肢を表示します。",
                    ],
                    &["↓", "一覧を開き、その後は表示中の次の選択肢に移ります。"],
                    &["↑", "表示中の前の選択肢に移ります。"],
                    &["Enter", "矢印キーで移った選択肢を選び、一覧を閉じます。"],
                    &[
                        "Escape",
                        "一覧を閉じます。一覧が閉じているときは文字列を消します。",
                    ],
                    &["Tab", "入力欄から離れ、一覧を閉じます。"],
                ],
            },
        ),
    ])
}

fn handle(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`Combobox`",
                ja: "`Combobox`",
            },
        ),
        para(
            site,
            L {
                en: "[`Combobox`](api:hypersig_ui/struct.Combobox.html) is a [handle](components/handles/): declare it as a `const` from two node refs, one for the text field and one for the list, since the runtime finds both by name. `render` takes the label, the state signal, the options as text, and the words of `ComboboxText`:",
                ja: "[`Combobox`](api:hypersig_ui/struct.Combobox.html) は[ハンドル](components/handles/)です。ランタイムはテキストフィールドと一覧の両方を名前で探すので、それぞれ 1 つずつ、2 つの node ref から `const` として宣言します。`render` は、ラベル、状態のシグナル、文字列の選択肢、`ComboboxText` の文言を受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Combobox<S> {\n    \
             pub const fn new(field: NodeRef<S>, list: NodeRef<S>) -> Self\n    \
             pub fn render<L: Into<String>>(\n        self,\n        label_text: impl Into<String>,\n        state: Sig<S, ComboboxState>,\n        options: impl IntoIterator<Item = L>,\n        text: &ComboboxText,\n    ) -> El<S>\n}",
        ),
        para(
            site,
            L {
                en: "With the field empty and the list closed, a combobox of two countries renders as follows, without the `class`, the check marks' `<svg>`, and the option ids. The field's `id` and the list's `id` are the names of the two node refs, `shop::COUNTRY_FIELD` and `shop::COUNTRY_OPTIONS` here:",
                ja: "入力欄が空で一覧が閉じているとき、2 つの国のコンボボックスは、`class`、チェックマークの `<svg>`、選択肢の id を除くと次のように描画されます。入力欄の `id` と一覧の `id` は 2 つの node ref の名前で、ここでは `shop::COUNTRY_FIELD` と `shop::COUNTRY_OPTIONS` です。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div>\n  <label for=\"shop::COUNTRY_FIELD\">Country</label>\n  <div>\n    \
             <input id=\"shop::COUNTRY_FIELD\" type=\"text\" role=\"combobox\" aria-autocomplete=\"list\" aria-controls=\"shop::COUNTRY_OPTIONS\" autocomplete=\"off\" aria-expanded=\"false\" aria-activedescendant=\"\" value=\"\">\n    \
             <div> <!-- shown while the list is open -->\n      <div id=\"shop::COUNTRY_OPTIONS\" role=\"listbox\" aria-label=\"Suggestions\">\n        \
             <div id=\"…\" role=\"option\" aria-selected=\"false\">France</div>\n        \
             <div id=\"…\" role=\"option\" aria-selected=\"false\">Japan</div>\n      </div>\n      \
             <p>No matches</p> <!-- shown while no option matches -->\n    </div>\n  </div>\n</div>",
        ),
        para(
            site,
            L {
                en: "`new` given one node ref for both fails to compile with E0080, as [Handles](components/handles/) shows. One combobox placed twice on a page makes `Page::render` panic.",
                ja: "`new` に両方で同じ node ref を渡すと、[ハンドル](components/handles/)にあるとおり E0080 でコンパイルできません。1 つのコンボボックスをページに 2 回置くと `Page::render` が panic します。",
            },
        ),
    ])
}

fn state(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`ComboboxState`",
                ja: "`ComboboxState`",
            },
        ),
        para(
            site,
            L {
                en: "[`ComboboxState`](api:hypersig_ui/struct.ComboboxState.html) is the struct of signals a combobox keeps: the text, whether its list is open, which option the arrow keys reached, whether the pointer is on the list, and whether the text field has focus. Put one in the page's signals per combobox. Only `text` is public, and it is what the visitor typed or chose:",
                ja: "[`ComboboxState`](api:hypersig_ui/struct.ComboboxState.html) は、コンボボックスが持つシグナルの構造体です。文字列、一覧が開いているか、矢印キーで移った選択肢、ポインターが一覧の上にあるか、テキストフィールドにフォーカスがあるかを持ちます。コンボボックスごとに 1 つをページのシグナルに置きます。公開されているのは `text` だけで、訪問者が入力したか選んだ文字列です。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "#[derive(Signals, Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]\n\
             pub struct ComboboxState {\n    pub text: String,\n    // and four private fields\n}\n\n\
             impl ComboboxState {\n    pub fn new(text: impl Into<String>) -> Self\n}",
        ),
        para(
            site,
            L {
                en: "`ComboboxState::new` makes a closed combobox whose field holds `text`, and `state.at(ComboboxState::text())` reads the text in an expression. `render` takes a `Sig<S, ComboboxState>`, so a plain `String` signal fails to compile with E0308:",
                ja: "`ComboboxState::new` は、入力欄に `text` を入れた閉じたコンボボックスを作り、式の中では `state.at(ComboboxState::text())` で文字列を読みます。`render` は `Sig<S, ComboboxState>` を受け取るので、ただの `String` のシグナルは E0308 でコンパイルできません。",
            },
        ),
        show(Snippet::UiComboboxStringState),
    ])
}

fn text(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`ComboboxText`",
                ja: "`ComboboxText`",
            },
        ),
        para(
            site,
            L {
                en: "[`ComboboxText`](api:hypersig_ui/struct.ComboboxText.html) holds the two words the combobox adds: `list`, the list's name, \"Suggestions\" by default, and `empty`, shown when no option matches, \"No matches\" by default. A page in another language passes its own, as [Text in other languages](components/component-text/) shows.",
                ja: "[`ComboboxText`](api:hypersig_ui/struct.ComboboxText.html) は、コンボボックスが加える 2 つの文言を持ちます。一覧の名前である `list`（既定は \"Suggestions\"）と、一致する選択肢がないときに表示する `empty`（既定は \"No matches\"）です。ほかの言語のページは[ほかの言語の文言](components/component-text/)にあるとおり、自分の文言を渡します。",
            },
        ),
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
                    en: "The visitor can leave text that matches no option. Where the page needs one of the options, check the text in a form rule or on the server.",
                    ja: "訪問者は、どの選択肢にも一致しない文字列を残せます。選択肢のどれかが必要なページでは、フォームのルールかサーバーで文字列を検査します。",
                },
                L {
                    en: "Escape is taken by the combobox while focus is in its text field, so a dialog around it closes on Escape only from elsewhere.",
                    ja: "フォーカスがテキストフィールドにある間は Escape をコンボボックスが受け取るので、コンボボックスを囲むダイアログが Escape で閉じるのは、フォーカスがほかの場所にあるときだけです。",
                },
                L {
                    en: "Every option is in the page and is filtered in the browser, so a list of thousands makes the page that large. A long list that the server searches belongs in an action.",
                    ja: "選択肢はすべてページに入り、ブラウザで絞り込まれるので、数千件の一覧ならページもその分大きくなります。サーバーで検索する長い一覧はアクションで扱います。",
                },
                L {
                    en: "Two options with the same text both show and both get a check mark when chosen.",
                    ja: "同じ文字列の選択肢が 2 つあると、両方が表示され、選んだときには両方にチェックマークが付きます。",
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
                en: "Example: a country",
                ja: "例: 国",
            },
        ),
        show(Snippet::UiCombobox),
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
                en: "[Listbox](components/listbox/) shows every option of an enum at once. [Handles](components/handles/) covers the `const` handle. [Keyboard patterns](learn/keyboard/) covers `El::active_descendant`, which keeps focus in the text field.",
                ja: "列挙型のすべての選択肢を一度に見せるのは[リストボックス](components/listbox/)です。`const` のハンドルは[ハンドル](components/handles/)で説明しています。フォーカスをテキストフィールドにとどめる `El::active_descendant` は[キーボード操作](learn/keyboard/)で説明しています。",
            },
        ),
    ])
}
