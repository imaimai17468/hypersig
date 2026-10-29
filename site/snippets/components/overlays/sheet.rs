use hypersig::html::{div, li, ul};
use hypersig::{DialogRef, Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, DialogText, Side, dialog_footer};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

hypersig::dialog_ref!(const CART_SHEET: DialogRef<Shop>);
const CART: Dialog<Shop> = Dialog::new(CART_SHEET);
# fn main() {

let html = Page::new("Shop", Shop {})
    .child(CART.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Cart"))
    .child(CART.sheet(
        Side::Right,
        "Cart",
        "Two items, ready to check out.",
        div()
            .child(ul().child(li().text("Green tea")).child(li().text("Two cups")))
            .child(
                dialog_footer()
                    .child(CART.close(ButtonVariant::Default, ButtonSize::Default).text("Done")),
            ),
        &DialogText { close: "Close the cart".into() },
    ))
    .render();
# assert!(html.contains(r#"closedby="any""#));
# assert!(html.contains(r#"aria-label="Close the cart""#));
# }
