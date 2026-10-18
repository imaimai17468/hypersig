//! Bar, stacked bar, horizontal bar, and donut charts, for the chart page.

use hypersig::html::div;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Chart, DonutChart, Tabs, button, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Kind {
    Bar,
    Stacked,
    Horizontal,
    Donut,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Month {
    name: String,
    desktop: i32,
    mobile: i32,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Browser {
    name: String,
    visitors: i32,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Traffic {
    kind: Kind,
    months: Vec<Month>,
    browsers: Vec<Browser>,
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
    let grow = Traffic::months().update_where(
        |month| month.index().equals(3),
        Month::desktop(),
        |month| month.at(Month::desktop()) + 300,
    );
    let tabs = Tabs::new(label, Traffic::kind())
        .tab(
            Kind::Bar,
            site.t(L {
                en: "Bar",
                ja: "棒",
            }),
            visitors(Chart::bar(label, Traffic::months(), Month::name())).render(),
        )
        .tab(
            Kind::Stacked,
            site.t(L {
                en: "Stacked",
                ja: "積み上げ",
            }),
            visitors(Chart::stacked_bar(label, Traffic::months(), Month::name())).render(),
        )
        .tab(
            Kind::Horizontal,
            site.t(L {
                en: "Horizontal",
                ja: "横棒",
            }),
            visitors(Chart::horizontal_bar(label, Traffic::months(), Month::name())).render(),
        )
        .tab(
            Kind::Donut,
            site.t(L {
                en: "Donut",
                ja: "ドーナツ",
            }),
            DonutChart::new(
                site.t(L {
                    en: "Visitors by browser",
                    ja: "ブラウザごとの訪問者数",
                }),
                Traffic::browsers(),
                Browser::name(),
                Browser::visitors(),
            )
            .render(),
        );
    Page::new(
        label,
        Traffic {
            kind: Kind::Bar,
            months,
            browsers,
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
                            en: "Add 300 desktop visitors in April",
                            ja: "4 月のデスクトップに 300 人足す",
                        })),
                ),
            )
            .child(tabs),
    )
}
