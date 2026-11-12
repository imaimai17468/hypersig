use hypersig::html::{El, button, div, span};
use axum::Router;
use hypersig::{Action, Patches, Signals};
use hypersig_desktop::{App, Frame, MovesWindow as _};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Notes {
    title: String,
}
# hypersig::routes! {
#     fn pages(Router<()>);
#     action CLOSE: Action<Notes> = post "/close" => close;
# }
# async fn close((): (), (): ()) -> Patches<Notes> { Patches::new() }

const APP: App = App::new("dev.example.notes").frame(Frame::None);

/// The title bar the page draws: dragging it moves the window, and its
/// button keeps its clicks.
fn title_bar() -> El<Notes> {
    div()
        .moves_window()
        .child(span().bind_text(Notes::title()))
        .child(button().on_click(CLOSE.call()).text("Close"))
}
# fn main() {
# let _ = (APP, pages(Router::new()));
# assert!(title_bar().render().contains("data-hypersig-moves-window"));
# }
