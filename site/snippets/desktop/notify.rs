# use axum::Router;
# use hypersig::{Page, Patches, Route, RouterExt, Signals};
# use hypersig_desktop::testing::FakeOs;
# use serde::{Deserialize, Serialize};
use hypersig::Action;
use hypersig_desktop::{Notification, Os};

#[derive(Signals, Serialize, Deserialize)]
struct Chat {
    status: String,
}

#[derive(Clone)]
struct AppState {
    os: Os,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const CHAT: Route = "/" => chat;
    action RECEIVE: Action<Chat, String> = post "/receive" => receive;
}

# async fn chat(_: AppState, (): ()) -> Page<Chat> {
#     Page::new("Chat", Chat { status: String::new() })
# }
/// Tells the person about a message, and brings a window forward if they click it.
async fn receive(state: AppState, message: String) -> Patches<Chat> {
    let notification = Notification::new("Ada").body(message);
    match state.os.notify(notification).await {
        Ok(shown) => {
            // `clicked` waits for the person, so a task waits for it.
            tokio::spawn(async move {
                if shown.clicked().await {
                    // Open or focus the chat's window here.
                }
            });
            Patches::new()
        }
        Err(error) => Chat::status().patch(format!("Not shown: {error}")).into(),
    }
}
# fn main() {
# let _ = pages(Router::new()).hypersig_runtime();
# tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
# let fake = FakeOs::new();
# let _ = receive(AppState { os: fake.os() }, "Lunch?".to_owned()).await;
# assert_eq!(fake.notifications()[0].text(), Some("Lunch?"));
# });
# }
