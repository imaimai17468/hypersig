use hypersig::html::button;
use hypersig::style::value::{Color, rem};
use hypersig::style::{PseudoClass, Style, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

let brand = Token::new("brand", Color::hex(0x0025_63eb));
let space = Token::new("space", rem(0.5));

// `var(--brand, #2563eb)` is a `Color`, and `var(--space, 0.5rem)` is a `Length`.
let buy = button::<Shop>()
    .css(
        Style::new()
            .background_color(brand.var())
            .padding(space.var())
            .on(PseudoClass::Hover, Style::new().border_color(brand.var())),
    )
    .text("Buy");
let html = Page::new("Shop", Shop {}).child(buy).render();
assert!(html.contains("{background-color:var(--brand, #2563eb)}"));
assert!(html.contains("{padding:var(--space, 0.5rem)}"));
