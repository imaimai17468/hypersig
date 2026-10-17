use hypersig::html::{div, span};
use hypersig::style::Media;
use hypersig::style::pattern::{center, circle, container, grid, hstack, spacer, stack, visually_hidden};
use hypersig::style::value::rem;
use hypersig::Signals;
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Gallery {
    count: i32,
}

let page = div::<Gallery>()
    // At most 64rem wide, centered, with 1rem on both sides.
    .css(container(rem(64), rem(1)))
    .child(
        div()
            .css(hstack(rem(0.5)))
            .child(div().css(circle(rem(2)).merge(center())).text("A"))
            .child(span().text("Ada"))
            .child(div().css(spacer()))
            .child(span().css(visually_hidden()).text("3 new messages")),
    )
    .child(
        // One column, then three from the medium breakpoint.
        div()
            .css(grid(1, rem(1)).on(Media::Md, grid(3, rem(1))))
            .children((1..=6).map(|n| div().css(stack(rem(0.25))).text(n.to_string()))),
    );
# let _ = page;
