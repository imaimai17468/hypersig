use hypersig::{Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Address {
    city: String,
    zip: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Account {
    name: String,
    address: Address,
}

// A field inside a field, set on its own: `zip` and `name` keep their values.
let city: Patches<Account> = Account::address()
    .at(Address::city())
    .patch("Kyoto".to_owned())
    .into();
assert_eq!(city.value_of(Account::address().at(Address::city())), Some("Kyoto".to_owned()));
assert_eq!(city.value_of(Account::name()), None);

// `all_signals` sends every field of the struct.
let account = Account {
    name: "Ada".to_owned(),
    address: Address { city: "Kyoto".to_owned(), zip: "600-8216".to_owned() },
};
let all = Patches::new().all_signals(&account);
assert_eq!(all.value_of(Account::name()), Some("Ada".to_owned()));
