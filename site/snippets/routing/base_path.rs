use axum::Router;
use hypersig::html::{a, button, p};
use hypersig::{Action, Base, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {
    count: i32,
}

// Chosen when the program is built. `option_env!` reads it from the build's environment.
const APP: Base = Base::new("/app");

hypersig::routes! {
    fn pages(Router<()>) under APP;
    const HOME: Route = "/" => home;
    const ITEM: Route<u32> = "/items/{id}" => item;
    action ADD: Action<Shop> = post "/add" => add;
}

async fn home((): (), (): ()) -> Page<Shop> {
    Page::new("Shop", Shop { count: 0 }).child(a().href(ITEM.link_with(&3_u32)).text("Tea"))
}

async fn item((): (), id: u32) -> Page<Shop> {
    Page::new("Item", Shop { count: 0 })
        .child(p().text(format!("Item {id}")))
        .child(button().on_click(ADD.call()).text("Add"))
        .child(a().href(HOME.link()).text("Back"))
}

async fn add((): (), (): ()) -> Patches<Shop> {
    Shop::count().patch(1).into()
}

// Links and URLs start with the base, and the router's paths do not.
assert_eq!(ITEM.url(3).as_str(), "/app/items/3");
assert_eq!(HOME.url(()).as_str(), "/app");
assert_eq!(ITEM.path(), "/items/{id}");
assert_eq!(ADD.path(), "/add");

// Nested at the base here. Behind a proxy that strips `/app`, serve `app` as it is.
let app = pages(Router::new()).hypersig_runtime();
let _served: Router = Router::new().nest(APP.as_str(), app);
