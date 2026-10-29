use hypersig::style::value::Color;

assert_eq!(Color::CRIMSON.to_string(), "crimson");
assert_eq!(Color::hex(0x00b8_3f45).to_string(), "#b83f45");
assert_eq!(Color::rgb(228, 228, 231).to_string(), "rgb(228 228 231)");
assert_eq!(Color::CURRENT_COLOR.to_string(), "currentColor");
assert_eq!(Color::TRANSPARENT.to_string(), "transparent");
assert_eq!(
    Color::CRIMSON.opacity(20).to_string(),
    "color-mix(in srgb, crimson 20%, transparent)"
);
assert_eq!(
    Color::CRIMSON.mix(&Color::WHITE, 70).to_string(),
    "color-mix(in srgb, crimson 70%, white)"
);
