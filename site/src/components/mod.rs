//! The Components section: setting up `hypersig_ui`, theming it, and one
//! page per component.
//!
//! A component page shows the component running with its source, says what
//! it is built on, lists the keys it answers to, and links its items in the
//! rustdoc output under `api/`, as the Reference section does.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::{L, Text};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

mod buttons;
mod display;
mod forms;
mod overlays;
mod pickers;
mod start;

/// One item: its name, its rustdoc page and anchor under `api/`, and its
/// summary.
type Row = (&'static str, &'static str, L<&'static str>);

/// What a component page holds.
struct Page {
    lead: L<&'static str>,
    demo: Demo,
    /// What the component is built on and what assistive technology hears.
    built_on: &'static [L<&'static str>],
    /// Each key and what it does, empty where the component takes no keys.
    keys: &'static [(L<&'static str>, L<&'static str>)],
    items: &'static [Row],
}

/// The article of `page`.
#[expect(
    clippy::match_same_arms,
    reason = "each group's arms change when its pages are written"
)]
pub fn content(site: Site, page: Component) -> Part {
    let head = title(Doc::Components(page).title(site));
    match page {
        // Get started
        Component::Installation => start::installation::page(site, head),
        Component::Theming => start::theming::page(site, head),
        Component::Tokens => start::tokens::page(site, head),
        Component::Handles => start::handles::page(site, head),
        Component::Translations => start::component_text::page(site, head),
        // Buttons
        Component::Button => buttons::button::page(site, head),
        Component::IconButton => buttons::icon_button::page(site, head),
        Component::LinkButton => buttons::link_button::page(site, head),
        Component::Toggle => buttons::toggle::page(site, head),
        Component::ToggleGroup => buttons::toggle_group::page(site, head),
        Component::MultiToggleGroup => buttons::multi_toggle_group::page(site, head),
        // Form fields
        Component::TextField => forms::text_field::page(site, head),
        Component::NumberField => forms::number_field::page(site, head),
        Component::DateField => forms::date_field::page(site, head),
        Component::Checkbox => forms::checkbox::page(site, head),
        Component::Switch => forms::switch::page(site, head),
        Component::RadioGroup => forms::radio_group::page(site, head),
        Component::Select => forms::select::page(site, head),
        Component::Slider => forms::slider::page(site, head),
        Component::FormFields => forms::form_fields::page(site, head),
        // Pickers
        Component::Listbox => pickers::listbox::page(site, head),
        Component::Combobox => pickers::combobox::page(site, head),
        // Layout
        Component::Tabs
        | Component::Accordion
        | Component::Collapsible
        | Component::Card
        | Component::Separator => carved(site, page),
        // Navigation
        Component::Navigation | Component::Breadcrumb | Component::Pagination => carved(site, page),
        // Data
        Component::DataTable
        | Component::TableSorting
        | Component::TableFiltering
        | Component::TableSelection
        | Component::ServerTable
        | Component::Chart
        | Component::BarChart
        | Component::DonutChart => carved(site, page),
        // Overlays
        Component::Dialog => overlays::dialog::page(site, head),
        Component::AlertDialog => overlays::alert_dialog::page(site, head),
        Component::Sheet => overlays::sheet::page(site, head),
        Component::Menu => overlays::menu::page(site, head),
        Component::Tooltip => overlays::tooltip::page(site, head),
        Component::Popover => overlays::popover::page(site, head),
        Component::Toast => overlays::toast::page(site, head),
        Component::ServerToasts => overlays::server_toasts::page(site, head),
        // Display
        Component::Alert => display::alert::page(site, head),
        Component::Badge => display::badge::page(site, head),
        Component::Progress => display::progress::page(site, head),
        Component::Skeleton => display::skeleton::page(site, head),
        Component::Spinner => display::spinner::page(site, head),
        Component::Avatar => display::avatar::page(site, head),
        Component::Kbd => display::kbd::page(site, head),
    }
}

/// The text of the page `page` is carved from, until `page` has its own.
fn carved(site: Site, page: Component) -> Part {
    component(site, page, &component_page(carved_from(page)))
}

/// The page whose text `page` shows until it is written.
fn carved_from(page: Component) -> Component {
    match page {
        // Navigation
        Component::Breadcrumb | Component::Pagination => Component::Navigation,
        // Data
        Component::TableSorting
        | Component::TableFiltering
        | Component::TableSelection
        | Component::ServerTable => Component::DataTable,
        // Data
        Component::BarChart | Component::DonutChart => Component::Chart,
        other => other,
    }
}

