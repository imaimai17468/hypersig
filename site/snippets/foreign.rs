use hypersig::html::{CustomEvent, div};
use hypersig::{NodeRef, Page, Signals};
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

// The script in `/map.js` draws a map into the element named `MAP`, reads
// `data-lat` and `data-lng`, and dispatches `map-moved` with the new center.
const MAP: NodeRef<Office> = NodeRef::new("map");
const MOVED: CustomEvent<Center> = CustomEvent::new("map-moved");

let center = Office::center();
let page = Page::new("Office", Office { center: Center { lat: 35.68, lng: 139.77 } })
    .script("/map.js")
    .child(
        div()
            // A patch keeps the tiles and classes the script added.
            .foreign(MAP)
            .bind_attr("data-lat", center.at(Center::lat()).to_text())
            .bind_attr("data-lng", center.at(Center::lng()).to_text())
            .on_custom(MOVED, move |moved| center.set(moved)),
    );
# assert!(page.render().contains(r#"data-hs-foreign="map""#));
