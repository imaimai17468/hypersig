//! A combobox, for the combobox page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{Combobox, ComboboxState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Shipping {
    country: ComboboxState,
}

const COUNTRY: Combobox<Shipping> = Combobox::new("country", "country-options");

pub fn page() -> Page<Shipping> {
    let initial = Shipping {
        country: ComboboxState::new(""),
    };
    Page::new("Shipping", initial).child(
        div().css(Style::new().max_width(rem(20))).child(COUNTRY.render(
            "Country",
            Shipping::country(),
            [
                "Brazil", "Canada", "France", "Germany", "India", "Japan", "Kenya", "Mexico",
                "Norway", "Spain",
            ],
        )),
    )
}
