//! The page on the words the components show or announce on their own, and
//! the structs that replace them for a page in another language.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Translations).leads()),
        structs(site),
        methods(site),
        missing(site),
        unchecked(site),
        example(site),
        next(site),
    ])
}

fn structs(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The text structs",
                ja: "文言の構造体",
            },
        ),
        para(
            site,
            L {
                en: "Every word a component needs from you is an argument: a field's label, a button's text, an icon button's name. The words a component adds by itself, such as the name of a dialog's close button, come from a struct with one public `String` field per word. Its `Default` is English, and the method that renders the component takes it by reference:",
                ja: "コンポーネントが必要とする文言のうち、入力欄のラベル、ボタンの文字、アイコンボタンの名前のように呼び出し側が決めるものは、すべて引数です。ダイアログの閉じるボタンの名前のように、コンポーネントが自分で加える文言は、1 語につき 1 つの公開 `String` フィールドを持つ構造体から取ります。その `Default` は英語で、コンポーネントを描画するメソッドが参照で受け取ります。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Struct", "Taken by", "Fields and their English defaults"],
                ja: &["構造体", "受け取るメソッド", "フィールドと英語の既定値"],
            },
            L {
                en: &[
                    &[
                        "[`ComboboxText`](api:hypersig_ui/struct.ComboboxText.html)",
                        "`Combobox::render`",
                        "`list`: \"Suggestions\", the list's name. `empty`: \"No matches\", shown when no option matches.",
                    ],
                    &[
                        "[`DialogText`](api:hypersig_ui/struct.DialogText.html)",
                        "`Dialog::dialog`, `Dialog::sheet`",
                        "`close`: \"Close\", the name of the button in the corner.",
                    ],
                    &[
                        "[`PaginationText`](api:hypersig_ui/struct.PaginationText.html)",
                        "`pagination`",
                        "`label`: \"Pagination\", the `<nav>`'s name. `previous`: \"Previous\". `next`: \"Next\".",
                    ],
                    &[
                        "[`TableText`](api:hypersig_ui/struct.TableText.html)",
                        "`DataTable`, `ServerTable`",
                        "`empty`: \"No results.\". `select_row`: \"Select row\". `select_all`: \"Select all rows\". `previous_page`: \"Previous page\". `next_page`: \"Next page\". `expand_row`: \"Show row details\".",
                    ],
                ],
                ja: &[
                    &[
                        "[`ComboboxText`](api:hypersig_ui/struct.ComboboxText.html)",
                        "`Combobox::render`",
                        "`list`: \"Suggestions\"（一覧の名前）。`empty`: \"No matches\"（一致する選択肢がないときの表示）。",
                    ],
                    &[
                        "[`DialogText`](api:hypersig_ui/struct.DialogText.html)",
                        "`Dialog::dialog`、`Dialog::sheet`",
                        "`close`: \"Close\"（右上のボタンの名前）。",
                    ],
                    &[
                        "[`PaginationText`](api:hypersig_ui/struct.PaginationText.html)",
                        "`pagination`",
                        "`label`: \"Pagination\"（`<nav>` の名前）。`previous`: \"Previous\"。`next`: \"Next\"。",
                    ],
                    &[
                        "[`TableText`](api:hypersig_ui/struct.TableText.html)",
                        "`DataTable`、`ServerTable`",
                        "`empty`: \"No results.\"。`select_row`: \"Select row\"。`select_all`: \"Select all rows\"。`previous_page`: \"Previous page\"。`next_page`: \"Next page\"。`expand_row`: \"Show row details\"。",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "Each struct derives `Debug`, `Clone`, `PartialEq`, and `Eq`. `ComboboxText` is declared as:",
                ja: "どの構造体も `Debug`、`Clone`、`PartialEq`、`Eq` を derive しています。たとえば `ComboboxText` の宣言は次のとおりです。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub struct ComboboxText {\n    pub list: String,\n    pub empty: String,\n}\n\nimpl Default for ComboboxText { /* \"Suggestions\", \"No matches\" */ }",
        ),
        para(
            site,
            L {
                en: "The words reach the page as text and as `aria-label`. With `ComboboxText { list: \"候補\".into(), empty: \"一致する候補はありません\".into() }`, the combobox renders:",
                ja: "文言はテキストか `aria-label` としてページに入ります。`ComboboxText { list: \"候補\".into(), empty: \"一致する候補はありません\".into() }` を渡すと、コンボボックスは次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div role=\"listbox\" aria-label=\"候補\">…</div>\n<p>一致する候補はありません</p>",
        ),
    ])
}

