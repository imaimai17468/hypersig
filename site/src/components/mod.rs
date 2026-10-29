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
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of, title};

mod buttons;
mod forms;
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
        Component::Dialog
        | Component::AlertDialog
        | Component::Sheet
        | Component::Menu
        | Component::Tooltip
        | Component::Popover
        | Component::Toast
        | Component::ServerToasts => carved(site, page),
        // Display
        Component::Alert
        | Component::Badge
        | Component::Progress
        | Component::Skeleton
        | Component::Spinner
        | Component::Avatar
        | Component::Kbd => carved(site, page),
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
        // Overlays
        Component::AlertDialog | Component::Sheet => Component::Dialog,
        // Overlays
        Component::ServerToasts => Component::Toast,
        // Display
        Component::Skeleton => Component::Progress,
        other => other,
    }
}

fn component(site: Site, page: Component, content: &Page) -> Part {
    let mut children = vec![
        title(Doc::Components(page).title(site)),
        lead(site, content.lead),
        demo::show(site, content.demo),
    ];
    if matches!(page, Component::Toast) {
        children.extend(toast_from_the_server(site));
    }
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

/// The section of the toast page on toasts a handler adds, which the demo
/// cannot show because the site has no server.
fn toast_from_the_server(site: Site) -> [Part; 4] {
    [
        heading(
            site,
            L {
                en: "Add a toast from the server",
                ja: "サーバーからトーストを追加する",
            },
        ),
        para(
            site,
            L {
                en: "A handler adds a toast with `patch_append`, which puts it after the toasts the browser shows, where `patch` would replace them. A failed call shows the server's message when `on_error` passes it to `Toast::push_expr` as `Toast::body()`.",
                ja: "ハンドラは `patch_append` でトーストを追加します。`patch_append` はブラウザが表示中のトーストの後ろに追加し、`patch` はそれらを置き換えます。呼び出しが失敗したときにサーバーのメッセージを表示するには、`on_error` が渡すメッセージを `Toast::push_expr` で `Toast::body()` に入れます。",
            },
        ),
        show(Snippet::ToastServer),
        para(
            site,
            L {
                en: "A toast from `Toast::new` has an id below 0, and a toast the browser pushes has an id above 0, so a toast from the server and one from the browser never share an id.",
                ja: "`Toast::new` で作ったトーストの id は 0 未満で、ブラウザが追加したトーストの id は 0 より大きいので、サーバーのトーストとブラウザのトーストの id は重なりません。",
            },
        ),
    ]
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
        | Component::Combobox => unreachable!("written in its own file"),
        Component::Breadcrumb
        | Component::Pagination
        | Component::TableSorting
        | Component::TableFiltering
        | Component::TableSelection
        | Component::ServerTable
        | Component::BarChart
        | Component::DonutChart
        | Component::AlertDialog
        | Component::Sheet
        | Component::ServerToasts
        | Component::Skeleton => unreachable!("carved_from maps it to its page"),
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
        Component::Avatar => Page {
            lead: L {
                en: "A round picture of a person, with their initials where there is no picture.",
                ja: "人物を表す丸い画像です。画像がない場合はイニシャルを表示します。",
            },
            demo: Demo::Avatar,
            built_on: &[
                L {
                    en: "The avatar is a `role=\"img\"` named after the person through `aria-label`, so a screen reader reads the name once and nothing inside it. The picture comes from a `SafeUrl`, a `Url`, or a signal of one, as the `src` of an `<img>` does.",
                    ja: "アバターは `aria-label` で人物の名前を付けた `role=\"img\"` です。そのため、スクリーンリーダーは名前を一度だけ読み上げ、中身は読み上げません。画像の URL は `<img>` の `src` と同じく `SafeUrl` か `Url`、またはそのシグナルから渡します。",
                },
                L {
                    en: "The initials are the picture's `alt`, and where the picture fails to load, the broken `<img>`'s `::after` draws them over it, as the second avatar of the demo shows. Chrome draws that `::after`. A browser that draws none on a broken image shows its own marker for one there instead.",
                    ja: "イニシャルは画像の `alt` で、画像を読み込めなかった場合は、壊れた `<img>` の `::after` が画像の上にイニシャルを描きます。デモの 2 つ目のアバターがこの場合です。Chrome はこの `::after` を描きます。壊れた画像に `::after` を描かないブラウザでは、代わりにそのブラウザの壊れた画像の印が表示されます。",
                },
            ],
            keys: &[],
            items: &[
                (
                    "Avatar::new",
                    "hypersig_ui/struct.Avatar.html#method.new",
                    L {
                        en: "An avatar of `name` that shows `initials`.",
                        ja: "`initials` を表示する、`name` のアバター",
                    },
                ),
                (
                    "picture",
                    "hypersig_ui/struct.Avatar.html#method.picture",
                    L {
                        en: "Shows the picture at `url` in place of the initials.",
                        ja: "イニシャルの代わりに `url` の画像を表示します。",
                    },
                ),
                (
                    "render",
                    "hypersig_ui/struct.Avatar.html#method.render",
                    L {
                        en: "The avatar, as a `<span>`.",
                        ja: "`<span>` としてのアバター",
                    },
                ),
            ],
        },
        Component::Kbd => Page {
            lead: L {
                en: "A key or a shortcut the visitor presses, drawn as a small cap in running text.",
                ja: "訪問者が押すキーやショートカットを、文中に小さなキーの形で表示します。",
            },
            demo: Demo::Kbd,
            built_on: &[L {
                en: "It is a native `<kbd>`, and a screen reader reads its text as it stands, so write the key's name in words where a symbol would be read as something else, such as \"Ctrl\" rather than \"^\". For a shortcut, place one `kbd` per key side by side.",
                ja: "ネイティブの `<kbd>` で、スクリーンリーダーはテキストをそのまま読み上げます。そのため、記号が別のものとして読まれる場合は、「^」ではなく「Ctrl」のように、キーの名前を言葉で書いてください。ショートカットはキーごとに 1 つの `kbd` を並べて書きます。",
            }],
            keys: &[],
            items: &[(
                "kbd",
                "hypersig_ui/fn.kbd.html",
                L {
                    en: "A `<kbd>` that shows a key or a shortcut the visitor presses.",
                    ja: "訪問者が押すキーやショートカットを表示する `<kbd>`",
                },
            )],
        },
        Component::Spinner => Page {
            lead: L {
                en: "A turning ring that marks work whose length is not known.",
                ja: "どれだけかかるかわからない処理の途中であることを示す、回転する輪です。",
            },
            demo: Demo::Loading,
            built_on: &[
                L {
                    en: "The spinner is a `role=\"status\"` named by its label through `aria-label`, and the ring inside it is hidden from assistive technology. A screen reader reads the label where it reaches the spinner. Whether it also reads the label as the spinner appears differs between screen readers, so say in text when the work finishes, such as in a toast.",
                    ja: "スピナーは `aria-label` でラベルを名前にした `role=\"status\"` で、中の輪は支援技術から隠されています。スクリーンリーダーはスピナーまで読み進めたときにラベルを読み上げます。スピナーが現れたときにも読み上げるかどうかはスクリーンリーダーによって違うので、処理が終わったことはトーストなどのテキストで伝えてください。",
                },
                L {
                    en: "The ring turns in the current text color, and four times slower for a visitor who asks for reduced motion. Use `progress` where the share of the work done is known.",
                    ja: "輪は現在の文字色で回り、動きを減らす設定の訪問者には 4 倍遅く回ります。処理の進み具合がわかる場合は `progress` を使ってください。",
                },
            ],
            keys: &[],
            items: &[(
                "spinner",
                "hypersig_ui/fn.spinner.html",
                L {
                    en: "A turning ring named `label`, which marks work that has no known length.",
                    ja: "`label` を名前に持ち、長さのわからない処理を示す回転する輪",
                },
            )],
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
        Component::Dialog => Page {
            lead: L {
                en: "A modal dialog, a sheet along an edge of the viewport, and an alert dialog that asks the visitor to confirm.",
                ja: "モーダルダイアログ、ビューポートの端に沿って出るシート、訪問者に確認を求めるアラートダイアログです。",
            },
            demo: Demo::Dialog,
            built_on: &[
                L {
                    en: "Declare the dialog as a `const` and use it for the dialog and for the buttons that open and close it. The buttons carry `commandfor` and `command`, and the browser opens the `<dialog>` with `showModal()`. While it is open, the rest of the page is inert, Escape closes the dialog, and on closing, focus returns to the button that opened it. Invoker commands are available from Chrome 135, Firefox 144, and Safari 26.2.",
                    ja: "ダイアログは `const` として宣言し、ダイアログ本体と、それを開くボタン、閉じるボタンに使います。ボタンには `commandfor` と `command` が付き、ブラウザが `<dialog>` を `showModal()` で開きます。開いている間はページのほかの部分が inert になって操作できず、Escape でダイアログが閉じ、閉じるとフォーカスはダイアログを開いたボタンに戻ります。invoker commands は Chrome 135、Firefox 144、Safari 26.2 から使えます。",
                },
                L {
                    en: "The dialog is labeled by its title and described by its description through ids derived from the name. An alert dialog has `role=\"alertdialog\"` and no close button in its corner, so the visitor answers with one of its buttons.",
                    ja: "ダイアログには名前から作った id を通して、タイトルがラベルとして、説明が説明文として結び付きます。アラートダイアログは `role=\"alertdialog\"` を持ち、角に閉じるボタンがないので、訪問者はダイアログのボタンのどれかで答えます。",
                },
                L {
                    en: "A sheet is the same modal `<dialog>`, placed along one edge of the viewport by `Dialog::sheet`, so it keeps the focus, Escape, and backdrop behavior of a dialog.",
                    ja: "シートは同じモーダルの `<dialog>` を `Dialog::sheet` でビューポートの端に沿って置いたものなので、フォーカス、Escape、背景のクリックの振る舞いはダイアログと同じです。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Moves between the dialog's controls and never reaches the page behind.",
                        ja: "ダイアログのコントロールの間を移り、後ろのページには移りません。",
                    },
                ),
                (
                    L {
                        en: "Escape",
                        ja: "Escape",
                    },
                    L {
                        en: "Closes the dialog and returns focus to the button that opened it.",
                        ja: "ダイアログを閉じ、ダイアログを開いたボタンにフォーカスを戻します。",
                    },
                ),
            ],
            items: &[
                (
                    "Dialog::new",
                    "hypersig_ui/struct.Dialog.html#method.new",
                    L {
                        en: "A dialog that `dialog` names, whose id is the name of `dialog`.",
                        ja: "`dialog` が名前を付け、`dialog` の名前を id とするダイアログ",
                    },
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Dialog.html#method.trigger",
                    L {
                        en: "A button that opens the dialog.",
                        ja: "ダイアログを開くボタン",
                    },
                ),
                (
                    "open",
                    "hypersig_ui/struct.Dialog.html#method.open",
                    L {
                        en: "A statement that opens the dialog, as its trigger button does.",
                        ja: "トリガーのボタンと同じようにダイアログを開く文",
                    },
                ),
                (
                    "dismiss",
                    "hypersig_ui/struct.Dialog.html#method.dismiss",
                    L {
                        en: "A statement that closes the dialog, as its close button does.",
                        ja: "閉じるボタンと同じようにダイアログを閉じる文",
                    },
                ),
                (
                    "close",
                    "hypersig_ui/struct.Dialog.html#method.close",
                    L {
                        en: "A button that closes the dialog. Put it inside the dialog.",
                        ja: "ダイアログの中に置く、ダイアログを閉じるボタン",
                    },
                ),
                (
                    "dialog",
                    "hypersig_ui/struct.Dialog.html#method.dialog",
                    L {
                        en: "The dialog, with `title`, `description`, `content`, and a close button in its corner.",
                        ja: "`title`、`description`、`content` と、角の閉じるボタンを持つダイアログ",
                    },
                ),
                (
                    "alert_dialog",
                    "hypersig_ui/struct.Dialog.html#method.alert_dialog",
                    L {
                        en: "An alert dialog, which asks the visitor to confirm or cancel before going on.",
                        ja: "先に進む前に、訪問者に確認か取り消しを求めるアラートダイアログ",
                    },
                ),
                (
                    "sheet",
                    "hypersig_ui/struct.Dialog.html#method.sheet",
                    L {
                        en: "A sheet: a dialog along the `side` edge of the viewport.",
                        ja: "シート。ビューポートの `side` の端に沿って出るダイアログ",
                    },
                ),
                (
                    "DialogText",
                    "hypersig_ui/struct.DialogText.html",
                    L {
                        en: "The words `Dialog::dialog` and `Dialog::sheet` name their close button with.",
                        ja: "`Dialog::dialog` と `Dialog::sheet` が閉じるボタンの名前に使う言葉",
                    },
                ),
                (
                    "Side",
                    "hypersig_ui/enum.Side.html",
                    L {
                        en: "The edge of the viewport a `Dialog::sheet` sits along.",
                        ja: "`Dialog::sheet` が沿うビューポートの端",
                    },
                ),
                (
                    "dialog_footer",
                    "hypersig_ui/fn.dialog_footer.html",
                    L {
                        en: "The row of buttons at the bottom of a dialog, aligned to the end.",
                        ja: "ダイアログの下部に、末尾に寄せて並べるボタンの列",
                    },
                ),
            ],
        },
        Component::Menu => Page {
            lead: L {
                en: "A menu of actions opened by a button.",
                ja: "ボタンで開く、操作のメニューです。",
            },
            demo: Demo::Menu,
            built_on: &[
                L {
                    en: "It follows the WAI-ARIA menu button pattern. The button has `aria-haspopup=\"menu\"`, the menu `role=\"menu\"`, and each action `role=\"menuitem\"`. The menu is a `popover`, so the browser shows it over the page and closes it on Escape or on a click outside. It is placed under the button with CSS anchor positioning and moves above it where there is no room below.",
                    ja: "WAI-ARIA menu button パターンに従います。ボタンは `aria-haspopup=\"menu\"` を、メニューは `role=\"menu\"` を、各操作は `role=\"menuitem\"` を持ちます。メニューは `popover` なので、ブラウザがページの上に重ねて表示し、Escape か外側のクリックで閉じます。メニューは CSS の anchor positioning でボタンの下に置かれ、下に場所がなければボタンの上に移ります。",
                },
                L {
                    en: "A disabled action keeps its place, and the arrow keys skip it.",
                    ja: "無効な操作はその位置に残り、矢印キーでは飛ばされます。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Enter, Space",
                        ja: "Enter、Space",
                    },
                    L {
                        en: "With focus on the button, opens the menu and moves focus to the first action. With focus on an action, runs it and closes the menu.",
                        ja: "ボタンにフォーカスがあるときに押すと、メニューを開いて最初の操作にフォーカスを移します。操作にフォーカスがあるときに押すと、その操作を実行してメニューを閉じます。",
                    },
                ),
                (
                    L {
                        en: "Up, Down",
                        ja: "上矢印、下矢印",
                    },
                    L {
                        en: "Move to the previous or next action.",
                        ja: "前か次の操作に移ります。",
                    },
                ),
                (
                    L {
                        en: "Home, End",
                        ja: "Home、End",
                    },
                    L {
                        en: "Move to the first or last action.",
                        ja: "最初か最後の操作に移ります。",
                    },
                ),
                (
                    L {
                        en: "Escape",
                        ja: "Escape",
                    },
                    L {
                        en: "Closes the menu and returns focus to the button.",
                        ja: "メニューを閉じ、ボタンにフォーカスを戻します。",
                    },
                ),
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Closes the menu.",
                        ja: "メニューを閉じます。",
                    },
                ),
            ],
            items: &[
                (
                    "Menu::new",
                    "hypersig_ui/struct.Menu.html#method.new",
                    L {
                        en: "A menu that `menu` names, whose id is the name of `menu`.",
                        ja: "`menu` が名前を付け、`menu` の名前を id とするメニュー",
                    },
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Menu.html#method.trigger",
                    L {
                        en: "The button that opens and closes the menu.",
                        ja: "メニューを開閉するボタン",
                    },
                ),
                (
                    "menu",
                    "hypersig_ui/struct.Menu.html#method.menu",
                    L {
                        en: "The menu, holding `items` in order.",
                        ja: "`items` を順に並べたメニュー",
                    },
                ),
                (
                    "MenuItem::new",
                    "hypersig_ui/struct.MenuItem.html#method.new",
                    L {
                        en: "An action labeled `label` that runs `run` when chosen.",
                        ja: "`label` をラベルに持ち、選ばれると `run` を実行する操作",
                    },
                ),
                (
                    "destructive",
                    "hypersig_ui/struct.MenuItem.html#method.destructive",
                    L {
                        en: "Draws the action in `token::destructive`. It is for an action that deletes.",
                        ja: "操作を `token::destructive` の色で描きます。削除する操作に使います。",
                    },
                ),
                (
                    "disabled",
                    "hypersig_ui/struct.MenuItem.html#method.disabled",
                    L {
                        en: "Disables the action while `when` is true, which the arrow keys then skip.",
                        ja: "`when` が true の間、操作を無効にします。無効な操作は矢印キーで飛ばされます。",
                    },
                ),
            ],
        },
        Component::Tooltip => Page {
            lead: L {
                en: "A short description shown while a button is hovered or focused.",
                ja: "ボタンにポインタが重なっている間か、フォーカスがある間に表示する短い説明です。",
            },
            demo: Demo::Tooltip,
            built_on: &[
                L {
                    en: "It follows the WAI-ARIA tooltip pattern. The tooltip has `role=\"tooltip\"` and the button `aria-describedby` pointing at it, so a screen reader reads the text after the button's name. `Tooltip` builds the button and gives it the attributes the tooltip needs.",
                    ja: "WAI-ARIA tooltip パターンに従います。ツールチップは `role=\"tooltip\"` を持ち、ボタンの `aria-describedby` がツールチップを指すので、スクリーンリーダーはボタンの名前のあとにツールチップのテキストを読み上げます。ボタンは `Tooltip` が作り、ツールチップに必要な属性をそのボタンに付けます。",
                },
                L {
                    en: "The tooltip is a `popover=\"hint\"`, which browser-compat-data 8.1.6 lists for Chrome 151 and Firefox 153, with partial older implementations in earlier versions, and for Safari only in its preview. A browser without it treats the value as `manual`, and there Escape closes the tooltip only while focus is on its button.",
                    ja: "ツールチップは `popover=\"hint\"` です。browser-compat-data 8.1.6 では、Chrome 151 と Firefox 153 が対応しており、それより前のバージョンには部分的な古い実装があり、Safari はプレビュー版だけが対応しています。対応していないブラウザはこの値を `manual` として扱うので、Escape でツールチップが閉じるのは、フォーカスがそのボタンにある間だけです。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Focusing the button opens the tooltip, and leaving it closes the tooltip.",
                        ja: "ボタンにフォーカスが移るとツールチップが開き、フォーカスが離れると閉じます。",
                    },
                ),
                (
                    L {
                        en: "Any key",
                        ja: "任意のキー",
                    },
                    L {
                        en: "With focus on the button, closes the tooltip. Escape also reaches a dialog around the tooltip.",
                        ja: "ボタンにフォーカスがあるときに押すと、ツールチップが閉じます。Escape の場合は、ツールチップを囲むダイアログにもキーが届きます。",
                    },
                ),
            ],
            items: &[
                (
                    "Tooltip::new",
                    "hypersig_ui/struct.Tooltip.html#method.new",
                    L {
                        en: "A tooltip that `tip` names, whose id is the name of `tip`.",
                        ja: "`tip` が名前を付け、`tip` の名前を id とするツールチップ",
                    },
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Tooltip.html#method.trigger",
                    L {
                        en: "A button described by the tooltip.",
                        ja: "ツールチップが説明するボタン",
                    },
                ),
                (
                    "icon_trigger",
                    "hypersig_ui/struct.Tooltip.html#method.icon_trigger",
                    L {
                        en: "An icon button named `label` and described by the tooltip.",
                        ja: "`label` を名前に持ち、ツールチップが説明するアイコンボタン",
                    },
                ),
                (
                    "tooltip",
                    "hypersig_ui/struct.Tooltip.html#method.tooltip",
                    L {
                        en: "The tooltip, holding `text`.",
                        ja: "`text` を入れたツールチップ",
                    },
                ),
            ],
        },
        Component::Popover => Page {
            lead: L {
                en: "A panel of content that a button opens beside itself.",
                ja: "ボタンの横に開く、内容を入れるパネルです。",
            },
            demo: Demo::Popover,
            built_on: &[
                L {
                    en: "The panel is a `popover=\"auto\"` with `role=\"dialog\"`, named by its button through `aria-labelledby`. The button has `aria-haspopup=\"dialog\"` and opens and closes it with `commandfor` and `command`, so the browser puts the panel above the page, closes it on Escape or on a click outside, and returns focus to the button when focus was inside the panel.",
                    ja: "パネルは `role=\"dialog\"` を持つ `popover=\"auto\"` で、`aria-labelledby` によってボタンのテキストがパネルの名前になります。ボタンは `aria-haspopup=\"dialog\"` を持ち、`commandfor` と `command` でパネルを開閉します。そのため、ブラウザがパネルをページの上に表示し、Escape かパネルの外のクリックで閉じ、フォーカスがパネルの中にあった場合はボタンに戻します。",
                },
                L {
                    en: "Focus stays on the button as the panel opens, so place the panel right after its button, where Tab moves from the button into it. A statement closes it with `NodeRef::hide_popover` on the node ref the popover is built from.",
                    ja: "パネルが開いてもフォーカスはボタンに残ります。パネルはボタンの直後に置いてください。そうすると、ボタンから Tab でパネルの中に移れます。ステートメントから閉じるには、ポップオーバーを作った node ref の `NodeRef::hide_popover` を使います。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Enter, Space",
                        ja: "Enter、Space",
                    },
                    L {
                        en: "On the button, opens or closes the panel.",
                        ja: "ボタンの上で押すと、パネルを開くか閉じます。",
                    },
                ),
                (
                    L {
                        en: "Escape",
                        ja: "Escape",
                    },
                    L {
                        en: "Closes the panel.",
                        ja: "パネルを閉じます。",
                    },
                ),
            ],
            items: &[
                (
                    "Popover::new",
                    "hypersig_ui/struct.Popover.html#method.new",
                    L {
                        en: "A popover that `panel` names, whose id is the name of `panel`.",
                        ja: "`panel` が名前を付け、`panel` の名前を id とするポップオーバー",
                    },
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Popover.html#method.trigger",
                    L {
                        en: "The button that opens and closes the panel.",
                        ja: "パネルを開閉するボタン",
                    },
                ),
                (
                    "popover",
                    "hypersig_ui/struct.Popover.html#method.popover",
                    L {
                        en: "The panel, empty, for the content added with `El::child`.",
                        ja: "`El::child` で内容を加える、空のパネル",
                    },
                ),
            ],
        },
        Component::Toast => Page {
            lead: L {
                en: "Short notices shown together in a corner of the page, added from the browser or by the server.",
                ja: "ページの隅に並べて表示する短いお知らせです。ブラウザからもサーバーからも追加できます。",
            },
            demo: Demo::Toast,
            built_on: &[
                L {
                    en: "The toaster is a landmark region named \"Notifications\" holding an `aria-live=\"polite\"` list, so a screen reader reads a toast when it is added, after what it is reading. Each toast has a button that removes it.",
                    ja: "トーストを並べる領域（toaster）は「Notifications」という名前のランドマークで、`aria-live=\"polite\"` の一覧を持ちます。そのため、トーストが追加されると、スクリーンリーダーは読み上げ中の内容のあとにそのトーストを読み上げます。各トーストには、そのトーストを消すボタンがあります。",
                },
                L {
                    en: "A toast stays until it is dismissed, unless `Toaster::dismiss_after` sets a time, because a visitor who reads slowly or uses a magnifier can miss a notice that is removed automatically (WCAG 2.2.1). The region is in the page rather than in the top layer, so an open modal dialog makes it inert until the dialog closes.",
                    ja: "トーストは `Toaster::dismiss_after` で時間を設定しない限り、消されるまで残ります。ゆっくり読む訪問者や拡大鏡を使う訪問者は、自動で消えるお知らせを見逃すことがあるからです（WCAG 2.2.1）。この領域は top layer ではなくページの中にあるので、モーダルダイアログが開いている間は inert になって操作できず、ダイアログが閉じると操作できるようになります。",
                },
            ],
            keys: &[(
                L {
                    en: "Tab, Enter",
                    ja: "Tab、Enter",
                },
                L {
                    en: "Reach a toast's Dismiss button and press it.",
                    ja: "トーストの Dismiss ボタンに移り、それを押します。",
                },
            )],
            items: &[
                (
                    "Toaster::new",
                    "hypersig_ui/struct.Toaster.html#method.new",
                    L {
                        en: "A region that shows `toasts`, newest last.",
                        ja: "`toasts` を新しいものほど後ろに並べて表示する領域",
                    },
                ),
                (
                    "label",
                    "hypersig_ui/struct.Toaster.html#method.label",
                    L {
                        en: "Names the region for assistive technology, in place of \"Notifications\".",
                        ja: "「Notifications」の代わりに、支援技術に向けた領域の名前を付けます。",
                    },
                ),
                (
                    "dismiss_label",
                    "hypersig_ui/struct.Toaster.html#method.dismiss_label",
                    L {
                        en: "Names the button that removes a toast, in place of \"Dismiss\".",
                        ja: "「Dismiss」の代わりに、トーストを取り除くボタンの名前を付けます。",
                    },
                ),
                (
                    "dismiss_after",
                    "hypersig_ui/struct.Toaster.html#method.dismiss_after",
                    L {
                        en: "Removes each toast `delay` after it appears.",
                        ja: "各トーストを表示してから `delay` が経ったら取り除きます。",
                    },
                ),
                (
                    "Toast::new",
                    "hypersig_ui/struct.Toast.html#method.new",
                    L {
                        en: "A toast titled `title`, with an id no other toast of the process has.",
                        ja: "`title` をタイトルに持ち、プロセスのほかのどのトーストとも違う id を持つトースト",
                    },
                ),
                (
                    "description",
                    "hypersig_ui/struct.Toast.html#method.description",
                    L {
                        en: "Adds a line of text under the title.",
                        ja: "タイトルの下に 1 行のテキストを追加します。",
                    },
                ),
                (
                    "variant",
                    "hypersig_ui/struct.Toast.html#method.variant",
                    L {
                        en: "Sets the look of the toast.",
                        ja: "トーストの見た目を設定します。",
                    },
                ),
                (
                    "push",
                    "hypersig_ui/struct.Toast.html#method.push",
                    L {
                        en: "A statement that adds this toast to `toasts` in the browser.",
                        ja: "ブラウザでこのトーストを `toasts` に追加する文",
                    },
                ),
                (
                    "Toast::push_expr",
                    "hypersig_ui/struct.Toast.html#method.push_expr",
                    L {
                        en: "A statement that adds `toast`, which the browser evaluates, to `toasts`.",
                        ja: "ブラウザが評価する `toast` を `toasts` に追加する文",
                    },
                ),
                (
                    "Toast::body",
                    "hypersig_ui/struct.Toast.html#method.body",
                    L {
                        en: "Typed reference to the `body` signal.",
                        ja: "`body` シグナルへの型付きの参照",
                    },
                ),
            ],
        },
        Component::Alert => Page {
            lead: L {
                en: "A bordered box for a message, with a title and a description.",
                ja: "タイトルと説明を持つ、メッセージ用の枠線付きのボックスです。",
            },
            demo: Demo::Alert,
            built_on: &[L {
                en: "Both variants are live regions. A destructive alert has `role=\"alert\"`, which a screen reader reads when it is put on the page, as the demo does with `when`. A default alert has `role=\"status\"`, which is read when its text changes. Some screen readers skip a status that arrives with its text already in it, so keep a status box on the page and change its text with `bind_text`.",
                ja: "どちらのバリアントもライブリージョンです。destructive のアラートは `role=\"alert\"` を持ち、ページに置かれたときにスクリーンリーダーが読み上げます。デモでは `when` でアラートをページに置いています。default のアラートは `role=\"status\"` を持ち、テキストが変わったときに読み上げられます。最初からテキストが入った状態で現れた status を読み飛ばすスクリーンリーダーもあるので、status のボックスはページに置いたままにして、テキストを `bind_text` で変えてください。",
            }],
            keys: &[],
            items: &[
                (
                    "alert",
                    "hypersig_ui/fn.alert.html",
                    L {
                        en: "A bordered box for a message, holding an `alert_title` and an `alert_description`.",
                        ja: "`alert_title` と `alert_description` を入れる、メッセージ用の枠線付きのボックス",
                    },
                ),
                (
                    "alert_title",
                    "hypersig_ui/fn.alert_title.html",
                    L {
                        en: "The first line of an `alert`.",
                        ja: "`alert` の最初の行",
                    },
                ),
                (
                    "alert_description",
                    "hypersig_ui/fn.alert_description.html",
                    L {
                        en: "The text under the title of an `alert`.",
                        ja: "`alert` のタイトルの下のテキスト",
                    },
                ),
                (
                    "AlertVariant",
                    "hypersig_ui/enum.AlertVariant.html",
                    L {
                        en: "The look and the urgency of an alert.",
                        ja: "アラートの見た目と緊急度",
                    },
                ),
            ],
        },
        Component::Badge => Page {
            lead: L {
                en: "A badge that shows a short label, such as a status.",
                ja: "状態などの短いラベルを表示するバッジです。",
            },
            demo: Demo::Badge,
            built_on: &[L {
                en: "A badge is a `<span>` and takes no role, so a screen reader reads it where it stands. Where the color alone carries the meaning, put the meaning in the text too, such as \"Failed\" rather than a red dot.",
                ja: "バッジは `<span>` でロールを持たないので、スクリーンリーダーは置かれた位置でそのまま読み上げます。色だけで意味を伝えている場合は、赤い点だけで済ませずに、「失敗」のように意味をテキストにも書いてください。",
            }],
            keys: &[],
            items: &[
                (
                    "badge",
                    "hypersig_ui/fn.badge.html",
                    L {
                        en: "A `<span>` that shows a short label, such as a status, as a badge.",
                        ja: "状態のような短いラベルをバッジとして表示する `<span>`",
                    },
                ),
                (
                    "BadgeVariant",
                    "hypersig_ui/enum.BadgeVariant.html",
                    L {
                        en: "The look of a badge.",
                        ja: "バッジの見た目",
                    },
                ),
            ],
        },
        Component::Progress => Page {
            lead: L {
                en: "A bar filled up to a percentage, and a block that stands in for content still loading.",
                ja: "割合の分だけ塗りつぶされるバーと、読み込み中の内容の代わりに置くブロックです。",
            },
            demo: Demo::Progress,
            built_on: &[
                L {
                    en: "The bar is a `role=\"progressbar\"` with `aria-valuenow` kept equal to the value, so a screen reader reads the label and the percentage. A value below 0 or above 100 is drawn and announced as 0 or 100.",
                    ja: "バーは `role=\"progressbar\"` を持ち、`aria-valuenow` が常に値と同じになるので、スクリーンリーダーはラベルと割合を読み上げます。0 未満や 100 を超える値は、0 や 100 として描かれ、読み上げられます。",
                },
                L {
                    en: "A skeleton is hidden from assistive technology. Tell a screen reader that the content is loading elsewhere, as the demo does with `aria-busy` on the region that holds it.",
                    ja: "スケルトンは支援技術から隠してあります。内容を読み込み中であることは、別の場所でスクリーンリーダーに伝えてください。デモではスケルトンを置いた領域に `aria-busy` を付けています。",
                },
            ],
            keys: &[],
            items: &[
                (
                    "progress",
                    "hypersig_ui/fn.progress.html",
                    L {
                        en: "A bar filled up to `percent`, named by `label`.",
                        ja: "`label` を名前に持ち、`percent` の分だけ塗りつぶされるバー",
                    },
                ),
                (
                    "skeleton",
                    "hypersig_ui/fn.skeleton.html",
                    L {
                        en: "A pulsing block in `token::accent` that stands in for loading content.",
                        ja: "読み込み中の内容の代わりに置く、`token::accent` の色で明滅するブロック",
                    },
                ),
            ],
        },
    }
}
