//! A counter that runs on Cloudflare Workers, with a clock the worker streams.
//!
//! The page, its action, and its stream are the same hypersig code an axum
//! server runs. The worker hands each request to the axum `Router`.
//!
//! Run `npx wrangler dev` and open <http://localhost:8787>, and publish it
//! with `npx wrangler deploy`. Each request may run in a different isolate,
//! so a value kept in memory does not reach the next request. Keep shared
//! state in KV, D1, or a Durable Object.

use std::time::Duration;

use axum::Router;
use futures::{Stream, stream};
use hypersig::html::{Stylesheet, button, div, h1, p, span};
use hypersig::{
    Action, DateFormat, DateStyle, IntlLocale, Page, Patches, Route, RouterExt, Signals,
    StreamAction, TimeStyle, Timestamp,
};
use serde::{Deserialize, Serialize};
use tower_service::Service as _;
use worker::send::SendFuture;
use worker::{Context, Delay, Env, HttpRequest, event};

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
    Page::new("hypersig on Cloudflare Workers", initial)
        .style(STYLE)
        .child(
            div()
                .on_load(CLOCK.call().reconnect())
                .child(h1().text("Counter"))
                .child(
                    p().child(button().on_click(count.set(count + 1)).text("+1"))
                        .child(span().bind_text(count.to_text()))
                        .child(
                            button()
                                .on_click(DOUBLE.call_with(count))
                                .text("Double on the worker"),
                        ),
                )
                .child(p().bind_text(Counter::now().format_date(&TIME))),
        )
}

async fn double((): (), count: i32) -> Patches<Counter> {
    Counter::count().patch(count.saturating_mul(2)).into()
}

/// The worker's time, once a second.
fn clock((): (), (): ()) -> impl Stream<Item = Patches<Counter>> {
    stream::unfold((), |()| async {
        // `Delay` waits on a JavaScript timer, which is not `Send`. A worker
        // runs on one thread, so the wrapper never moves it to another.
        SendFuture::new(Delay::from(Duration::from_secs(1))).await;
        Some((Counter::now().patch(Timestamp::now()).into(), ()))
    })
}

#[event(fetch)]
async fn fetch(
    request: HttpRequest,
    _env: Env,
    _context: Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    let mut app = pages(Router::new())
        .hypersig_runtime()
        .action(DOUBLE, double)
        .stream(CLOCK, clock);
    Ok(app.call(request).await?)
}
