//! The pages of the site, their paths, and the order the sidebar lists them in.

use crate::lang::L;
use crate::{Site, components, home, i18n, landing, layout, learn, reference, style, use_cases};

/// One page of the site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Doc {
    /// The landing page of hypersig, the framework.
    Home,
    /// The landing page of the Style docs.
    StyleHome,
    /// The landing page of the Components docs.
    ComponentsHome,
    /// The applications that hypersig suits, with the change each request makes.
    UseCases,
    /// A page of the Learn section.
    Learn(Learn),
    /// A page of the Reference section.
    Reference(Reference),
    /// A page of the Components section, about `hypersig_ui`.
    Components(Component),
    /// A page of the Style section, about `hypersig::style`.
    Style(StylePage),
    /// The landing page of the i18n docs.
    I18nHome,
    /// A page of the i18n section, about `hypersig_i18n`.
    I18n(I18nPage),
}

/// The pages of the Learn section, a guide read from top to bottom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Learn {
    // Get started
    QuickStart,
    Why,
    DevServer,
    // Building the page
    Elements,
    ThePage,
    Bindings,
    Components,
    Styling,
    DescribingThePage,
    // Browser state
    Signals,
    NestedSignals,
    Expressions,
    Statements,
    Events,
    BindingInputs,
    ComputedSignals,
    PersistedSignals,
    Lists,
    ListItems,
    // Talking to the server
    Actions,
    CallStatus,
    Optimistic,
    OverlappingCalls,
    Patches,
    ListPatches,
    Fragments,
    Streams,
    Server,
    CallsInDepth,
    // Pages and routing
    Routes,
    Links,
    ChildPages,
    SearchParams,
    ErrorPages,
    BasePath,
    Pages,
    // Forms and requests
    Forms,
    FormValidation,
    Requests,
    Errors,
    Uploads,
    // Data and sign-in
    Database,
    DatabaseWithoutSql,
    CheckedSql,
    HostedDatabases,
    DatabaseHosts,
    Api,
    Sessions,
    MembersOnly,
    SessionCookie,
    SessionStores,
    // Browser features
    NodeRefs,
    Timers,
    Keyboard,
    JavaScript,
    UncheckedValues,
    StaticPages,
    EscapeHatches,
    // Quality and operations
    Security,
    Testing,
    TestingPages,
    CompileErrors,
    Debugging,
    Upgrading,
    Troubleshooting,
    // Deploy
    Deploying,
    DeployContainer,
    DeployCloudflare,
    DeployVercel,
}

/// The pages of the Reference section, one per group of items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reference {
    Overview,
    Page,
    El,
    Html,
    Signals,
    Expr,
    Stmt,
    Action,
    Form,
    Patches,
    Route,
    NodeRef,
    RouterExt,
}

/// The pages of the Components section: setup, then one page per component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    Installation,
    Theming,
    Button,
    TextField,
    Checkbox,
    RadioGroup,
    Select,
    Slider,
    Toggle,
    Listbox,
    Combobox,
    Tabs,
    Accordion,
    Collapsible,
    DataTable,
    Chart,
    Card,
    Separator,
    Navigation,
    Dialog,
    Menu,
    Tooltip,
    Popover,
    Toast,
    Alert,
    Badge,
    Progress,
    Spinner,
    Avatar,
    Kbd,
}

/// The pages of the Style section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StylePage {
    Overview,
    Values,
    Conditions,
    Tokens,
    Global,
    Patterns,
    Animation,
    Unchecked,
}

/// The pages of the i18n section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I18nPage {
    Overview,
    Pages,
    Negotiation,
}

/// One of the sets of docs, each with its own header and landing
/// page, as the libraries of `TanStack` share one site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Library {
    /// hypersig itself.
    Framework,
    /// `hypersig::style`, from the `hypersig_style` crate.
    Style,
    /// The `hypersig_ui` crate.
    Components,
    /// The `hypersig_i18n` crate.
    I18n,
}

impl Library {
    /// Every library, in the order the switcher lists them.
    pub const ALL: [Self; 4] = [Self::Framework, Self::Style, Self::Components, Self::I18n];

    /// The name the header and the switcher show.
    pub fn name(self, site: Site) -> &'static str {
        site.t(match self {
            Self::Framework => L {
                en: "Framework",
                ja: "フレームワーク",
            },
            Self::Style => L {
                en: "Style",
                ja: "スタイル",
            },
            Self::Components => L {
                en: "Components",
                ja: "コンポーネント",
            },
            Self::I18n => L {
                en: "i18n",
                ja: "多言語化",
            },
        })
    }

    /// The path a program imports the library from.
    pub fn path(self) -> &'static str {
        match self {
            Self::Framework => "hypersig",
            Self::Style => "hypersig::style",
            Self::Components => "hypersig_ui",
            Self::I18n => "hypersig_i18n",
        }
    }

    /// What the library is, as the description of its landing page that
    /// search results and link previews show.
    fn descriptions(self) -> L<&'static str> {
        match self {
            Self::Framework => L {
                en: "hypersig is a server-driven hypermedia web framework for Rust. Signal names, expressions, actions, and links are built from Rust types, so a mistake in one is a compile error.",
                ja: "hypersig は Rust で書くサーバー駆動のハイパーメディアの Web フレームワークです。シグナル名、式、アクション、リンクを Rust の型から作るので、その誤りはコンパイルエラーになります。",
            },
            Self::Style => L {
                en: "Typed atomic CSS for hypersig, with properties and values generated from MDN's data.",
                ja: "hypersig の型付きアトミック CSS です。プロパティと値は MDN のデータから生成しています。",
            },
            Self::Components => L {
                en: "Components for hypersig styled after shadcn/ui, built on native elements and the WAI-ARIA patterns.",
                ja: "shadcn/ui の見た目に倣った hypersig のコンポーネントです。ネイティブの要素と WAI-ARIA のパターンをもとに作っています。",
            },
            Self::I18n => L {
                en: "Typed translations for hypersig, with the language picked from Accept-Language.",
                ja: "hypersig の型付きの翻訳です。言語は Accept-Language から選びます。",
            },
        }
    }

    /// The library's landing page.
    pub fn home(self) -> Doc {
        match self {
            Self::Framework => Doc::Home,
            Self::Style => Doc::StyleHome,
            Self::Components => Doc::ComponentsHome,
            Self::I18n => Doc::I18nHome,
        }
    }

    /// The sections the header links to, each with its first page.
    pub fn sections(self) -> &'static [(L<&'static str>, Doc)] {
        const LEARN: L<&str> = L {
            en: "Learn",
            ja: "学ぶ",
        };
        const USE_CASES: L<&str> = L {
            en: "Use cases",
            ja: "ユースケース",
        };
        const REFERENCE: L<&str> = L {
            en: "Reference",
            ja: "リファレンス",
        };
        const DOCS: L<&str> = L {
            en: "Docs",
            ja: "ドキュメント",
        };
        match self {
            Self::Framework => &[
                (LEARN, Doc::Learn(Learn::QuickStart)),
                (USE_CASES, Doc::UseCases),
                (REFERENCE, Doc::Reference(Reference::Overview)),
            ],
            Self::Style => &[(DOCS, Doc::Style(StylePage::Overview))],
            Self::Components => &[(DOCS, Doc::Components(Component::Installation))],
            Self::I18n => &[(DOCS, Doc::I18n(I18nPage::Overview))],
        }
    }

    /// The front page of the library's rustdoc output, as a link target.
    pub fn api(self) -> &'static str {
        match self {
            Self::Framework => "api:hypersig/index.html",
            Self::Style => "api:hypersig_style/index.html",
            Self::Components => "api:hypersig_ui/index.html",
            Self::I18n => "api:hypersig_i18n/index.html",
        }
    }
}

/// A titled run of pages in a sidebar.
#[derive(Debug)]
pub struct Group {
    pub title: L<&'static str>,
    pub docs: &'static [Doc],
}

