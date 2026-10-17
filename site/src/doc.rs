//! The pages of the site, their paths, and the order the sidebar lists them in.

use crate::{Site, components, home, landing, layout, learn, reference, style};

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
}

/// The pages of the Learn section, a guide read from top to bottom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Learn {
    QuickStart,
    DescribingThePage,
    Signals,
    Expressions,
    Lists,
    Server,
    Styling,
    EscapeHatches,
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
    Patches,
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
    Card,
    Separator,
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

/// One of the three sets of docs, each with its own header and landing
/// page, as the libraries of `TanStack` share one site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Library {
    /// hypersig itself.
    Framework,
    /// `hypersig::style`, from the `hypersig_style` crate.
    Style,
    /// The `hypersig_ui` crate.
    Components,
}

impl Library {
    /// Every library, in the order the switcher lists them.
    pub const ALL: [Self; 3] = [Self::Framework, Self::Style, Self::Components];

    /// The name the header and the switcher show.
    pub fn name(self) -> &'static str {
        match self {
            Self::Framework => "Framework",
            Self::Style => "Style",
            Self::Components => "Components",
        }
    }

    /// The library's landing page.
    pub fn home(self) -> Doc {
        match self {
            Self::Framework => Doc::Home,
            Self::Style => Doc::StyleHome,
            Self::Components => Doc::ComponentsHome,
        }
    }

    /// The sections the header links to, each with its first page.
    pub fn sections(self) -> &'static [(&'static str, Doc)] {
        match self {
            Self::Framework => &[
                ("Learn", Doc::Learn(Learn::QuickStart)),
                ("Reference", Doc::Reference(Reference::Overview)),
            ],
            Self::Style => &[("Docs", Doc::Style(StylePage::Overview))],
            Self::Components => &[("Docs", Doc::Components(Component::Installation))],
        }
    }

    /// The front page of the library's rustdoc output, as a link target.
    pub fn api(self) -> &'static str {
        match self {
            Self::Framework => "api:hypersig/index.html",
            Self::Style => "api:hypersig_style/index.html",
            Self::Components => "api:hypersig_ui/index.html",
        }
    }
}

/// A titled run of pages in a sidebar.
#[derive(Debug)]
pub struct Group {
    pub title: &'static str,
    pub docs: &'static [Doc],
}

/// The sidebar of the Learn section.
pub const LEARN: &[Group] = &[
    Group {
        title: "Get started",
        docs: &[Doc::Learn(Learn::QuickStart)],
    },
    Group {
        title: "Learn hypersig",
        docs: &[
            Doc::Learn(Learn::DescribingThePage),
            Doc::Learn(Learn::Signals),
            Doc::Learn(Learn::Expressions),
            Doc::Learn(Learn::Lists),
            Doc::Learn(Learn::Server),
            Doc::Learn(Learn::Styling),
            Doc::Learn(Learn::EscapeHatches),
        ],
    },
];

/// The sidebar of the Reference section.
pub const REFERENCE: &[Group] = &[
    Group {
        title: "Reference",
        docs: &[Doc::Reference(Reference::Overview)],
    },
    Group {
        title: "The page",
        docs: &[
            Doc::Reference(Reference::Page),
            Doc::Reference(Reference::El),
            Doc::Reference(Reference::Html),
        ],
    },
    Group {
        title: "State",
        docs: &[
            Doc::Reference(Reference::Signals),
            Doc::Reference(Reference::Expr),
            Doc::Reference(Reference::Stmt),
            Doc::Reference(Reference::NodeRef),
        ],
    },
    Group {
        title: "The server",
        docs: &[
            Doc::Reference(Reference::Action),
            Doc::Reference(Reference::Patches),
            Doc::Reference(Reference::RouterExt),
        ],
    },
];

/// The sidebar of the Components section.
pub const COMPONENTS: &[Group] = &[
    Group {
        title: "Get started",
        docs: &[
            Doc::Components(Component::Installation),
            Doc::Components(Component::Theming),
        ],
    },
    Group {
        title: "Forms",
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
        title: "Pickers",
        docs: &[
            Doc::Components(Component::Listbox),
            Doc::Components(Component::Combobox),
        ],
    },
    Group {
        title: "Layout",
        docs: &[
            Doc::Components(Component::Tabs),
            Doc::Components(Component::Accordion),
            Doc::Components(Component::Card),
            Doc::Components(Component::Separator),
        ],
    },
    Group {
        title: "Overlays",
        docs: &[
            Doc::Components(Component::Dialog),
            Doc::Components(Component::Menu),
            Doc::Components(Component::Tooltip),
            Doc::Components(Component::Toast),
        ],
    },
    Group {
        title: "Feedback",
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
        title: "Get started",
        docs: &[Doc::Style(StylePage::Overview)],
    },
    Group {
        title: "Writing styles",
        docs: &[
            Doc::Style(StylePage::Values),
            Doc::Style(StylePage::Conditions),
            Doc::Style(StylePage::Tokens),
        ],
    },
    Group {
        title: "The page",
        docs: &[
            Doc::Style(StylePage::Global),
            Doc::Style(StylePage::Patterns),
            Doc::Style(StylePage::Animation),
        ],
    },
    Group {
        title: "Escape hatches",
        docs: &[Doc::Style(StylePage::Unchecked)],
    },
];

