//! A clock the server pushes over a stream, for the page on talking to the
//! server.

use std::time::Duration;

use axum::{Router, response::Html, routing::get};
use futures::{Stream, stream};
use hypersig::html::{p, span};
use hypersig::{Action, Page, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Clock {
    seconds: i32,
}

const TICKS: Action<Clock> = Action::get("/ticks");

async fn index() -> Html<String> {
    let page = Page::new("Clock", Clock { seconds: 0 }).child(
        p().on_load(TICKS.call())
            .text("Connected for ")
            .child(span().bind_text(Clock::seconds().to_text()))
            .text(" s"),
    );
    Html(page.render())
}

fn ticks((): (), (): ()) -> impl Stream<Item = Patches<Clock>> {
    stream::unfold(0, |seconds| async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        let next = seconds + 1;
        Some((Clock::seconds().patch(next).into(), next))
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .hypersig_runtime()
        .stream(TICKS, ticks);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
