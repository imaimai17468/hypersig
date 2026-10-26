use futures::{Stream, stream};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    items: Vec<String>,
}

// A stream is a `StreamAction`, which is always served on GET.
hypersig::routes! {
    fn actions(Router<()>);
    stream UPDATES: Action<Feed> = "/updates" => updates;
}

fn updates((): (), (): ()) -> impl Stream<Item = Patches<Feed>> {
    stream::empty()
}

fn main() {
    let _ = actions;
}
