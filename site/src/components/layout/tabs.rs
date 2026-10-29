//! The component page on `Tabs`: a tab list over one panel per variant of
//! an enum signal.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Tabs).leads()),
        demo::show(site, Demo::Tabs),
        accessibility(site),
        building(site),
        html(site),
        unchecked(site),
        example(site),
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
                en: "The tabs follow the WAI-ARIA tabs pattern. A screen reader announces the list by its label, each tab as a tab with whether it is selected and its position in the list, and each panel by the title of its tab. A tab is selected as soon as it gets focus, so the arrow keys show its panel without Enter.",
                ja: "タブは WAI-ARIA の tabs パターンに従います。スクリーンリーダーは一覧をそのラベルで読み上げ、各タブを、選ばれているかどうかと一覧の中の位置を添えてタブとして読み上げます。パネルは対応するタブのタイトルで読み上げます。タブはフォーカスを受けた時点で選ばれるので、矢印キーで移るだけで、Enter を押さなくてもパネルが切り替わります。",
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
                        "Moves focus into the list at the selected tab, then on to its panel.",
                    ],
                    &[
                        "Left, Right",
                        "Move to the previous or next tab and select it.",
                    ],
                    &["Home, End", "Move to the first or last tab and select it."],
                ],
                ja: &[
                    &[
                        "Tab",
                        "一覧の中の選ばれたタブにフォーカスを移し、もう一度押すとそのパネルに移します。",
                    ],
                    &["左矢印、右矢印", "前か次のタブに移り、それを選びます。"],
                    &["Home、End", "最初か最後のタブに移り、それを選びます。"],
                ],
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Declaring the tabs",
                ja: "タブを宣言する",
            },
        ),
        para(
            site,
            L {
                en: "[`Tabs::new`](api:hypersig_ui/struct.Tabs.html#method.new) takes the name of the list, the signal that holds the selected tab, and a function that returns the title and the panel of each value:",
                ja: "[`Tabs::new`](api:hypersig_ui/struct.Tabs.html#method.new) は一覧の名前、選ばれたタブを持つシグナル、各値のタイトルとパネルを返す関数を受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S, T: Variants + Serialize + PartialEq> Tabs<S, T> {
    pub fn new<L: Into<String>>(
        label: impl Into<String>,
        selected: Sig<S, T>,
        tab: impl Fn(&T) -> (L, El<S>),
    ) -> Self;
    pub fn render(self) -> El<S>;
}",
        ),
        para(
            site,
            L {
                en: "`T` is an enum with `#[derive(Segment)]`, which implements `Variants`, the list of its variants in declaration order. `new` calls `tab` once per variant in that order, so the tabs appear as the variants are declared. `Tabs` converts into an `El<S>`, so `El::child` takes it without `render`.",
                ja: "`T` は `#[derive(Segment)]` を付けた列挙型です。この derive は宣言順のバリアントの一覧である `Variants` を実装します。`new` はその順に各バリアントで `tab` を 1 回ずつ呼ぶので、タブはバリアントの宣言順に並びます。`Tabs` は `El<S>` に変換できるので、`render` を呼ばずに `El::child` に渡せます。",
            },
        ),
        para(
            site,
            L {
                en: "Because `tab` is called with each variant, a `match` in it names a title and a panel for every variant, and a `match` that leaves one out does not compile. Adding a variant to the enum later stops the build until the variant has its tab:",
                ja: "`tab` は各バリアントで呼ばれるので、その中の `match` はすべてのバリアントにタイトルとパネルを書くことになり、1 つでも書き漏らした `match` はコンパイルできません。あとから列挙型にバリアントを足すと、そのタブを書くまでビルドが止まります。",
            },
        ),
        show(Snippet::UiTabsMissing),
        para(
            site,
            L {
                en: "The signal's type also rules out a selected value with no tab: every value of the enum is a variant, and every variant has a tab, so Tab always finds a selected tab to move to. A `String` or an `i32` signal has no `Variants` and is E0277.",
                ja: "シグナルの型により、タブのない値が選ばれることもありません。列挙型の値はどれもバリアントで、どのバリアントにもタブがあるので、Tab キーで移る先の選ばれたタブが必ずあります。`String` や `i32` のシグナルは `Variants` を実装しないので E0277 になります。",
            },
        ),
    ])
}

