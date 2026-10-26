use hypersig::html::div;
use hypersig::{NodeRef, Page, Signals};
use hypersig_ui::{Combobox, ComboboxState, TableState};
use serde::{Deserialize, Serialize};

// The state of every component on the page, in one field of the page's struct.
#[derive(Signals, Serialize, Deserialize, Default)]
struct Controls {
    country: ComboboxState,
    orders: TableState,
}

#[derive(Signals, Serialize, Deserialize)]
struct Orders {
    customer: String,
    controls: Controls,
}

hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Orders>);
hypersig::node_ref!(const COUNTRY_OPTIONS: NodeRef<Orders>);
const COUNTRY: Combobox<Orders> = Combobox::new(COUNTRY_FIELD, COUNTRY_OPTIONS);

# fn main() {
let controls = Orders::controls();
let page = Page::new(
    "Orders",
    Orders {
        customer: String::new(),
        controls: Controls::default(),
    },
)
.child(div().child(COUNTRY.render(
    "Country",
    controls.at(Controls::country()),
    ["France", "Japan", "Kenya"],
)));
# let _ = page.render();
# }
