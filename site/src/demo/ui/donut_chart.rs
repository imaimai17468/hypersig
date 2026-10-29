//! A donut chart of visitors by browser, for the donut chart page.

use hypersig::html::div;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, DonutChart, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Browser {
    name: String,
    visitors: i32,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Traffic {
    browsers: Vec<Browser>,
}

pub fn page(site: Site) -> Page<Traffic> {
    let browsers = ["Chrome", "Safari", "Firefox", "Edge"]
        .into_iter()
        .map(str::to_owned)
        .chain([site
            .t(L {
                en: "Other",
                ja: "その他",
            })
            .to_owned()])
        .zip([275, 200, 187, 173, 90])
        .map(|(name, visitors)| Browser { name, visitors })
        .collect();
    let grow = Traffic::browsers().update_where(
        |browser| browser.index().equals(2),
        Browser::visitors(),
        |browser| browser.at(Browser::visitors()) + 100,
    );
    Page::new(
        site.t(L {
            en: "Browsers",
            ja: "ブラウザ",
        }),
        Traffic { browsers },
    )
    .child(
        div()
            .css(stack(rem(1)).max_width(rem(20)))
            .child(
                div().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(grow)
                        .text(site.t(L {
                            en: "Add 100 Firefox visitors",
                            ja: "Firefox の訪問者を 100 人増やす",
                        })),
                ),
            )
            .child(
                DonutChart::new(
                    site.t(L {
                        en: "Visitors by browser",
                        ja: "ブラウザごとの訪問者数",
                    }),
                    Traffic::browsers(),
                    Browser::name(),
                    Browser::visitors(),
                ),
            ),
    )
}
