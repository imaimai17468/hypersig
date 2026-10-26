// A script's URL decides what code the page runs, so an asset refuses a scheme that runs a script.
use hypersig::Asset;

const PAYLOAD: Asset = Asset::new("javascript:alert(1)");

fn main() {
    let _ = PAYLOAD;
}
