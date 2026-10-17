//! The pages of the site, their paths, and the order the sidebar lists them in.

use crate::{Site, home, layout, learn, reference};

/// One page of the site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Doc {
    /// The landing page.
    Home,
    /// A page of the Learn section.
    Learn(Learn),
    /// A page of the Reference section.
    Reference(Reference),
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

impl Doc {
    /// Every page, in the order the site lists them.
    pub const ALL: &[Self] = &[
        Self::Home,
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
    ];

    /// The page's directory inside the site, ending in `/`, or `""` for the
    /// landing page.
    #[must_use]
    pub fn path(self) -> String {
        match self {
            Self::Home => String::new(),
            Self::Learn(Learn::QuickStart) => "learn/".to_owned(),
            Self::Reference(Reference::Overview) => "reference/".to_owned(),
            Self::Learn(page) => format!("learn/{}/", page.slug()),
            Self::Reference(page) => format!("reference/{}/", page.slug()),
        }
    }

    /// The page's title, as the sidebar and the browser tab show it.
    #[must_use]
    pub fn title(self) -> &'static str {
        match self {
            Self::Home => "hypersig",
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
        }
    }

    /// The URL of the page.
    pub(crate) fn url(self, site: Site) -> String {
        site.url(&self.path())
    }

    /// The sidebar the page is listed in.
    pub(crate) fn groups(self) -> &'static [Group] {
        match self {
            Self::Home => &[],
            Self::Learn(_) => LEARN,
            Self::Reference(_) => REFERENCE,
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
            Self::Learn(page) => layout::article(site, self, learn::content(site, page)),
            Self::Reference(page) => layout::article(site, self, reference::content(site, page)),
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
    fn every_page_but_the_landing_page_is_in_a_sidebar() {
        for &doc in Doc::ALL {
            let listed = doc.groups().iter().any(|group| group.docs.contains(&doc));
            assert_eq!(listed, doc != Doc::Home, "{doc:?}");
        }
    }
}