fn methods(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Words set by a method",
                ja: "メソッドで設定する文言",
            },
        ),
        para(
            site,
            L {
                en: "[`Toaster`](api:hypersig_ui/struct.Toaster.html) is a builder, so it takes its two words through methods instead of a struct: `label` names the region in place of \"Notifications\", and `dismiss_label` names the button that removes a toast in place of \"Dismiss\".",
                ja: "[`Toaster`](api:hypersig_ui/struct.Toaster.html) はビルダーなので、2 つの文言を構造体ではなくメソッドで受け取ります。`label` は \"Notifications\" の代わりに領域の名前を、`dismiss_label` は \"Dismiss\" の代わりにトーストを消すボタンの名前を設定します。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Toaster<S> {\n    pub fn label(self, label: impl Into<String>) -> Self\n    pub fn dismiss_label(self, label: impl Into<String>) -> Self\n}",
        ),
    ])
}

fn missing(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A word left out",
                ja: "抜けた文言",
            },
        ),
        para(
            site,
            L {
                en: "The fields are public and the structs have no constructor, so a translation is a struct expression that names every field. One that leaves a field out fails to compile with E0063, so a word added to a struct in a later version shows up as an error in every translation of it:",
                ja: "フィールドは公開されていて、構造体にコンストラクタはないので、翻訳はすべてのフィールドを書いた構造体式になります。フィールドを 1 つでも抜かすと E0063 でコンパイルできません。そのため、後のバージョンで構造体に文言が増えると、それを翻訳しているすべての箇所がエラーになります。",
            },
        ),
        show(Snippet::UiTextMissingField),
        para(
            site,
            L {
                en: "Writing `..ComboboxText::default()` after the fields you translate fills the rest with English and turns the error off, so leave it out of a translation.",
                ja: "翻訳したフィールドの後に `..ComboboxText::default()` と書くと、残りが英語で埋まり、エラーも出なくなります。翻訳ではこの書き方を避けます。",
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
                    en: "Nothing ties a text struct to the page's language. A Japanese page that passes `DialogText::default()` names its close button \"Close\" in English.",
                    ja: "文言の構造体とページの言語は結び付いていません。日本語のページで `DialogText::default()` を渡すと、閉じるボタンの名前は英語の \"Close\" になります。",
                },
                L {
                    en: "An empty string compiles. An empty `close` leaves the close button with no name, which a screen reader announces as just \"button\".",
                    ja: "空の文字列もコンパイルできます。`close` を空にすると閉じるボタンに名前がなくなり、スクリーンリーダーは「ボタン」としか読み上げません。",
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
                en: "Example: one function per struct",
                ja: "例: 構造体ごとに関数を 1 つ",
            },
        ),
        para(
            site,
            L {
                en: "A function per struct that matches on the page's language keeps every translation in one place, and the `match` stops compiling when a language is added without its words:",
                ja: "ページの言語で `match` する関数を構造体ごとに用意すると、翻訳を 1 か所にまとめられます。言語を追加して文言を書き忘れると、`match` がコンパイルできなくなります。",
            },
        ),
        show(Snippet::UiTexts),
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
                en: "[Writing a text](i18n/texts/) covers writing every text of a page once per language with `hypersig_i18n`. [Combobox](components/combobox/), [Dialog](components/dialog/), [Pagination](components/pagination/), and [Data table](components/data-table/) show each struct in use.",
                ja: "`hypersig_i18n` でページのすべての文言を言語ごとに書く方法は[文を書く](i18n/texts/)で説明しています。各構造体の使い方は[コンボボックス](components/combobox/)、[ダイアログ](components/dialog/)、[ページネーション](components/pagination/)、[データテーブル](components/data-table/)のページにあります。",
            },
        ),
    ])
}
