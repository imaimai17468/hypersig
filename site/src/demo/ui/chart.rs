//! A bar chart, for the chart page.

use hypersig::html::div;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Chart, ChartScale, button, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Month {
    name: String,
    desktop: i32,
    mobile: i32,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Traffic {
    months: Vec<Month>,
    scale: ChartScale,
}

pub fn page(site: Site) -> Page<Traffic> {
    let chart = Chart::bar(
        site.t(L {
            en: "Visitors",
            ja: "訪問者数",
        }),
        Traffic::months(),
        Traffic::scale(),
        Month::name(),
    )
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
    );
    let names = [
        L { en: "Jan", ja: "1 月" },
        L { en: "Feb", ja: "2 月" },
        L { en: "Mar", ja: "3 月" },
        L { en: "Apr", ja: "4 月" },
        L { en: "May", ja: "5 月" },
        L { en: "Jun", ja: "6 月" },
    ];
    let months: Vec<Month> = names
        .into_iter()
        .zip([(186, 80), (305, 200), (237, 120), (73, 190), (209, 130), (214, 140)])
        .map(|(name, (desktop, mobile))| Month {
            name: site.t(name).to_owned(),
            desktop,
            mobile,
        })
        .collect();
    let scale = chart.scale(&months);
    let grow = Traffic::months().update_where(
        |month| month.index().equals(3),
        Month::desktop(),
        |month| month.at(Month::desktop()) + 40,
    );
    Page::new(
        site.t(L {
            en: "Visitors",
            ja: "訪問者数",
        }),
        Traffic { months, scale },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(
                div().child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(grow)
                        .text(site.t(L {
                            en: "Add 40 desktop visitors in April",
                            ja: "4 月のデスクトップに 40 人足す",
                        })),
                ),
            )
            .child(chart),
    )
}
