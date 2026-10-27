//! A list of sections, a breadcrumb, and pagination over pages of this
//! site, for the navigation page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::{Display, raw, rem};
use hypersig::{Page, Rest, Signals};
use hypersig_ui::{Breadcrumb, NavList, PaginationText, pagination};
use serde::{Deserialize, Serialize};

use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::{PAGE, Site};

#[derive(Signals, Serialize, Deserialize)]
pub struct Docs {
    title: String,
}

/// The component pages the pagination steps through.
const PAGES: [Component; 6] = [
    Component::Tabs,
    Component::Accordion,
    Component::Card,
    Component::Separator,
    Component::Navigation,
    Component::DataTable,
];

/// The path of `doc` inside this site, as `PAGE` takes it.
fn at(site: Site, doc: Doc) -> Rest {
    Rest::parse(doc.url(site).trim_start_matches('/'))
        .expect("a component's page has a directory without `.` or `..` parts")
}

pub fn page(site: Site) -> Page<Docs> {
    let here = Doc::Components(Component::Navigation);
    let sections = NavList::new(
        site.t(L {
            en: "Sections",
            ja: "セクション",
        }),
        PAGE.url(at(site, here)),
    )
    .item(
        site.t(L {
            en: "Installation",
            ja: "インストール",
        }),
        PAGE,
        at(site, Doc::Components(Component::Installation)),
    )
    .item(
        site.t(L {
            en: "Navigation",
            ja: "ナビゲーション",
        }),
        PAGE,
        at(site, here),
    )
    .item(
        site.t(L {
            en: "Dialog",
            ja: "ダイアログ",
        }),
        PAGE,
        at(site, Doc::Components(Component::Dialog)),
    );
    let trail = Breadcrumb::new(site.t(L {
        en: "Breadcrumb",
        ja: "パンくずリスト",
    }))
    .link(
        site.t(L {
            en: "Components",
            ja: "コンポーネント",
        }),
        PAGE.link_with(at(site, Doc::ComponentsHome)),
    )
    .current(site.t(L {
        en: "Navigation",
        ja: "ナビゲーション",
    }));
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
        6,
        |page| PAGE.link_with(at(site, Doc::Components(PAGES[page as usize - 1]))),
        &text,
    );
    let initial = Docs {
        title: here.title(site).into(),
    };
    Page::new(
        site.t(L {
            en: "Docs",
            ja: "ドキュメント",
        }),
        initial,
    )
    .child(
        div()
            .css(
                Style::new()
                    .display(Display::Grid)
                    .grid_template_columns(raw("9rem 1fr"))
                    .gap(rem(1.5)),
            )
            .child(sections)
            .child(div().css(stack(rem(1.5))).child(trail).child(pager)),
    )
}
