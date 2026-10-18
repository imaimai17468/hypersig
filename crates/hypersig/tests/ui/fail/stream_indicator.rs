use hypersig::html::div;
use hypersig::{Signals, StreamAction};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    live: bool,
}

const UPDATES: StreamAction<Feed> = StreamAction::new("/updates");

fn main() {
    // A stream's flag says whether its answer is open, which `connected` names.
    let _ = div::<Feed>().on_load(UPDATES.call().indicator(Feed::live()));
}
