// A stylesheet's URL is text written in the program, so it refuses a scheme that runs a script where it is built.
use hypersig::style::value::{Url, url};

const PAYLOAD: Url = url("javascript:alert(1)");

fn main() {
    let _ = PAYLOAD;
}
