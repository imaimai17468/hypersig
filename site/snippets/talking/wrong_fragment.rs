use hypersig::html::{El, Id, ul};
use hypersig::{Fragment, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    total: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Account {}

// A fragment of the account page.
struct Orders;

impl Fragment for Orders {
    type Signals = Account;
    const ID: Id = Id::new("orders");

    fn render(&self) -> El<Account> {
        ul()
    }
}

// error[E0271]: type mismatch resolving `<Orders as Fragment>::Signals == Shop`
let patches = Patches::new()
    .signals(Shop::total().patch("¥1,200".to_owned()))
    .element(&Orders);
# let _ = patches;