impl Doc {
    /// Every page, in the order the site lists them.
    pub const ALL: &[Self] = &[
        Self::Home,
        Self::StyleHome,
        Self::ComponentsHome,
        Self::Learn(Learn::QuickStart),
        Self::Learn(Learn::DescribingThePage),
        Self::Learn(Learn::Signals),
        Self::Learn(Learn::Expressions),
        Self::Learn(Learn::Lists),
        Self::Learn(Learn::Server),
        Self::Learn(Learn::Styling),
        Self::Learn(Learn::EscapeHatches),
        Self::Reference(Reference::Overview),
        Self::Reference(Reference::Page),
        Self::Reference(Reference::El),
        Self::Reference(Reference::Html),
        Self::Reference(Reference::Signals),
        Self::Reference(Reference::Expr),
        Self::Reference(Reference::Stmt),
        Self::Reference(Reference::NodeRef),
        Self::Reference(Reference::Action),
        Self::Reference(Reference::Patches),
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
        }
    }

    /// The page's title, as the sidebar and the browser tab show it.
    #[must_use]
    pub fn title(self) -> &'static str {
        match self {
            Self::Home => "hypersig",
            Self::StyleHome => "hypersig style",
            Self::ComponentsHome => "hypersig components",
            Self::Learn(page) => match page {
                Learn::QuickStart => "Quick start",
                Learn::DescribingThePage => "Describing the page",
                Learn::Signals => "Signals",
                Learn::Expressions => "Expressions and statements",
                Learn::Lists => "Lists",
                Learn::Server => "Talking to the server",
                Learn::Styling => "Styling",
                Learn::EscapeHatches => "Escape hatches",
            },
            Self::Reference(page) => match page {
                Reference::Overview => "API reference",
                Reference::Page => "Page",
                Reference::El => "El",
                Reference::Html => "html functions",
                Reference::Signals => "Signals and Sig",
                Reference::Expr => "Expr",
                Reference::Stmt => "Stmt",
                Reference::Action => "Action and Call",
                Reference::Patches => "Patches and Fragment",
                Reference::NodeRef => "NodeRef",
                Reference::RouterExt => "RouterExt",
            },
            Self::Components(page) => match page {
                Component::Installation => "Installation",
                Component::Theming => "Theming",
                Component::Button => "Button",
                Component::TextField => "Text field",
                Component::Checkbox => "Checkbox and switch",
                Component::RadioGroup => "Radio group",
                Component::Select => "Select",
                Component::Slider => "Slider",
                Component::Listbox => "Listbox",
                Component::Combobox => "Combobox",
                Component::Tabs => "Tabs",
                Component::Accordion => "Accordion",
                Component::Card => "Card",
                Component::Separator => "Separator",
                Component::Dialog => "Dialog",
                Component::Menu => "Menu",
                Component::Tooltip => "Tooltip",
                Component::Toast => "Toast",
                Component::Alert => "Alert",
                Component::Badge => "Badge",
                Component::Progress => "Progress and skeleton",
            },
            Self::Style(page) => match page {
                StylePage::Overview => "Overview",
                StylePage::Values => "Properties and values",
                StylePage::Conditions => "Conditions",
                StylePage::Tokens => "Tokens and themes",
                StylePage::Global => "Global styles",
                StylePage::Patterns => "Layout patterns",
                StylePage::Animation => "Animation",
                StylePage::Unchecked => "Unchecked values",
            },
        }
    }

    /// The library whose docs the page belongs to.
    pub(crate) fn library(self) -> Library {
        match self {
            Self::Home | Self::Learn(_) | Self::Reference(_) => Library::Framework,
            Self::StyleHome | Self::Style(_) => Library::Style,
            Self::ComponentsHome | Self::Components(_) => Library::Components,
        }
    }

    /// Whether the page is a library's landing page, which has no sidebar.
    pub(crate) fn is_landing(self) -> bool {
        self == self.library().home()
    }

    /// The URL of the page.
    pub(crate) fn url(self, site: Site) -> String {
        site.url(&self.path())
    }

    /// The sidebar the page is listed in.
    pub(crate) fn groups(self) -> &'static [Group] {
        match self {
            Self::Home | Self::StyleHome | Self::ComponentsHome => &[],
            Self::Learn(_) => LEARN,
            Self::Reference(_) => REFERENCE,
            Self::Components(_) => COMPONENTS,
            Self::Style(_) => STYLE,
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
            Self::Learn(page) => layout::article(site, self, learn::content(site, page)),
            Self::Reference(page) => layout::article(site, self, reference::content(site, page)),
            Self::Components(page) => layout::article(site, self, components::content(site, page)),
            Self::Style(page) => layout::article(site, self, style::content(site, page)),
        }
    }
}

impl Learn {
    fn slug(self) -> &'static str {
        match self {
            Self::QuickStart => "",
            Self::DescribingThePage => "describing-the-page",
            Self::Signals => "signals",
            Self::Expressions => "expressions",
            Self::Lists => "lists",
            Self::Server => "server",
            Self::Styling => "styling",
            Self::EscapeHatches => "escape-hatches",
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
            Self::Patches => "patches",
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
