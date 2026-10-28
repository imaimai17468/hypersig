# use serde::{Deserialize, Serialize};
use hypersig::html::a;
use hypersig::{SafeUrl, Signals};

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    // Deserializing the signals checks this field as `SafeUrl::parse` does.
    website: SafeUrl,
}

// In a `const`, a URL whose scheme can run a script fails to compile.
const SOURCE: SafeUrl = SafeUrl::new("https://github.com/imaimai17468/hypersig");
const TOP: SafeUrl = SafeUrl::new("#top");

let source = a::<Profile>().href(SOURCE).text("Source");
let back = a::<Profile>().href(TOP).text("Back to top");

// A URL a visitor typed is checked when it arrives.
let typed = SafeUrl::parse("https://example.com/~ada").expect("an https URL");
assert!(SafeUrl::parse("javascript:alert(1)").is_err());
let theirs = a::<Profile>().href(Profile::website()).text("Website");
# let _ = (source, back, typed, theirs);
