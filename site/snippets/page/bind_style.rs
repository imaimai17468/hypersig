use hypersig::html::div;
use hypersig::style::Property;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Meter {
    level: i32,
}

// The width follows `level`. The string is not checked against the property.
let bar = div().bind_style(Property::Width, Meter::level().to_text().concat("%"));
let html = Page::new("Meter", Meter { level: 40 }).child(bar).render();
assert!(html.contains(r#"<div style="width: 40%""#));
