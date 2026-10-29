use hypersig::html::{div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{NodeRef, Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Popover, TextField, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Layer {
    width: String,
}

hypersig::node_ref!(const SIZE_PANEL: NodeRef<Layer>);
const SIZE: Popover<Layer> = Popover::new(SIZE_PANEL);
# fn main() {

let html = Page::new("Layer", Layer { width: "100%".into() })
    .child(
        div()
            .child(SIZE.trigger(ButtonVariant::Outline, ButtonSize::Default).text("Size"))
            // Right after its button, so Tab moves from the button into it.
            .child(
                SIZE.popover().child(
                    div()
                        .css(stack(rem(0.75)))
                        .child(p().text("Set the size of the layer."))
                        .child(TextField::new("Width", Layer::width()))
                        .child(
                            button(ButtonVariant::Default, ButtonSize::Sm)
                                .on_click(SIZE_PANEL.hide_popover())
                                .text("Done"),
                        ),
                ),
            ),
    )
    .render();
# assert!(html.contains(r#"aria-haspopup="dialog""#));
# assert!(html.contains(r#"command="toggle-popover""#));
# assert!(html.contains(r#"popover="auto" role="dialog""#));
# }
