//! Line and area charts of the same months, for the chart page.

use hypersig::html::div;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Segment, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Chart, Tabs, button, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Kind {
    Line,
    Area,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Month {
    name: String,
    desktop: i32,
    mobile: i32,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Traffic {
    kind: Kind,
    months: Vec<Month>,
}

pub fn page(site: Site) -> Page<Traffic> {
    // Both kinds of chart draw the same two series of the same months.
    let visitors = |chart: Chart<Traffic, Month>| {
        chart
            .series(
                site.t(L {
                    en: "Desktop",
                    ja: "デスクトップ",
                }),
                Month::desktop(),
                token::chart_1(),
            )
            .series(
                site.t(L {
                    en: "Mobile",
                    ja: "モバイル",
                }),
                Month::mobile(),
                token::chart_2(),
            )
    };
    let label = site.t(L {
        en: "Visitors",
        ja: "訪問者数",
    });
    let names = [
        L { en: "Jan", ja: "1 月" },
        L { en: "Feb", ja: "2 月" },
        L { en: "Mar", ja: "3 月" },
        L { en: "Apr", ja: "4 月" },
        L { en: "May", ja: "5 月" },
        L { en: "Jun", ja: "6 月" },
    ];
    let months = names
        .into_iter()
        .zip([(186, 80), (305, 200), (237, 120), (73, 190), (209, 130), (214, 140)])
        .map(|(name, (desktop, mobile))| Month {
            name: site.t(name).to_owned(),
            desktop,
            mobile,
        })
        .collect();
    let grow = Traffic::months().update_where(
        |month| month.index().equals(3),
        Month::desktop(),
        |month| month.at(Month::desktop()) + 400,
    );
    let tabs = Tabs::new(label, Traffic::kind(), |kind| match kind {
        Kind::Line => (
            site.t(L {
                en: "Line",
                ja: "折れ線",
            }),
            visitors(Chart::line(label, Traffic::months(), Month::name())).render(),
        ),
        Kind::Area => (
            site.t(L {
                en: "Area",
                ja: "面",
            }),
            visitors(Chart::area(label, Traffic::months(), Month::name())).render(),
        ),
    });
    Page::new(
        label,
        Traffic {
            kind: Kind::Line,
            months,
        },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(
                div().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(grow)
                        .text(site.t(L {
                            en: "Add 400 desktop visitors in April",
                            ja: "4 月のデスクトップの訪問者を 400 人増やす",
                        })),
                ),
            )
            .child(tabs),
    )
}
