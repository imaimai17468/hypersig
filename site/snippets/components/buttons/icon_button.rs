use hypersig::html::{attr, div, el, span};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonVariant, icon_button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Player {
    volume: i32,
}

// A Lucide icon, drawn in the button's text color and hidden from
// assistive technology, which reads the button's label instead.
let plus = el(hypersig::tag!("svg"))
    .attr(attr::ARIA_HIDDEN, "true")
    .attr(attr::VIEW_BOX, "0 0 24 24")
    .attr(attr::FILL, "none")
    .attr(attr::STROKE, "currentColor")
    .attr(attr::STROKE_WIDTH, "2")
    .child(el(hypersig::tag!("path")).attr(attr::D, "M5 12h14"))
    .child(el(hypersig::tag!("path")).attr(attr::D, "M12 5v14"));

let volume = Player::volume();
let page = Page::new("Player", Player { volume: 5 }).child(
    div()
        .child(
            icon_button(ButtonVariant::Outline, "Quieter")
                .on_click(volume.set(volume - 1))
                .disabled(volume.le(0))
                .text("−"),
        )
        .child(span().bind_text(volume.to_text()))
        .child(
            icon_button(ButtonVariant::Outline, "Louder")
                .on_click(volume.set(volume + 1))
                .disabled(volume.ge(10))
                .child(plus),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"aria-label="Louder""#));