fn html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTML",
                ja: "出力される HTML",
            },
        ),
        para(
            site,
            L {
                en: "For a page whose signal starts at `Section::Password`, the tabs render as follows, with the `aria-controls` and `aria-labelledby` values tying each tab to its panel:",
                ja: "シグナルの最初の値が `Section::Password` のページでは、タブは次のように描画されます。`aria-controls` と `aria-labelledby` の値が各タブとそのパネルを結び付けます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<div>
  <div role="tablist" aria-label="Settings">
    <button type="button" role="tab" aria-controls="…" aria-selected="false" tabindex="-1">Account</button>
    <button type="button" role="tab" aria-controls="…" aria-selected="true" tabindex="0">Password</button>
    <button type="button" role="tab" aria-controls="…" aria-selected="false" tabindex="-1">Billing</button>
  </div>
  <div role="tabpanel" aria-labelledby="…" tabindex="0"><p>Change your name and email.</p></div>
  <div role="tabpanel" aria-labelledby="…" tabindex="0"><p>Passwords are at least twelve characters.</p></div>
  <div role="tabpanel" aria-labelledby="…" tabindex="0"><p>Invoices arrive on the first of each month.</p></div>
</div>"#,
        ),
        para(
            site,
            L {
                en: "Every panel is in the HTML, and the browser shows only the panel of the selected tab. Selecting another tab moves `aria-selected=\"true\"` and `tabindex=\"0\"` to it and shows its panel, with no request to the server. Since the selection is a signal of the page, a handler that patches it switches the tab, and the first value of the page decides the tab the visitor sees first.",
                ja: "パネルはすべて HTML に含まれ、ブラウザは選ばれたタブのパネルだけを表示します。別のタブを選ぶと、`aria-selected=\"true\"` と `tabindex=\"0\"` がそのタブに移り、そのパネルが表示されます。サーバーへのリクエストはありません。選択はページのシグナルなので、ハンドラがパッチでシグナルを書き換えればタブが切り替わり、ページの最初の値が、訪問者が最初に見るタブを決めます。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What is not checked",
                ja: "検査されないこと",
            },
        ),
        para(
            site,
            L {
                en: "The ids that tie the tabs to their panels come from the signal's path, so two tab lists bound to one signal would carry the same ids. `Page::render` panics on such a page, naming the id of the first panel:",
                ja: "タブとパネルを結び付ける id はシグナルのパスから作られるので、1 つのシグナルに 2 つのタブの一覧を束縛すると、同じ id が 2 回現れます。そのようなページでは `Page::render` が最初のパネルの id を挙げてパニックします。",
            },
        ),
        show(Snippet::UiTabsTwice),
        para(
            site,
            L {
                en: "Give each tab list a signal of its own. The label and the titles are plain strings, so nothing checks that the label names the list or that a title is not empty. A tab with an empty title is announced as a tab with no name.",
                ja: "タブの一覧ごとに別のシグナルを用意してください。ラベルとタイトルはただの文字列なので、ラベルが一覧を表しているか、タイトルが空でないかは検査されません。タイトルが空のタブは、名前のないタブとして読み上げられます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a settings page",
                ja: "例: 設定ページ",
            },
        ),
        para(
            site,
            L {
                en: "The page opens on the Password tab, because that is the first value of its signal:",
                ja: "シグナルの最初の値が Password なので、このページは Password のタブを開いた状態で表示されます。",
            },
        ),
        show(Snippet::UiTabs),
        para(
            site,
            L {
                en: "Next, [Accordion](components/accordion/) stacks sections that the visitor opens and closes without a signal.",
                ja: "次の [アコーディオン](components/accordion/) ではシグナルを使わずに訪問者が開閉するセクションを縦に並べます。",
            },
        ),
    ])
}
