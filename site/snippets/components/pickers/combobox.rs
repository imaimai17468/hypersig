use hypersig::html::{div, p, span};
use hypersig::{NodeRef, Page, Signals};
use hypersig_ui::{Combobox, ComboboxState, ComboboxText};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shipping {
    country: ComboboxState,
}

// One node ref for the text field and one for the list.
hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Shipping>);
hypersig::node_ref!(const COUNTRY_OPTIONS: NodeRef<Shipping>);
const COUNTRY: Combobox<Shipping> = Combobox::new(COUNTRY_FIELD, COUNTRY_OPTIONS);
# fn main() {

let country = Shipping::country();
let page = Page::new("Shipping", Shipping { country: ComboboxState::new("") }).child(
    div()
        .child(COUNTRY.render(
            "Country",
            country,
            ["France", "Germany", "Japan", "Kenya", "Mexico"],
            &ComboboxText::default(),
        ))
        // The text the visitor typed or chose.
        .child(
            p().text("Ships to: ")
                .child(span().bind_text(country.at(ComboboxState::text()))),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"role="combobox" aria-autocomplete="list""#));
# assert!(html.contains(r#"aria-expanded="false""#));
# }
