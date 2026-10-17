//! A route declared outside `routes!`, which would have no handler.

use hypersig::Route;

const ORPHAN: Route = Route::new("/orphan");

fn main() {
    let _ = ORPHAN;
}
