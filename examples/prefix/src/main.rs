//! A shop served under the path prefix `/app`, as a proxy in front of
//! several applications would serve it. The prefix is named once, as a
//! `Base`, and the links, the action calls, the redirect, and the runtime's
//! path all start with it. A path under `/app` that names no page shows the
//! shop's own 404 page.
//!
//! Run with `cargo run -p example-prefix` and open <http://localhost:3009/app>.

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use axum::Router;
use axum::routing::get;
use hypersig::html::{a, button, h1, li, p, span, ul};
use hypersig::{Action, Base, Page, PageResponse, Patches, Route, RouterExt, Signals, Unserved};
use serde::{Deserialize, Serialize};

/// The prefix the shop is served under.
const APP: Base = Base::new("/app");

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    in_cart: i32,
}

hypersig::routes! {
    fn pages(Router<AppState>) under APP else missing;
    const HOME: Route = "/" => home;
    const ITEM: Route<u32> = "/items/{id}" => item;
}

const ADD: Action<Shop> = Action::post("/cart").under(APP);

const ITEMS: [&str; 3] = ["Tea", "Coffee", "Cocoa"];

#[derive(Clone, Default)]
struct AppState {
    in_cart: Arc<AtomicI32>,
}

fn shop(state: &AppState, title: &str) -> Page<Shop> {
    let signals = Shop {
        in_cart: state.in_cart.load(Ordering::Relaxed),
    };
    Page::new(title, signals).child(
        p().id("cart")
            .text("In the cart: ")
            .child(span().bind_text(Shop::in_cart().to_text())),
    )
}

async fn home(state: AppState, (): ()) -> Page<Shop> {
    let mut list = ul().id("items");
    for (id, name) in (0_u32..).zip(ITEMS) {
        list = list.child(li().child(a().href(ITEM.link_with(&id)).text(name)));
    }
    shop(&state, "Shop").child(h1().text("Shop")).child(list)
}

async fn item(state: AppState, id: u32) -> Option<Page<Shop>> {
    let name = ITEMS.get(usize::try_from(id).ok()?)?;
    Some(
        shop(&state, name)
            .child(h1().text(*name))
            .child(button().on_click(ADD.call()).text("Add to cart"))
            .child(a().href(HOME.link()).text("All items")),
    )
}

async fn missing(state: AppState, unserved: Unserved) -> Page<Shop> {
    shop(&state, "Not found")
        .child(h1().text("Not found"))
        .child(p().id("problem").text(unserved.to_string()))
        .child(a().href(HOME.link()).text("All items"))
}

async fn add(state: AppState, (): ()) -> Patches<Shop> {
    let in_cart = state.in_cart.fetch_add(1, Ordering::Relaxed) + 1;
    Shop::in_cart().patch(in_cart).into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let shop = pages(Router::new())
        .hypersig_runtime()
        .action(ADD, add)
        .with_state(AppState::default());
    // Behind a proxy that strips `/app`, serve `shop` itself instead.
    let app = Router::new()
        .route("/", get(|| async { PageResponse::redirect(HOME.url(())) }))
        .nest(APP.as_str(), shop);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3009".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}/app");
    axum::serve(listener, app).await
}
