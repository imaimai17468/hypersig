use hypersig::{Sig, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
    label: String,
}

// The derive adds one function per field, named after the field.
let count: Sig<Counter, i32> = Counter::count();
let label: Sig<Counter, String> = Counter::label();

assert_eq!(count.path(), "count");
assert_eq!(label.path(), "label");
