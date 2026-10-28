use futures::{Stream, stream};
use hypersig::{Patches, Signals, StreamAction};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Feed {
    items: Vec<String>,
}

// error[E0277]: `impl Future<Output = impl Stream<Item = Patches<Feed>>>` is not a stream of `Patches<Feed>` a stream's handler returns
hypersig::routes! {
    fn pages(Router<()>);
    stream UPDATES: StreamAction<Feed> = "/updates" => updates;
}

async fn updates((): (), (): ()) -> impl Stream<Item = Patches<Feed>> {
    stream::empty()
}
# let _ = pages;
