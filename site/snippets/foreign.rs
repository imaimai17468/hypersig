use hypersig::html::{CustomEvent, div};
use hypersig::{Page, Signals};
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

// The script in `/map.js` finds the element whose `data-hs-foreign` is the
// path of `MAP`, such as `office::MAP` in a crate named `office`, with
// `'[data-hs-foreign="' + CSS.escape("office::MAP") + '"]'`. It draws a map there,
// reads `data-lat` and `data-lng`, and dispatches `map-moved` with the new
// center.
hypersig::node_ref!(const MAP: NodeRef<Office>);
const MOVED: CustomEvent<Center> = CustomEvent::new("map-moved");
# fn main() {

let center = Office::center();
let page = Page::new("Office", Office { center: Center { lat: 35.68, lng: 139.77 } })
    .script("/map.js")
    .child(
        div()
            // A patch keeps the tiles and classes the script added.
            .foreign(MAP)
            .bind_attr(hypersig::attr!("data-lat"), center.at(Center::lat()).to_text())
            .bind_attr(hypersig::attr!("data-lng"), center.at(Center::lng()).to_text())
            .on_custom(MOVED, move |moved| center.set(moved)),
    );
# assert!(page.render().contains(&format!(r#"data-hs-foreign="{}::MAP""#, module_path!())));
# }