/// The sidebar of the Learn section.
pub const LEARN: &[Group] = &[
    Group {
        title: L {
            en: "Get started",
            ja: "はじめに",
        },
        docs: &[
            Doc::Learn(Learn::QuickStart),
            Doc::Learn(Learn::Why),
            Doc::Learn(Learn::DevServer),
        ],
    },
    Group {
        title: L {
            en: "Building the page",
            ja: "ページを組み立てる",
        },
        docs: &[
            Doc::Learn(Learn::Elements),
            Doc::Learn(Learn::ThePage),
            Doc::Learn(Learn::Bindings),
            Doc::Learn(Learn::Components),
            Doc::Learn(Learn::Styling),
            Doc::Learn(Learn::DescribingThePage),
        ],
    },
    Group {
        title: L {
            en: "Browser state",
            ja: "ブラウザ側の状態",
        },
        docs: &[
            Doc::Learn(Learn::Signals),
            Doc::Learn(Learn::NestedSignals),
            Doc::Learn(Learn::Expressions),
            Doc::Learn(Learn::Statements),
            Doc::Learn(Learn::Events),
            Doc::Learn(Learn::BindingInputs),
            Doc::Learn(Learn::ComputedSignals),
            Doc::Learn(Learn::PersistedSignals),
            Doc::Learn(Learn::Lists),
            Doc::Learn(Learn::ListItems),
        ],
    },
    Group {
        title: L {
            en: "Talking to the server",
            ja: "サーバーとのやり取り",
        },
        docs: &[
            Doc::Learn(Learn::Actions),
            Doc::Learn(Learn::CallStatus),
            Doc::Learn(Learn::Optimistic),
            Doc::Learn(Learn::OverlappingCalls),
            Doc::Learn(Learn::Patches),
            Doc::Learn(Learn::ListPatches),
            Doc::Learn(Learn::Fragments),
            Doc::Learn(Learn::Streams),
            Doc::Learn(Learn::Server),
            Doc::Learn(Learn::CallsInDepth),
        ],
    },
    Group {
        title: L {
            en: "Pages and routing",
            ja: "ページとルーティング",
        },
        docs: &[
            Doc::Learn(Learn::Routes),
            Doc::Learn(Learn::Links),
            Doc::Learn(Learn::ChildPages),
            Doc::Learn(Learn::SearchParams),
            Doc::Learn(Learn::ErrorPages),
            Doc::Learn(Learn::BasePath),
            Doc::Learn(Learn::Pages),
        ],
    },
    Group {
        title: L {
            en: "Forms and requests",
            ja: "フォームとリクエスト",
        },
        docs: &[
            Doc::Learn(Learn::Forms),
            Doc::Learn(Learn::FormValidation),
            Doc::Learn(Learn::Requests),
            Doc::Learn(Learn::Errors),
            Doc::Learn(Learn::Uploads),
        ],
    },
    Group {
        title: L {
            en: "Data and sign-in",
            ja: "データとサインイン",
        },
        docs: &[
            Doc::Learn(Learn::Database),
            Doc::Learn(Learn::DatabaseWithoutSql),
            Doc::Learn(Learn::CheckedSql),
            Doc::Learn(Learn::HostedDatabases),
            Doc::Learn(Learn::DatabaseHosts),
            Doc::Learn(Learn::Api),
            Doc::Learn(Learn::Sessions),
            Doc::Learn(Learn::MembersOnly),
            Doc::Learn(Learn::SessionCookie),
            Doc::Learn(Learn::SessionStores),
        ],
    },
    Group {
        title: L {
            en: "Browser features",
            ja: "ブラウザの機能を使う",
        },
        docs: &[
            Doc::Learn(Learn::NodeRefs),
            Doc::Learn(Learn::Timers),
            Doc::Learn(Learn::Keyboard),
            Doc::Learn(Learn::JavaScript),
            Doc::Learn(Learn::UncheckedValues),
            Doc::Learn(Learn::StaticPages),
            Doc::Learn(Learn::EscapeHatches),
        ],
    },
    Group {
        title: L {
            en: "Quality and operations",
            ja: "品質と運用",
        },
        docs: &[
            Doc::Learn(Learn::Security),
            Doc::Learn(Learn::Testing),
            Doc::Learn(Learn::TestingPages),
            Doc::Learn(Learn::CompileErrors),
            Doc::Learn(Learn::Debugging),
            Doc::Learn(Learn::Upgrading),
            Doc::Learn(Learn::Troubleshooting),
        ],
    },
    Group {
        title: L {
            en: "Deploy",
            ja: "デプロイ",
        },
        docs: &[
            Doc::Learn(Learn::Deploying),
            Doc::Learn(Learn::DeployContainer),
            Doc::Learn(Learn::DeployCloudflare),
            Doc::Learn(Learn::DeployVercel),
        ],
    },
];

/// The sidebar of the Reference section.
pub const REFERENCE: &[Group] = &[
    Group {
        title: L {
            en: "Reference",
            ja: "リファレンス",
        },
        docs: &[Doc::Reference(Reference::Overview)],
    },
    Group {
        title: L {
            en: "The page",
            ja: "ページ",
        },
        docs: &[
            Doc::Reference(Reference::Page),
            Doc::Reference(Reference::El),
            Doc::Reference(Reference::Html),
        ],
    },
    Group {
        title: L {
            en: "State",
            ja: "状態",
        },
        docs: &[
            Doc::Reference(Reference::Signals),
            Doc::Reference(Reference::Expr),
            Doc::Reference(Reference::Stmt),
            Doc::Reference(Reference::NodeRef),
        ],
    },
    Group {
        title: L {
            en: "The server",
            ja: "サーバー",
        },
        docs: &[
            Doc::Reference(Reference::Action),
            Doc::Reference(Reference::Form),
            Doc::Reference(Reference::Patches),
            Doc::Reference(Reference::Route),
            Doc::Reference(Reference::RouterExt),
        ],
    },
];

/// The sidebar of the Components section.
pub const COMPONENTS: &[Group] = &[
    Group {
        title: L {
            en: "Get started",
            ja: "はじめに",
        },
        docs: &[
            Doc::Components(Component::Installation),
            Doc::Components(Component::Theming),
        ],
    },
    Group {
        title: L {
            en: "Forms",
            ja: "フォーム",
        },
        docs: &[
            Doc::Components(Component::Button),
            Doc::Components(Component::TextField),
            Doc::Components(Component::Checkbox),
            Doc::Components(Component::RadioGroup),
            Doc::Components(Component::Select),
            Doc::Components(Component::Slider),
            Doc::Components(Component::Toggle),
        ],
    },
    Group {
        title: L {
            en: "Pickers",
            ja: "選択",
        },
        docs: &[
            Doc::Components(Component::Listbox),
            Doc::Components(Component::Combobox),
        ],
    },
    Group {
        title: L {
            en: "Layout",
            ja: "レイアウト",
        },
        docs: &[
            Doc::Components(Component::Tabs),
            Doc::Components(Component::Accordion),
            Doc::Components(Component::Collapsible),
            Doc::Components(Component::Card),
            Doc::Components(Component::Separator),
        ],
    },
    Group {
        title: L {
            en: "Navigation",
            ja: "ナビゲーション",
        },
        docs: &[Doc::Components(Component::Navigation)],
    },
    Group {
        title: L {
            en: "Data",
            ja: "データ",
        },
        docs: &[
            Doc::Components(Component::DataTable),
            Doc::Components(Component::Chart),
        ],
    },
    Group {
        title: L {
            en: "Overlays",
            ja: "オーバーレイ",
        },
        docs: &[
            Doc::Components(Component::Dialog),
            Doc::Components(Component::Menu),
            Doc::Components(Component::Tooltip),
            Doc::Components(Component::Popover),
            Doc::Components(Component::Toast),
        ],
    },
    Group {
        title: L {
            en: "Display",
            ja: "表示",
        },
        docs: &[
            Doc::Components(Component::Avatar),
            Doc::Components(Component::Kbd),
        ],
    },
    Group {
        title: L {
            en: "Feedback",
            ja: "フィードバック",
        },
        docs: &[
            Doc::Components(Component::Alert),
            Doc::Components(Component::Badge),
            Doc::Components(Component::Progress),
            Doc::Components(Component::Spinner),
        ],
    },
];

