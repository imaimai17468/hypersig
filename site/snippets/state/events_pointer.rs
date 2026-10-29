use hypersig::html::{El, Event, div, p, span};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Map {
    hovered: String,
}

let hovered = Map::hovered();
let pin = |city: &'static str| -> El<Map> {
    span()
        .on(Event::MouseEnter, hovered.set(city))
        .on(Event::MouseLeave, hovered.set(""))
        .text(city)
};

let map = div::<Map>()
    .child(pin("Kyoto"))
    .child(pin("Nara"))
    .child(p().show(!hovered.is_empty()).bind_text(hovered));
# let html = Page::new("Map", Map { hovered: String::new() }).child(map).render();
# let _ = html;
