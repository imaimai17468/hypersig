use hypersig::html::{a, article, attr, div, h3, img, main, p, span};
use hypersig::style::pattern::{
    aspect_ratio, center, circle, container, hstack, link_overlay, stack, visually_hidden,
};
use hypersig::style::value::{Color, ObjectFit, Position, pct, rem};
use hypersig::style::Style;
use hypersig::{Page, SafeUrl, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Blog {}

// The whole card is one link: the `::before` of the `<a>` covers the
// `<article>`, which `position: relative` makes the positioned ancestor.
let post = article::<Blog>()
    .css(stack(rem(0.75)).position(Position::Relative))
    .child(
        // A 16:9 box, which clips the image to its shape.
        div().css(aspect_ratio(16.0 / 9.0)).child(
            img()
                .src(SafeUrl::new("/covers/typed-css.jpg"))
                .attr(attr::ALT, "")
                .css(Style::new().width(pct(100)).height(pct(100)).object_fit(ObjectFit::Cover)),
        ),
    )
    .child(
        h3().child(
            a().href(SafeUrl::new("/posts/typed-css"))
                .css(link_overlay())
                .text("Typed CSS in Rust"),
        ),
    )
    .child(
        div()
            .css(hstack(rem(0.5)))
            .child(
                div()
                    .css(circle(rem(2)).background_color(Color::STEELBLUE).color(Color::WHITE))
                    .text("AL"),
            )
            .child(p().text("Ada Lovelace"))
            // Read by a screen reader after the name, and not shown.
            .child(span().css(visually_hidden()).text("3 comments")),
    );

let empty = div::<Blog>()
    .css(center().min_height(rem(12)))
    .child(p().text("No more posts"));

// At most 64rem wide, centered, with 1rem on both sides.
let page = Page::new("Blog", Blog {}).child(
    main().css(container(rem(64), rem(1))).child(post).child(empty),
);
assert!(page.render().contains("{aspect-ratio:1.7777777777777777}"));
