use axum::http::HeaderMap;
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Home {
    name: String,
}

// error[E0277]: `fn((), String, HeaderMap) -> impl Future<Output = Patches<Home>> {save}` cannot serve this page or action
hypersig::routes! {
    fn pages(Router<()>);
    action SAVE: Action<Home, String> = post "/save" => save;
}

// The extractor comes after the arguments, where the router expects it before them.
async fn save((): (), name: String, _headers: HeaderMap) -> Patches<Home> {
    Home::name().patch(name).into()
}
# let _ = pages;
