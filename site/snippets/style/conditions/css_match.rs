use hypersig::html::span;
use hypersig::style::value::{Color, rem};
use hypersig::style::Style;
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Status {
    Draft,
    Review,
    Published,
}

#[derive(Signals, Serialize, Deserialize)]
struct Post {
    status: Status,
}

let badge = span::<Post>()
    .css(Style::new().padding(rem(0.25)).color(Color::WHITE))
    // One case applies at a time, so the cases never compete for `background-color`.
    .css_match(
        Post::status(),
        [
            (Status::Draft, Style::new().background_color(Color::GRAY)),
            (Status::Review, Style::new().background_color(Color::ORANGE)),
            // Leaving out `Published` compiles, and the badge then has no background.
            (Status::Published, Style::new().background_color(Color::GREEN)),
        ],
    )
    .text("Status");
# let html = Page::new("Post", Post { status: Status::Review }).child(badge).render();
# assert!(html.contains("color_white when:background-color_orange\""));
