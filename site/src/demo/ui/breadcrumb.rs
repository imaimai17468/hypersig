//! A breadcrumb trail over pages of this site, for the breadcrumb page.

use hypersig::html::{div, h2};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Rest, Signals};
use hypersig_ui::Breadcrumb;
use serde::{Deserialize, Serialize};

use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::{PAGE, Site};

#[derive(Signals, Serialize, Deserialize)]
pub struct Docs {
    /// The title of the page the demo stands on.
    title: String,
}

/// The path of `doc` inside this site, as `PAGE` takes it.
fn at(site: Site, doc: Doc) -> Rest {
    Rest::parse(doc.url(site).trim_start_matches('/'))
        .expect("a component's page has a directory without `.` or `..` parts")
}

pub fn page(site: Site) -> Page<Docs> {
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
    .link(
        site.t(L {
            en: "Navigation",
            ja: "ナビゲーション",
        }),
        PAGE.link_with(at(site, Doc::Components(Component::Navigation))),
    )
    .current(site.t(L {
        en: "Breadcrumb",
        ja: "パンくずリスト",
    }));
    Page::new(
        site.t(L {
            en: "Docs",
            ja: "ドキュメント",
        }),
        Docs {
            title: Doc::Components(Component::Breadcrumb).title(site).into(),
        },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(trail)
            .child(h2().bind_text(Docs::title())),
    )
}
