use hypersig::html::{article, nav};
use hypersig::style::value::{Color, Display, pct};
use hypersig::style::{Media, Style};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Docs {}

let column = Style::new()
    // Below 48rem.
    .width(pct(100))
    // From 48rem, and from 64rem.
    .on(Media::Md, Style::new().width(pct(50)))
    .on(Media::Lg, Style::new().width(pct(33)))
    // From 48rem while the system prefers dark.
    .on(Media::Md, Style::new().on(Media::Dark, Style::new().color(Color::WHITE)));

let sidebar = nav::<Docs>()
    .css(Style::new().on(Media::Print, Style::new().display(Display::None)))
    .text("Contents");
# let html = Page::new("Docs", Docs {})
#     .child(sidebar)
#     .child(article().css(column))
#     .render();
# assert!(html.contains("@media (min-width: 48rem){"));
# assert!(html.contains("@media (min-width: 48rem) and (prefers-color-scheme: dark){"));
# assert!(html.contains("@media print{"));
# let at = |rule: &str| html.find(rule).unwrap_or_else(|| panic!("no {rule}"));
# assert!(at("{width:100%}") < at("{width:50%}") && at("{width:50%}") < at("{width:33%}"));
