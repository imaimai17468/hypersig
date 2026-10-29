//! A list of sections over pages of this site, for the navigation page.

use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Rest, Signals};
use hypersig_ui::NavList;
use serde::{Deserialize, Serialize};

use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::{PAGE, Site};

#[derive(Signals, Serialize, Deserialize)]
pub struct Docs {}

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
            en: "Breadcrumb",
            ja: "パンくずリスト",
        }),
        PAGE,
        at(site, Doc::Components(Component::Breadcrumb)),
    )
    .item(
        site.t(L {
            en: "Pagination",
            ja: "ページネーション",
        }),
        PAGE,
        at(site, Doc::Components(Component::Pagination)),
    );
    Page::new(
        site.t(L {
            en: "Docs",
            ja: "ドキュメント",
        }),
        Docs {},
    )
    .child(
        sections
            .render()
            .css(Style::new().max_width(rem(12))),
    )
}
