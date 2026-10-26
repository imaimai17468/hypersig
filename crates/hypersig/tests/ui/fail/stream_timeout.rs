use std::time::Duration;

use hypersig::html::div;
use hypersig::{Patches, Signals, StreamAction};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    live: bool,
}

hypersig::routes! {
    fn actions(Router<()>);
    stream UPDATES: StreamAction<Feed> = "/updates" => |(), ()| futures::stream::empty::<Patches<Feed>>();
}

fn main() {
    // A stream stays open until its handler ends it, so it has no time limit.
    let _ = div::<Feed>().on_load(UPDATES.call().timeout(Duration::from_secs(5)));
}
