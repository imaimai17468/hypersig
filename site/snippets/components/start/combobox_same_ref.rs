use hypersig::{NodeRef, Signals};
use hypersig_ui::Combobox;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shipping {}

hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Shipping>);
// The field's node ref given for the list as well.
const COUNTRY: Combobox<Shipping> = Combobox::new(COUNTRY_FIELD, COUNTRY_FIELD);
# fn main() {
#     let _ = COUNTRY;
# }
