//! Horizontal and vertical separators, for the separator page.

use hypersig::html::{Orientation, div, p, span};
use hypersig::style::Style;
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{FontWeight, rem};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, separator, token};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
pub struct Profile {
    section: String,
}

pub fn page() -> Page<Profile> {
    let section = Profile::section();
    let link = |name: &str| {
        button(ButtonVariant::Link, ButtonSize::Sm)
            .on_click(section.set(name))
            .text(name)
    };
    let initial = Profile {
        section: "Blog".into(),
    };
    Page::new("Profile", initial).child(
        div()
            .css(stack(rem(1)).max_width(rem(20)))
            .child(
                div()
                    .child(
                        p().css(Style::new().font_weight(FontWeight::Number(500.0)))
                            .text("hypersig"),
                    )
                    .child(
                        p().css(Style::new().color(token::muted_foreground().var()))
                            .text("Typed hypermedia for Rust."),
                    ),
            )
            .child(separator(Orientation::Horizontal))
            .child(
                div()
                    .css(hstack(rem(1)).height(rem(1.25)))
                    .child(link("Blog"))
                    .child(separator(Orientation::Vertical))
                    .child(link("Docs"))
                    .child(separator(Orientation::Vertical))
                    .child(link("Source")),
            )
            .child(p().text("Showing ").child(span().bind_text(section))),
    )
}
