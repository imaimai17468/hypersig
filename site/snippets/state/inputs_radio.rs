use hypersig::html::{div, input, label};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
enum Shipping {
    Standard,
    Express,
}

#[derive(Signals, Serialize, Deserialize)]
struct Checkout {
    shipping: Shipping,
}

let shipping = Checkout::shipping();

let choice = div::<Checkout>()
    .child(label().child(input().bind_radio(shipping, &Shipping::Standard)).text("Standard"))
    .child(label().child(input().bind_radio(shipping, &Shipping::Express)).text("Express"));
# let html = Page::new("Checkout", Checkout { shipping: Shipping::Standard }).child(choice).render();
# assert!(html.contains(r#"type="radio" checked"#), "{html}");
