use hypersig::html::span;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Address {
    city: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Account {
    name: String,
    address: Address,
}

// `at` reaches a field of the struct a signal holds.
let city = Account::address().at(Address::city());
assert_eq!(city.path(), "address.city");

let line = span::<Account>().bind_text(city);
# let _ = line;
