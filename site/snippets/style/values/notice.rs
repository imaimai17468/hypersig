use hypersig::html::{El, div, p, strong};
use hypersig::style::Style;
use hypersig::style::value::{
    AlignItems, BorderLeftStyle, Color, Display, FlexDirection, FontWeight, px, rem,
};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Billing {}

/// A notice with a colored bar on its left edge, a bold title, and a body.
fn notice(title: &str, body: &str) -> El<Billing> {
    div()
        .css(
            Style::new()
                .display(Display::Flex)
                .flex_direction(FlexDirection::Column)
                .align_items(AlignItems::FlexStart)
                .gap(rem(0.25))
                .padding(rem(1))
                .border_left_width(px(4))
                .border_left_style(BorderLeftStyle::Solid)
                .border_left_color(Color::hex(0x00d9_7706))
                .background_color(Color::hex(0x00ff_fbeb))
                .line_height(1.5),
        )
        .child(strong().css(Style::new().font_weight(FontWeight::Bold)).text(title))
        .child(p().css(Style::new().margin(px(0))).text(body))
}

let html = Page::new("Billing", Billing {})
    .child(notice("Card expiring", "Your card ends in 4242 and expires this month."))
    .render();
# for css in ["{display:flex}", "{flex-direction:column}", "{align-items:flex-start}", "{border-left-width:4px}", "{line-height:1.5}", "{margin:0}"] {
#     assert!(html.contains(css), "{css}");
# }
