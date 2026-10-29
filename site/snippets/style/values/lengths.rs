use hypersig::style::value::{Length, ch, em, pct, px, rem};

assert_eq!(px(16).to_string(), "16px");
assert_eq!(rem(1.5).to_string(), "1.5rem");
assert_eq!(em(-0.25).to_string(), "-0.25em");
assert_eq!(ch(65).to_string(), "65ch");
assert_eq!(pct(50).to_string(), "50%");
// A zero length has no unit, and a zero percentage keeps its `%`.
assert_eq!(px(0).to_string(), "0");
assert_eq!(Length::ZERO.to_string(), "0");
assert_eq!(pct(0).to_string(), "0%");
