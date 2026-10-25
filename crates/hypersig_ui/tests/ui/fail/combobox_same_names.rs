use hypersig::Signals;
use hypersig_ui::{Combobox, ComboboxState};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Form {
    country: ComboboxState,
}

hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Form>);
const COUNTRY: Combobox<Form> = Combobox::new(COUNTRY_FIELD, COUNTRY_FIELD);

fn main() {
    let _ = COUNTRY.render("Country", Form::country(), ["Japan"]);
}
