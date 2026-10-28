use hypersig::html::{Stylesheet, header};
use hypersig::style::Style;
use hypersig::style::value::raw;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Article {}

// Every page that takes part adds the same stylesheet.
const TRANSITIONS: Stylesheet = Stylesheet::new(
    "
@view-transition { navigation: auto; }
@media (prefers-reduced-motion: no-preference) {
  ::view-transition-old(root) { animation: 250ms ease-in both fade-out; }
  ::view-transition-new(root) { animation: 400ms ease-out both fade-in; }
}
@media (prefers-reduced-motion: reduce) {
  ::view-transition-group(*), ::view-transition-old(*), ::view-transition-new(*) { animation: none; }
}
@keyframes fade-out { to { opacity: 0; } }
@keyframes fade-in { from { opacity: 0; } }
",
);

let page = Page::new("Article", Article {})
    .style(TRANSITIONS)
    // A name of its own keeps the header in place while the rest moves.
    .child(header().css(Style::new().view_transition_name(raw("site-header"))));
assert!(page.render().contains("@view-transition"));
