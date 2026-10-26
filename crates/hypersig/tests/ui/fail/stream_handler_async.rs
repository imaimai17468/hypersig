//! A stream's handler written as an `async fn`, which returns a future of the stream.

use futures::{Stream, stream};
use hypersig::{Patches, Signals, StreamAction};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    items: Vec<String>,
}

hypersig::routes! {
    fn actions(Router<()>);
    stream UPDATES: StreamAction<Feed> = "/updates" => updates;
}

async fn updates((): (), (): ()) -> impl Stream<Item = Patches<Feed>> {
    stream::empty()
}

fn main() {
    let _ = actions;
}
