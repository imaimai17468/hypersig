//! An application that finds a newer version of itself, downloads it,
//! checks its signature, and installs it over the running one.
//!
//! Run with `cargo run -p example-update`. It looks for updates where
//! `HYPERSIG_UPDATE_ENDPOINT` said when it was built, and checks them with
//! the key in `HYPERSIG_UPDATE_PUBLIC_KEY`. Built without them, it asks an
//! example endpoint that does not answer. CI builds a signed `0.2.0`
//! `AppImage`, serves it on `127.0.0.1`, and has a `0.1.0` build install it.

use axum::Router;
use hypersig::html::{button, h1, p};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use hypersig_desktop::{App, Updater, Updates};
use serde::{Deserialize, Serialize};

/// Where the new versions are announced.
const ENDPOINT: &str = match option_env!("HYPERSIG_UPDATE_ENDPOINT") {
    Some(endpoint) => endpoint,
    None => "https://updates.example.com/update/{{target}}/{{arch}}/{{current_version}}",
};

/// The public key that checks the packages, as `cargo packager signer
/// generate` writes it. The default's private key is not kept anywhere.
const PUBLIC_KEY: &str = match option_env!("HYPERSIG_UPDATE_PUBLIC_KEY") {
    Some(key) => key,
    None => {
        "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEJFQjNFNjBDMjc0NTYxQTUKUldTbFlVVW5ET2F6dnJscS9QcVdGbVlLMTB6VWEzSzhndUlxbEtlL2NITk9ZVFV3dnk0TmpYaXQK"
    }
};

const APP: App<Updates> = App::new("dev.hypersig.example-update").updates(Updates::new(
    env!("CARGO_PKG_VERSION"),
    ENDPOINT,
    PUBLIC_KEY,
));

#[derive(Signals, Serialize, Deserialize)]
struct About {
    status: String,
}

hypersig::routes! {
    fn pages(Router<Updater>);
    const ABOUT: Route = "/" => about;
    action UPDATE: Action<About> = post "/update" => update;
}

async fn about(_: Updater, (): ()) -> Page<About> {
    Page::new(
        "hypersig update",
        About {
            status: String::new(),
        },
    )
    .child(h1().text(concat!("Version ", env!("CARGO_PKG_VERSION"))))
    .child(button().on_click(UPDATE.call()).text("Check for updates"))
    .child(p().bind_text(About::status()))
}

/// Finds, downloads, and installs a newer version, and says how far it got.
async fn update(updater: Updater, (): ()) -> Patches<About> {
    let status = About::status();
    let update = match updater.check().await {
        Ok(Some(update)) => update,
        Ok(None) => {
            return status
                .patch("This is the newest version.".to_owned())
                .into();
        }
        Err(error) => return status.patch(format!("Not checked: {error}")).into(),
    };
    let version = update.version().to_owned();
    let downloaded = match update.download().await {
        Ok(downloaded) => downloaded,
        Err(error) => return status.patch(format!("Not downloaded: {error}")).into(),
    };
    match downloaded.install().await {
        Ok(()) => status
            .patch(format!("Version {version} runs from the next start."))
            .into(),
        Err(error) => status.patch(format!("Not installed: {error}")).into(),
    }
}

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.run(&ABOUT.url(()), |_os, updater| async move {
        pages(Router::new()).hypersig_runtime().with_state(updater)
    })
}
