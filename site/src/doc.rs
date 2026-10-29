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
    // Pages and routing
    Routes,
    Links,
    ChildPages,
    SearchParams,
    ErrorPages,
    BasePath,
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
    // Quality and operations
    Security,
    Testing,
    TestingPages,
    CompileErrors,
    Debugging,
    Upgrading,
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
    // Get started
    Overview,
    Applying,
    // Values
    Values,
    Lengths,
    Colors,
    AnglesAndTimes,
    UrlsAndStrings,
    Keywords,
    // Conditions
    PseudoClasses,
    PseudoElements,
    Media,
    Containers,
    GroupAndPeer,
    Toggled,
    Conditions,
    // Tokens and themes
    Tokens,
    Themes,
    DarkMode,
    // Global styles
    Global,
    Fonts,
    // Layout
    Stacks,
    Grids,
    Patterns,
    // Animation
    Animation,
    Transitions,
    PageTransitions,
    ReducedMotion,
    // Opting out of type checks
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
        docs: &[
            Doc::Style(StylePage::Overview),
            Doc::Style(StylePage::Applying),
        ],
    },
    Group {
        title: L {
            en: "Values",
            ja: "値",
        },
        docs: &[
            Doc::Style(StylePage::Values),
            Doc::Style(StylePage::Lengths),
            Doc::Style(StylePage::Colors),
            Doc::Style(StylePage::AnglesAndTimes),
            Doc::Style(StylePage::UrlsAndStrings),
            Doc::Style(StylePage::Keywords),
        ],
    },
    Group {
        title: L {
            en: "Conditions",
            ja: "条件",
        },
        docs: &[
            Doc::Style(StylePage::PseudoClasses),
            Doc::Style(StylePage::PseudoElements),
            Doc::Style(StylePage::Media),
            Doc::Style(StylePage::Containers),
            Doc::Style(StylePage::GroupAndPeer),
            Doc::Style(StylePage::Toggled),
            Doc::Style(StylePage::Conditions),
        ],
    },
    Group {
        title: L {
            en: "Tokens and themes",
            ja: "トークンとテーマ",
        },
        docs: &[
            Doc::Style(StylePage::Tokens),
            Doc::Style(StylePage::Themes),
            Doc::Style(StylePage::DarkMode),
        ],
    },
    Group {
        title: L {
            en: "Global styles",
            ja: "グローバルスタイル",
        },
        docs: &[Doc::Style(StylePage::Global), Doc::Style(StylePage::Fonts)],
    },
    Group {
        title: L {
            en: "Layout",
            ja: "レイアウト",
        },
        docs: &[
            Doc::Style(StylePage::Stacks),
            Doc::Style(StylePage::Grids),
            Doc::Style(StylePage::Patterns),
        ],
    },
    Group {
        title: L {
            en: "Animation",
            ja: "アニメーション",
        },
        docs: &[
            Doc::Style(StylePage::Animation),
            Doc::Style(StylePage::Transitions),
            Doc::Style(StylePage::PageTransitions),
            Doc::Style(StylePage::ReducedMotion),
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
        // Pages and routing
        Self::Learn(Learn::Routes),
        Self::Learn(Learn::Links),
        Self::Learn(Learn::ChildPages),
        Self::Learn(Learn::SearchParams),
        Self::Learn(Learn::ErrorPages),
        Self::Learn(Learn::BasePath),
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
        // Quality and operations
        Self::Learn(Learn::Security),
        Self::Learn(Learn::Testing),
        Self::Learn(Learn::TestingPages),
        Self::Learn(Learn::CompileErrors),
        Self::Learn(Learn::Debugging),
        Self::Learn(Learn::Upgrading),
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
        // Style: Get started
        Self::Style(StylePage::Overview),
        Self::Style(StylePage::Applying),
        // Style: Values
        Self::Style(StylePage::Values),
        Self::Style(StylePage::Lengths),
        Self::Style(StylePage::Colors),
        Self::Style(StylePage::AnglesAndTimes),
        Self::Style(StylePage::UrlsAndStrings),
        Self::Style(StylePage::Keywords),
        // Style: Conditions
        Self::Style(StylePage::PseudoClasses),
        Self::Style(StylePage::PseudoElements),
        Self::Style(StylePage::Media),
        Self::Style(StylePage::Containers),
        Self::Style(StylePage::GroupAndPeer),
        Self::Style(StylePage::Toggled),
        Self::Style(StylePage::Conditions),
        // Style: Tokens and themes
        Self::Style(StylePage::Tokens),
        Self::Style(StylePage::Themes),
        Self::Style(StylePage::DarkMode),
        // Style: Global styles
        Self::Style(StylePage::Global),
        Self::Style(StylePage::Fonts),
        // Style: Layout
        Self::Style(StylePage::Stacks),
        Self::Style(StylePage::Grids),
        Self::Style(StylePage::Patterns),
        // Style: Animation
        Self::Style(StylePage::Animation),
        Self::Style(StylePage::Transitions),
        Self::Style(StylePage::PageTransitions),
        Self::Style(StylePage::ReducedMotion),
        // Style: Opting out of type checks
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
                // Browser state
                Learn::Signals => ("Signals", "シグナル"),
                Learn::NestedSignals => ("Signals inside signals", "入れ子のシグナル"),
                Learn::Expressions => ("Expressions", "式"),
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
                // Pages and routing
                Learn::Routes => ("Routes", "ルート"),
                Learn::Links => ("Links and redirects", "リンクとリダイレクト"),
                Learn::ChildPages => ("Pages under a page", "子のページ"),
                Learn::SearchParams => ("Search parameters", "検索パラメータ"),
                Learn::ErrorPages => ("Error pages", "エラーページ"),
                Learn::BasePath => ("Serving under a path prefix", "パスの接頭辞の下で配信する"),
                // Forms and requests
                Learn::Forms => ("Forms", "フォーム"),
                Learn::FormValidation => ("Validating a form", "フォームの検証"),
                Learn::Requests => ("Extractors and headers", "エクストラクタとヘッダー"),
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
                // Quality and operations
                Learn::Security => ("Security", "セキュリティ"),
                Learn::Testing => ("Testing handlers", "ハンドラのテスト"),
                Learn::TestingPages => (
                    "Testing a page without a browser",
                    "ブラウザなしでページをテストする",
                ),
                Learn::CompileErrors => ("Reading a compile error", "コンパイルエラーの読み方"),
                Learn::Debugging => ("Debugging a running page", "動いているページを調べる"),
                Learn::Upgrading => ("Upgrading", "新しいリリースに上げる"),
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
                // Get started
                StylePage::Overview => ("Overview", "概要"),
                StylePage::Applying => ("Applying a style", "スタイルを付ける"),
                // Values
                StylePage::Values => ("Properties", "プロパティ"),
                StylePage::Lengths => ("Lengths and units", "長さと単位"),
                StylePage::Colors => ("Colors", "色"),
                StylePage::AnglesAndTimes => ("Angles and times", "角度と時間"),
                StylePage::UrlsAndStrings => ("URLs and strings", "URL と文字列"),
                StylePage::Keywords => ("CSS-wide keywords", "CSS 全体のキーワード"),
                // Conditions
                StylePage::PseudoClasses => ("Pseudo-classes", "擬似クラス"),
                StylePage::PseudoElements => ("Pseudo-elements", "擬似要素"),
                StylePage::Media => ("Media queries", "メディアクエリ"),
                StylePage::Containers => ("Container queries", "コンテナクエリ"),
                StylePage::GroupAndPeer => ("Group and peer", "グループとピア"),
                StylePage::Toggled => (
                    "Styles switched by a signal",
                    "シグナルで切り替えるスタイル",
                ),
                StylePage::Conditions => ("Conditions", "条件"),
                // Tokens and themes
                StylePage::Tokens => ("Tokens", "トークン"),
                StylePage::Themes => ("Themes", "テーマ"),
                StylePage::DarkMode => ("Dark mode", "ダークモード"),
                // Global styles
                StylePage::Global => ("Rules by tag", "タグ単位のルール"),
                StylePage::Fonts => ("Fonts", "フォント"),
                // Layout
                StylePage::Stacks => ("Stacks", "スタック"),
                StylePage::Grids => ("Grids", "グリッド"),
                StylePage::Patterns => ("Other patterns", "そのほかのパターン"),
                // Animation
                StylePage::Animation => ("Keyframes", "キーフレーム"),
                StylePage::Transitions => ("Transitions", "トランジション"),
                StylePage::PageTransitions => ("Page transitions", "ページ遷移"),
                StylePage::ReducedMotion => ("Reduced motion", "動きを減らす設定"),
                // Opting out of type checks
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
                Learn::DevServer => L {
                    en: "`cargo hypersig dev` builds and restarts the server after each change, and the open tab reloads with the signals the visitor changed. `cargo hypersig new` writes a project to start from.",
                    ja: "`cargo hypersig dev` は変更のたびにサーバーをビルドして起動し直し、開いているタブは訪問者が変えたシグナルを保ったまま読み込み直されます。`cargo hypersig new` は最初のプロジェクトを書き出します。",
                },
                // Building the page
                Learn::Elements => L {
                    en: "A page is a tree of `El` values built with Rust functions and rendered to HTML on the server. The tags, the attribute names, and the ids that attributes point at are checked when the program compiles.",
                    ja: "ページは Rust の関数で組み立てた `El` の値の木で、サーバーが HTML に描画します。タグ、属性の名前、属性が指す id は、プログラムのコンパイル時に検査されます。",
                },
                Learn::ThePage => L {
                    en: "`Page` wraps the tree in the HTML document a handler sends, with the runtime, the initial signals, and the head that browsers, search engines, and link previews read.",
                    ja: "`Page` は木を包み、ハンドラが送る HTML 文書にします。文書には、ランタイム、シグナルの初期値、そしてブラウザ、検索エンジン、リンクのプレビューが読む head が入ります。",
                },
                Learn::Bindings => L {
                    en: "A binding keeps an element's text, attributes, visibility, or classes equal to an expression over the signals, in the first HTML and in the browser after each change.",
                    ja: "束縛は、要素のテキスト、属性、表示と非表示、クラスをシグナルの式の値に合わせ続けます。最初の HTML でも、変更のたびのブラウザでも同じです。",
                },
                Learn::Components => L {
                    en: "A component is a Rust function that returns an element, and a part is a component with its own signals and actions that several pages embed.",
                    ja: "コンポーネントは要素を返す Rust の関数です。部品は自分のシグナルとアクションを持つコンポーネントで、複数のページに埋め込めます。",
                },
                Learn::Styling => L {
                    en: "Styles are Rust values too. A misspelled property or keyword, or a value of the wrong type, is a compile error.",
                    ja: "スタイルも Rust の値です。プロパティやキーワードの打ち間違いや、型の違う値はコンパイルエラーになります。",
                },
                // Browser state
                Learn::Signals => L {
                    en: "The page's state is one Rust struct, and a signal is a typed reference to one of its fields.",
                    ja: "ページの状態は Rust の構造体 1 つで表し、シグナルはそのフィールド 1 つを指す型付きの参照です。",
                },
                Learn::NestedSignals => L {
                    en: "A field can hold a struct, an `Option`, or the state of a component, and `at` reaches the fields inside it.",
                    ja: "フィールドには構造体、`Option`、部品の状態を持たせられ、その中のフィールドへは `at` でたどります。",
                },
                Learn::Expressions => L {
                    en: "An expression computes a value from the signals. The browser evaluates it, and the server evaluates it for the first HTML.",
                    ja: "式はシグナルから値を計算します。ブラウザが式を評価し、最初の HTML のためにはサーバーも評価します。",
                },
                Learn::Statements => L {
                    en: "A statement changes the signals in the browser, without a request. Statements run in order and branch on expressions.",
                    ja: "文は、リクエストを送らずにブラウザでシグナルを変更します。文は順に実行され、式で分岐します。",
                },
                Learn::Events => L {
                    en: "Each event method writes a statement into an attribute of the element, and the runtime runs it on that DOM event or key.",
                    ja: "イベントのメソッドは文を要素の属性に書き込み、ランタイムはその DOM のイベントやキーで文を実行します。",
                },
                Learn::BindingInputs => L {
                    en: "`bind`, `bind_radio`, and `bind_select` keep a form control and a signal equal in both directions.",
                    ja: "`bind`、`bind_radio`、`bind_select` は、フォームの入力欄とシグナルを双方向に一致させます。",
                },
                Learn::ComputedSignals => L {
                    en: "A computed signal is a field the browser keeps equal to an expression. Writing one is a compile error.",
                    ja: "計算シグナルは、ブラウザが式の値に保ち続けるフィールドです。書き込もうとするとコンパイルエラーになります。",
                },
                Learn::PersistedSignals => L {
                    en: "`Page::persist` keeps a signal in `localStorage`, so the next page load starts from the visitor's last value.",
                    ja: "`Page::persist` はシグナルを `localStorage` に保存し、次にページを読み込むときは訪問者が最後に選んだ値から始めます。",
                },
                Learn::Lists => L {
                    en: "A `Vec` field is a list. `each` renders an element per item in the browser, and list statements change it without a request.",
                    ja: "`Vec` のフィールドはリストです。`each` はブラウザで項目ごとに要素を描画し、リストの文はリクエストを送らずにリストを変更します。",
                },
                Learn::ListItems => L {
                    en: "A field of a list item is a signal whose path starts at the item. The server cannot patch it, and state the browser keeps per item belongs at the page's root.",
                    ja: "リスト項目のフィールドは、パスが項目から始まるシグナルです。サーバーはこれを差分で変えられず、ブラウザが項目ごとに持つ状態はページの構造体の直下に置きます。",
                },
                // Talking to the server
                Learn::Actions => L {
                    en: "An action is a server endpoint a page calls with typed arguments. This page covers declaring one, calling it, and the request and the answer that go over the network.",
                    ja: "アクションは、ページが型付きの引数で呼び出すサーバーのエンドポイントです。宣言のしかた、呼び出し方、ネットワークを流れるリクエストと応答を扱います。",
                },
                Learn::CallStatus => L {
                    en: "A call can set a signal while it waits, write the error text when it fails, and run a statement on failure or success.",
                    ja: "呼び出しは応答を待つ間にシグナルを設定し、失敗したときにエラーの文言を書き込み、失敗や成功のときに文を実行できます。",
                },
                Learn::Optimistic => L {
                    en: "A statement before a call changes the page at once, and the server's answer then sets the value it stored.",
                    ja: "呼び出しの前に置いた文はすぐにページを変え、そのあとサーバーの応答が保存した値を設定します。",
                },
                Learn::OverlappingCalls => L {
                    en: "What happens when a call is sent while an earlier call to the same action still waits, and how to keep the newest answer, send one at a time, or give up after a time limit.",
                    ja: "同じアクションへの前の呼び出しが応答を待っている間に次の呼び出しを送ったときの動きと、最新の応答を採る、一度に 1 つだけ送る、一定の時間で打ち切る方法を扱います。",
                },
                Learn::Patches => L {
                    en: "A handler answers with a batch of patches: new values for signals, list changes, fragments, and URLs. The browser receives them as Server-Sent Events and applies them in order.",
                    ja: "ハンドラは差分のまとまりで応答します。中身はシグナルの新しい値、リストの変更、フラグメント、URL です。ブラウザはそれを Server-Sent Events として受け取り、順に適用します。",
                },
                Learn::ListPatches => L {
                    en: "Patches that add, replace, or remove items of a list without sending the rest of it, and a feed that loads more items as the visitor scrolls.",
                    ja: "リストの残りを送らずに項目を加える、置き換える、取り除く差分と、スクロールに合わせて項目を読み足すフィードを扱います。",
                },
                Learn::Fragments => L {
                    en: "A fragment is a part of the page the server renders again and sends as HTML, which the browser merges into the element with the fragment's id.",
                    ja: "フラグメントは、サーバーが描画し直して HTML として送るページの一部です。ブラウザはそれを、フラグメントの id を持つ要素に取り込みます。",
                },
                Learn::Streams => L {
                    en: "A stream keeps its response open and sends patches as changes happen on the server, and opens again when the connection drops.",
                    ja: "ストリームは応答を開いたままにして、サーバーで変化が起きるたびに差分を送り、接続が切れたら開き直します。",
                },
                // Pages and routing
                Learn::Routes => L {
                    en: "A route is a page the server renders at a typed path. This page covers declaring one in `routes!`, the types a path parameter can hold, and the handler that serves it.",
                    ja: "ルートは、型の付いたパスでサーバーが描画するページです。`routes!` での宣言、パスのパラメータに使える型、ルートを配信するハンドラを説明します。",
                },
                Learn::Links => L {
                    en: "Links, redirects, and navigation from an action or a statement are built from the route, so a URL with a parameter of the wrong type or in the wrong number does not compile.",
                    ja: "リンク、リダイレクト、アクションや文からのページ移動はルートから作るので、パラメータの型や数が違う URL はコンパイルできません。",
                },
                Learn::ChildPages => L {
                    en: "A route declared with `in PARENT` continues its parent's path and parameters, and the compiler checks that it does.",
                    ja: "`in PARENT` を付けて宣言したルートは親のパスとパラメータを引き継ぎ、コンパイラがそれを検査します。",
                },
                Learn::SearchParams => L {
                    en: "A route's second type parameter types its query string, and a statement or an action can change the URL in the address bar without loading a page.",
                    ja: "ルートの 2 つ目の型パラメータはクエリ文字列の型です。文やアクションは、ページを読み込まずにアドレスバーの URL を変えられます。",
                },
                Learn::ErrorPages => L {
                    en: "What a page handler answers when it cannot build its page, and the one page `routes!` shows for every request that reaches no page.",
                    ja: "ページを組み立てられなかったハンドラが返すものと、どのページにも届かなかったリクエストに `routes!` が出すページを説明します。",
                },
                Learn::BasePath => L {
                    en: "An application served under a prefix such as `/app` names it once as a `Base`, and every link, URL, and call of the block starts with it.",
                    ja: "`/app` のような接頭辞の下で配信するアプリは、接頭辞を `Base` として 1 回だけ宣言します。ブロックのリンク、URL、呼び出しはすべてその接頭辞で始まります。",
                },
                // Forms and requests
                Learn::Forms => L {
                    en: "A form on the page is a `Form<T>` field, which holds the values, the values they started from, and what the visitor has done to them. This page binds its controls, shows its messages, and submits it.",
                    ja: "ページのフォームは `Form<T>` のフィールドで、値、開始時の値、訪問者がフォームでした操作を持ちます。このページではコントロールの束縛、メッセージの表示、送信を説明します。",
                },
                Learn::FormValidation => L {
                    en: "A form's rules are written once in Rust. The browser checks them as the visitor types, and the handler checks the same rules before it trusts the values.",
                    ja: "フォームのルールは Rust で 1 回だけ書きます。ブラウザは入力に合わせてそのルールを検査し、ハンドラは値を使う前に同じルールで検査します。",
                },
                Learn::Requests => L {
                    en: "A handler reads the request through an axum extractor and sets cookies by returning headers beside its answer. The router refuses some requests before any handler runs.",
                    ja: "ハンドラは axum のエクストラクタでリクエストを読み、応答と一緒にヘッダーを返して Cookie を設定します。ハンドラが動く前にルーターが拒否するリクエストもあります。",
                },
                Learn::Errors => L {
                    en: "A handler fails by returning an error that has a response, and the page shows that response's status and text. What the visitor should not see goes to the server's log through `tracing`.",
                    ja: "ハンドラは応答を持つエラーを返して失敗し、ページはその応答のステータスと文言を表示します。訪問者に見せないことは、`tracing` でサーバーのログに書きます。",
                },
                Learn::Uploads => L {
                    en: "An upload is an action that also receives the files picked in a file input. Its handler reads them into memory, or one chunk at a time for large files.",
                    ja: "アップロードは、ファイル入力で選んだファイルも受け取るアクションです。ハンドラはファイルをメモリに読み込むか、大きなファイルなら少しずつ読みます。",
                },
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
                Learn::NodeRefs => L {
                    en: "A node ref names one element, so that a statement can focus it, select its text, scroll it into view, or open it as a popover or a dialog. The page covers declaring one, its statements, and the mistakes that rendering catches.",
                    ja: "ノード参照は要素を 1 つ指す名前で、文はこれを使って要素にフォーカスしたり、テキストを選択したり、見える位置までスクロールしたり、ポップオーバーやダイアログとして開いたりします。宣言の仕方、使える文、描画で見つかるミスを説明します。",
                },
                Learn::Timers => L {
                    en: "Statements that run on a timer, when an element is added to the document, or when a signal changes, and the browser's clock and random numbers that they can read.",
                    ja: "タイマーで、要素が文書に追加されたときに、またはシグナルが変わったときに実行される文と、それらの文が読めるブラウザの時計と乱数を説明します。",
                },
                Learn::Keyboard => L {
                    en: "Focus that moves between items with the arrow keys, the active option of a listbox while focus stays in a text field, and shortcuts that move focus through node refs.",
                    ja: "矢印キーで項目間を移るフォーカス、フォーカスをテキスト欄に置いたまま動かすリストボックスのアクティブな選択肢、ノード参照を通してフォーカスを動かすショートカットを説明します。",
                },
                Learn::JavaScript => L {
                    en: "A map, a chart, or a rich text editor runs in the browser as a JavaScript library. Wrapped in a custom element, it takes values from the page's signals and hands its events back as typed values.",
                    ja: "地図、グラフ、リッチテキストエディタは、ブラウザで JavaScript のライブラリとして動きます。カスタム要素で包むと、ページのシグナルから値を受け取り、イベントを型付きの値としてページに返せます。",
                },
                Learn::UncheckedValues => L {
                    en: "Some values pass the type check while their content goes unchecked: the host of a URL, the value of an attribute or a CSS property, and the path of a file the page loads. For each, what is checked and what the browser does with a wrong value.",
                    ja: "型の検査は通っても、中身は検査されない値があります。URL のホスト、属性や CSS プロパティの値、ページが読み込むファイルのパスです。それぞれについて、何が検査され、間違った値をブラウザがどう扱うかを説明します。",
                },
                Learn::StaticPages => L {
                    en: "A page that calls no action can be written to a file and served by any static host, with the runtime next to it. The page also covers rendering under a path prefix and what a static host leaves out.",
                    ja: "アクションを呼ばないページは、ファイルに書き出して、ランタイムと一緒に静的なホストで配信できます。パスの接頭辞の下での描画と、静的なホストで抜け落ちるものも説明します。",
                },
                // Quality and operations
                Learn::Security => L {
                    en: "What hypersig checks so that a page and its actions resist what a visitor can send, why data from the browser cannot run code on the server, and what is left to the application.",
                    ja: "訪問者が送るものからページとアクションを守るために hypersig が検査すること、ブラウザから届くデータがサーバーのコードを動かせない理由、アプリに任せることをまとめます。",
                },
                Learn::Testing => L {
                    en: "Tests that call a page's, an action's, and a stream's handler as functions and read the signals by their types, and tests that send a request through the router to check what the router adds.",
                    ja: "ページ、アクション、ストリームのハンドラを関数として呼び、シグナルの値を型で読むテストと、ルーターにリクエストを送って、ルーターが加えるものを確かめるテストを扱います。",
                },
                Learn::TestingPages => L {
                    en: "A test that clicks and types on a page without a browser, reads the signals and the elements afterwards, and checks the calls and the navigation the events asked for. The page also lists what only a browser test reaches.",
                    ja: "ブラウザなしでページをクリックして入力し、そのあとのシグナルと要素を読み、イベントが求めた呼び出しと移動を確かめるテストを扱います。ブラウザのテストでしか確かめられないことも挙げます。",
                },
                Learn::CompileErrors => L {
                    en: "How to read the compile errors that hypersig's traits and constants produce: which line names the mistake, which lines to skip, and where to look up a message this page does not list.",
                    ja: "hypersig のトレイトと定数が出すコンパイルエラーの読み方を扱います。どの行が間違いを指し、どの行を読み飛ばしてよいか、このページにないメッセージをどこで調べるかを説明します。",
                },
                Learn::Debugging => L {
                    en: "Where a running page reports a mistake the types do not catch: a panic on the server while the page is built, a line in the browser's console, and a failed call that the page can show.",
                    ja: "型で防げない間違いを、動いているページがどこに報告するかを扱います。ページを組み立てるときのサーバーでのパニック、ブラウザのコンソールの行、ページに表示できる失敗した呼び出しの 3 つです。",
                },
                Learn::Upgrading => L {
                    en: "Each change of a public signature from 0.5 to 0.13, shown as the code after it, with the line it replaces.",
                    ja: "0.5 から 0.13 までに公開のシグネチャが変わった箇所を、変更後のコードと、それが置き換える行で示します。",
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
                // Get started
                StylePage::Overview => L {
                    en: "`hypersig::style` is typed atomic CSS. A misspelled property name or keyword, or a value of the wrong type, is a compile error. Each declaration becomes one class.",
                    ja: "`hypersig::style` は型の付いた atomic CSS です。プロパティ名やキーワードの綴りを間違えたり、型の違う値を渡したりするとコンパイルエラーになります。宣言は 1 つごとに 1 つのクラスになります。",
                },
                StylePage::Applying => Doc::Style(StylePage::Overview).leads(),
                // Values
                StylePage::Values => L {
                    en: "Each property method takes its own value type, which has the property's keywords and accepts values in the units the property allows.",
                    ja: "プロパティのメソッドはそれぞれ専用の値の型を受け取ります。その型はプロパティのキーワードを持ち、プロパティで使える単位の値を受け付けます。",
                },
                StylePage::Lengths
                | StylePage::Colors
                | StylePage::AnglesAndTimes
                | StylePage::UrlsAndStrings
                | StylePage::Keywords => Doc::Style(StylePage::Values).leads(),
                // Conditions
                StylePage::PseudoClasses
                | StylePage::PseudoElements
                | StylePage::Media
                | StylePage::Containers
                | StylePage::GroupAndPeer
                | StylePage::Toggled => Doc::Style(StylePage::Conditions).leads(),
                StylePage::Conditions => L {
                    en: "`Style::on` applies part of a style only under a condition. The condition can be a pseudo-class, a media feature, a container width, or the state of a marked ancestor or sibling.",
                    ja: "`Style::on` はスタイルの一部を条件の下でだけ適用します。条件には擬似クラス、メディア特性、コンテナの幅、印を付けた祖先や兄弟の状態を使えます。",
                },
                // Tokens and themes
                StylePage::Tokens => L {
                    en: "A token is a named value written as a CSS custom property. A theme sets its value for the whole page, and a style sets it for one element.",
                    ja: "トークンは CSS のカスタムプロパティとして書かれる名前付きの値です。テーマはページ全体に、スタイルは 1 つの要素にトークンの値を設定します。",
                },
                StylePage::Themes | StylePage::DarkMode => Doc::Style(StylePage::Tokens).leads(),
                // Global styles
                StylePage::Global => L {
                    en: "A `Global` holds rules that select elements by tag, a small reset, and the fonts the page loads.",
                    ja: "`Global` には、タグで要素を選ぶルール、小さなリセット、ページが読み込むフォントが入ります。",
                },
                StylePage::Fonts => Doc::Style(StylePage::Global).leads(),
                // Layout
                StylePage::Stacks => L {
                    en: "A stack puts an element's children in a row or a column with a gap between them. Each stack is a function that returns a `Style` of flexbox declarations.",
                    ja: "スタックは要素の子を間隔を空けて横 1 行か縦 1 列に並べます。どのスタックも flexbox の宣言からなる `Style` を返す関数です。",
                },
                StylePage::Grids => L {
                    en: "`grid` lays children out in columns of equal width, and `grid_item` makes one child span several of them.",
                    ja: "`grid` は子を等しい幅の列に並べ、`grid_item` は 1 つの子を複数の列にまたがらせます。",
                },
                StylePage::Patterns => L {
                    en: "The rest of [`hypersig::style::pattern`](api:hypersig/style/pattern/index.html) centers content, gives a box a fixed shape, makes a whole card one link, and hides text from sight but not from screen readers.",
                    ja: "[`hypersig::style::pattern`](api:hypersig/style/pattern/index.html) の残りの関数は、内容を中央に置き、ボックスの形を決め、カード全体を 1 つのリンクにし、テキストを画面からは隠してスクリーンリーダーには読ませます。",
                },
                // Animation
                StylePage::Animation => L {
                    en: "Keyframes are Rust values too. When an element uses keyframes, their rule is written to the page along with the element.",
                    ja: "キーフレームも Rust の値です。要素がキーフレームを使うと、そのキーフレームのルールも要素と一緒にページに書かれます。",
                },
                StylePage::Transitions => L {
                    en: "A transition animates a property from its old value to its new one when a pseudo-class or a signal changes it.",
                    ja: "トランジションは、擬似クラスやシグナルがプロパティの値を変えたときに、前の値から新しい値へアニメーションさせます。",
                },
                StylePage::PageTransitions => L {
                    en: "The browser's view transitions animate the move from one page to the next, and `Stmt::view_transition` animates a change within a page.",
                    ja: "ブラウザの View Transitions は、ページから次のページへの移動をアニメーションさせます。ページの中の変化は `Stmt::view_transition` でアニメーションさせます。",
                },
                StylePage::ReducedMotion => L {
                    en: "`Media::ReducedMotion` applies a style for a visitor who asks the operating system for less motion, so an animation can stop, slow down, or become instant for them.",
                    ja: "`Media::ReducedMotion` は、OS で動きを減らすよう設定した訪問者にだけスタイルを適用します。その訪問者に対してはアニメーションを止めたり、遅くしたり、すぐに終わらせたりできます。",
                },
                // Opting out of type checks
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
            // Pages and routing
            Self::Routes => "routes",
            Self::Links => "links",
            Self::ChildPages => "child-pages",
            Self::SearchParams => "search-params",
            Self::ErrorPages => "error-pages",
            Self::BasePath => "base-path",
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
            // Quality and operations
            Self::Security => "security",
            Self::Testing => "testing",
            Self::TestingPages => "testing-pages",
            Self::CompileErrors => "compile-errors",
            Self::Debugging => "debugging",
            Self::Upgrading => "upgrading",
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
            // Get started
            Self::Overview => "overview",
            Self::Applying => "applying",
            // Values
            Self::Values => "values",
            Self::Lengths => "lengths",
            Self::Colors => "colors",
            Self::AnglesAndTimes => "angles-and-times",
            Self::UrlsAndStrings => "urls-and-strings",
            Self::Keywords => "keywords",
            // Conditions
            Self::PseudoClasses => "pseudo-classes",
            Self::PseudoElements => "pseudo-elements",
            Self::Media => "media",
            Self::Containers => "containers",
            Self::GroupAndPeer => "group-and-peer",
            Self::Toggled => "toggled",
            Self::Conditions => "conditions",
            // Tokens and themes
            Self::Tokens => "tokens",
            Self::Themes => "themes",
            Self::DarkMode => "dark-mode",
            // Global styles
            Self::Global => "global",
            Self::Fonts => "fonts",
            // Layout
            Self::Stacks => "stacks",
            Self::Grids => "grids",
            Self::Patterns => "patterns",
            // Animation
            Self::Animation => "animation",
            Self::Transitions => "transitions",
            Self::PageTransitions => "page-transitions",
            Self::ReducedMotion => "reduced-motion",
            // Opting out of type checks
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
