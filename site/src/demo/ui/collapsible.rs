//! A collapsible list of past releases, for the collapsible page.

use hypersig::html::{div, li, p, ul};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{JustifyContent, rem};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Collapsible};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Releases {
    older: bool,
}

pub fn page(site: Site) -> Page<Releases> {
    let older = Collapsible::new(Releases::older());
    Page::new(
        site.t(L {
            en: "Releases",
            ja: "リリース",
        }),
        Releases { older: false },
    )
    .child(
        div()
            .css(stack(rem(0.5)))
            .child(
                div()
                    .css(hstack(rem(1)).justify_content(JustifyContent::SpaceBetween))
                    .child(p().text(site.t(L {
                        en: "0.8.2 is the latest release.",
                        ja: "最新のリリースは 0.8.2 です。",
                    })))
                    .child(
                        older
                            .trigger(ButtonVariant::Outline, ButtonSize::Sm)
                            .text(site.t(L {
                                en: "Older releases",
                                ja: "以前のリリース",
                            })),
                    ),
            )
            .child(
                older.content().child(
                    ul().css(stack(rem(0.25)))
                        .child(li().text("0.8.1"))
                        .child(li().text("0.8.0"))
                        .child(li().text("0.7.1")),
                ),
            ),
    )
}
