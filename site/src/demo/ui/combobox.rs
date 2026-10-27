//! A combobox, for the combobox page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{Combobox, ComboboxState, ComboboxText};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Shipping {
    country: ComboboxState,
}

hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Shipping>);
hypersig::node_ref!(const COUNTRY_OPTIONS: NodeRef<Shipping>);
const COUNTRY: Combobox<Shipping> = Combobox::new(COUNTRY_FIELD, COUNTRY_OPTIONS);

pub fn page(site: Site) -> Page<Shipping> {
    let initial = Shipping {
        country: ComboboxState::new(""),
    };
    Page::new(
        site.t(L {
            en: "Shipping",
            ja: "配送先",
        }),
        initial,
    )
    .child(
        div()
            .css(Style::new().max_width(rem(20)))
            .child(COUNTRY.render(
                site.t(L {
                    en: "Country",
                    ja: "国",
                }),
                Shipping::country(),
                site.t(L {
                    en: [
                        "Brazil", "Canada", "France", "Germany", "India", "Japan", "Kenya",
                        "Mexico", "Norway", "Spain",
                    ],
                    ja: [
                        "インド",
                        "カナダ",
                        "ケニア",
                        "スペイン",
                        "ドイツ",
                        "日本",
                        "ノルウェー",
                        "ブラジル",
                        "フランス",
                        "メキシコ",
                    ],
                }),
                &ComboboxText {
                    list: site
                        .t(L {
                            en: "Suggestions",
                            ja: "候補",
                        })
                        .into(),
                    empty: site
                        .t(L {
                            en: "No matches",
                            ja: "一致する国はありません",
                        })
                        .into(),
                },
            )),
    )
}