/// The sidebar of the Style section.
pub const STYLE: &[Group] = &[
    Group {
        title: L {
            en: "Get started",
            ja: "はじめに",
        },
        docs: &[Doc::Style(StylePage::Overview)],
    },
    Group {
        title: L {
            en: "Writing styles",
            ja: "スタイルを書く",
        },
        docs: &[
            Doc::Style(StylePage::Values),
            Doc::Style(StylePage::Conditions),
            Doc::Style(StylePage::Tokens),
        ],
    },
    Group {
        title: L {
            en: "The page",
            ja: "ページ",
        },
        docs: &[
            Doc::Style(StylePage::Global),
            Doc::Style(StylePage::Patterns),
            Doc::Style(StylePage::Animation),
        ],
    },
    Group {
        title: L {
            en: "Opting out of type checks",
            ja: "型の検査を外す",
        },
        docs: &[Doc::Style(StylePage::Unchecked)],
    },
];

/// The sidebar of the i18n section.
pub const I18N: &[Group] = &[
    Group {
        title: L {
            en: "Get started",
            ja: "はじめに",
        },
        docs: &[Doc::I18n(I18nPage::Overview)],
    },
    Group {
        title: L {
            en: "Guides",
            ja: "ガイド",
        },
        docs: &[Doc::I18n(I18nPage::Pages), Doc::I18n(I18nPage::Negotiation)],
    },
];

impl Doc {
    /// Every page, in the order the site lists them.
    pub const ALL: &[Self] = &[
        Self::Home,
        Self::StyleHome,
        Self::ComponentsHome,
        Self::UseCases,
        // Get started
        Self::Learn(Learn::QuickStart),
        Self::Learn(Learn::Why),
        Self::Learn(Learn::DevServer),
        // Building the page
        Self::Learn(Learn::Elements),
        Self::Learn(Learn::ThePage),
        Self::Learn(Learn::Bindings),
        Self::Learn(Learn::Components),
        Self::Learn(Learn::Styling),
        Self::Learn(Learn::DescribingThePage),
        // Browser state
        Self::Learn(Learn::Signals),
        Self::Learn(Learn::NestedSignals),
        Self::Learn(Learn::Expressions),
        Self::Learn(Learn::Statements),
        Self::Learn(Learn::Events),
        Self::Learn(Learn::BindingInputs),
        Self::Learn(Learn::ComputedSignals),
        Self::Learn(Learn::PersistedSignals),
        Self::Learn(Learn::Lists),
        Self::Learn(Learn::ListItems),
        // Talking to the server
        Self::Learn(Learn::Actions),
        Self::Learn(Learn::CallStatus),
        Self::Learn(Learn::Optimistic),
        Self::Learn(Learn::OverlappingCalls),
        Self::Learn(Learn::Patches),
        Self::Learn(Learn::ListPatches),
        Self::Learn(Learn::Fragments),
        Self::Learn(Learn::Streams),
        Self::Learn(Learn::Server),
        Self::Learn(Learn::CallsInDepth),
        // Pages and routing
        Self::Learn(Learn::Routes),
        Self::Learn(Learn::Links),
        Self::Learn(Learn::ChildPages),
        Self::Learn(Learn::SearchParams),
        Self::Learn(Learn::ErrorPages),
        Self::Learn(Learn::BasePath),
        Self::Learn(Learn::Pages),
        // Forms and requests
        Self::Learn(Learn::Forms),
        Self::Learn(Learn::FormValidation),
        Self::Learn(Learn::Requests),
        Self::Learn(Learn::Errors),
        Self::Learn(Learn::Uploads),
        // Data and sign-in
        Self::Learn(Learn::Database),
        Self::Learn(Learn::DatabaseWithoutSql),
        Self::Learn(Learn::CheckedSql),
        Self::Learn(Learn::HostedDatabases),
        Self::Learn(Learn::DatabaseHosts),
        Self::Learn(Learn::Api),
        Self::Learn(Learn::Sessions),
        Self::Learn(Learn::MembersOnly),
        Self::Learn(Learn::SessionCookie),
        Self::Learn(Learn::SessionStores),
        // Browser features
        Self::Learn(Learn::NodeRefs),
        Self::Learn(Learn::Timers),
        Self::Learn(Learn::Keyboard),
        Self::Learn(Learn::JavaScript),
        Self::Learn(Learn::UncheckedValues),
        Self::Learn(Learn::StaticPages),
        Self::Learn(Learn::EscapeHatches),
        // Quality and operations
        Self::Learn(Learn::Security),
        Self::Learn(Learn::Testing),
        Self::Learn(Learn::TestingPages),
        Self::Learn(Learn::CompileErrors),
        Self::Learn(Learn::Debugging),
        Self::Learn(Learn::Upgrading),
        Self::Learn(Learn::Troubleshooting),
        // Deploy
        Self::Learn(Learn::Deploying),
        Self::Learn(Learn::DeployContainer),
        Self::Learn(Learn::DeployCloudflare),
        Self::Learn(Learn::DeployVercel),
        Self::Reference(Reference::Overview),
        Self::Reference(Reference::Page),
        Self::Reference(Reference::El),
        Self::Reference(Reference::Html),
        Self::Reference(Reference::Signals),
        Self::Reference(Reference::Expr),
        Self::Reference(Reference::Stmt),
        Self::Reference(Reference::NodeRef),
        Self::Reference(Reference::Action),
        Self::Reference(Reference::Form),
        Self::Reference(Reference::Patches),
        Self::Reference(Reference::Route),
        Self::Reference(Reference::RouterExt),
        Self::Components(Component::Installation),
        Self::Components(Component::Theming),
        Self::Components(Component::Button),
        Self::Components(Component::TextField),
        Self::Components(Component::Checkbox),
        Self::Components(Component::RadioGroup),
        Self::Components(Component::Select),
        Self::Components(Component::Slider),
        Self::Components(Component::Toggle),
        Self::Components(Component::Listbox),
        Self::Components(Component::Combobox),
        Self::Components(Component::Tabs),
        Self::Components(Component::Accordion),
        Self::Components(Component::Collapsible),
        Self::Components(Component::Card),
        Self::Components(Component::Separator),
        Self::Components(Component::Navigation),
        Self::Components(Component::DataTable),
        Self::Components(Component::Chart),
        Self::Components(Component::Dialog),
        Self::Components(Component::Menu),
        Self::Components(Component::Tooltip),
        Self::Components(Component::Popover),
        Self::Components(Component::Toast),
        Self::Components(Component::Alert),
        Self::Components(Component::Badge),
        Self::Components(Component::Progress),
        Self::Components(Component::Spinner),
        Self::Components(Component::Avatar),
        Self::Components(Component::Kbd),
        Self::Style(StylePage::Overview),
        Self::Style(StylePage::Values),
        Self::Style(StylePage::Conditions),
        Self::Style(StylePage::Tokens),
        Self::Style(StylePage::Global),
        Self::Style(StylePage::Patterns),
        Self::Style(StylePage::Animation),
        Self::Style(StylePage::Unchecked),
        Self::I18nHome,
        Self::I18n(I18nPage::Overview),
        Self::I18n(I18nPage::Pages),
        Self::I18n(I18nPage::Negotiation),
    ];

