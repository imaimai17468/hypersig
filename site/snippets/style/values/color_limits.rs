use hypersig::style::value::Color;

// `#rrggbbaa` goes through `hex_alpha`, and `Color::hex(0x2563_eb80)` panics.
assert_eq!(Color::hex_alpha(0x2563_eb80).to_string(), "#2563eb80");
// A red byte of `00` makes the literal six digits, so `hex` takes it.
assert_eq!(Color::hex(0x00ff_0080).to_string(), "#ff0080");
// A percentage above 100 counts as 100.
assert_eq!(
    Color::BLACK.opacity(150).to_string(),
    "color-mix(in srgb, black 100%, transparent)"
);
