use hypersig::Signals;
use hypersig_ui::TextField;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    age: i32,
}

// A number signal bound to a field whose value is text.
let age = TextField::new("Age", Profile::age());
# let _ = age;
