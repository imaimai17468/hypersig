# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
use hypersig::Route;
use hypersig::html::a;

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    team: String,
}

const MEMBER: Route<(String, i32)> = Route::new("/teams/{team}/members/{id}");

// Literal parameters give a fixed href, percent-encoded on the server.
let fixed = a::<Search>().href(MEMBER.link_with(("red team", 7)));
assert_eq!(fixed.render(), r#"<a href="/teams/red%20team/members/7"></a>"#);

// A parameter that reads a signal is kept up to date in the browser.
let bound = a().href(MEMBER.link_with((Search::team(), 7)));
assert!(bound.render().contains("data-hs-attr"));
