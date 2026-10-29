use hypersig::html::{button, div};
use hypersig::style::value::{Color, rem};
use hypersig::style::{Style, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Account {}

let brand = Token::new("brand", Color::hex(0x0025_63eb));
let action = Style::new().background_color(brand.var()).padding(rem(0.5));

let danger_zone = div::<Account>()
    // `--brand: #dc2626` on this element, inherited by the elements inside it.
    .css(Style::new().token(&brand, Color::hex(0x00dc_2626)))
    .child(button().css(action.clone()).text("Delete account"));

let html = Page::new("Account", Account {})
    // Outside the zone, `--brand` is unset and `var()` falls back to #2563eb.
    .child(button().css(action).text("Save"))
    .child(danger_zone)
    .render();
# assert!(html.contains("{--brand:#dc2626}"));
# let _ = html;
