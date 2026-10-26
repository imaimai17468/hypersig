//! A counter whose increments happen in the browser and whose reset goes to the server.
//!
//! Run it with `cargo run` and open <http://localhost:3000>.
//! `cargo hypersig dev` runs it too, and starts it again after each change.
//! Build the container image with `docker build -t {{project-name}} .` and
//! run it with `docker run --rm -p 3000:3000 {{project-name}}`.

use axum::Router;
use hypersig::html::{div, span};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
    action RESET: Action<Counter> = post "/reset" => reset;
}

// A handler takes the router's state first and the request's arguments
// second. This router has no state, `/` has no path parameters, and `RESET`
// takes no arguments, so both are `()`. With `.with_state(app)` on the
// router, the first is that `app`, and an action declared as
// `Action<Counter, i32>` receives the `i32` its call sends as the second.
async fn index((): (), (): ()) -> Page<Counter> {
    let count = Counter::count();
    Page::new("Counter", Counter { count: 0 })
        .global(hypersig_ui::global())
        .theme(hypersig_ui::theme())
        .child(
            div()
                .child(
                    button(ButtonVariant::Default, ButtonSize::Default)
                        .on_click(count.set(count + 1))
                        .text("+1"),
                )
                .child(span().bind_text(count.to_text()))
                .child(
                    button(ButtonVariant::Outline, ButtonSize::Default)
                        .on_click(RESET.call())
                        .text("Reset on the server"),
                ),
        )
}

async fn reset((): (), (): ()) -> Patches<Counter> {
    Counter::count().patch(0).into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new()).hypersig_runtime();
    // Listens on 127.0.0.1:3000, or, in the container, where the Dockerfile
    // sets `PORT`, on every interface at that port. Ctrl+C and the SIGTERM a
    // host sends stop it.
    hypersig::serve(app, 3000).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn the_page_starts_at_zero() {
        let page = index((), ()).await;
        assert_eq!(page.value_of(Counter::count()), 0);
    }

    #[tokio::test]
    async fn reset_sets_the_count_to_zero() {
        let patches = reset((), ()).await;
        assert_eq!(patches.value_of(Counter::count()), Some(0));
    }
}
