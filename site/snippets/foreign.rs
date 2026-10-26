use hypersig::html::{CustomEvent, Id, div};
use hypersig::{Asset, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Center {
    lat: f64,
    lng: f64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Office {
    center: Center,
}

// The script in `/map.js` finds the element with
// `document.getElementById("map")`, draws a map there, reads `data-lat` and
// `data-lng`, and dispatches `map-moved` with the new center.
hypersig::node_ref!(const MAP: NodeRef<Office>);
const MAP_ID: Id = Id::new("map");
const MOVED: CustomEvent<Center> = CustomEvent::new("map-moved");
const MAP_JS: Asset = Asset::new("/map.js");
# fn main() {

let center = Office::center();
let page = Page::new("Office", Office { center: Center { lat: 35.68, lng: 139.77 } })
    .script(MAP_JS)
    .child(
        div()
            // A patch keeps the tiles and classes the script added.
            .foreign(MAP)
            .id(MAP_ID)
            .bind_attr(hypersig::attr!("data-lat"), center.at(Center::lat()).to_text())
            .bind_attr(hypersig::attr!("data-lng"), center.at(Center::lng()).to_text())
            .on_custom(MOVED, move |moved| center.set(moved)),
    );
# assert!(page.render().contains(r#"id="map""#));
# }
