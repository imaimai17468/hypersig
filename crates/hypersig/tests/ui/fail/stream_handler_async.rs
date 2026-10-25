//! A stream's handler written as an `async fn`, which returns a future of the stream.

use axum::Router;
use futures::{Stream, stream};
use hypersig::{Patches, RouterExt, Signals, StreamAction};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    items: Vec<String>,
}

const UPDATES: StreamAction<Feed> = StreamAction::new("/updates");

async fn updates((): (), (): ()) -> impl Stream<Item = Patches<Feed>> {
    stream::empty()
}

fn main() {
    let _: Router = Router::new().stream(UPDATES, updates);
}
