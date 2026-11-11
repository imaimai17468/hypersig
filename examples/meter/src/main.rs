//! A microphone level meter in a desktop window, whose page may use the
//! microphone because the application allows it with `App::allow`.
//!
//! Run with `cargo run -p example-meter`. On macOS the web view offers the
//! microphone only to a bundle whose `Info.plist` has
//! `NSMicrophoneUsageDescription`, so build one with
//! `cargo packager -p example-meter --release` and open it; the key comes
//! from this directory's `Info.plist`.

use axum::Router;
use axum::http::header;
use axum::routing::get;
use hypersig::html::{CustomEvent, Stylesheet, el, h1, p};
use hypersig::{Asset, Page, Route, RouterExt, Signals};
use hypersig_desktop::{App, Device};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Meter {
    state: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

/// The script that defines `<mic-level>`, served next to the page.
const METER_JS: Asset = Asset::new("/meter.js");

/// `meter.js` dispatches it with "listening", or with the name of the error
/// the microphone was refused with.
const STATE: CustomEvent<String> = CustomEvent::new("state");

const STYLE: Stylesheet = Stylesheet::new(
    "
body { font-family: system-ui, sans-serif; max-width: 32rem; margin: 3rem auto; padding: 0 1rem; }
mic-level { display: block; block-size: 1.5rem; border: 1px solid #888; }
mic-level .bar { block-size: 100%; inline-size: 0; background: seagreen; }
",
);

async fn index((): (), (): ()) -> Page<Meter> {
    let state = Meter::state();
    Page::new(
        "hypersig meter",
        Meter {
            state: "Opening the microphone".to_owned(),
        },
    )
    .style(STYLE)
    .script(METER_JS)
    .child(h1().text("Microphone"))
    .child(el(hypersig::tag!("mic-level")).on_custom(STATE, move |heard| state.set(heard)))
    .child(p().bind_text(state))
}

const APP: App = App::new("dev.hypersig.example-meter").allow(Device::Microphone);

fn main() -> Result<(), hypersig_desktop::Error> {
    APP.size(560, 320).run(&INDEX.url(()), |_os| async {
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
    })
}
