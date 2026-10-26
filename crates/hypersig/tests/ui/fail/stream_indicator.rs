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
    // A stream's flag says whether its answer is open, which `connected` names.
    let _ = div::<Feed>().on_load(UPDATES.call().indicator(Feed::live()));
}
