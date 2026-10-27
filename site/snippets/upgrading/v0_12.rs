# use hypersig::html::p;
# use hypersig::{DialogRef, Signals};
# use hypersig_ui::{Dialog, DialogText};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Shop {}
#
# hypersig::dialog_ref!(const CART_SHEET: DialogRef<Shop>);
# const CART: Dialog<Shop> = Dialog::new(CART_SHEET);
#
# fn main() {
//-let cart = CART.dialog("Cart", "Two items.", p().text("Tea, cups"));
let english = CART.dialog("Cart", "Two items.", p().text("Tea, cups"), &DialogText::default()); //+
let japanese = DialogText { close: "閉じる".into() }; //+
let cart = CART.dialog("カート", "2 点あります。", p().text("お茶、湯のみ"), &japanese); //+
# let _ = (english, cart);
# }
