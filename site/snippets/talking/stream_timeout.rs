use std::time::Duration;

use hypersig::html::div;
use hypersig::{Patches, Signals, StreamAction};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    items: Vec<String>,
}

hypersig::routes! {
    fn pages(Router<()>);
    stream UPDATES: StreamAction<Feed> = "/updates" => |(), ()| {
        futures::stream::empty::<Patches<Feed>>()
    };
}

// error[E0599]: no method named `timeout` found for struct `StreamCall<S>` in the current scope
let feed = div::<Feed>().on_load(UPDATES.call().timeout(Duration::from_secs(5)));
# let _ = (feed, pages);
