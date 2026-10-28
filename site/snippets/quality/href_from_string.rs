use hypersig::Signals;
use hypersig::html::a;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    website: String,
}

// error[E0277]: `Sig<Profile, String>` is not a URL an `href` or a `src`
// takes on the page whose signals are `Profile`
let link = a::<Profile>().href(Profile::website());
