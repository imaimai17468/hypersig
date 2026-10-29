use hypersig::html::{div, h3, p};
use hypersig::style::value::{ContainerType, Display, FlexDirection, rem};
use hypersig::style::{Container, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Shop {}

// The card lays itself out by the width of the column it is in,
// whatever the width of the viewport.
let card = div::<Shop>()
    .css(
        Style::new()
            .display(Display::Flex)
            .flex_direction(FlexDirection::Column)
            .gap(rem(0.5))
            .on(Container::Md, Style::new().flex_direction(FlexDirection::Row)),
    )
    .child(h3().text("Desk lamp"))
    .child(p().text("A lamp with a long arm."));

let column = div::<Shop>()
    // The query container the card measures.
    .css(Style::new().container_type(ContainerType::InlineSize))
    .child(card);
# let html = Page::new("Shop", Shop {}).child(column).render();
# assert!(html.contains("{container-type:inline-size}"));
# assert!(html.contains("@container (min-width: 28rem){"));