    /// The page's directory inside the site, ending in `/`, or `""` for the
    /// landing page.
    #[must_use]
    pub fn path(self) -> String {
        match self {
            Self::Home => String::new(),
            Self::StyleHome => "style/".to_owned(),
            Self::ComponentsHome => "components/".to_owned(),
            Self::UseCases => "use-cases/".to_owned(),
            Self::Learn(Learn::QuickStart) => "learn/".to_owned(),
            Self::Reference(Reference::Overview) => "reference/".to_owned(),
            Self::Learn(page) => format!("learn/{}/", page.slug()),
            Self::Reference(page) => format!("reference/{}/", page.slug()),
            Self::Components(page) => format!("components/{}/", page.slug()),
            Self::Style(page) => format!("style/{}/", page.slug()),
            Self::I18nHome => "i18n/".to_owned(),
            Self::I18n(page) => format!("i18n/{}/", page.slug()),
        }
    }

    /// The page's title, as the sidebar and the browser tab show it.
    #[must_use]
    pub fn title(self, site: Site) -> &'static str {
        site.t(self.titles())
    }

    /// The page's title in each language.
    fn titles(self) -> L<&'static str> {
        let (en, ja) = match self {
            Self::Home => ("hypersig", "hypersig"),
            Self::StyleHome => ("hypersig style", "hypersig style"),
            Self::I18nHome => ("hypersig i18n", "hypersig i18n"),
            Self::UseCases => ("Use cases", "ユースケース"),
            Self::I18n(page) => match page {
                I18nPage::Overview => ("Overview", "概要"),
                I18nPage::Pages => ("Pages in each language", "言語ごとのページ"),
                I18nPage::Negotiation => ("Choosing the language", "言語の選び方"),
            },
            Self::ComponentsHome => ("hypersig components", "hypersig components"),
            Self::Learn(page) => match page {
                // Get started
                Learn::QuickStart => ("Quick start", "クイックスタート"),
                Learn::Why => ("Why hypersig", "hypersig を選ぶ理由"),
                Learn::DevServer => ("The development server", "開発サーバー"),
                // Building the page
                Learn::Elements => ("Elements and attributes", "要素と属性"),
                Learn::ThePage => ("The page and its head", "ページと head"),
                Learn::Bindings => ("Values that follow the signals", "シグナルに追従する値"),
                Learn::Components => ("Components", "コンポーネント"),
                Learn::Styling => ("Styling", "スタイル"),
                Learn::DescribingThePage => ("Building the page", "ページを組み立てる"),
                // Browser state
                Learn::Signals => ("Signals", "シグナル"),
                Learn::NestedSignals => ("Signals inside signals", "入れ子のシグナル"),
                Learn::Expressions => ("Expressions and statements", "式と文"),
                Learn::Statements => ("Statements", "文"),
                Learn::Events => ("Events", "イベント"),
                Learn::BindingInputs => ("Binding form controls", "入力欄の束縛"),
                Learn::ComputedSignals => ("Computed signals", "計算シグナル"),
                Learn::PersistedSignals => (
                    "Signals kept across page loads",
                    "ページをまたいで保つシグナル",
                ),
                Learn::Lists => ("Lists", "リスト"),
                Learn::ListItems => ("The state of a list item", "リスト項目の状態"),
                // Talking to the server
                Learn::Actions => ("Actions", "アクション"),
                Learn::CallStatus => ("Busy and failed calls", "呼び出し中と失敗"),
                Learn::Optimistic => ("Optimistic updates", "楽観的更新"),
                Learn::OverlappingCalls => ("Calls that overlap", "重なった呼び出し"),
                Learn::Patches => ("Patches", "差分"),
                Learn::ListPatches => ("List patches", "リストの差分"),
                Learn::Fragments => ("Fragments", "フラグメント"),
                Learn::Streams => ("Streams", "ストリーム"),
                Learn::Server => ("Talking to the server", "サーバーとのやり取り"),
                Learn::CallsInDepth => ("Calls in depth", "呼び出しの詳細"),
                // Pages and routing
                Learn::Routes => ("Routes", "ルート"),
                Learn::Links => ("Links and redirects", "リンクとリダイレクト"),
                Learn::ChildPages => ("Pages under a page", "子のページ"),
                Learn::SearchParams => ("Search parameters", "検索パラメータ"),
                Learn::ErrorPages => ("Error pages", "エラーページ"),
                Learn::BasePath => ("Serving under a path prefix", "パスの接頭辞の下で配信する"),
                Learn::Pages => ("Pages and links", "ページとリンク"),
                // Forms and requests
                Learn::Forms => ("Forms", "フォーム"),
                Learn::FormValidation => ("Validating a form", "フォームの検証"),
                Learn::Requests => (
                    "Requests, cookies, and errors",
                    "リクエスト、Cookie、エラー",
                ),
                Learn::Errors => ("Errors and logging", "エラーとログ"),
                Learn::Uploads => ("Uploads", "アップロード"),
                // Data and sign-in
                Learn::Database => ("Storing data", "データを保存する"),
                Learn::DatabaseWithoutSql => {
                    ("A database without SQL", "SQL を使わないデータベース")
                }
                Learn::CheckedSql => ("SQL the compiler checks", "コンパイラが検査する SQL"),
                Learn::HostedDatabases => (
                    "Hosted PostgreSQL and MySQL",
                    "ホスティングされた PostgreSQL と MySQL",
                ),
                Learn::DatabaseHosts => (
                    "The database each host can reach",
                    "デプロイ先ごとのデータベース",
                ),
                Learn::Api => (
                    "Calling another service's API",
                    "ほかのサービスの API を呼ぶ",
                ),
                Learn::Sessions => ("Signing in", "サインインとセッション"),
                Learn::MembersOnly => (
                    "Pages for signed-in members",
                    "サインインした人だけのページ",
                ),
                Learn::SessionCookie => ("The session cookie", "セッションの Cookie"),
                Learn::SessionStores => ("Where sessions are stored", "セッションの保存先"),
                // Browser features
                Learn::NodeRefs => ("Node refs", "ノード参照"),
                Learn::Timers => ("Timers and reactions", "タイマーと変化への反応"),
                Learn::Keyboard => ("Keyboard patterns", "キーボード操作"),
                Learn::JavaScript => ("JavaScript libraries", "JavaScript のライブラリ"),
                Learn::UncheckedValues => ("Values the types do not check", "型で検査しない値"),
                Learn::StaticPages => ("Static pages", "静的ページ"),
                Learn::EscapeHatches => (
                    "When bindings and statements are not enough",
                    "束縛と文で足りないとき",
                ),
                // Quality and operations
                Learn::Security => ("Security", "セキュリティ"),
                Learn::Testing => ("Testing", "テストを書く"),
                Learn::TestingPages => (
                    "Testing a page without a browser",
                    "ブラウザなしでページをテストする",
                ),
                Learn::CompileErrors => ("Reading a compile error", "コンパイルエラーの読み方"),
                Learn::Debugging => ("Debugging a running page", "動いているページを調べる"),
                Learn::Upgrading => ("Upgrading", "新しいリリースに上げる"),
                Learn::Troubleshooting => ("Troubleshooting", "うまく動かないとき"),
                // Deploy
                Learn::Deploying => ("Choosing a host", "デプロイ先の選び方"),
                Learn::DeployContainer => ("In a container", "コンテナ"),
                Learn::DeployCloudflare => ("Cloudflare Workers", "Cloudflare Workers"),
                Learn::DeployVercel => ("Vercel", "Vercel"),
            },
            Self::Reference(page) => match page {
                Reference::Overview => ("API reference", "API リファレンス"),
                Reference::Page => ("Page", "Page"),
                Reference::El => ("El", "El"),
                Reference::Html => ("html functions", "html の関数"),
                Reference::Signals => ("Signals and Sig", "Signals と Sig"),
                Reference::Expr => ("Expr", "Expr"),
                Reference::Stmt => ("Stmt", "Stmt"),
                Reference::Action => ("Action and Call", "Action と Call"),
                Reference::Form => ("Form and Validate", "Form と Validate"),
                Reference::Patches => ("Patches and Fragment", "Patches と Fragment"),
                Reference::Route => ("Route and Link", "Route と Link"),
                Reference::NodeRef => ("NodeRef", "NodeRef"),
                Reference::RouterExt => ("RouterExt", "RouterExt"),
            },
            Self::Components(page) => match page {
                Component::Installation => ("Installation", "インストール"),
                Component::Theming => ("Theming", "テーマ"),
                Component::Button => ("Button", "ボタン"),
                Component::TextField => ("Text field", "テキストフィールド"),
                Component::Checkbox => ("Checkbox and switch", "チェックボックスとスイッチ"),
                Component::RadioGroup => ("Radio group", "ラジオグループ"),
                Component::Select => ("Select", "セレクト"),
                Component::Slider => ("Slider", "スライダー"),
                Component::Toggle => ("Toggle and toggle group", "トグルとトグルグループ"),
                Component::Listbox => ("Listbox", "リストボックス"),
                Component::Combobox => ("Combobox", "コンボボックス"),
                Component::Tabs => ("Tabs", "タブ"),
                Component::Accordion => ("Accordion", "アコーディオン"),
                Component::Collapsible => ("Collapsible", "折りたたみ"),
                Component::Card => ("Card", "カード"),
                Component::Separator => ("Separator", "区切り線"),
                Component::Navigation => ("Navigation", "ナビゲーション"),
                Component::DataTable => ("Data table", "データテーブル"),
                Component::Chart => ("Chart", "チャート"),
                Component::Dialog => ("Dialog", "ダイアログ"),
                Component::Menu => ("Menu", "メニュー"),
                Component::Tooltip => ("Tooltip", "ツールチップ"),
                Component::Popover => ("Popover", "ポップオーバー"),
                Component::Toast => ("Toast", "トースト"),
                Component::Alert => ("Alert", "アラート"),
                Component::Badge => ("Badge", "バッジ"),
                Component::Progress => ("Progress and skeleton", "プログレスとスケルトン"),
                Component::Spinner => ("Spinner", "スピナー"),
                Component::Avatar => ("Avatar", "アバター"),
                Component::Kbd => ("Kbd", "Kbd"),
            },
            Self::Style(page) => match page {
                StylePage::Overview => ("Overview", "概要"),
                StylePage::Values => ("Properties and values", "プロパティと値"),
                StylePage::Conditions => ("Conditions", "条件"),
                StylePage::Tokens => ("Tokens and themes", "トークンとテーマ"),
                StylePage::Global => ("Global styles", "グローバルスタイル"),
                StylePage::Patterns => ("Layout patterns", "レイアウトのパターン"),
                StylePage::Animation => ("Animation", "アニメーション"),
                StylePage::Unchecked => ("Unchecked values", "検査しない値"),
            },
        };
        L { en, ja }
    }

    /// The paragraph under the page's title that says what it covers, and
    /// the description search results and link previews show for it. A
    /// landing page has no such paragraph and takes its library's
    /// description.
    pub(crate) fn leads(self) -> L<&'static str> {
        match self {
            Self::Home | Self::StyleHome | Self::ComponentsHome | Self::I18nHome => {
                self.library().descriptions()
            }
            Self::UseCases => L {
                en: "Applications that hypersig suits. For each feature added to one of them, the page shows the change in hypersig and in Next.js.",
                ja: "hypersig に向くアプリと、そのアプリに仕様を 1 つ追加するごとに、hypersig と Next.js でコードがどう変わるかを並べます。",
            },
            Self::Learn(page) => match page {
                // Get started
                Learn::QuickStart => L {
                    en: "Build a counter that counts in the browser and resets on the server, in one Rust file.",
                    ja: "ブラウザで数を数え、サーバーで 0 に戻すカウンターを、Rust のファイル 1 つで作ります。",
                },
                Learn::Why => L {
                    en: "hypersig renders the page on the server and writes its behavior into the HTML. Every string the browser acts on comes from a Rust type, so the page needs no browser build, no hydration, and no string typed by hand.",
                    ja: "hypersig はページをサーバーで描画し、その動きを HTML に書き込みます。ブラウザが扱う文字列はすべて Rust の型から作られるので、ブラウザ向けのビルドもハイドレーションも不要で、手で書く文字列もありません。",
                },
                Learn::DevServer => Doc::Learn(Learn::QuickStart).leads(),
                // Building the page
                Learn::Elements | Learn::ThePage | Learn::Bindings | Learn::Components => {
                    Doc::Learn(Learn::DescribingThePage).leads()
                }
                Learn::Styling => L {
                    en: "Styles are Rust values too. A misspelled property or keyword, or a value of the wrong type, is a compile error.",
                    ja: "スタイルも Rust の値です。プロパティやキーワードの打ち間違いや、型の違う値はコンパイルエラーになります。",
                },
                Learn::DescribingThePage => L {
                    en: "A page is a tree of `El` values built with Rust functions and rendered to HTML on the server.",
                    ja: "ページは Rust の関数で組み立てた `El` の値の木で、サーバーが HTML に描画します。",
                },
                // Browser state
                Learn::Signals => L {
                    en: "The page's state is one Rust struct, and a signal is a typed reference to one of its fields.",
                    ja: "ページの状態は Rust の構造体 1 つで表し、シグナルはそのフィールド 1 つを指す型付きの参照です。",
                },
                Learn::NestedSignals => Doc::Learn(Learn::Signals).leads(),
                Learn::Expressions => L {
                    en: "An expression computes a value from the signals, and a statement changes them. Both are Rust values that the browser receives as JSON.",
                    ja: "式はシグナルから値を計算し、文はシグナルを変更します。どちらも Rust の値で、ブラウザは JSON として受け取ります。",
                },
                Learn::Statements | Learn::Events | Learn::BindingInputs => {
                    Doc::Learn(Learn::Expressions).leads()
                }
                Learn::ComputedSignals | Learn::PersistedSignals => {
                    Doc::Learn(Learn::Signals).leads()
                }
                Learn::Lists => L {
                    en: "A `Vec` field is a list. `each` renders an element per item in the browser, and list statements change it without a request.",
                    ja: "`Vec` のフィールドはリストです。`each` はブラウザで項目ごとに要素を描画し、リストの文はリクエストを送らずにリストを変更します。",
                },
                Learn::ListItems => Doc::Learn(Learn::Lists).leads(),
                // Talking to the server
                Learn::Actions
                | Learn::CallStatus
                | Learn::Optimistic
                | Learn::Patches
                | Learn::Fragments
                | Learn::Streams => Doc::Learn(Learn::Server).leads(),
                Learn::OverlappingCalls | Learn::ListPatches => {
                    Doc::Learn(Learn::CallsInDepth).leads()
                }
                Learn::Server => L {
                    en: "The browser calls an action, and the server answers with patches: new values for signals, or new HTML for a part of the page.",
                    ja: "ブラウザがアクションを呼び出すと、サーバーは差分を返します。差分はシグナルの新しい値か、ページの一部の新しい HTML です。",
                },
                Learn::CallsInDepth => L {
                    en: "Patches that send only the items of a list that change, calls that overlap, streams that reconnect, and parts that several pages share.",
                    ja: "リストの変わった項目だけを送る差分、重なった呼び出し、つなぎ直すストリーム、複数のページで使う部品を扱います。",
                },
                // Pages and routing
                Learn::Routes => L {
                    en: "Typed query strings, URLs that change without loading a page, pages for missing paths, what search results and link previews show, and an application served under a path prefix.",
                    ja: "型の付いたクエリ文字列、ページを読み込まずに変える URL、存在しないパスのページ、検索結果とリンクのプレビューに出る内容、パスの接頭辞の下で配信するアプリを扱います。",
                },
                Learn::Links | Learn::ChildPages => Doc::Learn(Learn::Pages).leads(),
                Learn::SearchParams | Learn::ErrorPages | Learn::BasePath => {
                    Doc::Learn(Learn::Routes).leads()
                }
                Learn::Pages => L {
                    en: "A route is a page the server renders at a path. The router serves it and the links on other pages point to it from the same value, so a parameter of the wrong type or in the wrong number is a compile error.",
                    ja: "ルートは、あるパスでサーバーが描画するページです。ルーターの配信もほかのページからのリンクも同じ値から作るので、パラメータの型や数の誤りはコンパイルエラーになります。",
                },
                // Forms and requests
                Learn::Forms => L {
                    en: "A form's rules are written once in Rust. The browser checks them as the visitor types, and the handler checks the same rules before it trusts the values.",
                    ja: "フォームのルールは Rust で 1 回だけ書きます。ブラウザは入力に合わせてそのルールを検査し、ハンドラは値を使う前に同じルールで検査します。",
                },
                Learn::FormValidation => Doc::Learn(Learn::Forms).leads(),
                Learn::Requests => L {
                    en: "A handler reads the request through an axum extractor, sets cookies by returning headers beside its answer, and fails with `?`.",
                    ja: "ハンドラは axum の extractor でリクエストを読み、応答と一緒にヘッダーを返して Cookie を設定し、`?` で失敗を返します。",
                },
                Learn::Errors | Learn::Uploads => Doc::Learn(Learn::Requests).leads(),
                // Data and sign-in
                Learn::Database => L {
                    en: "A handler keeps the data that requests share in a database its own Rust code connects to, with the connection in the router's state. The page also covers telling the streams of every server process about a change.",
                    ja: "ハンドラはリクエストをまたいで使うデータを、自分の Rust のコードからつなぐデータベースに置き、接続はルーターの状態に持たせます。変更をサーバーのすべてのプロセスのストリームに知らせる方法も、このページで説明します。",
                },
                Learn::DatabaseWithoutSql => L {
                    en: "A database read through an SDK, such as DynamoDB, is used the same way as one read with SQL: the client goes in the router's state, and the handler awaits its calls.",
                    ja: "DynamoDB のように SDK で読むデータベースも、SQL で読むものと同じように使います。クライアントをルーターの状態に置き、ハンドラはその呼び出しを `.await` します。",
                },
                Learn::CheckedSql => L {
                    en: "sqlx's `query!` macros check each query against the database's tables when the program is built, and the `.sqlx` directory lets a machine without a database build it.",
                    ja: "sqlx の `query!` などのマクロはビルドのときにクエリをデータベースのテーブルと照らし合わせて検査します。`.sqlx` ディレクトリがあれば、データベースのないマシンでもビルドできます。",
                },
                Learn::HostedDatabases => L {
                    en: "Supabase, PlanetScale, Cloud SQL, AlloyDB, RDS, and Aurora each take a connection string in `DATABASE_URL`, and differ in their ports, poolers, and TLS settings.",
                    ja: "Supabase、PlanetScale、Cloud SQL、AlloyDB、RDS、Aurora は、どれも `DATABASE_URL` の接続文字列でつなぎます。違うのはポート、プーラー、TLS の設定です。",
                },
                Learn::DatabaseHosts => L {
                    en: "A container, a Vercel Function, and a Cloudflare Worker each limit where the data can be kept: a file on a volume, a database over the network, or a binding the host provides.",
                    ja: "コンテナ、Vercel Functions、Cloudflare Workers では、データを置ける場所が違います。ボリューム上のファイル、ネットワーク越しのデータベース、ホストが用意するバインディングのいずれかです。",
                },
                Learn::Api => L {
                    en: "A handler can call another service's HTTP API and answer the browser with what it returned, so the service's address and token stay on the server.",
                    ja: "ハンドラはほかのサービスの HTTP の API を呼び、その結果でブラウザに応答できます。サービスのアドレスとトークンはサーバーから出ません。",
                },
                Learn::Sessions => L {
                    en: "The `tower-sessions` layer keeps a session for each browser, which a cookie points to. An action signs the visitor in by writing the member's name into the session, and signs them out by emptying it.",
                    ja: "`tower-sessions` のレイヤーはブラウザごとのセッションを持ち、ブラウザは Cookie でそのセッションを指します。アクションはセッションにメンバーの名前を書いて訪問者をサインインさせ、セッションを空にしてサインアウトさせます。",
                },
                Learn::MembersOnly => L {
                    en: "An extractor reads the signed-in member from the session, so a handler that takes it runs only for a signed-in visitor. Anyone else gets a redirect to the sign-in page from a page, and a 401 from an action.",
                    ja: "エクストラクタがセッションからサインイン中のメンバーを読むので、それを受け取るハンドラはサインインした訪問者に対してだけ動きます。それ以外の訪問者には、ページならサインインのページへのリダイレクトを、アクションなら 401 を返します。",
                },
                Learn::SessionCookie => L {
                    en: "The session's cookie is `HttpOnly`, `Secure`, and `SameSite=Strict` unless the layer says otherwise, and lasts until the browser closes unless the layer is given an expiry.",
                    ja: "セッションの Cookie には、レイヤーで変えない限り `HttpOnly`、`Secure`、`SameSite=Strict` が付きます。レイヤーに有効期限を渡さなければ、ブラウザを閉じるまで保持されます。",
                },
                Learn::SessionStores => L {
                    en: "`MemoryStore` keeps the sessions inside one process, so a server that runs more than one instance keeps them in PostgreSQL, SQLite, MySQL, or Redis.",
                    ja: "`MemoryStore` はセッションを 1 つのプロセスの中に置きます。インスタンスを複数動かすサーバーでは、PostgreSQL、SQLite、MySQL、Redis に置きます。",
                },
                // Browser features
                Learn::NodeRefs
                | Learn::Timers
                | Learn::Keyboard
                | Learn::UncheckedValues
                | Learn::StaticPages => Doc::Learn(Learn::EscapeHatches).leads(),
                Learn::JavaScript => L {
                    en: "A map, a chart, or a rich text editor runs in the browser as a JavaScript library. Wrapped in a custom element, it takes values from the page's signals and hands its events back as typed values.",
                    ja: "地図、グラフ、リッチテキストエディタは、ブラウザで JavaScript のライブラリとして動きます。カスタム要素で包むと、ページのシグナルから値を受け取り、イベントを型付きの値としてページに返せます。",
                },
                Learn::EscapeHatches => L {
                    en: "Most pages need only bindings and statements. The pages here cover focus, timers, and reactions to a change, links outside the router, and serving a page as a static file.",
                    ja: "たいていのページは束縛と文だけで書けます。ここで扱うのは、フォーカス、タイマー、変化への反応、ルーターの外へのリンク、ページを静的ファイルとして配信する方法です。",
                },
                // Quality and operations
                Learn::Security => L {
                    en: "What hypersig checks so that a page and its actions resist what a visitor can send, why data from the browser cannot run code on the server, and what is left to the application.",
                    ja: "訪問者が送るものからページとアクションを守るために hypersig が検査すること、ブラウザから届くデータがサーバーのコードを動かせない理由、アプリに任せることをまとめます。",
                },
                Learn::Testing => L {
                    en: "Tests that call a page's and an action's handler as functions and read the signals with their types, tests that send a request through the router, tests that click and type on a page without a browser, and what only a browser test reaches.",
                    ja: "ページとアクションのハンドラを関数として呼び、シグナルの値を型で読むテスト、ルーターにリクエストを送るテスト、ブラウザなしでページをクリックして入力するテスト、ブラウザのテストでしか確かめられないことを扱います。",
                },
                Learn::TestingPages => Doc::Learn(Learn::Testing).leads(),
                Learn::CompileErrors | Learn::Debugging => {
                    Doc::Learn(Learn::Troubleshooting).leads()
                }
                Learn::Upgrading => L {
                    en: "Each change of a public signature from 0.4 to 0.8, shown as the code before and after it.",
                    ja: "0.4 から 0.8 までに公開のシグネチャが変わった箇所を、変更前と変更後のコードで示します。",
                },
                Learn::Troubleshooting => L {
                    en: "How to read the compile errors hypersig's types and constants produce, and where a running page reports a panic, a failed call, or a problem the runtime meets.",
                    ja: "hypersig の型と定数が出すコンパイルエラーの読み方と、動いているページがパニック、失敗した呼び出し、ランタイムが見つけた問題をどこに報告するかを扱います。",
                },
                // Deploy
                Learn::Deploying => L {
                    en: "A template gives a project that deploys as it is: a container image for a container host, a worker for Cloudflare Workers, or a function for Vercel. Each takes a few commands from an empty directory to a running application.",
                    ja: "テンプレートから、そのままデプロイできるプロジェクトを作れます。コンテナのホスト向けのイメージ、Cloudflare Workers 向けの Worker、Vercel 向けの関数の 3 つがあり、どれも空のディレクトリから数行のコマンドで動くところまで進みます。",
                },
                Learn::DeployContainer => L {
                    en: "The server template builds a container image with Docker, which a host such as Fly.io, Cloud Run, or Render runs.",
                    ja: "サーバーのテンプレートは Docker でコンテナのイメージを作ります。このイメージは Fly.io、Cloud Run、Render のようなホストで動きます。",
                },
                Learn::DeployCloudflare => L {
                    en: "The Workers template compiles the same pages, actions, and streams to wasm, and the worker hands each request to the axum `Router`.",
                    ja: "Workers のテンプレートは同じページ、アクション、ストリームを wasm にコンパイルし、Worker は受け取ったリクエストを axum の `Router` に渡します。",
                },
                Learn::DeployVercel => L {
                    en: "The Vercel template runs the same pages, actions, and streams as a Vercel Function, where `VercelLayer` hands each request to the axum `Router`.",
                    ja: "Vercel のテンプレートは同じページ、アクション、ストリームを Vercel の関数として動かし、`VercelLayer` が受け取ったリクエストを axum の `Router` に渡します。",
                },
            },
            Self::Style(page) => match page {
                StylePage::Overview => L {
                    en: "`hypersig::style` is typed atomic CSS. A misspelled property name or keyword, or a value of the wrong type, is a compile error. Each declaration becomes one class.",
                    ja: "`hypersig::style` は型の付いた atomic CSS です。プロパティ名やキーワードの綴りを間違えたり、型の違う値を渡したりするとコンパイルエラーになります。宣言は 1 つごとに 1 つのクラスになります。",
                },
                StylePage::Values => L {
                    en: "Each property method takes its own value type, which has the property's keywords and accepts values in the units the property allows.",
                    ja: "プロパティのメソッドはそれぞれ専用の値の型を受け取ります。その型はプロパティのキーワードを持ち、プロパティで使える単位の値を受け付けます。",
                },
                StylePage::Conditions => L {
                    en: "`Style::on` applies part of a style only under a condition. The condition can be a pseudo-class, a media feature, a container width, or the state of a marked ancestor or sibling.",
                    ja: "`Style::on` はスタイルの一部を条件の下でだけ適用します。条件には擬似クラス、メディア特性、コンテナの幅、印を付けた祖先や兄弟の状態を使えます。",
                },
                StylePage::Tokens => L {
                    en: "A token is a named value written as a CSS custom property. A theme sets its value for the whole page, and a style sets it for one element.",
                    ja: "トークンは CSS のカスタムプロパティとして書かれる名前付きの値です。テーマはページ全体に、スタイルは 1 つの要素にトークンの値を設定します。",
                },
                StylePage::Global => L {
                    en: "A `Global` holds rules that select elements by tag, a small reset, and the fonts the page loads.",
                    ja: "`Global` には、タグで要素を選ぶルール、小さなリセット、ページが読み込むフォントが入ります。",
                },
                StylePage::Patterns => L {
                    en: "[`hypersig::style::pattern`](api:hypersig_style/pattern/index.html) has functions that return the styles of common layouts. Merge the returned style with your own.",
                    ja: "[`hypersig::style::pattern`](api:hypersig_style/pattern/index.html) には、よく使うレイアウトのスタイルを返す関数があります。返されたスタイルは自分のスタイルと `merge` して使います。",
                },
                StylePage::Animation => L {
                    en: "Keyframes are Rust values too. When an element uses keyframes, their rule is written to the page along with the element. The move from one page to the next animates with the browser's view transitions.",
                    ja: "キーフレームも Rust の値です。要素がキーフレームを使うと、そのキーフレームのルールも要素と一緒にページに書かれます。ページから次のページへの移動は、ブラウザの View Transitions でアニメーションできます。",
                },
                StylePage::Unchecked => L {
                    en: "A few forms reach the browser without a check. Each has its own name, so searching for that name finds every place that uses an unchecked value.",
                    ja: "いくつかの書き方は検査を受けずにブラウザへ渡ります。どの書き方にも決まった名前があるので、その名前で検索すれば、検査なしの値を使う箇所をすべて見つけられます。",
                },
            },
            Self::I18n(page) => match page {
                I18nPage::Overview => L {
                    en: "`hypersig_i18n` keeps a text and its translations together in one value, so a text missing a translation into any language fails to compile.",
                    ja: "`hypersig_i18n` は 1 つの文とその訳を 1 つの値にまとめて持ちます。そのため、どれかの言語の訳が欠けた文はコンパイルエラーになります。",
                },
                I18nPage::Pages => L {
                    en: "The language is the first segment of each page's route, so the page in another language is the same route with only the language changed.",
                    ja: "言語は各ページのルートの最初のセグメントに置きます。そのため、別の言語のページは言語だけを変えた同じルートになります。",
                },
                I18nPage::Negotiation => L {
                    en: "A visitor who opens `/` is redirected to the page in the language their browser asks for.",
                    ja: "`/` を開いた訪問者を、ブラウザが求める言語のページへリダイレクトします。",
                },
            },
            Self::Components(page) => match page {
                Component::Installation => L {
                    en: "`hypersig_ui` is a set of components that look like shadcn/ui and take their behavior from the browser's native elements and the WAI-ARIA Authoring Practices.",
                    ja: "`hypersig_ui` は、見た目が shadcn/ui に似ており、振る舞いはブラウザのネイティブ要素と WAI-ARIA Authoring Practices に従うコンポーネント集です。",
                },
                Component::Theming => L {
                    en: "The components read their colors and radius from design tokens, which a theme sets for the page and a style sets for one element.",
                    ja: "コンポーネントは色と角の丸みをデザイントークンから読みます。トークンの値は、ページ全体にはテーマが、1 つの要素にはスタイルが設定します。",
                },
                _ => components::component_lead(page),
            },
            Self::Reference(Reference::Overview) => L {
                en: "The items of the `hypersig` crate, grouped by what they build. Each name links to its full documentation.",
                ja: "`hypersig` クレートの項目を、組み立てるものごとにまとめています。名前をクリックすると、その項目の完全なドキュメントを開けます。",
            },
            Self::Reference(page) => reference::page_lead(page),
        }
    }

    /// The library whose docs the page belongs to.
    pub(crate) fn library(self) -> Library {
        match self {
            Self::Home | Self::UseCases | Self::Learn(_) | Self::Reference(_) => Library::Framework,
            Self::StyleHome | Self::Style(_) => Library::Style,
            Self::I18nHome | Self::I18n(_) => Library::I18n,
            Self::ComponentsHome | Self::Components(_) => Library::Components,
        }
    }

    /// Whether the page is a library's landing page.
    pub(crate) fn is_landing(self) -> bool {
        self == self.library().home()
    }

    /// Whether the page is listed in a sidebar, which it then shows.
    pub(crate) fn has_sidebar(self) -> bool {
        !self.groups().is_empty()
    }

    /// The URL of the page.
    pub(crate) fn url(self, site: Site) -> String {
        site.page_url(&self.path())
    }

    /// The sidebar the page is listed in.
    pub(crate) fn groups(self) -> &'static [Group] {
        match self {
            Self::Home
            | Self::StyleHome
            | Self::ComponentsHome
            | Self::I18nHome
            | Self::UseCases => &[],
            Self::Learn(_) => LEARN,
            Self::Reference(_) => REFERENCE,
            Self::Components(_) => COMPONENTS,
            Self::Style(_) => STYLE,
            Self::I18n(_) => I18N,
        }
    }

    /// The pages before and after this one in its sidebar.
    pub(crate) fn neighbors(self) -> (Option<Self>, Option<Self>) {
        let docs: Vec<Self> = self
            .groups()
            .iter()
            .flat_map(|group| group.docs.iter().copied())
            .collect();
        let Some(at) = docs.iter().position(|&doc| doc == self) else {
            return (None, None);
        };
        let before = at.checked_sub(1).map(|i| docs[i]);
        (before, docs.get(at + 1).copied())
    }

    /// The page as an HTML document.
    #[must_use]
    pub fn render(self, site: Site) -> String {
        match self {
            Self::Home => home::render(site),
            Self::StyleHome => landing::style(site),
            Self::ComponentsHome => landing::components(site),
            Self::I18nHome => landing::i18n(site),
            Self::UseCases => use_cases::render(site),
            Self::Learn(page) => layout::article(site, self, learn::content(site, page)),
            Self::Reference(page) => layout::article(site, self, reference::content(site, page)),
            Self::Components(page) => layout::article(site, self, components::content(site, page)),
            Self::Style(page) => layout::article(site, self, style::content(site, page)),
            Self::I18n(page) => layout::article(site, self, i18n::content(site, page)),
        }
    }
}

