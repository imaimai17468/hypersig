//! A theme picker over a card, for the theming page.

use hypersig::html::div;
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{
    ButtonSize, ButtonVariant, Level, button, card, card_content, card_description, card_header,
    card_title, select_field, switch, token,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Scheme {
    System,
    Light,
    Dark,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Preview {
    scheme: Scheme,
    on: bool,
}

pub fn page() -> Page<Preview> {
    let initial = Preview {
        scheme: Scheme::System,
        on: true,
    };
    Page::new("Preview", initial).child(
        div()
            .css(
                stack(rem(1))
                    .padding(rem(1))
                    .background_color(token::background().var())
                    .color(token::foreground().var()),
            )
            .css_match(
                Preview::scheme(),
                [
                    (Scheme::Light, hypersig_ui::light()),
                    (Scheme::Dark, hypersig_ui::dark()),
                ],
            )
            .child(
                div().css(Style::new().max_width(rem(12))).child(select_field(
                    "Theme",
                    Preview::scheme(),
                    [
                        (Scheme::System, "System"),
                        (Scheme::Light, "Light"),
                        (Scheme::Dark, "Dark"),
                    ],
                )),
            )
            .child(
                card()
                    .child(
                        card_header()
                            .child(card_title(Level::H2).text("Notifications"))
                            .child(card_description().text("Choose what reaches your inbox.")),
                    )
                    .child(
                        card_content()
                            .css(stack(rem(1)))
                            .child(switch("Product updates", Preview::on()))
                            .child(
                                div().child(
                                    button(ButtonVariant::Default, ButtonSize::Sm).text("Save"),
                                ),
                            ),
                    ),
            ),
    )
}
