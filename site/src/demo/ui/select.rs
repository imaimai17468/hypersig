//! A select, for the select page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::select_field;
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Region {
    Tokyo,
    Frankfurt,
    Virginia,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Deploy {
    region: Region,
}

pub fn page(site: Site) -> Page<Deploy> {
    Page::new(
        site.t(L {
            en: "Deploy",
            ja: "デプロイ",
        }),
        Deploy {
            region: Region::Tokyo,
        },
    )
    .child(
        div()
            .css(Style::new().max_width(rem(16)))
            .child(select_field(
                site.t(L {
                    en: "Region",
                    ja: "リージョン",
                }),
                Deploy::region(),
                [
                    (
                        Region::Tokyo,
                        site.t(L {
                            en: "Tokyo",
                            ja: "東京",
                        }),
                    ),
                    (
                        Region::Frankfurt,
                        site.t(L {
                            en: "Frankfurt",
                            ja: "フランクフルト",
                        }),
                    ),
                    (
                        Region::Virginia,
                        site.t(L {
                            en: "Virginia",
                            ja: "バージニア",
                        }),
                    ),
                ],
            )),
    )
}
