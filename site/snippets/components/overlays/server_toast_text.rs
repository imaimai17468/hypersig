use hypersig::{Patches, Signals};
use hypersig_ui::Toast;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {
    toasts: Vec<Toast>,
}

async fn save((): (), (): ()) -> Patches<App> {
    // The list holds toasts, so a bare message is not one.
    App::toasts().patch_append(&["Saved"]).into()
}