fn component(site: Site, page: Component, content: &Page) -> Part {
    let mut children = vec![
        title(Doc::Components(page).title(site)),
        lead(site, content.lead),
        demo::show(site, content.demo),
    ];
    children.push(heading(
        site,
        L {
            en: "Accessibility",
            ja: "アクセシビリティ",
        },
    ));
    children.extend(content.built_on.iter().map(|&text| para(site, text)));
    if !content.keys.is_empty() {
        let rows: Vec<[&str; 2]> = content
            .keys
            .iter()
            .map(|&(key, action)| [site.t(key), site.t(action)])
            .collect();
        let rows: Vec<&[&str]> = rows.iter().map(<[&str; 2]>::as_slice).collect();
        children.push(table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            Text::Chosen(rows.as_slice()),
        ));
    }
    children.push(heading(
        site,
        L {
            en: "API",
            ja: "API",
        },
    ));
    let cells: Vec<[String; 2]> = content
        .items
        .iter()
        .map(|&(name, target, summary)| {
            [
                format!("[`{name}`](api:{target})"),
                site.t(summary).to_owned(),
            ]
        })
        .collect();
    let rows: Vec<[&str; 2]> = cells
        .iter()
        .map(|[name, summary]| [name.as_str(), summary.as_str()])
        .collect();
    let rows: Vec<&[&str]> = rows.iter().map(<[&str; 2]>::as_slice).collect();
    children.push(table_of(
        site,
        L {
            en: &["Item", "Summary"],
            ja: &["項目", "概要"],
        },
        Text::Chosen(rows.as_slice()),
    ));
    parts(children)
}

/// The lead of the page of `page`, a component rather than Installation or
/// Theming.
pub(crate) fn component_lead(page: Component) -> L<&'static str> {
    component_page(carved_from(page)).lead
}

