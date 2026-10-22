//! A counter that runs as a Vercel Function, with a clock the function streams.
//!
//! The page, its action, and its stream are the same hypersig code an axum
//! server runs. `VercelLayer` hands each request Vercel sends to the axum
//! `Router`.
//!
//! Run `cargo run` and open <http://localhost:3000>, and publish it with
//! `npx vercel deploy`. Vercel may run more than one instance of the function,
//! so a value kept in memory may not reach the next request. Keep shared state
//! in a database.

use std::time::Duration;

use axum::Router;
use futures::{Stream, stream};
use hypersig::html::{Stylesheet, button, div, h1, p, span};
use hypersig::{
    Action, DateFormat, DateStyle, IntlLocale, Page, Patches, Route, RouterExt, Signals,
    StreamAction, TimeStyle, Timestamp,
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use vercel_runtime::axum::VercelLayer;

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
    now: Timestamp,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

const DOUBLE: Action<Counter, i32> = Action::post("/double");
const CLOCK: StreamAction<Counter> = StreamAction::new("/clock");

const TIME: DateFormat =
    DateFormat::date_time(IntlLocale::En, DateStyle::Medium, TimeStyle::Medium);

const STYLE: Stylesheet = Stylesheet::new(
    "
body { font-family: system-ui, sans-serif; max-width: 32rem; margin: 3rem auto; padding: 0 1rem; }
button { font-size: 1rem; padding: .4rem .9rem; }
",
);

async fn index((): (), (): ()) -> Page<Counter> {
    let count = Counter::count();
    let initial = Counter {
        count: 0,
        now: Timestamp::now(),
    };
    Page::new("hypersig on Vercel", initial).style(STYLE).child(
        div()
            .on_load(CLOCK.call().reconnect())
            .child(h1().text("Counter"))
            .child(
                p().child(button().on_click(count.set(count + 1)).text("+1"))
                    .child(span().bind_text(count.to_text()))
                    .child(
                        button()
                            .on_click(DOUBLE.call_with(count))
                            .text("Double on the function"),
                    ),
            )
            .child(p().bind_text(Counter::now().format_date(&TIME))),
    )
}

async fn double((): (), count: i32) -> Patches<Counter> {
    Counter::count().patch(count.saturating_mul(2)).into()
}

/// The function's time, once a second.
fn clock((): (), (): ()) -> impl Stream<Item = Patches<Counter>> {
    stream::unfold((), |()| async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Some((Counter::now().patch(Timestamp::now()).into(), ()))
    })
}

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(DOUBLE, double)
        .stream(CLOCK, clock);
    // Outside Vercel, `run` listens on 127.0.0.1 at the port in
    // `VERCEL_DEV_PORT`, or 3000.
    vercel_runtime::run(ServiceBuilder::new().layer(VercelLayer::new()).service(app)).await
}
