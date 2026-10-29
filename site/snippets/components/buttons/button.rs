use hypersig::html::{div, form, p, span};
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button, submit_button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    count: i32,
    ordered: bool,
}

let count = Cart::count();
let page = Page::new("Cart", Cart { count: 1, ordered: false }).child(
    form()
        .on_submit(Cart::ordered().set(true))
        .child(
            div()
                // `type="button"`, so these two do not submit the form.
                .child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(count.set(count - 1))
                        .disabled(count.le(1))
                        .text("Remove one"),
                )
                .child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(count.set(count + 1))
                        .disabled(count.ge(10))
                        .text("Add one"),
                ),
        )
        .child(p().text("Quantity: ").child(span().bind_text(count.to_text())))
        .child(
            div()
                .child(button(ButtonVariant::Ghost, ButtonSize::Default).on_click(count.set(1)).text("Reset"))
                // `type="submit"`, so this one runs the form's `on_submit`.
                .child(submit_button(ButtonVariant::Default, ButtonSize::Lg).text("Order")),
        ),
);
# let html = page.render();
# assert!(html.contains(r#"type="submit""#));
