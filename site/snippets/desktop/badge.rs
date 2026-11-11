# use axum::Router;
# use hypersig::{Page, Patches, Route, RouterExt, Signals};
# use hypersig::html::button;
# use hypersig_desktop::testing::FakeOs;
# use serde::{Deserialize, Serialize};
use hypersig::Action;
use hypersig_desktop::Os;

#[derive(Signals, Serialize, Deserialize)]
struct Inbox {
    unread: u32,
}

#[derive(Clone)]
struct AppState {
    os: Os,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INBOX: Route = "/" => inbox;
    action READ_ALL: Action<Inbox> = post "/read-all" => read_all;
}

# async fn inbox(_: AppState, (): ()) -> Page<Inbox> {
#     Page::new("Inbox", Inbox { unread: 3 })
#         .child(button().on_click(READ_ALL.call()).text("Mark all read"))
# }
/// Marks every message read, and clears the count on the icon.
async fn read_all(state: AppState, (): ()) -> Patches<Inbox> {
    if let Err(error) = state.os.badge(0) {
        eprintln!("the badge was not cleared: {error}");
    }
    Inbox::unread().patch(0).into()
}
# fn main() {
# let _ = pages(Router::new()).hypersig_runtime();
# tokio::runtime::Builder::new_current_thread().build().unwrap().block_on(async {
# let fake = FakeOs::new();
# fake.os().badge(3).unwrap();
# let _ = read_all(AppState { os: fake.os() }, ()).await;
# assert_eq!(fake.badge(), 0);
# });
# }