impl I18nPage {
    fn slug(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Pages => "pages",
            Self::Negotiation => "negotiation",
        }
    }
}

impl Learn {
    fn slug(self) -> &'static str {
        match self {
            // Get started
            Self::QuickStart => "",
            Self::Why => "why",
            Self::DevServer => "dev-server",
            // Building the page
            Self::Elements => "elements",
            Self::ThePage => "page",
            Self::Bindings => "bindings",
            Self::Components => "components",
            Self::Styling => "styling",
            Self::DescribingThePage => "describing-the-page",
            // Browser state
            Self::Signals => "signals",
            Self::NestedSignals => "nested-signals",
            Self::Expressions => "expressions",
            Self::Statements => "statements",
            Self::Events => "events",
            Self::BindingInputs => "inputs",
            Self::ComputedSignals => "computed-signals",
            Self::PersistedSignals => "persisted-signals",
            Self::Lists => "lists",
            Self::ListItems => "list-items",
            // Talking to the server
            Self::Actions => "actions",
            Self::CallStatus => "call-status",
            Self::Optimistic => "optimistic-updates",
            Self::OverlappingCalls => "overlapping-calls",
            Self::Patches => "patches",
            Self::ListPatches => "list-patches",
            Self::Fragments => "fragments",
            Self::Streams => "streams",
            Self::Server => "server",
            Self::CallsInDepth => "calls",
            // Pages and routing
            Self::Routes => "routes",
            Self::Links => "links",
            Self::ChildPages => "child-pages",
            Self::SearchParams => "search-params",
            Self::ErrorPages => "error-pages",
            Self::BasePath => "base-path",
            Self::Pages => "pages",
            // Forms and requests
            Self::Forms => "forms",
            Self::FormValidation => "form-validation",
            Self::Requests => "requests",
            Self::Errors => "errors",
            Self::Uploads => "uploads",
            // Data and sign-in
            Self::Database => "database",
            Self::DatabaseWithoutSql => "database-without-sql",
            Self::CheckedSql => "checked-sql",
            Self::HostedDatabases => "hosted-databases",
            Self::DatabaseHosts => "database-hosts",
            Self::Api => "api",
            Self::Sessions => "sessions",
            Self::MembersOnly => "members-only",
            Self::SessionCookie => "session-cookie",
            Self::SessionStores => "session-stores",
            // Browser features
            Self::NodeRefs => "node-refs",
            Self::Timers => "timers",
            Self::Keyboard => "keyboard",
            Self::JavaScript => "javascript",
            Self::UncheckedValues => "unchecked-values",
            Self::StaticPages => "static-pages",
            Self::EscapeHatches => "escape-hatches",
            // Quality and operations
            Self::Security => "security",
            Self::Testing => "testing",
            Self::TestingPages => "testing-pages",
            Self::CompileErrors => "compile-errors",
            Self::Debugging => "debugging",
            Self::Upgrading => "upgrading",
            Self::Troubleshooting => "troubleshooting",
            // Deploy
            Self::Deploying => "deploying",
            Self::DeployContainer => "deploying/container",
            Self::DeployCloudflare => "deploying/cloudflare",
            Self::DeployVercel => "deploying/vercel",
        }
    }
}

