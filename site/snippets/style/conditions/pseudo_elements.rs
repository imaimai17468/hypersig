use hypersig::html::{a, input, li, ul};
use hypersig::style::value::{Color, quoted};
use hypersig::style::{PseudoClass, PseudoElement, Style};
use hypersig::{Page, SafeUrl, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Search {
    query: String,
}

// `::after` is drawn only when `content` is set.
let more = a::<Search>()
    .href(SafeUrl::new("/docs"))
    .css(
        Style::new()
            .pseudo(PseudoElement::After, Style::new().content(quoted(" →")))
            // `:hover::after`: the pseudo-class is tested on the link.
            .on(
                PseudoClass::Hover,
                Style::new().pseudo(PseudoElement::After, Style::new().color(Color::RED)),
            ),
    )
    .text("More");

let field = input::<Search>()
    .bind(Search::query())
    .css(Style::new().pseudo(PseudoElement::Placeholder, Style::new().color(Color::GRAY)));

// `::marker` is the bullet of a list item.
let step = Style::new().pseudo(PseudoElement::Marker, Style::new().color(Color::hex(0x0025_63eb)));
let steps = ul::<Search>().child(li().css(step).text("Install"));
# let html = Page::new("Search", Search { query: String::new() })
#     .child(more)
#     .child(field)
#     .child(steps)
#     .render();
# assert!(html.contains("::after{content:\" →\"}"));
# assert!(html.contains(":hover::after{color:red}"));
# assert!(html.contains("::placeholder{color:gray}"));
