use axum::Router;
use hypersig::html::button;
use hypersig::{Action, Expr, Patches, RouterExt, Signals};
use hypersig_ui::{Toast, ToastVariant};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {
    toasts: Vec<Toast>,
}

const SAVE: Action<App> = Action::post("/save");

// The server adds its toast after the ones the browser already shows.
async fn save((): (), (): ()) -> Patches<App> {
    App::toasts()
        .patch_append(&[Toast::new("Saved").description("Your changes are live.")])
        .into()
}

// A failed call shows the server's message under the title.
let failed = Toast::new("Save failed").variant(ToastVariant::Destructive);
let call = SAVE.call().on_error(|message| {
    Toast::push_expr(App::toasts(), Expr::lit(&failed).with(Toast::body(), message))
});
let _save = button().on_click(call).text("Save");
let _app: Router = Router::new().action(SAVE, save);
