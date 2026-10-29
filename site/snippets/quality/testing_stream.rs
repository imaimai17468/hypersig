use futures::{Stream, StreamExt as _, stream};
use hypersig::{Patches, Signals, StreamAction};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Launch {
    left: i32,
}

hypersig::routes! {
    fn streams(Router<()>);
    stream COUNTDOWN: StreamAction<Launch, i32> = "/countdown" => countdown;
}

fn countdown((): (), from: i32) -> impl Stream<Item = Patches<Launch>> {
    stream::iter((0..from).rev()).map(|left| Launch::left().patch(left).into())
}

# tokio::runtime::Runtime::new().unwrap().block_on(async {
// The body of a `#[tokio::test] async fn`.
let batches: Vec<Patches<Launch>> = countdown((), 3).collect().await;
let left: Vec<Option<i32>> = batches.iter().map(|batch| batch.value_of(Launch::left())).collect();
assert_eq!(left, [Some(2), Some(1), Some(0)]);
# });
# let _ = streams;
