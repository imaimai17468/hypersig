use hypersig::html::{h2, section};
use hypersig::{Page, Signals};
use hypersig_ui::{
    ButtonSize, ButtonVariant, Level, button, card, card_content, card_description, card_footer,
    card_header, card_title,
};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Pricing {}

// The section's heading is an `<h2>`, so each card's title is an `<h3>`.
let plan = |name: &str, audience: &str, price: &str| {
    card()
        .child(
            card_header()
                .child(card_title(Level::H3).text(name))
                .child(card_description().text(audience)),
        )
        .child(card_content().text(price))
        .child(
            card_footer()
                .child(button(ButtonVariant::Default, ButtonSize::Default).text("Choose")),
        )
};

let html = Page::new("Pricing", Pricing {})
    .child(
        section()
            .child(h2().text("Plans"))
            .child(plan("Hobby", "For one person.", "Free"))
            .child(plan("Pro", "For teams of up to ten.", "$20 a month")),
    )
    .render();
# assert_eq!(html.matches("<h3").count(), 2);
# assert!(html.contains(">For teams of up to ten.</p>"));
