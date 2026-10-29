use hypersig::html::{div, p};
use hypersig::{CharClass, Expr, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Signup {
    name: String,
    zip: String,
}

let name = Signup::name();
let zip = Signup::zip();

let greeting = Expr::text("Hello, ").concat(name.trim()).concat("!");
let valid_zip = zip.char_count().equals(7).and(zip.all_chars(CharClass::AsciiDigit));

let form = div::<Signup>()
    .child(p().show(!name.trim().is_empty()).bind_text(greeting))
    .child(p().show(!valid_zip).text("Seven digits, without a hyphen."));
# let _ = form;
