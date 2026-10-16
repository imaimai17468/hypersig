use hypersig::Signals;
use hypersig_ui::{Combobox, ComboboxState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Form {
    country: ComboboxState,
}

const COUNTRY: Combobox<Form> = Combobox::new("country", "country");

fn main() {
    let _ = COUNTRY.render("Country", Form::country(), ["Japan"]);
}
