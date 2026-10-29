use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::value::ZIndex;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Board {}

let style = Style::new()
    .order(2) // order: 2
    .flex_grow(1) // flex-grow: 1
    .z_index(ZIndex::Auto) // z-index: auto
    .font_weight(600) // font-weight: 600
    .opacity(0.5) // opacity: 0.5
    .line_height(1.5); // line-height: 1.5
# let html = Page::new("Board", Board {}).child(div().css(style)).render();
# for css in ["{order:2}", "{flex-grow:1}", "{z-index:auto}", "{font-weight:600}", "{opacity:0.5}", "{line-height:1.5}"] {
#     assert!(html.contains(css), "{css}");
# }
