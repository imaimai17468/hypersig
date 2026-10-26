use hypersig::Signals;
use hypersig_ui::Accordion;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Faq {}

// A group name written by hand could match another accordion's, and the two
// would close each other's sections, so the group is an `Id`, which the page
// checks for a second element carrying it.
fn main() {
    let _ = Accordion::<Faq>::exclusive("faq");
}
