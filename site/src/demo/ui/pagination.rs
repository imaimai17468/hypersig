//! Pagination over the component pages of this site, for the pagination
//! page.

use hypersig::html::{div, h2};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Rest, Signals};
use hypersig_ui::{PaginationText, pagination};
use serde::{Deserialize, Serialize};

use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::{PAGE, Site};

#[derive(Signals, Serialize, Deserialize)]
pub struct Docs {
    /// The title of the page the demo stands on.
    title: String,
}

/// The component pages the pagination steps through, this page fifth.
const PAGES: [Component; 9] = [
    Component::Tabs,
    Component::Accordion,
    Component::Collapsible,
    Component::Card,
    Component::Pagination,
    Component::Separator,
    Component::Navigation,
    Component::Breadcrumb,
    Component::DataTable,
];

/// The path of `doc` inside this site, as `PAGE` takes it.
fn at(site: Site, doc: Doc) -> Rest {
    Rest::parse(doc.url(site).trim_start_matches('/'))
        .expect("a component's page has a directory without `.` or `..` parts")
}

pub fn page(site: Site) -> Page<Docs> {
    let text = PaginationText {
        label: site
            .t(L {
                en: "Pagination",
                ja: "ページ送り",
            })
            .into(),
        previous: site
            .t(L {
                en: "Previous",
                ja: "前へ",
            })
            .into(),
        next: site
            .t(L {
                en: "Next",
                ja: "次へ",
            })
            .into(),
    };
    let pager = pagination(
        5,
        9,
        |page| PAGE.link_with(at(site, Doc::Components(PAGES[page as usize - 1]))),
        &text,
    );
    Page::new(
        site.t(L {
            en: "Docs",
            ja: "ドキュメント",
        }),
        Docs {
            title: Doc::Components(Component::Pagination).title(site).into(),
        },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(h2().bind_text(Docs::title()))
            .child(pager),
    )
}
