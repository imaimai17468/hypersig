# use axum::Router;
# use hypersig::{Page, Route, RouterExt, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct ListPage {}
# #[derive(Clone)]
# struct AppState { os: Os }
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     const INDEX: Route = "/" => index;
# }
# async fn index(state: AppState, (): ()) -> Page<ListPage> { let _ = state.os; Page::new("Notes", ListPage {}) }
use hypersig_desktop::{App, Os, Tray, TrayItem};

const APP: App = App::new("dev.example.notes");
const ICON: &[u8] = include_bytes!("../../../examples/notes/icons/icon.png");

/// Opens the list again each time "Show notes" is clicked.
async fn show_on_click(os: Os, show: TrayItem) {
    let mut clicks = show.clicks();
    while clicks.next().await {
        if os.windows().open(&INDEX.url(())).await.is_err() {
            return;
        }
    }
}

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.run(&INDEX.url(()), |os| async move {
        let show = TrayItem::new("Show notes");
        let tray = Tray::new(ICON, "Quit notes").tooltip("Notes").item(&show);
        tokio::spawn(show_on_click(os.clone(), show));
        os.tray(tray).await?;
        Ok::<_, hypersig_desktop::Error>(pages(Router::new()).hypersig_runtime().with_state(AppState { os }))
    })
}
