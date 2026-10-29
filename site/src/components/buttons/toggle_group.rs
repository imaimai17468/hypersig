//! The page on `ToggleGroup`: a row of toggles that picks one value of a
//! signal, on the radio group pattern.

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
        lead(site, Doc::Components(Component::ToggleGroup).leads()),
        demo::show(site, Demo::Toggle),
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
                en: "Exactly one toggle is on at a time, so the group follows the WAI-ARIA radio group pattern. The group has `role=\"radiogroup\"` and the name you give it, and each toggle has `role=\"radio\"` and `aria-checked`. A screen reader announces the Alignment group in the demo above as a radio group and each toggle as \"Center, radio button, 2 of 3\". The group is one Tab stop: Tab enters it at the toggle that is on, and the arrow keys move between toggles and turn on the one they reach.",
                ja: "常に 1 つのトグルだけがオンになるので、グループは WAI-ARIA の radio group パターンに従います。グループは `role=\"radiogroup\"` と指定した名前を持ち、各トグルは `role=\"radio\"` と `aria-checked` を持ちます。上のデモの「配置」のグループは、スクリーンリーダーではラジオグループとして読み上げられ、各トグルは「中央、ラジオボタン、3 個中 2 個目」のように読み上げられます。グループの Tab の停止位置は 1 つです。Tab キーでオンのトグルに入り、矢印キーで移ったトグルがオンになります。",
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
                        "Tab",
                        "Moves focus into the group, to the toggle that is on.",
                    ],
                    &[
                        "Left, Right",
                        "Moves to the previous or the next toggle, wrapping at either end, and turns it on.",
                    ],
                    &[
                        "Home, End",
                        "Moves to the first or the last toggle and turns it on.",
                    ],
                    &["Enter, Space", "Turns on the toggle that has focus."],
                ],
                ja: &[
                    &[
                        "Tab",
                        "グループの中の、オンのトグルにフォーカスを移します。",
                    ],
                    &[
                        "←、→",
                        "前または次のトグルに移り、端では反対の端に戻り、移ったトグルをオンにします。",
                    ],
                    &["Home、End", "最初または最後のトグルに移り、オンにします。"],
                    &["Enter、Space", "フォーカスのあるトグルをオンにします。"],
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
                en: "`ToggleGroup`",
                ja: "`ToggleGroup`",
            },
        ),
        para(
            site,
            L {
                en: "[`ToggleGroup`](api:hypersig_ui/struct.ToggleGroup.html) is a builder: `new` takes the group's name and the signal, each `item` or `icon_item` adds a toggle with the value it picks, and `render` returns the `<div>`. `El::child` takes the builder itself, so the `render` call is optional:",
                ja: "[`ToggleGroup`](api:hypersig_ui/struct.ToggleGroup.html) はビルダーです。`new` はグループの名前とシグナルを受け取り、`item` と `icon_item` は選ぶ値を持つトグルを 1 つずつ加え、`render` が `<div>` を返します。`El::child` はビルダーをそのまま受け取るので、`render` の呼び出しは省けます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S, T: Serialize + PartialEq> ToggleGroup<S, T> {\n    \
             pub fn new(label: impl Into<String>, selected: Sig<S, T>) -> Self\n    \
             pub fn item(self, value: T, text: impl Into<String>) -> Self\n    \
             pub fn icon_item(self, value: T, label: impl Into<String>, icon: impl Into<Node<S>>) -> Self\n    \
             pub fn render(self) -> El<S>\n}",
        ),
        para(
            site,
            L {
                en: "A toggle from `icon_item` is square and named by `label` through `aria-label`. With the signal holding `Plan::Hobby`, `ToggleGroup::new(\"Plan\", Billing::plan()).item(Plan::Hobby, \"Hobby\").item(Plan::Pro, \"Pro\")` renders as follows, without the `class`:",
                ja: "`icon_item` のトグルは正方形で、`label` が `aria-label` で名前になります。シグナルが `Plan::Hobby` のとき、`ToggleGroup::new(\"Plan\", Billing::plan()).item(Plan::Hobby, \"Hobby\").item(Plan::Pro, \"Pro\")` は、`class` を除くと次のように描画されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "<div role=\"radiogroup\" aria-label=\"Plan\">\n  \
             <button type=\"button\" role=\"radio\" aria-checked=\"true\" tabindex=\"0\">Hobby</button>\n  \
             <button type=\"button\" role=\"radio\" aria-checked=\"false\" tabindex=\"-1\">Pro</button>\n\
             </div>",
        ),
        para(
            site,
            L {
                en: "Each `item` takes a value of the signal's type `T`, so a value of another type, such as text where the signal holds an enum, fails to compile with E0308:",
                ja: "`item` はシグナルの型 `T` の値を受け取るので、シグナルが列挙型を持つところに文字列を渡すような、別の型の値は E0308 でコンパイルできません。",
            },
        ),
        show(Snippet::UiGroupWrongValue),
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
                    en: "Nothing checks that every value of `T` has a toggle. While the signal holds a value no toggle has, no toggle is on, every toggle has `tabindex=\"-1\"`, and Tab skips the group, so the keyboard cannot reach it. Give the signal an initial value that has a toggle. [Listbox](components/listbox/) takes its options from the variants instead.",
                    ja: "`T` のすべての値にトグルがあるかどうかは検査しません。シグナルがどのトグルにもない値を持つ間は、オンのトグルがなく、どのトグルも `tabindex=\"-1\"` なので、Tab キーはグループを飛ばし、キーボードでは到達できません。シグナルの初期値にはトグルのある値を与えます。[リストボックス](components/listbox/)は代わりにバリアントから選択肢を作ります。",
                },
                L {
                    en: "Two items with one value are both on at once, and both are Tab stops.",
                    ja: "同じ値の項目が 2 つあると、両方が同時にオンになり、両方が Tab の停止位置になります。",
                },
                L {
                    en: "An empty group name compiles, and a screen reader then announces the group without saying what it chooses.",
                    ja: "グループの名前を空にしてもコンパイルでき、スクリーンリーダーは何を選ぶグループかを伝えずに読み上げます。",
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
                en: "Example: text alignment",
                ja: "例: 文字の配置",
            },
        ),
        show(Snippet::UiToggleGroup),
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
                en: "[Multiple-choice toggle group](components/multi-toggle-group/) lets any number of toggles be on. [Radio group](components/radio-group/) picks one value in a form with native radio buttons. [Keyboard patterns](learn/keyboard/) covers `El::roving_focus`, which moves focus within the group.",
                ja: "いくつでもオンにできるのは[複数選択のトグルグループ](components/multi-toggle-group/)です。フォームでネイティブのラジオボタンから値を 1 つ選ぶには[ラジオグループ](components/radio-group/)を使います。グループの中でフォーカスを移す `El::roving_focus` は[キーボード操作](learn/keyboard/)で説明しています。",
            },
        ),
    ])
}
