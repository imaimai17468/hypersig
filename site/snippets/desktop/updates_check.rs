# use axum::Router;
# use hypersig::html::{button, p};
# use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use hypersig_desktop::{App, Os, Updates};
# use serde::{Deserialize, Serialize};

/// The public key `cargo packager signer generate` printed.
const PUBLIC_KEY: &str = "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEJFQjNFNjBDMjc0NTYxQTUKUldTbFlVVW5ET2F6dnJscS9QcVdGbVlLMTB6VWEzSzhndUlxbEtlL2NITk9ZVFV3dnk0TmpYaXQK";

const APP: App = App::new("dev.example.notes").updates(Updates::new(
    env!("CARGO_PKG_VERSION"),
    "https://updates.example.com/notes/{{target}}/{{arch}}/{{current_version}}",
    PUBLIC_KEY,
));

#[derive(Signals, Serialize, Deserialize)]
struct About {
    status: String,
}

#[derive(Clone)]
struct AppState {
    os: Os,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const ABOUT: Route = "/" => about;
    action UPDATE: Action<About> = post "/update" => update;
}

async fn about(_: AppState, (): ()) -> Page<About> {
    Page::new("About", About { status: String::new() })
        .child(button().on_click(UPDATE.call()).text("Check for updates"))
        .child(p().bind_text(About::status()))
}

async fn update(state: AppState, (): ()) -> Patches<About> {
    let status = About::status();
    let update = match state.os.updates().await {
        Ok(Some(update)) => update,
        Ok(None) => return status.patch("This is the newest version.".to_owned()).into(),
        Err(error) => return status.patch(format!("Not checked: {error}")).into(),
    };
    let version = update.version().to_owned();
    let downloaded = match update.download().await {
        Ok(downloaded) => downloaded,
        Err(error) => return status.patch(format!("Not downloaded: {error}")).into(),
    };
    match downloaded.install().await {
        Ok(()) => status.patch(format!("Version {version} runs from the next start.")).into(),
        Err(error) => status.patch(format!("Not installed: {error}")).into(),
    }
}

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.run(&ABOUT.url(()), |os| async move {
        pages(Router::new()).hypersig_runtime().with_state(AppState { os })
    })
}
