use hypersig::style::Style;
use hypersig::style::value::{clamp, max, pct, px, rem, vw};

// A heading that grows with the viewport, from 1.5rem up to 3rem.
let title = Style::new().font_size(clamp(rem(1.5), vw(4), rem(3)));
// `+`, `-`, `*`, and `/` write a `calc()`.
let sidebar = Style::new()
    .width(max(pct(25), pct(10) + pct(5)))
    .padding_left(px(16) * 1.5);

assert_eq!(clamp(rem(1.5), vw(4), rem(3)).to_string(), "clamp(1.5rem, 4vw, 3rem)");
assert_eq!((rem(1) - px(2) * 2.0).to_string(), "calc(1rem - (2px * 2))");
# assert!(!title.is_empty() && !sidebar.is_empty());
