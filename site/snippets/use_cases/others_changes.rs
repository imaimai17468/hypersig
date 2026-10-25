# use axum::Router;
use futures::{Stream, StreamExt as _, stream}; //+
use hypersig::html::{div, each, li, ul};
use hypersig::{Action, Page, Patches, Signals, StreamAction};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
#
# #[derive(Signals, Serialize, Deserialize, Clone)]
# struct Task {
#     title: String,
# }
#
# #[derive(Signals, Serialize, Deserialize)]
# struct Board {
#     tasks: Vec<Task>,
# }
#
# #[derive(Clone)]
# struct AppState {
#     changed: broadcast::Sender<Vec<Task>>,
# }
#
# impl AppState {
#     fn add(&self, title: String) -> Vec<Task> {
#         vec![Task { title }]
#     }
#
#     fn list(&self) -> Vec<Task> {
#         Vec::new()
#     }
# }

hypersig::routes! {
    fn pages(Router<AppState>);
    action ADD: Action<Board, String> = post "/tasks" => add;
    stream UPDATES: StreamAction<Board> = "/tasks/updates" => updates; //+
}

let page = Page::new("Board", Board { tasks: Vec::new() }).child(
    div()
        .on_load(UPDATES.call().reconnect()) //+
        .child(ul().child(each(Board::tasks(), |task| li().bind_text(task.at(Task::title()))))),
);

async fn add(state: AppState, title: String) -> Patches<Board> {
    let tasks = state.add(title);
//-    Board::tasks().patch(tasks).into()
    let _ = state.changed.send(tasks); //+
    Patches::new() //+
}

// Each open tab holds one stream. It sends the list as it is now, which a //+
// tab that reconnects needs, then the list after each write. //+
fn updates(state: AppState, (): ()) -> impl Stream<Item = Patches<Board>> { //+
    let now = state.list(); //+
    let changes = stream::unfold(state.changed.subscribe(), |mut changes| async move { //+
        let tasks = changes.recv().await.ok()?; //+
        Some((tasks, changes)) //+
    }); //+
    stream::iter([now]).chain(changes).map(|tasks| Board::tasks().patch(tasks).into()) //+
} //+

let (changed, _) = broadcast::channel(16);
let _app: Router = pages(Router::new()).with_state(AppState { changed });
# let _ = page.render();
