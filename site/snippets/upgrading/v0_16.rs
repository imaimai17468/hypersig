# use axum::Router;
# use hypersig::html::button;
# use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use hypersig_desktop::{App, Updater, Updates};
# use serde::{Deserialize, Serialize};
#
# const PUBLIC_KEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEJFQjNFNjBDMjc0NTYxQTUKUldTbFlVVW5ET2F6dnJscS9QcVdGbVlLMTB6VWEzSzhndUlxbEtlL2NITk9ZVFV3dnk0TmpYaXQK";
#
# #[derive(Signals, Serialize, Deserialize)]
# struct About {}
#
# hypersig::routes! {
#     fn pages(Router<Updater>);
#     const ABOUT: Route = "/" => about;
#     action UPDATE: Action<About> = post "/update" => update;
# }
#
# async fn about(_: Updater, (): ()) -> Page<About> {
#     Page::new("About", About {}).child(button().on_click(UPDATE.call()).text("Update"))
# }

//-const APP: App = App::new("dev.example.notes").updates(Updates::new(
const APP: App<Updates> = App::new("dev.example.notes").updates(Updates::new( //+
    env!("CARGO_PKG_VERSION"),
    "https://updates.example.com/notes/{{target}}/{{arch}}/{{current_version}}",
    PUBLIC_KEY,
));

async fn update(updater: Updater, (): ()) -> Patches<About> {
//-    let found = state.os.updates().await;
    let found = updater.check().await; //+
    let _ = found;
    Patches::new()
}

fn main() -> Result<(), hypersig_desktop::Error> {
//-    APP.run(&ABOUT.url(()), |os| async move {
    APP.run(&ABOUT.url(()), |_os, updater| async move { //+
        pages(Router::new()).hypersig_runtime().with_state(updater)
    })
}
