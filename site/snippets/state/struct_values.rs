use hypersig::html::{button, span};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Address {
    city: String,
    zip: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Account {
    home: Address,
    shipping: Address,
}

let home = Account::home();
let shipping = Account::shipping();

// One statement copies the whole struct.
let same_as_home = button::<Account>().on_click(shipping.set(home)).text("Same as home");
// `with` copies it with one field replaced.
let moved = button::<Account>()
    .on_click(home.set(home.with(Address::city(), "Osaka")))
    .text("Moved to Osaka");
let city = span::<Account>().bind_text(shipping.at(Address::city()));
# let address = |city: &str| Address { city: city.into(), zip: String::new() };
# let html = Page::new("Account", Account { home: address("Kyoto"), shipping: address("Nara") })
#     .child(same_as_home)
#     .child(moved)
#     .child(city)
#     .render();
# assert!(html.contains(">Nara</span>"));
