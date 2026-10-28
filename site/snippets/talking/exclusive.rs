use std::time::Duration;

use hypersig::html::{button, div, p};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Checkout {
    ordering: bool,
    problem: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    action ORDER: Action<Checkout> = post "/order" => order;
    action QUOTE: Action<Checkout> = post "/quote" => quote;
}
# async fn order((): (), (): ()) -> Patches<Checkout> { Patches::new() }
# async fn quote((): (), (): ()) -> Patches<Checkout> { Patches::new() }

// A second click while the first order is on its way sends nothing.
let order = ORDER.call().indicator(Checkout::ordering()).exclusive();
// A quote that takes more than 10 seconds counts as failed.
let quote = QUOTE
    .call()
    .error(Checkout::problem())
    .timeout(Duration::from_secs(10));

let page = div::<Checkout>()
    .child(button().on_click(order).text("Place the order"))
    .child(button().on_click(quote).text("Get a shipping quote"))
    .child(p().bind_text(Checkout::problem()));
# let _ = (page, pages);