impl Reference {
    fn slug(self) -> &'static str {
        match self {
            Self::Overview => "",
            Self::Page => "page",
            Self::El => "el",
            Self::Html => "html",
            Self::Signals => "signals",
            Self::Expr => "expr",
            Self::Stmt => "stmt",
            Self::Action => "action",
            Self::Form => "form",
            Self::Patches => "patches",
            Self::Route => "route",
            Self::NodeRef => "node-ref",
            Self::RouterExt => "router-ext",
        }
    }
}

impl Component {
    fn slug(self) -> &'static str {
        match self {
            Self::Installation => "installation",
            Self::Theming => "theming",
            Self::Button => "button",
            Self::TextField => "text-field",
            Self::Checkbox => "checkbox",
            Self::RadioGroup => "radio-group",
            Self::Select => "select",
            Self::Slider => "slider",
            Self::Toggle => "toggle",
            Self::Listbox => "listbox",
            Self::Combobox => "combobox",
            Self::Tabs => "tabs",
            Self::Accordion => "accordion",
            Self::Collapsible => "collapsible",
            Self::Card => "card",
            Self::Separator => "separator",
            Self::Navigation => "navigation",
            Self::DataTable => "data-table",
            Self::Chart => "chart",
            Self::Dialog => "dialog",
            Self::Menu => "menu",
            Self::Tooltip => "tooltip",
            Self::Popover => "popover",
            Self::Toast => "toast",
            Self::Alert => "alert",
            Self::Badge => "badge",
            Self::Progress => "progress",
            Self::Spinner => "spinner",
            Self::Avatar => "avatar",
            Self::Kbd => "kbd",
        }
    }
}

impl StylePage {
    fn slug(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Values => "values",
            Self::Conditions => "conditions",
            Self::Tokens => "tokens",
            Self::Global => "global",
            Self::Patterns => "patterns",
            Self::Animation => "animation",
            Self::Unchecked => "unchecked",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn every_page_has_its_own_path() {
        let paths: BTreeSet<String> = Doc::ALL.iter().map(|doc| doc.path()).collect();
        assert_eq!(paths.len(), Doc::ALL.len());
    }

    #[test]
    fn every_page_but_the_landing_pages_and_the_use_cases_is_in_a_sidebar() {
        for &doc in Doc::ALL {
            let listed = doc.groups().iter().any(|group| group.docs.contains(&doc));
            let alone = doc.is_landing() || doc == Doc::UseCases;
            assert_eq!(listed, !alone, "{doc:?}");
            assert_eq!(doc.has_sidebar(), !alone, "{doc:?}");
        }
    }

    #[test]
    fn every_library_has_a_landing_page_of_its_own() {
        for library in Library::ALL {
            assert_eq!(library.home().library(), library);
            assert!(library.home().is_landing());
        }
    }
}
