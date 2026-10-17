use hypersig::html::a;
use hypersig::{Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Team {
    lead: i32,
}

#[derive(Signals, Serialize, Deserialize)]
struct Other {
    id: i32,
}

const MEMBER: Route<i32> = Route::new("/members/{id}");

fn main() {
    let _ = a::<Team>().href(MEMBER.link_with(Other::id()));
}
