use hypersig::html::p;
use hypersig::{DialogRef, Signals};
use hypersig_ui::{Dialog, DialogText};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

hypersig::dialog_ref!(const CART_SHEET: DialogRef<Shop>);
const CART: Dialog<Shop> = Dialog::new(CART_SHEET);
# fn main() {

// The edge is a `Side`, not a string.
let sheet = CART.sheet("right", "Cart", "Two items.", p(), &DialogText::default());
# }
