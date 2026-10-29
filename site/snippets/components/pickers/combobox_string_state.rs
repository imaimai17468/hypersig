use hypersig::{NodeRef, Signals};
use hypersig_ui::{Combobox, ComboboxText};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shipping {
    // Only the text, without the list's state.
    country: String,
}

hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Shipping>);
hypersig::node_ref!(const COUNTRY_OPTIONS: NodeRef<Shipping>);
const COUNTRY: Combobox<Shipping> = Combobox::new(COUNTRY_FIELD, COUNTRY_OPTIONS);
# fn main() {

let country = COUNTRY.render(
    "Country",
    Shipping::country(),
    ["France", "Japan"],
    &ComboboxText::default(),
);
# let _ = country;
# }
