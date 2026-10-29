use hypersig::html::{El, span};
use hypersig::style::Style;
use hypersig::style::value::Color;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
enum Status {
    Draft,
    Review,
    Published,
}

#[derive(Signals, Serialize, Deserialize)]
struct Posts {}

// Adding a variant to `Status` is a compile error here until it has a style.
fn status_style(status: Status) -> Style {
    match status {
        Status::Draft => Style::new().background_color(Color::GRAY),
        Status::Review => Style::new().background_color(Color::ORANGE),
        Status::Published => Style::new().background_color(Color::GREEN),
    }
}

fn badge(status: Status) -> El<Posts> {
    span().css(status_style(status)).text("Status")
}
# let _ = badge(Status::Draft);
