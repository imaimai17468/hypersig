//! The pages of the site, their paths, and the order the sidebar lists them in.

use crate::lang::L;
use crate::{Site, components, home, i18n, landing, layout, learn, reference, style};

/// One page of the site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Doc {
    /// The landing page of hypersig, the framework.
    Home,
    /// The landing page of the Style docs.
    StyleHome,
    /// The landing page of the Components docs.
    ComponentsHome,
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
    Why,
    QuickStart,
    DescribingThePage,
    Signals,
    Expressions,
    Lists,
    Server,
    Forms,
    Requests,
    Pages,
    Styling,
    EscapeHatches,
    JavaScript,
    Security,
    CallsInDepth,
    RoutesInDepth,
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
    Listbox,
    Combobox,
    Tabs,
    Accordion,
    DataTable,
    Chart,
    Card,
    Separator,
    Navigation,
    Dialog,
    Menu,
    Tooltip,
    Toast,
    Alert,
    Badge,
    Progress,
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

    /// What the library is, as the description of its landing page that
    /// search results and link previews show.
    fn descriptions(self) -> L<&'static str> {
        match self {
            Self::Framework => L {
                en: "hypersig is a server-driven hypermedia library for Rust. Signal names, expressions, actions, and links are built from Rust types, so a mistake in one is a compile error.",
                ja: "hypersig は Rust のサーバー駆動のハイパーメディアライブラリです。シグナル名、式、アクション、リンクを Rust の型から作るので、その誤りはコンパイルエラーになります。",
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
        docs: &[Doc::Learn(Learn::QuickStart), Doc::Learn(Learn::Why)],
    },
    Group {
        title: L {
            en: "Learn hypersig",
            ja: "hypersig を学ぶ",
        },
        docs: &[
            Doc::Learn(Learn::DescribingThePage),
            Doc::Learn(Learn::Signals),
            Doc::Learn(Learn::Expressions),
            Doc::Learn(Learn::Lists),
            Doc::Learn(Learn::Server),
            Doc::Learn(Learn::Forms),
            Doc::Learn(Learn::Requests),
            Doc::Learn(Learn::Pages),
            Doc::Learn(Learn::Styling),
            Doc::Learn(Learn::EscapeHatches),
            Doc::Learn(Learn::JavaScript),
            Doc::Learn(Learn::Security),
        ],
    },
    Group {
        title: L {
            en: "In depth",
            ja: "詳しく知る",
        },
        docs: &[
            Doc::Learn(Learn::CallsInDepth),
            Doc::Learn(Learn::RoutesInDepth),
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
            Doc::Components(Component::Toast),
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
        Self::Learn(Learn::QuickStart),
        Self::Learn(Learn::Why),
        Self::Learn(Learn::DescribingThePage),
        Self::Learn(Learn::Signals),
        Self::Learn(Learn::Expressions),
        Self::Learn(Learn::Lists),
        Self::Learn(Learn::Server),
        Self::Learn(Learn::Forms),
        Self::Learn(Learn::Requests),
        Self::Learn(Learn::Pages),
        Self::Learn(Learn::Styling),
        Self::Learn(Learn::EscapeHatches),
        Self::Learn(Learn::JavaScript),
        Self::Learn(Learn::Security),
        Self::Learn(Learn::CallsInDepth),
        Self::Learn(Learn::RoutesInDepth),
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
        Self::Components(Component::Listbox),
        Self::Components(Component::Combobox),
        Self::Components(Component::Tabs),
        Self::Components(Component::Accordion),
        Self::Components(Component::Card),
        Self::Components(Component::Separator),
        Self::Components(Component::Navigation),
        Self::Components(Component::DataTable),
        Self::Components(Component::Chart),
        Self::Components(Component::Dialog),
        Self::Components(Component::Menu),
        Self::Components(Component::Tooltip),
        Self::Components(Component::Toast),
        Self::Components(Component::Alert),
        Self::Components(Component::Badge),
        Self::Components(Component::Progress),
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
            Self::I18n(page) => match page {
                I18nPage::Overview => ("Overview", "概要"),
                I18nPage::Pages => ("Pages in each language", "言語ごとのページ"),
                I18nPage::Negotiation => ("Choosing the language", "言語の選び方"),
            },
            Self::ComponentsHome => ("hypersig components", "hypersig components"),
            Self::Learn(page) => match page {
                Learn::Why => ("Why hypersig", "hypersig を選ぶ理由"),
                Learn::QuickStart => ("Quick start", "クイックスタート"),
                Learn::DescribingThePage => ("Building the page", "ページを組み立てる"),
                Learn::Signals => ("Signals", "シグナル"),
                Learn::Expressions => ("Expressions and statements", "式と文"),
                Learn::Lists => ("Lists", "リスト"),
                Learn::Server => ("Talking to the server", "サーバーとのやり取り"),
                Learn::Forms => ("Forms", "フォーム"),
                Learn::Requests => (
                    "Requests, cookies, and errors",
                    "リクエスト、Cookie、エラー",
                ),
                Learn::Pages => ("Pages and links", "ページとリンク"),
                Learn::Styling => ("Styling", "スタイル"),
                Learn::EscapeHatches => (
                    "When bindings and statements are not enough",
                    "束縛と文で足りないとき",
                ),
                Learn::JavaScript => ("JavaScript libraries", "JavaScript のライブラリ"),
                Learn::Deploying => ("Choosing a host", "デプロイ先の選び方"),
                Learn::DeployContainer => ("In a container", "コンテナ"),
                Learn::DeployCloudflare => ("Cloudflare Workers", "Cloudflare Workers"),
                Learn::DeployVercel => ("Vercel", "Vercel"),
                Learn::Security => ("Security", "セキュリティ"),
                Learn::CallsInDepth => ("Calls in depth", "呼び出しの詳細"),
                Learn::RoutesInDepth => ("Routes in depth", "ルートの詳細"),
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
                Component::Listbox => ("Listbox", "リストボックス"),
                Component::Combobox => ("Combobox", "コンボボックス"),
                Component::Tabs => ("Tabs", "タブ"),
                Component::Accordion => ("Accordion", "アコーディオン"),
                Component::Card => ("Card", "カード"),
                Component::Separator => ("Separator", "区切り線"),
                Component::Navigation => ("Navigation", "ナビゲーション"),
                Component::DataTable => ("Data table", "データテーブル"),
                Component::Chart => ("Chart", "チャート"),
                Component::Dialog => ("Dialog", "ダイアログ"),
                Component::Menu => ("Menu", "メニュー"),
                Component::Tooltip => ("Tooltip", "ツールチップ"),
                Component::Toast => ("Toast", "トースト"),
                Component::Alert => ("Alert", "アラート"),
                Component::Badge => ("Badge", "バッジ"),
                Component::Progress => ("Progress and skeleton", "プログレスとスケルトン"),
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
            Self::Learn(page) => match page {
                Learn::QuickStart => L {
                    en: "Build a counter that counts in the browser and resets on the server, in one Rust file.",
                    ja: "ブラウザで数を数え、サーバーで 0 に戻すカウンターを、Rust のファイル 1 つで作ります。",
                },
                Learn::DescribingThePage => L {
                    en: "A page is a tree of `El` values built with Rust functions and rendered to HTML on the server.",
                    ja: "ページは Rust の関数で組み立てた `El` の値の木で、サーバーが HTML に描画します。",
                },
                Learn::Forms => L {
                    en: "A form's rules are written once in Rust. The browser checks them as the visitor types, and the handler checks the same rules before it trusts the values.",
                    ja: "フォームのルールは Rust で 1 回だけ書きます。ブラウザは入力に合わせてそのルールを検査し、ハンドラは値を使う前に同じルールで検査します。",
                },
                Learn::Requests => L {
                    en: "A handler reads the request through an axum extractor, sets cookies by returning headers beside its answer, and fails with `?`.",
                    ja: "ハンドラは axum の extractor でリクエストを読み、応答と一緒にヘッダーを返して Cookie を設定し、`?` で失敗を返します。",
                },
                Learn::Server => L {
                    en: "The browser calls an action, and the server answers with patches: new values for signals, or new HTML for a part of the page.",
                    ja: "ブラウザがアクションを呼び出すと、サーバーは差分を返します。差分はシグナルの新しい値か、ページの一部の新しい HTML です。",
                },
                Learn::Pages => L {
                    en: "A route is a page the server renders at a path. The router serves it and the links on other pages point to it from the same value, so a parameter of the wrong type or in the wrong number is a compile error.",
                    ja: "ルートは、あるパスでサーバーが描画するページです。ルーターの配信もほかのページからのリンクも同じ値から作るので、パラメータの型や数の誤りはコンパイルエラーになります。",
                },
                Learn::Styling => L {
                    en: "Styles are Rust values too. A misspelled property or keyword, or a value of the wrong type, is a compile error.",
                    ja: "スタイルも Rust の値です。プロパティやキーワードの打ち間違いや、型の違う値はコンパイルエラーになります。",
                },
                Learn::EscapeHatches => L {
                    en: "Most pages need only bindings and statements. These cover focus, timers, and reactions to a change, links outside the router, and serving a page as a static file.",
                    ja: "たいていのページは束縛と文だけで書けます。ここで扱うのは、フォーカス、タイマー、変化への反応、ルーターの外へのリンク、ページを静的ファイルとして配信する方法です。",
                },
                Learn::JavaScript => L {
                    en: "A map, a chart, or a rich text editor runs in the browser as a JavaScript library. Wrapped in a custom element, it takes values from the page's signals and hands its events back as typed values.",
                    ja: "地図、グラフ、リッチテキストエディタは、ブラウザで JavaScript のライブラリとして動きます。カスタム要素で包むと、ページのシグナルから値を受け取り、イベントを型付きの値としてページに返せます。",
                },
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
                Learn::Security => L {
                    en: "What hypersig checks so that a page and its actions resist what a visitor can send, why data from the browser cannot run code on the server, and what is left to the application.",
                    ja: "訪問者が送るものからページとアクションを守るために hypersig が検査すること、ブラウザから届くデータがサーバーのコードを動かせない理由、アプリに任せることをまとめます。",
                },
                Learn::CallsInDepth => L {
                    en: "Patches that send only the items of a list that change, calls that overlap, streams that reconnect, and parts that several pages share.",
                    ja: "リストの変わった項目だけを送る差分、重なった呼び出し、つなぎ直すストリーム、複数のページで使う部品を扱います。",
                },
                Learn::RoutesInDepth => L {
                    en: "Typed query strings, URLs that change without loading a page, pages for missing paths, what search results and link previews show, and an application served under a path prefix.",
                    ja: "型の付いたクエリ文字列、ページを読み込まずに変える URL、存在しないパスのページ、検索結果とリンクのプレビューに出る内容、パスの接頭辞の下で配信するアプリを扱います。",
                },
                Learn::Signals => L {
                    en: "The page's state is one Rust struct, and a signal is a typed reference to one of its fields.",
                    ja: "ページの状態は Rust の構造体 1 つで表し、シグナルはそのフィールド 1 つを指す型付きの参照です。",
                },
                Learn::Expressions => L {
                    en: "An expression computes a value from the signals, and a statement changes them. Both are Rust values that the browser receives as JSON.",
                    ja: "式はシグナルから値を計算し、文はシグナルを変更します。どちらも Rust の値で、ブラウザは JSON として受け取ります。",
                },
                Learn::Lists => L {
                    en: "A `Vec` field is a list. `each` renders an element per item in the browser, and list statements change it without a request.",
                    ja: "`Vec` のフィールドはリストです。`each` はブラウザで項目ごとに要素を描画し、リストの文はリクエストを送らずにリストを変更します。",
                },
                Learn::Why => L {
                    en: "hypersig renders the page on the server and writes its behavior into the HTML. Every string the browser acts on comes from a Rust type, so the page needs no browser build, no hydration, and no string typed by hand.",
                    ja: "hypersig はページをサーバーで描画し、その動きを HTML に書き込みます。ブラウザが扱う文字列はすべて Rust の型から作られるので、ブラウザ向けのビルドもハイドレーションも不要で、手で書く文字列もありません。",
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
                    en: "Keyframes are Rust values too. When an element uses keyframes, their rule is written to the page along with the element.",
                    ja: "キーフレームも Rust の値です。要素がキーフレームを使うと、そのキーフレームのルールも要素と一緒にページに書かれます。",
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
                    ja: "`hypersig_ui` は、見た目が shadcn/ui に似ていて、振る舞いはブラウザのネイティブ要素と WAI-ARIA Authoring Practices に従うコンポーネント集です。",
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
            Self::Home | Self::Learn(_) | Self::Reference(_) => Library::Framework,
            Self::StyleHome | Self::Style(_) => Library::Style,
            Self::I18nHome | Self::I18n(_) => Library::I18n,
            Self::ComponentsHome | Self::Components(_) => Library::Components,
        }
    }

    /// Whether the page is a library's landing page, which has no sidebar.
    pub(crate) fn is_landing(self) -> bool {
        self == self.library().home()
    }

    /// The URL of the page.
    pub(crate) fn url(self, site: Site) -> String {
        site.page_url(&self.path())
    }

    /// The sidebar the page is listed in.
    pub(crate) fn groups(self) -> &'static [Group] {
        match self {
            Self::Home | Self::StyleHome | Self::ComponentsHome | Self::I18nHome => &[],
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
            Self::Why => "why",
            Self::QuickStart => "",
            Self::DescribingThePage => "describing-the-page",
            Self::Signals => "signals",
            Self::Expressions => "expressions",
            Self::Lists => "lists",
            Self::Server => "server",
            Self::Forms => "forms",
            Self::Requests => "requests",
            Self::Pages => "pages",
            Self::Styling => "styling",
            Self::EscapeHatches => "escape-hatches",
            Self::JavaScript => "javascript",
            Self::Deploying => "deploying",
            Self::DeployContainer => "deploying/container",
            Self::DeployCloudflare => "deploying/cloudflare",
            Self::DeployVercel => "deploying/vercel",
            Self::Security => "security",
            Self::CallsInDepth => "calls",
            Self::RoutesInDepth => "routes",
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
            Self::Listbox => "listbox",
            Self::Combobox => "combobox",
            Self::Tabs => "tabs",
            Self::Accordion => "accordion",
            Self::Card => "card",
            Self::Separator => "separator",
            Self::Navigation => "navigation",
            Self::DataTable => "data-table",
            Self::Chart => "chart",
            Self::Dialog => "dialog",
            Self::Menu => "menu",
            Self::Tooltip => "tooltip",
            Self::Toast => "toast",
            Self::Alert => "alert",
            Self::Badge => "badge",
            Self::Progress => "progress",
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
    fn every_page_but_the_landing_pages_is_in_a_sidebar() {
        for &doc in Doc::ALL {
            let listed = doc.groups().iter().any(|group| group.docs.contains(&doc));
            assert_eq!(listed, !doc.is_landing(), "{doc:?}");
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
