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

// error[E0308]: mismatched types: expected `Sig<Address, _>`, found `Sig<Account, String>`
let name = Account::address().at(Account::name());
