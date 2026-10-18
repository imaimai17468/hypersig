use axum::Router;
use futures::{Stream, stream};
use hypersig::{Action, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    items: Vec<String>,
}

// A stream is a `StreamAction`, which is always served on GET.
const UPDATES: Action<Feed> = Action::post("/updates");

fn updates((): (), (): ()) -> impl Stream<Item = Patches<Feed>> {
    stream::empty()
}

fn main() {
    let _: Router = Router::new().stream(UPDATES, updates);
}
