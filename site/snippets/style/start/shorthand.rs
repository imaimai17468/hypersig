use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::{px, rem};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Card {}

// `padding` is called last, and the top padding is still 0.
let card = Style::new().padding_top(px(0)).padding(rem(1));
let html = Page::new("Card", Card {}).child(div().css(card)).render();
let shorthand = html.find("{padding:1rem}").expect("the shorthand has a rule");
let longhand = html.find("{padding-top:0}").expect("the longhand has a rule");
assert!(shorthand < longhand);
