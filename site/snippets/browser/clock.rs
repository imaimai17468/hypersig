use hypersig::html::{button, div, span};
use hypersig::{Expr, Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Dice {
    face: i32,
    rolled_at: f64,
}

let (face, rolled_at) = (Dice::face(), Dice::rolled_at());
let page = Page::new("Dice", Dice { face: 1, rolled_at: 0.0 }).child(
    div()
        // A number from 0 to 5, plus one.
        .child(
            button()
                .on_click(face.set(Expr::random_below(6) + 1).then(rolled_at.set(Expr::now())))
                .text("Roll"),
        )
        .child(span().bind_text(face.to_text()))
        // Milliseconds since the Unix epoch, read from the browser's clock.
        .child(span().bind_text(Expr::now().to_fixed(0))),
);
# assert!(page.render().contains("></span></div>"));
