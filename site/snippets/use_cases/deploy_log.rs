# use futures::{Stream, stream};
# use hypersig::html::{div, each, li, ol};
# use hypersig::{Patches, Signals, StreamAction};
# use serde::{Deserialize, Serialize};
#
# #[derive(Signals, Serialize, Deserialize, Clone)]
# struct Line {
#     id: i32,
#     text: String,
# }
#
#[derive(Signals, Serialize, Deserialize)]
struct Deploy {
    lines: Vec<Line>,
    last: i32, //+
}

# #[derive(Clone)]
# struct AppState;
#
# impl AppState {
#     /// The lines the deploy has written after line `after`, waiting for one.
#     async fn lines_after(&self, after: i32) -> Vec<Line> {
#         vec![Line { id: after + 1, text: "Building…".to_owned() }]
#     }
# }
#
hypersig::routes! {
    fn pages(Router<AppState>);
//-    stream LOG: StreamAction<Deploy> = "/deploy/log" => log;
    stream LOG: StreamAction<Deploy, i32> = "/deploy/log" => log; //+
}

# let _ = div()
//-    .on_load(LOG.call().reconnect())
    // Each reconnection sends the id of the last line on screen. //+
    .on_load(LOG.call_with(Deploy::last()).reconnect()) //+
    .child(ol().child(each(Deploy::lines(), |line| li().bind_text(line.at(Line::text())))));

//-fn log(state: AppState, (): ()) -> impl Stream<Item = Patches<Deploy>> {
//-    stream::unfold(0, move |after| {
fn log(state: AppState, last: i32) -> impl Stream<Item = Patches<Deploy>> { //+
    stream::unfold(last, move |after| {
        let state = state.clone();
        async move {
            let lines = state.lines_after(after).await;
            let next = lines.last().map_or(after, |line| line.id);
//-            Some((Deploy::lines().patch_append(&lines).into(), next))
            let patches = Patches::new() //+
                .list(Deploy::lines().patch_append(&lines)) //+
                .signals(Deploy::last().patch(next)); //+
            Some((patches, next)) //+
        }
    })
}
# let _ = pages;
