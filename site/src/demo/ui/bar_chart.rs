//! Bar, stacked bar, and horizontal bar charts of the same months, for the
//! bar chart page.

use hypersig::{Page, Segment, Signals};
use hypersig_ui::{Chart, Tabs, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Kind {
    Bar,
    Stacked,
    Horizontal,
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
    // Each kind of chart draws the same two series of the same months.
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
    let tabs = Tabs::new(label, Traffic::kind(), |kind| match kind {
        Kind::Bar => (
            site.t(L {
                en: "Bar",
                ja: "棒",
            }),
            visitors(Chart::bar(label, Traffic::months(), Month::name())).render(),
        ),
        Kind::Stacked => (
            site.t(L {
                en: "Stacked",
                ja: "積み上げ",
            }),
            visitors(Chart::stacked_bar(label, Traffic::months(), Month::name())).render(),
        ),
        Kind::Horizontal => (
            site.t(L {
                en: "Horizontal",
                ja: "横棒",
            }),
            visitors(Chart::horizontal_bar(label, Traffic::months(), Month::name())).render(),
        ),
    });
    Page::new(
        label,
        Traffic {
            kind: Kind::Bar,
            months,
        },
    )
    .child(tabs)
}
