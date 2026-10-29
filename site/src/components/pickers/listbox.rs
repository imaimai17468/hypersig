//! The page on `Listbox`: one option per variant of an enum, of which one is
//! selected.

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
        lead(site, Doc::Components(Component::Listbox).leads()),
        demo::show(site, Demo::Listbox),
        accessibility(site),
        builder(site),
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
                en: "It follows the WAI-ARIA listbox pattern for a single selection. The list has `role=\"listbox\"` and the name you give it, and each option `role=\"option\"` with `aria-selected`, so a screen reader announces \"Font, list box\" and then \"Serif, selected, 2 of 3\". The list is one Tab stop, and each option is selected as it gains focus. A check mark, hidden from assistive technology, shows the selected option.",
                ja: "単一選択の WAI-ARIA listbox パターンに従います。一覧は `role=\"listbox\"` と指定した名前を持ち、各選択肢は `role=\"option\"` と `aria-selected` を持つので、スクリーンリーダーは「Font、リストボックス」と読み上げ、続いて「Serif、選択済み、3 個中 2 個目」のように読み上げます。一覧の Tab の停止位置は 1 つで、各選択肢はフォーカスを受けると選ばれます。選ばれた選択肢には、支援技術からは隠れたチェックマークが付きます。",
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
                    &["Tab", "Moves focus into the list, to the selected option."],
                    &[
                        "Up, Down",
                        "Moves to the previous or the next option, wrapping at either end, and selects it.",
                    ],
                    &[
                        "Home, End",
                        "Moves to the first or the last option and selects it.",
                    ],
                ],
                ja: &[
                    &["Tab", "一覧の中の、選ばれた選択肢にフォーカスを移します。"],
                    &[
                        "↑、↓",
                        "前または次の選択肢に移り、端では反対の端に戻り、移った選択肢を選びます。",
                    ],
                    &["Home、End", "最初または最後の選択肢に移り、選びます。"],
                ],
            },
        ),
    ])
}

fn builder(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "`Listbox`",
                ja: "`Listbox`",
            },
        ),
        para(
            site,
            L {
                en: "[`Listbox`](api:hypersig_ui/struct.Listbox.html) takes the list's name, the signal, and a function that gives each variant its title. The value type `T` implements [`Variants`](api:hypersig/trait.Variants.html), which `#[derive(Segment)]` gives a fieldless enum, so the listbox has one option per variant, in declaration order:",
                ja: "[`Listbox`](api:hypersig_ui/struct.Listbox.html) は、一覧の名前、シグナル、各バリアントの見出しを返す関数を受け取ります。値の型 `T` は [`Variants`](api:hypersig/trait.Variants.html) を実装します。フィールドのない列挙型には `#[derive(Segment)]` がこれを与えるので、リストボックスには宣言の順にバリアントごとの選択肢が 1 つずつ並びます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S, T: Variants + Serialize + PartialEq> Listbox<S, T> {\n    \
             pub fn new<L: Into<String>>(label: impl Into<String>, selected: Sig<S, T>, option: impl Fn(&T) -> L) -> Self\n    \
             pub fn render(self) -> El<S>\n}",
        ),
        para(
            site,
            L {
                en: "`option` is called once per variant, so a `match` on the variant names a title for each, and the compiler rejects a `match` that leaves one out. With the signal holding `Plan::Hobby`, a listbox of two plans renders as follows, without the `class`, the check mark's `<svg>`, and the option ids, which come from the signal's path and the option's position:",
                ja: "`option` はバリアントごとに 1 回呼ばれるので、バリアントで `match` すれば各バリアントに見出しを付けられ、バリアントを書き漏らした `match` はコンパイラが拒否します。シグナルが `Plan::Hobby` のとき、2 つのプランのリストボックスは、`class`、チェックマークの `<svg>`、シグナルのパスと選択肢の位置から作られる選択肢の id を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div role=\"listbox\" aria-label=\"Plan\">\n  \
             <div id=\"…\" role=\"option\" aria-selected=\"true\" tabindex=\"0\">Hobby<svg …></svg></div>\n  \
             <div id=\"…\" role=\"option\" aria-selected=\"false\" tabindex=\"-1\">Pro<svg …></svg></div>\n\
             </div>",
        ),
        para(
            site,
            L {
                en: "An enum without `#[derive(Segment)]` does not implement `Variants`, so the listbox cannot list its options, and the call fails to compile with E0277, `Plan does not list its variants`:",
                ja: "`#[derive(Segment)]` のない列挙型は `Variants` を実装しないので、リストボックスは選択肢を並べられず、呼び出しは E0277（`Plan does not list its variants`）でコンパイルできません。",
            },
        ),
        show(Snippet::UiListboxNoSegment),
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
                    en: "Every value of the enum has an option, so the selected value always has one Tab can reach. The types do not check the titles: an empty title leaves an option with no name.",
                    ja: "列挙型のすべての値に選択肢があるので、選ばれた値には必ず Tab キーで到達できる選択肢があります。見出しは型では検査しないので、空の見出しの選択肢には名前がありません。",
                },
                L {
                    en: "Two listboxes bound to one signal on a page get the same option ids.",
                    ja: "1 つのページで同じシグナルに結び付けた 2 つのリストボックスは、選択肢の id が重複します。",
                },
                L {
                    en: "A listbox selects as focus moves, so each arrow key press sets the signal, and a statement that watches the signal runs once per option passed. Where a change calls the server, put a [select](components/select/) or a [radio group](components/radio-group/) in a form, so the server hears only the submitted choice.",
                    ja: "リストボックスはフォーカスの移動と同時に選ぶので、矢印キーを押すたびにシグナルが設定され、シグナルを監視する文は通過した選択肢ごとに実行されます。変更のたびにサーバーを呼び出す場合は、[セレクト](components/select/)か[ラジオグループ](components/radio-group/)をフォームに置き、送信された選択だけがサーバーに届くようにします。",
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
                en: "Example: a font",
                ja: "例: フォント",
            },
        ),
        show(Snippet::UiListbox),
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
                en: "[Combobox](components/combobox/) filters a list of text options as the visitor types. [Tabs](components/tabs/) take their tabs from an enum's variants in the same way. [Keyboard patterns](learn/keyboard/) covers `El::roving_focus`.",
                ja: "入力に合わせて文字列の選択肢を絞り込むのは[コンボボックス](components/combobox/)です。[タブ](components/tabs/)も同じように列挙型のバリアントからタブを作ります。`El::roving_focus` は[キーボード操作](learn/keyboard/)で説明しています。",
            },
        ),
    ])
}
