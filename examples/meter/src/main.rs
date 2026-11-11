//! A microphone level meter in a desktop window, with a push-to-talk key.
//!
//! The page may use the microphone because the application allows it with
//! `App::allow`, and it shows "Talking" while the person holds
//! Control+Shift+Space (Command+Shift+Space on macOS), which the application
//! hears through `Os::shortcut` even while another application has focus.
//!
//! Run with `cargo run -p example-meter`. On macOS the web view offers the
//! microphone only to a bundle whose `Info.plist` has
//! `NSMicrophoneUsageDescription`, so build one with
//! `cargo packager -p example-meter --release` and open it; the key comes
//! from this directory's `Info.plist`.

use axum::Router;
use axum::http::header;
use axum::routing::get;
use futures::{Stream, StreamExt as _};
use hypersig::html::{CustomEvent, Stylesheet, div, el, h1, p};
use hypersig::{Asset, Page, Patches, Route, RouterExt, Signals, StreamAction, if_else};
use hypersig_desktop::{App, Device, Key, Modifiers, Press, Shortcut};
use serde::{Deserialize, Serialize};
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

#[derive(Signals, Serialize, Deserialize)]
struct Meter {
    state: String,
    talking: bool,
    hint: String,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
    stream TALKING: StreamAction<Meter> = "/talking" => talking;
}

#[derive(Clone)]
struct AppState {
    /// Whether the push-to-talk key is held, which each page's stream follows.
    talking: watch::Receiver<bool>,
    /// What the page says while the key is not held: how to talk, or why
    /// push-to-talk is off.
    hint: String,
}

/// The script that defines `<mic-level>`, served next to the page.
const METER_JS: Asset = Asset::new("/meter.js");

/// `meter.js` dispatches it with "listening", or with the name of the error
/// the microphone was refused with.
const STATE: CustomEvent<String> = CustomEvent::new("state");

/// The push-to-talk key.
const TALK: Shortcut = Shortcut::new(Modifiers::PRIMARY.and(Modifiers::SHIFT), Key::Space);

const STYLE: Stylesheet = Stylesheet::new(
    "
body { font-family: system-ui, sans-serif; max-width: 32rem; margin: 3rem auto; padding: 0 1rem; }
mic-level { display: block; block-size: 1.5rem; border: 1px solid #888; }
mic-level .bar { block-size: 100%; inline-size: 0; background: seagreen; }
",
);

async fn index(app: AppState, (): ()) -> Page<Meter> {
    let state = Meter::state();
    let talking = Meter::talking();
    Page::new(
        "hypersig meter",
        Meter {
            state: "Opening the microphone".to_owned(),
            talking: false,
            hint: app.hint,
        },
    )
    .style(STYLE)
    .script(METER_JS)
    .child(
        div()
            .on_load(TALKING.call().reconnect())
            .child(h1().text("Microphone"))
            .child(el(hypersig::tag!("mic-level")).on_custom(STATE, move |heard| state.set(heard)))
            .child(p().bind_text(state))
            .child(p().bind_text(if_else(talking, "Talking", Meter::hint()))),
    )
}

/// Sends whether the push-to-talk key is held now, then each change.
fn talking(state: AppState, (): ()) -> impl Stream<Item = Patches<Meter>> {
    WatchStream::new(state.talking).map(|held| Meter::talking().patch(held).into())
}

const APP: App = App::new("dev.hypersig.example-meter").allow(Device::Microphone);

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.size(560, 360).run(&INDEX.url(()), |os| async move {
        let (held, talking) = watch::channel(false);
        let hint = match os.shortcut(TALK).await {
            Ok(mut presses) => {
                tokio::spawn(async move {
                    while let Some(press) = presses.next().await {
                        held.send_replace(press == Press::Pressed);
                    }
                });
                "Hold Control+Shift+Space (Command+Shift+Space on macOS) to talk".to_owned()
            }
            Err(error) => format!("Push-to-talk is off: {error}"),
        };
        pages(Router::new())
            .route(
                METER_JS.as_str(),
                get(|| async {
                    (
                        [(header::CONTENT_TYPE, "text/javascript")],
                        include_str!("meter.js"),
                    )
                }),
            )
            .hypersig_runtime()
            .with_state(AppState { talking, hint })
    })
}