/// The content of a component page.
#[expect(clippy::too_many_lines, reason = "one block of data per component")]
fn component_page(page: Component) -> Page {
    match page {
        // Get started, Buttons, Form fields, and Pickers
        Component::Installation
        | Component::Theming
        | Component::Tokens
        | Component::Handles
        | Component::Translations
        | Component::Button
        | Component::IconButton
        | Component::LinkButton
        | Component::Toggle
        | Component::ToggleGroup
        | Component::MultiToggleGroup
        | Component::TextField
        | Component::NumberField
        | Component::DateField
        | Component::Checkbox
        | Component::Switch
        | Component::RadioGroup
        | Component::Select
        | Component::Slider
        | Component::FormFields
        | Component::Listbox
        | Component::Combobox
        // Overlays and Display
        | Component::Dialog
        | Component::AlertDialog
        | Component::Sheet
        | Component::Menu
        | Component::Tooltip
        | Component::Popover
        | Component::Toast
        | Component::ServerToasts
        | Component::Alert
        | Component::Badge
        | Component::Progress
        | Component::Skeleton
        | Component::Spinner
        | Component::Avatar
        | Component::Kbd => unreachable!("written in its own file"),
        Component::Breadcrumb
        | Component::Pagination
        | Component::TableSorting
        | Component::TableFiltering
        | Component::TableSelection
        | Component::ServerTable
        | Component::BarChart
        | Component::DonutChart => unreachable!("carved_from maps it to its page"),
        Component::Tabs => Page {
            lead: L {
                en: "A tab list that keeps the selected tab in a signal, with one panel per tab.",
                ja: "選ばれたタブをシグナルに持つタブの一覧です。タブごとに 1 つのパネルがあります。",
            },
            demo: Demo::Tabs,
            built_on: &[L {
                en: "It follows the WAI-ARIA tabs pattern. The list has `role=\"tablist\"`, each tab `role=\"tab\"` with `aria-selected` and `aria-controls`, and each panel `role=\"tabpanel\"` labeled by its tab. A tab is selected as it gains focus, through `El::roving_focus`. The ids are derived from the signal's path and the tab's position. The signal holds an enum with `#[derive(Segment)]`, and the tabs are its variants, so the selected value always has a tab that Tab reaches, and a `match` that leaves a variant without a tab fails to compile.",
                ja: "WAI-ARIA tabs パターンに従います。一覧は `role=\"tablist\"` を持ち、各タブは `role=\"tab\"` と `aria-selected`、`aria-controls` を持ちます。各パネルは `role=\"tabpanel\"` を持ち、対応するタブがパネルのラベルになります。タブは `El::roving_focus` でフォーカスを受けると同時に選ばれます。id はシグナルのパスとタブの位置から作られます。シグナルの値は `#[derive(Segment)]` を付けた列挙型で、タブはそのバリアントです。そのため、選ばれた値には必ず Tab キーで移れるタブがあり、タブのないバリアントを残した `match` はコンパイルできません。",
            }],
            keys: &[
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Moves focus into the list at the selected tab, then on to the panel.",
                        ja: "一覧の中の選ばれたタブにフォーカスを移し、もう一度押すとパネルに移します。",
                    },
                ),
                (
                    L {
                        en: "Left, Right",
                        ja: "左矢印、右矢印",
                    },
                    L {
                        en: "Move to the previous or next tab and select it.",
                        ja: "前か次のタブに移り、それを選びます。",
                    },
                ),
                (
                    L {
                        en: "Home, End",
                        ja: "Home、End",
                    },
                    L {
                        en: "Move to the first or last tab and select it.",
                        ja: "最初か最後のタブに移り、それを選びます。",
                    },
                ),
            ],
            items: &[(
                "Tabs::new",
                "hypersig_ui/struct.Tabs.html#method.new",
                L {
                    en: "A tab list named `label`, with the title and panel `tab` gives each variant.",
                    ja: "`label` を名前に持ち、`tab` が各バリアントに返すタイトルとパネルを並べるタブの一覧",
                },
            )],
        },
        Component::Accordion => Page {
            lead: L {
                en: "A stack of sections that open and close, one at a time or each on its own.",
                ja: "開閉するセクションを縦に並べたものです。一度に 1 つだけ開く形と、それぞれが独立に開く形があります。",
            },
            demo: Demo::Accordion,
            built_on: &[
                L {
                    en: "Each section is a native `<details>` whose `<summary>` opens and closes it. The browser handles the keyboard, and each summary is announced as a button that is expanded or collapsed, and find-in-page opens a closed section that holds a match.",
                    ja: "各セクションはネイティブの `<details>` で、その `<summary>` がセクションを開閉します。キーボード操作はブラウザが扱い、各 summary は展開か折りたたみの状態を持つボタンとして読み上げられます。ページ内検索で一致した箇所が閉じたセクションにあれば、ブラウザがそのセクションを開きます。",
                },
                L {
                    en: "An exclusive accordion gives its sections one `name`, so the browser closes the open one when another opens. The name is an `Id` that the accordion's `<div>` also carries, so two accordions given one `Id::new` make the page panic when it renders, where they would otherwise close each other's sections.",
                    ja: "排他的なアコーディオンはセクションに同じ `name` を付けるので、別のセクションが開くと、ブラウザが開いていたセクションを閉じます。この名前は `Id` で、アコーディオンの `<div>` も同じ id を持ちます。そのため、1 つの `Id::new` を 2 つのアコーディオンに渡すと、互いのセクションを閉じ合う代わりに、ページの描画がパニックします。",
                },
            ],
            keys: &[(
                L {
                    en: "Enter, Space",
                    ja: "Enter、Space",
                },
                L {
                    en: "Opens or closes the focused section.",
                    ja: "フォーカスのあるセクションを開くか閉じます。",
                },
            )],
            items: &[
                (
                    "Accordion::new",
                    "hypersig_ui/struct.Accordion.html#method.new",
                    L {
                        en: "An accordion whose sections open and close independently.",
                        ja: "セクションがそれぞれ独立に開閉するアコーディオン",
                    },
                ),
                (
                    "Accordion::exclusive",
                    "hypersig_ui/struct.Accordion.html#method.exclusive",
                    L {
                        en: "An accordion that keeps at most one section open.",
                        ja: "開いているセクションを多くても 1 つにするアコーディオン",
                    },
                ),
                (
                    "item",
                    "hypersig_ui/struct.Accordion.html#method.item",
                    L {
                        en: "Adds a section titled `title` whose content is `content`.",
                        ja: "`title` をタイトルに持ち、中身が `content` のセクションを追加します。",
                    },
                ),
            ],
        },
        Component::Collapsible => Page {
            lead: L {
                en: "A button that shows and hides one region, bound to a `bool` signal.",
                ja: "1 つの領域を表示したり隠したりするボタンです。`bool` のシグナルに束縛します。",
            },
            demo: Demo::Collapsible,
            built_on: &[
                L {
                    en: "It follows the WAI-ARIA disclosure pattern. The button is a native `<button>` with `aria-expanded` bound to the signal and `aria-controls` pointing at the region, so a screen reader announces it as expanded or collapsed. The region's id is derived from the signal's path.",
                    ja: "WAI-ARIA disclosure パターンに従います。ボタンはネイティブの `<button>` で、`aria-expanded` をシグナルに束縛し、`aria-controls` で領域を指します。そのため、スクリーンリーダーは展開されているか折りたたまれているかを読み上げます。領域の id はシグナルのパスから作られます。",
                },
                L {
                    en: "The button and the region are separate elements, so the button can sit in a heading or a toolbar away from the region. An `Accordion` is built on `<details>` instead, and no signal holds which of its sections is open.",
                    ja: "ボタンと領域は別の要素なので、ボタンを領域から離れた見出しやツールバーに置けます。`Accordion` は代わりに `<details>` の上に作られており、どのセクションが開いているかはシグナルに入りません。",
                },
            ],
            keys: &[(
                L {
                    en: "Enter, Space",
                    ja: "Enter、Space",
                },
                L {
                    en: "Shows or hides the region.",
                    ja: "領域を表示するか隠します。",
                },
            )],
            items: &[
                (
                    "Collapsible::new",
                    "hypersig_ui/struct.Collapsible.html#method.new",
                    L {
                        en: "A collapsible whose region is shown while `open` is true.",
                        ja: "`open` が true の間、領域を表示する折りたたみ",
                    },
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Collapsible.html#method.trigger",
                    L {
                        en: "The button that shows and hides the region.",
                        ja: "領域を表示したり隠したりするボタン",
                    },
                ),
                (
                    "content",
                    "hypersig_ui/struct.Collapsible.html#method.content",
                    L {
                        en: "The region, empty, for the content added with `El::child`.",
                        ja: "`El::child` で内容を加える、空の領域",
                    },
                ),
            ],
        },
        Component::Card => Page {
            lead: L {
                en: "A bordered box with a header, content, and a footer.",
                ja: "ヘッダー、中身、フッターを持つ、枠線付きのボックスです。",
            },
            demo: Demo::Card,
            built_on: &[L {
                en: "A card is a `<div>` and takes no role. Its title is a heading whose level you choose with `Level`, so that it sits one level below the section around it and a screen reader's list of headings stays in order.",
                ja: "カードは `<div>` で、ロールを持ちません。タイトルは見出しで、そのレベルは `Level` で選びます。周りのセクションより 1 つ下のレベルにすると、スクリーンリーダーの見出しの一覧が順番どおりに並びます。",
            }],
            keys: &[],
            items: &[
                (
                    "card",
                    "hypersig_ui/fn.card.html",
                    L {
                        en: "A `<div>` with a border, a shadow, and `token::card` behind it.",
                        ja: "枠線と影を持ち、背景が `token::card` の `<div>`",
                    },
                ),
                (
                    "card_header",
                    "hypersig_ui/fn.card_header.html",
                    L {
                        en: "The top of a `card`, holding its title and description.",
                        ja: "タイトルと説明を入れる、`card` の上部",
                    },
                ),
                (
                    "card_title",
                    "hypersig_ui/fn.card_title.html",
                    L {
                        en: "The title of a `card`, as a heading of `level`.",
                        ja: "`level` の見出しとして描く、`card` のタイトル",
                    },
                ),
                (
                    "card_description",
                    "hypersig_ui/fn.card_description.html",
                    L {
                        en: "A line under the title of a `card`, in `token::muted_foreground`.",
                        ja: "`card` のタイトルの下に `token::muted_foreground` の色で書く 1 行",
                    },
                ),
                (
                    "card_content",
                    "hypersig_ui/fn.card_content.html",
                    L {
                        en: "The body of a `card`.",
                        ja: "`card` の本体",
                    },
                ),
                (
                    "card_footer",
                    "hypersig_ui/fn.card_footer.html",
                    L {
                        en: "The bottom of a `card`, laying its children out in a row.",
                        ja: "子要素を横に並べる、`card` の下部",
                    },
                ),
                (
                    "Level",
                    "hypersig_ui/enum.Level.html",
                    L {
                        en: "The level of a heading, which follows the headings around it.",
                        ja: "周りの見出しに合わせて決める、見出しのレベル",
                    },
                ),
            ],
        },
        Component::Separator => Page {
            lead: L {
                en: "A one-pixel line between groups of content, across or along.",
                ja: "内容のまとまりの間に引く、横向きか縦向きの 1 ピクセルの線です。",
            },
            demo: Demo::Separator,
            built_on: &[L {
                en: "A horizontal separator is an `<hr>`, and a vertical one is a `<div>` with `role=\"separator\"` and `aria-orientation=\"vertical\"`, so a screen reader announces both as separators.",
                ja: "横向きの区切り線は `<hr>` で、縦向きの区切り線は `role=\"separator\"` と `aria-orientation=\"vertical\"` を持つ `<div>` です。そのため、スクリーンリーダーはどちらも区切り線として読み上げます。",
            }],
            keys: &[],
            items: &[(
                "separator",
                "hypersig_ui/fn.separator.html",
                L {
                    en: "A one-pixel line in `token::border` that runs along `orientation`.",
                    ja: "`orientation` の向きに引く、`token::border` の色の 1 ピクセルの線",
                },
            )],
        },
        Component::Navigation => Page {
            lead: L {
                en: "Links between the pages of an application: a list of its sections, a breadcrumb trail, and the pages of a long list.",
                ja: "アプリケーションのページの間を移るリンクです。セクションの一覧、パンくずリスト、長い一覧のページ送りがあります。",
            },
            demo: Demo::Navigation,
            built_on: &[
                L {
                    en: "Each is a `<nav>` named by the label it takes, holding a list of links, so a screen reader lists it among the page's landmarks. The links are typed `Link` values from routes, and the one to the page the visitor is on has `aria-current=\"page\"`.",
                    ja: "どれも受け取ったラベルを名前に持つ `<nav>` で、中にリンクの一覧を持ちます。そのため、スクリーンリーダーはページのランドマークの 1 つとして一覧に出します。リンクはルートから作った型付きの `Link` で、訪問者がいるページへのリンクは `aria-current=\"page\"` を持ちます。",
                },
                L {
                    en: "`NavList` decides the mark on the server, from the page's own `Url` that `Route::url` writes. A link to the same path has `aria-current=\"page\"`, and a link to a path that the page's path begins with has `aria-current=\"true\"`. `Breadcrumb::current` ends the trail with the current page as text and returns the element, so no link can follow it. `pagination` shows the first and the last page and the pages next to the current one, and marks the previous or next link `aria-disabled` where that page does not exist. The links in the demo lead to pages of this site.",
                    ja: "`NavList` は `Route::url` で書いたページ自身の `Url` をもとに、どのリンクに印を付けるかをサーバーで決めます。同じパスへのリンクは `aria-current=\"page\"` を持ち、ページのパスがリンク先のパスの続きになっているリンクは `aria-current=\"true\"` を持ちます。`Breadcrumb::current` は今のページをテキストとして最後に置き、要素を返すので、そのあとにリンクは続けられません。`pagination` は最初と最後のページ、今のページの前後のページを表示し、前や次のページがないときはそのリンクを `aria-disabled` にします。デモのリンクは、このサイトのページにつながります。",
                },
            ],
            keys: &[],
            items: &[
                (
                    "NavList::new",
                    "hypersig_ui/struct.NavList.html#method.new",
                    L {
                        en: "An empty list whose `<nav>` is named `label`, on the page at `current`.",
                        ja: "`<nav>` の名前が `label` で、`current` のページに置く空の一覧",
                    },
                ),
                (
                    "item",
                    "hypersig_ui/struct.NavList.html#method.item",
                    L {
                        en: "Adds a link to `route` with the arguments `args`, as `Route::url` takes them.",
                        ja: "`Route::url` と同じ形の引数 `args` を付けた、`route` へのリンクを追加します。",
                    },
                ),
                (
                    "Breadcrumb::new",
                    "hypersig_ui/struct.Breadcrumb.html#method.new",
                    L {
                        en: "An empty trail whose `<nav>` is named `label`, such as \"Breadcrumb\".",
                        ja: "`<nav>` の名前が `label`（たとえば「Breadcrumb」）の空のパンくずリスト",
                    },
                ),
                (
                    "link",
                    "hypersig_ui/struct.Breadcrumb.html#method.link",
                    L {
                        en: "Adds a link to a page above the current one.",
                        ja: "今のページより上のページへのリンクを追加します。",
                    },
                ),
                (
                    "current",
                    "hypersig_ui/struct.Breadcrumb.html#method.current",
                    L {
                        en: "The trail, ending at the current page `text`.",
                        ja: "今のページ `text` で終わるパンくずリスト",
                    },
                ),
                (
                    "pagination",
                    "hypersig_ui/fn.pagination.html",
                    L {
                        en: "Links to the pages of a list of `count` pages, on page `current`.",
                        ja: "`count` ページある一覧の各ページへのリンク。今のページは `current`",
                    },
                ),
                (
                    "PaginationText",
                    "hypersig_ui/struct.PaginationText.html",
                    L {
                        en: "The words `pagination` names its parts with.",
                        ja: "`pagination` が各部分の名前に使う言葉",
                    },
                ),
            ],
        },
        Component::DataTable => Page {
            lead: L {
                en: "A table of the rows of a list signal under typed columns, which sorts, filters, pages, and selects rows in the browser, or asks the server for one page at a time.",
                ja: "リストのシグナルの行を、型の付いた列の下に並べる表です。並べ替え、絞り込み、ページ送り、行の選択をブラウザで行うほか、1 ページずつサーバーに求めることもできます。",
            },
            demo: Demo::DataTable,
            built_on: &[
                L {
                    en: "It is a `<table>` whose headers have `scope=\"col\"`. A sortable column's header holds a button and carries `aria-sort`, which says whether the rows are sorted by that column in ascending order, in descending order, or not at all. The header checkbox is a button with `role=\"checkbox\"` whose `aria-checked` is `mixed` while some rows are selected, because a native checkbox's indeterminate state is a property that no attribute can set.",
                    ja: "見出しに `scope=\"col\"` を付けた `<table>` です。並べ替えられる列の見出しはボタンを持ち、その列で昇順か降順に並んでいるか、並んでいないかを `aria-sort` で伝えます。見出しのチェックボックスは `role=\"checkbox\"` のボタンで、一部の行が選ばれている間は `aria-checked` が `mixed` になります。ネイティブのチェックボックスの中間状態はプロパティで、属性からは設定できないためです。",
                },
                L {
                    en: "A column is built from a field of the row type, so a field of another struct does not compile, and the key that `sortable_column` sorts by must be a number or a string. The table keeps its filter text, page, and sort column in a `TableState` field of the page's signals, and sorting reorders the list signal itself. The selection is a list of the selected rows' keys in another signal of the page, so the row type holds no field for it and a `ServerTable` keeps it across pages.",
                    ja: "列は行の型のフィールドから作るので、別の構造体のフィールドを渡すとコンパイルエラーになります。`sortable_column` で並べ替えるキーは数値か文字列に限られます。表は絞り込みの文字列、ページ、並べ替えた列を、ページのシグナルにある `TableState` のフィールドに持ちます。並べ替えはリストのシグナルそのものの順序を変えます。選択は、選んだ行のキーのリストとしてページの別のシグナルに持つので、行の型に選択用のフィールドは不要です。`ServerTable` ではページを移っても選択が保持されます。",
                },
                L {
                    en: "The browser counts the rows on each change, and with a filter and pagination both on, the work grows with the square of the number of rows. A list of thousands of rows belongs in a `ServerTable`. Its filter field, sortable headers, and page buttons write the `TableState` and then call an action with it, and the action answers with a patch of the page's rows and the total. A sortable column takes an enum with `#[derive(Segment)]` as its key, and the server reads it back from the state with `TableState::sorting`. The demo below runs in the browser alone, since this site has no server, and `examples/ui` has a `ServerTable` beside its `DataTable`.",
                    ja: "ブラウザは変更のたびに行を数え直し、絞り込みとページ送りを両方使うと、その手間は行数の 2 乗で増えます。数千行のリストには `ServerTable` を使ってください。絞り込みの欄、並べ替えの見出し、ページのボタンが `TableState` を書き換えてから、それを引数にアクションを呼びます。アクションは、そのページの行と総数をパッチで返します。並べ替えられる列は `#[derive(Segment)]` を付けた列挙型をキーに取り、サーバーは `TableState::sorting` で状態からキーを読み戻します。このサイトにはサーバーがないので下のデモはブラウザだけで動き、`examples/ui` には `DataTable` と並べて `ServerTable` を置いています。",
                },
                L {
                    en: "To keep a `ServerTable`'s state in the URL, give the page a route whose search parameters are the application's own struct with `#[derive(Search)]` and `Option` fields, so the address without a query string still loads. The page handler builds the `TableState` from it with `TableState::with_sorting`, and the action answers with `Patches::replace_url` of the URL its state names, so a reload or a shared link shows the same page of rows. `TableState` itself is not a search struct, because a URL without one of its fields would get a 400.",
                    ja: "`ServerTable` の状態を URL に保存するには、`Option` のフィールドを持ち `#[derive(Search)]` を付けたアプリケーション自身の構造体を、ページのルートのクエリパラメータにします。こうするとクエリ文字列のないアドレスも読み込めます。ページのハンドラはそこから `TableState::with_sorting` で `TableState` を作り、アクションは状態が指す URL を `Patches::replace_url` で応答に含めます。再読み込みしても、共有したリンクを開いても、同じページの行が出ます。`TableState` そのものをクエリパラメータにしないのは、フィールドが 1 つでも欠けた URL に 400 が返るためです。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Space",
                        ja: "Space",
                    },
                    L {
                        en: "Toggles the focused checkbox. On the header checkbox, selects every row, or clears them once all are selected.",
                        ja: "フォーカスのあるチェックボックスを切り替えます。見出しのチェックボックスでは、すべての行を選び、すべて選ばれていれば選択を外します。",
                    },
                ),
                (
                    L {
                        en: "Enter, Space",
                        ja: "Enter、Space",
                    },
                    L {
                        en: "On a column header, sorts the rows by that column. On the button at the start of a row, shows or hides the row's details.",
                        ja: "列の見出しで押すと、その列で行を並べ替えます。行の先頭のボタンで押すと、その行の詳細を開閉します。",
                    },
                ),
            ],
            items: &[
                (
                    "DataTable::new",
                    "hypersig_ui/struct.DataTable.html#method.new",
                    L {
                        en: "An empty table named `label`, listing `rows` by `key`, with its state in `state`.",
                        ja: "`label` を名前に持ち、`rows` を `key` で並べ、状態を `state` に持つ空の表",
                    },
                ),
                (
                    "column",
                    "hypersig_ui/struct.DataTable.html#method.column",
                    L {
                        en: "Adds `column` after the columns added so far.",
                        ja: "それまでに追加した列の後ろに `column` を追加します。",
                    },
                ),
                (
                    "sortable_column",
                    "hypersig_ui/struct.DataTable.html#method.sortable_column",
                    L {
                        en: "Adds `column`, whose header sorts the rows by `key`.",
                        ja: "見出しで行を `key` の順に並べ替える列 `column` を追加します。",
                    },
                ),
                (
                    "selectable",
                    "hypersig_ui/struct.DataTable.html#method.selectable",
                    L {
                        en: "Adds a first column of checkboxes that keep selected rows in `selected`.",
                        ja: "選んだ行を `selected` に持つチェックボックスの列を先頭に追加します。",
                    },
                ),
                (
                    "expandable",
                    "hypersig_ui/struct.DataTable.html#method.expandable",
                    L {
                        en: "Adds a first column of buttons that show `details` of a row under it.",
                        ja: "行の下に `details` を開閉するボタンの列を先頭に追加します。",
                    },
                ),
                (
                    "filter",
                    "hypersig_ui/struct.DataTable.html#method.filter",
                    L {
                        en: "Adds a field named `label` that shows the rows whose `text` contains its text.",
                        ja: "`label` を名前に持つ欄を追加し、`text` がその欄の文字列を含む行だけを表示します。",
                    },
                ),
                (
                    "paginate",
                    "hypersig_ui/struct.DataTable.html#method.paginate",
                    L {
                        en: "Shows `size` rows at a time, with buttons for the previous and next page.",
                        ja: "一度に `size` 行を表示し、前と次のページに移るボタンを置きます。",
                    },
                ),
                (
                    "max_height",
                    "hypersig_ui/struct.DataTable.html#method.max_height",
                    L {
                        en: "Keeps the table at most `height` high.",
                        ja: "表の高さを `height` までに抑えます。",
                    },
                ),
                (
                    "Column::text",
                    "hypersig_ui/struct.Column.html#method.text",
                    L {
                        en: "A column headed `header` that shows the text field `field` of each row.",
                        ja: "見出しが `header` で、各行の文字列のフィールド `field` を表示する列",
                    },
                ),
                (
                    "Column::number",
                    "hypersig_ui/struct.Column.html#method.number",
                    L {
                        en: "A column headed `header` that shows the number field `field`, aligned to the end.",
                        ja: "見出しが `header` で、数値のフィールド `field` を行末に揃えて表示する列",
                    },
                ),
                (
                    "Column::formatted",
                    "hypersig_ui/struct.Column.html#method.formatted",
                    L {
                        en: "A column headed `header` that shows the number field `field` written in `format`.",
                        ja: "見出しが `header` で、数値のフィールド `field` を `format` の形で表示する列",
                    },
                ),
                (
                    "Column::custom",
                    "hypersig_ui/struct.Column.html#method.custom",
                    L {
                        en: "A column headed `header` whose cells hold what `render` returns for each row.",
                        ja: "見出しが `header` で、各行に対して `render` が返す要素をセルに入れる列",
                    },
                ),
                (
                    "visible",
                    "hypersig_ui/struct.Column.html#method.visible",
                    L {
                        en: "Shows the column only while `when` is true.",
                        ja: "`when` が真の間だけ列を表示します。",
                    },
                ),
                (
                    "TableState",
                    "hypersig_ui/struct.TableState.html",
                    L {
                        en: "The signals a `DataTable` or `ServerTable` keeps: filter text, page, and sort.",
                        ja: "`DataTable` と `ServerTable` が持つシグナル。絞り込みの文字列、ページ、並べ替え",
                    },
                ),
                (
                    "TableText",
                    "hypersig_ui/struct.TableText.html",
                    L {
                        en: "The words a `DataTable` or `ServerTable` shows or names its controls with.",
                        ja: "`DataTable` と `ServerTable` が表示する言葉と、操作部品に付ける名前",
                    },
                ),
                (
                    "ServerTable::new",
                    "hypersig_ui/struct.ServerTable.html#method.new",
                    L {
                        en: "An empty table named `label`, listing `rows` by `key`.",
                        ja: "`label` を名前に持ち、`rows` を `key` で並べる空の表",
                    },
                ),
                (
                    "ServerTable::sortable_column",
                    "hypersig_ui/struct.ServerTable.html#method.sortable_column",
                    L {
                        en: "Adds `column`, whose header asks the server to sort by `key`.",
                        ja: "見出しでサーバーに `key` での並べ替えを求める列 `column` を追加します。",
                    },
                ),
                (
                    "ServerTable::selectable",
                    "hypersig_ui/struct.ServerTable.html#method.selectable",
                    L {
                        en: "Adds a first column of checkboxes that keep selected rows in `selected`.",
                        ja: "選んだ行を `selected` に持つチェックボックスの列を先頭に追加します。",
                    },
                ),
                (
                    "ServerTable::filter",
                    "hypersig_ui/struct.ServerTable.html#method.filter",
                    L {
                        en: "Adds a field named `label` whose text the server filters the rows by.",
                        ja: "`label` を名前に持つ欄を追加し、サーバーがその文字列で行を絞り込みます。",
                    },
                ),
                (
                    "ServerTable::paginate",
                    "hypersig_ui/struct.ServerTable.html#method.paginate",
                    L {
                        en: "Shows pages of `size` rows out of `total`.",
                        ja: "`total` 行のうち `size` 行ずつのページを表示します。",
                    },
                ),
                (
                    "TableState::sorting",
                    "hypersig_ui/struct.TableState.html#method.sorting",
                    L {
                        en: "The key of the column a `ServerTable` is sorted by, and its order.",
                        ja: "`ServerTable` が並べ替えている列のキーと、その向き",
                    },
                ),
            ],
        },
        Component::Chart => Page {
            lead: L {
                en: "Bar, stacked bar, horizontal bar, line, area, and donut charts of the rows of a list signal, on a scale the browser fits to the values.",
                ja: "リストのシグナルの行を、棒、積み上げ棒、横棒、折れ線、面、ドーナツで描くチャートです。値の軸の目盛りはブラウザが値に合わせて決めます。",
            },
            demo: Demo::Chart,
            built_on: &[
                L {
                    en: "The drawing is an `<svg>` with `viewBox=\"0 0 100 100\"` and `preserveAspectRatio=\"none\"`, so it stretches to its box, and its lines keep their width with `vector-effect=\"non-scaling-stroke\"`. The axis labels, the legend, and the box that shows a row's values under the pointer are HTML, which does not stretch. A `DonutChart` draws each row as a circle with `pathLength=\"100\"`, whose `stroke-dasharray` is the row's share of the total in hundredths.",
                    ja: "描画は `viewBox=\"0 0 100 100\"` と `preserveAspectRatio=\"none\"` を持つ `<svg>` で、枠に合わせて伸び縮みします。線は `vector-effect=\"non-scaling-stroke\"` で太さを保ちます。軸のラベル、凡例、ポインタの下に出る行の値は、伸び縮みしない HTML で置きます。`DonutChart` は各行を `pathLength=\"100\"` の円で描き、その `stroke-dasharray` に合計に占める行の割合を百分率で書きます。",
                },
                L {
                    en: "The drawing is hidden from assistive technology. A screen reader reads a visually hidden `<table>` of the same rows, captioned with the chart's name, and the values under the pointer have no keyboard equivalent beyond that table.",
                    ja: "描画は支援技術から隠します。スクリーンリーダーは、チャートの名前をキャプションに持つ、視覚的に隠した同じ行の `<table>` を読みます。ポインタの下に出る値をキーボードで見る手段は、この表のほかにありません。",
                },
                L {
                    en: "A series reads a number field of the row type, so a field of another struct or a text field does not compile. The scale runs from a round number at or below the smallest value, or 0, to one at or above the largest, with grid lines on a step of 1, 2, 2.5, or 5 times a power of ten. The browser computes it with `max_of`, `min_of`, and `let_in`, so a statement or a patch that changes the rows refits it, and a negative value draws its bar down from the line at 0. Each bar and grid line computes the scale from every row, so the work of a change grows with the square of the number of rows.",
                    ja: "系列は行の型の数値のフィールドを読むので、別の構造体のフィールドや文字列のフィールドを渡すとコンパイルエラーになります。目盛りは、最小値か 0 の小さいほう以下の切りのよい数から、最大値以上の切りのよい数までで、刻み幅は 10 の累乗の 1、2、2.5、5 倍です。ブラウザが `max_of`、`min_of`、`let_in` で計算するので、文やパッチで行が変わると目盛りも合わせ直され、負の値の棒は 0 の線から下に伸びます。棒や目盛りの線はそれぞれ全行から目盛りを計算するので、変更 1 回の手間は行数の 2 乗で増えます。",
                },
            ],
            keys: &[],
            items: &[
                (
                    "Chart::bar",
                    "hypersig_ui/struct.Chart.html#method.bar",
                    L {
                        en: "A bar chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける棒グラフ",
                    },
                ),
                (
                    "Chart::stacked_bar",
                    "hypersig_ui/struct.Chart.html#method.stacked_bar",
                    L {
                        en: "A stacked bar chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける積み上げ棒グラフ",
                    },
                ),
                (
                    "Chart::horizontal_bar",
                    "hypersig_ui/struct.Chart.html#method.horizontal_bar",
                    L {
                        en: "A horizontal bar chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける横棒グラフ",
                    },
                ),
                (
                    "Chart::line",
                    "hypersig_ui/struct.Chart.html#method.line",
                    L {
                        en: "A line chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける折れ線グラフ",
                    },
                ),
                (
                    "Chart::area",
                    "hypersig_ui/struct.Chart.html#method.area",
                    L {
                        en: "An area chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける面グラフ",
                    },
                ),
                (
                    "series",
                    "hypersig_ui/struct.Chart.html#method.series",
                    L {
                        en: "Adds a series named `name` that draws the number field `field` in `color`.",
                        ja: "数値のフィールド `field` を `color` で描く、`name` という名前の系列を追加します。",
                    },
                ),
                (
                    "max_labels",
                    "hypersig_ui/struct.Chart.html#method.max_labels",
                    L {
                        en: "Labels at most `count` rows along the axis of the categories, 12 until set.",
                        ja: "カテゴリの軸にラベルを付ける行を最大 `count` 個にします。設定しなければ 12 個です。",
                    },
                ),
                (
                    "format",
                    "hypersig_ui/struct.Chart.html#method.format",
                    L {
                        en: "Writes the values and the axis labels in `format`.",
                        ja: "値と軸のラベルを `format` の形で書きます。",
                    },
                ),
                (
                    "DonutChart::new",
                    "hypersig_ui/struct.DonutChart.html#method.new",
                    L {
                        en: "A donut chart named `label` of `rows`, with slices of `value` labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` の `value` を `category` でラベルを付けた弧で描くドーナツグラフ",
                    },
                ),
                (
                    "token::chart_1",
                    "hypersig_ui/token/fn.chart_1.html",
                    L {
                        en: "The first series of a chart.",
                        ja: "チャートの 1 つめの系列の色",
                    },
                ),
            ],
        },
    }
}
