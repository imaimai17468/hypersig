//! A counter whose increments happen in the browser and whose saves go to the server.
//!
//! Run with `cargo run -p example-counter` and open <http://localhost:3000>.

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use axum::Router;
use hypersig::html::{Stylesheet, attr, button, div, h1, input, label, p, span};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
    note: String,
    status: String,
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

const SAVE: Action<Counter, (i32, String)> = Action::post("/save");
const RESET: Action<Counter> = Action::post("/reset");

#[derive(Clone, Default)]
struct AppState {
    saved: Arc<AtomicI32>,
}

mod class {
    use hypersig::html::Class;

    use super::STYLE;

    pub const ROW: Class = STYLE.class("row");
    pub const COUNT: Class = STYLE.class("count");
    pub const BIG: Class = STYLE.class("big");
    pub const STATUS: Class = STYLE.class("status");
}

const STYLE: Stylesheet = Stylesheet::new(
    "
body { font-family: system-ui, sans-serif; max-width: 32rem; margin: 3rem auto; padding: 0 1rem; }
.row { display: flex; gap: .5rem; align-items: center; }
.count { font-size: 3rem; font-variant-numeric: tabular-nums; min-width: 4ch; text-align: center; }
.big { color: crimson; }
button { font-size: 1rem; padding: .4rem .9rem; }
input { font-size: 1rem; padding: .3rem; flex: 1; }
.status { color: #555; }
",
);

async fn index(state: AppState, (): ()) -> Page<Counter> {
    let count = Counter::count();
    let note = Counter::note();
    let initial = Counter {
        count: state.saved.load(Ordering::Relaxed),
        note: String::new(),
        status: String::new(),
    };
    Page::new("hypersig counter", initial).style(STYLE).child(
        div()
            .child(h1().text("Counter"))
            .child(
                div()
                    .class(class::ROW)
                    .child(button().on_click(count.set(count - 1)).text("-"))
                    .child(
                        span()
                            .class(class::COUNT)
                            .class_when(class::BIG, count.ge(10))
                            .bind_text(count.to_text()),
                    )
                    .child(button().on_click(count.set(count + 1)).text("+")),
            )
            .child(
                div().class(class::ROW).child(label().text("Note")).child(
                    input()
                        .attr(attr::PLACEHOLDER, "why this number?")
                        .bind(note),
                ),
            )
            .child(
                p().show(!note.is_empty())
                    .bind_text(note.concat(" → ").concat(count.to_text())),
            )
            .child(
                div()
                    .class(class::ROW)
                    .child(
                        button()
                            .on_click(SAVE.call_with((count, note)))
                            .text("Save to server"),
                    )
                    .child(
                        button()
                            .show(count.not_equals(0))
                            .on_click(RESET.call())
                            .text("Reset"),
                    ),
            )
            .child(p().class(class::STATUS).bind_text(Counter::status())),
    )
}

async fn save(state: AppState, (count, note): (i32, String)) -> Patches<Counter> {
    state.saved.store(count, Ordering::Relaxed);
    let status = if note.is_empty() {
        format!("Saved {count}.")
    } else {
        format!("Saved {count} ({note}).")
    };
    Counter::status().patch(status).into()
}

async fn reset(state: AppState, (): ()) -> Patches<Counter> {
    state.saved.store(0, Ordering::Relaxed);
    Patches::new().signals(
        Counter::count()
            .patch(0)
            .and(Counter::note().patch(String::new()))
            .and(Counter::status().patch("Reset by the server.".to_owned())),
    )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(SAVE, save)
        .action(RESET, reset)
        .with_state(AppState::default());
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}
